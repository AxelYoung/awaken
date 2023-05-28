use harmony::*;

use super::common::Position;
use super::looping::Clone;
use super::physics::{check_collision, Bounds, Collider};
use super::player::Player;
use super::render::{Sprite, SPRITE_SIZE};
use super::Game;

pub struct MasterButton {
    pub gates: Vec<usize>,
    pub slaves: Vec<usize>,
}

pub struct SlaveButton {
    pub r#type: ButtonType,
    pub collided: Option<usize>,
}

#[derive(PartialEq, Clone, Copy)]
pub enum ButtonType {
    Any,
    AnyColor,
    Color(u8),
}

use ButtonType::*;

pub fn fixed_update(game: &mut Game) {
    check_button_collision(game);
    check_master_button(game);
}

fn check_button_collision(game: &mut Game) {
    iterate_entities!(
        game.world,
        [Position],
        (SlaveButton, Sprite),
        |button_pos: &Position, button: &mut SlaveButton, button_sprite: &mut Sprite| {
            match button.r#type {
                Any => {
                    iterate_entities_with_id!(
                        game.world,
                        [Position, Collider],
                        |id, collider_pos: &Position, _| {
                            if check_collision(
                                button_pos.value,
                                Bounds {
                                    right: SPRITE_SIZE as f32,
                                    bottom: SPRITE_SIZE as f32,
                                },
                                collider_pos.value,
                                Bounds {
                                    right: SPRITE_SIZE as f32,
                                    bottom: SPRITE_SIZE as f32,
                                },
                            ) {
                                if button.collided == None {
                                    let sprite_index = match button.r#type {
                                        Any => 23,
                                        AnyColor => 21,
                                        Color(4) => 19,
                                        Color(3) => 17,
                                        Color(2) => 15,
                                        Color(1) => 13,
                                        Color(0) => 11,
                                        _ => 0,
                                    };

                                    button_sprite.index = sprite_index;
                                    button.collided = Some(id);
                                }
                            } else if button.collided == Some(id) {
                                let sprite_index = match button.r#type {
                                    Any => 22,
                                    AnyColor => 20,
                                    Color(4) => 18,
                                    Color(3) => 16,
                                    Color(2) => 14,
                                    Color(1) => 12,
                                    Color(0) => 10,
                                    _ => 0,
                                };

                                button_sprite.index = sprite_index;
                                button.collided = None;
                            }
                        }
                    );
                }
                AnyColor => {
                    if game.clone_count > 0 {
                        iterate_entities_with_id!(
                            game.world,
                            [Position, Clone],
                            |id, collider_pos: &Position, _| {
                                if check_collision(
                                    button_pos.value,
                                    Bounds {
                                        right: SPRITE_SIZE as f32,
                                        bottom: SPRITE_SIZE as f32,
                                    },
                                    collider_pos.value,
                                    Bounds {
                                        right: SPRITE_SIZE as f32,
                                        bottom: SPRITE_SIZE as f32,
                                    },
                                ) {
                                    if button.collided == None {
                                        let sprite_index = match button.r#type {
                                            Any => 23,
                                            AnyColor => 21,
                                            Color(4) => 19,
                                            Color(3) => 17,
                                            Color(2) => 15,
                                            Color(1) => 13,
                                            Color(0) => 11,
                                            _ => 0,
                                        };

                                        button_sprite.index = sprite_index;
                                        button.collided = Some(id);
                                    }
                                } else if button.collided == Some(id) {
                                    let sprite_index = match button.r#type {
                                        Any => 22,
                                        AnyColor => 20,
                                        Color(4) => 18,
                                        Color(3) => 16,
                                        Color(2) => 14,
                                        Color(1) => 12,
                                        Color(0) => 10,
                                        _ => 0,
                                    };

                                    button_sprite.index = sprite_index;
                                    button.collided = None;
                                }
                            }
                        );
                    }
                    iterate_entities_with_id!(
                        game.world,
                        [Position, Player],
                        |id, collider_pos: &Position, _| {
                            if check_collision(
                                button_pos.value,
                                Bounds {
                                    right: SPRITE_SIZE as f32,
                                    bottom: SPRITE_SIZE as f32,
                                },
                                collider_pos.value,
                                Bounds {
                                    right: SPRITE_SIZE as f32,
                                    bottom: SPRITE_SIZE as f32,
                                },
                            ) {
                                if button.collided == None {
                                    let sprite_index = match button.r#type {
                                        Any => 23,
                                        AnyColor => 21,
                                        Color(4) => 19,
                                        Color(3) => 17,
                                        Color(2) => 15,
                                        Color(1) => 13,
                                        Color(0) => 11,
                                        _ => 0,
                                    };

                                    button_sprite.index = sprite_index;
                                    button.collided = Some(id);
                                }
                            } else if button.collided == Some(id) {
                                let sprite_index = match button.r#type {
                                    Any => 22,
                                    AnyColor => 20,
                                    Color(4) => 18,
                                    Color(3) => 16,
                                    Color(2) => 14,
                                    Color(1) => 12,
                                    Color(0) => 10,
                                    _ => 0,
                                };

                                button_sprite.index = sprite_index;
                                button.collided = None;
                            }
                        }
                    );
                }
                Color(color) => {
                    if game.clone_count > 0 {
                        iterate_entities_with_id!(
                            game.world,
                            [Position, Clone],
                            |id, collider_pos: &Position, clone: &Clone| {
                                if color == clone.color {
                                    if check_collision(
                                        button_pos.value,
                                        Bounds {
                                            right: SPRITE_SIZE as f32,
                                            bottom: SPRITE_SIZE as f32,
                                        },
                                        collider_pos.value,
                                        Bounds {
                                            right: SPRITE_SIZE as f32,
                                            bottom: SPRITE_SIZE as f32,
                                        },
                                    ) {
                                        if button.collided == None {
                                            let sprite_index = match button.r#type {
                                                Any => 23,
                                                AnyColor => 21,
                                                Color(4) => 19,
                                                Color(3) => 17,
                                                Color(2) => 15,
                                                Color(1) => 13,
                                                Color(0) => 11,
                                                _ => 0,
                                            };

                                            button_sprite.index = sprite_index;
                                            button.collided = Some(id);
                                        }
                                    } else if button.collided == Some(id) {
                                        let sprite_index = match button.r#type {
                                            Any => 22,
                                            AnyColor => 20,
                                            Color(4) => 18,
                                            Color(3) => 16,
                                            Color(2) => 14,
                                            Color(1) => 12,
                                            Color(0) => 10,
                                            _ => 0,
                                        };

                                        button_sprite.index = sprite_index;
                                        button.collided = None;
                                    }
                                }
                            }
                        );
                    }
                    iterate_entities_with_id!(
                        game.world,
                        [Position, Player],
                        |id, collider_pos: &Position, player: &Player| {
                            if color == player.color {
                                if check_collision(
                                    button_pos.value,
                                    Bounds {
                                        right: SPRITE_SIZE as f32,
                                        bottom: SPRITE_SIZE as f32,
                                    },
                                    collider_pos.value,
                                    Bounds {
                                        right: SPRITE_SIZE as f32,
                                        bottom: SPRITE_SIZE as f32,
                                    },
                                ) {
                                    if button.collided == None {
                                        let sprite_index = match button.r#type {
                                            Any => 23,
                                            AnyColor => 21,
                                            Color(4) => 19,
                                            Color(3) => 17,
                                            Color(2) => 15,
                                            Color(1) => 13,
                                            Color(0) => 11,
                                            _ => 0,
                                        };

                                        button_sprite.index = sprite_index;
                                        button.collided = Some(id);
                                    }
                                } else if button.collided == Some(id) {
                                    let sprite_index = match button.r#type {
                                        Any => 22,
                                        AnyColor => 20,
                                        Color(4) => 18,
                                        Color(3) => 16,
                                        Color(2) => 14,
                                        Color(1) => 12,
                                        Color(0) => 10,
                                        _ => 0,
                                    };

                                    button_sprite.index = sprite_index;
                                    button.collided = None;
                                }
                            }
                        }
                    );
                }
            }
        }
    );
}

fn check_master_button(game: &mut Game) {
    let mut gates_to_remove: Vec<usize> = vec![];
    let mut gates_to_add: Vec<usize> = vec![];

    iterate_entities!(
        game.world,
        [MasterButton, SlaveButton],
        |master: &MasterButton, self_slave: &SlaveButton| {
            let mut open = true;

            if self_slave.collided.is_none() {
                open = false;
            }

            iterate_entities_with_id!(game.world, [SlaveButton], |id, slave: &SlaveButton| {
                for slaves in &master.slaves {
                    if *slaves == id {
                        if slave.collided.is_none() {
                            open = false;
                        }
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
        }
    );

    for gate in gates_to_remove {
        game.world.remove_component_from_entity::<Sprite>(gate);
        game.world.remove_component_from_entity::<Collider>(gate);
    }

    for gate in gates_to_add {
        game.world
            .add_component_to_entity(gate, Sprite::new(37, 25));
        game.world.add_component_to_entity(gate, Collider {});
    }
}
