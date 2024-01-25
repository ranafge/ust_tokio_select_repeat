async fn action() {
    println!("action function")
}

#[tokio::main]
async fn main() {
    let (mut tx, mut rx) = tokio::sync::mpsc::channel(128);
    let operation = action();

    tokio::spawn(async move {
        tx.send(2).await.unwrap();
    });

    tokio::pin!(operation);

    loop {
        tokio::select! {
            _ = &mut operation => break,
            Some(v) = rx.recv() => {
                if v % 2 == 0 {
                    break;
                }
            }
        }
    }

}
/*
The use of tokio::pin! is necessary to pin the operation because it is being awaited on a reference (&mut operation).
The pinning ensures that the referenced future can be safely used in this context.
*/