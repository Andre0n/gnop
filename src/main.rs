mod paddle;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::time::FixedTimestep;
use paddle::spawn_paddles;

// Window defaults
const WINDOW_WIDTH: f32 = 854.;
const WINDOW_HEIGHT: f32 = 480.;

// Defines the amount of time that should elapse between each physics step. (FPS)
const TIME_STEP: f32 = 1.0 / 60.0;

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
        .add_system_set(SystemSet::new().with_run_criteria(FixedTimestep::step(TIME_STEP as f64)))
        .run();
}

fn setup(mut commands: Commands) {
    setup_camera_2d(&mut commands);
    spawn_paddles(&mut commands);
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
