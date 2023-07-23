use super::paddle::{Paddle, PADDLE_HEIGHT, PADDLE_WIDTH};
use super::score::{Score, ScoreChanged};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::window::PrimaryWindow;

const BALL_SIZE: f32 = 30.;
const BALL_DEFAULT_SPEED: f32 = 800.;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (ball_movement, serve_on_score_change, serve_on_button_press),
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
        Ball {
            serve_left: true,
            velocity: Vec2::new(rand::random::<f32>(), rand::random::<f32>()).normalize()
                * (BALL_DEFAULT_SPEED / 1.5),
        }
    }
}

impl Ball {
    fn serve(&mut self) {
        let x_multiplier = if self.serve_left { -1.0 } else { 1.0 };
        self.serve_left = !self.serve_left;
        self.velocity = Vec2::new(rand::random::<f32>() * x_multiplier, rand::random::<f32>())
            .normalize()
            * (BALL_DEFAULT_SPEED / 1.5);
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

    for (mut ball_transform, mut ball) in ball_query.iter_mut() {
        let mut ball_translation = &mut ball_transform.translation;

        // Border collision handling
        let half_ball_size = BALL_SIZE / 2.0;
        let horizontal_border = window.width() / 2.0;
        let vertical_border = window.height() / 2.0;
        let x_min = -horizontal_border + half_ball_size;
        let x_max = horizontal_border - half_ball_size;
        let y_min = -vertical_border + half_ball_size;
        let y_max = vertical_border - half_ball_size;

        // Check if the player is out of bounds, reverse velocity if so
        if ball_translation.y < y_min || ball_translation.y > y_max {
            ball.velocity.y *= -1.0;
        }

        if ball_translation.x < x_min {
            ball.velocity.x *= -1.0;
            score_event.send(ScoreChanged(Score {
                left_score: score.left_score + 1,
                right_score: score.right_score,
            }));
        } else if ball_translation.x > x_max {
            ball.velocity.x *= -1.0;
            score_event.send(ScoreChanged(Score {
                left_score: score.left_score,
                right_score: score.right_score + 1,
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

fn serve_on_button_press(
    mut ball_query: Query<(&mut Transform, &mut Ball)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::R) {
        for (mut transform, mut ball) in ball_query.iter_mut() {
            *transform = Transform::IDENTITY;
            ball.serve();
        }
    }
}
