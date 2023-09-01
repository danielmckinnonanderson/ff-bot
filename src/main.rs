use sleeper::client::Client;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let client = sleeper::Client::new();

    Ok(())
}

