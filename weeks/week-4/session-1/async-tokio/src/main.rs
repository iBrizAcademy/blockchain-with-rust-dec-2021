use std::{thread::sleep, time::Duration};


async fn fn_async() {
    for i in 0..10 {
        sleep(Duration::from_millis(1000));
        println!("{}", i);
    }
}

fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let future = fn_async();
    rt.block_on(future);
}