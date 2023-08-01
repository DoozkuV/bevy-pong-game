use bevy::prelude::*;

use crate::game::GameData;
use crate::{menu, AppState, MAIN_FONT};

pub struct EndPlugin;

impl Plugin for EndPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::End), setup_end)
            .add_systems(Update, end_button.run_if(in_state(AppState::End)))
            .add_systems(OnExit(AppState::End), cleanup_end);
    }
}

#[derive(Resource)]
struct EndData(Entity);

fn setup_end(mut commands: Commands, asset_server: Res<AssetServer>, game_data: Res<GameData>) {
    let end_ui = commands
        .spawn(menu::create_ui_base()) // Get the ui template from menu
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                // Display the correct text based on who won
                if game_data.is_left_win {
                    "Blue Wins"
                } else {
                    "Orange Wins"
                },
                TextStyle {
                    font: asset_server.load(MAIN_FONT),
                    font_size: 200.0,
                    color: if game_data.is_left_win {
                        Color::BLUE
                    } else {
                        Color::ORANGE
                    },
                },
            ));

            // Again, create the button using the default styling from menu
            parent.spawn(menu::create_button()).with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "GG",
                    TextStyle {
                        font: asset_server.load(MAIN_FONT),
                        font_size: 40.,
                        color: Color::BLACK,
                    },
                ));
            });
        })
        .id();

    // Despawn the Game Data and spawn the EndData
    commands.remove_resource::<GameData>();
    commands.insert_resource(EndData(end_ui));
}

fn cleanup_end(mut commands: Commands, end_data: Res<EndData>) {
    commands.entity(end_data.0).despawn_recursive();
    commands.remove_resource::<EndData>();
}

fn end_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        Changed<Interaction>,
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut color, mut border_color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => next_state.set(AppState::Menu),
            Interaction::Hovered => {
                *color = Color::rgb_u8(96, 36, 78).into();
                *border_color = Color::INDIGO.into();
            }
            Interaction::None => {
                *color = Color::rgb_u8(153, 48, 122).into();
                *border_color = Color::PURPLE.into();
            }
        }
    }
}
