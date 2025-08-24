use crate::config::Config;
use tracing::Level;
use tracing_subscriber;

/// Configuration trait that defines the contract for application settings
/// Implementations should provide concrete values for server configuration
#[allow(dead_code)]
pub trait ApplicationConfig {
    fn get_addrs() -> String;

    fn get_max_level_log() -> Level;

    fn get_mode() -> &'static str;

    fn get_port() -> &'static str;

    fn get_host() -> &'static str;

    fn get_max_pool_size() -> u32;

    fn get_with_migrations() -> bool;
}

/// Main application trait that defines the lifecycle and behavior of the server
/// Provides default implementations for common initialization tasks
pub trait Application {
    /// Sets up logging infrastructure with tracing subscriber
    /// Configures log level and span events for debugging
    fn initialize_logging(&self) -> anyhow::Result<()> {
        env_logger::init();

        let subscriber = tracing_subscriber::fmt()
            .with_max_level(Config::get_max_level_log())
            .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
            .finish();

        tracing::subscriber::set_global_default(subscriber)?;
        Ok(())
    }

    /// Basic initialization that sets up logging
    /// Can be overridden to add more initialization steps
    fn initialize(&self) -> anyhow::Result<()> {
        self.initialize_logging()
    }

    /// Application-specific setup logic (must be implemented by concrete types)
    async fn setup(&self) -> anyhow::Result<()>;

    /// Server creation logic (must be implemented by concrete types)
    async fn create_server(&self) -> anyhow::Result<()>;

    /// Main entry point that orchestrates the complete application startup
    /// Loads environment variables, initializes logging, sets up the app, and starts the server
    async fn start(&self) -> anyhow::Result<()> {
        dotenv::dotenv().ok();
        self.initialize()?;
        self.setup().await?;
        self.create_server().await
    }
}