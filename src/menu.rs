use bevy::prelude::*;

use crate::MAIN_FONT;

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::BLACK.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "PONG",
                TextStyle {
                    font: asset_server.load(MAIN_FONT),
                    font_size: 200.0,
                    color: Color::WHITE,
                },
            ));
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        width: Val::Percent(70.0),
                        justify_content: JustifyContent::SpaceAround,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(create_menu_button()).with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "One Player",
                            TextStyle {
                                font: asset_server.load(MAIN_FONT),
                                font_size: 40.,
                                color: Color::BLACK,
                            },
                        ));
                    });
                    parent.spawn(create_menu_button()).with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Two Player",
                            TextStyle {
                                font: asset_server.load(MAIN_FONT),
                                font_size: 40.,
                                color: Color::BLACK,
                            },
                        ));
                    });
                });
        });
}

fn create_menu_button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            width: Val::Px(150.),
            height: Val::Px(65.0),
            border: UiRect::all(Val::Px(5.0)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::rgb_u8(153, 48, 122).into(),
        border_color: BorderColor(Color::PURPLE),
        ..default()
    }
}
