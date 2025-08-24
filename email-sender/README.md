# Email Sender

Una librería simple y eficiente para envío de emails en Rust, construida sobre `lettre`.

## Características

- ✅ Envío de emails a múltiples destinatarios
- ✅ Soporte para emails de texto plano y HTML
- ✅ Configuración simple de credenciales SMTP
- ✅ Manejo de errores robusto con `anyhow`
- ✅ Funciones asíncronas para mejor rendimiento
- ✅ Verificación de conexión SMTP

## Instalación

Agrega esto a tu `Cargo.toml`:

```toml
[dependencies]
email-sender = { path = "./email-sender" }
```

## Uso Básico

### 1. Crear una instancia del enviador

```rust
use email_sender::EmailSender;

let email_sender = EmailSender::new(
    "smtp.gmail.com",           // Servidor SMTP
    "tu_email@gmail.com",       // Usuario SMTP
    "tu_password_app",          // Contraseña de aplicación
    "tu_email@gmail.com",       // Email remitente
)?;
```

### 2. Enviar emails

```rust
// Lista de destinatarios
let recipients = vec![
    "destinatario1@example.com".to_string(),
    "destinatario2@example.com".to_string(),
];

// Email simple de texto
email_sender.send_simple_email(
    &recipients,
    "Asunto del email",
    "Contenido del email en texto plano",
).await?;

// Email HTML
let html_body = r#"
    <html>
        <body>
            <h1>¡Hola!</h1>
            <p>Este es un email <strong>HTML</strong>.</p>
        </body>
    </html>
"#;

email_sender.send_html_email(
    &recipients,
    "Email HTML",
    html_body,
).await?;
```

### 3. Email personalizado

```rust
use email_sender::EmailContent;

let custom_content = EmailContent {
    subject: "Asunto personalizado".to_string(),
    body: "Contenido personalizado".to_string(),
    is_html: false,
};

email_sender.send_email_to_multiple(&recipients, &custom_content).await?;
```

## Funciones Disponibles

- `new()` - Crea una nueva instancia del enviador
- `send_simple_email()` - Envía email de texto plano
- `send_html_email()` - Envía email HTML
- `send_email_to_multiple()` - Envía email personalizado a múltiples destinatarios
- `send_single_email()` - Envía email a un solo destinatario
- `test_connection()` - Verifica la conexión SMTP

## Configuración SMTP

### Gmail
- Servidor: `smtp.gmail.com`
- Puerto: 587 (TLS) o 465 (SSL)
- Usar contraseña de aplicación, no la contraseña normal

### Outlook/Hotmail
- Servidor: `smtp-mail.outlook.com`
- Puerto: 587

### Otros proveedores
Consulta la documentación de tu proveedor de email para los detalles SMTP.

## Ejemplo Completo

Ver el archivo `examples/basic_usage.rs` para un ejemplo completo de uso.

## Manejo de Errores

La librería usa `anyhow::Result` para manejo de errores. Todos los errores incluyen contexto descriptivo:

```rust
match email_sender.send_simple_email(&recipients, "Asunto", "Contenido").await {
    Ok(()) => println!("Email enviado exitosamente"),
    Err(e) => eprintln!("Error al enviar email: {}", e),
}
```

## Licencia

MIT
