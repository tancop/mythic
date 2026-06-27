use bevy::prelude::*;

use crate::widgets::{label, layout};

pub fn library_ui(token: &str) -> impl Scene {
    bsn! {
        layout()

        Children [
            label("Your secret token:"),

            label(token)
        ]
    }
}
