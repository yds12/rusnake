use ggez;
use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::event;
use ggez::input::keyboard::{self, KeyCode};
use rand::prelude::*;

const SCREEN: (f32, f32) = (800.0, 600.0);
const TILES: (u16, u16) = (40, 30);
const TILE_SIZE: (u16, u16) = (20, 20);
const BG_COLOR: (u8, u8, u8) = (127, 127, 255);
const SNAKE_COLOR: (u8, u8, u8) = (0, 255, 0);
const FOOD_COLOR: (u8, u8, u8) = (0, 255, 0);

enum Direction {
  Up,
  Right,
  Down,
  Left
}

struct GameState {
  snake: Vec<(u16, u16)>,
  food: (u16, u16),
  direction: Direction
}

impl GameState {
  pub fn new() -> Self {
    let mut rng = rand::thread_rng();
    let mut snake = vec![(1, 1), (2, 1), (3, 1)];
    let mut food = (5, 5);
    let mut direction = Direction::Right;

    GameState {
      snake,
      food,
      direction
    }
  }

  fn draw_snake(&mut self, ctx: &mut Context) -> GameResult {
    for cell in &self.snake {
      let rect = graphics::Rect::new(
        (cell.0 * TILE_SIZE.0) as f32,
        (cell.1 * TILE_SIZE.1) as f32,
        TILE_SIZE.0 as f32, TILE_SIZE.1 as f32);

      let rect_mesh = graphics::Mesh::new_rectangle(ctx,
        graphics::DrawMode::fill(), rect,
        graphics::Color::from_rgb(
          SNAKE_COLOR.0, SNAKE_COLOR.1, SNAKE_COLOR.2))?;

        graphics::draw(ctx, &rect_mesh, graphics::DrawParam::default())?;

    }

    Ok(())
  }
}

impl event::EventHandler for GameState {
  fn update(&mut self, ctx: &mut Context) -> GameResult {
    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    graphics::clear(ctx, graphics::Color::from_rgb(
      BG_COLOR.0, BG_COLOR.1, BG_COLOR.2));

    self.draw_snake(ctx);

    graphics::present(ctx)?;
    Ok(())
  }
}

fn main() -> GameResult {
  let cb = ggez::ContextBuilder::new("Snake", "Y.D.S.")
    .window_mode(ggez::conf::WindowMode::default()
      .dimensions(SCREEN.0, SCREEN.1));

  let (ctx, event_loop) = &mut cb.build()?;
  graphics::set_window_title(ctx, "Rusty Snake: RUSNAKE");

  let mut state = GameState::new();
  event::run(ctx, event_loop, &mut state);

  Ok(())
}
