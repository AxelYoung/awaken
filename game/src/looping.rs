use harmony::*;

use crate::map_gen::{ROOM_PIXEL_WIDTH, ROOM_PIXEL_HEIGHT};
use crate::math::Vec2;
use crate::pushables::Pushable;
use crate::render::SPRITE_CENTER;

use super::Game;
use super::common::Position;
use super::physics::{Collider, Velocity};
use super::animation::{Animator, Animation, AnimationFrame};
use super::render::{Sprite, SPRITE_SIZE};
use super::buttons::SlaveButton;

struct Timer {
    first_digit: usize,
    second_digit: usize,
    current_time: f32
}

struct Clone { 
    id: usize,
    current_command: usize
}

pub fn create_ui(game: &mut Game) {
    let first_digit = game.world.new_entity();

    game.world.add_component_to_entity(first_digit, 
        Position::new(((game.chroma.camera.x / 4.0) * ROOM_PIXEL_WIDTH as f32) + SPRITE_CENTER, 
        (game.chroma.camera.y / 4.0) + -14.0 * SPRITE_SIZE as f32 + SPRITE_CENTER));
    game.world.add_component_to_entity(first_digit, Sprite::new(25, 100));

    let second_digit = game.world.new_entity();

    game.world.add_component_to_entity(second_digit, Position::new(((game.chroma.camera.x / 4.0) * ROOM_PIXEL_WIDTH as f32) + SPRITE_CENTER, 
    (game.chroma.camera.y / 4.0) + -14.0 * SPRITE_SIZE as f32 + SPRITE_CENTER));
    game.world.add_component_to_entity(second_digit, Sprite::new(25, 100));

    let timer = game.world.new_entity();

    game.world.add_component_to_entity(timer, Timer {first_digit, second_digit, current_time: 30.0});

    game.timer = timer;
}

pub fn update(game: &mut Game) {
    loop_early(game);
    set_timer(game);
    replay_clones(game);
}

fn replay_clones(game: &mut Game) {
    if game.clone_count > 0 {
        iterate_entities!(game.world, (Clone, Velocity, Animator),
        |clone: &mut Clone, velocity: &mut Velocity, animator: &mut Animator| {
    
            if clone.current_command == game.clone_commands[clone.id].len() { return }

            if game.time > game.clone_commands[clone.id][clone.current_command].1 {
                clone.current_command += 1;
            };
    
            let dir = game.clone_commands[clone.id][clone.current_command- 1].0;
    
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
        });
    }
}

fn loop_early(game: &mut Game) {
    if game.input.loop_pressed {
        restart_loop(game);
    }
}

fn restart_loop(game: &mut Game) {
    iterate_entities!(game.world, [Pushable], (Position), 
    |pushable: &Pushable, position: &mut Position| {
        position.value = pushable.origin;
    });

    iterate_entities!(game.world, (SlaveButton, Sprite), 
    |button: &mut SlaveButton, sprite: &mut Sprite| {
        button.collided = None;
        let sprite_index = match button.r#type {
            super::buttons::ButtonType::Any => {22},
            super::buttons::ButtonType::AnyColor => {20},
            super::buttons::ButtonType::Color(4) => {18},
            super::buttons::ButtonType::Color(3) => {16},
            super::buttons::ButtonType::Color(2) => {14},
            super::buttons::ButtonType::Color(1) => {12},
            super::buttons::ButtonType::Color(0) => {10},
            _ => {0}
        };
        sprite.index = sprite_index;
    });

    if game.clone_count > 0 {
        iterate_entities!(game.world, (Clone, Position), 
        |clone: &mut Clone, position: &mut Position| {
            position.value = game.clone_spawns[clone.id];
            clone.current_command = 1;
        });
    }

    let clone = game.world.new_entity();
    game.clones[game.current_clone] = clone;
    game.world.add_component_to_entity(clone, Sprite::new(2 * game.current_clone as u32, 50));
    game.world.add_component_to_entity(clone, Position {value: game.clone_spawns[game.current_clone]});
    game.world.add_component_to_entity(clone, Velocity::new(0.0, 0.0));
    game.world.add_component_to_entity(clone, Collider{});
    game.world.add_component_to_entity(clone, Clone{id: game.current_clone, current_command: 1});
    game.world.add_component_to_entity(clone, Animator{
        animation: Animation {
            frames: vec![AnimationFrame::new(1 + (2 * game.current_clone as u32), 75), AnimationFrame::new(2 * game.current_clone as u32, 75)],
            r#loop: true
        },
        frame_index: 0,
        time: 0,
        playing: false
    });
    game.world.add_component_to_entity(clone, Pushable{origin: Vec2::zero()});

    game.current_clone += 1;

    if game.clone_count != 4 {
        game.clone_count += 1;
    } else {
        if game.current_clone == 5 {
            game.current_clone = 0;
        }
        game.world.delete_entity(game.clones[game.current_clone]);
        game.clone_commands[game.current_clone] = vec![];
    }

    game.world.get_component_from_entity_mut::<Position>(game.player)
        .unwrap().as_mut().unwrap().value = game.clone_spawns[game.current_clone];

    game.world.get_component_from_entity_mut::<Sprite>(game.player)
        .unwrap().as_mut().unwrap().index = 2 * game.current_clone as u32;

    game.world.get_component_from_entity_mut::<Animator>(game.player)
        .unwrap().as_mut().unwrap().animation = Animation {
            frames: vec![AnimationFrame::new(1 + (2 * game.current_clone as u32), 75), AnimationFrame::new(2 * game.current_clone as u32, 75)],
            r#loop: true
        };

    let timer = game.world.get_component_from_entity_mut::<Timer>(game.timer).unwrap();
    
    timer.as_mut().unwrap().current_time = 30.0;

    game.chroma.update_camera(-4.0, 8.0);

    game.time = 0;
}

fn set_timer(game: &mut Game) {
    let first_digit;
    let second_digit;
    let first_digit_id;
    let second_digit_id;

    {
        let timer = game.world.get_component_from_entity_mut::<Timer>(game.timer).unwrap();
        
        timer.as_mut().unwrap().current_time -= game.delta_time as f32 / 1000.0;

        if timer.as_ref().unwrap().current_time <= 0.0 {
            restart_loop(game);
            return
        }

        first_digit = (timer.as_ref().unwrap().current_time / 10.0) as u8;
        second_digit = (timer.as_ref().unwrap().current_time - first_digit as f32 * 10.0) as u8;

        first_digit_id = timer.as_ref().unwrap().first_digit;
        second_digit_id = timer.as_ref().unwrap().second_digit;
    }


    let first_sprite = game.world.get_component_from_entity_mut::<Sprite>(first_digit_id).unwrap();
    first_sprite.as_mut().unwrap().index = (first_digit + 25) as u32;
    let second_sprite = game.world.get_component_from_entity_mut::<Sprite>(second_digit_id).unwrap();
    second_sprite.as_mut().unwrap().index = (second_digit + 25) as u32;

    let first_position = game.world.get_component_from_entity_mut::<Position>(first_digit_id).unwrap().as_mut().unwrap();
    first_position.value = Vec2::new(
    ((-game.chroma.camera.x / 4.0) * ROOM_PIXEL_WIDTH as f32) + SPRITE_CENTER, 
    ((-game.chroma.camera.y / 4.0) * ROOM_PIXEL_HEIGHT as f32) as f32 + SPRITE_CENTER);
    let second_position = game.world.get_component_from_entity_mut::<Position>(second_digit_id).unwrap().as_mut().unwrap();
    second_position.value = Vec2::new(
        ((-game.chroma.camera.x / 4.0) * ROOM_PIXEL_WIDTH as f32) + SPRITE_SIZE as f32 + SPRITE_CENTER, 
    ((-game.chroma.camera.y / 4.0) * ROOM_PIXEL_HEIGHT as f32) as f32 + SPRITE_CENTER);
}