use crate::command::Executable;

#[derive(clap::Args)]
pub(crate) struct TouchArgs {}

impl Executable for TouchArgs {
    fn execute(self) -> color_eyre::Result<()> {
        todo!()
    }
}
