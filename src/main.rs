use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async {
        let _ = tx1.send("one");
        // _ is a place holder for a value of the `tx1.send("one");` but here intentionally ignore
    });
    tokio::spawn(async {
        let _ = tx2.send("two");
    });
    // there are two channel to send message first one is send `one` and second channel send  'two'

    // in tokio::select wait for a message to complete one . there are two branch of val 
    // if rx1 or rx2 is complete message will print for that complete first. other one will be drop 
    // drop or cancellatin occure by dropin future
    tokio::select! {
        // val is a pttern = rx1 is async operion `{}` block is handler
        val = rx1 => {
            println!("rx1 completed first with {:?}", val);
        }
        val = rx2 => {
            println!("rx2 completed first with {:?}", val)
        }
    }
}

// if <pattern> = <async operation>  
//when async operation result match with pattern then cancell other branches and execute handler=> <handler>