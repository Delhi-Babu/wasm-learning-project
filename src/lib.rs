use std::usize;

use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(module = "/www/utils/rnd.js")]
extern "C" {
    fn rnd(ma: usize) -> usize;
}

#[wasm_bindgen]
#[derive(PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum GameStatus {
    Won,
    Lost,
    Played,
}

struct Snake {
    body: Vec<usize>,
    direction: Direction,
    next_cell: Option<usize>,
}

impl Snake {
    fn new(spanw_index: usize, size: usize) -> Snake {
        let mut body = vec![];
        for i in 0..size {
            body.push(spanw_index - i);
        }
        Snake {
            body,
            direction: Direction::Right,
            next_cell: None,
        }
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
    reward_cell: usize,
    state: Option<GameStatus>,
    points: usize,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_start_index: usize, snake_size: usize) -> World {
        let size = width * width;
        let snake = Snake::new(snake_start_index, snake_size);
        let reward_cell = World::generate_reward_cell(size, &snake.body);
        World {
            width,
            size,
            snake,
            reward_cell,
            state: None,
            points: 0,
        }
    }

    pub fn get_snake_head_idx(&self) -> usize {
        self.snake.body[0]
    }

    pub fn set_snake_direction(&mut self, snake_direction: Direction) {
        let next_cell = self.gen_next_snake_cell(&snake_direction);
        if self.snake.body[1] == next_cell {
            return;
        }
        self.snake.next_cell = Some(next_cell);
        self.snake.direction = snake_direction;
    }

    pub fn get_reward_cell(&self) -> usize {
        self.reward_cell
    }

    fn generate_reward_cell(size: usize, snake_body: &Vec<usize>) -> usize {
        let mut reward_cell;
        loop {
            reward_cell = rnd(size);
            if !snake_body.contains(&reward_cell) {
                break;
            }
        }
        reward_cell
    }

    pub fn start_game(&mut self) {
        self.state = Some(GameStatus::Played);
    }

    pub fn get_points(&self) -> usize {
        self.points
    }

    pub fn get_game_status(&self) -> Option<GameStatus> {
        self.state
    }
    pub fn game_status_text(&self) -> String {
        match self.state {
            Some(GameStatus::Won) => String::from("You have won!"),
            Some(GameStatus::Lost) => String::from("You have lost!"),
            Some(GameStatus::Played) => String::from("Playing"),
            None => String::from("No Status"),
        }
    }

    pub fn step(&mut self) {
        match self.state {
            Some(GameStatus::Played) => {
                let tmp = self.snake.body.clone();
                match self.snake.next_cell {
                    Some(cell) => {
                        self.snake.body[0] = cell;
                        self.snake.next_cell = None
                    }
                    None => self.snake.body[0] = self.gen_next_snake_cell(&self.snake.direction),
                }
                let snake_len = self.get_snake_length();
                for i in 1..snake_len {
                    self.snake.body[i] = tmp[i - 1];
                }

                if self.snake.body[1..self.get_snake_length()].contains(&self.snake.body[0]) {
                    self.state = Some(GameStatus::Lost)
                }

                if self.get_reward_cell() == self.get_snake_head_idx() {
                    self.snake.body.push(tmp[snake_len - 1]);
                    self.points += 1;

                    if self.get_snake_length() < self.size {
                        self.reward_cell = World::generate_reward_cell(self.size, &self.snake.body);
                    } else {
                        self.reward_cell = 1000;
                        self.state = Some(GameStatus::Won);
                    }
                }
            }
            _ => {}
        }
    }

    fn gen_next_snake_cell(&self, direction: &Direction) -> usize {
        let snake_idx = self.get_snake_head_idx();
        let row = snake_idx / self.width;
        match direction {
            Direction::Right => {
                let threshold = (row + 1) * self.width;
                if snake_idx + 1 == threshold {
                    threshold - self.width
                } else {
                    snake_idx + 1
                }
            }
            Direction::Left => {
                let threshold = (row) * self.width;
                if snake_idx == threshold {
                    threshold + self.width - 1
                } else {
                    snake_idx - 1
                }
            }
            Direction::Up => {
                let threshold = snake_idx - (row * self.width);
                if snake_idx == threshold {
                    self.size - self.width + threshold
                } else {
                    snake_idx - self.width
                }
            }
            Direction::Down => {
                let threshold = snake_idx + (self.width - row) * self.width;
                if snake_idx + self.width == threshold {
                    threshold - (row + 1) * self.width
                } else {
                    snake_idx + self.width
                }
            }
        }
    }

    pub fn get_snake_cells(&self) -> *const usize {
        self.snake.body.as_ptr()
    }

    pub fn get_snake_length(&self) -> usize {
        self.snake.body.len()
    }
}
