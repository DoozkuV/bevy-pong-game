use super::paddle::{Paddle, PADDLE_HEIGHT, PADDLE_WIDTH};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::window::PrimaryWindow;

const BALL_SIZE: f32 = 30.;
pub const BALL_SPEED: f32 = 500.;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ball_movement);
    }
}

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Speed {
    pub x_speed: f32,
    pub y_speed: f32,
}

fn ball_movement(
    time: Res<Time>,
    mut ball_query: Query<(&mut Transform, &mut Speed), With<Ball>>,
    paddle_query: Query<&Transform, (With<Paddle>, Without<Ball>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // Collision handling
    let window = window_query
        .get_single()
        .expect("Only one primary window should exist!");

    for (mut transform, mut speed) in ball_query.iter_mut() {
        let mut translation = &mut transform.translation;

        // Border collision handling
        let half_ball_size = BALL_SIZE / 2.0;
        let horizontal_border = window.width() / 2.0;
        let vertical_border = window.height() / 2.0;
        let x_min = -horizontal_border + half_ball_size;
        let x_max = horizontal_border - half_ball_size;
        let y_min = -vertical_border + half_ball_size;
        let y_max = vertical_border - half_ball_size;

        // Check if the player is out of bounds, reverse velocity if so
        if translation.x < x_min || translation.x > x_max {
            speed.x_speed *= -1.0;
        }

        // TODO - Implement gaining points and resetting ball position upon getting a point.
        if translation.y < y_min || translation.y > y_max {
            speed.y_speed *= -1.0;
        }

        // Check for collisions with paddles
        for paddle_transform in paddle_query.into_iter() {
            if collide(
                *translation,
                Vec2 {
                    x: BALL_SIZE,
                    y: BALL_SIZE,
                },
                paddle_transform.translation,
                Vec2 {
                    x: PADDLE_WIDTH,
                    y: PADDLE_HEIGHT,
                },
            )
            .is_some()
            {
                speed.x_speed *= -1.0;
            }
        }

        // Begin to move the ball
        translation.x += speed.x_speed * time.delta_seconds();
        translation.y += speed.y_speed * time.delta_seconds();
    }
}
