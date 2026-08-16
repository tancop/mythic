use std::sync::LazyLock;

use bevy::ecs::system::Commands;
use serde::{Deserialize, Serialize};

use crate::{auth, epic::AuthResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct Cache {
    pub tokens: Option<AuthResult>,
}

impl Cache {
    pub fn save(&self) -> anyhow::Result<()> {
        let path = get_cache_dir().join("cache.json");
        std::fs::create_dir_all(get_cache_dir())?;
        let contents = serde_json::to_string(self)?;
        std::fs::write(&path, contents)?;
        Ok(())
    }

    pub fn load() -> anyhow::Result<Self> {
        let path = get_cache_dir().join("cache.json");
        let Ok(contents) = std::fs::read_to_string(&path) else {
            return Err(anyhow::anyhow!("Failed to read cache file"));
        };
        serde_json::from_str(&contents).map_err(|e| anyhow::anyhow!(e))
    }
}

static CACHE_DIR: LazyLock<std::path::PathBuf> = LazyLock::new(|| {
    let mut dir = dirs::cache_dir().unwrap();
    dir.push("Mythic");
    dir
});

pub fn get_cache_dir() -> &'static std::path::PathBuf {
    &CACHE_DIR
}

pub fn load_cache(mut cmd: Commands) {
    let Ok(cache) = Cache::load() else {
        return;
    };

    if let Some(tokens) = cache.tokens {
        cmd.insert_resource(auth::EpicToken(tokens.access_token));
    }
}
