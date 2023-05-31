use itertools::iproduct;

use crate::Game;

use super::buttons::{Button, ButtonType};
use super::math::{Vec2, Vec2i};
use super::common::Cell;
use super::render::{Sprite, SPRITE_SIZE, SPRITE_CENTER};

pub const ROOM_TILE_WIDTH : u32 = 16;
pub const ROOM_TILE_HEIGHT : u32 = 14;

pub const ROOM_PIXEL_WIDTH : u32 = ROOM_TILE_WIDTH * SPRITE_SIZE;
pub const ROOM_PIXEL_HEIGHT : u32 = ROOM_TILE_HEIGHT * SPRITE_SIZE;

pub const MAP_ROOM_WIDTH: u8 = 3;
pub const MAP_ROOM_HEIGHT: u8 = 3;

pub const MAP_TILE_WIDTH: usize = 
   MAP_ROOM_WIDTH as usize * ROOM_TILE_WIDTH as usize;
pub const MAP_TILE_HEIGHT: usize = 
   MAP_ROOM_HEIGHT as usize * ROOM_TILE_HEIGHT as usize;

const MAP: &[&[&[&[Tile]]]] = &[
   &[EMPTY_ROOM, START_ROOM, EMPTY_ROOM],
   &[EMPTY_ROOM, START_HALL, HALL_RIGHT],
   &[EMPTY_ROOM, HALL_UP, EMPTY_ROOM]
];

#[derive(PartialEq)]
enum Tile<'a> {
   SW,
   SF,
   PB,
   PL(usize, u32),
   BT(ButtonType, &'a [Cell]),
   EM
}

use Tile::*;

const PN : Tile = PL(0, 41);
const PG : Tile = PL(1, 42);
const PY : Tile = PL(2, 43);
const PR : Tile = PL(3, 44);
const PP : Tile = PL(4, 45);

const BB : Tile = BT(ButtonType::Color(0), &[
   Cell { value: Vec2i {x: 7, y: 0} },
   Cell { value: Vec2i {x: 8, y: 0} }
   ]
);

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
   &[SW,BB,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
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

const HALL_RIGHT: &[&[Tile]] = &[
   &[SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW],
   &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
   &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
   &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
   &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
   &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
   &[SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
   &[SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,PB,SF,SF,SW],
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
      0..MAP_ROOM_WIDTH as usize, 
      0..MAP_ROOM_HEIGHT as usize, 
      0..ROOM_TILE_WIDTH as usize, 
      0..ROOM_TILE_HEIGHT as usize);

   for (room_x, room_y, x, y) in map_loop {
      let cell = get_tile_cell(room_x, room_y, x, y);

      match &MAP[room_y][room_x][y][x] {
         SW => { create_stone_wall(game, cell) },
         SF => { create_stone_floor(game, cell) },
         PB => { create_push_block(game, cell) },
         BT(button_type, cells) => {
            let mut relative_cells = vec![]; 
            for cell in *cells {
               relative_cells.push(
                  get_tile_cell(
                     room_x, room_y, 
                     cell.x as usize, cell.y as usize
                  )
               );
            }
            create_button(game, cell, button_type.clone(), relative_cells) 
         },
         EM => { continue; },
         PL(clone, _) => { create_player_spawn(game, cell, *clone) }
      }
   }
}

fn create_button(
   game: &mut Game, cell: Cell, button_type: ButtonType, cells: Vec<Cell>
) {
   create_stone_floor(game, cell);

   let button = game.world.new_entity();

   game.world.add_component_to_entity(button, cell);
   game.world.add_component_to_entity(button, cell.to_position());
   game.world.add_component_to_entity(button, button_sprite(button_type));

   let mut cells_entities = vec![];

   for cell in cells {
      let cell_entity = game.world.new_entity();

      game.world.add_component_to_entity(cell_entity, Sprite::new(2,5,1));
      game.world.add_component_to_entity(cell_entity, cell);
      game.world.add_component_to_entity(cell_entity, cell.to_position());
      game.colliders[cell.x as usize][cell.y as usize] = true;

      cells_entities.push(cell_entity);
   }

   game.world.add_component_to_entity(button, 
      Button::new(button_type, cells_entities)
   );
}

fn button_sprite(button_type: ButtonType) -> Sprite {
   match button_type {
      ButtonType::Any => { Sprite::new(6, 7, 1) },
      ButtonType::AnyColor => { Sprite::new(6, 7, 1) },
      ButtonType::Color(col) => { Sprite::new(col as u32, 7, 1) }
   }
}

fn create_player_spawn(game: &mut Game, cell: Cell, clone: usize) {
   create_stone_floor(game, cell);

   let player_spawn = game.world.new_entity();

   game.clone_spawns[clone] = cell;

   game.world.add_component_to_entity(player_spawn, cell.to_position());

   game.world.add_component_to_entity(
      player_spawn, Sprite::new(clone as u32, 6, 1)
   );
}

fn create_stone_floor(game: &mut Game, cell: Cell) {
   let stone_floor = game.world.new_entity();

   game.world.add_component_to_entity(stone_floor, cell.to_position());
   game.world.add_component_to_entity(stone_floor, Sprite::new(1, 5, 0));
}

fn create_stone_wall(game: &mut Game, cell: Cell) {
   let stone_wall = game.world.new_entity();

   game.world.add_component_to_entity(stone_wall, cell.to_position());
   game.world.add_component_to_entity(stone_wall, Sprite::new(0, 5, 0));
   game.colliders[cell.x as usize][cell.y as usize] = true;
}

fn create_push_block(game: &mut Game, cell: Cell) {
   create_stone_floor(game, cell);

   let push_box = game.world.new_entity();

   game.world.add_component_to_entity(push_box, cell.to_position());
   game.world.add_component_to_entity(push_box, Sprite::new(40, 0, 0));
}

fn get_tile_cell(room_x: usize, room_y: usize, x: usize, y: usize) -> Cell {
   let rev_y = (ROOM_TILE_HEIGHT - 1) - y as u32;

   let cell = Cell::new(            
      (room_x as i32 * ROOM_TILE_WIDTH as i32) + x as i32,
      (room_y as i32 * ROOM_TILE_HEIGHT as i32) + rev_y as i32
   );

   cell
}