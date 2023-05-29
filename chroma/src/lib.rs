mod camera;
mod instance;
mod pixel_renderer;
mod scaling;
mod texture;
mod upscale_renderer;
mod vertex;

use std::iter;

use instance::Instance;
use pixel_renderer::PixelRenderer;
use upscale_renderer::UpscaleRenderer;
use wgpu::util::DeviceExt;

pub struct Chroma {
   surface: wgpu::Surface,
   device: wgpu::Device,
   queue: wgpu::Queue,
   pub pixel_renderer: PixelRenderer,
   upscale_renderer: UpscaleRenderer,
   depth_texture: texture::Texture
}

impl Chroma {
   pub async fn new(
      pixel_width: u32, pixel_height: u32, 
      window: &winit::window::Window
   ) -> Self {
      let window_size = window.inner_size();

      let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
         backends: wgpu::Backends::all(),
         dx12_shader_compiler: Default::default()
      });

      let surface = unsafe { instance.create_surface(&window) }.unwrap();

      let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
         power_preference: wgpu::PowerPreference::default(),
         compatible_surface: Some(&surface),
         force_fallback_adapter: false
      }).await.unwrap();

      let surface_capabilities = surface.get_capabilities(&adapter);

      let surface_format = surface_capabilities.formats.iter().copied()
         .filter(|format| format.is_srgb()).next()
         .unwrap_or(surface_capabilities.formats[0]);

      let limits = 
         if cfg!(target_arch = "wasm32") {
            wgpu::Limits::downlevel_webgl2_defaults()
         } else {
            wgpu::Limits::default()
         };
      
      let (device, queue) = adapter.request_device(
         &wgpu::DeviceDescriptor {
            label: None,
            features: wgpu::Features::empty(),
            limits
         },
         None
      ).await.unwrap();
      

      let pixel_renderer = PixelRenderer::new(
         &surface, 
         &adapter, 
         &surface_format, 
         &surface_capabilities, 
         &queue, 
         &device, 
         pixel_width, 
         pixel_height
      );

      let upscale_renderer = UpscaleRenderer::new(
         &surface,
         &adapter,
         &device,
         window_size,
         &surface_format,
         &surface_capabilities,
         &pixel_renderer.texture_view,
         pixel_width,
         pixel_height
      );

      let depth_texture = texture::Texture::create_depth_texture(
         &device, 
         pixel_width, 
         pixel_height, 
         "depth_texture"
      );

      Chroma {
         surface,
         device,
         queue,
         pixel_renderer,
         upscale_renderer,
         depth_texture
      }
   }

   pub fn render(&mut self) {
      self.configure_instances();

      let mut encoder = self.device.create_command_encoder(
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

      let output = self.surface.get_current_texture().unwrap();
      
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

      self.queue.submit(iter::once(encoder.finish()));
      output.present();
   }

   pub fn configure_instances(&mut self) {
      let instance_data =
         self.pixel_renderer.instances.iter()
         .map(Instance::to_raw).collect::<Vec<_>>();

      self.pixel_renderer.instance_buffer = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
               label: Some("instance_buffer"),
               contents: bytemuck::cast_slice(&instance_data),
               usage: wgpu::BufferUsages::VERTEX,
            }
         );
   }

   pub fn update_camera(&mut self, x: f32, y:f32) {
      self.pixel_renderer.camera.x = x;
      self.pixel_renderer.camera.y = y;
      self.pixel_renderer.camera_raw = self.pixel_renderer.camera.to_raw();
      self.queue.write_buffer(
         &self.pixel_renderer.camera_buffer,
         0, 
         bytemuck::cast_slice(&[self.pixel_renderer.camera_raw])
      );
   }
}