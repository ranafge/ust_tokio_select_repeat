use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx1, mut rx1) = mpsc::channel(128);
    let (tx2, mut rx2) = mpsc::channel(128);

    tokio::spawn(async move {
        tx1.send(1).await.unwrap();
    });
    tokio::spawn(async move {
        tx2.send(2).await.unwrap();
    });
    /*
        else block will be executing when rx1 and rx2 return None
            tokio::spawn(async move {
        tx1.send(1);

    });
    tokio::spawn(async move {
        tx2.send(2).;
    });

    use this code to see the else block execution.


     */

    tokio::select! {
        Some(v) = rx1.recv() => {
            println!("Got {:?} from rx1", v);
        }
        Some(v) = rx2.recv() => {
            println!("Got {:?} from rx2", v);
        }
        else => {
            println!("Both channels closed");


        }
    }
}
