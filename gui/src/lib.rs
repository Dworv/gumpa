use std::sync::Arc;

use winit::{dpi::PhysicalSize, event_loop::EventLoop, window::Window};



pub struct App {
    runtime: Option<AppRuntime>
}

struct AppRuntime {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
}

impl App {
    pub fn new() -> Self {
        App { runtime: None }
    }

    pub async fn launch(&mut self) {
        let event_loop = EventLoop::new().unwrap();
        let window = Arc::new(Window::new(&event_loop).unwrap());
        let (device, queue, surface, size, config) = init_gpu_and_surface(window.clone()).await;
        self.runtime = Some(AppRuntime {
            window,
            surface,
            device,
            queue,
            config,
            size
        });
    }

}

async fn init_gpu_and_surface(window: Arc<Window>) -> (wgpu::Device, wgpu::Queue, wgpu::Surface<'static>, PhysicalSize<u32>, wgpu::SurfaceConfiguration) {
    let size = window.inner_size();

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });

    let surface = instance.create_surface(window).unwrap();

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .unwrap();

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
            },
            None,
        )
        .await
        .unwrap();

    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps
        .formats
        .iter()
        .copied()
        .filter(|f| f.is_srgb())
        .next()
        .unwrap_or(surface_caps.formats[0]);
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: surface_caps.present_modes[0],
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };
    surface.configure(&device, &config);

    (device, queue, surface, size, config)
}