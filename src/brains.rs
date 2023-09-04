#![allow(dead_code)]
use sleeper::data::{ InjuryStatus, AllPlayers, PlayerId, NflPlayer };
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

// / Given a roster, get a Vector of their injured players (of size 0 if they have none).
// / Returns errors for various invalid states
pub fn injured_from_starters(starters: Vec<PlayerId>, 
                             players: &AllPlayers
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
                Ok(st) => Some((ply, st)),
                Err(_) => None
            }
        }).collect::<Vec<(&NflPlayer, InjuryStatus)>>();
       
    Ok(result)
}

