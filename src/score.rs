use bevy::prelude::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ScoreChanged>()
            .add_systems(Update, update_score);
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
    }
}
