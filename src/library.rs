use bevy::prelude::*;

use crate::{
    auth::EpicTokens,
    widgets::{label, layout, text},
};

pub fn library_ui() -> impl Scene {
    bsn! {
        layout()

        Children [
            label("Your secret token:"),

            text(|ctx| ctx.resource::<EpicTokens>().access_token.clone())
        ]
    }
}
