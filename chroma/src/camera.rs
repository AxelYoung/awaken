pub struct Camera {
  pub x: f32,
  pub y: f32  
}

impl Camera {
  pub fn new() -> Self { Self { x: 0.0, y: 0.0 } }

  pub fn to_raw(&self) -> CameraRaw { 
    let matrix_raw: [[f32; 4]; 4] = self.to_matrix().into();
    
    CameraRaw::new(matrix_raw) 
  }

  fn to_matrix(&self) -> cgmath::Matrix4<f32>{
    let translation = cgmath::Vector3 { 
      x: self.x, 
      y: self.y, 
      z: 0.0 
    };
    
    cgmath::Matrix4::from_translation(translation)
  }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraRaw {
  matrix: [[f32; 4]; 4]
}

impl CameraRaw {
  pub fn new(matrix: [[f32; 4]; 4]) -> Self { Self { matrix } }
}