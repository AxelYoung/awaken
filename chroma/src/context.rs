pub struct GraphicsContext {
  pub surface: wgpu::Surface,
  pub device: wgpu::Device,
  pub queue: wgpu::Queue,
  pub config: wgpu::SurfaceConfiguration,
}

impl GraphicsContext {
  pub async fn new(window: &winit::window::Window) -> Self {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
      backends: wgpu::Backends::all(),
      dx12_shader_compiler: Default::default()
    });

    let surface = unsafe { instance.create_surface(&window) }.unwrap();

    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
      power_preference: wgpu::PowerPreference::HighPerformance,
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

    let window_size = window.inner_size();

    let config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: surface_format,
      width: window_size.width,
      height: window_size.height,
      present_mode: wgpu::PresentMode::Fifo,
      alpha_mode: surface_capabilities.alpha_modes[0],
      view_formats: vec![]
    };

    Self {
      surface,
      device,
      queue,
      config
    }
  }
}