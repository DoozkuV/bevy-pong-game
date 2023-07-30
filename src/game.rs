use bevy::prelude::*;

use crate::ball::Ball;
use crate::menu::MenuData;
use crate::paddle::{Controller, Paddle, PADDLE_WIDTH};
use crate::score::Score;
use crate::{AppState, WINDOW_WIDTH};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), setup_game);
    }
}

fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>, menu_data: Res<MenuData>) {
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
        if menu_data.is_single_player {
            Controller::Computer
        } else {
            Controller::Player {
                input_up: KeyCode::W,
                input_down: KeyCode::S,
            }
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
            input_up: KeyCode::Up,
            input_down: KeyCode::Down,
        },
    ));

    // Initialize a score of 0,0
    commands.spawn(Score::default());

    commands.remove_resource::<MenuData>();
}
