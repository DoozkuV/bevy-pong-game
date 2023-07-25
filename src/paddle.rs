use crate::{UI_HEIGHT, WINDOW_HEIGHT};

use super::ball::Ball;
use bevy::prelude::*;

const PADDLE_SPEED: f32 = 500.;
pub const PADDLE_HEIGHT: f32 = 120.;
pub const PADDLE_WIDTH: f32 = 17.;
const AI_SPEED_MODIFIER: f32 = 1.;

const INPUT_UP: KeyCode = KeyCode::W;
const INPUT_DOWN: KeyCode = KeyCode::S;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (player_control, computer_control));
    }
}
#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Computer;

fn player_control(
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut transform = player_query
        .get_single_mut()
        .expect("Only one player has been implemented yet!");

    if keyboard_input.pressed(INPUT_UP) {
        move_paddle(&mut transform, 1.0, &time);
    }
    if keyboard_input.pressed(INPUT_DOWN) {
        move_paddle(&mut transform, -1.0, &time);
    }
}

fn computer_control(
    ball_query: Query<&Transform, With<Ball>>,
    mut paddle_query: Query<&mut Transform, (With<Computer>, Without<Ball>)>,
    time: Res<Time>,
) {
    // Extract out the variables
    let ball_y_pos = ball_query
        .get_single()
        .expect("Only one ball has been implemented yet!")
        .translation
        .y;

    // Loop over every computer found
    for mut transform in paddle_query.iter_mut() {
        // Move the paddle towards the ball
        if ball_y_pos > transform.translation.y {
            move_paddle(&mut transform, AI_SPEED_MODIFIER, &time);
        } else {
            move_paddle(&mut transform, -AI_SPEED_MODIFIER, &time);
        }
    }
}

fn move_paddle(transform: &mut Transform, multiplier: f32, time: &Time) {
    let half_paddle_height = PADDLE_HEIGHT / 2.0;
    let vertical_border = WINDOW_HEIGHT / 2.0;
    let y_min = -vertical_border + half_paddle_height;
    // Make sure the paddle cannot enter in the UI's domain
    let y_max = vertical_border - UI_HEIGHT - half_paddle_height;

    let y_pos = &mut transform.translation.y;

    *y_pos = (*y_pos + PADDLE_SPEED * multiplier * time.delta_seconds()).clamp(y_min, y_max);
}
