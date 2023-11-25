use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    /// Stop the service
    #[arg(long)]
    pub stop_service: bool,

    /// Start the service
    #[arg(long)]
    pub start_service: bool,

    /// Restart the service
    #[arg(long)]
    pub restart_service: bool,
}
