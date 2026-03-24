use eyre::Context;
use tokio::process::Command;

use crate::{recipe::Node, ticket::TicketId};
use std::path::PathBuf;

#[derive(Clone)]
pub(crate) struct Create {
    pub(crate) id: TicketId,
    pub(crate) path: PathBuf,
}

impl Create {
    pub(crate) async fn build(self, node: &Node) -> impl Future<Output = eyre::Result<()>> {
        // 准备环境，比如下载源码

        // 创建task future
        let task = async move { self.build_inner(node).await };

        task
    }

    async fn build_inner(self, node: &Node) -> eyre::Result<()> {
        let cmd = node.create.cmd.clone();
        let task = tokio::spawn(async move {
            let mut cmd_split = cmd.split_whitespace();
            // let xxx = cmd_split.next().ok_or(eyre!("no program"));
            // let cmd = Command::new(cmd_split.next().ok_or(yyy)?);
            let mut cmd = Command::new(cmd_split.next().ok_or(eyre::eyre!("no program"))?);
            cmd.args(cmd_split);
            // let xx = cmd.output().await.context("xxx")?;
            // tracing::info!("{xx:#?}");
            // Ok(())
            cmd.output().await.context("xxx")
        });
        // let task: JoinHandle<eyre::Result<()>> = tokio::spawn(async move {
        //     let mut cmd_split = cmd.split_whitespace();
        //     // let xxx = cmd_split.next().ok_or(eyre!("no program"));
        //     // let cmd = Command::new(cmd_split.next().ok_or(yyy)?);
        //     let cmd = Command::new(cmd_split.next().ok_or(eyre!("no program"))?);
        //     // let child = cmd.spawn();
        //     Ok(())
        // });
        let output = task.await?;
        tracing::info!("{output:#?}");
        Ok(())
    }
}
