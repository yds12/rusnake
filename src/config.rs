use std::fs;

const CONFIG_PATH: &str = "config.txt";

#[derive(Clone, Copy)]
pub struct Config {
  pub tiles: (u16, u16),
  pub tile_size: (u16, u16),
  pub bg_color: (u8, u8, u8),
  pub snake_color: (u8, u8, u8),
  pub food_color: (u8, u8, u8),
  pub text_color: (u8, u8, u8),
  pub padding: u16,
  pub tick: f32
}

fn get_u16pair(value: &str) -> (u16, u16) {
  let items: Vec<&str> = value.trim().split(",").collect();
  (items[0].parse::<u16>().unwrap(), items[1].parse::<u16>().unwrap())
}

fn get_color(value: &str) -> (u8, u8, u8) {
  let items: Vec<&str> = value.trim().split(",").collect();
  (items[0].parse::<u8>().unwrap(), 
   items[1].parse::<u8>().unwrap(),
   items[2].parse::<u8>().unwrap())
}

pub fn get_config() -> Config {
  let data = fs::read_to_string(CONFIG_PATH).expect("Failed to read config file.");
  let lines = data.split("\n");

  let mut tiles: (u16, u16) = (12, 12);
  let mut tile_size: (u16, u16) = (64, 64);
  let mut bg_color: (u8, u8, u8) = (0, 0, 0);
  let mut snake_color: (u8, u8, u8) = (127, 127, 127);
  let mut food_color: (u8, u8, u8) = (255, 255, 255);
  let mut text_color: (u8, u8, u8) = (255, 255, 127);
  let mut padding: u16 = 8;
  let mut tick: f32 = 0.15;

  for line in lines {
    if !line.trim().is_empty() {
      let parts: Vec<&str> = line.split("=").collect();

      if parts.len() == 2 {
        match &parts[0].trim().to_uppercase()[..] {
          "TILES" => tiles = get_u16pair(parts[1]),
          "TILE_SIZE" => tile_size = get_u16pair(parts[1]),
          "BG_COLOR" => bg_color = get_color(parts[1]),
          "SNAKE_COLOR" => snake_color = get_color(parts[1]),
          "FOOD_COLOR" => food_color = get_color(parts[1]),
          "TEXT_COLOR" => text_color = get_color(parts[1]),
          "PADDING" => padding = parts[1].trim().parse::<u16>().unwrap(),
          "TICK" => tick = parts[1].trim().parse::<f32>().unwrap(),
          _ => println!("Unrecognized configuration key: {}",
                 parts[0].trim().to_uppercase())
        }
      }
      else {
        println!("Configuration error!");
      }
    }
  }

  Config {
    tiles,
    tile_size,
    bg_color,
    snake_color,
    food_color,
    text_color,
    padding,
    tick
  }
}

