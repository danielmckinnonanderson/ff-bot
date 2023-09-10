use groupme::{ client::*, OutgoingBotMessage };
use sleeper::data::{ InjuryStatus, RosterPosition };

/// Creates the formatted message String in cases where the owner is starting an injured or
/// otherwise inactive player at the given position
pub fn create_injured_msg_string(team_name: &str,
                                 owner_username: &str,
                                 player_name: &str,
                                 status: InjuryStatus,
                                 position: RosterPosition) -> String {
    format!("⛔ Team {team_name} ({owner_username}) is starting {player_name} ({status}) at {position}! ⛔")
}

/// Creates the formatted message String in cases where the owner is not starting a player at the
/// given position
pub fn create_empty_msg_string(team_name: &str, owner_username: &str, position: RosterPosition) -> String {
    format!("🕳️ Team {team_name} ({owner_username}) is not starting a player at {position}! 🕳️")
}

/// Creates the formatted message String in cases where the owner is starting a player on bye at
/// the given position
pub fn create_bye_msg_string(team_name: &str, owner_username: &str, player_name: &str, position: RosterPosition) -> String {
    format!("💤 Team {team_name} ({owner_username}) is starting {player_name} (on bye) at {position}! 💤")
}

