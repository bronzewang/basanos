#![allow(dead_code)]

use std::{fs::File, path::Path};

use color_eyre::eyre::Context;
use ron::de::from_reader;
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
    pub(crate) casting: Casting,
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
pub(crate) struct Casting {
    cmd: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Output {
    path: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Target {
    path: String,
}

pub(crate) fn load<P: AsRef<Path>>(path: P) -> color_eyre::Result<Recipe> {
    let f = File::open(path).context("Failed recipe open")?;
    from_reader(f).context("Failed read recipe")
}
