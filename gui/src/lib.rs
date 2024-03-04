use std::sync::Arc;

use winit::{event_loop::EventLoop, window::Window};

mod element;
mod math;
mod runtime;

pub use element::Element;
pub use math::{Vec2, Colour};
use runtime::AppRuntime;

pub struct App {
    elements: Vec<Element>,
}

impl App {
    pub fn new(elements: Vec<Element>) -> Self {
        App { elements }
    }

    pub async fn launch(&mut self) {
        let event_loop = EventLoop::new().unwrap();
        let window = Arc::new(Window::new(&event_loop).unwrap());
        let mut runtime = AppRuntime::init(window.clone(), &self.elements).await;

        runtime.run(event_loop);
    }
}
