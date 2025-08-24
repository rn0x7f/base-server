use email_sender::{EmailSender, EmailContent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Crear una instancia del enviador de emails
    let email_sender = EmailSender::new(
        "smtp.gmail.com",           // Servidor SMTP
        "tu_email@gmail.com",       // Usuario SMTP
        "tu_password_app",          // Contraseña de aplicación
        "tu_email@gmail.com",       // Email remitente
    )?;

    // Lista de destinatarios
    let recipients = vec![
        "destinatario1@example.com".to_string(),
        "destinatario2@example.com".to_string(),
    ];

    // Enviar email simple de texto
    email_sender.send_simple_email(
        &recipients,
        "Hola desde Rust!",
        "Este es un email enviado usando la librería email-sender en Rust.",
    ).await?;

    // Enviar email HTML
    let html_body = r#"
        <html>
            <body>
                <h1>¡Hola desde Rust!</h1>
                <p>Este es un email <strong>HTML</strong> enviado usando la librería email-sender.</p>
                <p>Puedes usar cualquier contenido HTML válido.</p>
            </body>
        </html>
    "#;

    email_sender.send_html_email(
        &recipients,
        "Email HTML desde Rust",
        html_body,
    ).await?;

    // Enviar email personalizado usando EmailContent
    let custom_content = EmailContent {
        subject: "Email personalizado".to_string(),
        body: "Este es un email con contenido personalizado.".to_string(),
        is_html: false,
    };

    email_sender.send_email_to_multiple(&recipients, &custom_content).await?;

    println!("¡Emails enviados exitosamente!");

    Ok(())
}
