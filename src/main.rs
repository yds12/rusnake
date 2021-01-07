use ggez;
use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::event;
use ggez::input::keyboard::{self, KeyCode};
use rand::prelude::*;

// Number of tiles in the screen
const TILES: (u16, u16) = (10, 10);

// Size of the tiles
const TILE_SIZE: (u16, u16) = (20, 20);

// Colors of the game entities
const BG_COLOR: (u8, u8, u8) = (127, 127, 255);
const SNAKE_COLOR: (u8, u8, u8) = (63, 127, 63);
const FOOD_COLOR: (u8, u8, u8) = (255, 127, 0);

// padding around the tiles
const PADDING: u16 = 2;

// secs between each move of the snake
const TICK: f32 = 0.3;

enum Direction {
  Up,
  Right,
  Down,
  Left
}

struct GameState {
  snake: Vec<(u16, u16)>,
  food: (u16, u16),
  direction: Direction,
  acc_time: f32
}

impl GameState {
  pub fn new() -> Self {
    let mut rng = rand::thread_rng();
    let mut snake = vec![(1, 1), (2, 1), (3, 1)];
    let mut food = (5, 5);
    let mut direction = Direction::Right;
    let mut acc_time = 0.0;

    GameState {
      snake,
      food,
      direction,
      acc_time
    }
  }

  fn draw_snake(&mut self, ctx: &mut Context) -> GameResult {
    for cell in &self.snake {
      let rect = graphics::Rect::new(
        (cell.0 * TILE_SIZE.0 + PADDING) as f32,
        (cell.1 * TILE_SIZE.1 + PADDING) as f32,
        (TILE_SIZE.0 - PADDING) as f32,
        (TILE_SIZE.1 - PADDING) as f32);

      let rect_mesh = graphics::Mesh::new_rectangle(ctx,
        graphics::DrawMode::fill(), rect,
        graphics::Color::from_rgb(
          SNAKE_COLOR.0, SNAKE_COLOR.1, SNAKE_COLOR.2))?;

      graphics::draw(ctx, &rect_mesh, graphics::DrawParam::default())?;
    }

    Ok(())
  }

  fn draw_food(&mut self, ctx: &mut Context) -> GameResult {
    let rect = graphics::Rect::new(
      (self.food.0 * TILE_SIZE.0 + PADDING) as f32,
      (self.food.1 * TILE_SIZE.1 + PADDING) as f32,
      (TILE_SIZE.0 - PADDING) as f32,
      (TILE_SIZE.1 - PADDING) as f32);

    let rect_mesh = graphics::Mesh::new_rectangle(ctx,
      graphics::DrawMode::fill(), rect,
      graphics::Color::from_rgb(FOOD_COLOR.0, FOOD_COLOR.1, FOOD_COLOR.2))?;

    graphics::draw(ctx, &rect_mesh, graphics::DrawParam::default())?;

    Ok(())
  }

  fn move_snake(&mut self) -> GameResult {
    let last_cell = &self.snake[self.snake.len() - 1];

    match self.direction {
      Direction::Right => self.snake.push((last_cell.0 + 1, last_cell.1)),
      Direction::Down => self.snake.push((last_cell.0, last_cell.1 + 1)),
      Direction::Left => self.snake.push((last_cell.0 - 1, last_cell.1)),
      Direction::Up => self.snake.push((last_cell.0, last_cell.1 - 1))
    };

    self.snake.remove(0);
    Ok(())
  }

  fn update_keyboard(&mut self, ctx: &mut Context) -> GameResult {
    if keyboard::is_key_pressed(ctx, KeyCode::Right) {
      self.direction = Direction::Right;
    }
    if keyboard::is_key_pressed(ctx, KeyCode::Down) {
      self.direction = Direction::Down;
    }
    if keyboard::is_key_pressed(ctx, KeyCode::Left) {
      self.direction = Direction::Left;
    }
    if keyboard::is_key_pressed(ctx, KeyCode::Up) {
      self.direction = Direction::Up;
    }

    Ok(())
  }

  fn update_food(&mut self) {
    let last_cell = &self.snake[self.snake.len() - 1];

    if &self.food == last_cell {
      let mut rng = rand::thread_rng();
      let food_x = (rng.gen::<f32>() * TILES.0 as f32) as u16;
      let food_y = (rng.gen::<f32>() * TILES.0 as f32) as u16;

      self.food = (food_x, food_y);
      self.grow();
    }
  }

  fn grow(&mut self) {
    let last_cell = &self.snake[self.snake.len() - 1];
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
    self.acc_time += ggez::timer::delta(ctx).as_secs_f32();

    if self.acc_time >= TICK {
      self.move_snake();
      self.acc_time = 0.0;
    }

    self.update_keyboard(ctx);
    self.update_food();

    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    graphics::clear(ctx, graphics::Color::from_rgb(
      BG_COLOR.0, BG_COLOR.1, BG_COLOR.2));

    self.draw_snake(ctx);
    self.draw_food(ctx);

    graphics::present(ctx)?;
    Ok(())
  }
}

fn main() -> GameResult {
  let cb = ggez::ContextBuilder::new("Snake", "Y.D.S.")
    .window_mode(ggez::conf::WindowMode::default().dimensions(
      (TILES.0 * TILE_SIZE.0) as f32,
      (TILES.1 * TILE_SIZE.1) as f32));

  let (ctx, event_loop) = &mut cb.build()?;
  graphics::set_window_title(ctx, "Rusty Snake: RUSNAKE");

  let mut state = GameState::new();
  event::run(ctx, event_loop, &mut state);

  Ok(())
}
