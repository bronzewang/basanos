use crate::command::Executable;

#[derive(clap::Args)]
pub(crate) struct StoreArgs {}

impl Executable for StoreArgs {
    async fn execute(self) -> color_eyre::Result<()> {
        todo!()
    }
}
