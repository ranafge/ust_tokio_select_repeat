async fn computational1() -> String {
    "t".to_string()
}

async fn computational2() -> String {
    "t2".to_string()
}

#[tokio::main]
async fn main() {
    let out = tokio::select! {
        res1 = computational1() => {
            println!("first one")
        }
        res2 = computational2 => {
            println!("second one")
        }
    };
    println!("Got = {:?} ", out);
}
