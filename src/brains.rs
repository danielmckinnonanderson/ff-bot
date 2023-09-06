#![allow(dead_code)]
use sleeper::data::{ InjuryStatus, AllPlayers, PlayerId, NflPlayer, Roster, OwnerId };
use std::{collections::HashMap, slice::Iter};

pub enum BotError {
    InjuryStatusNotFound,
    Generic // remove
}

pub struct PlayerInjuryInfo {
    body_part: Option<String>,
    full_name: String,
    status: InjuryStatus,
}

pub struct OwnerInfo {
    name: String,
    team_name: String,
    owner_id: String,
}

pub struct InjuryWarningData {
    info: PlayerInjuryInfo,
    owner: OwnerInfo
}

/// Predicate used to evaluate what will be considered injured
fn is_injured(st: InjuryStatus) -> Option<InjuryStatus> {
    match st {
        // if you're healthy you're not injured
        InjuryStatus::Healthy | InjuryStatus::Questionable => None,
        // Otherwise, you're injured (or suspended, or have COVID, or whatever else)
        otherwise => Some(otherwise)
    }
}


// / Given a roster, get a Vector of their injured players (of size 0 if they have none).
// / Returns errors for various invalid states
pub fn injured_from_starters(starters: Vec<PlayerId>, 
                             players: &AllPlayers,
) -> Result<Vec<(&NflPlayer, InjuryStatus)>, BotError> {

    let result: Vec<(&NflPlayer, InjuryStatus)> = starters.into_iter()
        .filter_map(|p: PlayerId| {
            match players {
                AllPlayers::NFL(all_nfl) => all_nfl.get(&p).map_or(None, |starter_info| Some(starter_info)),
                AllPlayers::LCS(_) => todo!(),
                AllPlayers::NBA(_) => todo!()
            }
        }).filter_map(|ply: &NflPlayer| {
            match InjuryStatus::from_opt_string(ply.injury_status.clone()) {
                Ok(st) => match is_injured(st) {
                    Some(injury) => Some((ply, injury)),
                    None => None
                },
                Err(_) => None
            }
        }).collect::<Vec<(&NflPlayer, InjuryStatus)>>();
       
    Ok(result)
}

/// Checks the provided rosters for starters who are injured or unlikely to play, returning a list
/// of tuples mapping an OwnerId (String) to their list of injured players. The list can be empty.
pub fn check_rosters<'a>(rosters: &Vec<Roster>, all_players: &'a AllPlayers) -> Vec<(OwnerId, Vec<(&'a NflPlayer, InjuryStatus)>)> {
    rosters.iter()
        .filter_map(|rost| {
            match injured_from_starters(rost.starters.to_owned(), all_players) {
                Ok(injured_players) => Some((rost.owner_id.to_owned(), injured_players)),
                Err(_) => None
            }
        }).collect()
}

