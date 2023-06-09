use wgpu::util::DeviceExt;

use crate::context::GraphicsContext;

use super::scaling::ScalingMatrix;

pub struct UpscalePipeline {
  pub pipeline: wgpu::RenderPipeline, 
  pub vertex_buffer: wgpu::Buffer, 
  pub diffuse_bind_group: wgpu::BindGroup, 
  pub clip_rect: (u32, u32, u32, u32)
}

impl UpscalePipeline {
  pub fn new(
    context: &GraphicsContext,
    texture_view: &wgpu::TextureView, 
    pixel_width: u32, pixel_height: u32
  ) -> Self {

    let shader = wgpu::include_wgsl!("../shaders/scale.wgsl");
    let module = context.device.create_shader_module(shader);

    let sampler = context.device.create_sampler(&wgpu::SamplerDescriptor {
      label: Some("scaling_renderer"),
      address_mode_u: wgpu::AddressMode::ClampToEdge,
      address_mode_v: wgpu::AddressMode::ClampToEdge,
      address_mode_w: wgpu::AddressMode::ClampToEdge,
      mag_filter: wgpu::FilterMode::Nearest,
      min_filter: wgpu::FilterMode::Nearest,
      mipmap_filter: wgpu::FilterMode::Nearest,
      lod_min_clamp: 0.0,
      lod_max_clamp: 1.0,
      compare: None,
      anisotropy_clamp: 1,
      border_color: None
    });

    let vertex_data: [[f32; 2]; 3] = [
      [-1.0, -1.0],
      [3.0, -1.0],
      [-1.0, 3.0]
    ];

    let vertex_slice = bytemuck::cast_slice(&vertex_data);
    let vertex_buffer = context.device.create_buffer_init(
      &wgpu::util::BufferInitDescriptor {
        label: Some("upscale_renderer_vertex_buffer"),
        contents: vertex_slice,
        usage: wgpu::BufferUsages::VERTEX
      }
    );
    let vertex_buffer_layout = wgpu::VertexBufferLayout {
      array_stride: vertex_slice.len() as u64 / vertex_data.len() as u64,
      step_mode: wgpu::VertexStepMode::Vertex,
      attributes: &[wgpu::VertexAttribute {
        format: wgpu::VertexFormat::Float32x2,
        offset: 0,
        shader_location: 0
      }]
    };

    let matrix = ScalingMatrix::new(
      (pixel_width as f32, pixel_height as f32),
      (context.config.width as f32, context.config.height as f32)
    );

    let transform_bytes = matrix.as_bytes();

    let uniform_buffer = context.device.create_buffer_init(
      &wgpu::util::BufferInitDescriptor {
        label: Some("upscale_renderer_bind_matrix_uniform_buffer"),
        contents: transform_bytes,
        usage: wgpu::BufferUsages::UNIFORM | 
            wgpu::BufferUsages::COPY_DST
      }
    );

    let bind_group_layout = context.device.create_bind_group_layout(
      &wgpu::BindGroupLayoutDescriptor {
        label: Some("upscale_renderer_bind_group_layout"),
        entries: &[
          wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture { 
              sample_type: wgpu::TextureSampleType::Float { 
                filterable: true 
              },
              view_dimension: wgpu::TextureViewDimension::D2, 
              multisampled: false 
            },
            count: None
          },
          wgpu::BindGroupLayoutEntry {
            binding: 1,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Sampler(
              wgpu::SamplerBindingType::Filtering
            ),
            count: None
          },
          wgpu::BindGroupLayoutEntry {
            binding: 2,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer { 
              ty: wgpu::BufferBindingType::Uniform, 
              has_dynamic_offset: false, 
              min_binding_size: wgpu::BufferSize::new(
                transform_bytes.len() as u64
              ) 
            },
            count: None
          }
        ]
      }
    );

    let diffuse_bind_group = context.device.create_bind_group(
      &wgpu::BindGroupDescriptor {
        label: Some("upscale_renderer_bind_group"),
        layout: &bind_group_layout,
        entries: &[
          wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::TextureView(&texture_view)
          },
          wgpu::BindGroupEntry {
            binding: 1,
            resource: wgpu::BindingResource::Sampler(&sampler)
          },
          wgpu::BindGroupEntry {
            binding: 2,
            resource: uniform_buffer.as_entire_binding()
          }
        ]
      }
    );

    let pipeline_layout = context.device.create_pipeline_layout(
      &wgpu::PipelineLayoutDescriptor {
        label: Some("upscale_renderer_pipeline_layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[]
      }
    );

    let pipeline = context.device.create_render_pipeline(
      &wgpu::RenderPipelineDescriptor {
        label: Some("upscale_renderer_pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState { 
          module: &module, 
          entry_point: "vs_main", 
          buffers: &[vertex_buffer_layout]
        },
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        fragment: Some(wgpu::FragmentState {
          module: &module,
          entry_point: "fs_main",
          targets: &[Some(wgpu::ColorTargetState {
            format: context.config.format,
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::ALL
          })]
        }),
        multiview: None
      }
    );

    let clip_rect = matrix.clip_rect();

    context.surface.configure(&context.device, &context.config);

    Self {
      pipeline,
      vertex_buffer,
      diffuse_bind_group,
      clip_rect
    }
  }

  pub fn pass(
    &mut self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView
  ) {
    let mut upscale_pass = encoder.begin_render_pass(
      &wgpu::RenderPassDescriptor {
        label: Some("upscale_pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
          view: view,
          resolve_target: None,
          ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
            store: true
          }
        })],
        depth_stencil_attachment: None
      }
    );
    
    upscale_pass.set_pipeline(&self.pipeline);

    upscale_pass.set_bind_group(
      0, &self.diffuse_bind_group, &[]
    );

    upscale_pass.set_vertex_buffer(
      0, self.vertex_buffer.slice(..)
    );

    upscale_pass.set_scissor_rect(
      self.clip_rect.0, 
      self.clip_rect.1, 
      self.clip_rect.2,
      self.clip_rect.3
    );

    upscale_pass.draw(0..3, 0..1);
  }
} 