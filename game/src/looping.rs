use chroma::*;
use harmony::*;

use super::{Game, SCREEN_WIDTH, SCREEN_HEIGHT};
use super::input::Input;
use super::common::Position;
use super::physics::{Collider, Velocity};
use super::animation::{Animator, Animation, AnimationFrame};
use super::render::{Sprite, SPRITE_SIZE};
use super::player::Player;
use super::map_gen::ROOM_HEIGHT;

struct Timer {
    first_digit: usize,
    second_digit: usize,
    current_time: f32
}

struct Clone { }

pub fn create_ui(world: &mut World, game: &mut Game) {
    let loop_color = world.new_entity();

    world.add_component_to_entity(loop_color, Position::new(0.0, -14.0 * SPRITE_SIZE as f32));
    world.add_component_to_entity(loop_color, Sprite::new(20));

    let first_digit = world.new_entity();

    world.add_component_to_entity(first_digit, Position::new(1.0 * SPRITE_SIZE as f32, -14.0 * SPRITE_SIZE as f32));
    world.add_component_to_entity(first_digit, Sprite::new(25));

    let second_digit = world.new_entity();

    world.add_component_to_entity(second_digit, Position::new(2.0 * SPRITE_SIZE as f32, -14.0 * SPRITE_SIZE as f32));
    world.add_component_to_entity(second_digit, Sprite::new(25));

    let timer = world.new_entity();

    world.add_component_to_entity(timer, Timer {first_digit, second_digit, current_time: 30.0});

    game.timer = timer;
}

pub fn update(world: &mut World, chroma: &mut Chroma, input: &Input, game: &mut Game, delta_time: &u128) {
    restart_loop(world, chroma, input, game);
    set_timer(world, delta_time, game);
    replay_clones(world, game);
}

fn replay_clones(world: &mut World,  game: &mut Game) {
    if game.clone_count > 0 {
        let mut x = 0;

        while game.time > game.clone_commands[0][x].1 { x += 1 };
    
        let dir = game.clone_commands[0][x].0;
    
        iterate_entities!(world, (Clone, Velocity, Animator, Sprite),
            |_, velocity: &mut Velocity, animator: &mut Animator, sprite: &mut Sprite| {
    
                let dir_x = dir.x;
                let dir_y = dir.y; 
    
                let magnitude = dir_x.abs() + dir_y.abs();
            
                let normalized_x = dir_x / magnitude;
                let normalized_y = dir_y / magnitude;
    
                if magnitude != 0.0 {
                    velocity.x = normalized_x * 0.8;
                    velocity.y = normalized_y * 0.8;
                    animator.playing = true;
                } else {
                    animator.playing = false;
                    animator.time = 0;
                }
            }
        );
    }
}

fn restart_loop(world: &mut World, chroma: &mut Chroma, input: &Input, game: &mut Game) {
    if input.loop_pressed {
        world.remove_component_from_entity::<Player>(game.player);
        world.remove_component_from_entity::<Animator>(game.player);
        world.remove_component_from_entity::<Sprite>(game.player);
        world.remove_component_from_entity::<Collider>(game.player);
        let clone = world.new_entity();
        game.clones[0] = clone;
        world.add_component_to_entity(clone, Sprite::new(0));
        world.add_component_to_entity(clone, Position::new(SCREEN_WIDTH as f32 / 2.0, (SCREEN_HEIGHT as f32 / 2.0) - (ROOM_HEIGHT as u16 * SPRITE_SIZE) as f32));
        world.add_component_to_entity(clone, Velocity::new(0.0, 0.0));
        world.add_component_to_entity(clone, Collider{});
        world.add_component_to_entity(clone, Clone{});
        world.add_component_to_entity(clone, Animator{
            animation: Animation {
                frames: vec![AnimationFrame::new(1, 75), AnimationFrame::new(0, 75)],
                r#loop: true
            },
            frame_index: 0,
            time: 0,
            playing: false
        });
        game.clone_count += 1;
        game.time = 0;
    }
}

fn set_timer(world: &mut World, delta_time: &u128, game: &mut Game) {
    let first_digit;
    let second_digit;
    let first_digit_id;
    let second_digit_id;

    {
        let timer = world.get_component_from_entity_mut::<Timer>(game.timer).unwrap();
        
        timer.as_mut().unwrap().current_time -= *delta_time as f32 / 1000.0;

        first_digit = (timer.as_ref().unwrap().current_time / 10.0) as u8;
        second_digit = (timer.as_ref().unwrap().current_time - first_digit as f32 * 10.0) as u8;

        first_digit_id = timer.as_ref().unwrap().first_digit;
        second_digit_id = timer.as_ref().unwrap().second_digit;
    }


    let first_sprite = world.get_component_from_entity_mut::<Sprite>(first_digit_id).unwrap();
    first_sprite.as_mut().unwrap().index = (first_digit + 25) as u32;
    let second_sprite = world.get_component_from_entity_mut::<Sprite>(second_digit_id).unwrap();
    second_sprite.as_mut().unwrap().index = (second_digit + 25) as u32;
}

