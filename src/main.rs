use ggez;
use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::event;
use ggez::input::keyboard::{self, KeyCode};
use rand::prelude::*;

const SPEED: f32 = 100.0;
const SCREEN: (f32, f32) = (800.0, 600.0);
const TILES: (u16, u16) = (40, 30);
const TILE_SIZE: (u16, u16) = (20, 20);

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
    let mut snake = vec![];
    let mut food = (0, 0);
    let mut direction = Direction::Right;

    GameState {
      snake,
      food,
      direction
    }
  }
}

impl event::EventHandler for GameState {
  fn update(&mut self, ctx: &mut Context) -> GameResult {
    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    graphics::clear(ctx, graphics::Color::from_rgb(127, 127, 255));
    graphics::present(ctx)?;
    Ok(())
  }
}

fn main() -> GameResult {
  let cb = ggez::ContextBuilder::new("Snake", "Y.D.S.")
    .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN.0, SCREEN.1));
  let (ctx, event_loop) = &mut cb.build()?;
  graphics::set_window_title(ctx, "Rusty Snake: RUSNAKE");

  let mut state = GameState::new();
  event::run(ctx, event_loop, &mut state);

  Ok(())
}
