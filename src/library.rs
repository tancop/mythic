use bevy::prelude::*;

use crate::{
    auth::EpicToken,
    widgets::{label, layout, text},
};

pub fn library_ui() -> impl Scene {
    bsn! {
        layout()

        Children [
            label("Your secret token:"),

            text(|ctx| ctx.resource::<EpicToken>().0.clone())
        ]
    }
}
