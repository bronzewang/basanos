use crate::command::Executable;

#[derive(clap::Args)]
pub(crate) struct BuildArgs {}

impl Executable for BuildArgs {
    fn execute(self) -> color_eyre::Result<()> {
        todo!()
    }
}
