use harmony::*;

use super::render::Sprite;

use super::common::Position;

use super::Game;

use super::math::Vec2;

use super::clones::Clone;

const TRAIL_FREQUENCY: f32 = 16.0;

pub struct Trail {
   pub last_trail: Vec2,
   pub sprite: Sprite,
   pub entities: Vec<usize>
}

impl Trail {
   pub fn new(sprite: Sprite) -> Self {
      Self {
         last_trail: Vec2::zero(),
         sprite,
         entities: vec![]
      }
   }
}

pub fn update(game: &mut Game) {
   create_trails(game);
   render_upcoming_trails(game);
}

fn create_trails(game: &mut Game) {

   let mut new_trails: Vec<(Position, Sprite, usize)> = vec![];

   iterate_entities_with_id!(game.world, [Position], (Trail), 
   |id, position: &Position, trail: &mut Trail| {
      let dist = position.dist(trail.last_trail);

      if dist >= TRAIL_FREQUENCY {
         new_trails.push((
            *position, 
            trail.sprite,
            id
         ));
         trail.last_trail = position.value;
      }
   });

   for trail in new_trails {
      let new_trail = game.world.new_entity();

      game.world.add_component_to_entity(new_trail, trail.0);
      game.world.add_component_to_entity(new_trail, trail.1);

      let trail_parent = 
         game.world.get_component_from_entity_mut::<Trail>(trail.2)
         .unwrap().as_mut().unwrap();

      trail_parent.entities.push(new_trail);
   }
}

fn render_upcoming_trails(game: &mut Game) {

   let mut hide_trails = vec![];
   
   iterate_entities!(game.world, [Clone], 
   |clone: &Clone| {
      for i in 0..=clone.current_move {
         hide_trails.push(clone.trails[i]);
      }
   });

   for trail in hide_trails {
      let trail_sprite = 
         game.world.get_component_from_entity_mut::<Sprite>(trail)
         .unwrap().as_mut().unwrap();

      trail_sprite.render = false;
   }
}