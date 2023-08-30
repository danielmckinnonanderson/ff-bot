mod sleeper;

#[tokio::main]
async fn main() -> Result<(), sleeper::SleeperError> {

    let client = sleeper::Client::new();

    let resp_body = client.get_sport_state(sleeper::SleeperSport::NBA).await?;

    println!("{:?}", resp_body);

    Ok(())
}


