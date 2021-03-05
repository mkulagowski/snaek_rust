//! Module that gathers all more or less modifiable parameters
use crate::game::coords::Coords;

pub const GAME_ID: &str = "snaek_rust";
pub const GAME_AUTHOR: &str = "mk.kulagowski";

pub const SCREEN_SIZE: Coords = Coords { x: 800.0, y: 800.0 };
pub const SNAKE_WIDTH: f32 = 20.;
pub const SNAKE_START_LEN: u8 = 8;
pub const SPEED: f32 = SNAKE_WIDTH * 15.;

pub const WALL_MARGIN: f32 = SNAKE_WIDTH * 0.5;
pub const COLLISION_PIXELS_MARGIN: f32 = 1.;
pub const FOOD_SIZE: f32 = SNAKE_WIDTH;
pub const FOOD_HALF_SIZE: f32 = FOOD_SIZE / 2.;
pub const SNAKE_HALF_WIDTH: f32 = SNAKE_WIDTH / 2.;
pub const SNAKE_START_HEIGHT: f32 = SNAKE_WIDTH * SNAKE_START_LEN as f32;
pub const TURN_MARGIN: f32 = SNAKE_WIDTH * 0.15;
pub const HALF_TURN_MARGIN: f32 = TURN_MARGIN / 2.;
pub const SECS_PER_INPUT_UPDATE: f32 = (SNAKE_WIDTH + TURN_MARGIN) / SPEED;

pub const PREGAME_TXT: &str = "Press SPACE to start the game";

/// Cannot use const value here, as macro requires literals
#[macro_export]
macro_rules! SCORE_FMT {
    () => {
        "Score: {}"
    };
}
