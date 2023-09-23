use bevy::prelude::*;

use crate::ball::Ball;
use crate::menu::MenuData;
use crate::paddle::{Paddle, PADDLE_WIDTH};
use crate::score::{Score, ScoreChanged};
use crate::{AppState, WINDOW_WIDTH};

// Amount of points needed for one side to win
const VICTORY_POINT_REQ: u32 = 10;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), setup_game)
            .add_systems(Update, check_for_victory.run_if(in_state(AppState::Game)))
            .add_systems(OnExit(AppState::Game), cleanup_game);
    }
}

// Holds assets to be despawned as well as passing
// victory data to the next state.
#[derive(Resource)]
pub struct GameData {
    game_entities: Vec<Entity>,
    pub is_left_win: bool,
}

impl GameData {
    fn new(game_entities: Vec<Entity>) -> Self {
        GameData {
            game_entities,
            is_left_win: false,
        }
    }
}

fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>, menu_data: Res<MenuData>) {
    // Create a vector that stores all the spawned entities for teardown later
    let entities = vec![
        // Spawn the ball
        commands
            .spawn((
                SpriteBundle {
                    texture: asset_server.load("sprites/Ball.png"),
                    ..default()
                },
                Ball::default(),
            ))
            .id(),
        // Spawn the left-most paddle
        commands
            .spawn((
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
            ))
            .id(),
        // Spawn the right-most paddle
        commands
            .spawn((
                SpriteBundle {
                    texture: asset_server.load("sprites/Player.png"),
                    transform: Transform::from_xyz((WINDOW_WIDTH / 2.0) - PADDLE_WIDTH, 0.0, 1.0),
                    ..default()
                },
                Paddle::Player {
                    input_up: KeyCode::Up,
                    input_down: KeyCode::Down,
                },
            ))
            .id(),
        // Initialize a score of 0,0
        commands.spawn(Score::default()).id(),
    ];

    commands.remove_resource::<MenuData>();
    commands.insert_resource(GameData::new(entities));
}

fn check_for_victory(
    mut change_events: EventReader<ScoreChanged>,
    mut next_state: ResMut<NextState<AppState>>,
    mut game_data: ResMut<GameData>,
) {
    for event in change_events.iter() {
        if event.0.right_score >= VICTORY_POINT_REQ {
            game_data.is_left_win = false;
            next_state.set(AppState::End);
        } else if event.0.left_score >= VICTORY_POINT_REQ {
            game_data.is_left_win = true;
            next_state.set(AppState::End);
        }
    }
}

fn cleanup_game(mut commands: Commands, game_data: Res<GameData>) {
    for entitiy in game_data.game_entities.iter() {
        commands.entity(*entitiy).despawn();
    }
}
