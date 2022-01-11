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


fn main(){
    let result_sum = block_on(async_sum());
    let result_prod = block_on(async_prod());
    println!("result_sum: {}", result_sum);
    println!("result_prod: {}", result_prod);
}
