use crate::{ResetBallEvent, TIME_STEP};
use bevy::prelude::*;
use rand::Rng;

const BALL_SPEED: f32 = 350.;
const BALL_SIZE: Vec3 = Vec3::new(20., 20.0, 0.0);
const BALL_COLOR: Color = Color::rgb(1., 1., 1.);

#[derive(Component)]
pub struct Ball;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec2);

pub fn spawn_ball(commands: &mut Commands) {
    // Generate a random direction
    let ball_initial_direction = Vec2::new(
        rand::thread_rng().gen_range(-1.0..1.0),
        rand::thread_rng().gen_range(-0.2..0.2),
    );

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                scale: BALL_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: BALL_COLOR,
                ..default()
            },
            ..default()
        },
        Ball,
        Velocity(ball_initial_direction.normalize() * BALL_SPEED),
    ));
}

pub fn apply_ball_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * TIME_STEP;
        transform.translation.y += velocity.y * TIME_STEP;
    }
}

pub fn reset_ball_to_center(
    mut query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    reset_ball_event: EventReader<ResetBallEvent>,
) {
    if reset_ball_event.is_empty() {
        return;
    }
    let (mut ball_transform, mut ball_velocity) = query.single_mut();
    ball_transform.translation = Vec3::new(0., 0., 0.);
    ball_velocity.x += rand::thread_rng().gen_range(-1.0..1.0);
    ball_velocity.y += rand::thread_rng().gen_range(-1.0..1.0);
}
