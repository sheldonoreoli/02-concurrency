use std::thread;

use anyhow::Result;
use rand::Rng;

use concurrency::Metrics;

const N: usize = 2;
const M: usize = 4;

fn main() -> Result<()> {
    let metrics = Metrics::new();

    for idx in 0..N {
        task_worker(idx, metrics.clone());
    }

    for _ in 0..M {
        request_worker(metrics.clone());
    }

    loop {
        thread::sleep(std::time::Duration::from_secs(10));
        println!("{:?}", metrics.snapshot());
    }
}

fn task_worker(idx: usize, metrics: Metrics) {
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();
        thread::sleep(std::time::Duration::from_millis(rng.gen_range(100..5000)));
        metrics.incr(format!("call.thread.worker.{}", idx)).unwrap();
    });
}

fn request_worker(metrics: Metrics) {
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();
        thread::sleep(std::time::Duration::from_millis(rng.gen_range(50..800)));

        let page = rng.gen_range(1..256);
        metrics.incr(format!("req.page.{}", page)).unwrap();
    });
}
