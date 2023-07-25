use crate::UI_HEIGHT;

use super::score::ScoreChanged;
use bevy::prelude::*;

pub struct UiPlugin;

#[derive(Component)]
pub enum ScoreText {
    Left,
    Right,
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
            .add_systems(Update, update_score_text);
    }
}

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Teko-Regular.ttf");

    // Spawn the top-Scorebar
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(UI_HEIGHT),
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            z_index: ZIndex::Global(-1),
            background_color: Color::rgb_u8(0, 3, 11).into(),
            ..default()
        })
        .with_children(|parent| {
            // Left side UI Bar
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(341.0),
                            height: Val::Px(47.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        // A NodeBundle is transparent by default, so to to see
                        // the image we have to change its color to WHITE
                        background_color: Color::WHITE.into(),
                        ..default()
                    },
                    UiImage::new(asset_server.load("sprites/ScoreBar.png")),
                ))
                // Left Scoretext
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "0",
                            TextStyle {
                                font: font.clone(),
                                font_size: 60.0,
                                color: Color::WHITE,
                            },
                        ),
                        ScoreText::Left,
                    ));
                });
            // Right side UI Bar
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(341.0),
                            height: Val::Px(47.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::WHITE.into(),
                        ..default()
                    },
                    UiImage::new(asset_server.load("sprites/ScoreBar.png")).with_flip_x(),
                ))
                // Right Scoretext
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "0",
                            TextStyle {
                                font: font.clone(),
                                font_size: 60.0,
                                color: Color::WHITE,
                            },
                        ),
                        ScoreText::Right,
                    ));
                });
        });
    // The Board
    commands.spawn(SpriteBundle {
        texture: asset_server.load("sprites/Board.png"),
        transform: Transform::from_translation(Vec3 {
            y: -47.,
            ..default()
        }),
        ..default()
    });
}

fn update_score_text(
    mut text_query: Query<(&mut Text, &ScoreText)>,
    mut change_events: EventReader<ScoreChanged>,
) {
    for score_changed in change_events.iter() {
        let score = score_changed.0;
        for (mut text, score_text) in text_query.iter_mut() {
            match score_text {
                ScoreText::Right => text.sections[0].value = format!("{}", score.left_score),
                ScoreText::Left => text.sections[0].value = format!("{}", score.right_score),
            }
        }
    }
}
