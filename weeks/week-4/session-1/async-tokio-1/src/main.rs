use futures::executor::block_on;
use std::{thread::sleep, time::Duration};
use tokio::task;

async fn async_sum() -> i32{
    let mut sum = 0;
    for mut i in 0..5 {
        i = i + 1;
        sum += i;

        sleep(Duration::from_millis(1000));
        println!("{}", i);
    }

    return sum;
}
async fn async_prod() -> i32{
    let mut prod = 1;
    for mut i in 0..5 {
        i = i + 1;
        prod *= i;

        sleep(Duration::from_millis(1000));
        println!("{}", i);
    }

    return prod;
}

async fn async_sum_prod() {
    let sum = async_sum();
    let prod = async_prod();
    futures::join!(sum, prod);
}

// #[tokio::main]
 fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let future = async_sum();
    let i = rt.block_on(future);
    println!("Sum: {}", i);
}