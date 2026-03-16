mod build;
mod store;
mod touch;
mod trace;

#[derive(clap::Parser)]
pub(crate) struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
pub(crate) enum Command {
    Build(build::BuildArgs),
    Store(store::StoreArgs),
    Touch(touch::TouchArgs),
    Trace(trace::TraceArgs),
}

pub(crate) trait Executable {
    fn execute(self) -> color_eyre::Result<()>;
}

pub(crate) async fn execute(cli: Cli) -> color_eyre::Result<()> {
    match cli.command {
        Command::Build(args) => args.execute(),
        Command::Store(args) => args.execute(),
        Command::Touch(args) => args.execute(),
        Command::Trace(args) => args.execute(),
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
