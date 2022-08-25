use rand::{thread_rng, Rng};
use rayon::{prelude::*, ThreadPoolBuilder};
use std::thread;
use std::time::Duration;

fn hard_computation(a: usize) -> usize {
    let milis_for_computation = Duration::from_millis(thread_rng().gen_range(1..10));
    // thread::sleep(milis_for_computation);
    thread::sleep(Duration::from_millis(1000));
    a * a
}

fn parallel_computation() {
    let time = std::time::SystemTime::now();

    let jobs = 1..10;
    jobs.into_par_iter()
        .map(hard_computation)
        .for_each(|x| println!("lambda: {:?}", x));

    println!("Computation in {} sec.", time.elapsed().unwrap().as_secs());
}

fn parallel_computation_limited_threads() {
    let time = std::time::SystemTime::now();

    let thread_pool = ThreadPoolBuilder::new().num_threads(5).build().unwrap();
    thread_pool.install(|| {
        (1..10)
            .into_par_iter()
            .map(hard_computation)
            .for_each(|x| println!("lambda: {:?}", x));
    });

    println!("Computation in {} sec.", time.elapsed().unwrap().as_secs());
}

fn main() {
    parallel_computation_limited_threads();
}
