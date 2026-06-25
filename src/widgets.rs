use bevy::{
    animation::{AnimationTargetId, animated_field},
    ecs::system::IntoObserverSystem,
    prelude::*,
    text::FontSourceTemplate,
};

use crate::fx;

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
    let btn_name = Name::new("Button");

    let animations = fx::Library::new()
        .add(
            "enter",
            fx::single(
                &btn_name,
                animated_field!(UiTransform::scale),
                [0.0, 0.05],
                [1.0, 1.1].map(Vec2::splat),
            ),
        )
        .add(
            "leave",
            fx::single(
                &btn_name,
                animated_field!(UiTransform::scale),
                [0.0, 0.1],
                [1.1, 1.0].map(Vec2::splat),
            ),
        );

    bsn! {
        label(msg)
        Node {
            border: UiRect::all(Val::Px(1.0)),
            padding: UiRect::px(4.0, 4.0, 2.0, 2.0),
        }
        BorderColor::all(Color::BLACK)
        Button

        UiTransform
        template(move |ctx| animations.build(ctx))
        fx::target(btn_name)

        on(on_press)
        on(|event: On<Pointer<Enter>>, mut query: Query<(&mut AnimationPlayer, &fx::Library)>| {
            let (mut player, library) = query.get_mut(event.entity).unwrap();
            player.stop_all();
            player.play(library.get_index("enter").unwrap());
        })
        on(|event: On<Pointer<Leave>>, mut query: Query<(&mut AnimationPlayer, &fx::Library)>| {
            let (mut player, library) = query.get_mut(event.entity).unwrap();
            player.stop_all();
            player.play(library.get_index("leave").unwrap());
        })
    }
}
