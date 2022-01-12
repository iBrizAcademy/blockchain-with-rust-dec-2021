use futures::executor::block_on;
use std::{thread::sleep, time::Duration};

async fn async_sum() -> i32{
    let mut sum = 0;
    for mut i in 0..5 {
        i = i + 1;
        sum += i;

        sleep(Duration::from_millis(1000));
        println!("{}", i);
    }
    println!("Sum: {}", sum);

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
    println!("Prod: {}", prod);
    return prod;
}

async fn async_sum_prod() {
    let sum = async_sum();
    let prod = async_prod();
    futures::join!(sum, prod);
}

fn main(){
    let async_future_join = async_sum_prod();
    let val = block_on(async_future_join);
}
