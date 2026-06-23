use bevy::{ecs::system::IntoObserverSystem, prelude::*, text::FontSourceTemplate};

pub fn layout() -> impl Scene {
    bsn! {
        Node {
            height: percent(100.0),
            width: percent(100.0),

            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,

            flex_direction: FlexDirection::Column,
            row_gap: px(10),
        }
        BackgroundColor(Color::WHITE)
    }
}

pub fn label(msg: &str) -> impl Scene {
    bsn! {
        Text(msg)
        TextFont {
            font: FontSourceTemplate::SystemUi,
            font_size: px(24),
        }
        TextColor(Color::BLACK)
    }
}

pub fn button<I, B, M>(msg: &str, on_press: I) -> impl Scene
where
    I: IntoObserverSystem<Pointer<Press>, B, M> + Send + Sync + Clone,
    B: Bundle,
    M: 'static,
{
    bsn! {
        label(msg)
        Node {
            border: UiRect::all(Val::Px(1.0)),
            padding: UiRect::px(4.0, 4.0, 2.0, 2.0),
        }
        BorderColor::all(Color::BLACK)
        Button
        on(on_press)
        on(|hover: On<Pointer<Enter>>, mut cmd: Commands| {
            if let Ok(mut cmd) = cmd.get_spawned_entity(hover.entity) {
                let xform = UiTransform::from_scale(Vec2::splat(1.1));
                cmd.insert(xform);
            }
        })
        on(|hover: On<Pointer<Leave>>, mut cmd: Commands| {
            if let Ok(mut cmd) = cmd.get_spawned_entity(hover.entity) {
                let xform = UiTransform::from_scale(Vec2::splat(1.0));
                cmd.insert(xform);
            }
        })
    }
}
