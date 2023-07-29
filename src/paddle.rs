use crate::{AppState, UI_HEIGHT, WINDOW_HEIGHT};

use super::ball::Ball;
use bevy::prelude::*;

const PADDLE_SPEED: f32 = 500.;
pub const PADDLE_HEIGHT: f32 = 120.;
pub const PADDLE_WIDTH: f32 = 17.;
const AI_SPEED_MODIFIER: f32 = 0.8;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, paddle_control.run_if(in_state(AppState::Game)));
    }
}
#[derive(Component)]
pub struct Paddle;

// #[derive(Component)]
// pub struct Player {
//     pub input_up: KeyCode,
//     pub input_down: KeyCode,
// }

#[derive(Component)]
pub enum Controller {
    Computer,
    Player {
        input_up: KeyCode,
        input_down: KeyCode,
    },
}

// #[derive(Component)]
// pub struct Computer;

// fn player_control(
//     mut player_query: Query<(&mut Transform, &Player)>,
//     keyboard_input: Res<Input<KeyCode>>,
//     time: Res<Time>,
// ) {
//     for (mut transform, player) in player_query.iter_mut() {
//         if keyboard_input.pressed(player.input_up) {
//             move_paddle(&mut transform, 1.0, &time);
//         }
//         if keyboard_input.pressed(player.input_down) {
//             move_paddle(&mut transform, -1.0, &time);
//         }
//     }
// }

fn paddle_control(
    mut paddle_query: Query<(&mut Transform, &Controller), Without<Ball>>,
    ball_query: Query<&Transform, With<Ball>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, controller) in paddle_query.iter_mut() {
        match *controller {
            Controller::Player {
                input_up,
                input_down,
            } => {
                if keyboard_input.pressed(input_up) {
                    move_paddle(&mut transform, 1.0, &time);
                }
                if keyboard_input.pressed(input_down) {
                    move_paddle(&mut transform, -1.0, &time);
                }
            }
            Controller::Computer => {
                let ball_y_pos = ball_query
                    .get_single()
                    .expect("Only one ball has been implemented yet!")
                    .translation
                    .y;

                if ball_y_pos > transform.translation.y {
                    move_paddle(&mut transform, AI_SPEED_MODIFIER, &time);
                } else {
                    move_paddle(&mut transform, -AI_SPEED_MODIFIER, &time);
                }
            }
        }
    }
}

// fn computer_control(
//     ball_query: Query<&Transform, With<Ball>>,
//     mut paddle_query: Query<(&mut Transform, &Controller), Without<Ball>>,
//     time: Res<Time>,
// ) {
//     // Extract out the variables
//     let ball_y_pos = ball_query
//         .get_single()
//         .expect("Only one ball has been implemented yet!")
//         .translation
//         .y;

//     // Loop over every computer found
//     for mut transform in paddle_query.iter_mut() {
//         // Move the paddle towards the ball
//         if ball_y_pos > transform.translation.y {
//             move_paddle(&mut transform, AI_SPEED_MODIFIER, &time);
//         } else {
//             move_paddle(&mut transform, -AI_SPEED_MODIFIER, &time);
//         }
//     }
// }

fn move_paddle(transform: &mut Transform, multiplier: f32, time: &Time) {
    let half_paddle_height = PADDLE_HEIGHT / 2.0;
    let vertical_border = WINDOW_HEIGHT / 2.0;
    let y_min = -vertical_border + half_paddle_height;
    // Make sure the paddle cannot enter in the UI's domain
    let y_max = vertical_border - UI_HEIGHT - half_paddle_height;

    let y_pos = &mut transform.translation.y;

    *y_pos = (*y_pos + PADDLE_SPEED * multiplier * time.delta_seconds()).clamp(y_min, y_max);
}
