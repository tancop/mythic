use bevy::prelude::*;

use crate::{
    CurrentPage,
    enter_token::enter_token_ui,
    epic,
    widgets::{button, label, layout},
};

pub fn login_ui() -> impl Scene {
    bsn! {
        layout()

        Children [
            label("Log in with Epic Games")
            Label,

            button("Open", |_: On<Pointer<Press>>, mut page: ResMut<CurrentPage>, mut cmd: Commands| {
                if let Err(e) = open::that(epic::get_auth_url()) {
                    log::error!("Error opening auth URL: {e}");
                    return;
                }

                if let Err(e) = page.replace(enter_token_ui(), &mut cmd) {
                    log::error!("Failed to replace page: {e}");
                }
            })
        ]
    }
}

pub fn show_login(world: &mut World) {
    match world.spawn_scene(login_ui()) {
        Ok(entity) => {
            let page = CurrentPage(entity.id());
            world.insert_resource(page);
        }
        Err(e) => {
            log::error!("Failed to spawn login page: {e}");
        }
    }
}
