use draws::run;

#[pollster::main]
async fn main() {
    env_logger::init();
    run().await;
}
