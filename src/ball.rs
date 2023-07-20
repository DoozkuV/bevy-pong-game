use super::paddle::{Paddle, PADDLE_HEIGHT, PADDLE_WIDTH};
use super::score::{Score, ScoreChanged};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::window::PrimaryWindow;

const BALL_SIZE: f32 = 30.;
const BALL_DEFAULT_SPEED: f32 = 500.;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ball_movement);
    }
}

#[derive(Component)]
pub struct Ball {
    x_velocity: f32,
    y_velocity: f32,
}

impl Default for Ball {
    fn default() -> Self {
        Ball {
            x_velocity: BALL_DEFAULT_SPEED,
            y_velocity: BALL_DEFAULT_SPEED,
        }
    }
}

fn ball_movement(
    time: Res<Time>, // For movement calculations
    mut ball_query: Query<(&mut Transform, &mut Ball)>,
    // For hitbox calculations
    paddle_query: Query<&Transform, (With<Paddle>, Without<Ball>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    // Manipulating the score
    score_query: Query<&Score>,
    mut score_event: EventWriter<ScoreChanged>,
) {
    // Unwrap the queries
    let window = window_query
        .get_single()
        .expect("Only one primary window should exist!");
    let score = score_query
        .get_single()
        .expect("Only one score object should exist at a time!");

    for (mut transform, mut ball) in ball_query.iter_mut() {
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
        if translation.y < y_min || translation.y > y_max {
            ball.y_velocity *= -1.0;
        }

        if translation.x < x_min {
            ball.x_velocity *= -1.0;
            score_event.send(ScoreChanged(Score {
                left_score: score.left_score + 1,
                right_score: score.right_score,
            }));
        } else if translation.x > x_max {
            ball.x_velocity *= -1.0;
            score_event.send(ScoreChanged(Score {
                left_score: score.left_score,
                right_score: score.right_score + 1,
            }));
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
                ball.x_velocity *= -1.0;
            }
        }

        // Begin to move the ball
        translation.x += ball.x_velocity * time.delta_seconds();
        translation.y += ball.y_velocity * time.delta_seconds();
    }
}
