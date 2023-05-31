#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Vec2 {
   pub x: f32,
   pub y: f32
}

impl Vec2 {
   pub fn new(x: f32, y: f32) -> Self { Self { x, y } }
   
   pub fn dist(&self, comp: Vec2) -> f32 {
      let dx = comp.x - self.x;
      let dy = comp.y - self.y;
      (dx * dx + dy * dy).sqrt()
   }

   pub fn zero() -> Self { Self { x: 0.0, y: 0.0 } }
}

impl std::ops::Mul<f32> for Vec2 {
   type Output = Vec2;
   
   fn mul(self, rhs: f32) -> Self::Output {
      Self {
         x: self.x * rhs,
         y: self.y * rhs
      }
   }
}

impl std::ops::Add<Vec2> for Vec2 {
   type Output = Vec2;
   
   fn add(self, rhs: Vec2) -> Self::Output {
      Self {
         x: self.x + rhs.x,
         y: self.y + rhs.y
      }
   }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Vec2i {
   pub x: i32,
   pub y: i32
}

impl Vec2i {
   pub fn new(x: i32, y: i32) -> Self { Self {x, y} }
   pub fn zero() -> Self { Self { x: 0, y: 0 } }
}

impl std::ops::Add<Vec2i> for Vec2i {
   type Output = Vec2i;
   
   fn add(self, rhs: Vec2i) -> Self::Output {
      Self {
         x: self.x + rhs.x,
         y: self.y + rhs.y
      }
   }
}

impl std::ops::Sub<Vec2i> for Vec2i {
   type Output = Vec2i;

   fn sub(self, rhs: Vec2i) -> Self::Output {
       Vec2i::new(self.x - rhs.x, self.y - rhs.y)
   }
}