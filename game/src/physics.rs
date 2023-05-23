use harmony::*;

use super::math::Vec2;
use super::common::Position;
use super::Game;
use super::render::SPRITE_SIZE;

pub struct Bounds {
    pub right: f32,
    pub bottom: f32,
}

pub struct Velocity {
    pub value: Vec2
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self { Self { value: Vec2::new(x, y) } }
}

impl std::ops::Deref for Velocity {
    type Target = Vec2;
    fn deref(&self) -> &Vec2 { &self.value }
}

impl std::ops::DerefMut for Velocity {
    fn deref_mut(&mut self) -> &mut Vec2 { &mut self.value }
}

pub struct Collider{}

pub fn fixed_update(game: &mut Game) {
    check_collisions(game);
    move_entity(game);
    velocity_drag(game);
}

fn move_entity(game: &mut Game) {
    iterate_entities!(game.world, [Velocity], (Position), 
        |velocity: &Velocity, position: &mut Position| {
            position.x += velocity.x;
            position.y += velocity.y;
    });
}

fn velocity_drag(game: &mut Game) {
    iterate_entities!(game.world, (Velocity), 
        |velocity: &mut Velocity| {
            velocity.x -= velocity.x * 0.2;
            velocity.y -= velocity.y * 0.2;
        }
    );
}

fn check_collisions(game: &mut Game) {
    iterate_entities!(game.world, [Position, Collider], (Velocity), 
    |position_a: &Position, _, velocity: &mut Velocity| {            
        iterate_entities!(game.world, [Position, Collider], 
        |position_b: &Position, _| {
            if position_a != position_b {
                let next_pos = Vec2::new(position_a.x + velocity.x, position_a.y+ velocity.y);
                if check_collision(next_pos, Bounds{right: SPRITE_SIZE as f32, bottom: SPRITE_SIZE as f32}, position_b.value, Bounds{right: SPRITE_SIZE as f32, bottom: SPRITE_SIZE as f32}) {
                    velocity.x = 0.0;
                    velocity.y = 0.0;
                }
            }
        });
    });
}

pub fn check_collision(pos_a: Vec2, bounds_a: Bounds, pos_b: Vec2, bounds_b: Bounds) -> bool {
    let right_a = pos_a.x + bounds_a.right;
    let bot_a = pos_a.y + bounds_a.bottom;
    let right_b = pos_b.x + bounds_b.right;
    let bot_b = pos_b.y + bounds_b.bottom;

    if pos_a.x < right_b && right_a > pos_b.x && pos_a.y < bot_b && bot_a > pos_b.y {
        return true;
    }

    false
}