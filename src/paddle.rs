use bevy::prelude::*;

use crate::{TIME_STEP, WINDOW_HEIGHT, WINDOW_WIDTH};

const PADDLE_SPEED: f32 = 500.;
const PADDLE_WIDTH: f32 = 20.;
const PADDLE_PADDING: f32 = 10.0;
const PADDLE_COLOR: Color = Color::rgb(1., 1., 1.);
const PADDLE_SIZE: Vec3 = Vec3::new(PADDLE_WIDTH, 120.0, 0.0);
const X_PADDLE_POS: f32 = WINDOW_WIDTH / 2. - PADDLE_WIDTH;

#[derive(Component)]
pub struct Paddle;

#[derive(Component, Clone, Copy)]
pub enum PaddleLocation {
    Left,
    Right,
}

impl PaddleLocation {
    fn position(&self) -> Vec2 {
        match self {
            PaddleLocation::Left => Vec2::new(-X_PADDLE_POS, 0.0),
            PaddleLocation::Right => Vec2::new(X_PADDLE_POS, 0.0),
        }
    }
    fn movement_keys(&self) -> (KeyCode, KeyCode) {
        match self {
            PaddleLocation::Left => (KeyCode::W, KeyCode::S),
            PaddleLocation::Right => (KeyCode::Up, KeyCode::Down),
        }
    }
}

pub fn spawn_paddles(commands: &mut Commands) {
    spawn_paddle(commands, PaddleLocation::Left);
    spawn_paddle(commands, PaddleLocation::Right);
}

fn spawn_paddle(commands: &mut Commands, location: PaddleLocation) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: location.position().extend(0.0),
                scale: PADDLE_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                ..default()
            },
            ..default()
        },
        Paddle,
        location,
    ));
}

pub fn paddle_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&PaddleLocation, &mut Transform, With<Paddle>)>,
) {
    for (paddle_loc, mut transform, _) in query.iter_mut() {
        let mut direction = 0.;

        let (up_keycode, down_keycode) = paddle_loc.movement_keys();

        if keyboard_input.pressed(up_keycode) {
            direction += 1.0;
        }

        if keyboard_input.pressed(down_keycode) {
            direction -= 1.0;
        }

        // Calculate the new vertical paddle position based on player input
        let new_paddle_position = transform.translation.y + direction * PADDLE_SPEED * TIME_STEP;

        // Limits
        let top_bound = WINDOW_HEIGHT / 2.0 - PADDLE_SIZE.y / 2.0 - PADDLE_PADDING;
        let bottom_bound = -WINDOW_HEIGHT / 2.0 + PADDLE_SIZE.y / 2.0 + PADDLE_PADDING;

        // Update the paddle position,
        transform.translation.y = new_paddle_position.clamp(bottom_bound, top_bound);
    }
}
