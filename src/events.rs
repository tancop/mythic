use bevy::{input::keyboard::Key, input_focus::InputFocus, prelude::*};

#[derive(EntityEvent)]
pub struct EnterPressed(Entity);

pub fn on_keyboard_input(focus: Res<InputFocus>, input: Res<ButtonInput<Key>>, mut cmd: Commands) {
    if input.just_pressed(Key::Enter)
        && let Some(entity) = focus.get()
    {
        cmd.entity(entity).trigger(EnterPressed);
    }
}
