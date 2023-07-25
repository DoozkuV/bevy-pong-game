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
pub const WINDOW_WIDTH: f32 = 802.;
pub const WINDOW_HEIGHT: f32 = 455.;
pub const UI_HEIGHT: f32 = 47.;

fn main() {
    App::new()
        .add_plugins((
            // Set the resolution
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                    resizable: false,
                    title: "Pong".to_string(),
                    ..default()
                }),
                ..default()
            }),
            // Set up the main game plugins
            BallPlugin,
            PaddlePlugin,
            ScorePlugin,
            UiPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    let new_window_height = WINDOW_HEIGHT - 47.;

    window_query
        .get_single_mut()
        .expect("Only one primary window should exist!")
        .resolution
        .set(WINDOW_WIDTH, new_window_height);

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
            transform: Transform::from_xyz((-WINDOW_WIDTH / 2.0) + PADDLE_WIDTH, 0.0, 1.0),
            ..default()
        },
        Paddle,
        Computer,
    ));

    // Spawn the right-most paddle
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/Player.png"),
            transform: Transform::from_xyz((WINDOW_WIDTH / 2.0) - PADDLE_WIDTH, 0.0, 1.0),
            ..default()
        },
        Paddle,
        Player {
            input_up: KeyCode::W,
            input_down: KeyCode::S,
        },
    ));
    // Initialize a score of 0,0
    commands.spawn(Score::default());
}
