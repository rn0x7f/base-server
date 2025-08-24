use email_sender::EmailContent;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Demo de Email Sender ===\n");

    // Simular creación del enviador (sin credenciales reales)
    println!("1. Creando instancia del EmailSender...");
    println!("   - Servidor SMTP: smtp.gmail.com");
    println!("   - Usuario: tu_email@gmail.com");
    println!("   - Remitente: tu_email@gmail.com");
    println!("   - Nombre: Tu Nombre\n");

    // Lista de destinatarios de ejemplo
    let recipients = vec![
        "usuario1@example.com".to_string(),
        "usuario2@example.com".to_string(),
        "usuario3@example.com".to_string(),
    ];

    println!("2. Lista de destinatarios:");
    for (i, recipient) in recipients.iter().enumerate() {
        println!("   {}. {}", i + 1, recipient);
    }
    println!();

    // Mostrar diferentes tipos de emails
    println!("3. Tipos de emails disponibles:");
    println!("   a) Email de texto plano");
    println!("   b) Email HTML");
    println!("   c) Email personalizado con EmailContent");
    println!();

    // Simular envío de email simple
    println!("4. Simulando envío de email simple...");
    println!("   Asunto: ¡Hola desde Rust!");
    println!("   Contenido: Este es un email de prueba enviado usando email-sender.");
    println!("   Destinatarios: {} personas", recipients.len());
    println!("   Estado: Simulado (no se envió realmente)\n");

    // Simular envío de email HTML
    println!("5. Simulando envío de email HTML...");
    println!("   Asunto: Email HTML desde Rust");
    println!("   Contenido: <h1>¡Hola!</h1><p>Email con formato HTML</p>");
    println!("   Destinatarios: {} personas", recipients.len());
    println!("   Estado: Simulado (no se envió realmente)\n");

    // Mostrar estructura EmailContent
    println!("6. Estructura EmailContent:");
    let demo_content = EmailContent {
        subject: "Email personalizado".to_string(),
        body: "Contenido personalizado del email".to_string(),
        is_html: false,
    };
    println!("   Asunto: {}", demo_content.subject);
    println!("   Contenido: {}", demo_content.body);
    println!("   Es HTML: {}", demo_content.is_html);
    println!();

    println!("=== Funcionalidades disponibles ===");
    println!("✅ Envío a múltiples destinatarios");
    println!("✅ Soporte para texto plano y HTML");
    println!("✅ Configuración flexible de SMTP");
    println!("✅ Manejo robusto de errores");
    println!("✅ Funciones asíncronas");
    println!("✅ Verificación de conexión");
    println!();

    println!("Para usar con credenciales reales, modifica el ejemplo basic_usage.rs");
    println!("con tus credenciales SMTP válidas.");

    Ok(())
}
