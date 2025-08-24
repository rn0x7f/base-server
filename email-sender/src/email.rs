use anyhow::{anyhow, Result};
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use lettre::message::{header::ContentType, MultiPart, SinglePart};
use serde::{Deserialize, Serialize};

/// Estructura para configurar el contenido del email
#[derive(Debug, Serialize, Deserialize)]
pub struct EmailContent {
    pub subject: String,
    pub body: String,
    pub is_html: bool,
}

/// Struct que encapsula configuración y lógica de envío de correos
pub struct EmailSender {
    mailer: SmtpTransport,
    from_email: String,
}

impl EmailSender {
    /// Crea un nuevo enviador de correo
    pub fn new(
        smtp_server: &str,
        smtp_user: &str,
        smtp_password: &str,
        from_email: &str,
    ) -> Result<Self> {
        let creds = Credentials::new(smtp_user.to_string(), smtp_password.to_string());
        let mailer = SmtpTransport::relay(smtp_server)?
            .credentials(creds)
            .build();

        Ok(Self {
            mailer,
            from_email: from_email.to_string(),
        })
    }

    /// Envía un email a múltiples destinatarios
    pub async fn send_email_to_multiple(
        &self,
        recipients: &[String],
        content: &EmailContent,
    ) -> Result<()> {
        for recipient in recipients {
            self.send_single_email(recipient, content).await?;
        }
        Ok(())
    }

    /// Envía un email a un solo destinatario
    pub async fn send_single_email(
        &self,
        recipient: &str,
        content: &EmailContent,
    ) -> Result<()> {
        let from_address = self.from_email.clone();

        let email_builder = Message::builder()
            .from(from_address.parse().map_err(|_| anyhow!("Email 'from' inválido"))?)
            .to(recipient.parse().map_err(|_| anyhow!("Email 'to' inválido"))?)
            .subject(&content.subject);

        let email = if content.is_html {
            // Para HTML, usar el ContentType correcto
            email_builder.multipart(
                MultiPart::alternative()
                    .singlepart(
                        SinglePart::builder()
                            .header(ContentType::TEXT_HTML)
                            .body(content.body.clone())
                    )
            )?
        } else {
            // Para texto plano
            email_builder.body(content.body.clone())?
        };

        self.mailer
            .send(&email)
            .map_err(|e| anyhow!("Fallo al enviar el correo a {}: {}", recipient, e))?;

        Ok(())
    }

    /// Envía un email simple con texto plano
    pub async fn send_simple_email(
        &self,
        recipients: &[String],
        subject: &str,
        body: &str,
    ) -> Result<()> {
        let content = EmailContent {
            subject: subject.to_string(),
            body: body.to_string(),
            is_html: false,
        };

        self.send_email_to_multiple(recipients, &content).await
    }

    /// Envía un email HTML
    pub async fn send_html_email(
        &self,
        recipients: &[String],
        subject: &str,
        html_body: &str,
    ) -> Result<()> {
        let content = EmailContent {
            subject: subject.to_string(),
            body: html_body.to_string(),
            is_html: true,
        };

        self.send_email_to_multiple(recipients, &content).await
    }

    /// Verifica la conexión SMTP
    pub fn test_connection(&self) -> Result<()> {
        // Crear un email de prueba simple
        let test_email = Message::builder()
            .from(self.from_email.parse().map_err(|_| anyhow!("Email 'from' inválido"))?)
            .to(self.from_email.parse().map_err(|_| anyhow!("Email 'to' inválido"))?)
            .subject("Test de conexión")
            .body("Este es un email de prueba para verificar la conexión SMTP.".to_string())?;

        self.mailer
            .send(&test_email)
            .map_err(|e| anyhow!("Fallo en la prueba de conexión SMTP: {}", e))?;

        Ok(())
    }
}
