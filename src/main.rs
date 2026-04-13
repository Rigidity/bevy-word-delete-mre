use bevy::{
    color::palettes::css,
    prelude::*,
    text::{EditableText, TextCursorStyle},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_ui)
        .run();
}

fn spawn_ui(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: px(32),
                ..default()
            },
            BackgroundColor(Color::BLACK),
        ))
        .with_children(|parent| {
            parent.spawn(labelled_container()).with_children(|parent| {
                parent.spawn(Text::new(
                    "RTL Embedding at beginning and Pop Directional Formatting at end",
                ));

                parent.spawn(editable_text("\u{202b}Hello, world!\u{202C}"));
            });

            parent.spawn(labelled_container()).with_children(|parent| {
                parent.spawn(Text::new("Only RTL Embedding at beginning"));

                parent.spawn(editable_text("\u{202b}Hello, world!"));
            });

            parent.spawn(labelled_container()).with_children(|parent| {
                parent.spawn(Text::new("Not RTL"));

                parent.spawn(editable_text("Hello, world!"));
            });
        });
}

fn labelled_container() -> impl Bundle {
    Node {
        flex_direction: FlexDirection::Column,
        row_gap: px(4),
        width: px(600),
        ..default()
    }
}

fn editable_text(text: &str) -> impl Bundle {
    (
        EditableText::new(text),
        TextFont::from_font_size(28.0),
        TextCursorStyle {
            color: css::WHITE.into(),
            selection_color: css::GRAY.into(),
            ..default()
        },
        Node {
            width: Val::Percent(100.0),
            height: px(48),
            border: UiRect::all(px(2)),
            border_radius: BorderRadius::all(px(8)),
            padding: UiRect::new(px(8), px(8), px(5), px(5)),
            ..default()
        },
        BorderColor::all(css::WHITE),
        BackgroundColor(css::BLACK.into()),
    )
}
