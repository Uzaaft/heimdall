use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    /// Name of the person to greet
    #[arg(long)]
    pub stop_service: bool,

    #[arg(long)]
    pub start_service: bool,

    #[arg(long)]
    pub restart_service: bool,
}
