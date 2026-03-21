use crate::{recipe::Node, ticket::TicketId};
use std::path::PathBuf;

#[derive(Clone)]
pub(crate) struct Create {
    pub(crate) id: TicketId,
    pub(crate) path: PathBuf,
}

impl Create {
    pub(crate) async fn build_node(self, _node: &Node) -> eyre::Result<()> {
        Ok(())
    }
}
