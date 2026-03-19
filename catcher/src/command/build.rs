use std::path::PathBuf;

use eyre::Context;

use crate::{command::Executable, config::Config, recipe::Recipe, ticket::Ticket};

#[derive(clap::Args)]
pub(crate) struct BuildArgs {
    recipe: PathBuf,
    level: Option<String>,
    store: Option<String>,
}

impl Executable for BuildArgs {
    async fn execute(self, config: &Config) -> color_eyre::Result<()> {
        let recipe = crate::recipe::load(&self.recipe)
            .await
            .context("Failed load recipe")?;
        tracing::info!("{:#?}", recipe);
        let mut ticket = Ticket::init(self.recipe, self.level, self.store).await;
        tracing::info!("{ticket:#?}");
        build_execute(&mut ticket, &config, &recipe)
            .await
            .context("Faile build execute")?;
        ticket.save().await.context("Faile ticket save")
    }
}

async fn build_execute(
    _ticket: &mut Ticket,
    _config: &Config,
    _recipe: &Recipe,
) -> color_eyre::Result<()> {
    Ok(())
}
