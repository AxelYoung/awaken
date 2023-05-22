use harmony::*;

use super::{Game, SCREEN_WIDTH, SCREEN_HEIGHT};
use super::common::Position;
use super::math::Vec2;
use super::physics::{Collider, Velocity};
use super::render::{Sprite, SPRITE_SIZE};
use super::animation::*;
use super::map_gen::ROOM_HEIGHT;

pub struct Player {
    pub speed: f32,
    pub active: bool,
    pub dir: Vec2
}

pub fn create(game: &mut Game) {
    let e = game.world.new_entity();

    game.world.add_component_to_entity(e, Sprite::new(0));
    game.world.add_component_to_entity(e, Position::new(SCREEN_WIDTH as f32 / 2.0, (SCREEN_HEIGHT as f32 / 2.0) - (ROOM_HEIGHT as u16 * SPRITE_SIZE) as f32));
    game.world.add_component_to_entity(e, Velocity::new(0.0, 0.0));
    game.world.add_component_to_entity(e, Player {speed: 0.8, active: true, dir: Vec2::new(0.0, 0.0)});
    game.world.add_component_to_entity(e, Collider{});
    game.world.add_component_to_entity(e, Animator{
        animation: Animation {
            frames: vec![AnimationFrame::new(1, 75), AnimationFrame::new(0, 75)],
            r#loop: true
        },
        frame_index: 0,
        time: 0,
        playing: false
    });

    game.chroma.update_camera(0.0, 4.0);

    game.player = e;
}

pub fn update(game: &mut Game) {
    set_dir(game);
}

fn set_dir(game: &mut Game) {
    iterate_entities!(game.world, (Player, Velocity, Animator, Sprite),
        |moveable: &mut Player, velocity: &mut Velocity, animator: &mut Animator, sprite: &mut Sprite| {
            let dir_x : f32 = 
            if game.input.right_pressed {
                sprite.flip_x = false;
                 1.0 
            } else if game.input.left_pressed {
                sprite.flip_x = true;
                 -1.0 
            } else { 0.0 };

            let dir_y : f32 = if game.input.up_pressed { 1.0 } else if game.input.down_pressed { -1.0 } else { 0.0 };
        
            let magnitude = dir_x.abs() + dir_y.abs();
        
            let normalized_x = dir_x / magnitude;
            let normalized_y = dir_y / magnitude;
            
            moveable.dir.x = normalized_x;
            moveable.dir.y = normalized_y;

            if moveable.dir.x.is_nan() { moveable.dir = Vec2::new(0.0, 0.0); }

            if game.clone_commands[game.current_clone].len() == 0 ||
                game.clone_commands[game.current_clone][game.clone_commands[game.current_clone].len() - 1].0 != moveable.dir {
                    game.clone_commands[game.current_clone].push((moveable.dir, game.time));
                }

            if magnitude != 0.0 {
                velocity.x = normalized_x * moveable.speed;
                velocity.y = normalized_y * moveable.speed;
                animator.playing = true;
            } else {
                animator.playing = false;
                animator.time = 0;
            }
        }
    );
}