use bevy::prelude::*;

use crate::{Collider, WINDOW_HEIGHT, WINDOW_WIDTH};

const WALL_POSITION_Y: f32 = WINDOW_HEIGHT / 2.0 + 5.;
const WALL_POSITION_X: f32 = WINDOW_WIDTH / 2.0 + 5.;
const WALL_THICK: f32 = 10.;
const WALL_COLOR: Color = Color::rgb(0.4, 0.4, 0.4);

#[derive(Component)]
pub struct Wall;

#[derive(Component, Clone, Copy)]
pub enum WallLocation {
    Top,
    Left,
    Bottom,
    Right,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Top => Vec2::new(0.0, WALL_POSITION_Y),
            WallLocation::Left => Vec2::new(-WALL_POSITION_X, 0.0),
            WallLocation::Bottom => Vec2::new(0.0, -WALL_POSITION_Y),
            WallLocation::Right => Vec2::new(WALL_POSITION_X, 0.0),
        }
    }
    fn size(&self) -> Vec2 {
        match self {
            WallLocation::Top => Vec2::new(WINDOW_WIDTH, WALL_THICK),
            WallLocation::Left => Vec2::new(WALL_THICK, WINDOW_HEIGHT),
            WallLocation::Bottom => Vec2::new(WINDOW_WIDTH, WALL_THICK),
            WallLocation::Right => Vec2::new(WALL_THICK, WINDOW_HEIGHT),
        }
    }
}

pub fn spawn_walls(commands: &mut Commands) {
    spawn_wall(commands, WallLocation::Left);
    spawn_wall(commands, WallLocation::Right);
    spawn_wall(commands, WallLocation::Top);
    spawn_wall(commands, WallLocation::Bottom);
}

fn spawn_wall(commands: &mut Commands, location: WallLocation) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: location.position().extend(0.0),
                scale: location.size().extend(0.0),
                ..default()
            },
            sprite: Sprite {
                color: WALL_COLOR,
                ..default()
            },
            ..default()
        },
        Wall,
        location,
        Collider,
    ));
}
