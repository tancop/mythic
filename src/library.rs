use std::time::Duration;

use async_compat::Compat;
use bevy::{prelude::*, tasks::IoTaskPool};

use crate::{
    CurrentPage, HttpClient,
    auth::EpicTokens,
    epic,
    events::listen,
    widgets::{button, label, layout, text},
};

use async_io::Timer;

#[derive(Resource)]
pub struct Library {
    pub items: Vec<epic::LibraryItem>,
}

#[derive(Event)]
pub struct LibraryUpdated;

pub fn load_library(client: Res<HttpClient>, tokens: Res<EpicTokens>, mut cmd: Commands) {
    let client = client.as_ref().clone();
    let access_token = tokens.access_token.clone();
    IoTaskPool::get().spawn(async move {
        let result = Compat::new(epic::get_library_items(&client, &access_token)).await;

        match result {
            Ok(items) => {
                cmd.insert_resource(Library { items });
                cmd.trigger(LibraryUpdated);
            }
            Err(e) => {
                log::error!("Failed to load library: {}", e);
                Timer::after(Duration::from_secs(1)).await;
                cmd.run_system_cached(load_library);
            }
        }
    });
}

pub fn library_ui() -> impl Scene {
    bsn! {
        layout()

        Children [
            label("Loading...")
            listen(|event: On<LibraryUpdated>, mut cmd: Commands| {
                cmd.entity(event.observer()).despawn();
            }),
        ]

        listen(|event: On<LibraryUpdated>, lib: Res<Library>, mut cmd: Commands| {
            log::info!("Library updated: {} items", lib.items.len());

            for item in &lib.items {
                cmd.spawn_scene(label(&item.app_name.clone()))
                    .insert(ChildOf(event.observer()));
            }
        })
    }
}

pub fn show_library(world: &mut World) {
    match world.spawn_scene(library_ui()) {
        Ok(entity) => {
            let page = CurrentPage(entity.id());
            world.insert_resource(page);
        }
        Err(e) => {
            log::error!("Failed to spawn library page: {e}");
        }
    }

    world.run_system_cached(load_library).unwrap();
}
