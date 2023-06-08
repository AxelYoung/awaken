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
use pixel_renderer::PixelRenderer;
use upscale_renderer::UpscaleRenderer;

pub struct Chroma {
  context : GraphicsContext,
  pub pixel_renderer: PixelRenderer,
  upscale_renderer: UpscaleRenderer,
  depth_texture: texture::Texture
}

impl Chroma {
  pub async fn new(
    pixel_width: u32, pixel_height: u32, 
    window: &winit::window::Window
  ) -> Self {

    let context = pollster::block_on(GraphicsContext::new(window));

    let pixel_renderer = PixelRenderer::new(
      &context,
      pixel_width, 
      pixel_height
    );

    let upscale_renderer = UpscaleRenderer::new(
      &context,
      &pixel_renderer.texture_view,
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
      pixel_renderer,
      upscale_renderer,
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

    {
      let mut render_pass = encoder.begin_render_pass(
        &wgpu::RenderPassDescriptor {
          label: Some("render_pass"),
          color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &self.pixel_renderer.texture_view,
            resolve_target: None,
            ops: wgpu::Operations {
              load: wgpu::LoadOp::Clear(wgpu::Color {
                r: 0.0,
                g: 1.0,
                b: 0.0,
                a: 1.0,
              }),
              store: true,
            },
          })],
          depth_stencil_attachment: Some(
              wgpu::RenderPassDepthStencilAttachment {
              view: &self.depth_texture.view,
              depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: true,
              }),
              stencil_ops: None,
            }
          ),
        }
      );

      render_pass.set_pipeline(&self.pixel_renderer.pipeline
      );
      render_pass.set_bind_group(
        0, &self.pixel_renderer.diffuse_bind_group, &[]
      );
      render_pass.set_bind_group(
        1, &self.pixel_renderer.camera_bind_group, &[]
      );

      render_pass.set_vertex_buffer(
        0, self.pixel_renderer.vertex_buffer.slice(..)
      );
      render_pass.set_index_buffer(
        self.pixel_renderer.index_buffer.slice(..), 
        wgpu::IndexFormat::Uint16
      );

      render_pass.set_vertex_buffer(
        1, self.pixel_renderer.instance_buffer.slice(..)
      );
      render_pass.set_index_buffer(
        self.pixel_renderer.index_buffer.slice(..), 
        wgpu::IndexFormat::Uint16
      );

      render_pass.draw_indexed(
        0..self.pixel_renderer.index_count, 
        0, 
        0..self.pixel_renderer.instances.len() as u32
      );
    }

    let output = self.context.surface.get_current_texture().unwrap();
    
    let view = output.texture.create_view(
      &wgpu::TextureViewDescriptor::default()
    );

    {
      let mut render_pass = encoder.begin_render_pass(
        &wgpu::RenderPassDescriptor {
          label: Some("scaling_renderer_render_pass"),
          color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &view,
            resolve_target: None,
            ops: wgpu::Operations {
              load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
              store: true
            }
          })],
          depth_stencil_attachment: None
        }
      );
      
      render_pass.set_pipeline(&self.upscale_renderer.pipeline);

      render_pass.set_bind_group(
        0, &self.upscale_renderer.diffuse_bind_group, &[]
      );

      render_pass.set_vertex_buffer(
        0, self.upscale_renderer.vertex_buffer.slice(..)
      );

      render_pass.set_scissor_rect(
        self.upscale_renderer.clip_rect.0, 
        self.upscale_renderer.clip_rect.1, 
        self.upscale_renderer.clip_rect.2,
        self.upscale_renderer.clip_rect.3
      );

      render_pass.draw(0..3, 0..1);
    }

    self.context.queue.submit(iter::once(encoder.finish()));
    output.present();
  }

  pub fn configure_instances(&mut self) {
    let instance_data =
      self.pixel_renderer.instances.iter()
      .map(Instance::to_raw).collect::<Vec<_>>();

    self.pixel_renderer.instance_buffer = 
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
    self.pixel_renderer.camera.x = x;
    self.pixel_renderer.camera.y = y;
    self.pixel_renderer.camera_raw = self.pixel_renderer.camera.to_raw();
    self.context.queue.write_buffer(
      &self.pixel_renderer.camera_buffer,
      0, 
      bytemuck::cast_slice(&[self.pixel_renderer.camera_raw])
    );
  }
}