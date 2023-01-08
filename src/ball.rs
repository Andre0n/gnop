use crate::TIME_STEP;
use bevy::prelude::*;
use rand::Rng;

const BALL_SPEED: f32 = 350.;
const BALL_SIZE: Vec3 = Vec3::new(20., 20.0, 0.0);
const BALL_COLOR: Color = Color::rgb(1., 1., 1.);

#[derive(Component)]
pub struct Ball;

impl Ball {
    fn get_random_initial_direction() -> Vec2 {
        let x_safe_dir = [-1.0, 1.0];
        let y_safe_dir = [-1.0, 0.5, 1.0];

        let initial_direction: Vec2 = Vec2::new(
            x_safe_dir[rand::thread_rng().gen_range(0..2)],
            y_safe_dir[rand::thread_rng().gen_range(0..3)],
        );
        // Generate a random direction
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
            Ball::get_random_initial_direction().normalize() * BALL_SPEED,
        ));
}

pub fn apply_ball_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * TIME_STEP;
        transform.translation.y += velocity.y * TIME_STEP;
    }
}

pub fn reset_ball_to_center(
    mut commands: Commands,
    mut query: Query<Entity, With<Ball>>,
    reset_event: EventReader<ResetBallEvent>,
) {
    if reset_event.is_empty() {
        return;
    }

    let ball_entity = query.single_mut();
    commands.entity(ball_entity).despawn();
    commands
        .spawn(Ball::get_bundle())
        .insert(Ball)
        .insert(Velocity(
            Ball::get_random_initial_direction().normalize() * BALL_SPEED,
        ));
}
