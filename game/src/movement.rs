use harmony::*;

use super::common::Cell;

use super::common::Position;
use super::math::Vec2;

use super::Game;

pub struct Moveable {
   pub start_cell: Cell,
   pub end_cell: Cell,
   pub duration: u128,
   pub accumulator: u128,
   pub moving: bool
}

pub fn update(game: &mut Game) {
   move_entities(game);
}

fn move_entities(game: &mut Game) {
   iterate_entities!(game.world, (Moveable, Position, Cell), 
   |moveable: &mut Moveable, position: &mut Position, cell: &mut Cell| {
      if moveable.moving {
         let start_pos = moveable.start_cell.to_position();
         let end_pos = moveable.end_cell.to_position();
         
         game.colliders[cell.value.x as usize][cell.value.y as usize] = false;

         cell.value = moveable.end_cell.value;

         game.colliders[cell.value.x as usize][cell.value.y as usize] = true;

         if moveable.accumulator >= moveable.duration {
            position.value = end_pos.value;
            moveable.accumulator = 0;
            moveable.moving = false;
         } else {

            let x = start_pos.x as f32 + (
               (end_pos.x - start_pos.x) as f32 * 
               (moveable.accumulator as f32 / moveable.duration as f32)
            );

            let y = start_pos.y as f32 + (
               (end_pos.y - start_pos.y) as f32 * 
               (moveable.accumulator as f32 / moveable.duration as f32)
            );
            
            position.value = Vec2::new(x, y);
      
            moveable.accumulator += game.delta_time;
         }
      }
   });
}