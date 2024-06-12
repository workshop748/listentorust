use std::task::Poll;

use futures::Future;
use tokio::net::TcpStream;

extern "C" {
    fn MeaningOfLifeTheUniverseAndEverything() -> i32;
}


#[tokio::main]
async fn main() {
    unsafe {
        eprintln!("The meaning of life is {}", MeaningOfLifeTheUniverseAndEverything());
    }
    let mut tasks = vec![];
    let mut target = "192.168.86.42";
    for i in 0..65535 {
        tasks.push(async move{
            let connection = TcpStream::connect(format!("{target}:{i}")).await;
            if connection.is_ok() {
                eprintln!("Port {i} is open");
            }
        });
    }
    futures::future::join_all(tasks).await;
}
