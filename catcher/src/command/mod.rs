use std::path::PathBuf;

use eyre::Context;

use crate::config::Config;

mod build;
mod gitie;
mod store;
mod touch;
mod trace;

#[derive(clap::Parser)]
pub(crate) struct Cli {
    config: Option<PathBuf>,
    #[clap(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
pub(crate) enum Command {
    Gitie(gitie::GitieArgs),
    Build(build::BuildArgs),
    Store(store::StoreArgs),
    Touch(touch::TouchArgs),
    Trace(trace::TraceArgs),
}

pub(crate) trait Executable {
    async fn execute(self, config: &Config) -> color_eyre::Result<()>;
}

pub(crate) async fn execute(cli: Cli) -> color_eyre::Result<()> {
    let config = crate::config::Config::load(cli.config)
        .await
        .context("Failed config load")?;
    tracing::error!("{:#?}", config);

    match cli.command {
        Command::Gitie(args) => args.execute(&config).await,
        Command::Build(args) => args.execute(&config).await,
        Command::Store(args) => args.execute(&config).await,
        Command::Touch(args) => args.execute(&config).await,
        Command::Trace(args) => args.execute(&config).await,
    }
}

// // use clap::{Args, Parser, Subcommand, ValueEnum};
// use clap::{Parser, Subcommand};

// #[derive(Debug, Parser)]
// #[command(about = "Catcher a image to target")]
// pub struct Cli {
//     #[clap(subcommand)]
//     command: Command,
// }

// // build
// // store
// // touch
// // trace

// #[derive(Debug, Subcommand)]
// pub enum Command {
//     /// Mock a model----模拟出一个终端及外部环境(虚拟)
//     #[command(arg_required_else_help = true)]
//     Mock {
//         #[clap(flatten)]
//         image_options: ImageOptions,
//     },

//     /// Fake a model----伪造出一个终端及外部环境(实物)
//     #[command(arg_required_else_help = true)]
//     Fake {
//         #[clap(flatten)]
//         image_options: ImageOptions,
//     },
// }

// #[derive(Debug, Parser)]
// pub struct ImageOptions {
//     /// The path to the ELF file to flash and run.
//     #[clap(index = 1, help = "The path to the image file to flash and run.")]
//     pub(crate) path: PathBuf,
// }
//
// let args = Cli::parse();

// match args.command {
//     Command::Mock { image_options } => {
//         println!("Mock {image_options:?}");
//     }
//     Command::Fake { image_options } => {
//         println!("Fake {image_options:?}");
//     }
// }
