use clap::Parser;

pub mod command;
pub mod config;
pub mod recipe;
pub mod ticket;

pub async fn execute() -> color_eyre::Result<()> {
    let cli = crate::command::Cli::parse();
    crate::command::execute(cli).await
}
