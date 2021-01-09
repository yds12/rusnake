use ggez::{Context, GameResult};
use ggez::input::keyboard::{self, KeyCode};
use rand::prelude::*;
use ggez::graphics;
use crate::*;

enum Direction {
  Up,
  Right,
  Down,
  Left
}

enum State {
  Waiting,
  Ongoing,
  Dead
}

pub struct GameState {
  cfg: Config,
  snake: Vec<(u16, u16)>,
  food: (u16, u16),
  direction: Direction,
  acc_time: f32,
  state: State
}

impl GameState {
  pub fn new(cfg: Config) -> Self {
    let snake = vec![(1, 1), (2, 1), (3, 1)];
    let food = (5, 5);
    let direction = Direction::Right;
    let acc_time = 0.0;
    let state = State::Waiting;

    GameState {
      cfg,
      snake,
      food,
      direction,
      acc_time,
      state
    }
  }

  fn draw_snake(&mut self, ctx: &mut Context) -> GameResult {
    for cell in &self.snake {
      let rect = graphics::Rect::new(
        (cell.0 * self.cfg.tile_size.0 + self.cfg.padding) as f32,
        (cell.1 * self.cfg.tile_size.1 + self.cfg.padding) as f32,
        (self.cfg.tile_size.0 - self.cfg.padding) as f32,
        (self.cfg.tile_size.1 - self.cfg.padding) as f32);

      let rect_mesh = graphics::Mesh::new_rectangle(ctx,
        graphics::DrawMode::fill(), rect,
        graphics::Color::from_rgb(self.cfg.snake_color.0, 
          self.cfg.snake_color.1, self.cfg.snake_color.2))?;

      graphics::draw(ctx, &rect_mesh, graphics::DrawParam::default())?;
    }

    Ok(())
  }

  fn draw_food(&mut self, ctx: &mut Context) -> GameResult {
    let rect = graphics::Rect::new(
      (self.food.0 * self.cfg.tile_size.0 + self.cfg.padding) as f32,
      (self.food.1 * self.cfg.tile_size.1 + self.cfg.padding) as f32,
      (self.cfg.tile_size.0 - self.cfg.padding) as f32,
      (self.cfg.tile_size.1 - self.cfg.padding) as f32);

    let rect_mesh = graphics::Mesh::new_rectangle(ctx,
      graphics::DrawMode::fill(), rect,
      graphics::Color::from_rgb(
        self.cfg.food_color.0, self.cfg.food_color.1, self.cfg.food_color.2))?;

    graphics::draw(ctx, &rect_mesh, graphics::DrawParam::default())?;

    Ok(())
  }

  fn move_snake(&mut self) -> GameResult {
    let last_cell = &self.snake[self.snake.len() - 1];
    let new_cell;

    match self.direction {
      Direction::Right => {
        if last_cell.0 + 1 >= self.cfg.tiles.0 {
          new_cell = (0, last_cell.1);
        } else {
          new_cell = (last_cell.0 + 1, last_cell.1);
        }
      },
      Direction::Down => {
        if last_cell.1 + 1 >= self.cfg.tiles.1 {
          new_cell = (last_cell.0, 0);
        } else {
          new_cell = (last_cell.0, last_cell.1 + 1);
        }
      },
      Direction::Left => {
        if last_cell.0 == 0 {
          new_cell = (self.cfg.tiles.0 - 1, last_cell.1);
        } else {
          new_cell = (last_cell.0 - 1, last_cell.1);
        }
      },
      Direction::Up => {
        if last_cell.1 == 0 {
          new_cell = (last_cell.0, self.cfg.tiles.1 - 1);
        } else {
          new_cell = (last_cell.0, last_cell.1 - 1);
        }
      }
    };

    self.snake.remove(0);

    if self.snake.iter().any(|&cell| cell == new_cell) {
      self.state = State::Dead;
    }

    self.snake.push(new_cell);

    Ok(())
  }

  fn update_keyboard(&mut self, ctx: &mut Context) -> GameResult {
    match self.state {
      State::Waiting => {
        if keyboard::is_key_pressed(ctx, KeyCode::Space) {
          self.state = State::Ongoing;
        }
      },
      _ => {
        if keyboard::is_key_pressed(ctx, KeyCode::Right) {
          match self.direction {
            Direction::Left => (),
            _ => self.direction = Direction::Right
          }
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Down) {
          match self.direction {
            Direction::Up => (),
            _ => self.direction = Direction::Down
          }
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Left) {
          match self.direction {
            Direction::Right => (),
            _ => self.direction = Direction::Left
          }
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Up) {
          match self.direction {
            Direction::Down => (),
            _ => self.direction = Direction::Up
          }
        }
      }
    }

    Ok(())
  }

  fn update_food(&mut self) {
    let last_cell = &self.snake[self.snake.len() - 1];

    if &self.food == last_cell {
      self.grow();
      self.gen_food();
    }
  }

  fn gen_food(&mut self) {
    let mut rng = rand::thread_rng();

    loop {
      let food_x = (rng.gen::<f32>() * (self.cfg.tiles.0 - 2) as f32) as u16 + 1;
      let food_y = (rng.gen::<f32>() * (self.cfg.tiles.1 - 2) as f32) as u16 + 1;

      if self.snake.iter().any(|&cell| cell == (food_x, food_y)) {
        continue;
      }

      self.food = (food_x, food_y);
      break;
    }
  }

  fn grow(&mut self) {
    let last_cell = self.snake[self.snake.len() - 1];

    match self.direction {
      Direction::Right => self.snake.push((last_cell.0 + 1, last_cell.1)),
      Direction::Down => self.snake.push((last_cell.0, last_cell.1 + 1)),
      Direction::Left => self.snake.push((last_cell.0 - 1, last_cell.1)),
      Direction::Up => self.snake.push((last_cell.0, last_cell.1 - 1))
    };
  }
}

impl event::EventHandler for GameState {
  fn update(&mut self, ctx: &mut Context) -> GameResult {
    self.update_keyboard(ctx)?;

    match self.state {
      State::Waiting => return Ok(()),
      State::Ongoing => {
        self.acc_time += ggez::timer::delta(ctx).as_secs_f32();

        if self.acc_time >= self.cfg.tick {
          self.move_snake()?;
          self.update_food();
          self.acc_time = 0.0;
        }
      },
      State::Dead => ()
    }

    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    graphics::clear(ctx, graphics::Color::from_rgb(
      self.cfg.bg_color.0, self.cfg.bg_color.1, self.cfg.bg_color.2));

    self.draw_snake(ctx)?;
    self.draw_food(ctx)?;

    graphics::present(ctx)?;
    Ok(())
  }
}

