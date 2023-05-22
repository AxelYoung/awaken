use harmony::*;
use super::Game;
use super::common::Position;

pub const SPRITE_SIZE: u16 = 8;

pub struct Sprite {
    pub index: u32,
    pub flip_x: bool
}

impl Sprite {
    pub fn new(index: u32) -> Self {
        Self {
            index,
            flip_x: false
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
            game.chroma.add_tile(position.x, position.y, sprite.index);
        }
    );
}