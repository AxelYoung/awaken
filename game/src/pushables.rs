use harmony::*;
use crate::Game;

use super::common::Position;
use super::physics::{Collider, Bounds, Velocity, check_collision};
use super::math::Vec2;
use super::render::SPRITE_SIZE;

pub struct Pushable { 
    pub origin: Vec2
}


pub fn fixed_update(game: &mut Game) {
    check_pushable_collision(game);
}

fn check_pushable_collision(game: &mut Game) {

    let mut vel : Option<(f32, f32)> = None;
    let mut pushable : Option<usize> = None;

    iterate_entities!(game.world, [Position, Collider], (Velocity), 
        |position_a: &Position, _, velocity: &mut Velocity| {            
            iterate_entities_with_id!(game.world, [Collider, Pushable, Position], 
                |id, _, _, position_b: &Position| {
                    let next_pos = Vec2::new(position_a.x + velocity.x, position_a.y+ velocity.y);
                    if check_collision(next_pos, Bounds{right: SPRITE_SIZE as f32, bottom: SPRITE_SIZE as f32}, position_b.value, Bounds{right: SPRITE_SIZE as f32, bottom: SPRITE_SIZE as f32}) {
                        pushable = Some(id);
                        velocity.x *= 0.5;
                        velocity.y *= 0.5;
                        vel = Some((velocity.x, velocity.y));
                    }
                }
            );
        }
    );

    if let Some(id) = pushable {
        let pos = game.world.get_component_from_entity_mut::<Velocity>(id).unwrap().as_mut().unwrap();
        pos.x = vel.unwrap().0;
        pos.y = vel.unwrap().1;
    }
}