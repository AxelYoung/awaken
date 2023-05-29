use harmony::*;

use super::math::Vec2;

use super::common::Position;
use super::movement::Moveable;
use super::trails::Trail;

use super::Game;

use super::common::Cell;
use super::player::Player;
use super::render::Sprite;

use super::math::Vec2i;

pub struct Clone {
   color: u8,
   playback: Vec<Cell>,
   pub current_move: usize,
   pub trails: Vec<usize>
}

pub struct Playback { }

pub fn update(game: &mut Game) {
   switch_clone(game);
   playback(game);
}

pub fn switch_clone(game: &mut Game) {
   if game.input.loop_pressed {
      
      let mut clone_count = 0;

      let mut playback = vec![];

      let trails;

      {
         let player = 
         game.world.get_component_from_entity_mut::<Player>(game.player)
         .unwrap().as_mut().unwrap();

         playback = player.playback.clone();

         player.playback.clear();
      }

      let mut playbacks = vec![];

      iterate_entities_with_id!(game.world, [Clone, Playback], |id, _, _| {
         playbacks.push(id);
      });

      for clone in playbacks {
         game.world.remove_component_from_entity::<Playback>(clone);
      }

      let mut clone_trails = vec![];

      iterate_entities!(game.world, (Clone, Position, Cell),
         |clone: &mut Clone, position: &mut Position, cell: &mut Cell| {
            clone.current_move = 0;
            let spawn = game.clone_spawns[clone.color as usize];
            *cell = spawn;
            *position = cell.to_position();
            for trail in clone.trails.clone() {
               clone_trails.push(trail);
            }
            clone_count += 1;
         }
      );

      for trail in clone_trails {
         let trail_sprite = 
            game.world.get_component_from_entity_mut::<Sprite>(trail)
            .unwrap().as_mut().unwrap();

         trail_sprite.render = true;
      }

      {
         let player_trail = 
         game.world.get_component_from_entity_mut::<Trail>(game.player)
         .unwrap().as_mut().unwrap();

         trails = player_trail.entities.clone();

         player_trail.entities = vec![];

         player_trail.last_trail = Vec2::zero();

         player_trail.sprite = Sprite::new(4, clone_count + 1, 5);
      }
 
      if clone_count < 4 {
         let clone = game.world.new_entity();

         game.world.add_component_to_entity(clone, 
            Clone {
               color: clone_count as u8,
               playback,
               current_move: 0,
               trails
            }
         );

         game.world.add_component_to_entity(clone, 
            Sprite::new(0, clone_count.into(), 100)
         );

         game.world.add_component_to_entity(clone, 
            game.clone_spawns[clone_count as usize]
         );

         game.world.add_component_to_entity(clone, 
            game.clone_spawns[clone_count as usize].to_position()
         );

         game.world.add_component_to_entity(clone, 
            Moveable {
               start_cell: Cell::new(0, 0),
               end_cell: Cell::new(0, 0),
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