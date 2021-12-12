use futures::executor::block_on;

async fn hello_world() {
    println!("hello world")
}

async fn hello_world2() {
    println!("hello world2")
}

async fn run() {
    hello_world().await;
    hello_world2().await;
}

fn main() {
    block_on(run())
}
