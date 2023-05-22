use chroma::*;
use harmony::*;
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

pub fn draw(world: &mut World, chroma: &mut Chroma){
    chroma.clear();

    draw_entity(world, chroma);

    chroma.render();
}

fn draw_entity(world: &mut World, chroma: &mut Chroma) {
    iterate_entities!(world, [Position, Sprite], 
        |position: &Position, sprite : &Sprite| {
            chroma.add_tile(position.x, position.y, sprite.index);
        }
    );
}