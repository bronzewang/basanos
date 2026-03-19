use crate::{command::Executable, config::Config};

#[derive(clap::Args)]
pub(crate) struct TraceArgs {}

impl Executable for TraceArgs {
    async fn execute(self, _config: &Config) -> color_eyre::Result<()> {
        todo!()
    }
}
