#![allow(dead_code)]

use std::path::PathBuf;

use eyre::Context;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    recipe: RecipeConfig,
}

#[derive(Debug, Deserialize)]
pub(crate) struct RecipeConfig {
    path: PathBuf,
}

impl Config {
    pub(crate) async fn load(path: Option<PathBuf>) -> color_eyre::Result<Self> {
        let mut builder = config::Config::builder();
        if let Some(path) = path {
            builder = builder.add_source(config::File::from(path));
        }
        builder = builder.add_source(
            glob::glob(".config/*")
                .context("glob config/*")?
                .map(|path| config::File::from(path.unwrap()))
                .collect::<Vec<_>>(),
        );
        if let Some(sys_path) = Self::get_system_config_glob() {
            builder = builder.add_source(
                glob::glob(&sys_path)
                    .context("glob")?
                    .map(|path| config::File::from(path.unwrap()))
                    .collect::<Vec<_>>(),
            );
        }
        let config = builder.build().context("Faild build config")?;
        config
            .try_deserialize()
            .context("Failed deserialize config")
    }

    pub(crate) fn get_system_config_glob() -> Option<String> {
        dirs::config_dir().map(|mut path| {
            path.push("basanos");
            path.push("*");
            path.to_string_lossy().into_owned()
        })
    }
}
