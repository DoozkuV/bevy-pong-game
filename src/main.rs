use bevy::prelude::*;
use bevy::window::PrimaryWindow;

mod ball;
use ball::{Ball, BallPlugin};

mod paddle;
use paddle::{Computer, Paddle, PaddlePlugin, Player, PADDLE_WIDTH};

mod score;
use score::{Score, ScorePlugin};
mod ui;
use ui::UiPlugin;

fn main() {
    App::new()
        .add_plugins((
            // Set the resolution
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (802., 502.).into(),
                    resizable: false,
                    title: "Pong".to_string(),
                    ..default()
                }),
                ..default()
            }),
            BallPlugin,
            PaddlePlugin,
            ScorePlugin,
            UiPlugin,
        ))
        .add_systems(Startup, (setup, ui::setup_ui))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    let window = window_query
        .get_single()
        .expect("Only one primary window should exist!");

    commands.spawn(Camera2dBundle::default());

    // Spawn the ball
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/Ball.png"),
            // transform: Transform::IDENTITY, // Object should be centered
            ..default()
        },
        // Spawns a ball with default speed
        Ball::default(),
    ));

    // Spawn the left-most paddle
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/Computer.png"),
            transform: Transform::from_xyz((-window.width() / 2.0) + PADDLE_WIDTH, 0.0, 1.0),
            ..default()
        },
        Paddle,
        Computer,
    ));

    // Spawn the right-most paddle
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/Player.png"),
            transform: Transform::from_xyz((window.width() / 2.0) - PADDLE_WIDTH, 0.0, 1.0),
            ..default()
        },
        Paddle,
        Player,
    ));
    // Initialize a score of 0,0
    commands.spawn(Score::default());
}
