use harmony::*;
use chroma::*;

use super::Game;
use super::common::Position;
use super::math::Vec2;
use super::physics::{Bounds, check_collision};
use super::render::{SPRITE_SIZE};

pub struct Transition {
    pub dir: Vec2,
    pub collided: bool
}

pub fn fixed_update(game: &mut Game) {
    check_transitions_collision(game);
}

fn check_transitions_collision(game: &mut Game) {
    let player_position = game.world.borrow_components::<Position>().unwrap();
    let player_position = player_position[game.player].as_ref();
    iterate_entities!(game.world, [Position], (Transition), 
        |position_b: &Position, transition: &mut Transition| {
            let player_position = player_position.unwrap().value;
            if check_collision(player_position, Bounds{right: SPRITE_SIZE as f32, bottom: SPRITE_SIZE as f32}, position_b.value, Bounds{right: SPRITE_SIZE as f32, bottom: 0.1 as f32}) {
                if !transition.collided {
                    transition.collided = true;
                    if transition.dir.x == 0.0 {
                        let dir = if player_position.y + 4.0 > position_b.y { -1.0 } else { 1.0 };
                        game.chroma.update_camera(game.chroma.camera.x, game.chroma.camera.y - (dir * 4.0));
                    } else {
                        let dir = if player_position.x + 4.0 > position_b.x { -1.0 } else { 1.0 };
                        game.chroma.update_camera(game.chroma.camera.x + (dir * 4.0), game.chroma.camera.y);
                    }
                }
            } else {
                transition.collided = false;
            }
        }
    );
}