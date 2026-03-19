use std::path::PathBuf;

use crate::{command::Executable, config::Config};

#[derive(clap::Args)]
pub(crate) struct BuildArgs {
    path: PathBuf,
}

impl Executable for BuildArgs {
    async fn execute(self, _config: &Config) -> color_eyre::Result<()> {
        let recipe = crate::recipe::load(self.path).await?;
        tracing::info!("load recipe {:#?}", recipe);
        Ok(())
    }
}
