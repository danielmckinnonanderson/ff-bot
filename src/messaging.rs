use groupme::{ client::*, OutgoingBotMessage };
use sleeper::data::{ InjuryStatus, RosterPosition, OwnerId };

use crate::brains::BadStarters;

/// Creates the formatted message String in cases where the owner is starting an injured or
/// otherwise inactive player at the given position
pub fn create_injured_msg_string(team_name: Option<String>,
                                 owner_username: &String,
                                 player_name: &str,
                                 status: InjuryStatus,
                                 position: RosterPosition) -> String {
    let team_name = match team_name {
        Some(team_name) => team_name,
        None => "".to_string()
    };

    let owner_username_string: String = match team_name.as_ref() {
        "" => owner_username.to_string(),  // No team name, so just say username
        _  => format!("({})", owner_username) // Team name present, owner in parentheses
    };

    format!("⛔ {team_name} {owner_username_string} is starting {player_name} ({status}) at {position}! ⛔")
}

/// Creates the formatted message String in cases where the owner is not starting a player at the
/// given position
pub fn create_empty_msg_string(team_name: Option<String>, owner_username: &String, position: RosterPosition) -> String {
    let team_name = match team_name {
        Some(team_name) => team_name,
        None => "".to_string()
    };

    let owner_username_string: String = match team_name.as_ref() {
        "" => owner_username.to_string(),  // No team name, so just say username
        _  => format!("({})", owner_username) // Team name present, owner in parentheses
    };

    format!("🕳️ {team_name} {owner_username_string} is not starting a player at {position}! 🕳️")
}

/// Creates the formatted message String in cases where the owner is starting a player on bye at
/// the given position
pub fn create_bye_msg_string(team_name: Option<String>, owner_username: &String, player_name: &str, position: RosterPosition) -> String {
    let team_name = match team_name {
        Some(team_name) => team_name,
        None => "".to_string()
    };

    let owner_username_string: String = match team_name.as_ref() {
        "" => owner_username.to_string(),  // No team name, so just say username
        _  => format!("({})", owner_username) // Team name present, owner in parentheses
    };
    format!("💤 {team_name} {owner_username_string} is starting {player_name} (on bye) at {position}! 💤")
}

