use email_sender::EmailSender;

use once_cell::sync::OnceCell;
// use actix_files as fs;
use actix_web::{App, HttpServer, middleware::Logger, web};
use common::{Application, ApplicationConfig};
use config::Config;
use tracing::info;
use anyhow::{anyhow, Result};
use app_state::AppState;

// Internal modules
mod common;
mod config;
mod controllers;
//mod database;
mod models;
//mod schema;
//mod middlewares;
mod app_state;
mod rate_limiter;

struct AppServer;

static APP_STATE: OnceCell<web::Data<AppState>> = OnceCell::new();

impl Application for AppServer {
    async fn setup(&self) -> Result<()> {
        //info!("Initializing the database...");
        //Database::init(Config::get_max_pool_size(), Config::get_with_migrations())?;
        //info!("Migrations applied successfully");

        info!("Initializing SMTP client");
        let email_sender = EmailSender::new(
            Config::get_smtp_server(),
            Config::get_smtp_user(),
            Config::get_smtp_password(),
            Config::get_smtp_from(),
        )?;
        info!("SMTP client initialized successfully");

        let app_state = web::Data::new(AppState::new(email_sender).await?);
        APP_STATE.set(app_state).map_err(|_| anyhow!("APP_STATE ya fue inicializado"))?;
        Ok(())
    }

    async fn create_server(&self) -> Result<()> {
        info!("Starting the server...");

        let state = APP_STATE.get().expect("App state not initialized").clone();

        let server = HttpServer::new(move || {
            App::new()
                // Inyectar el estado con cosas como el cliente SMTP
                .app_data(state.clone())
                .wrap(Logger::default())

                .service(
                    web::scope("/api/v1")
                        .service(controllers::email::routes())
                )
        });

        info!("Listening on http://{}", Config::get_addrs());
        server.bind(Config::get_addrs())?.run().await?;
        Ok(())
    }
}

#[actix_web::main]
async fn main() -> Result<()> {
    AppServer.start().await
}