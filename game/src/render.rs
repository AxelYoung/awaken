use harmony::*;

use super::Game;
use super::common::Position;

pub const SPRITE_SIZE: u32 = 16;
pub const SPRITE_CENTER: f32 = SPRITE_SIZE as f32 / 2.0;

#[derive(Clone, Copy, PartialEq)]
pub struct Sprite {
  pub index_x: u32,
  pub index_y: u32,
  pub flip_x: bool,
  pub layer: i8,
  pub render: bool
}

impl Sprite {
  pub fn new(index_x: u32, index_y: u32, layer: i8) -> Self {
    Self {
      index_x,
      index_y,
      flip_x: false,
      layer,
      render: true
    }
  }
}

pub fn draw(game: &mut Game){
  game.chroma.pixel_pipeline.clear();

  draw_entity(game);

  game.chroma.render();
}

fn draw_entity(game: &mut Game) {
  iterate_entities!(game.world, [Position, Sprite], 
    |position: &Position, sprite : &Sprite| {
      if sprite.render {
        let z = (127 - sprite.layer) as f32 / 10000.0;
        game.chroma.pixel_pipeline.add_tile(
          position.x, position.y, z, 
          sprite.index_x, sprite.index_y
        );
      }
    }
  );
}