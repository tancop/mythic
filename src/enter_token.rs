use bevy::prelude::*;

use crate::widgets::{label, layout};

pub fn enter_token_ui() -> impl Scene {
    bsn! {
        layout()

        Children [
            label("Paste authorizationCode here")
            Label
        ]
    }
}
