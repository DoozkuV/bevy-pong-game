use super::score::ScoreChanged;
use bevy::prelude::*;

pub struct UiPlugin;

#[derive(Component)]
pub enum ScoreText {
    Left,
    Right,
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_score_text);
    }
}

/// This function sets up the UI for the score in our game;
/// Instantiating the Text elements that will later be used to store the score.
pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // The Board
    commands.spawn(SpriteBundle {
        texture: asset_server.load("sprites/Board.png"),
        transform: Transform::from_translation(Vec3 {
            y: -47.,
            ..default()
        }),
        ..default()
    });
    // Size of the Scorebar - 341x47
    // commands.spawn(SpriteBundle {
    //     texture: asset_server.load("sprites/ScoreBar.png"),
    //     ..default()
    // });
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font: asset_server.load("fonts/Teko-Regular.ttf"),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "0",
                TextStyle {
                    font: asset_server.load("fonts/Teko-Regular.ttf"),
                    font_size: 60.0,
                    color: Color::BLUE,
                },
            ),
        ]),
        ScoreText::Left,
    ));
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font: asset_server.load("fonts/Teko-Regular.ttf"),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "0",
                TextStyle {
                    font: asset_server.load("fonts/Teko-Regular.ttf"),
                    font_size: 60.0,
                    color: Color::ORANGE,
                },
            ),
        ]),
        ScoreText::Right,
    ));
}

fn update_score_text(
    mut text_query: Query<(&mut Text, &ScoreText)>,
    mut change_events: EventReader<ScoreChanged>,
) {
    for score_changed in change_events.iter() {
        let score = score_changed.0;
        for (mut text, score_text) in text_query.iter_mut() {
            match score_text {
                ScoreText::Right => text.sections[1].value = format!("{}", score.left_score),
                ScoreText::Left => text.sections[1].value = format!("{}", score.right_score),
            }
        }
    }
}
