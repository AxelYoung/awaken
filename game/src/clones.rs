use std::path;

use harmony::*;

use crate::boxes::PushBox;
use crate::collision::{Collider, check_collision, Hit};

use super::math::Vec2;

use super::common::Position;
use super::movement::Moveable;

use super::Game;

use super::common::Cell;
use super::player::Player;
use super::render::Sprite;

use super::math::Vec2i;

pub struct Clone {
   pub color: u8,
   playback_dir: Vec<Vec2i>,
   pub current_move: usize,
   pub paths: Vec<usize>
}

pub struct Playback { }


pub fn update(game: &mut Game) {
   switch_clone(game);
   playback(game);
}

pub fn fixed_update(game: &mut Game) {
   update_paths(game);
}

pub fn switch_clone(game: &mut Game) {
   if game.input.loop_pressed {

      let mut clone_count = 0;

      let playback_dir;

      {
         let player = 
         game.world.get_component_from_entity_mut::<Player>(game.player)
         .unwrap().as_mut().unwrap();

         playback_dir = player.playback_dir.clone();

         player.playback_dir.clear();
      }

      let mut clones = vec![];

      iterate_entities_with_id!(game.world, [Clone], |id, _| {
         clones.push(id);
      });

      for clone in clones {
         game.world.remove_component_from_entity::<Playback>(clone);

         let clone_color = {
            let clone_component = 
               game.world.get_component_from_entity_mut::<Clone>(clone)
               .unwrap().as_mut().unwrap();

            clone_component.color
         };
         
         let moveable = 
            game.world.get_component_from_entity_mut::<Moveable>(clone)
            .unwrap().as_mut().unwrap();

         moveable.start_cell = game.clone_spawns[clone_color as usize];
         moveable.end_cell = game.clone_spawns[clone_color as usize];
      }

      let clone_trails = vec![];

      iterate_entities!(game.world, (Clone, Position, Cell),
         |clone: &mut Clone, position: &mut Position, cell: &mut Cell| {
            clone.current_move = 0;
            game.colliders[cell.x as usize][cell.y as usize] = Collider::Empty;
            let spawn = game.clone_spawns[clone.color as usize];
            *cell = spawn;
            *position = cell.to_position();
            clone_count += 1;
         }
      );

      for trail in clone_trails {
         let trail_sprite = 
            game.world.get_component_from_entity_mut::<Sprite>(trail)
            .unwrap().as_mut().unwrap();

         trail_sprite.render = true;
      }

      let clone = game.world.new_entity();

      let mut paths = vec![];

      for _ in 0..playback_dir.len() {
         paths.push(game.world.new_entity());
      }

      game.world.add_component_to_entity(clone, 
         Clone {
            color: game.current_clone as u8,
            playback_dir,
            current_move: 0,
            paths
         }
      );

      game.world.add_component_to_entity(clone, 
         Sprite::new(0, game.current_clone as u32, 100)
      );

      let start_cell =  game.clone_spawns[game.current_clone];

      game.world.add_component_to_entity(clone, 
         start_cell
      );

      game.world.add_component_to_entity(clone, 
         start_cell.to_position()
      );

      game.world.add_component_to_entity(clone, 
         Moveable {
            start_cell: start_cell,
            end_cell: start_cell,
            duration: 150,
            accumulator: 0,
            moving: false,
            box_moveable: false
         }
      );

      game.clones[game.current_clone] = clone;

      game.current_clone += 1;
      clone_count += 1;

      if clone_count > 0 {
         if game.current_clone == 5 {
            game.current_clone = 0;
         }
         if clone_count > 4 {
            let old_clone = 
               game.world.get_component_from_entity_mut::<Clone>
               (game.clones[game.current_clone]).unwrap().as_mut().unwrap();

            let old_paths = old_clone.paths.clone();

            for path in old_paths {
               game.world.delete_entity(path);
            }

            game.world.delete_entity(game.clones[game.current_clone]);
         }   
      }

      let player_color;

      {
         let player = 
         game.world.get_component_from_entity_mut::<Player>(game.player)
         .unwrap().as_mut().unwrap();

         player.color = game.current_clone as u8;

         player_color = player.color;
      }

      let player_sprite =
         game.world.get_component_from_entity_mut::<Sprite>(game.player)
         .unwrap().as_mut().unwrap();

      player_sprite.index_y = player_color.into();

      let player_cell = 
         game.world.get_component_from_entity_mut::<Cell>(game.player)
         .unwrap().as_mut().unwrap();

      game.colliders[player_cell.x as usize][player_cell.y as usize] = 
         Collider::Empty;

      *player_cell = game.clone_spawns[player_color as usize];

      let player_position =
         game.world.get_component_from_entity_mut::<Position>(game.player)
         .unwrap().as_mut().unwrap();

      *player_position = game.clone_spawns[player_color as usize].to_position();

      let mut old_boxes = vec![];
      let mut new_boxes = vec![];

      iterate_entities!(game.world, [PushBox], (Position, Cell),
      |box_component: &PushBox, position: &mut Position, cell: &mut Cell| {
         old_boxes.push(cell.clone());

         cell.value = box_component.start_cell.value;
         position.value = box_component.start_cell.to_position().value;

         new_boxes.push(box_component.start_cell);
      });

      let mut box_colliders = vec![];

      for old_box in old_boxes {
         box_colliders.push(game.colliders[old_box.x as usize][old_box.y as usize]);
         game.colliders[old_box.x as usize][old_box.y as usize] = Collider::Empty;
      }

      for i in 0..new_boxes.len() {
         game.colliders[new_boxes[i].x as usize][new_boxes[i].y as usize] = box_colliders[i];
      }
   }
}

fn playback(game: &mut Game) {

   let mut played = vec![];

   let mut box_new_cells = vec![];

   iterate_entities_with_id!(game.world, 
      [Playback, Cell], (Clone, Sprite, Moveable), 
      |id, _, cell: &Cell, clone: &mut Clone, 
      sprite: &mut Sprite, moveable: &mut Moveable| {

         if clone.current_move < clone.playback_dir.len() {
            moveable.start_cell = *cell;

            let mut next_cell = 
               cell.value + clone.playback_dir[clone.current_move];

            match check_collision(game, clone.color, Cell::new(next_cell.x, next_cell.y)) {
               Hit::None => {

               }
               Hit::Solid => {
                  next_cell = cell.value;
               }
               Hit::Box(box_id) => {
                  let box_new_cell = Cell::new(
                     next_cell.x + clone.playback_dir[clone.current_move].x,
                     next_cell.y + clone.playback_dir[clone.current_move].y
                  );

                  if check_collision(game, 6, box_new_cell) == Hit::None {
                     box_new_cells.push((box_id, next_cell, box_new_cell))
                  } else {
                     next_cell = cell.value
                  }
               }
            }

            let next_cell = Cell::new(next_cell.x, next_cell.y);
            
            moveable.end_cell = next_cell;
            moveable.moving = true;
   
            update_sprite_dir(clone.playback_dir[clone.current_move], sprite);

            played.push(id);
   
            clone.current_move += 1;
         }
      }
   );

   for clone in played {
      game.world.remove_component_from_entity::<Playback>(clone);
   }

   for box_entity in box_new_cells {
      let moveable = 
         game.world.get_component_from_entity_mut::<Moveable>(box_entity.0)
         .unwrap().as_mut().unwrap();

      moveable.start_cell = Cell::new(box_entity.1.x, box_entity.1.y);
      moveable.end_cell = box_entity.2;
      moveable.moving = true;
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

fn update_paths(game: &mut Game) {

   let mut paths_to_add : Vec<(usize, Cell, Sprite, usize)>= vec![];

   let mut end_points = vec![];

   iterate_entities_with_id!(game.world, [Moveable], (Clone), 
   |id, moveable: &Moveable, clone: &mut Clone| {

      let mut clone_paths = vec![];

      if clone.paths.len() > 0 {
         clone_paths.push((id, moveable.end_cell, 
            Sprite::new(5, clone.color.into(), 2), clone.paths[0])
         );
   
         let mut path_end = clone.current_move + 6;
   
         if path_end > clone.playback_dir.len() { 
            path_end = clone.playback_dir.len() 
         }
   
         for i in clone.current_move..path_end {
            let mut new_cell = 
            clone_paths[i - clone.current_move].1.value + 
               clone.playback_dir[i];
            
            if check_collision(game, clone.color, Cell::new(new_cell.x, new_cell.y)) == Hit::Solid {
               new_cell = clone_paths[i - clone.current_move].1.value;
            }
   
            let x = match clone.playback_dir[i] {
               Vec2i {x: 0, y: 1} => {5},
               Vec2i {x: 0, y: -1} => {4},
               Vec2i {x: 1, y: 0} => {6},
               _ => {7},
            };
         
            clone_paths.push((id, Cell { value: new_cell }, 
               Sprite::new(x, clone.color.into(), 2), clone.paths[i]));
         }
   
         clone_paths.remove(0);
   
         paths_to_add.append(&mut clone_paths);
   
         end_points.push((id, clone.current_move));
      }
   });

   for end in end_points {
      let clone_paths;

      {
         let clone = 
            game.world.get_component_from_entity_mut::<Clone>(end.0)
            .unwrap().as_mut().unwrap();

         clone_paths = clone.paths.clone();
      }
      for i in 0..end.1 {
         let path_sprite = 
            game.world.get_component_from_entity_mut::<Sprite>(
               clone_paths[i]
            ).unwrap().as_mut().unwrap();

         path_sprite.render = false;
      }
   }

   for i in 0..paths_to_add.len() {
      
      let path = paths_to_add[i].3;

      game.world.add_component_to_entity(path, paths_to_add[i]);
      game.world.add_component_to_entity(path, paths_to_add[i].1.to_position());
      game.world.add_component_to_entity(path, paths_to_add[i].2);

      let clone = 
         game.world.get_component_from_entity_mut::<Clone>(paths_to_add[i].0)
         .unwrap().as_mut().unwrap();

      clone.paths.push(path);

   }
}