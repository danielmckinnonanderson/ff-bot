mod sleeper;

#[tokio::main]
async fn main() -> Result<(), sleeper::SleeperError> {
    let client = sleeper::Client::new();

    Ok(())
}

