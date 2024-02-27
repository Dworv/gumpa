use std::sync::Arc;

use winit::{event_loop::EventLoop, window::Window};

mod runtime;
use runtime::AppRuntime;

pub struct App;

impl App {
    pub fn new() -> Self {
        App
    }

    pub async fn launch(&mut self) {
        let event_loop = EventLoop::new().unwrap();
        let window = Arc::new(Window::new(&event_loop).unwrap());
        let mut runtime = AppRuntime::init(window.clone()).await;

        runtime.run(event_loop);
    }
}

