use bevy::{
    animation::animated_field, ecs::system::IntoObserverSystem, prelude::*,
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

#[derive(Resource, Default)]
pub struct ButtonAnim(fx::Library);

impl fx::GetLibrary for ButtonAnim {
    fn get_library(&self) -> &fx::Library {
        &self.0
    }
}

pub fn add_button_animations(
    mut res: ResMut<ButtonAnim>,

    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut clips: ResMut<Assets<AnimationClip>>,
) {
    let btn_name = Name::new("Button");

    let library = fx::Library::new()
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
        )
        .build(graphs.as_mut(), clips.as_mut())
        .unwrap();

    res.0 = library;
}

pub fn button<I, B, M>(msg: &str, on_press: I) -> impl Scene
where
    I: IntoObserverSystem<Pointer<Press>, B, M> + Send + Sync + Clone,
    B: Bundle,
    M: 'static,
{
    let btn_name = Name::new("Button");

    bsn! {
        label(msg)
        Node {
            border: UiRect::all(Val::Px(1.0)),
            padding: UiRect::px(4.0, 4.0, 2.0, 2.0),
        }
        BorderColor::all(Color::BLACK)
        Button

        UiTransform
        fx::player::<ButtonAnim>()
        fx::target(btn_name)

        on(on_press)
        on(|event: On<Pointer<Enter>>, mut query: Query<&mut AnimationPlayer>, res: Res<ButtonAnim>| {
            let mut player = query.get_mut(event.entity).unwrap();

            player.stop_all();
            player.play(res.0.get_index("enter").unwrap());
        })
        on(|event: On<Pointer<Leave>>, mut query: Query<&mut AnimationPlayer>, res: Res<ButtonAnim>| {
            let mut player = query.get_mut(event.entity).unwrap();

            player.stop_all();
            player.play(res.0.get_index("leave").unwrap());
        })
    }
}
