use futures::executor::block_on;
use futures::future::Future;
use tokio::join;
use std::pin::Pin;
// use tokio::runtime::Runtime;
//
use futures::future::join_all;
use tokio::task;

fn main() {
    block_on(speak());
}

async fn speak() {
    let hold = vec![
        Box::pin(say()) as Pin<Box<dyn Future<Output = Result<(),&'static str>>>>,
        Box::pin(greet())
    ];
    // block_on(join_all(hold));
    let results = join_all(hold).await;
}

async fn say() -> Result<(),&'static str>{
    println!("hello");
    return Ok(());
}

async fn greet() -> Result<(),&'static str>{
    println!("world");
    return Ok(());
}
