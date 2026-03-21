use crate::{recipe::Node, ticket::TicketId};
use std::path::PathBuf;

#[derive(Clone)]
pub(crate) struct Create {
    pub(crate) id: TicketId,
    pub(crate) path: PathBuf,
}

impl Create {
    pub(crate) async fn build(self, _node: &Node) -> impl Future<Output = eyre::Result<()>> {
        // 准备环境，比如下载源码

        // 创建task future
        let task = async move { Self::build_inner().await };

        task
    }

    async fn build_inner() -> eyre::Result<()> {
        Ok(())
    }
}
