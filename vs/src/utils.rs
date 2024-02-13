use wgpu::{Device, DeviceDescriptor, Features, Instance, Limits, Queue, RequestAdapterOptions};

pub struct Test {
    pub name: String,
    pub cpu: Box<dyn Fn()>,
    pub gpu: Box<dyn FnOnce(&mut (Device, Queue))>,
}

const ADAPTER_OPTIONS: RequestAdapterOptions = RequestAdapterOptions {
    power_preference: wgpu::PowerPreference::HighPerformance,
    force_fallback_adapter: false,
    compatible_surface: None,
};

pub async fn init_gpu() -> (Device, Queue) {
    let instance = Instance::default();
    let adapter = instance.request_adapter(&ADAPTER_OPTIONS).await.unwrap();
    adapter
        .request_device(&device_descriptor(), None)
        .await
        .unwrap()
}

fn device_descriptor() -> DeviceDescriptor<'static> {
    DeviceDescriptor {
        label: None,
        required_features: Features::empty(),
        required_limits: Limits::downlevel_defaults(),
    }
}
