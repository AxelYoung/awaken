mod camera;
mod context;
mod instance;
mod pixel_renderer;
mod scaling;
mod texture;
mod upscale_renderer;
mod vertex;

use wgpu::util::DeviceExt;

use std::iter;

use context::GraphicsContext;
use instance::Instance;
use pixel_renderer::PixelPipeline;
use upscale_renderer::UpscalePipeline;

pub struct Chroma {
  context : GraphicsContext,
  pub pixel_pipeline: PixelPipeline,
  upscale_pipeline: UpscalePipeline,
  depth_texture: texture::Texture
}

impl Chroma {
  pub async fn new(
    pixel_width: u32, pixel_height: u32, 
    window: &winit::window::Window
  ) -> Self {

    let context = pollster::block_on(GraphicsContext::new(window));

    let pixel_pipeline = PixelPipeline::new(
      &context,
      pixel_width, 
      pixel_height
    );

    let upscale_pipeline = UpscalePipeline::new(
      &context,
      &pixel_pipeline.texture_view,
      pixel_width,
      pixel_height
    );

    let depth_texture = texture::Texture::create_depth_texture(
      &context.device, 
      pixel_width, 
      pixel_height, 
      "depth_texture"
    );

    Chroma {
      context,
      pixel_pipeline,
      upscale_pipeline,
      depth_texture
    }
  }

  pub fn render(&mut self) {
    self.configure_instances();

    let mut encoder = self.context.device.create_command_encoder(
      &wgpu::CommandEncoderDescriptor {
        label: Some("render_encoder")
      }
    );

    self.pixel_pipeline.pass(&mut encoder, &self.depth_texture);

    let output = self.context.surface.get_current_texture().unwrap();
    
    let view = output.texture.create_view(
      &wgpu::TextureViewDescriptor::default()
    );

    self.upscale_pipeline.pass(&mut encoder, &view);

    self.context.queue.submit(iter::once(encoder.finish()));
    output.present();
  }

  pub fn configure_instances(&mut self) {
    let instance_data =
      self.pixel_pipeline.instances.iter()
      .map(Instance::to_raw).collect::<Vec<_>>();

    self.pixel_pipeline.instance_buffer = 
      self.context.device.create_buffer_init(
          &wgpu::util::BufferInitDescriptor {
            label: Some("instance_buffer"),
            contents: bytemuck::cast_slice(&instance_data),
            usage: wgpu::BufferUsages::VERTEX,
          }
      )
    ;
  }

  pub fn update_camera(&mut self, x: f32, y:f32) {
    self.pixel_pipeline.camera.x = x;
    self.pixel_pipeline.camera.y = y;
    self.pixel_pipeline.camera_raw = self.pixel_pipeline.camera.to_raw();
    self.context.queue.write_buffer(
      &self.pixel_pipeline.camera_buffer,
      0, 
      bytemuck::cast_slice(&[self.pixel_pipeline.camera_raw])
    );
  }
}