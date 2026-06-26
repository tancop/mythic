use bevy::winit::WinitSettings;
use bevy::{ecs::entity::EntityNotSpawnedError, prelude::*};

mod decode;
mod epic;
mod fx;
mod scope;
mod widgets;

mod enter_token;
mod login;

fn main() {
    App::new()
        .insert_resource(WinitSettings::desktop_app())
        .init_resource::<widgets::ButtonAnim>()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (
                spawn_camera,
                login::show_login,
                widgets::add_button_animations,
            ),
        )
        .run();
}

fn spawn_camera(world: &mut World) {
    world.spawn(Camera2d);
}

#[derive(Resource)]
struct CurrentPage(Entity);

impl CurrentPage {
    fn replace(
        &mut self,
        new: impl Scene,
        cmd: &mut Commands,
    ) -> Result<(), EntityNotSpawnedError> {
        let new_page = cmd.spawn_scene(new).id();

        cmd.get_spawned_entity(self.0).map(|mut cmd| {
            cmd.despawn();
            self.0 = new_page;
        })
    }
}
