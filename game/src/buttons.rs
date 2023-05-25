use harmony::*;

use super::Game;
use super::common::Position;
use super::physics::{Bounds, Collider, check_collision};
use super::render::{Sprite, SPRITE_SIZE};

pub struct MasterButton {
    pub gates: Vec<usize>,
    pub slaves: Vec<usize>
}

pub struct SlaveButton {
    pub collided: Option<usize>
}

pub fn fixed_update(game: &mut Game) {
    check_button_collision(game);
    check_master_button(game);
}

fn check_button_collision(game: &mut Game) {
    iterate_entities_with_id!(game.world, [Position], (Collider), 
    |id, position_a: &Position, _| {
        iterate_entities!(game.world, [Position], (SlaveButton, Sprite), 
        |position_b: &Position, button: &mut SlaveButton, sprite: &mut Sprite| {
            if check_collision(position_a.value, Bounds{right: SPRITE_SIZE as f32, bottom: SPRITE_SIZE as f32}, position_b.value, Bounds{right: SPRITE_SIZE as f32, bottom: SPRITE_SIZE as f32}) {
                if button.collided == None {
                    sprite.index = 23;
                    button.collided = Some(id);
                }
            } else if button.collided == Some(id) {
                sprite.index = 22;
                button.collided = None;
            }
        });
    });
}

fn check_master_button(game: &mut Game) {
    let mut gates_to_remove: Vec<usize> = vec![];
    let mut gates_to_add: Vec<usize> = vec![];

    iterate_entities!(game.world, [MasterButton, SlaveButton], 
    |master: &MasterButton, self_slave: &SlaveButton| {
        let mut open = true;

        if self_slave.collided.is_none() { open = false; }

        iterate_entities_with_id!(game.world, [SlaveButton], |id, slave: &SlaveButton| {
            for slaves in &master.slaves {
                if *slaves == id {
                    if slave.collided.is_none() { open = false; }
                }
            }
        });

        if open {
            for gate in &master.gates {
                gates_to_remove.push(*gate);
            }
        } else {
            for gate in &master.gates {
                gates_to_add.push(*gate);
            }
        }
    });

    for gate in gates_to_remove {
        game.world.remove_component_from_entity::<Sprite>(gate);
        game.world.remove_component_from_entity::<Collider>(gate);
    }

    for gate in gates_to_add {
        game.world.add_component_to_entity(gate, Sprite::new(37, 25));
        game.world.add_component_to_entity(gate, Collider{});
    }
}