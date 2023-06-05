use super::render::{SPRITE_SIZE, SPRITE_CENTER};

use mathrix::vec::{Vec2, Vec2i};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Position {
  pub value: Vec2
}

impl Position {
  pub fn new(x: f32, y: f32) -> Self { Self { value: Vec2::new(x, y) } }
}

impl std::ops::Deref for Position {
  type Target = Vec2;
  fn deref(&self) -> &Vec2 { &self.value }
}

impl std::ops::DerefMut for Position {
  fn deref_mut(&mut self) -> &mut Vec2 { &mut self.value }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Cell {
  pub value: Vec2i
}

impl Cell {
  pub fn new(x: i32, y: i32) -> Self { Self { value: Vec2i::new(x, y) } }
  pub fn to_position(&self) -> Position {
    let position = Position::new(
      (self.x as f32 * SPRITE_SIZE as f32) + SPRITE_CENTER,
      (self.y as f32 * SPRITE_SIZE as f32) + SPRITE_CENTER
    );
  
    position
  }
}

impl std::ops::Deref for Cell {
  type Target = Vec2i;
  fn deref(&self) -> &Vec2i { &self.value }
}

impl std::ops::DerefMut for Cell {
  fn deref_mut(&mut self) -> &mut Vec2i { &mut self.value }
}
