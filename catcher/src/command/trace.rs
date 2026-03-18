use crate::command::Executable;

#[derive(clap::Args)]
pub(crate) struct TraceArgs {}

impl Executable for TraceArgs {
    async fn execute(self) -> color_eyre::Result<()> {
        todo!()
    }
}
