use crate::{command::Executable, config::Config};

#[derive(clap::Args)]
pub(crate) struct TouchArgs {}

impl Executable for TouchArgs {
    async fn execute(self, _config: &Config) -> color_eyre::Result<()> {
        todo!()
    }
}
