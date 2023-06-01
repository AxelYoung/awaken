use crate::boxes::BoxType;

use super::Game;
use super::common::Cell;

#[derive(Clone, Copy, PartialEq)]
pub enum Collider {
   Solid,
   Box(BoxType, usize),
   Empty
}

#[derive(PartialEq)]
pub enum Hit {
   None,
   Box(usize),
   Solid
}

pub fn check_collision(game: &Game, player_color: u8, cell: Cell) -> Hit {
   match game.colliders[cell.x as usize][cell.y as usize] {
      Collider::Solid => { Hit::Solid }
      Collider::Box(box_type, id) => {
         match box_type {
            BoxType::Any => { Hit::Box(id) }
            BoxType::Color(col) => {
               if col == player_color {
                  Hit::Box(id)
               } else {
                  Hit::Solid
               }
            }
         }
      },
      Collider::Empty => { Hit::None }
   }
}