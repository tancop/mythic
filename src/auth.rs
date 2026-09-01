use async_compat::Compat;
use bevy::{
    ecs::{
        resource::Resource,
        system::{Commands, Res},
    },
    tasks::IoTaskPool,
};

use crate::{HttpClient, cache, epic, login};

#[derive(Resource)]
pub struct EpicTokens {
    pub access_token: String,
    pub refresh_token: String,
}

pub fn refresh_token(client: Res<HttpClient>, tokens: Res<EpicTokens>, mut cmd: Commands) {
    IoTaskPool::get().scope(|s| {
        s.spawn(async {
            let result = Compat::new(epic::refresh_token(&client, &tokens.refresh_token)).await;

            match result {
                Ok(res) => {
                    cmd.insert_resource(EpicTokens {
                        access_token: res.access_token.clone(),
                        refresh_token: res.refresh_token.clone(),
                    });

                    let cache = cache::Cache { tokens: Some(res) };

                    if let Err(e) = cache.save() {
                        log::warn!("Failed to save cache: {e}");
                    }
                }
                Err(e) => {
                    log::error!("Failed to refresh token: {}", e);
                    cmd.run_system_cached(login::show_login);
                }
            }
        });
    });
}
