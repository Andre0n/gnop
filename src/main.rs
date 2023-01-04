mod ball;
mod paddle;
mod wall;
use ball::{apply_ball_velocity, spawn_ball, Ball, Velocity};
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::time::FixedTimestep;
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use paddle::{paddle_movement_system, spawn_paddles};
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
pub struct ResetBallEvent;

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
        .add_startup_system(setup)
        .add_event::<CollisionEvent>()
        .add_event::<ResetBallEvent>()
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(check_collisions)
                .with_system(paddle_movement_system.before(check_collisions))
                .with_system(apply_ball_velocity.before(check_collisions)),
        )
        .run();
}

fn setup(mut commands: Commands) {
    setup_camera_2d(&mut commands);
    spawn_paddles(&mut commands);
    spawn_ball(&mut commands);
    spawn_walls(&mut commands);
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
    collider_query: Query<(Entity, &Transform, Option<&WallLocation>), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
    mut reset_ball_event: EventWriter<ResetBallEvent>,
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

        if let Some(collision) = collision {
            collision_events.send_default();

            if wall.is_some() {
                match wall.unwrap() {
                    WallLocation::Left => reset_ball_event.send_default(),
                    WallLocation::Right => reset_ball_event.send_default(),
                    WallLocation::Top => {}
                    WallLocation::Bottom => {}
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
