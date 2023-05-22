use harmony::*;
use super::common::Position;
use super::physics::{Bounds, Collider, check_collision};
use super::render::{Sprite, SPRITE_SIZE};


pub struct Button {
    pub gate_ids: Vec<usize>,
    pub collided: Option<usize>
}

pub fn fixed_update(world: &mut World) {
    check_button_collision(world);
}

fn check_button_collision(world: &mut World) {
    let mut gates_to_remove: Vec<usize> = vec![];
    let mut gates_to_add: Vec<usize> = vec![];

    iterate_entities_with_id!(world, [Position], (Collider), |id, position_a: &Position, _| {
        iterate_entities!(world, [Position], (Button, Sprite), 
        |position_b: &Position, button: &mut Button, sprite: &mut Sprite| {
            if check_collision(position_a.value, Bounds{right: SPRITE_SIZE as f32, bottom: SPRITE_SIZE as f32}, position_b.value, Bounds{right: SPRITE_SIZE as f32, bottom: SPRITE_SIZE as f32}) {
                if button.collided == None {
                    for gate in &button.gate_ids {
                        gates_to_remove.push(*gate);
                    }
                    sprite.index = 39;
                    button.collided = Some(id);
                }
            } else if button.collided == Some(id) {
                for gate in &button.gate_ids {
                    gates_to_add.push(*gate);
                }
                sprite.index = 38;
                button.collided = None;
            }
        }
    );
    });

    for gate in gates_to_add {
        world.add_component_to_entity(gate, Sprite::new(37));
        world.add_component_to_entity(gate, Collider{});
    }

    for gate in gates_to_remove {
        world.remove_component_from_entity::<Sprite>(gate);
        world.remove_component_from_entity::<Collider>(gate);
    }
}