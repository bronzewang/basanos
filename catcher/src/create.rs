use std::process::Stdio;

use eyre::Context;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::{ChildStderr, ChildStdout, Command},
};
use tokio_stream::{StreamExt, wrappers::LinesStream};

use crate::{config::Config, recipe::Node, ticket::Ticket};

#[derive(Clone)]
pub(crate) struct Create<'a> {
    pub(crate) config: &'a Config,
    pub(crate) ticket: &'a Ticket,
}

impl<'a> Create<'a> {
    pub(crate) async fn build(self, node: &Node) -> impl Future<Output = eyre::Result<()>> {
        // 准备环境，比如下载源码

        // 创建task future
        let task = async move { self.build_inner(node).await };
        task
    }

    async fn build_inner(self, node: &Node) -> eyre::Result<()> {
        tracing::info!("{:#?}", self.config);
        tracing::info!("{:#?}", self.ticket);
        let cmd = node.create.cmd.clone();
        let dir = self.ticket.path.clone();
        let task = tokio::spawn(async move {
            let mut cmd_split = cmd.split_whitespace();
            let mut cmd = Command::new(cmd_split.next().ok_or(eyre::eyre!("no program"))?);
            cmd.args(cmd_split);
            cmd.current_dir(dir);
            cmd.stdout(Stdio::piped());
            cmd.stderr(Stdio::piped());
            tracing::info!("{cmd:#?}");
            let mut child = cmd.spawn().context("failed cmd spawn")?;
            let child_stdout = child.stdout.take().expect("none child stdout");
            let child_stderr = child.stderr.take().expect("none child stderr");
            tokio::spawn(async move {
                forward_child_stdio(child_stdout, child_stderr).await;
            });
            let child_status = child.wait().await.context("wait")?;
            if child_status.success() {
                return Err(eyre::eyre!("child wait failed"));
            }
            Ok(())
        });

        let output = task.await?;
        tracing::info!("Id {:?} {output:#?}", self.ticket.id);
        Ok(())
    }
}

async fn forward_child_stdio(stdout: ChildStdout, stderr: ChildStderr) {
    let mut merged = LinesStream::new(BufReader::new(stdout).lines())
        .merge(LinesStream::new(BufReader::new(stderr).lines()));

    while let Some(line) = merged.next().await {
        tracing::warn!("{line:#?}");
    }
}
