use bevy::prelude::*;

use crate::AppState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ScoreChanged>().add_systems(
            Update,
            (update_score, play_sound_on_score).run_if(in_state(AppState::Game)),
        );
    }
}

#[derive(Component, Default, Clone, Copy)]
pub struct Score {
    pub right_score: u32,
    pub left_score: u32,
}

#[derive(Event)]
pub struct ScoreChanged(pub Score);

fn update_score(mut change_events: EventReader<ScoreChanged>, mut score_query: Query<&mut Score>) {
    let mut score = score_query.get_single_mut().unwrap();
    for event in change_events.iter() {
        *score = event.0;
    }
}

fn play_sound_on_score(
    mut change_events: EventReader<ScoreChanged>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for _ in change_events.iter() {
        commands.spawn(AudioBundle {
            source: asset_server.load("sounds/score_sound.wav"),
            settings: PlaybackSettings::DESPAWN,
        });
    }
}
