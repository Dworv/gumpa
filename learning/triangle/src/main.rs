use std::sync::Arc;

use wgpu::{include_wgsl, Backends, Instance, InstanceDescriptor, PowerPreference, RenderPipelineDescriptor, RequestAdapterOptions};
use winit::{event::{Event, WindowEvent}, event_loop::EventLoop, window::Window};

#[pollster::main]
async fn main() {
    let ev = EventLoop::new().unwrap();
    let window = Arc::new(Window::new(&ev).unwrap());

    let size = window.inner_size();

    let instance = Instance::new(InstanceDescriptor {
        backends: Backends::all(),
        ..Default::default()
    });

    let surface = instance.create_surface(window.clone()).unwrap();

    let adapter = instance.request_adapter(&RequestAdapterOptions {
        power_preference: PowerPreference::default(),
        compatible_surface: None,
        force_fallback_adapter: false,
    }).await.unwrap();

    let (dev, queue) = adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
        },
        None,
    ).await.unwrap();

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
    surface.configure(&dev, &config);

    let module = dev.create_shader_module(include_wgsl!("triangle.wgsl"));

    let pipeline = dev.create_render_pipeline(&RenderPipelineDescriptor {
        label: None,
        layout: None,
        vertex: wgpu::VertexState {
            module: &module,
            entry_point: "vs_main",
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: &module,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        primitive: wgpu::PrimitiveState::default(),
        multiview: None
    });

    ev.run(move |event, elwt| {
        match event {
            Event::WindowEvent{ event: WindowEvent::RedrawRequested, .. } => {
                let output = surface.get_current_texture().unwrap();

                let view = output
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                let mut encoder = dev
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("Render Encoder"),
                    });

                {
                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Render Pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: 0.1,
                                    g: 0.2,
                                    b: 0.3,
                                    a: 1.0,
                                }),
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: None,
                        occlusion_query_set: None,
                        timestamp_writes: None,
                    });

                    render_pass.set_pipeline(&pipeline);
                    render_pass.draw(0..3, 0..1)
                }

                queue.submit(std::iter::once(encoder.finish()));
                output.present();
                window.request_redraw();
            }
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                println!("Closing");
                elwt.exit();
            }
            _ => {}
        }
    }).unwrap();
}
