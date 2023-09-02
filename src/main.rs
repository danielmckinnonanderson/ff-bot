use sleeper::client::Client;
mod brains;

#[tokio::main]
async fn main() -> Result<(), ()> {

    let league_id = match std::env::args().nth(1) {
        Some(id) => id,
        None => panic!("No league ID was passed to the executable!")
    };

    let client = sleeper::Client::new();
    let rosters = client.get_rosters(&league_id).await.unwrap();
    let players = client.get_all_players(sleeper::SleeperSport::NFL).await.unwrap();

    let result: Vec<(String, Vec<(&serde_json::Value, sleeper::InjuryStatus)>)> = rosters.iter()
        .filter_map(|rost| {
            match brains::injured_from_starters(rost.starters.to_owned(), &players) {
                Ok(injured_players) => Some((rost.owner_id.to_owned(), injured_players)),
                Err(_) => None
            }
        }).collect();

    for value in &result {
        println!("Owner: {}", value.0);
        println!("Injured players: {}", value.1.len());
    };

    Ok(())
}

