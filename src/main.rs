#[macro_use]
mod consts;
mod coords;
mod direction;
mod food;
mod line;
mod renderer;
mod resourceloader;
mod snake;
mod turn;

use coords::Coords;
use direction::Direction;
use food::Food;
use ggez::{
    event::{self, EventHandler},
    graphics::{Font, Text},
};
use ggez::{
    event::{KeyCode, KeyMods},
    graphics::TextFragment,
};
use ggez::{graphics, Context, ContextBuilder, GameResult};
use renderer::Renderer;
use resourceloader::ResourceLoader;
use snake::LineSnake;
use std::{collections::VecDeque, time::Instant};

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new(consts::GAME_ID, consts::GAME_AUTHOR)
        .window_setup(ggez::conf::WindowSetup::default().title(consts::GAME_ID))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(consts::SCREEN_SIZE.x as f32, consts::SCREEN_SIZE.y as f32),
        )
        .build()
        .expect("Error while creating ggez Context!");

    let mut my_game = GameData::new(&mut ctx);

    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum GameState {
    PreGame,
    Game,
}

struct GameData {
    snake: LineSnake,
    food: Food,
    delta_time: std::time::Instant,
    inputs: VecDeque<Direction>,
    input_timer: f32,
    score: u32,
    score_txt: Text,
    pregame_txt: Text,
    state: GameState,
    resources: ResourceLoader,
}

impl GameData {
    fn new(ctx: &mut Context) -> GameData {
        graphics::set_default_filter(ctx, graphics::FilterMode::Nearest);
        let resources = ResourceLoader::new(ctx);
        GameData {
            snake: LineSnake::new(consts::SCREEN_SIZE.x / 2.0, consts::SCREEN_SIZE.y / 2.0),
            delta_time: Instant::now(),
            food: Food::random(),
            inputs: VecDeque::new(),
            input_timer: 0.0,
            score: 0,
            score_txt: Self::create_score_txt(0, resources.font),
            pregame_txt: Self::create_pregame_txt(resources.font),
            state: GameState::PreGame,
            resources: resources,
        }
    }

    fn reset(&mut self) {
        self.snake = LineSnake::new(consts::SCREEN_SIZE.x / 2.0, consts::SCREEN_SIZE.y / 2.0);
        self.food = Food::random();
        while self.snake.collide(&self.food.bbox) {
            self.food = Food::random();
        }
        self.inputs.clear();
        self.score = 0;
        self.score_txt = Self::create_score_txt(0, self.resources.font);
        self.state = GameState::PreGame;
    }

    fn inc_score(&mut self) {
        self.score += 1;
        self.score_txt = Self::create_score_txt(self.score, self.resources.font);
    }

    fn create_score_txt(score: u32, font: Font) -> Text {
        Text::new(
            TextFragment::new(format!(SCORE_FMT!(), score))
                .scale(graphics::Scale::uniform(24.))
                .font(font),
        )
    }
    fn create_pregame_txt(font: Font) -> Text {
        Text::new(
            TextFragment::new(consts::PREGAME_TXT)
                .scale(graphics::Scale::uniform(64.))
                .font(font),
        )
    }

    fn update_input(&mut self, time_delta: f32) {
        self.input_timer += time_delta;
        if self.input_timer >= consts::SECS_PER_INPUT_UPDATE {
            if let Some((idx, &new_dir)) = self
                .inputs
                .iter()
                .enumerate()
                .find(|&(_, x)| x != &self.snake.dir.inverse() && x != &self.snake.dir)
            {
                self.snake.dir = new_dir;
                self.inputs.drain(0..=idx);
                self.input_timer = 0.0;
            } else {
                self.inputs.clear();
            }
        }
    }

    fn update_snake(&mut self, time_delta: f32) {
        if self.snake.collide(&self.food.bbox) {
            self.snake.grow(consts::FOOD_SIZE);
            self.inc_score();
            while self.snake.collide(&self.food.bbox) {
                self.food = Food::random();
            }
        } else if self.snake.self_collide() || self.snake.wall_collide() {
            self.reset();
        } else {
            self.snake.do_move(time_delta * consts::SPEED);
        }
    }
}

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
