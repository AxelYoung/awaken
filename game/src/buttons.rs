use harmony::*;

use crate::collision::Collider;

use super::player::Player;

use super::Game;

use super::render::Sprite;
use super::common::Cell;

use super::clones::Clone;

pub struct Button {
   cells: Vec<usize>,
   button_type: ButtonType,
   pressed: bool,
   slaves: Vec<usize>,
   wires: Vec<usize>
}

pub struct SlaveButton {
   pub button_type: ButtonType,
   pub pressed: bool
}

#[derive(PartialEq, Clone, Copy)]
pub enum ButtonType {
   AnyColor,
   Color(u8)
}

impl Button {
   pub fn new(button_type: ButtonType, cells: Vec<usize>, slaves: Vec<usize>,
   wires: Vec<usize>) 
   -> Self {
      Self {
         cells,
         button_type,
         pressed: false,
         slaves,
         wires
      }
   }
}

pub fn update(game: &mut Game) {
   check_buttons(game);
}

fn check_buttons(game: &mut Game) {

   let mut pressed_buttons = vec![];
   let mut unpressed_buttons = vec![];

   let mut masters = vec![];

   iterate_entities_with_id!(game.world, [Cell], (Button, Sprite), 
   |id, button_cell: &Cell, button: &mut Button, sprite: &mut Sprite| {
      
      let mut pressed = false;

      match button.button_type {
         ButtonType::AnyColor => {
            iterate_entities!(game.world, [Cell, Player], 
            |player_cell: &Cell, _| {
               if player_cell == button_cell {
                  pressed = true;
               }
            });
            iterate_entities!(game.world, [Cell, Clone], 
            |clone_cell: &Cell, _| {
               if clone_cell == button_cell {
                  pressed = true;
               }
            });
            match game.colliders[button_cell.x as usize][button_cell.y as usize] {
               Collider::Box(_, _) => {
                  pressed = true;
               },
               _ => {}
            }
         }
         ButtonType::Color(color) => {
            iterate_entities!(game.world, [Cell, Player], 
            |player_cell: &Cell, player: &Player| {
               if player.color == color {
                  if player_cell == button_cell {
                     pressed = true;
                  }
               }
            });
            iterate_entities!(game.world, [Cell, Clone], 
            |clone_cell: &Cell, clone: &Clone| {
               if clone.color == color {
                  if clone_cell == button_cell {
                     pressed = true;
                  }
               }
            });
            match game.colliders[button_cell.x as usize][button_cell.y as usize] {
               Collider::Box(box_type, _) => {
                  match box_type {
                     crate::boxes::BoxType::Color(box_color) => {
                        if box_color == color {
                           pressed = true;
                        }
                     }
                     _ => {}
                  }
               },
               _ => {}
            }
         }
      }

      if pressed && !button.pressed{
         pressed_buttons.push(id);
         sprite.index_y = 8;
         button.pressed = true;

      } else if !pressed && button.pressed {
         sprite.index_y = 7;
         unpressed_buttons.push(id);
         button.pressed = false;
      }

      masters.push(id);
   });

   iterate_entities_with_id!(game.world, [Cell], (SlaveButton, Sprite), 
   |id, button_cell: &Cell, button: &mut SlaveButton,  sprite: &mut Sprite| {
      
      let mut pressed = false;

      match button.button_type {
         ButtonType::AnyColor => {
            iterate_entities!(game.world, [Cell, Player], 
            |player_cell: &Cell, _| {
               if player_cell == button_cell {
                  pressed = true;
               }
            });
            iterate_entities!(game.world, [Cell, Clone], 
            |player_cell: &Cell, _| {
               if player_cell == button_cell {
                  pressed = true;
               }
            });
            match game.colliders[button_cell.x as usize][button_cell.y as usize] {
               Collider::Box(_, _) => {
                  pressed = true;
               },
               _ => {}
            }
         }
         ButtonType::Color(color) => {
            iterate_entities!(game.world, [Cell, Player], 
            |player_cell: &Cell, player: &Player| {
               if player.color == color {
                  pressed = player_cell == button_cell;
               }
            });
            iterate_entities!(game.world, [Cell, Clone], 
            |player_cell: &Cell, clone: &Clone| {
               if clone.color == color {
                  pressed = player_cell == button_cell;
               }
            });
            match game.colliders[button_cell.x as usize][button_cell.y as usize] {
               Collider::Box(box_type, _) => {
                  match box_type {
                     crate::boxes::BoxType::Color(box_color) => {
                        if box_color == color {
                           pressed = true;
                        }
                     }
                     _ => {}
                  }
               },
               _ => {}
            }
         }
      }

      if pressed && !button.pressed{
         pressed_buttons.push(id);
         sprite.index_y = 8;
         button.pressed = true;

      } else if !pressed && button.pressed{
         sprite.index_y = 7;
         unpressed_buttons.push(id);
         button.pressed = false;
      }
   });

   for button in pressed_buttons {

      let button_sprite = 
         game.world.get_component_from_entity_mut::<Sprite>(button)
         .unwrap().as_mut().unwrap();

      button_sprite.index_y = 8;
   }

   for button in unpressed_buttons {

      let button_sprite = 
         game.world.get_component_from_entity_mut::<Sprite>(button)
         .unwrap().as_mut().unwrap();

      button_sprite.index_y = 7;
   }

   for master in masters {
      let mut open = true;

      let slaves;
      let master_pressed;

      let wires;

      {
         let master_component =
         game.world.get_component_from_entity_mut::<Button>(master)
         .unwrap().as_mut().unwrap();

         master_pressed = master_component.pressed;
         slaves = master_component.slaves.clone();
         wires = master_component.wires.clone();
      }

      let mut total_pressed = 0;

      if !master_pressed {
         open = false;
      } else {
         total_pressed += 1;
      }

      let total_buttons = 1 + slaves.len();

      for button in slaves {
         let button_component = 
            game.world.get_component_from_entity_mut::<SlaveButton>(button)
            .unwrap().as_mut().unwrap();

            if !button_component.pressed {
               open = false;
            } else {
               total_pressed += 1;
            }
      }

      for wire in wires {
         let wire_sprite = 
            game.world.get_component_from_entity_mut::<Sprite>(wire)
            .unwrap().as_mut().unwrap();

         let index = (total_pressed as f32 / total_buttons as f32) * 4.0;

         wire_sprite.index_y = 12 + index as u32;
      }

      if open {
         let button_cells;

         {
            let button_component =
            game.world.get_component_from_entity_mut::<Button>(master)
            .unwrap().as_mut().unwrap();
   
            button_cells = button_component.cells.clone();
         }
   
   
         for cell in button_cells {
            let cell_sprite = 
               game.world.get_component_from_entity_mut::<Sprite>(cell)
               .unwrap().as_mut().unwrap();
   
            cell_sprite.render = false;
   
            let cell_component = 
               game.world.get_component_from_entity_mut::<Cell>(cell)
               .unwrap().as_mut().unwrap();
   
            if game.colliders
               [cell_component.x as usize][cell_component.y as usize] == Collider::Solid {
                  game.colliders
                  [cell_component.x as usize][cell_component.y as usize] = 
                  Collider::Empty;
               }
         }
      } else {
         let button_cells;

         {
            let button_component =
            game.world.get_component_from_entity_mut::<Button>(master)
            .unwrap().as_mut().unwrap();
   
            button_cells = button_component.cells.clone();
         }
   
   
         for cell in button_cells {
            let cell_sprite = 
               game.world.get_component_from_entity_mut::<Sprite>(cell)
               .unwrap().as_mut().unwrap();
   
            cell_sprite.render = true;
   
            let cell_component = 
               game.world.get_component_from_entity_mut::<Cell>(cell)
               .unwrap().as_mut().unwrap();
   
            game.colliders
               [cell_component.x as usize][cell_component.y as usize] = 
               Collider::Solid;
         }
      }
   }
}