use harmony::*;

use super::Game;
use super::common::Position;

pub const SPRITE_SIZE: u8 = 8;
pub const SPRITE_CENTER: f32 = SPRITE_SIZE as f32 / 2.0;

pub struct Sprite {
    pub index: u32,
    pub flip_x: bool,
    pub layer: i8
}

impl Sprite {
    pub fn new(index: u32, layer: i8) -> Self {
        Self {
            index,
            flip_x: false,
            layer
        }
    }
}

pub fn draw(game: &mut Game){
    game.chroma.clear();

    draw_entity(game);

    game.chroma.render();
}

fn draw_entity(game: &mut Game) {
    iterate_entities!(game.world, [Position, Sprite], 
        |position: &Position, sprite : &Sprite| {
            game.chroma.add_tile(position.x, position.y, (127 - sprite.layer) as f32 / 10000.0, sprite.index);
        }
    );
}