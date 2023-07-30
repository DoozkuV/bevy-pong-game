use bevy::prelude::*;

mod menu;
use menu::MenuPlugin;

mod game;
use game::GamePlugin;

mod ball;
use ball::BallPlugin;

mod paddle;
use paddle::PaddlePlugin;

mod score;
use score::ScorePlugin;

mod ui;
use ui::UiPlugin;

// Consts to define the resolution of the game window in pixels
pub const WINDOW_WIDTH: f32 = 802.;
pub const WINDOW_HEIGHT: f32 = 455.;
// Defines the pixel height of the top UI Scorebar
pub const UI_HEIGHT: f32 = 47.;
// Main font to be used
pub const MAIN_FONT: &str = "fonts/Teko-Regular.ttf";

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    End,
}

fn main() {
    App::new()
        .add_plugins((
            // Set the resolution
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                    resizable: false,
                    title: "Pong".to_string(),
                    ..default()
                }),
                ..default()
            }),
            // Set up the main game plugins
            // These plugins initialize important game functionality
            BallPlugin,
            PaddlePlugin,
            ScorePlugin,
            UiPlugin,
            // These plugins concern the actual running of the game
            MenuPlugin,
            GamePlugin,
        ))
        .add_state::<AppState>()
        .run();
}
