use bevy::prelude::*;
use bevy::window::close_on_esc;

// Set up all the modules for the program
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

mod end;
use end::EndPlugin;

// Consts to define the resolution of the game window in pixels
pub const WINDOW_WIDTH: f32 = 802.;
pub const WINDOW_HEIGHT: f32 = 455.;
// Defines the pixel height of the top UI Scorebar
pub const UI_HEIGHT: f32 = 47.;
// Main font to be used
pub const MAIN_FONT: &str = "fonts/Teko-Regular.ttf";

/// State management Enum to be used throughout the entire project.
/// Each state corresponds to a different screen in the game as well
/// as a different module in code.
///
/// - **Menu** refers to the Start up menu which displays the logo and
/// prompts the player to select one or two players
/// - **Game** refers to the actual game itself, and loads both the
/// *game* module and it's respective components as well as the *ui* module
/// - **End** refers to the simple end screen which loops back into the *Menu*
/// state
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
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
            // Plugins that are used during the actual game itself
            // These plugins only run on the 'AppState::Game'
            BallPlugin,
            PaddlePlugin,
            ScorePlugin,
            UiPlugin,
            // Plugins which refer to state-management
            MenuPlugin,
            GamePlugin,
            EndPlugin,
        ))
        .add_state::<AppState>()
        // Do this outside of any state management
        .add_systems(Startup, init_game)
        // Built in bevy utility func for testing
        .add_systems(Update, close_on_esc)
        .run();
}

// Spawns the camera and does other initialization functionality
fn init_game(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
