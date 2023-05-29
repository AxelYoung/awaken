pub struct Instance {
   pub position_offset: cgmath::Vector3<f32>,
   pub uv_offset: cgmath::Vector2<f32> 
}

impl Instance {
   pub fn to_raw(&self) -> InstanceRaw {
      InstanceRaw::new(self.position_offset, self.uv_offset)
   }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
   model: [f32; 5],
}

impl InstanceRaw {
   pub fn new(
      pos_offset: cgmath::Vector3<f32>, uv_offset: cgmath::Vector2<f32>
   ) -> Self {
      InstanceRaw {
         model: [
            pos_offset.x, pos_offset.y, pos_offset.z, 
            uv_offset.x, uv_offset.y
         ]
      }
   }
   
   pub fn layout<'a>() -> wgpu::VertexBufferLayout<'a> {
      use std::mem;
      wgpu::VertexBufferLayout {
         array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
         step_mode: wgpu::VertexStepMode::Instance,
         attributes: &[
            // Position offset attribute 
            wgpu::VertexAttribute {
               offset: 0,
               shader_location: 5,
               format: wgpu::VertexFormat::Float32x3,
            },
            // UV offset attribute
            wgpu::VertexAttribute {
               offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
               shader_location: 6,
               format: wgpu::VertexFormat::Float32x2,
            },
         ],
      }
   }
}
