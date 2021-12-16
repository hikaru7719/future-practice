use futures::executor::block_on;
use futures::select;
use futures::FutureExt;

async fn hello_world() -> String {
    "hello world".to_string()
}

async fn hello_world2() -> String {
    "hello world2".to_string()
}

async fn run() {
    select! {
        a = hello_world().fuse() => println!("{}", a),
        b = hello_world2().fuse() => println!("{}", b)
    }
}

fn main() {
    block_on(run())
}
