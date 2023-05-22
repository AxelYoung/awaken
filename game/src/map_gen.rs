use harmony::*;
use super::common::Position;
use super::math::Vec2;
use super::buttons::Button;
use super::render::{Sprite, SPRITE_SIZE};
use super::physics::{Collider, Velocity};
use super::transitions::Transition;
use super::pushables::Pushable;

pub const ROOM_WIDTH : u8 = 16;
pub const ROOM_HEIGHT : u8 = 14;

pub const MAP_WIDTH: u8 = 1;
pub const MAP_HEIGHT: u8 = 2;

const MAP: [[[[u8;ROOM_WIDTH as usize];ROOM_HEIGHT as usize]; MAP_WIDTH as usize]; MAP_HEIGHT as usize] = [
    [START_HALL],
    [START_ROOM]
];

const START_ROOM: [[u8;ROOM_WIDTH as usize];ROOM_HEIGHT as usize] = [
    [0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
];

const START_HALL: [[u8;ROOM_WIDTH as usize];ROOM_HEIGHT as usize] = [
    [0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
    [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0]
];

pub fn create(world: &mut World) {
    for room_x in 0..MAP_WIDTH as usize {
        for room_y in 0..MAP_HEIGHT as usize {
            for x in 0..ROOM_WIDTH as usize {
                for y in 0..ROOM_HEIGHT as usize{
                    let sprite = Sprite::new(match MAP[room_y][room_x][y][x] {
                        0 => 36,
                        _ => 35
                    });
        
                    let position = Position::new(
                        (x as f32 * SPRITE_SIZE as f32) + (room_x as u8 * ROOM_WIDTH * SPRITE_SIZE as u8) as f32, 
                        (((ROOM_HEIGHT - 1) as f32 - y as f32) * SPRITE_SIZE as f32) - (room_y as u8 * ROOM_HEIGHT * SPRITE_SIZE as u8) as f32);
                    let e = world.new_entity();
                    world.add_component_to_entity(e, sprite);
                    world.add_component_to_entity(e, position);
                    if MAP[room_y][room_x][y][x] == 0 {
                        world.add_component_to_entity(e, Collider{});
                    }
                }
            }
        } 
    } 

    let e = world.new_entity();

    world.add_component_to_entity(e, Position::new(8.0 * SPRITE_SIZE as f32, 0.0));
    world.add_component_to_entity(e, Transition {dir: Vec2::new(0.0, 1.0), collided: false});

    let gate_1 = world.new_entity();

    world.add_component_to_entity(gate_1, Position::new(8.0 * SPRITE_SIZE as f32, -1.0 * SPRITE_SIZE as f32));
    world.add_component_to_entity(gate_1, Sprite::new(37));
    world.add_component_to_entity(gate_1, Collider {});

    let gate_2 = world.new_entity();

    world.add_component_to_entity(gate_2, Position::new(7.0 * SPRITE_SIZE as f32, -1.0 * SPRITE_SIZE as f32));
    world.add_component_to_entity(gate_2, Sprite::new(37));
    world.add_component_to_entity(gate_2, Collider {});

    let button = world.new_entity();

    world.add_component_to_entity(button, Position::new(4.0 * SPRITE_SIZE as f32, -5.0 * SPRITE_SIZE as f32));
    world.add_component_to_entity(button, Sprite::new(38));
    world.add_component_to_entity(button, Button { gate_ids: vec![gate_1, gate_2], collided: None});

    let push_box = world.new_entity();

    world.add_component_to_entity(push_box, Position::new(12.0 * SPRITE_SIZE as f32, -9.0 * SPRITE_SIZE as f32));
    world.add_component_to_entity(push_box, Sprite::new(40));
    world.add_component_to_entity(push_box, Velocity::new(0.0, 0.0));
    world.add_component_to_entity(push_box, Collider{});
    world.add_component_to_entity(push_box, Pushable{});
}