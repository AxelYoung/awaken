use harmony::*;

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
   playback: Vec<Cell>,
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

      let mut playback = vec![];

      {
         let player = 
         game.world.get_component_from_entity_mut::<Player>(game.player)
         .unwrap().as_mut().unwrap();

         playback = player.playback.clone();

         player.playback.clear();
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

      let mut clone_trails = vec![];

      iterate_entities!(game.world, (Clone, Position, Cell),
         |clone: &mut Clone, position: &mut Position, cell: &mut Cell| {
            clone.current_move = 0;
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

      if clone_count < 4 {
         let clone = game.world.new_entity();

         let mut playback_dir = vec![];

         playback_dir.push(playback[0].value - game.clone_spawns[clone_count].value);

         for i in 1..playback.len() {
            playback_dir.push(playback[i].value - playback[i - 1].value);
         }

         game.world.add_component_to_entity(clone, 
            Clone {
               color: clone_count as u8,
               playback,
               playback_dir,
               current_move: 0,
               paths: vec![]
            }
         );

         game.world.add_component_to_entity(clone, 
            Sprite::new(0, clone_count as u32, 100)
         );

         let start_cell =  game.clone_spawns[clone_count as usize];

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
               moving: false
            }
         );
      }
      
      let player_color;

      {
         let player = 
         game.world.get_component_from_entity_mut::<Player>(game.player)
         .unwrap().as_mut().unwrap();

         player.color += 1;

         player_color = player.color;
      }


      let player_sprite =
         game.world.get_component_from_entity_mut::<Sprite>(game.player)
         .unwrap().as_mut().unwrap();

      player_sprite.index_y = player_color.into();

      let player_cell = 
         game.world.get_component_from_entity_mut::<Cell>(game.player)
         .unwrap().as_mut().unwrap();

      *player_cell = game.clone_spawns[player_color as usize];

      let player_position =
         game.world.get_component_from_entity_mut::<Position>(game.player)
         .unwrap().as_mut().unwrap();

      *player_position = game.clone_spawns[player_color as usize].to_position();
   }
}

fn playback(game: &mut Game) {

   let mut played = vec![];

   iterate_entities_with_id!(game.world, 
      [Playback, Cell], (Clone, Sprite, Moveable), 
      |id, _, cell: &Cell, clone: &mut Clone, 
      sprite: &mut Sprite, moveable: &mut Moveable| {

         if clone.current_move < clone.playback.len() {
            moveable.start_cell = *cell;
            moveable.end_cell = clone.playback[clone.current_move];
            moveable.moving = true;
   
            let dir = moveable.end_cell.value - moveable.start_cell.value;
   
            update_sprite_dir(dir, sprite);

            played.push(id);
   
            clone.current_move += 1;
         }
      }
   );

   for clone in played {
      game.world.remove_component_from_entity::<Playback>(clone);
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

   let mut paths_to_remove = vec![];
   let mut paths_to_add : Vec<(usize, Cell, Sprite)>= vec![];

   // iterate through all clones
   iterate_entities_with_id!(game.world, [Moveable], (Clone), 
   |id, moveable: &Moveable, clone: &mut Clone| {
      // remove all paths in that clone and add it to the paths to remove
      paths_to_remove.append(&mut clone.paths);

      let mut clone_paths = vec![];

      // start paths to add with the clone spawn point
      clone_paths.push((id, moveable.end_cell, 
         Sprite::new(5, clone.color.into(), 2))
      );

      let mut path_end = clone.current_move + 10;

      if path_end > clone.playback_dir.len() { 
         path_end = clone.playback_dir.len() 
      }

      for i in clone.current_move..path_end {
         let mut new_cell = 
         clone_paths[i - clone.current_move].1.value + 
            clone.playback_dir[i];
         
         if game.colliders[new_cell.x as usize][new_cell.y as usize] {
            new_cell = clone_paths[i - clone.current_move].1.value;
         }

         let x = match clone.playback_dir[i] {
            Vec2i {x: 0, y: 1} => {5},
            Vec2i {x: 0, y: -1} => {4},
            Vec2i {x: 1, y: 0} => {6},
            _ => {7},
         };
      
         clone_paths.push((id, Cell { value: new_cell }, 
            Sprite::new(x, clone.color.into(), 2)));
      }

      clone_paths.remove(0);

      paths_to_add.append(&mut clone_paths);

   });

   for i in 0..paths_to_add.len() {
      let path = game.world.new_entity();

      game.world.add_component_to_entity(path, paths_to_add[i]);
      game.world.add_component_to_entity(path, paths_to_add[i].1.to_position());
      game.world.add_component_to_entity(path, paths_to_add[i].2);

      let clone = 
         game.world.get_component_from_entity_mut::<Clone>(paths_to_add[i].0)
         .unwrap().as_mut().unwrap();

      clone.paths.push(path);

   }

   for path in paths_to_remove {
      game.world.delete_entity(path);
   }

}