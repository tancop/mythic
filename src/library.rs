use std::time::Duration;

use async_compat::Compat;
use bevy::{prelude::*, tasks::IoTaskPool};

use crate::{
    CurrentPage, HttpClient,
    auth::EpicTokens,
    epic,
    events::listen,
    widgets::{button, layout, text},
};

use async_io::Timer;

#[derive(Resource)]
pub struct Library {
    pub items: Vec<epic::LibraryItem>,
}

#[derive(Event)]
pub struct LibraryUpdated;

pub fn load_library(client: Res<HttpClient>, tokens: Res<EpicTokens>, mut cmd: Commands) {
    IoTaskPool::get().scope(|s| {
        let access_token = tokens.access_token.clone();

        s.spawn(async move {
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
            })
            listen(|event: On<LibraryUpdated>, lib: Res<Library>, mut text: Query<&mut Text>| {
                log::info!("Library updated: {} items", lib.items.len());

                if let Ok(mut text) = text.get_mut(event.observer()) {
                    text.0 = format!("{} items in library", lib.items.len());
                }
            }),

            button("Reload")
            on(|_: On<Pointer<Press>>, mut cmd: Commands| {
                cmd.run_system_cached(load_library);
            })
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
