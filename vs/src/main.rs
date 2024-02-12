use std::time::Instant;

mod utils;

#[pollster::main]
async fn main() {
    println!("Running CPU vs GPU benchmarks");

    println!("Initiating GPU...");
    let (device, queue) = utils::init_gpu().await;

    println!("Running tests");
    let tests: Vec<utils::Test> = vec![];

    for test in tests {
        let cpu_time = {
            let now = Instant::now();
            (test.cpu)();
            now.elapsed()
        };

        let (gpu_fn, gpu_setup_time) = {
            let now = Instant::now();
            let gpu_fn = (test.gpu)();
            let gpu_setup_time = now.elapsed();
            (gpu_fn, gpu_setup_time)
        };

        let gpu_time = {
            let now = Instant::now();
            gpu_fn();
            now.elapsed()
        };

        println!(
            "CPU -> {:>8.3}ms VS {:>8.3}ms <- GPU + {:>8.3}ms setting up GPU on test {}",
            cpu_time.as_secs_f64() / 1000.,
            gpu_time.as_secs_f64() / 1000.,
            gpu_setup_time.as_secs_f64() / 1000.,
            test.name
        );
    }

    println!("Done!");
}
