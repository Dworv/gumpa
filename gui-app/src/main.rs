use gui::{Element, Vec2, Colour};

#[pollster::main]
async fn main() {
    let mut app = gui::App::new(vec![Element::new(Vec2::new(0., 0.5), Vec2::new(400., 400.), Colour { r: 1., g: 0., b: 1., a:1. })]);

    app.launch().await;
}
