use sleeper::data::{ InjuryStatus, AllPlayers, PlayerId, NflPlayer,
    Roster, OwnerId, RosterPosition, };

#[derive(Debug)]
pub enum BotError {
    InjuryStatusNotFound,
    InvalidStarterIndex,
    Generic // remove
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
                             players: &AllPlayers) -> Result<Vec<(&NflPlayer, InjuryStatus)>, BotError> {
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

/// Given a vector of PlayerId's representing the starters,
/// produces a vector of the RosterPositions for which the PlayerId
/// was "0" (signifying empty).
pub fn empties_from_starters(starters: Vec<PlayerId>) -> Vec<RosterPosition> {
    let mut result: Vec<RosterPosition> = vec![];

    for (index, value) in starters.iter().enumerate() {
        if value == "0" {
            let empty_pos = position_from_index(index).unwrap();
            result.push(empty_pos);
        }
    }

    result
}

// TODO - determine a way to make this generic, using the league_settings field of league
/// This relies on whatever the league settings / starter positions are.
/// It works for my league, which uses QB, WR x 3, RB x 2, FLEX x 2, TE, K, DST
fn position_from_index(index: usize) -> Result<RosterPosition, BotError> {
    match index {
        0         => Ok(RosterPosition::QB),
        1 | 2     => Ok(RosterPosition::RB),
        3 | 4 | 5 => Ok(RosterPosition::WR),
        6         => Ok(RosterPosition::TE),
        7 | 8     => Ok(RosterPosition::FLEX),
        9         => Ok(RosterPosition::K),
        10        => Ok(RosterPosition::DEF),
        _         => Err(BotError::InvalidStarterIndex)
    }
}

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

