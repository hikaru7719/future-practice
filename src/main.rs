use futures::executor::block_on;
use futures::join;

async fn hello_world() {
    println!("hello world")
}

async fn hello_world2() {
    println!("hello world2")
}

async fn run() {
    join!(hello_world(), hello_world2());
}

fn main() {
    block_on(run())
}
