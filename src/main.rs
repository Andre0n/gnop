use bevy::prelude::*;

// Window defaults
const WINDOW_WIDTH: f32 = 854.;
const WINDOW_HEIGHT: f32 = 480.;

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
        .run();
}
