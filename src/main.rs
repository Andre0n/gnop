mod ball;
mod paddle;
mod scoreboard;
mod wall;
use ball::{apply_ball_velocity, reset_ball_to_center, spawn_ball, Ball, ResetBallEvent, Velocity};
use bevy::app::AppExit;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::time::FixedTimestep;
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use paddle::{paddle_movement_system, spawn_paddles};
use scoreboard::{reset_scoreboard, spawn_score_board, update_scoreboard, Scoreboard};
use wall::{spawn_walls, WallLocation};

// Window defaults
const WINDOW_WIDTH: f32 = 854.;
const WINDOW_HEIGHT: f32 = 480.;

// Defines the amount of time that should elapse between each physics step. (FPS)
const TIME_STEP: f32 = 1.0 / 60.0;

#[derive(Component)]
struct Collider;

#[derive(Default)]
struct CollisionEvent;

#[derive(Default)]
pub struct ResetGameEvent;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
                title: "GNOP - The Rust Pong".to_string(),
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .insert_resource(Scoreboard::new())
        .add_startup_system(setup)
        .add_event::<CollisionEvent>()
        .add_event::<ResetBallEvent>()
        .add_event::<ResetGameEvent>()
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(check_collisions)
                .with_system(reset_ball_to_center.after(check_collisions))
                .with_system(paddle_movement_system.before(check_collisions))
                .with_system(apply_ball_velocity.before(check_collisions)),
        )
        .add_system(update_scoreboard)
        .add_system(get_keyboard_events.after(check_collisions))
        .add_system(reset_scoreboard.after(check_collisions))
        .run();
}

fn setup(mut commands: Commands, mut asset_server: Res<AssetServer>) {
    setup_camera_2d(&mut commands);
    spawn_paddles(&mut commands);
    spawn_ball(&mut commands);
    spawn_walls(&mut commands);
    spawn_score_board(&mut commands, &mut asset_server);
}

// Camera
fn setup_camera_2d(commands: &mut Commands) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::rgb(0.4, 0.4, 0.4)),
        },
        ..default()
    });
}

fn check_collisions(
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    mut collision_events: EventWriter<CollisionEvent>,
    mut reset_ball_event: EventWriter<ResetBallEvent>,
    mut reset_game_event: EventWriter<ResetGameEvent>,
    mut scoreboard: ResMut<Scoreboard>,
    collider_query: Query<(Entity, &Transform, Option<&WallLocation>), With<Collider>>,
) {
    let (mut ball_velocity, ball_transform) = ball_query.single_mut();
    let ball_size = ball_transform.scale.truncate();

    for (_, transform, wall) in &collider_query {
        let collision = collide(
            ball_transform.translation,
            ball_size,
            transform.translation,
            transform.scale.truncate(),
        );

        if scoreboard.has_winner() {
            reset_game_event.send_default();
            reset_ball_event.send_default();
        }

        if let Some(collision) = collision {
            collision_events.send_default();

            if wall.is_some() {
                match wall.unwrap() {
                    WallLocation::Left => {
                        scoreboard.increase_player_two_score();
                        reset_ball_event.send_default();
                    }
                    WallLocation::Right => {
                        scoreboard.increase_player_one_score();
                        reset_ball_event.send_default();
                    }
                    WallLocation::Top => { /* do nothing */ }
                    WallLocation::Bottom => { /* do nothing */ }
                }
            }

            // reflect the ball when it collides
            let mut reflect_x = false;
            let mut reflect_y = false;

            match collision {
                Collision::Left => reflect_x = ball_velocity.x > 0.0,
                Collision::Right => reflect_x = ball_velocity.x < 0.0,
                Collision::Top => reflect_y = ball_velocity.y < 0.0,
                Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                Collision::Inside => { /* do nothing */ }
            }

            // reflect velocity on the x-axis if we hit something on the x-axis
            if reflect_x {
                ball_velocity.x = -ball_velocity.x;
            }

            // reflect velocity on the y-axis if we hit something on the y-axis
            if reflect_y {
                ball_velocity.y = -ball_velocity.y;
            }
        }
    }
}

fn get_keyboard_events(
    keyboard_input: Res<Input<KeyCode>>,
    mut exit: EventWriter<AppExit>,
    mut reset_ball_event: EventWriter<ResetBallEvent>,
    mut reset_game_event: EventWriter<ResetGameEvent>,
) {
    if keyboard_input.pressed(KeyCode::R) {
        reset_game_event.send_default();
        reset_ball_event.send_default();
    }
    if keyboard_input.pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
