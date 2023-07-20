use bevy::prelude::*;
use bevy::window::PrimaryWindow;

mod ball;
use ball::{Ball, BallPlugin};

mod paddle;
use paddle::PADDLE_WIDTH;
use paddle::{Computer, Paddle, PaddlePlugin, Player};

mod score;
use score::{Score, ScorePlugin};
mod ui;
use ui::UiPlugin;

// const RIGHT_PADDLE_POS: Vec3 = Vec3::new(625.0, 0.0, 0.0);
// const LEFT_PADDLE_POS: Vec3 = Vec3::new(-625.0, 0.0, 0.0);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
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
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query
        .get_single()
        .expect("Only one primary window should exist!");

    commands.spawn(Camera2dBundle::default());
    // Spawn the ball
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/Ball.png"),
            transform: Transform::IDENTITY, // Object should be centered
            ..default()
        },
        // Spawns a ball with default speed
        Ball::default(),
    ));

    // Spawn the left-most paddle
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/Player.png"),
            transform: Transform::from_xyz((-window.width() / 2.0) + PADDLE_WIDTH, 0.0, 0.0),
            ..default()
        },
        Paddle,
        Player,
    ));

    // Spawn the right-most paddle
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/Computer.png"),
            transform: Transform::from_xyz((window.width() / 2.0) - PADDLE_WIDTH, 0.0, 0.0),
            ..default()
        },
        Paddle,
        Computer,
    ));
    // Initialize a score of 0,0
    commands.spawn(Score::default());
}
