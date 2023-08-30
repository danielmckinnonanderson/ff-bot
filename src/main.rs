mod sleeper;

#[tokio::main]
async fn main() -> Result<(), sleeper::Error> {
    let league_id = match std::env::args().nth(1) {
        Some(l) => l,
        None    => panic!("D'oh! You forgot to pass your league ID to the command line")
    };

    let client = sleeper::Client::new();
    let resp_body = client.get_league(&league_id).await;

    println!("{:?}", resp_body);

    Ok(())
}

