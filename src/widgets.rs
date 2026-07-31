use bevy::{
    animation::animated_field,
    prelude::*,
    text::{EditableText, FontCx, FontSourceTemplate},
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
            font: FontSourceTemplate::SansSerif,
            weight: FontWeight::MEDIUM,
            font_size: px(16),
        }
        TextColor(Color::BLACK)
    }
}

pub fn title(msg: &str) -> impl Scene {
    bsn! {
        Text(msg)
        TextFont {
            font: FontSourceTemplate::SansSerif,
            weight: FontWeight::BOLD,
            font_size: px(24),
        }
        TextColor(Color::BLACK)
    }
}

#[derive(Resource, Default)]
pub struct ButtonAnim {
    library: fx::Library,
    enter: fx::AnimationHandle,
    leave: fx::AnimationHandle,
}

impl fx::GetLibrary for ButtonAnim {
    fn get_library(&self) -> &fx::Library {
        &self.library
    }
}

pub fn add_button_animations(
    mut res: ResMut<ButtonAnim>,

    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut clips: ResMut<Assets<AnimationClip>>,
) {
    let btn_name = Name::new("Button");

    let mut library = fx::Library::new();

    let enter = library.add(fx::single(
        &btn_name,
        animated_field!(UiTransform::scale),
        [0.0, 0.05],
        [1.0, 1.1].map(Vec2::splat),
    ));

    let leave = library.add(fx::single(
        &btn_name,
        animated_field!(UiTransform::scale),
        [0.0, 0.1],
        [1.1, 1.0].map(Vec2::splat),
    ));

    res.library = library.build(graphs.as_mut(), clips.as_mut()).unwrap();
    res.enter = enter;
    res.leave = leave;
}

pub fn button(msg: &str) -> impl Scene {
    let btn_name = Name::new("Button");

    bsn! {
        label(msg)
        Node {
            border: UiRect::all(Val::Px(2.0)),
            padding: UiRect::px(8.0, 8.0, 4.0, 4.0),
        }
        BorderColor::all(Color::BLACK)
        Button

        UiTransform
        fx::player::<ButtonAnim>()
        fx::target(btn_name)

        on(|event: On<Pointer<Enter>>, mut query: Query<&mut AnimationPlayer>, res: Res<ButtonAnim>| {
            let mut player = query.get_mut(event.entity).unwrap();

            player.stop_all();
            res.library.play(&mut player, res.enter);
        })
        on(|event: On<Pointer<Leave>>, mut query: Query<&mut AnimationPlayer>, res: Res<ButtonAnim>| {
            let mut player = query.get_mut(event.entity).unwrap();

            player.stop_all();
            res.library.play(&mut player, res.leave);
        })
    }
}

pub fn text_field() -> impl Scene {
    bsn! {
        TextFont {
            font: FontSourceTemplate::SystemUi,
            font_size: px(24),
        }
        TextColor(Color::BLACK)
        Node {
            border: UiRect::all(Val::Px(1.0)),
            padding: UiRect::px(4.0, 4.0, 2.0, 2.0),
            max_width: px(400.0)
        }
        BorderColor::all(Color::BLACK)
        EditableText
    }
}

pub fn load_fonts(mut font_cx: ResMut<FontCx>, server: Res<AssetServer>) {
    let _ = server.load::<Font>("assets/Inter.ttf");
    font_cx.set_sans_serif_family("Inter");
}
