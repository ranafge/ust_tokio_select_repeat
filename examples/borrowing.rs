use std::io;
use std::net::SocketAddr;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::oneshot;

async fn race(data: &[u8], addr1: SocketAddr, addr2: SocketAddr) -> io::Result<()> {
    tokio::select! {
        Ok(_) = async {
            let mut socket  = TcpStream::connect(addr1).await?;
            socket.write_all(data).await?;
            Ok::<_, io::Error(())>
        } => {}

        Ok(_) = async {
            let mut socket = TcpStream::connect(addr2).await?;
            socket.write_all(data).await?;
            Ok::<_, io::Error(())>
        } => {}
        else => {}
    }

    Ok(())
}

/*
    the data variable is being borrowed immutably for both asyn expressions.
    when one branch execute successfull other will drop because of pattern match
*/
#[tokio::main]
async fn main() {
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    let mut out = String::new();

    tokio::spawn(async move { tx1.send("one".to_string()) });
    tokio::spawn(async move { tx2.send("two".to_string()) });

    tokio::select! {
        _= rx1 => {
            out.push_str("rx1 completed");
        }
        _= rx2 => {
            out.push_str("rx2 copleted")
        }
    }

    println!("{}", out);
}
