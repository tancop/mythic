use bevy::{prelude::*, text::EditableText};

use crate::{
    events::EnterPressed,
    widgets::{label, layout, text_field},
};

pub fn enter_token_ui() -> impl Scene {
    bsn! {
        layout()

        Children [
            label("Paste authorizationCode here")
            Label,

            text_field()
            on(|event: On<EnterPressed>, query: Query<&EditableText>| {
                match query.get(event.event_target()){
                    Ok(editor) => log::info!("value: {}", editor.value()),
                    Err(e) => log::error!("Error getting text editor: {e}"),
                }
            })
        ]
    }
}
