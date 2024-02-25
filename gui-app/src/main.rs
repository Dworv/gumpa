#[pollster::main]
async fn main() {
    let mut app = gui::App::new();

    app.launch().await;
}
