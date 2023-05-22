#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self { Self { x, y } }
    /*
    pub fn dist(&self, comp: Vec2) -> f32 {
        let dx = comp.x - self.x;
        let dy = comp.y - self.y;
        (dx * dx + dy * dy).sqrt()
    }
     */
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