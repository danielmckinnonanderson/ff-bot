use sleeper::client::Client;
use sleeper::data::*;
mod brains;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let league_id = match std::env::args().nth(1) {
        Some(id) => id,
        None => panic!("No league ID was passed to the executable!")
    };

    let client  = Client::new();

    let rosters: Vec<Roster> = client.get_rosters(&league_id).await.expect("Could not get rosters. Request was not completed");
    let all_players: AllPlayers = client.get_all_players(SleeperSport::NFL).await.expect("Could not get all NFL players. Request was not completed");

    let result = brains::check_rosters(&rosters, &all_players);

    for value in &result {
        println!("Owner: {}", value.0);
        println!("Injured players: {}", value.1.len());
    };

    Ok(())
}

