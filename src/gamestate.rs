use ggez::{Context, GameResult};
use ggez::input::keyboard::{self, KeyCode};
use rand::prelude::*;
use ggez::graphics;
use ggez::mint::Point2;
use ggez::audio;
use ggez::audio::SoundSource;
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
  Paused,
  Dead
}

pub struct GameState {
  cfg: Config,
  snake: Vec<(u16, u16)>,
  food: (u16, u16),
  direction: Direction,
  acc_time: f32,
  state: State,
  eat_sound: Option<Box<dyn SoundSource>>
}

impl GameState {
  pub fn new(cfg: Config) -> Self {
    let snake = vec![(1, 1), (2, 1), (3, 1)];
    let food = (5, 5);
    let direction = Direction::Right;
    let acc_time = 0.0;
    let state = State::Waiting;
    let eat_sound = None;

    GameState {
      cfg,
      snake,
      food,
      direction,
      acc_time,
      state,
      eat_sound
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

  fn draw_dead_text(&mut self, ctx: &mut Context) -> GameResult {
    let mut param = graphics::DrawParam::default();
    param.dest = Point2 { x: 10.0, y: 10.0 };

    let text_fragment =
      graphics::TextFragment::new("You died! Press SPACE to restart.")
      .color(graphics::Color::new(
        self.cfg.text_color.0 as f32,
        self.cfg.text_color.1 as f32,
        self.cfg.text_color.2 as f32, 255.0));

    let text = graphics::Text::new(text_fragment);

    graphics::draw(ctx, &text, param)?;
    Ok(())
  }

  pub fn load_sounds(&mut self, ctx: &mut Context) -> GameResult {
    self.eat_sound = Some(Box::new(audio::Source::new(ctx, "/coin.wav")?));
    Ok(())
  }

  fn play_eat_sound(&mut self) -> GameResult {
    self.eat_sound.as_mut().unwrap().play_detached()?;
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
      State::Waiting | State::Paused => {
        if keyboard::is_key_pressed(ctx, KeyCode::Space) &&
           !keyboard::is_key_repeated(ctx) {
          self.state = State::Ongoing;
        }
      },
      State::Ongoing => {
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
        if keyboard::is_key_pressed(ctx, KeyCode::Space) &&
           !keyboard::is_key_repeated(ctx) {
          self.state = State::Paused;
        }
      },
      State::Dead => {
        if keyboard::is_key_pressed(ctx, KeyCode::Space) &&
           !keyboard::is_key_repeated(ctx) {
          *self = GameState::new(self.cfg);
          self.load_sounds(ctx)?;
        }
      }
    }

    Ok(())
  }

  fn update_food(&mut self) -> GameResult {
    let last_cell = &self.snake[self.snake.len() - 1];

    if &self.food == last_cell {
      self.play_eat_sound()?;
      self.grow();
      self.gen_food();
    }

    Ok(())
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
      State::Waiting | State::Paused => return Ok(()),
      State::Ongoing => {
        self.acc_time += ggez::timer::delta(ctx).as_secs_f32();

        if self.acc_time >= self.cfg.tick {
          self.move_snake()?;
          self.update_food()?;
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

    match self.state {
      State::Dead => self.draw_dead_text(ctx)?,
      _ => ()
    }

    graphics::present(ctx)?;
    Ok(())
  }
}

