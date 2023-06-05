use wgpu::util::DeviceExt;

use super::camera::{Camera, CameraRaw};
use super::instance::{Instance, InstanceRaw};
use super::texture::Texture;

use super::vertex::Vertex;

const SPRITE_SHEET_WIDTH: u8 = 40;
const SPRITE_SHEET_HEIGHT: u8 = 40;

const SCREEN_WIDTH: u32 = 256;
const SCREEN_HEIGHT: u32= 224;

const VERTICES: &[Vertex] = &[
  Vertex {
    position: [
      -32.0 / SCREEN_WIDTH as f32 - 2.0, 
      32.0 / SCREEN_HEIGHT as f32 - 2.0, 0.0
    ],
    uvs: [0.0, 0.0],
  }, 
  Vertex {
    position: [
      -32.0 / SCREEN_WIDTH as f32 - 2.0, 
      -32.0 / SCREEN_HEIGHT as f32 - 2.0, 0.0
    ],
    uvs: [0.0, 1.0 / SPRITE_SHEET_HEIGHT as f32],
  }, 
  Vertex {
    position: [
      32.0 / SCREEN_WIDTH as f32 - 2.0, 
      -32.0 / SCREEN_HEIGHT as f32 - 2.0, 0.0
    ],
    uvs: [1.0 / SPRITE_SHEET_WIDTH as f32, 1.0 / SPRITE_SHEET_HEIGHT as f32],
  }, 
  Vertex {
    position: [
      32.0 / SCREEN_WIDTH as f32 - 2.0, 
      32.0 / SCREEN_HEIGHT as f32 - 2.0, 0.0
    ],
    uvs: [1.0 / SPRITE_SHEET_WIDTH as f32, 0.0],
  }, 
];

const INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];

pub struct PixelRenderer {
  pub pipeline: wgpu::RenderPipeline, 

  pub vertex_buffer: wgpu::Buffer, 
  pub index_buffer: wgpu::Buffer, 
  pub index_count: u32, 

  pub diffuse_bind_group: wgpu::BindGroup, 
  texture: wgpu::Texture, 
  pub texture_view: wgpu::TextureView, 

  pub instance_buffer: wgpu::Buffer, 
  pub instances: Vec<Instance>, 

  pub camera: Camera, 
  pub camera_raw: CameraRaw, 
  pub camera_buffer: wgpu::Buffer, 
  pub camera_bind_group: wgpu::BindGroup
}

impl PixelRenderer {
  pub fn new(
    surface: &wgpu::Surface, 
    adapter: &wgpu::Adapter, 
    surface_format: &wgpu::TextureFormat,
    surface_capabilities: &wgpu::SurfaceCapabilities, 
    queue: &wgpu::Queue, 
    device: &wgpu::Device, 
    width: u32, height: u32
  ) -> Self {

    let texture_description = wgpu::TextureDescriptor {
      label: None,
      size: wgpu::Extent3d {
        width, height,
        depth_or_array_layers: 1
      },
      mip_level_count: 1,
      sample_count: 1,
      dimension: wgpu::TextureDimension::D2,
      format: *surface_format,
      usage: wgpu::TextureUsages::TEXTURE_BINDING |
          wgpu::TextureUsages::RENDER_ATTACHMENT,
      view_formats: &[]
    };

    let texture = device.create_texture(&texture_description);
    let texture_view = texture.create_view(&Default::default());

    let diffuse_bytes = include_bytes!("../img/sprite_sheet.png");
    let diffuse_texture = Texture::from_bytes(
      &device, &queue, diffuse_bytes, "sprite_sheet"
    ).unwrap();

    let texture_layout_entry = wgpu::BindGroupLayoutEntry {
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
    };

    let sampler_layout_entry = wgpu::BindGroupLayoutEntry {
      binding: 1,
      visibility: wgpu::ShaderStages::FRAGMENT,
      ty: wgpu::BindingType::Sampler(
        wgpu::SamplerBindingType::Filtering
      ),
      count: None
    };

    let texture_bind_group_layout = device.create_bind_group_layout(
      &wgpu::BindGroupLayoutDescriptor {
        label: Some("pixel_renderer_texture_bind_group_layout"),
        entries: &[
          texture_layout_entry,
          sampler_layout_entry
        ]
      }
    );

    let diffuse_bind_group = 
      device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("pixel_renderer_diffuse_bind_group"),
        layout: &texture_bind_group_layout,
        entries: &[
          wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::TextureView(
              &diffuse_texture.view
            )
          },
          wgpu::BindGroupEntry {
            binding: 1,
            resource: wgpu::BindingResource::Sampler(
              &diffuse_texture.sampler
            )
          }
        ]
      }
    );

    let camera = Camera::new();
    let camera_raw = camera.to_raw();

    let camera_buffer = 
      device.create_buffer_init(
      &wgpu::util::BufferInitDescriptor {
        label: Some("camera_buffer"),
        contents: bytemuck::cast_slice(&[camera_raw]),
        usage: wgpu::BufferUsages::UNIFORM | 
            wgpu::BufferUsages::COPY_DST
      }
    );

    let camera_bind_group_layout =
      device.create_bind_group_layout(
      &wgpu::BindGroupLayoutDescriptor {
        entries: &[
          wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
              ty: wgpu::BufferBindingType::Uniform,
              has_dynamic_offset: false,
              min_binding_size: None,
            },
            count: None,
          }
        ],
        label: Some("camera_bind_group_layout"),
      }
    );

    let camera_bind_group = 
      device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("camera_bind_group"),
        layout: &camera_bind_group_layout,
        entries: &[
          wgpu::BindGroupEntry {
            binding: 0,
            resource: camera_buffer.as_entire_binding()
          }
        ]
      }
    );

    let shader = device.create_shader_module(
      wgpu::ShaderModuleDescriptor {
        label: Some("pixel_renderer_shader"),
        source: wgpu::ShaderSource::Wgsl(
          include_str!("../shaders/shader.wgsl").into()
        )
      }
    );

    let instances = vec![];

    let instance_data = 
      instances.iter().map(Instance::to_raw).collect::<Vec<_>>();

    let instance_buffer = device.create_buffer_init(
      &wgpu::util::BufferInitDescriptor {
        label: Some("pixel_renderer_instance_buffer"),
        contents: bytemuck::cast_slice(&instance_data),
        usage: wgpu::BufferUsages::VERTEX
      }
    );

    let render_pipeline_layout = device.create_pipeline_layout(
      &wgpu::PipelineLayoutDescriptor {
        label: Some("pixel_renderer_pipeline_layout"),
        bind_group_layouts: &[
          &texture_bind_group_layout,
          &camera_bind_group_layout,
        ],
        push_constant_ranges: &[]
      }
    );

    let vertex_state = wgpu::VertexState {
      module: &shader,
      entry_point: "vs_main",
      buffers: &[Vertex::layout(), InstanceRaw::layout()]
    };

    let fragment_state = wgpu::FragmentState {
      module: &shader,
      entry_point: "fs_main",
      targets: &[Some(wgpu::ColorTargetState {
        format: *surface_format,
        blend: Some(wgpu::BlendState{
          color: wgpu::BlendComponent{
            src_factor: wgpu::BlendFactor::SrcAlpha,
            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
            operation: wgpu::BlendOperation::Add,},
          alpha: wgpu::BlendComponent::OVER
        }),

        write_mask: wgpu::ColorWrites::ALL,
      })]
    };

    let primitive_state = wgpu::PrimitiveState {
      topology: wgpu::PrimitiveTopology::TriangleList,
      strip_index_format: None,
      front_face: wgpu::FrontFace::Ccw,
      cull_mode: Some(wgpu::Face::Back),
      polygon_mode: wgpu::PolygonMode::Fill,
      unclipped_depth: false,
      conservative: false
    };

    let depth_stencil_state = wgpu::DepthStencilState {
      format: Texture::DEPTH_FORMAT,
      depth_write_enabled: true,
      depth_compare: wgpu::CompareFunction::Less,
      stencil: wgpu::StencilState::default(),
      bias: wgpu::DepthBiasState::default(),
    };

    let pipeline = device.create_render_pipeline(
      &wgpu::RenderPipelineDescriptor {
        label: Some("pixel_render_pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: vertex_state,
        fragment: Some(fragment_state),
        primitive: primitive_state,
        depth_stencil: Some(depth_stencil_state),
        multisample: wgpu::MultisampleState {
          count: 1,
          mask: !0,
          alpha_to_coverage_enabled: true
        },
        multiview: None
      }
    );

    let vertex_buffer = device.create_buffer_init(
      &wgpu::util::BufferInitDescriptor {
        label: Some("pixel_vertex_buffer"),
        contents: bytemuck::cast_slice(VERTICES),
        usage: wgpu::BufferUsages::VERTEX,
      }
    );

    let index_buffer = device.create_buffer_init(
      &wgpu::util::BufferInitDescriptor {
        label: Some("pixel_index_buffer"),
        contents: bytemuck::cast_slice(INDICES),
        usage: wgpu::BufferUsages::INDEX,
      }
    );

    let index_count = INDICES.len() as u32;

    Self {
      pipeline,

      vertex_buffer,
      index_buffer,
      index_count,

      diffuse_bind_group,
      texture,
      texture_view,

      instance_buffer,
      instances,
      
      camera,
      camera_raw,
      camera_buffer,
      camera_bind_group
    }
  }
  
  pub fn add_tile(
    &mut self, 
    pos_x: f32, pos_y: f32, pos_z: f32, 
    index_x: u32, index_y: u32
  ) {
    self.instances.push(
      Instance { 
        position_offset: cgmath::Vector3 {
          x: pos_x * 4.0 / SCREEN_WIDTH as f32,
          y: pos_y * 4.0 / SCREEN_HEIGHT as f32,
          z: pos_z
        },
        uv_offset: cgmath::Vector2 {
          x: index_x as f32 / SPRITE_SHEET_WIDTH as f32,
          y: index_y as f32 / SPRITE_SHEET_HEIGHT as f32
        }
      }
    );
  }

  pub fn clear(&mut self) {
    self.instances = vec![];
  }
}