use std::path;
use std::env;
use ggez;
use ggez::{GameResult};
use ggez::graphics;
use ggez::event;
use ggez::audio;
use ggez::audio::SoundSource;

mod config;
use config::*;

mod gamestate;
use gamestate::*;

fn main() -> GameResult {
  let cfg = get_config();

  let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
    let mut path = path::PathBuf::from(manifest_dir);
    path.push("data");
    path
  } else {
    path::PathBuf::from("./data")
  };

  let cb = ggez::ContextBuilder::new("Snake", "Y.D.S.")
    .add_resource_path(resource_dir)
    .window_mode(ggez::conf::WindowMode::default().dimensions(
      (cfg.tiles.0 * cfg.tile_size.0) as f32,
      (cfg.tiles.1 * cfg.tile_size.1) as f32));

  let (ctx, event_loop) = &mut cb.build()?;
  graphics::set_window_title(ctx, "Rusty Snake: RUSNAKE");

  let mut state = GameState::new(cfg);
  state.load_sounds(ctx)?;
  let mut music = audio::Source::new(ctx, "/synthwave.ogg")?;
  music.set_repeat(true);
  music.set_volume(0.5);
  music.play_detached()?;
  event::run(ctx, event_loop, &mut state)?;

  Ok(())
}

