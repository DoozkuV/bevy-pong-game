use std::time::Duration;

use crate::{AppState, MAIN_FONT, UI_HEIGHT};

use super::score::ScoreChanged;
use bevy::prelude::*;

pub struct UiPlugin;

#[derive(Component)]
pub enum ScoreText {
    Left,
    Right,
}

#[derive(Component)]
struct TimerText {
    timer: Timer,
    minutes: u32,
    seconds: u32,
}

impl TimerText {
    fn new() -> TimerText {
        TimerText {
            timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
            minutes: 0,
            seconds: 0,
        }
    }
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), setup_game_ui)
            .add_systems(
                Update,
                (update_score_text, update_timer).run_if(in_state(AppState::Game)),
            );
    }
}

pub fn setup_game_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load(MAIN_FONT);

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
            // Timer
            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new(
                        "0",
                        TextStyle {
                            font: font.clone(),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                    ),
                    TextSection::new(
                        ":",
                        TextStyle {
                            font: font.clone(),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                    ),
                    TextSection::new(
                        "00",
                        TextStyle {
                            font: font.clone(),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                    ),
                ]),
                TimerText::new(),
            ));
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
        let score = &score_changed.0;
        for (mut text, score_text) in text_query.iter_mut() {
            match score_text {
                ScoreText::Right => text.sections[0].value = format!("{}", score.left_score),
                ScoreText::Left => text.sections[0].value = format!("{}", score.right_score),
            }
        }
    }
}

fn update_timer(mut text_query: Query<(&mut Text, &mut TimerText)>, time: Res<Time>) {
    // Unwrap the timer query
    let (mut text, mut timer_text) = text_query.single_mut();
    // Tick the timer
    timer_text.timer.tick(time.delta());

    // Exit if the timer has not gone off
    if !timer_text.timer.finished() {
        return;
    }

    timer_text.seconds += 1;
    if timer_text.seconds == 60 {
        timer_text.seconds = 0;
        timer_text.minutes += 1;
    }
    text.sections[0].value = timer_text.minutes.to_string();
    text.sections[2].value = if timer_text.seconds <= 9 {
        format!("0{}", timer_text.seconds)
    } else {
        timer_text.seconds.to_string()
    }
}
