use itertools::iproduct;

use crate::Game;
use crate::animation::{Animator, Animation, AnimationFrame};
use crate::boxes::PushBox;
use crate::buttons::SlaveButton;
use crate::collision::Collider;
use crate::movement::Moveable;

use super::buttons::{Button, ButtonType};
use super::math::{Vec2, Vec2i};
use super::common::Cell;
use super::render::{Sprite, SPRITE_SIZE, SPRITE_CENTER};

use super::boxes::BoxType;

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
   PL(usize, u32),
   BT(ButtonType, &'a [Cell], &'a[Slave], &'a[&'a[WireTile]]),
   BX(BoxType),
   EM,
   WT(u8)
}

#[derive(PartialEq)]
struct Slave {
   pub button_type: ButtonType,
   pub cell: Cell
}

use Tile::*;

const PN : Tile = PL(0, 41);
const PG : Tile = PL(1, 42);
const PY : Tile = PL(2, 43);
const PR : Tile = PL(3, 44);
const PP : Tile = PL(4, 45);

const B0: Tile = BX(BoxType::Color(0));
const B1: Tile = BX(BoxType::Color(1));
const B2: Tile = BX(BoxType::Color(2));
const B3: Tile = BX(BoxType::Color(3));
const B4: Tile = BX(BoxType::Color(4));
const BA: Tile = BX(BoxType::Any);

const W0: Tile = WT(0);
const W1: Tile = WT(1);
const W2: Tile = WT(2);
const W3: Tile = WT(3);
const W4: Tile = WT(4);

const BB : Tile = BT(ButtonType::Color(0), 
   &[
      Cell { value: Vec2i {x: 7, y: 0} },
      Cell { value: Vec2i {x: 8, y: 0} }
   ],
   &[
      Slave { 
         button_type: ButtonType::Color(1), 
         cell: Cell { 
            value: { 
               Vec2i { x: 4, y: 4}
            }
         }
      },
      Slave { 
         button_type: ButtonType::AnyColor, 
         cell: Cell { 
            value: { 
               Vec2i { x: 10, y: 4}
            }
         }
      },
      Slave { 
         button_type: ButtonType::Color(4), 
         cell: Cell { 
            value: { 
               Vec2i { x: 13, y: 4}
            }
         }
      }
   ],
   START_WIRE_ROOM
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

const START_WIRE_ROOM: &[&[WireTile]] = &[
   &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
   &[ET,ET,ET,ET,ET,ET,VR,ET,ET,VR,ET,ET,ET,ET,ET,ET],
   &[ET,ET,ET,ET,ET,ET,VR,ET,ET,VR,ET,ET,ET,ET,ET,ET],
   &[ET,ET,ET,ET,ET,ET,VR,ET,ET,VR,ET,ET,ET,ET,ET,ET],
   &[ET,SR,HR,HR,HR,HR,TU,HR,HR,TU,HR,HR,HR,SL,ET,ET],
   &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
   &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
   &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
   &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
   &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
   &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
   &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
   &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
   &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET]
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
   &[SW,SF,SF,SF,SF,BA,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
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
   &[SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,BA,SF,SF,SW],
   &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
   &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
   &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
   &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
   &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
   &[SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW]
];


const HALL_UP: &[&[Tile]] = &[
   &[SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW],
   &[SW,W0,SF,SF,W1,SF,SF,W2,SF,SF,W3,SF,SF,W4,SF,SW],
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

#[derive(PartialEq)]
enum WireTile {
   ET,
   VR,
   HR,
   SU,
   SD,
   SL,
   SR,
   XC,
   TU,
   TR,
   TL,
   TD,
   L1,
   L2,
   L3,
   L4
}

use WireTile::*;


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
         BT(button_type, cells, slave_buttons, wires) => {
            let mut relative_cells = vec![]; 
            for cell in *cells {
               relative_cells.push(
                  get_tile_cell(
                     room_x, room_y, 
                     cell.x as usize, cell.y as usize
                  )
               );
            }
            let mut slaves = vec![];
            for slave in slave_buttons.clone() {
               slaves.push(
                  create_slave_button(
                     game, 
                     get_tile_cell(
                        room_x, room_y, 
                        slave.cell.x as usize, slave.cell.y as usize
                     ), 
                     slave.button_type
                  )
               );
            }
            let wires = draw_wires(game, wires, room_x, room_y);
            create_button(
               game, 
               cell, 
               button_type.clone(), 
               relative_cells,
               slaves,
               wires
            ) 
         },
         PL(clone, _) => { create_player_spawn(game, cell, *clone) },
         BX(box_type) => { create_box(game, cell, box_type) }
         WT(color) => { create_win_tile(game, cell, *color) }
         _ => { continue; }
      }
   }
}

fn create_win_tile(game: &mut Game, cell: Cell, color: u8) {
   create_stone_floor(game, cell);

   let win_tile = game.world.new_entity();

   game.world.add_component_to_entity(win_tile, cell);
   game.world.add_component_to_entity(win_tile, cell.to_position());
   game.world.add_component_to_entity(win_tile, Sprite::new(0, 18 + color as u32, 1));
   game.world.add_component_to_entity(win_tile, Animator {
      animation: Animation {
         frames: vec![
            AnimationFrame::new(0, 18 + color as u32, 100),
            AnimationFrame::new(1, 18 + color as u32, 100),
            AnimationFrame::new(2, 18 + color as u32, 100)
         ],
         r#loop: true
      },
      frame_index: 0,
      time: 0,
      playing: true
   })
}

fn draw_wires(
   game: &mut Game, wires: &[&[WireTile]], room_x: usize, room_y: usize
) -> Vec<usize> {
   let mut wire_entities : Vec<usize> = vec![];

   for x in 0..ROOM_TILE_WIDTH as usize {
      for y in 0..ROOM_TILE_HEIGHT as usize {
         let sprite_x = match &wires[y][x] {
            VR => {1}
            HR => {0}
            SU => {4}
            SD => {5}
            SL => {2}
            SR => {3}
            XC => {6}
            TU => {7}
            TR => {10}
            TL => {9}
            TD => {8}
            L1 => {14}
            L2 => {12}
            L3 => {13}
            L4 => {11}
            _ => { continue; }
         };
         let wire = game.world.new_entity();
         game.world.add_component_to_entity(wire, 
            Sprite::new(sprite_x as u32, 12, 1)
         );

         let cell = get_tile_cell(room_x, room_y, x, y);

         game.world.add_component_to_entity(wire, cell);
         game.world.add_component_to_entity(wire, cell.to_position());

         wire_entities.push(wire);
      }
   }

   wire_entities
}

fn create_box (game: &mut Game, cell: Cell, box_type: &BoxType) {
   create_stone_floor(game, cell);

   let box_entity = game.world.new_entity();

   game.world.add_component_to_entity(box_entity, cell);
   game.world.add_component_to_entity(box_entity, cell.to_position());
   game.world.add_component_to_entity(box_entity, box_sprite(box_type));
   game.world.add_component_to_entity(box_entity, Moveable {
      start_cell: cell,
      end_cell: cell,
      duration: 150,
      accumulator: 0,
      moving: false,
      box_moveable: true
   });
   game.world.add_component_to_entity(box_entity, PushBox { start_cell: cell} );

   game.colliders[cell.x as usize][cell.y as usize] = 
      Collider::Box(box_type.clone(), box_entity);
}

fn box_sprite(box_type: &BoxType) -> Sprite {
   match box_type {
      BoxType::Any => { Sprite::new(5, 11, 50) },
      BoxType::Color(col) => { Sprite::new(*col as u32, 11, 50) }
   }
}

fn create_button(
   game: &mut Game, cell: Cell, button_type: ButtonType, cells: Vec<Cell>,
   slaves: Vec<usize>, wires: Vec<usize>
) {
   create_stone_floor(game, cell);

   let button = game.world.new_entity();

   game.world.add_component_to_entity(button, cell);
   game.world.add_component_to_entity(button, cell.to_position());
   game.world.add_component_to_entity(button, button_sprite(button_type));

   let mut cells_entities = vec![];

   for cell in cells {
      let cell_entity = game.world.new_entity();

      game.world.add_component_to_entity(cell_entity, Sprite::new(2,5,2));
      game.world.add_component_to_entity(cell_entity, cell);
      game.world.add_component_to_entity(cell_entity, cell.to_position());
      game.colliders[cell.x as usize][cell.y as usize] = Collider::Solid;

      cells_entities.push(cell_entity);
   }

   game.world.add_component_to_entity(button, 
      Button::new(button_type, cells_entities, slaves, wires)
   );
}

fn create_slave_button(
   game: &mut Game, cell: Cell, button_type: ButtonType
) -> usize {
   create_stone_floor(game, cell);

   let button = game.world.new_entity();

   game.world.add_component_to_entity(button, cell);
   game.world.add_component_to_entity(button, cell.to_position());
   game.world.add_component_to_entity(button, button_sprite(button_type));

   game.world.add_component_to_entity(button, 
      SlaveButton { button_type, pressed: false }
   );

   button
}

fn button_sprite(button_type: ButtonType) -> Sprite {
   match button_type {
      ButtonType::AnyColor => { Sprite::new(5, 7, 1) },
      ButtonType::Color(col) => { Sprite::new(col as u32, 7, 2) }
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
   game.colliders[cell.x as usize][cell.y as usize] = Collider::Solid;
}


fn get_tile_cell(room_x: usize, room_y: usize, x: usize, y: usize) -> Cell {
   let rev_y = (ROOM_TILE_HEIGHT - 1) - y as u32;

   let cell = Cell::new(            
      (room_x as i32 * ROOM_TILE_WIDTH as i32) + x as i32,
      (room_y as i32 * ROOM_TILE_HEIGHT as i32) + rev_y as i32
   );

   cell
}