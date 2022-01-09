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

async fn async_wait_sum_prod() -> (i32, i32){
    let sum = async_sum().await;
    let prod = async_prod().await;
    (sum, prod)
}

fn main(){
    let val = block_on(async_wait_sum_prod());
    println!("val: {:?}", val);
}
