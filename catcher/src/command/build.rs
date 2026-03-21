use std::{path::PathBuf, str::FromStr};

use eyre::Context;

use crate::{command::Executable, config::Config, create::Create, recipe::Recipe, ticket::Ticket};

#[derive(clap::Args)]
pub(crate) struct BuildArgs {
    recipe: PathBuf,
    level: Option<String>,
    store: Option<String>,
}

impl Executable for BuildArgs {
    async fn execute(self, config: &Config) -> eyre::Result<()> {
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

async fn build_execute(ticket: &mut Ticket, _config: &Config, recipe: &Recipe) -> eyre::Result<()> {
    let create = Create {
        id: ticket.id.clone(),
        path: PathBuf::from_str("").unwrap(),
    };

    let mut tasks = Vec::new();
    for node in recipe.nodes.iter() {
        let task = create.clone().build(node).await;
        tasks.push(task);
    }
    for task in tasks {
        task.await?;
    }

    Ok(())
}
