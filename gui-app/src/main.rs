use gui::{Element, Vec2, Colour};

#[pollster::main]
async fn main() {
    let mut app = gui::App::new((0..10).map(|i| {
        (0..5).map(move |j| {
            Element::new(
                Vec2::new(20. + (i as f32 * 80.), 20. + (j as f32 * 80.)),
                Vec2::new(60.0, 60.0),
                Colour::new(1.0 - i as f32 * 0.1, 1.0 - j as f32 * 0.2, i as f32 * 0.05 + j as f32 * 0.1, 1.0)
            )
        })
    }).flatten().collect());

    app.launch().await;
}
