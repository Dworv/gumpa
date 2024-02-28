use gui::{Element, Vec2};

#[pollster::main]
async fn main() {
    let mut app = gui::App::new(vec![Element::new(Vec2::new(50., 50.), Vec2::new(100., 100.))]);

    app.launch().await;
}
