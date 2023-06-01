use harmony::*;

use crate::{clones, movement};
use crate::collision::{Collider, Hit};
use crate::math::Vec2;

use super::common::Position;

use super::movement::Moveable;

use super::render::Sprite;

use super::Game;

use super::common::Cell;
use super::math::Vec2i;
use super::collision::check_collision;

use super::clones::{Clone, Playback};

pub struct Player {
   pub moved: bool,
   pub playback_dir: Vec<Vec2i>,
   pub color: u8,
   pub marker: usize
}

impl Player {
   pub fn new(marker: usize) -> Self {
      Self {
         moved: false,
         playback_dir: vec![],
         color: 0,
         marker
      }
   }
}

pub fn create(game: &mut Game) {
   let player = game.world.new_entity();
   let spawn_cell = game.clone_spawns[0];
   game.world.add_component_to_entity(player, spawn_cell);
   game.world.add_component_to_entity(player, spawn_cell.to_position());
   game.world.add_component_to_entity(player, Sprite::new(0, 0, 100));
   let marker = game.world.new_entity();
   game.world.add_component_to_entity(marker, Position::new(0.0, 0.0));
   game.world.add_component_to_entity(marker, Sprite::new(0, 17, 127));
   game.world.add_component_to_entity(player, Player::new(marker));
   game.world.add_component_to_entity(player, Moveable {
      start_cell: Cell::new(0, 0),
      end_cell: Cell::new(0, 0),
      duration: 150,
      accumulator: 0,
      moving: false,
      box_moveable: false
   });

   game.player = player;
}

pub fn update(game: &mut Game) {
   set_movement(game);
   skip(game);
   update_marker(game);
}

fn update_marker(game: &mut Game) {

   let player_position = 
      game.world.get_component_from_entity_mut::<Position>(game.player)
      .unwrap().as_mut().unwrap().value.clone();

   let marker = 
      game.world.get_component_from_entity_mut::<Player>(game.player)
      .unwrap().as_mut().unwrap().marker;

   let marker_position =
      game.world.get_component_from_entity_mut::<Position>(marker)
      .unwrap().as_mut().unwrap();

   marker_position.value = player_position + Vec2::new(0.0, 5.0);
}

fn set_movement(game: &mut Game) {
   let direction = game.input.direction();

   let mut set_moveable = false;

   let mut goal_cell = Cell::new(0, 0);

   let mut box_entity = None; 
   let mut box_new_cell = Cell::new(0, 0);

   iterate_entities!(game.world, 
      [Moveable], (Player, Cell, Position, Sprite),
      |moveable: &Moveable, player: &mut Player, cell: &mut Cell, 
      position: &mut Position, sprite: &mut Sprite| 
   {
      if !moveable.moving {
         if direction != Vec2i::zero() {
            let new_cell = Cell::new(
               cell.x + direction.x, 
               cell.y + direction.y
            );
            match check_collision(game, player.color, new_cell) {
               Hit::None => {
                  set_moveable = true;

                  player.playback_dir.push(direction);
   
                  player.moved = true;
   
                  update_sprite_dir(direction, sprite);
   
                  goal_cell = new_cell;
               }
               Hit::Solid => {
                  player.moved = true;
                  set_moveable = true;
   
                  player.playback_dir.push(direction);
                  
                  update_sprite_dir(direction, sprite);
   
                  goal_cell = cell.clone();
               }
               Hit::Box(box_id) => {
                  box_new_cell = Cell::new(
                     new_cell.x + direction.x,
                     new_cell.y + direction.y
                  );

                  if check_collision(game, 6, box_new_cell) == Hit::None {
                     box_entity = Some(box_id);

                     set_moveable = true;

                     player.playback_dir.push(direction);
      
                     player.moved = true;
      
                     update_sprite_dir(direction, sprite);
      
                     goal_cell = new_cell;
                  }                  
               }
            }
         }
      }
   });

   if set_moveable {

      let mut cell = Cell::new(0, 0);

      {
         cell = *game.world.get_component_from_entity_mut::<Cell>(game.player)
            .unwrap().as_mut().unwrap();
      }

      let moveable = 
         game.world.get_component_from_entity_mut::<Moveable>(game.player)
         .unwrap().as_mut().unwrap();

      moveable.start_cell = cell;
      moveable.end_cell = goal_cell;
      moveable.moving = true;

      let mut clones = vec![];

      iterate_entities_with_id!(game.world, [Clone], |id, _| {
         clones.push(id);
      });

      for clone in clones {
         game.world.add_component_to_entity(clone, Playback{});
      }
   }

   if let Some(box_id) = box_entity {
      let moveable = 
         game.world.get_component_from_entity_mut::<Moveable>(box_id)
         .unwrap().as_mut().unwrap();

      moveable.start_cell = goal_cell;
      moveable.end_cell = box_new_cell;
      moveable.moving = true;
   }
}

fn skip(game: &mut Game) {
   if game.input.skip_pressed {

      let player_cell;

      {
         let player_cell_component =
            game.world.get_component_from_entity_mut::<Cell>(game.player)
            .unwrap().as_mut().unwrap();

         player_cell = player_cell_component.clone();
      }

      let player = 
         game.world.get_component_from_entity_mut::<Player>(game.player)
         .unwrap().as_mut().unwrap();

      player.playback_dir.push(Vec2i::zero());

      let mut clones = vec![];

      iterate_entities_with_id!(game.world, [Clone], |id, _| {
         clones.push(id);
      });
   
      for clone in clones {
         game.world.add_component_to_entity(clone, Playback{});
      }
   }
}

fn update_sprite_dir(dir: Vec2i, sprite: &mut Sprite) {
   sprite.index_x = match dir {
      Vec2i {x: 0, y: -1 } => { 0 },
      Vec2i {x: 0, y: 1 } => { 1 },
      Vec2i {x: 1, y: 0 } => { 2 },
      Vec2i {x: -1, y: 0 } => { 3 },
      _ =>  {0}
   }
}