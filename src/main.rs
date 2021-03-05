//! # Snaek_rust
//! Little redefinition of a standard game of snake, written for the sake of learning Rust
//!
//! Main differences are:
//! * removal of locked framerate
//! * not splitting the snake into square segments
//! * making the turns look a little bit more like turns
//!
//! Uses [ggez crate](https://crates.io/crates/ggez) for game engine related stuff.

use game::{GameData, GAME_AUTHOR, GAME_ID, SCREEN_SIZE};
use ggez::{
    conf,
    event::{self},
};
use ggez::{ContextBuilder, GameResult};
use std::path;
mod game;

/// Main function that sets-up the window, creates GameData and runs the main game loop.
///
fn main() -> GameResult {
    let window_setup = conf::WindowSetup::default().title(GAME_ID);
    let window_mode = conf::WindowMode::default().dimensions(SCREEN_SIZE.x, SCREEN_SIZE.y);
    let resource_path = path::PathBuf::from("./resources");

    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new(GAME_ID, GAME_AUTHOR)
        .window_setup(window_setup)
        .window_mode(window_mode)
        .add_resource_path(resource_path)
        .build()?;

    let game_state = &mut GameData::new(ctx);
    event::run(ctx, event_loop, game_state)
}
