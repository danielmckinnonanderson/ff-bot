use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use chrono::Utc;
use cron::Schedule;
use groupme::client::GroupmeClient;
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
    let bot_id = std::env::args()
                          .nth(2)
                          .expect("No bot ID was passed to the executable!");

    // let sunday_before_noon        = Schedule::from_str("0 0 11 * * SUN").expect("Invalid cron string!");
    // let monday_thursday_primetime = Schedule::from_str("0 30 18 * * MON,THURS").expect("Invalid cron string!");

    let sleeper_client = SleeperClient::new();

    let result = check_rosters_and_message(league_id.as_ref(), &sleeper_client, bot_id.as_ref()).await;

    Ok(())

    // loop {
    //     let now = Utc::now();
    //     println!("Now {}", now);

    //     let next_before_primetime = monday_thursday_primetime.after(&now)
    //         .next()
    //         .expect("Failed to find next run for Monday & Thursday primetime!");

    //     let next_before_noon = sunday_before_noon.after(&now)
    //         .next()
    //         .expect("Failed to find next run for Sunday before noon!");

    //     let until_next_sun_noon = next_before_noon.signed_duration_since(now);
    //     let until_next_mon_thurs_prime = next_before_primetime.signed_duration_since(now);
    //     let min_duration = until_next_sun_noon.min(until_next_mon_thurs_prime);

    //     time::sleep(min_duration.to_std().unwrap()).await;

    //     check_rosters_and_message(league_id.as_ref(), &sleeper_client, bot_id.as_ref()).await;
    // }
}

async fn check_rosters_and_message(league_id: &str, sleeper_client: &SleeperClient, bot_id: &str) -> () {

    let msg_client = Arc::new(GroupmeClient::new(bot_id.to_string()));

    let all_players: AllPlayers = sleeper_client.get_all_players(SleeperSport::NFL).await.expect("Could not get all NFL players. Request was not completed");
    let all_nfl_players: HashMap<PlayerId, NflPlayer> = match all_players {
        AllPlayers::NFL(players) => players,
        _ => panic!("Wrong sport! This is a fantasy football bot :)")
    };

    let rosters: Vec<Roster>  = sleeper_client.get_rosters(&league_id).await.expect("Could not get rosters. Request was not completed");
    let owners: Vec<SleeperUser> = sleeper_client.get_users_in_league(&league_id).await.expect("Could not get owners in league. Request was not completed");
    let nfl_state: SportState = sleeper_client.get_sport_state(SleeperSport::NFL).await.expect("Could not get NFL sport state. Request was not completed");

    let current_week = nfl_state.week;

    let result: Box<Vec<(SleeperUser, BadStarters)>> = Box::new(brains::check_rosters(&rosters, &all_nfl_players, current_week)
        .into_iter()
        .map(|value| {
            let owner = owners.iter().find(|user| user.user_id == value.0).unwrap();
            (owner.to_owned(), value.1)
        }).collect());

    println!("Got result");

    let tasks = result.into_iter().flat_map(|(user, bad_starters)| {
        let bad_starters_arc = Arc::new(bad_starters);
        let msg_client_copy = Arc::clone(&msg_client);
        // FIXME - Update user metadata struct to not be obnoxious to work with
        // let metadata  = user.metadata.clone();
        // let team_name = metadata.get("team_name").unwrap();

        let username  = Box::new(user.display_name.unwrap());

        let injured_st_task = tokio::spawn(async move {
            let msg_client_copy = Arc::clone(&msg_client_copy);
            let bad_st_clone = Arc::clone(&bad_starters_arc);
            let injured_starters = &bad_st_clone.0;

            for injured_starter in injured_starters {
                println!("{:?}", injured_starter.name);
                println!("{:?}", injured_starter.pos);
                let content = messaging::create_injured_msg_string(
                    None,
                    &username.clone(),
                    injured_starter.name.as_ref(), 
                    injured_starter.status.clone(),
                    injured_starter.pos.clone());
                
                println!("Content is {content}");

                match msg_client_copy.post_bot_message(content.as_ref()).await {
                    Ok(_) => {
                        println!("Posted message successfully!");
                        ()
                    },
                    Err(e) => {
                        println!("Error posting message!");
                        eprintln!("{e}");
                    }
                }
            }
        });

        let empty_st_task = tokio::spawn(async move {
            let msg_client_copy = &msg_client.clone();
            let bad_st_clone = Arc::clone(&bad_starters_arc);
            let empty_starters = &bad_st_clone.1;

            for empty_starter in empty_starters {
                let content = messaging::create_empty_msg_string(
                    None,
                    &username.clone(),
                    empty_starter.clone());
                
                println!("Content is {content}");

                match msg_client_copy.post_bot_message(content.as_ref()).await {
                    Ok(_) => {
                        println!("Posted message successfully!");
                        ()
                    },
                    Err(e) => {
                        println!("Error posting message!");
                        eprintln!("{e}");
                    }
                }
            }
        });

        let bye_st_task = tokio::spawn(async move {
            let msg_client_copy = Arc::clone(&msg_client);
            let bad_st_clone = Arc::clone(&bad_starters_arc);
            let bye_starters = &bad_st_clone.2;

            for bye_starter in bye_starters {
                let content = messaging::create_bye_msg_string(
                    None,
                    &username.clone(),
                    bye_starter.name.as_ref(),
                    bye_starter.pos.clone());
                
                println!("Content is {content}");

                match msg_client_copy.post_bot_message(content.as_ref()).await {
                    Ok(_) => {
                        println!("Posted message successfully!");
                        ()
                    },
                    Err(e) => {
                        println!("Error posting message!");
                        eprintln!("{e}");
                    }
                }
            }
        });

        vec![injured_st_task, empty_st_task, bye_st_task]
    });


    for task in tasks {
        println!("Running task...");
        task.await.expect("Task failed!");
    }

    println!("All tasks completed.");
}

