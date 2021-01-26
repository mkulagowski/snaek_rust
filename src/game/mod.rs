#[macro_use]
mod consts;
mod coords;
mod direction;
mod food;
mod line;
mod renderer;
mod resourceloader;
mod snake;
mod state;
mod turn;

use ggez::{
    event::{EventHandler, KeyCode, KeyMods},
    graphics, Context, GameResult,
};
use state::GameState;
use std::time::Instant;

pub use crate::game::consts::*;
pub use crate::game::state::GameData;

use self::{coords::Coords, direction::Direction, renderer::Renderer};

impl EventHandler for GameData {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let time_delta = (Instant::now() - self.delta_time).as_secs_f32();

        match self.state {
            GameState::PreGame => {}
            GameState::Game => {
                self.update_input(time_delta);
                self.update_snake(time_delta);
            }
        }

        self.delta_time = Instant::now();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        Renderer::draw_bg(ctx, &self.resources.bg_image);
        Renderer::draw_snake(ctx, &self.snake);
        Renderer::draw_food(ctx, &self.food, &self.resources.food_image);

        match self.state {
            GameState::PreGame => {
                let (xdim, ydim) = self.pregame_txt.dimensions(ctx);
                let (xdim, ydim) = (xdim as f32, ydim as f32);
                Renderer::draw_text_with_outline(
                    ctx,
                    &self.pregame_txt,
                    Coords::new(
                        consts::SCREEN_SIZE.x / 2. - xdim / 2.,
                        consts::SCREEN_SIZE.y / 2. - ydim / 2.,
                    ),
                );
            }
            GameState::Game => {
                Renderer::draw_text_with_outline(ctx, &self.score_txt, Coords::new(10., 10.));
            }
        }

        graphics::present(ctx)
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _km: KeyMods, _rpt: bool) {
        if let Some(dir) = match keycode {
            KeyCode::W => Some(Direction::UP),
            KeyCode::S => Some(Direction::DOWN),
            KeyCode::A => Some(Direction::LEFT),
            KeyCode::D => Some(Direction::RIGHT),
            _ => None,
        } {
            if self.inputs.is_empty() || self.inputs.back().unwrap() != &dir {
                self.inputs.push_back(dir);
            }
        } else if keycode == KeyCode::Space {
            if self.state == GameState::PreGame {
                self.state = GameState::Game;
            }
            self.inputs.clear();
        }
    }
}
