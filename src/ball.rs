use crate::{AppState, UI_HEIGHT, WINDOW_HEIGHT, WINDOW_WIDTH};
use rand::{thread_rng, Rng};
use std::f32::consts::PI;

use super::paddle::{Paddle, PADDLE_HEIGHT, PADDLE_WIDTH};
use super::score::{Score, ScoreChanged};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

const BALL_SIZE: f32 = 30.;
const BALL_DEFAULT_SPEED: f32 = 800.;
const BALL_SERVE_MULTIPLIER: f32 = 0.65;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (ball_movement, serve_on_score_change).run_if(in_state(AppState::Game)),
        );
    }
}

#[derive(Component)]
pub struct Ball {
    velocity: Vec2,
    serve_left: bool,
}

impl Default for Ball {
    fn default() -> Self {
        let mut rng = thread_rng();
        Ball {
            serve_left: true,
            velocity: Vec2::from_angle(rng.gen_range((7.0 * PI) / 4.0..(9.0 * PI) / 4.0))
                * (BALL_DEFAULT_SPEED * BALL_SERVE_MULTIPLIER),
        }
    }
}

impl Ball {
    fn serve(&mut self) {
        let serve_modifier = if self.serve_left { 0.0 } else { PI };
        self.serve_left = !self.serve_left;

        let mut rng = thread_rng();
        self.velocity =
            Vec2::from_angle(rng.gen_range((7.0 * PI) / 4.0..(9.0 * PI) / 4.0) + serve_modifier)
                * (BALL_DEFAULT_SPEED * BALL_SERVE_MULTIPLIER);
    }
}

fn ball_movement(
    time: Res<Time>, // For movement calculations
    mut ball_query: Query<(&mut Transform, &mut Ball)>,
    // For hitbox calculations
    paddle_query: Query<&Transform, (With<Paddle>, Without<Ball>)>,
    // Manipulating the score
    score_query: Query<&Score>,
    mut score_event: EventWriter<ScoreChanged>,
) {
    let score = score_query
        .get_single()
        .expect("Only one score object should exist at a time!");

    for (mut ball_transform, mut ball) in ball_query.iter_mut() {
        let mut ball_translation = &mut ball_transform.translation;

        // Border collision handling
        let half_ball_size = BALL_SIZE / 2.0;
        let horizontal_border = WINDOW_WIDTH / 2.0;
        let vertical_border = WINDOW_HEIGHT / 2.0;
        let x_min = -horizontal_border + half_ball_size;
        let x_max = horizontal_border - half_ball_size;
        let y_min = -vertical_border + half_ball_size;
        let y_max = vertical_border - UI_HEIGHT - half_ball_size;

        // Check for collisions with the goals
        if ball_translation.x < x_min {
            ball.velocity.x *= -1.0;
            score_event.send(ScoreChanged(Score {
                right_score: score.right_score + 1,
                left_score: score.left_score,
            }));
        } else if ball_translation.x > x_max {
            ball.velocity.x *= -1.0;
            score_event.send(ScoreChanged(Score {
                right_score: score.right_score,
                left_score: score.left_score + 1,
            }));
        }

        // Check for collisions with paddles
        for paddle_transform in paddle_query.into_iter() {
            if collide(
                *ball_translation,
                Vec2::splat(BALL_SIZE),
                paddle_transform.translation,
                Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT),
            )
            .is_some()
            {
                // Generate the new launch angle using the line between the center of the paddle
                // and the ball, and setting that as the new speed of the ball
                ball.velocity = Vec2::new(
                    ball_translation.x - paddle_transform.translation.x,
                    ball_translation.y - paddle_transform.translation.y,
                )
                .normalize()
                    * BALL_DEFAULT_SPEED;
            }
        }

        // Check for collisions with top/bottom borders
        if ball_translation.y < y_min {
            ball_translation.y = y_min;
            ball.velocity.y *= -1.0;
        } else if ball_translation.y > y_max {
            ball_translation.y = y_max;
            ball.velocity.y *= -1.0;
        }

        // Begin to move the ball
        ball_translation.x += ball.velocity.x * time.delta_seconds();
        ball_translation.y += ball.velocity.y * time.delta_seconds();
    }
}

// Reset object position every time the score changes
fn serve_on_score_change(
    mut ball_query: Query<(&mut Transform, &mut Ball)>,
    mut score_event: EventReader<ScoreChanged>,
) {
    for _ in score_event.iter() {
        for (mut transform, mut ball) in ball_query.iter_mut() {
            *transform = Transform::IDENTITY;
            ball.serve();
        }
    }
}
