use std::io;
use tokio::net::TcpListener;
use tokio::sync::oneshot;

#[tokio::main]
async fn main() -> io::Result<()> {
    // [setup `rx` oneshot channel]
    let (t, x) = oneshot::channel();
    let _ = t.send("t");

    let listener = TcpListener::bind("localhost:3465").await?;

    tokio::select! {
        res = async {
            loop {
                let (socket, _) = listener.accept().await?;
                tokio::spawn(async move { process(socket) });
            }

            // Help the rust type inferencer out
            Ok::<_, io::Error>(())
        } => {
            res?;
        }
        _ = x=> {
            println!("terminating accept loop");
        }
    }

    Ok(())
}
