use std::{process::Command, sync::Once};
use tracing_subscriber::filter::{EnvFilter, LevelFilter};
use tracing_subscriber::fmt::format::FmtSpan;

// spawn a command with SHELL
pub fn spawn_command(command: &str) {
    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg(command);
    cmd.spawn().expect("failed to execute process");
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
