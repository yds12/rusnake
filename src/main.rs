use ggez;
use ggez::{GameResult};
use ggez::graphics;
use ggez::event;

mod gamestate;
use gamestate::*;

// Number of tiles in the screen
const TILES: (u16, u16) = (12, 12);

// Size of the tiles
const TILE_SIZE: (u16, u16) = (64, 64);

// Colors of the game entities
const BG_COLOR: (u8, u8, u8) = (0, 0, 0);
const SNAKE_COLOR: (u8, u8, u8) = (127, 127, 127);
const FOOD_COLOR: (u8, u8, u8) = (255, 255, 255);

// padding around the tiles
const PADDING: u16 = 8;

// secs between each move of the snake
const TICK: f32 = 0.15;

enum Direction {
  Up,
  Right,
  Down,
  Left
}


fn main() -> GameResult {
  let cb = ggez::ContextBuilder::new("Snake", "Y.D.S.")
    .window_mode(ggez::conf::WindowMode::default().dimensions(
      (TILES.0 * TILE_SIZE.0) as f32,
      (TILES.1 * TILE_SIZE.1) as f32));

  let (ctx, event_loop) = &mut cb.build()?;
  graphics::set_window_title(ctx, "Rusty Snake: RUSNAKE");

  let mut state = GameState::new();
  event::run(ctx, event_loop, &mut state)?;

  Ok(())
}

