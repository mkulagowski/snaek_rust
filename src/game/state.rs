use std::{collections::VecDeque, time::Instant};

use ggez::{
    graphics::{self, Font, Text, TextFragment},
    Context,
};

use crate::game::snake::LineSnake;
use crate::game::{consts, direction::Direction, food::Food, resourceloader::ResourceLoader};

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum GameState {
    PreGame,
    Game,
}

pub struct GameData {
    pub snake: LineSnake,
    pub food: Food,
    pub delta_time: std::time::Instant,
    pub inputs: VecDeque<Direction>,
    pub input_timer: f32,
    pub score: u32,
    pub score_txt: Text,
    pub pregame_txt: Text,
    pub state: GameState,
    pub resources: ResourceLoader,
}

impl GameData {
    pub fn new(ctx: &mut Context) -> Self {
        graphics::set_default_filter(ctx, graphics::FilterMode::Nearest);
        let resources = ResourceLoader::new(ctx);
        Self {
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

    pub fn update_input(&mut self, time_delta: f32) {
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

    pub fn update_snake(&mut self, time_delta: f32) {
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
