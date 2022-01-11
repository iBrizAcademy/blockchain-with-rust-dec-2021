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

fn main(){
    let result = block_on(async_sum());
    println!("result: {}", result);
}
