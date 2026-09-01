use async_compat::Compat;
use bevy::{prelude::*, tasks::IoTaskPool, text::EditableText};

use crate::{
    CurrentPage, HttpClient, auth,
    cache::Cache,
    epic,
    events::EnterPressed,
    library::library_ui,
    widgets::{label, layout, text_field},
};

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
            cmd.insert_resource(auth::EpicTokens {
                access_token: res.access_token.clone(),
                refresh_token: res.refresh_token.clone(),
            });

            let cache = Cache { tokens: Some(res) };

            if let Err(e) = cache.save() {
                log::warn!("Failed to save cache: {e}");
            }

            if let Err(e) = page.replace(library_ui(), &mut cmd) {
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
