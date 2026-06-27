use async_compat::Compat;
use bevy::{prelude::*, tasks::IoTaskPool, text::EditableText};

use crate::{
    CurrentPage, HttpClient, epic,
    events::EnterPressed,
    library::library_ui,
    widgets::{label, layout, text_field},
};

#[derive(Resource)]
struct EpicToken(String);

fn handle_auth(
    event: On<EnterPressed>,
    query: Query<&EditableText>,
    mut cmd: Commands,
    client: Res<HttpClient>,
    mut page: ResMut<CurrentPage>,
) {
    let editor = match query.get(event.event_target()) {
        Ok(editor) => editor,
        Err(e) => {
            log::error!("Error getting text editor: {e}");
            return;
        }
    };
    let value = editor.value().to_string();

    IoTaskPool::get().scope(|s| {
        s.spawn(async {
            let res = match Compat::new(epic::authenticate(&client, &value)).await {
                Ok(res) => res,
                Err(e) => {
                    log::error!("Auth failed: {e}");
                    return;
                }
            };

            log::info!("Auth successful");
            cmd.insert_resource(EpicToken(res.access_token.clone()));

            if let Err(e) = page.replace(library_ui(&res.access_token), &mut cmd) {
                log::error!("Failed to replace page: {e}");
            }
        })
    });
}

pub fn enter_token_ui() -> impl Scene {
    bsn! {
        layout()

        Children [
            label("Paste authorizationCode here")
            Label,

            text_field()
            on(handle_auth)
        ]
    }
}
