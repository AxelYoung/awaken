use harmony::*;

use crate::pushables::Pushable;

use super::{Game, SCREEN_WIDTH, SCREEN_HEIGHT};
use super::common::Position;
use super::math::Vec2;
use super::physics::{Collider, Velocity};
use super::render::{Sprite, SPRITE_SIZE};
use super::animation::*;

const TRAIL_FREQUENCY: u128 = 10;

pub struct Player {
    pub speed: f32,
    pub active: bool,
    pub dir: Vec2,
    pub color: u8,
    trail_timer: u128
}

pub struct PlayerMarker{}
pub struct Trail{
    pub color: u8
}

pub fn create(game: &mut Game, clone: usize) {
    let player = game.world.new_entity();

    game.world.add_component_to_entity(player, Sprite::new(0, 50));
    game.world.add_component_to_entity(player, Position {value: game.clone_spawns[clone]});
    game.world.add_component_to_entity(player, Velocity::new(0.0, 0.0));
    game.world.add_component_to_entity(player, Player {speed: 0.8, active: true, dir: Vec2::new(0.0, 0.0), color: 0, trail_timer: 0});
    game.world.add_component_to_entity(player, Collider{});
    game.world.add_component_to_entity(player, Pushable { origin: Vec2::zero() });
    game.world.add_component_to_entity(player, Animator{
        animation: Animation {
            frames: vec![AnimationFrame::new(1, 75), AnimationFrame::new(0, 75)],
            r#loop: true
        },
        frame_index: 0,
        time: 0,
        playing: false
    });
    
    let marker = game.world.new_entity();

    game.world.add_component_to_entity(marker, PlayerMarker{});
    game.world.add_component_to_entity(marker, Sprite::new(24, 100));
    game.world.add_component_to_entity(marker, Position::new(0.0, 0.0));

    game.chroma.update_camera(-4.0, 8.0);

    game.player = player;
}

pub fn update(game: &mut Game) {
    set_dir(game);
    set_marker(game);
    create_trail(game);
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

fn set_marker(game: &mut Game) {
    let player_pos = game.world.get_component_from_entity_mut::<Position>(game.player).unwrap().as_mut().unwrap().value;
    iterate_entities!(game.world, [PlayerMarker], (Position),
    |_, marker_pos: &mut Position| {
        marker_pos.value = Vec2::new(player_pos.x, player_pos.y + 5.0);
    });
}

fn create_trail(game: &mut Game) {

    let mut create = false;

    let player_pos = game.world.get_component_from_entity_mut::<Position>(game.player).unwrap().as_mut().unwrap().value;

    let player = game.world.get_component_from_entity_mut::<Player>(game.player).unwrap().as_mut().unwrap();

    let player_color = player.color;
   
    player.trail_timer += game.delta_time;
    if player.trail_timer >= TRAIL_FREQUENCY {
        create = true;
        player.trail_timer = 0;
    }

    if create {
        let trail = game.world.new_entity();
        game.world.add_component_to_entity(trail, Sprite::new(player_color as u32 + 46, 10));
        game.world.add_component_to_entity(trail, Position::new(player_pos.x, player_pos.y));
        game.world.add_component_to_entity(trail, Trail {color: player_color});
        create = false;
    }
}