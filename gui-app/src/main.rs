use gui::{Element, Vec2};

#[pollster::main]
async fn main() {
    let mut app = gui::App::new(vec![Element::new(Vec2::new(0., 0.5), Vec2::new(0.25, 0.25))]);

    app.launch().await;
}
