use crate::common::ApplicationConfig;
use derive_builder::Builder;
use lazy_static::lazy_static;
use tracing::Level;
use std::env;

lazy_static! {
    pub static ref CONFIG: Config = {
        let mut config = ConfigBuilder::default().build().unwrap();

        // Load environment variables with fallback to default values
        config.mode = env::var("MODE").unwrap_or_else(|_| {
            config.mode.clone()
        });

        config.port = env::var("PORT").unwrap_or_else(|_| {
            config.port.clone()
        });

        config.host = env::var("HOST").unwrap_or_else(|_| {
            config.host.clone()
        });

        config.smtp_server = env::var("SMTP_SERVER").unwrap_or(config.smtp_server);
        config.smtp_user = env::var("SMTP_USER").unwrap_or(config.smtp_user);
        config.smtp_password = env::var("SMTP_PASSWORD").unwrap_or(config.smtp_password);
        config.smtp_from = env::var("FROM_EMAIL").unwrap_or(config.smtp_from);

        // Configuración para emails de admin
        config.admin_emails = env::var("ADMIN_EMAILS")
            .ok()
            .map(|s| s.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or(config.admin_emails);

        config
    };
}

/// Main application configuration structure with default values
#[derive(Builder, Debug)]
pub struct Config {
    #[builder(default = "Level::DEBUG")]
    pub max_level_log: Level,
    #[builder(default = "String::from(\"dev\")")]
    pub mode: String,
    #[builder(default = "String::from(\"8080\")")]
    pub port: String,
    #[builder(default = "String::from(\"0.0.0.0\")")]
    pub host: String,
    #[builder(default = "String::from(\"smtp.gmail.com\")")]
    pub smtp_server: String,
    #[builder(default = "String::from(\"usuario@gmail.com\")")]
    pub smtp_user: String,
    #[builder(default = "String::from(\"password\")")]
    pub smtp_password: String,
    #[builder(default = "String::from(\"no-reply@tusitio.com\")")]
    pub smtp_from: String,
    #[builder(default = "vec![\"admin@example.com\".to_string()]")]
    pub admin_emails: Vec<String>,
}

/// Implementation of common application configuration interface
impl ApplicationConfig for Config {
    fn get_addrs() -> String {
        format!("{}:{}", CONFIG.host, CONFIG.port)
    }

    fn get_max_level_log() -> Level {
        CONFIG.max_level_log
    }

    fn get_mode() -> &'static str {
        &CONFIG.mode
    }

    fn get_port() -> &'static str {
        &CONFIG.port
    }

    fn get_host() -> &'static str {
        &CONFIG.host
    }

    fn get_max_pool_size() -> u32 {
        8 // Valor por defecto
    }

    fn get_with_migrations() -> bool {
        true // Valor por defecto
    }
    
}

/// Additional configuration methods specific to this application
impl Config {
    pub fn get_smtp_server() -> &'static str {
        &CONFIG.smtp_server
    }

    pub fn get_smtp_user() -> &'static str {
        &CONFIG.smtp_user
    }

    pub fn get_smtp_password() -> &'static str {
        &CONFIG.smtp_password
    }

    pub fn get_smtp_from() -> &'static str {
        &CONFIG.smtp_from
    }

    /// Obtiene la lista de emails de admin como un vector
    pub fn get_admin_emails_list() -> Vec<String> {
        CONFIG.admin_emails.clone()
    }
}