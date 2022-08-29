use futures::StreamExt;

async fn long_io(a: usize) -> usize {
    const COMPUTATION_TIME: u64 = 1;
    tokio::time::sleep(tokio::time::Duration::from_secs(COMPUTATION_TIME)).await;
    println!("Computation of: {}", a);
    a * a
}

async fn print_result(res: &usize) {
    println!("Println result: {}", res);
}

async fn example_without_result_colection() {
    let time = std::time::SystemTime::now();

    let jobs = 0..10;
    let concurency = 5;

    futures::stream::iter(jobs)
        .for_each_concurrent(concurency, |job| async move {
            let result = long_io(job).await;
            print_result(&result).await;
        })
        .await;

    println!("Computed in: {}", time.elapsed().unwrap().as_secs());
}

async fn example_with_result_collection() {
    const CONCURENCY: usize = 5;
    let jobs = 1..10;
    let results: Vec<usize> = futures::stream::iter(jobs)
        .map(long_io)
        .buffer_unordered(CONCURENCY)
        .collect()
        .await;
    println!("Results: {:?}", results);
}

#[tokio::main]
async fn main() {
    println!("Without result collection:\n");
    example_without_result_colection().await;

    print!("\n\n\n");

    println!("With result collection:\n");
    example_with_result_collection().await;
}