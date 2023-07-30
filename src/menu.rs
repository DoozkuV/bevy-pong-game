use bevy::prelude::*;

use crate::{AppState, MAIN_FONT};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_start_menu)
            .add_systems(Update, button_system.run_if(in_state(AppState::MainMenu)))
            .add_systems(OnExit(AppState::MainMenu), cleanup_menu)
            ;
    }
}

// Resource for controlling
#[derive(Resource)]
pub struct MenuData {
    main_ui: Entity,
    pub is_single_player: bool,
}

impl MenuData {
    fn new(main_ui: Entity) -> Self {
        MenuData {
            main_ui,
            is_single_player: false,
        }
    }
}

#[derive(Component, PartialEq)]
enum StartButton {
    SinglePlayer,
    Multiplayer,
}

fn setup_start_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let main_ui = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::BLACK.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "PONG",
                TextStyle {
                    font: asset_server.load(MAIN_FONT),
                    font_size: 200.0,
                    color: Color::WHITE,
                },
            ));
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        width: Val::Percent(70.0),
                        justify_content: JustifyContent::SpaceAround,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((create_menu_button(), StartButton::SinglePlayer))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "One Player",
                                TextStyle {
                                    font: asset_server.load(MAIN_FONT),
                                    font_size: 40.,
                                    color: Color::BLACK,
                                },
                            ));
                        });
                    parent
                        .spawn((create_menu_button(), StartButton::Multiplayer))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Two Player",
                                TextStyle {
                                    font: asset_server.load(MAIN_FONT),
                                    font_size: 40.,
                                    color: Color::BLACK,
                                },
                            ));
                        });
                });
        })
        // Make sure commands returns the Entity so that we can pass it into the resource
        .id();
    commands.insert_resource(MenuData::new(main_ui));
}

fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
   commands.entity(menu_data.main_ui).despawn_recursive();
}

fn create_menu_button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            width: Val::Px(150.),
            height: Val::Px(65.0),
            border: UiRect::all(Val::Px(5.0)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::rgb_u8(153, 48, 122).into(),
        border_color: BorderColor(Color::PURPLE),
        ..default()
    }
}

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &StartButton,
        ),
        Changed<Interaction>,
    >,
    mut next_state: ResMut<NextState<AppState>>,
    mut menu_data: ResMut<MenuData>,
) {
    for (interaction, mut color, mut border_color, start_button) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {}
        match *interaction {
            Interaction::Pressed => {
                menu_data.is_single_player = *start_button == StartButton::SinglePlayer;
                next_state.set(AppState::Game);
            }
            Interaction::Hovered => {
                *color = Color::rgb_u8(96, 36, 78).into();
                *border_color = Color::BLUE.into();
            }
            Interaction::None => {
                *color = Color::rgb_u8(153, 48, 122).into();
                *border_color = Color::PURPLE.into();
            }
        }
    }
}
