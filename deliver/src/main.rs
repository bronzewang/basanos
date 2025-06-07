#[derive(Debug, clap::Parser)]
#[command(about = "Deliver a image to target")]
pub struct Cli {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
    /// Fake a model
    #[command(arg_required_else_help = true)]
    Fake {
        #[clap(flatten)]
        pub(crate) image_options: ImageOptions,
    },
}

#[derive(Debug, clap::Parser)]
pub struct ImageOptions {
    /// The path to the ELF file to flash and run.
    #[clap(
        index = 1,
        help = "The path to the image file to flash and run."
    )]
    pub(crate) path: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    match args.subcommand {
        Subcommand::Fake { image_options } => {
            println!("Fake {image_options}");
        }
    }
}
