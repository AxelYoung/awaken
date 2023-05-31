use harmony::*;

use super::player::Player;

use super::Game;

use super::render::Sprite;
use super::common::Cell;

use super::clones::Clone;


pub struct Button {
   cells: Vec<usize>,
   button_type: ButtonType,
   pressed: bool
}

#[derive(PartialEq, Clone, Copy)]
pub enum ButtonType {
   Any,
   AnyColor,
   Color(u8)
}

impl Button {
   pub fn new(button_type: ButtonType, cells: Vec<usize>) -> Self {
      Self {
         cells,
         button_type,
         pressed: false
      }
   }
}

pub fn update(game: &mut Game) {
   check_buttons(game);
}

fn check_buttons(game: &mut Game) {

   let mut pressed_buttons = vec![];
   let mut unpressed_buttons = vec![];

   iterate_entities_with_id!(game.world, [Button, Cell], (Sprite), 
   |id, button: &Button, button_cell: &Cell, sprite: &mut Sprite| {
      
      let mut pressed = false;

      match button.button_type {
         ButtonType::Any => {
            iterate_entities!(game.world, [Cell, Player], 
            |player_cell: &Cell, _| {
               pressed = player_cell == button_cell;
            });
            iterate_entities!(game.world, [Cell, Clone], 
               |player_cell: &Cell, _| {
                  pressed = player_cell == button_cell;
            });
         }
         ButtonType::AnyColor => {
            iterate_entities!(game.world, [Cell, Player], 
            |player_cell: &Cell, _| {
               pressed = player_cell == button_cell;
            });
            iterate_entities!(game.world, [Cell, Clone], 
               |player_cell: &Cell, _| {
                  pressed = player_cell == button_cell;
            });
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
         }
      }

      if pressed && !button.pressed{
         pressed_buttons.push(id);
         sprite.index_y = 8;

      } else {
         sprite.index_y = 7;
         unpressed_buttons.push(id);
      }
   });

   for button in pressed_buttons {

      let button_sprite = 
         game.world.get_component_from_entity_mut::<Sprite>(button)
         .unwrap().as_mut().unwrap();

      button_sprite.index_y = 8;

      let button_cells;

      {
         let button_component =
         game.world.get_component_from_entity_mut::<Button>(button)
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

         game.colliders
            [cell_component.x as usize]
            [cell_component.y as usize]
            = false;
      }
   }

   for button in unpressed_buttons {

      let button_sprite = 
         game.world.get_component_from_entity_mut::<Sprite>(button)
         .unwrap().as_mut().unwrap();

      button_sprite.index_y = 7;

      let button_cells;

      {
         let button_component =
         game.world.get_component_from_entity_mut::<Button>(button)
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
            [cell_component.x as usize]
            [cell_component.y as usize]
            = true;
      }
   }
}