use std::collections::HashMap;
use std::str::FromStr;
use chrono::Utc;
use cron::Schedule;
use tokio::time::{self, Duration};
use sleeper::client::Client as SleeperClient;
use sleeper::data::*;

mod brains;
mod messaging;

use brains::*;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let league_id = std::env::args()
                             .nth(1)
                             .expect("No league ID was passed to the executable!");

    let sunday_before_noon        = Schedule::from_str("0 0 11 * * SUN").expect("Invalid cron string!");
    let monday_thursday_primetime = Schedule::from_str("0 30 18 * * MON,THURS").expect("Invalid cron string!");

    let sleeper_client = SleeperClient::new();

    loop {
        let now = Utc::now();
        println!("Now {}", now);

        let next_before_primetime = monday_thursday_primetime.after(&now)
            .next()
            .expect("Failed to find next run for Monday & Thursday primetime!");

        let next_before_noon = sunday_before_noon.after(&now)
            .next()
            .expect("Failed to find next run for Sunday before noon!");

        let until_next_sun_noon = next_before_noon.signed_duration_since(now);
        let until_next_mon_thurs_prime = next_before_primetime.signed_duration_since(now);
        let min_duration = until_next_sun_noon.min(until_next_mon_thurs_prime);

        time::sleep(min_duration.to_std().unwrap()).await;

        check_rosters_and_message(league_id.as_ref(), &sleeper_client).await;
    }
}

async fn check_rosters_and_message(league_id: &str, sleeper_client: &SleeperClient) -> () {
    let all_players: AllPlayers = sleeper_client.get_all_players(SleeperSport::NFL).await.expect("Could not get all NFL players. Request was not completed");
    let all_nfl_players: HashMap<PlayerId, NflPlayer> = match all_players {
        AllPlayers::NFL(players) => players,
        _ => panic!("Wrong sport! This is a fantasy football bot :)")
    };

    let rosters: Vec<Roster>  = sleeper_client.get_rosters(&league_id).await.expect("Could not get rosters. Request was not completed");
    let nfl_state: SportState = sleeper_client.get_sport_state(SleeperSport::NFL).await.expect("Could not get NFL sport state. Request was not completed");

    let current_week = nfl_state.week;

    let result: Vec<(OwnerId, BadStarters)> = brains::check_rosters(&rosters, &all_nfl_players, current_week);

    print_invalid_starters(result);
}

fn print_invalid_starters(values: Vec<(OwnerId, BadStarters)>) -> () {
    values.iter().for_each(|value| {
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
}

