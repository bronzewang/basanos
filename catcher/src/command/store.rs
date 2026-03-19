use crate::{command::Executable, config::Config};

#[derive(clap::Args)]
pub(crate) struct StoreArgs {}

impl Executable for StoreArgs {
    async fn execute(self, _config: &Config) -> color_eyre::Result<()> {
        todo!()
    }
}
