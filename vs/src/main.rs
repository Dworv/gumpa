use std::time::Instant;

mod utils;
mod vecadd;

#[pollster::main]
async fn main() {
    println!("Running CPU vs GPU benchmarks");

    println!("Initiating GPU...");
    let mut gpu_tools = utils::init_gpu().await;

    println!("Running tests...");
    let tests: Vec<utils::Test> = vec![vecadd::vecadd()];

    for test in tests {
        let cpu_time = {
            let now = Instant::now();
            (test.cpu)();
            now.elapsed()
        };

        let gpu_time = {
            let now = Instant::now();
            (test.gpu)(&mut gpu_tools);
            now.elapsed()
        };

        println!(
            "CPU -> {:>10.8}ms VS {:>10.8}ms <- GPU on test {}",
            cpu_time.as_secs_f64() * 1000.,
            gpu_time.as_secs_f64() * 1000.,
            test.name
        );
    }

    println!("Done!");
}
