use crate::TIME_STEP;
use bevy::prelude::*;
use rand::Rng;

const BALL_SPEED: f32 = 350.;
const BALL_SIZE: Vec3 = Vec3::new(20., 20.0, 0.0);
const BALL_COLOR: Color = Color::rgb(1., 1., 1.);

#[derive(Component)]
pub struct Ball;

impl Ball {
    fn get_initial_direction() -> Vec2 {
        // Generate a random direction
        let initial_direction = Vec2::new(
            rand::thread_rng().gen_range(-1.0..1.0),
            rand::thread_rng().gen_range(-1.0..1.0),
        );
        return initial_direction;
    }
    fn get_bundle() -> SpriteBundle {
        return SpriteBundle {
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
        };
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec2);

#[derive(Default)]
pub struct ResetBallEvent;

pub fn spawn_ball(commands: &mut Commands) {
    commands
        .spawn(Ball::get_bundle())
        .insert(Ball)
        .insert(Velocity(
            Ball::get_initial_direction().normalize() * BALL_SPEED,
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
