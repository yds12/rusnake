use ggez;
use ggez::{GameResult};
use ggez::graphics;
use ggez::event;

mod config;
use config::*;

mod gamestate;
use gamestate::*;

fn main() -> GameResult {
  let cfg = get_config();
  let cb = ggez::ContextBuilder::new("Snake", "Y.D.S.")
    .window_mode(ggez::conf::WindowMode::default().dimensions(
      (cfg.tiles.0 * cfg.tile_size.0) as f32,
      (cfg.tiles.1 * cfg.tile_size.1) as f32));

  let (ctx, event_loop) = &mut cb.build()?;
  graphics::set_window_title(ctx, "Rusty Snake: RUSNAKE");

  let mut state = GameState::new(cfg);
  event::run(ctx, event_loop, &mut state)?;

  Ok(())
}

