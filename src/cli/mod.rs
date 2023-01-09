mod add;
mod commands;

use clap::Parser;
use commands::{parse_commands, Commands};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

pub fn init_cli() {
    let cli = Cli::parse();

    parse_commands(cli)
}
