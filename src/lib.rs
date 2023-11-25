mod args;

use std::sync::Once;
use tracing_subscriber::filter::{EnvFilter, LevelFilter};
use tracing_subscriber::fmt::format::FmtSpan;

// Macro that takes in a string and spawns a command with that string
#[macro_export]
macro_rules! spawn_command {
    ($command:expr) => {
        Command::new("sh")
            .arg("-c")
            .arg($command)
            .spawn()
            .expect("Failed to execute command");
    };
}

/// Initialize logger
pub fn configure_logger() {
    static LOGGER: Once = Once::new();

    // Make sure this is only called once
    LOGGER.call_once(|| {
        // Configure logger definition from environment
        // define the RUST_LOG env variable to change logging
        let env = EnvFilter::builder()
            .with_default_directive(LevelFilter::INFO.into())
            .from_env_lossy();

        // This will install a global tracing collector with default formatting
        tracing_subscriber::fmt()
            .with_env_filter(env)
            .with_span_events(FmtSpan::CLOSE)
            .init();
    });
}
