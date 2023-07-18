use bevy::prelude::*;
use bevy::window::PrimaryWindow;

// Hello mod for testing bevy functionality
// mod hello;
// use hello::HelloPlugin;

// const RIGHT_PADDLE_POS: Vec3 = Vec3::new(625.0, 0.0, 0.0);
// const LEFT_PADDLE_POS: Vec3 = Vec3::new(-625.0, 0.0, 0.0);
const PADDLE_SPEED: f32 = 500.;
const PADDLE_HEIGHT: f32 = 120.;
const PADDLE_WIDTH: f32 = 17.;

const INPUT_UP: KeyCode = KeyCode::W;
const INPUT_DOWN: KeyCode = KeyCode::S;

const BALL_SPEED: f32 = 500.;
const BALL_SIZE: f32 = 30.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (ball_movement, player_control, computer_control))
        .run();
}

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Computer;

#[derive(Component)]
struct Speed {
    x_speed: f32,
    y_speed: f32,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query
        .get_single()
        .expect("Only one primary window should exist!");

    commands.spawn(Camera2dBundle::default());
    // Spawn the ball
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/Ball.png"),
            transform: Transform::IDENTITY, // Object should be centered
            ..default()
        },
        Ball,
        Speed {
            x_speed: BALL_SPEED,
            y_speed: BALL_SPEED,
        },
    ));

    // Spawn the left-most paddle
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/Player.png"),
            transform: Transform::from_xyz((-window.width() / 2.0) + PADDLE_WIDTH, 0.0, 0.0),
            ..default()
        },
        Paddle,
        Player,
    ));

    // Spawn the right-most paddle
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/Computer.png"),
            transform: Transform::from_xyz((window.width() / 2.0) - PADDLE_WIDTH, 0.0, 0.0),
            ..default()
        },
        Paddle,
        Computer,
    ));
}

fn ball_movement(
    time: Res<Time>,
    mut ball_query: Query<(&mut Transform, &mut Speed), With<Ball>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // Extract the ball - Can be rewritten to support multiple balls
    let (mut transform, mut speed) = ball_query
        .get_single_mut()
        .expect("Only one ball should exist!");

    // Collision handling
    let window = window_query
        .get_single()
        .expect("Only one primary window should exist!");

    let half_player_size = BALL_SIZE / 2.0; // 30.0
    let horizontal_border = window.width() / 2.0;
    let vertical_border = window.height() / 2.0;
    let x_min = -horizontal_border + half_player_size;
    let x_max = horizontal_border - half_player_size;
    let y_min = -vertical_border + half_player_size;
    let y_max = vertical_border - half_player_size;

    let mut translation = &mut transform.translation;

    // Bound the players x position
    if translation.x < x_min || translation.x > x_max {
        speed.x_speed *= -1.0;
    }

    // Bound the players y position
    if translation.y < y_min || translation.y > y_max {
        speed.y_speed *= -1.0;
    }

    // Begin to move the ball
    translation.x += speed.x_speed * time.delta_seconds();
    translation.y += speed.y_speed * time.delta_seconds();
}

fn player_control(
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    let window = window_query
        .get_single()
        .expect("Only one primary window should exist!");

    let mut transform = player_query
        .get_single_mut()
        .expect("Only one player has been implemented yet!");

    if keyboard_input.pressed(INPUT_UP) {
        move_paddle(&mut transform, window, 1.0, &time);
    }
    if keyboard_input.pressed(INPUT_DOWN) {
        move_paddle(&mut transform, window, -1.0, &time);
    }
}

fn computer_control(
    mut computer_query: Query<&mut Transform, With<Computer>>,
    ball_query: Query<&Transform, With<Ball>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    let mut transform = computer_query
        .get_single_mut()
        .expect("Only one computer has been implemented yet!");

    let ball_y_pos = ball_query
        .get_single()
        .expect("Only one ball has been implemented yet!")
        .translation
        .y;

    let window = window_query
        .get_single()
        .expect("Only one primary window should exist!");

    if ball_y_pos > transform.translation.y {
        move_paddle(&mut transform, window, 1.0, &time);
    } else {
        move_paddle(&mut transform, window, -1.0, &time);
    }
}

fn move_paddle(transform: &mut Transform, window: &Window, multiplier: f32, time: &Time) {
    let half_paddle_height = PADDLE_HEIGHT / 2.0;
    let vertical_border = window.height() / 2.0;
    let y_min = -vertical_border + half_paddle_height;
    let y_max = vertical_border - half_paddle_height;

    let y_pos = &mut transform.translation.y;

    *y_pos = (*y_pos + PADDLE_SPEED * multiplier * time.delta_seconds()).clamp(y_min, y_max);
}
