//! 管理git仓库过程,尤其submodule

use crate::{command::Executable, config::Config};

#[derive(clap::Args)]
pub(crate) struct GitieArgs {}

impl Executable for GitieArgs {
    async fn execute(self, _config: &Config) -> color_eyre::Result<()> {
        todo!()
    }
}
