//TODO: Add scoreboard things here

use bevy::prelude::*;

use crate::WINDOW_WIDTH;

const SCOREBOARD_FONT_SIZE: f32 = 120.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(WINDOW_WIDTH / 2.0 - 110.);
const SCORE_COLOR: Color = Color::rgb(1., 1., 1.);

#[derive(Resource)]
pub struct Scoreboard {
    score_player_one: usize,
    score_player_two: usize,
}

impl Scoreboard {
    pub fn new() -> Self {
        return Self {
            score_player_one: 0,
            score_player_two: 0,
        };
    }
    pub fn increase_player_one_score(&mut self) {
        self.score_player_one += 1;
    }
    pub fn increase_player_two_score(&mut self) {
        self.score_player_two += 1;
    }
}

pub fn spawn_score_board(commands: &mut Commands, asset_server: &mut Res<AssetServer>) {
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "",
                TextStyle {
                    font: asset_server.load("fonts/bit5x3.ttf"),
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: SCORE_COLOR,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/bit5x3.ttf"),
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCORE_COLOR,
            }),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/bit5x3.ttf"),
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCORE_COLOR,
            }),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/bit5x3.ttf"),
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCORE_COLOR,
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(10.0),
                left: SCOREBOARD_TEXT_PADDING,
                ..default()
            },
            ..default()
        }),
    );
}

pub fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.score_player_one.to_string();
    text.sections[2].value = " ".to_string();
    text.sections[3].value = scoreboard.score_player_two.to_string();
}
