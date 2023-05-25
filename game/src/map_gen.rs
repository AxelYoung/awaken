use itertools::iproduct;

use crate::Game;
use crate::buttons::SlaveButton;

use super::common::Position;
use super::math::{Vec2, Vec2i};
use super::buttons::MasterButton;
use super::render::{Sprite, SPRITE_SIZE, SPRITE_CENTER};
use super::physics::{Collider, Velocity};
use super::pushables::Pushable;

pub const ROOM_TILE_WIDTH : u32 = 16;
pub const ROOM_TILE_HEIGHT : u32 = 14;

pub const ROOM_PIXEL_WIDTH : u32 = ROOM_TILE_WIDTH * SPRITE_SIZE;
pub const ROOM_PIXEL_HEIGHT : u32 = ROOM_TILE_HEIGHT * SPRITE_SIZE;

pub const MAP_WIDTH: u8 = 3;
pub const MAP_HEIGHT: u8 = 3;

const MAP: &[&[&[&[Tile]]]] = &[
    &[EMPTY_ROOM, HALL_UP, EMPTY_ROOM],
    &[EMPTY_ROOM, START_HALL, HALL_RIGHT],
    &[EMPTY_ROOM, START_ROOM, EMPTY_ROOM]
];

#[derive(PartialEq)]
enum Tile<'a> {
    SW,
    SF,
    BT(&'a [Vec2i], &'a [Vec2i]),
    PB,
    PL(usize, u32),
    EM
}

use Tile::*;

const SB : Tile = BT(&[Vec2i {x: 4, y: 4}, Vec2i {x: 10, y: 4}, Vec2i {x: 13, y: 4}], &[Vec2i {x: 7, y: 0}, Vec2i {x: 8, y: 0}]);

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
    &[SW,SB,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,PN,SF,SF,PG,SF,SF,PY,SF,SF,PR,SF,SF,PP,SF,SW],
    &[SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW]
];

const START_HALL: &[&[Tile]] = &[
    &[SW,SW,SW,SW,SW,SW,SW,SF,SF,SW,SW,SW,SW,SW,SW,SW],
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
    &[SW,SF,SF,SF,SF,SF,PB,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SW,SW,SW,SW,SW,SW,SF,SF,SW,SW,SW,SW,SW,SW,SW]
];

const BB : Tile = BT(&[], &[Vec2i {x: 8, y: 1}, Vec2i {x: 8, y: 2}, Vec2i {x: 8, y: 3}, Vec2i {x: 8, y: 4}, Vec2i {x: 8, y: 5}, Vec2i {x: 8, y: 6}, Vec2i {x: 8, y: 7}, Vec2i {x: 8, y: 8}, Vec2i {x: 8, y: 9}, Vec2i {x: 8, y: 10}, Vec2i {x: 8, y: 11}, Vec2i {x: 8, y: 12}]);

const HALL_RIGHT: &[&[Tile]] = &[
    &[SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
    &[SF,SF,SF,SF,BB,SF,SF,SF,SF,SF,SF,SF,PB,SF,SF,SW],
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
            BT(buttons, gates) => { 

                let mut gate_positions : Vec<Position> = vec![];

                for gate in *gates {
                    gate_positions.push(get_tile_position(room_x, room_y, gate.x as usize, gate.y as usize));
                }

                let mut button_positions : Vec<Position> = vec![];

                for button in *buttons {
                    button_positions.push(get_tile_position(room_x, room_y, button.x as usize, button.y as usize));
                }

                create_button(game, position, button_positions, gate_positions)
             },
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

fn create_button(game: &mut Game, position: Position, button_positions: Vec<Position>, gate_positions: Vec<Position>) {
    create_stone_floor(game, position);

    let mut gates : Vec<usize> = vec![];

    for gate in gate_positions {
        let id = game.world.new_entity();

        game.world.add_component_to_entity(id, gate);
        game.world.add_component_to_entity(id, Sprite::new(37, 25));
        game.world.add_component_to_entity(id, Collider {});

        gates.push(id);
    }

    let mut slaves : Vec<usize> = vec![];

    for button in button_positions {
        let id = game.world.new_entity();

        game.world.add_component_to_entity(id, button);
        game.world.add_component_to_entity(id, Sprite::new(22, 10));
        game.world.add_component_to_entity(id, SlaveButton { collided: None });

        slaves.push(id);
    }

    let button = game.world.new_entity();

    game.world.add_component_to_entity(button, Position::new(position.value.x, position.value.y));
    game.world.add_component_to_entity(button, Sprite::new(22, 10));
    game.world.add_component_to_entity(button, MasterButton { gates, slaves});
    game.world.add_component_to_entity(button, SlaveButton { collided: None });
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