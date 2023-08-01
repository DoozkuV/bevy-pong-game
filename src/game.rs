use bevy::prelude::*;

use crate::ball::Ball;
use crate::menu::MenuData;
use crate::paddle::{Paddle, PADDLE_WIDTH};
use crate::score::{Score, ScoreChanged};
use crate::{AppState, WINDOW_WIDTH};

pub struct GamePlugin;

const VICTORY_POINT_REQ: u32 = 10;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), setup_game)
            .add_systems(Update, check_for_win.run_if(in_state(AppState::Game)));
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
        if menu_data.is_single_player {
            Paddle::Computer
        } else {
            Paddle::Player {
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
        Paddle::Player {
            input_up: KeyCode::Up,
            input_down: KeyCode::Down,
        },
    ));

    // Initialize a score of 0,0
    commands.spawn(Score::default());

    commands.remove_resource::<MenuData>();
}

fn check_for_win(
    mut change_events: EventReader<ScoreChanged>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for event in change_events.iter() {
        if event.0.right_score >= VICTORY_POINT_REQ {
            println!("Right wins!");
            next_state.set(AppState::End);
        } else if event.0.left_score >= VICTORY_POINT_REQ {
            next_state.set(AppState::End);
            println!("Left wins!");
        }
    }
}
