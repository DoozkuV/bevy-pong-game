use bevy::prelude::*;

mod menu;

mod ball;
use ball::{Ball, BallPlugin};

mod paddle;
use paddle::{Controller, Paddle, PaddlePlugin, PADDLE_WIDTH};

mod score;
use score::{Score, ScorePlugin};
mod ui;
use ui::UiPlugin;

// Consts to define the resolution of the game window in pixels
pub const WINDOW_WIDTH: f32 = 802.;
pub const WINDOW_HEIGHT: f32 = 455.;
// Defines the pixel height of the top UI Scorebar
pub const UI_HEIGHT: f32 = 47.;
// Main font to be used
pub const MAIN_FONT: &str = "fonts/Teko-Regular.ttf";

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    End,
}

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
        .add_state::<AppState>()
        .add_systems(Startup, menu::setup_menu)
        .add_systems(OnEnter(AppState::Game), setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Spawn the ball
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/Ball.png"),
            ..default()
        },
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
        if false {
            Controller::Player {
                input_up: KeyCode::W,
                input_down: KeyCode::S,
            }
        } else {
            Controller::Computer
        },
    ));

    // Spawn the right-most paddle
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/Player.png"),
            transform: Transform::from_xyz((WINDOW_WIDTH / 2.0) - PADDLE_WIDTH, 0.0, 1.0),
            ..default()
        },
        Paddle,
        Controller::Player {
            input_up: KeyCode::W,
            input_down: KeyCode::S,
        },
    ));

    // Initialize a score of 0,0
    commands.spawn(Score::default());
}
