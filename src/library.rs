use async_compat::Compat;
use bevy::{prelude::*, tasks::IoTaskPool};

use crate::{
    CurrentPage, HttpClient,
    auth::EpicTokens,
    epic,
    widgets::{label, layout, text},
};

#[derive(Resource)]
pub struct Library {
    pub items: Vec<epic::LibraryItem>,
}

pub fn load_library(client: Res<HttpClient>, tokens: Res<EpicTokens>, mut cmd: Commands) {
    IoTaskPool::get().scope(|s| {
        let access_token = tokens.access_token.clone();

        s.spawn(async move {
            let result = Compat::new(epic::get_library_items(&client, &access_token)).await;

            match result {
                Ok(items) => {
                    cmd.insert_resource(Library { items });
                }
                Err(e) => {
                    log::error!("Failed to load library: {}", e);
                }
            }
        });
    });
}

pub fn library_ui() -> impl Scene {
    bsn! {
        layout()

        Children [
            text(|ctx| {
                match ctx.entity.get_resource::<Library>().as_ref() {
                    None => "Loading...".to_string(),
                    Some(lib) => format!("{} items in library", lib.items.len()),
                }
            }),
        ]
    }
}

pub fn show_library(world: &mut World) {
    world.run_system_cached(load_library).unwrap();

    match world.spawn_scene(library_ui()) {
        Ok(entity) => {
            let page = CurrentPage(entity.id());
            world.insert_resource(page);
        }
        Err(e) => {
            log::error!("Failed to spawn library page: {e}");
        }
    }
}
