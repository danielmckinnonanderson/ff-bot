use std::collections::HashMap;
use sleeper::client::Client;
use sleeper::data::*;
mod brains;
use brains::*;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let league_id = std::env::args()
                             .nth(1)
                             .expect("No league ID was passed to the executable!");

    let client = Client::new();

    let all_players: AllPlayers = client.get_all_players(SleeperSport::NFL).await.expect("Could not get all NFL players. Request was not completed");
    let all_nfl_players: HashMap<PlayerId, NflPlayer> = match all_players {
        AllPlayers::NFL(players) => players,
        _ => panic!("Wrong sport! This is a fantasy football bot :)")
    };

    let rosters: Vec<Roster>    = client.get_rosters(&league_id).await.expect("Could not get rosters. Request was not completed");
    let nfl_state: SportState   = client.get_sport_state(SleeperSport::NFL)
                                        .await.expect("Could not get NFL sport state. Request was not completed");

    let current_week            = nfl_state.week;

    let result: Vec<(OwnerId, BadStarters)> = brains::check_rosters(&rosters, &all_nfl_players, current_week);

    result.iter().for_each(|value| {
        println!("OwnerID: {}", value.0);
        if value.1.0.len() > 0 {
            println!("->  Injured starters...");
            for inj in &value.1.0 {
                println!("    ->  {} - {} is {}", inj.pos, inj.name, inj.status);
            }
        } else {
            println!("->  No injured starters!");
        }


        if value.1.1.len() > 0 {
            println!("->  Empty starters...");
            for empty in &value.1.1 {
                println!("    ->  {} is empty", empty);
            }
        } else {
            println!("->  No empty starters!");
        }

        if value.1.2.len() > 0 {
            println!("->  Starters on bye...");
            for bye in &value.1.2 {
                println!("    ->  {} - {} is on bye", bye.pos, bye.name);
            }
        } else {
            println!("->  No starters on bye!");
        }
    });

    Ok(())
}

