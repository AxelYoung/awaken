use harmony::*;

use super::trails::Trail;

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
   pub playback: Vec<Cell>,
   pub color: u8
}

impl Player {
   pub fn new() -> Self {
      Self {
         moved: false,
         playback: vec![],
         color: 0
      }
   }
}

pub fn create(game: &mut Game) {
   let player = game.world.new_entity();
   let spawn_cell = game.clone_spawns[0];
   game.world.add_component_to_entity(player, spawn_cell);
   game.world.add_component_to_entity(player, spawn_cell.to_position());
   game.world.add_component_to_entity(player, Sprite::new(0, 0, 100));
   game.world.add_component_to_entity(player, Player::new());
   game.world.add_component_to_entity(player, Moveable {
      start_cell: Cell::new(0, 0),
      end_cell: Cell::new(0, 0),
      duration: 150,
      accumulator: 0,
      moving: false
   });
   game.world.add_component_to_entity(player, Trail::new(Sprite::new(4, 0, 5)));

   game.player = player;
}

pub fn update(game: &mut Game) {
   set_movement(game);
}

fn set_movement(game: &mut Game) {
   let direction = game.input.directon();

   let mut set_moveable = false;

   iterate_entities!(game.world, 
      [Moveable], (Player, Cell, Position, Sprite, Trail),
      |moveable: &Moveable, player: &mut Player, cell: &mut Cell, 
      position: &mut Position, sprite: &mut Sprite, trail: &mut Trail| 
   {
      if !moveable.moving {
         if direction != Vec2i::zero() {
            let goal_cell = Cell::new(
               cell.x + direction.x, 
               cell.y + direction.y
            );
            if !check_collision(game, goal_cell) {
               set_moveable = true;

               player.playback.push(goal_cell);

               player.moved = true;

               update_sprite_dir(direction, sprite);

               trail.sprite = Sprite::new(sprite.index_x + 4, sprite.index_y, 5);
            }
         }
      }
   });

   if set_moveable {

      let mut cell = Cell::new(0, 0);
      let mut goal_cell = Cell::new(0, 0);

      {
         cell = *game.world.get_component_from_entity_mut::<Cell>(game.player)
            .unwrap().as_mut().unwrap();

         goal_cell = Cell::new(cell.x + direction.x, cell.y + direction.y);
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