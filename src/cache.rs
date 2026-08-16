use std::sync::LazyLock;

use bevy::ecs::system::Commands;
use serde::{Deserialize, Serialize};

use crate::{auth, epic::AuthResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct Cache {
    pub tokens: Option<AuthResult>,
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
    let path = get_cache_dir().join("cache.json");

    let Ok(contents) = std::fs::read_to_string(&path) else {
        return;
    };
    let cache: Cache = serde_json::from_str(&contents).unwrap();

    if let Some(tokens) = cache.tokens {
        cmd.insert_resource(auth::EpicToken(tokens.access_token));
    }
}
