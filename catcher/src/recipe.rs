#![allow(dead_code)]

use std::path::Path;

use color_eyre::eyre::Context;
use ron::from_str;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Recipe {
    pub(crate) id: String,
    pub(crate) name: Option<String>,
    pub(crate) note: Option<String>,
    pub(crate) nodes: Vec<Node>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Node {
    pub(crate) id: String,
    pub(crate) name: Option<String>,
    pub(crate) note: Option<String>,
    pub(crate) source: Source,
    pub(crate) create: Create,
    pub(crate) output: Output,
    pub(crate) target: Target,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Source {
    pub(crate) repo: SourceRepo,
}

#[derive(Debug, Deserialize)]
pub(crate) enum SourceRepo {
    Native(String),
}

#[derive(Debug, Deserialize)]
pub(crate) struct Create {
    pub(crate) cmd: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Output {
    pub(crate) path: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Target {
    pub(crate) path: String,
}

pub(crate) async fn load<P: AsRef<Path>>(path: P) -> color_eyre::Result<Recipe> {
    let contents = tokio::fs::read_to_string(path)
        .await
        .context("Failed recipe open")?;
    from_str(&contents).context("Failed read recipe")
}
