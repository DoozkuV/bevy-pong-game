use crate::ball::Ball;

use crate::paddle::Paddle;
use bevy::prelude::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ScoreChanged>()
            .add_systems(Update, (update_score, reset_on_score_change));
    }
}

#[derive(Component, Default, Clone, Copy, Debug)]
pub struct Score {
    pub left_score: u32,
    pub right_score: u32,
}

#[derive(Event)]
pub struct ScoreChanged(pub Score);

fn update_score(mut change_events: EventReader<ScoreChanged>, mut score_query: Query<&mut Score>) {
    let mut score = score_query.get_single_mut().unwrap();
    for event in change_events.iter() {
        *score = event.0;
        println!("New score is {:?}", score);
    }
}

// Reset object position every time the score changes
fn reset_on_score_change(
    mut ball_query: Query<(&mut Transform, &mut Ball)>,
    mut paddle_query: Query<&mut Transform, (With<Paddle>, Without<Ball>)>,
    mut score_event: EventReader<ScoreChanged>,
) {
    for _ in score_event.iter() {
        for (mut transform, mut ball) in ball_query.iter_mut() {
            *transform = Transform::IDENTITY;
            *ball = Ball::default();
        }
        for mut transform in paddle_query.iter_mut() {
            transform.translation.y = 0.0;
        }
    }
}
