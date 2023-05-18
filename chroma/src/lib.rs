use builder::ChromaBuilder;
use renderers::ScalingRenderer;

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

use winit::{window::Window, dpi::PhysicalSize};

mod renderers;
mod builder;

pub const SCALE: u32 = 4;

pub const TILE_SIZE: u32 = 8;
pub const TILE_DATA_SIZE: usize = (TILE_SIZE * TILE_SIZE * 4) as usize;

pub struct ChromaContext {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    surface: wgpu::Surface,
    pub texture: wgpu::Texture,
    pub texture_extent: wgpu::Extent3d,
    pub scaling_renderer: ScalingRenderer
}

pub struct Chroma {
    context: ChromaContext,
    surface_size: PhysicalSize<u32>,
    width: u16,
    height: u16,
    alpha_mode: wgpu::CompositeAlphaMode,
    adapter: wgpu::Adapter,
    pixels: Vec<u8>,
    scaling_matrix_inverse: ultraviolet::Mat4
}

impl Chroma {
    pub fn new(width: u16, height: u16, window: &Window) -> Self {
        ChromaBuilder::new(width, height, window).build()
    }

    pub fn reconfigure_surface(&self) {
        self.context.surface.configure(
            &self.context.device, 
            &wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                width: self.surface_size.width as u32,
                height: self.surface_size.height as u32,
                present_mode: wgpu::PresentMode::Immediate,
                alpha_mode: self.alpha_mode,
                view_formats: vec![]
            });
    }

    pub fn clear(&mut self) {
        let mut pixels: Vec<u8> = Vec::with_capacity(self.pixels.len());
        pixels.resize_with(self.pixels.len(), Default::default);
        self.pixels = pixels;
    }

    pub fn render(&self) {
        self.render_with(|encoder, render_target, context| {
            context.scaling_renderer.render(encoder, render_target);
        });
    }

    pub fn render_with<F>(&self, render_function: F) 
        where F: FnOnce(
            &mut wgpu::CommandEncoder,
            &wgpu::TextureView,
            &ChromaContext
        ) {
        let frame = self.context.surface.get_current_texture().or_else(|_| {

            self.context.surface.get_current_texture()
        }).unwrap();

        let mut encoder = 
            self.context.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("command_encoder")
                });

        let bytes_per_row = (self.context.texture_extent.width as f32 * 4.0) as u32;
    
        self.context.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.context.texture,
                mip_level: 0,
                origin: wgpu::Origin3d { x: 0, y: 0, z: 0 },
                aspect: wgpu::TextureAspect::All
            }, 
            &self.pixels, 
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(bytes_per_row as u32),
                rows_per_image: Some(self.context.texture_extent.height as u32)
            }, 
            self.context.texture_extent
        );

        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
    
        (render_function)(&mut encoder, &view, &self.context);

        self.context.queue.submit(Some(encoder.finish()));
        frame.present();
    }

    pub fn frame_mut(&mut self) -> &mut [u8] {
        &mut self.pixels
    }

    pub fn frame(&self) -> &[u8] {
        &self.pixels
    }
    
    /*
    pub fn draw_tile(&mut self, sprite: &[u8], pos_x: u32, pos_y: u32) {
        self.draw_sprite(sprite, pos_x * TILE_SIZE, pos_y * TILE_SIZE);
    }
    */

    pub fn draw_sprite_from_sheet(&mut self, sprite: &[u8], index: u32, pos_x: u32, pos_y: u32, flip_x: bool) {
        for y in 0..TILE_SIZE {
            for x in 0..TILE_SIZE {
                let pixel_index = (((y * TILE_SIZE) + x) * 4) + index * TILE_DATA_SIZE as u32;
                self.draw_pixel(&sprite[pixel_index as usize..(pixel_index + 4) as usize], if flip_x {TILE_SIZE - x - 1} else {x} + pos_x, y + pos_y)
            }
        }
    }
    
    pub fn draw_sprite(&mut self, sprite: &[u8], pos_x: u32, pos_y: u32, flip_x: bool) {
        for y in 0..TILE_SIZE {
            for x in 0..TILE_SIZE {
                let index = ((y * TILE_SIZE) + x) * 4;
                self.draw_pixel(&sprite[index as usize..(index + 4) as usize], if flip_x {TILE_SIZE - x - 1} else {x} + pos_x, y + pos_y)
            }
        }
    }
    
    pub fn draw_pixel(&mut self, pixel: &[u8], x: u32, y: u32) {
        let index = ((y * self.width as u32) + x) * 4;
        if pixel[3] == 0 { return }
        for offset in 0..4 {
            self.pixels[(index + offset) as usize] = pixel[offset as usize];
        }
    }
    
    pub fn get_pixel(&self, x: u32, y: u32) -> [u8; 4] {
        let index = ((y * self.width as u32) + x) * 4;
        let mut pixel = [0 as u8; 4];
        for offset in 0..4 {
            pixel[offset] = self.pixels[(index as usize + offset) as usize];
        }
        pixel
    }
}