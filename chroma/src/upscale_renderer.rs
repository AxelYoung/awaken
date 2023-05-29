use wgpu::util::DeviceExt;

use super::scaling::ScalingMatrix;

pub struct UpscaleRenderer {
   config: wgpu::SurfaceConfiguration,
   pub pipeline: wgpu::RenderPipeline, 
   pub vertex_buffer: wgpu::Buffer, 
   pub diffuse_bind_group: wgpu::BindGroup, 
   pub clip_rect: (u32, u32, u32, u32)
}

impl UpscaleRenderer {
   pub fn new(
      surface: &wgpu::Surface, 
      adapter: &wgpu::Adapter, 
      device: &wgpu::Device, 
      window_size: winit::dpi::PhysicalSize<u32>,
      surface_format: &wgpu::TextureFormat,
      surface_capabilities: &wgpu::SurfaceCapabilities, 
      texture_view: &wgpu::TextureView, 
      pixel_width: u32, pixel_height: u32
   ) -> Self {

      let config = wgpu::SurfaceConfiguration {
         usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
         format: *surface_format,
         width: window_size.width,
         height: window_size.height,
         present_mode: wgpu::PresentMode::Fifo,
         alpha_mode: surface_capabilities.alpha_modes[0],
         view_formats: vec![]
      };

      let shader = wgpu::include_wgsl!("../shaders/scale.wgsl");
      let module = device.create_shader_module(shader);

      let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
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
      let vertex_buffer = device.create_buffer_init(
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
         (window_size.width as f32, window_size.height as f32)
      );

      let transform_bytes = matrix.as_bytes();

      let uniform_buffer = device.create_buffer_init(
         &wgpu::util::BufferInitDescriptor {
            label: Some("upscale_renderer_bind_matrix_uniform_buffer"),
            contents: transform_bytes,
            usage: wgpu::BufferUsages::UNIFORM | 
                  wgpu::BufferUsages::COPY_DST
         }
      );

      let bind_group_layout = device.create_bind_group_layout(
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

      let diffuse_bind_group = device.create_bind_group(
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

      let pipeline_layout = device.create_pipeline_layout(
         &wgpu::PipelineLayoutDescriptor {
            label: Some("upscale_renderer_pipeline_layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[]
         }
      );

      let pipeline = device.create_render_pipeline(
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
                  format: *surface_format,
                  blend: Some(wgpu::BlendState::REPLACE),
                  write_mask: wgpu::ColorWrites::ALL
               })]
            }),
            multiview: None
         }
      );

      let clip_rect = matrix.clip_rect();

      surface.configure(&device, &config);

      Self {
         config,
         pipeline,
         vertex_buffer,
         diffuse_bind_group,
         clip_rect
      }
   }
} 