use super::Score;
use bevy::prelude::*;

pub struct UiPlugin;

#[derive(Component)]
struct ScoreText;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {}
}

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font: asset_server.load("fonts/Teko-Regular"),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/Teko-Bold"),
                font_size: 60.0,
                color: Color::GOLD,
            }),
        ]),
        ScoreText,
    ));
}

fn update_score_text() {}
