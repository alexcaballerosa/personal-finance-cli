use super::Cli;
use crate::cli::add::{add_data, Add};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Add new record (income/expense) to the database
    #[command(subcommand)]
    Add(Add),
}

pub fn parse_commands(cli: Cli) {
    match cli.command {
        Commands::Add(add) => add_data(add),
    }
}
