use bevy::{
    animation::animated_field,
    prelude::*,
    text::{EditableText, FontSourceTemplate},
};

use crate::{colors, fx};

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
        BackgroundColor(colors::BACKGROUND)
    }
}

pub fn label(msg: &str) -> impl Scene {
    bsn! {
        Text(msg)
        TextFont {
            font: FontSourceTemplate::Family("Inter"),
            weight: FontWeight::MEDIUM,
            font_size: px(16),
        }
        TextColor(colors::TEXT)
    }
}

pub fn title(msg: &str) -> impl Scene {
    bsn! {
        Text(msg)
        TextFont {
            font: FontSourceTemplate::Family("Inter"),
            weight: FontWeight::BOLD,
            font_size: px(24),
        }
        TextColor(colors::TEXT)
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
    let padding_x = Val::Px(16.0);
    let padding_y = Val::Px(8.0);

    bsn! {
        label(msg)
        Node {
            padding: UiRect::new(padding_x, padding_x, padding_y, padding_y),
            border_radius: BorderRadius::all(Val::Px(4.0)),
        }
        BackgroundColor(colors::ALT)
        TextColor(colors::TEXT)
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
            font: FontSourceTemplate::Family("Inter"),
            font_size: px(24),
        }
        TextColor(colors::TEXT)
        Node {
            padding: UiRect::px(4.0, 4.0, 2.0, 2.0),
            border_radius: BorderRadius::all(Val::Px(4.0)),
            max_width: px(400.0),
        }
        BackgroundColor(colors::ALT)
        EditableText
    }
}

#[derive(Resource)]
pub struct Fonts {
    pub inter: Handle<Font>,
}

pub fn load_fonts(server: Res<AssetServer>, mut cmd: Commands) {
    cmd.insert_resource(Fonts {
        inter: server.load::<Font>("Inter.ttf"),
    });
}
