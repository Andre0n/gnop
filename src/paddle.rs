use bevy::prelude::*;

use crate::WINDOW_WIDTH;

// const PADDLE_SPEED: f32 = 500.; TODO: Implement movimentation function
const PADDLE_WIDTH: f32 = 20.;
const PADDLE_COLOR: Color = Color::rgb(1., 1., 1.);
const PADDLE_SIZE: Vec3 = Vec3::new(PADDLE_WIDTH, 120.0, 0.0);
const X_PADDLE_POS: f32 = WINDOW_WIDTH / 2. - PADDLE_WIDTH;

#[derive(Component)]
struct Paddle;

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
    ));
}

