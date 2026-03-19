#![allow(dead_code)]

use std::{path::PathBuf, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Ticket {
    level: Level,
    store: Store,
    recipe: PathBuf,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Level {
    #[default]
    Debug,
    Release,
}

pub enum LevelError {
    Unknown,
}
impl FromStr for Level {
    type Err = LevelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "debug" => Ok(Level::Debug),
            "release" => Ok(Level::Release),
            _ => Err(LevelError::Unknown),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Store {
    #[default]
    Native,
}

pub enum StoreError {
    Unknown,
}

impl FromStr for Store {
    type Err = StoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "native" => Ok(Store::Native),
            _ => Err(StoreError::Unknown),
        }
    }
}

impl Ticket {
    pub(crate) async fn init(
        recipe: PathBuf,
        level: Option<String>,
        store: Option<String>,
    ) -> Self {
        let mut ticket = Ticket::default();
        if let Some(level) = level
            && let Ok(level) = level.parse()
        {
            ticket.level = level;
        } else {
            ticket.level = Level::Debug;
        }
        if let Some(store) = store
            && let Ok(store) = store.parse()
        {
            ticket.store = store;
        } else {
            ticket.store = Store::Native;
        }
        ticket.recipe = recipe;
        ticket
    }

    pub(crate) async fn load(&self) -> Self {
        Ticket::default()
    }

    pub(crate) async fn save(&self) -> color_eyre::Result<()> {
        Ok(())
    }
}
