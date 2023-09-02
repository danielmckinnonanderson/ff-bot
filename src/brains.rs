#![allow(dead_code)]
use std::{collections::HashMap, slice::Iter};

use serde_json::Value;
use sleeper::*;


pub enum BotError {
    InjuryStatusNotFound,
    Generic // remove
}

struct PlayerInjuryInfo {
    body_part: Option<String>,
    full_name: String,
    status: sleeper::InjuryStatus,
}

/// Given a roster, get a Vector of their injured players (of size 0 if they have none).
/// Returns errors for various invalid states
pub fn injured_from_starters(starters: Vec<String>, 
                             players: &HashMap<String, serde_json::value::Value>
) -> Result<Vec<(&Value, InjuryStatus)>, BotError> {

    let result: Vec<(&Value, InjuryStatus)> = starters.into_iter()
        .filter_map(|p: PlayerId| players.get(&p))
        .filter_map(|ply: &Value| {
            if !ply.is_object() {
                println!("Queried for a player but did not get an object back. Skipping.");
                return None;
            };

            let status_field = ply.get("injury_status");

            if status_field.is_none() {
                println!("Player object did not have a field 'injury_status'. That's odd.");
                return None;
            }

            let inj_status_raw: &Value = status_field.unwrap();

            let injury_status = match inj_status_raw {
                Value::String(inj_st_str) => inj_st_str,
                Value::Null => return None, // No logging bc this is an expected value
                unmatched => {
                    println!("JSON value for field 'injury_status' was {:?}", unmatched);
                    return None;
                }
            };

            let status: InjuryStatus = match InjuryStatus::from_str(injury_status) {
                Ok(value) => value,
                Err(e) => {
                    eprintln!("{e}");
                    println!("Could not convert string '{}' into InjuryStatus", &injury_status);
                    return None;
                }
            };

            Some((ply, status))
        }).collect::<Vec<(&Value, InjuryStatus)>>();
       
    Ok(result)
}

