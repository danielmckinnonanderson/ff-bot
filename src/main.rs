mod sleeper;

#[tokio::main]
async fn main() -> Result<(), sleeper::SleeperError> {

    let client = sleeper::Client::new();

    let all_players = client.get_all_players(sleeper::SleeperSport::NFL).await?;

    Ok(())
}

