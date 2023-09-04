use sleeper::client::Client;
use sleeper::data::*;
// use groupme::client::GroupmeClient;
mod brains;

type OwnerId = String;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let league_id = match std::env::args().nth(1) {
        Some(id) => id,
        None => panic!("No league ID was passed to the executable!")
    };

    let client  = Client::new();

    // let owners = client.get_users_in_league(&league_id).await.expect("Could not get owners. Request was not completed");
    let rosters: Vec<Roster> = client.get_rosters(&league_id).await.expect("Could not get rosters. Request was not completed");
    let players: AllPlayers = client.get_all_players(SleeperSport::NFL).await.expect("Could not get all NFL players. Request was not completed");

    let result: Vec<(OwnerId, Vec<(&NflPlayer, InjuryStatus)>)> = rosters.iter()
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

