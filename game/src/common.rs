use super::math::Vec2;

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