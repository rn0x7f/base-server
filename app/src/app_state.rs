// app_state.rs
use email_sender::EmailSender;
use anyhow::Result;
use crate::rate_limiter::RateLimiter;

pub struct AppState {
    pub email_sender: EmailSender,
    pub rate_limiter: RateLimiter,
}

impl AppState {
    pub async fn new(email_sender: EmailSender) -> Result<Self, anyhow::Error> {
        let rate_limiter = RateLimiter::new();
        
        // Iniciar la limpieza automática de IPs expiradas
        rate_limiter.start_auto_cleanup().await;
        
        Ok(AppState {
            email_sender,
            rate_limiter,
        })
    }
}