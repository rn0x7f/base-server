use actix_web::{web, HttpResponse, Responder, post, HttpRequest};
use crate::models::email::ContactRequest;
use crate::config::Config;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ContactResponse {
    pub success: bool,
    pub message: String,
    pub remaining_requests: Option<(usize, usize)>, // (por minuto, por 12 horas)
}

#[derive(Debug, Serialize)]
pub struct RateLimitResponse {
    pub success: bool,
    pub message: String,
    pub remaining_requests: (usize, usize), // (por minuto, por 12 horas)
    pub retry_after: Option<String>,
}

#[post("")]
pub async fn contact_request(
    req: HttpRequest,
    data: web::Json<ContactRequest>,
    app_state: web::Data<crate::app_state::AppState>,
) -> impl Responder {
    // Obtener la IP del cliente
    let client_ip = req
        .connection_info()
        .realip_remote_addr()
        .unwrap_or("unknown")
        .to_string();

    // Verificar rate limiting
    let (can_request, remaining_requests) = app_state.rate_limiter.check_rate_limit(&client_ip);
    
    if !can_request {
        let (remaining_minute, remaining_12h) = remaining_requests;
        
        let message = if remaining_minute == 0 {
            "Demasiadas solicitudes por minuto. Intenta de nuevo en 1 minuto."
        } else if remaining_12h == 0 {
            "Has alcanzado el límite máximo de solicitudes por 12 horas. Intenta de nuevo más tarde."
        } else {
            "Demasiadas solicitudes. Intenta de nuevo más tarde."
        };

        return HttpResponse::TooManyRequests().json(RateLimitResponse {
            success: false,
            message: message.to_string(),
            remaining_requests,
            retry_after: Some("60s".to_string()), // 1 minuto
        });
    }

    // Obtener la lista de emails de admin desde la configuración
    let admin_emails = Config::get_admin_emails_list();

    // Crear el contenido del email de notificación con la IP
    let html_body = format!(
        r#"
        <!DOCTYPE html>
        <html lang="es">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Nueva Solicitud de Contacto</title>
            <style>
                body {{
                    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
                    line-height: 1.6;
                    color: #333;
                    max-width: 600px;
                    margin: 0 auto;
                    padding: 20px;
                    background-color: #f4f4f4;
                }}
                .container {{
                    background-color: #ffffff;
                    border-radius: 10px;
                    padding: 30px;
                    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
                }}
                .header {{
                    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                    color: white;
                    padding: 20px;
                    border-radius: 8px;
                    text-align: center;
                    margin-bottom: 25px;
                }}
                .header h1 {{
                    margin: 0;
                    font-size: 24px;
                    font-weight: 300;
                }}
                .info-section {{
                    background-color: #f8f9fa;
                    border-left: 4px solid #667eea;
                    padding: 15px;
                    margin: 15px 0;
                    border-radius: 0 5px 5px 0;
                }}
                .info-section h3 {{
                    margin: 0 0 10px 0;
                    color: #667eea;
                    font-size: 16px;
                }}
                .field {{
                    margin: 10px 0;
                }}
                .field strong {{
                    color: #555;
                    display: inline-block;
                    width: 80px;
                }}
                .message-box {{
                    background-color: #fff3cd;
                    border: 1px solid #ffeaa7;
                    border-radius: 5px;
                    padding: 15px;
                    margin: 15px 0;
                }}
                .footer {{
                    text-align: center;
                    margin-top: 30px;
                    padding-top: 20px;
                    border-top: 1px solid #eee;
                    color: #666;
                    font-size: 14px;
                }}
                .ip-badge {{
                    background-color: #e3f2fd;
                    color: #1976d2;
                    padding: 5px 10px;
                    border-radius: 15px;
                    font-size: 12px;
                    font-family: monospace;
                }}
            </style>
        </head>
        <body>
            <div class="container">
                <div class="header">
                    <h1>📧 Nueva Solicitud de Contacto</h1>
                </div>
                
                <div class="info-section">
                    <h3>📍 Información del Cliente</h3>
                    <div class="field">
                        <strong>IP:</strong> 
                        <span class="ip-badge">{}</span>
                    </div>
                    <div class="field">
                        <strong>Nombre:</strong> {}
                    </div>
                    <div class="field">
                        <strong>Empresa:</strong> {}
                    </div>
                    <div class="field">
                        <strong>Email:</strong> {}
                    </div>
                    <div class="field">
                        <strong>Servicio:</strong> {}
                    </div>
                </div>
                
                <div class="message-box">
                    <h3>💬 Mensaje del Cliente</h3>
                    <p style="white-space: pre-wrap; margin: 0;">{}</p>
                </div>
                
                <div class="footer">
                    <p>Este email fue generado automáticamente por el sistema de contactos.</p>
                    <p>Fecha y hora: {}</p>
                </div>
            </div>
        </body>
        </html>
        "#,
        client_ip, 
        data.name, 
        data.company, 
        data.email, 
        data.service, 
        data.message,
        chrono::Utc::now().format("%d/%m/%Y %H:%M:%S").to_string()
    );

    // Enviar el email de notificación HTML a los admins
    match app_state.email_sender.send_html_email(
        &admin_emails,
        "Nueva solicitud de contacto",
        &html_body,
    ).await {
        Ok(()) => {
            HttpResponse::Ok().json(ContactResponse {
                success: true,
                message: "Solicitud de contacto enviada exitosamente".to_string(),
                remaining_requests: Some(remaining_requests),
            })
        }
        Err(e) => {
            eprintln!("Error al enviar email de notificación: {}", e);
            HttpResponse::InternalServerError().json(ContactResponse {
                success: false,
                message: "Error al procesar la solicitud de contacto".to_string(),
                remaining_requests: None,
            })
        }
    }
}

pub fn routes() -> actix_web::Scope {
    web::scope("/contact")
        .service(contact_request)
} 