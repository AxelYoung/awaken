use itertools::iproduct;

use crate::Game;

use super::common::Position;
use super::math::{Vec2, Vec2i};
use super::buttons::Button;
use super::render::{Sprite, SPRITE_SIZE, SPRITE_CENTER};
use super::physics::{Collider, Velocity};
use super::transitions::Transition;
use super::pushables::Pushable;

pub const ROOM_TILE_WIDTH : u32 = 16;
pub const ROOM_TILE_HEIGHT : u32 = 14;

pub const ROOM_PIXEL_WIDTH : u32 = ROOM_TILE_WIDTH * SPRITE_SIZE;
pub const ROOM_PIXEL_HEIGHT : u32 = ROOM_TILE_HEIGHT * SPRITE_SIZE;

pub const MAP_WIDTH: u8 = 3;
pub const MAP_HEIGHT: u8 = 3;

const MAP: &[&[&[&[Tile]]]] = &[
    &[EMPTY_ROOM, HALL_UP, EMPTY_ROOM],
    &[HALL_LEFT, START_HALL, HALL_RIGHT],
    &[EMPTY_ROOM, START_ROOM, EMPTY_ROOM]
];

#[derive(PartialEq)]
enum Tile {
    SW,
    SF,
    BT([Vec2i; 2]),
    PB,
    TR(Vec2i),
    PL(usize, u32),
    EM
}

use Tile::*;

const TN : Tile = TR(Vec2i {x: 0, y: 1});
const TW : Tile = TR(Vec2i {x: -1, y: 0});
const TE : Tile = TR(Vec2i {x: 1, y: 0});
const TS : Tile = TR(Vec2i {x: 0, y: -1});

const SB : Tile = BT([Vec2i {x: 7, y: 0}, Vec2i {x: 8, y: 0}]);

const PN : Tile = PL(0, 41);
const PG : Tile = PL(1, 42);
const PY : Tile = PL(2, 43);
const PR : Tile = PL(3, 44);
const PP : Tile = PL(4, 45);

const EMPTY_ROOM: &[&[Tile]] = &[
    &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM],
    &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM],
    &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM],
    &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM],
    &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM],
    &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM],
    &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM],
    &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM],
    &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM],
    &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM],
    &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM],
    &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM],
    &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM],
    &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM]
];

const START_ROOM: &[&[Tile]] = &[
    &[SW,SW,SW,SW,SW,SW,SW,SF,SF,SW,SW,SW,SW,SW,SW,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SB,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,PB,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,PN,SF,SF,PG,SF,SF,PY,SF,SF,PR,SF,SF,PP,SF,SW],
    &[SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW]
];

const START_HALL: &[&[Tile]] = &[
    &[SW,SW,SW,SW,SW,SW,SW,TN,SF,SW,SW,SW,SW,SW,SW,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[TW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,TE],
    &[SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SW,SW,SW,SW,SW,SW,TS,SF,SW,SW,SW,SW,SW,SW,SW]
];

const HALL_LEFT: &[&[Tile]] = &[
    &[SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW]
];

const HALL_RIGHT: &[&[Tile]] = &[
    &[SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW]
];

const HALL_UP: &[&[Tile]] = &[
    &[SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SW,SW,SW,SW,SW,SW,SF,SF,SW,SW,SW,SW,SW,SW,SW]
];

pub fn create(game: &mut Game) {
    let map_loop = iproduct!(
        0..MAP_WIDTH as usize, 
        0..MAP_HEIGHT as usize, 
        0..ROOM_TILE_WIDTH as usize, 
        0..ROOM_TILE_HEIGHT as usize);

    for (room_x, room_y, x, y) in map_loop {
        let position = get_tile_position(room_x, room_y, x, y);

        match &MAP[room_y][room_x][y][x] {
            SW => { create_stone_wall(game, position) },
            SF => { create_stone_floor(game, position) },
            PB => { create_push_block(game, position) },
            EM => { continue; }
            BT(gates) => { 
                let gate_1_position = 
                    get_tile_position(room_x, room_y, gates[0].x as usize, gates[0].y as usize);
                let gate_2_position = 
                    get_tile_position(room_x, room_y, gates[1].x as usize, gates[1].y as usize);
                    
                create_button(game, position, gate_1_position, gate_2_position)
             },
            TR(dir) => { create_transition(game, position, *dir) },
            PL(clone, sprite) => { create_player_spawn(game, position, *clone, *sprite) }
        }
    }
}

fn create_player_spawn(game: &mut Game, position: Position, clone: usize, sprite: u32) {
    let player_spawn = game.world.new_entity();

    game.clone_spawns[clone] = position.value;

    game.world.add_component_to_entity(player_spawn, position);
    game.world.add_component_to_entity(player_spawn, Sprite::new(sprite, 0));
}

fn create_stone_floor(game: &mut Game, position: Position) {
    let stone_floor = game.world.new_entity();

    game.world.add_component_to_entity(stone_floor, position);
    game.world.add_component_to_entity(stone_floor, Sprite::new(35, 0));
}

fn create_stone_wall(game: &mut Game, position: Position) {
    let stone_wall = game.world.new_entity();

    game.world.add_component_to_entity(stone_wall, position);
    game.world.add_component_to_entity(stone_wall, Sprite::new(36, 25));
    game.world.add_component_to_entity(stone_wall, Collider{});
}

fn create_push_block(game: &mut Game, position: Position) {
    create_stone_floor(game, position);

    let push_box = game.world.new_entity();

    game.world.add_component_to_entity(push_box, position);
    game.world.add_component_to_entity(push_box, Sprite::new(40, 25));
    game.world.add_component_to_entity(push_box, Velocity::new(0.0, 0.0));
    game.world.add_component_to_entity(push_box, Collider{});
    game.world.add_component_to_entity(push_box, Pushable{ origin: position.value });
}

fn create_button(game: &mut Game, position: Position, gate_1_position: Position, gate_2_position: Position) {
    create_stone_floor(game, position);

    let gate_1 = game.world.new_entity();

    game.world.add_component_to_entity(gate_1, gate_1_position);
    game.world.add_component_to_entity(gate_1, Sprite::new(37, 25));
    game.world.add_component_to_entity(gate_1, Collider {});

    let gate_2 = game.world.new_entity();

    game.world.add_component_to_entity(gate_2, gate_2_position);
    game.world.add_component_to_entity(gate_2, Sprite::new(37, 25));
    game.world.add_component_to_entity(gate_2, Collider {});

    let button = game.world.new_entity();

    game.world.add_component_to_entity(button, position);
    game.world.add_component_to_entity(button, Sprite::new(38, 10));
    game.world.add_component_to_entity(button, Button { gate_ids: vec![gate_1, gate_2], collided: None});
}

fn create_transition(game: &mut Game, position: Position, dir: Vec2i) {
    create_stone_floor(game, position);

    let transition = game.world.new_entity();

    let offset_x = if dir.x == 0 { (SPRITE_SIZE as i16) / 2 }
                    else if dir.x == 1 { SPRITE_SIZE as i16 } 
                    else { -(SPRITE_SIZE as i16) };

    let offset_y = if dir.y == 0 { -(SPRITE_SIZE as i16) / 2 } 
                    else if dir.y == -1 { -(SPRITE_SIZE as i16) } 
                    else { SPRITE_SIZE as i16 };

    let position = Position::new(position.x + offset_x as f32, position.y + offset_y as f32);

    game.world.add_component_to_entity(transition, position);
    game.world.add_component_to_entity(transition, Transition {dir, collided: false});
}

fn get_tile_position(room_x: usize, room_y: usize, x: usize, y: usize) -> Position {
    let room_position = Vec2::new(
        (room_x as u32 * ROOM_PIXEL_WIDTH) as f32,
        (room_y as u32 * ROOM_PIXEL_HEIGHT) as f32
    );

    let rev_y = (ROOM_TILE_HEIGHT - 1) - y as u32;

    let tile_position = Vec2::new(
        (x as f32 * SPRITE_SIZE as f32) + SPRITE_CENTER,
        (rev_y as f32 * SPRITE_SIZE as f32) + SPRITE_CENTER
    );

    let position = Position::new(                
        tile_position.x + room_position.x,
        tile_position.y - room_position.y
    );

    position
}