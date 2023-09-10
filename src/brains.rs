use std::collections::HashMap;
use sleeper::data::{ InjuryStatus, PlayerId, NflPlayer,
    Roster, OwnerId, RosterPosition, };

/// Type representing a starter RosterPosition which was found to be empty
pub type EmptySpot = RosterPosition;

/// Data struct representing a player who is starting,
/// despite their status indicating an injury that will
/// prevent them from playing in the current week.
pub struct InjuredStarter {
    pub name: String,
    pub status: InjuryStatus,
    pub pos: RosterPosition
}

/// Data struct representing a player who is starting,
/// despite being on bye for the current week.
pub struct ByeStarter {
    pub name: String,
    pub pos: RosterPosition
}

/// Type to represent all of a specific fantasy team's invalid starters,
/// including starters who are injured, on bye, or empty.
pub type BadStarters = (Vec<InjuredStarter>, Vec<EmptySpot>, Vec<ByeStarter>);

#[derive(Debug)]
pub enum BotError {
    InjuryStatusNotFound,
    InvalidStarterIndex,
    Generic // remove
}

pub enum NflTeamAbbrv {
    ARI, ATL, BAL, BUF,
    CAR, CHI, CIN, CLE,
    DAL, DEN, DET, GB,
    HOU, IND, JAX, KC,
    LAC, LAR, LV,  MIA,
    MIN, NE,  NO,  NYG,
    NYJ, PHI, PIT, SEA,
    SF,  TB,  TEN, WAS,
    FA // Not on a team
}

impl NflTeamAbbrv {
    // TODO - Network call to make this current year-after-year w/o human intervention
    /// Get the bye week for this team in the 2023 season. Week is literal value,
    /// not zero-indexed (ie week 1 is literally the first week of the season)
    pub fn get_bye_week(&self) -> u8 {
        match self {
            Self::CLE | Self::LAC | Self::SEA | Self::TB  => 5,
            Self::GB  | Self::PIT => 6,
            Self::CAR | Self::CIN | Self::DAL | Self::HOU | Self::NYJ | Self::TEN => 7,
            Self::DEN | Self::DET | Self::JAX | Self::SF  =>  9,
            Self::KC  | Self::LAR | Self::MIA | Self::PHI => 10,
            Self::ATL | Self::IND | Self::NE  | Self::NO  => 11,
            Self::BAL | Self::BUF | Self::CHI | Self::LV  | Self::MIN | Self::NYG => 13,
            Self::ARI | Self::WAS => 14,
            Self::FA  => 0 // Don't call this on a free agent
        }
    }

    // TODO - Make this generic year-after-year instead of hardcoded
    /// Depends upon the get_bye_week function to test.
    pub fn is_on_bye(&self, current_week: u8) -> bool {
        match self {
            Self::FA  => true, // All free agents are always 'on bye', since they aren't playing
            unmatched => unmatched.get_bye_week() == current_week
        }
    }

    // TODO - Maybe move this to the Sleeper library and deserialize the player JSON 'team' field
    //        into this directly...
    /// Takes an optional team name, where 'None' means 'free agent'
    pub fn new(team: Option<String>) -> Option<Self> {
        match team {
            None => Some(Self::FA),

            Some(team_name) => match team_name.as_str() {
                "ARI" => Some(NflTeamAbbrv::ARI),
                "ATL" => Some(NflTeamAbbrv::ATL),
                "BAL" => Some(NflTeamAbbrv::BAL),
                "BUF" => Some(NflTeamAbbrv::BUF),
                "CAR" => Some(NflTeamAbbrv::CAR),
                "CHI" => Some(NflTeamAbbrv::CHI),
                "CIN" => Some(NflTeamAbbrv::CIN),
                "CLE" => Some(NflTeamAbbrv::CLE),
                "DAL" => Some(NflTeamAbbrv::DAL),
                "DEN" => Some(NflTeamAbbrv::DEN),
                "DET" => Some(NflTeamAbbrv::DET),
                "GB"  => Some(NflTeamAbbrv::GB),
                "HOU" => Some(NflTeamAbbrv::HOU),
                "IND" => Some(NflTeamAbbrv::IND),
                "JAX" => Some(NflTeamAbbrv::JAX),
                "KC"  => Some(NflTeamAbbrv::KC),
                "LAC" => Some(NflTeamAbbrv::LAC),
                "LAR" => Some(NflTeamAbbrv::LAR),
                "LV"  => Some(NflTeamAbbrv::LV),
                "MIA" => Some(NflTeamAbbrv::MIA),
                "MIN" => Some(NflTeamAbbrv::MIN),
                "NE"  => Some(NflTeamAbbrv::NE),
                "NO"  => Some(NflTeamAbbrv::NO),
                "NYG" => Some(NflTeamAbbrv::NYG),
                "NYJ" => Some(NflTeamAbbrv::NYJ),
                "PHI" => Some(NflTeamAbbrv::PHI),
                "PIT" => Some(NflTeamAbbrv::PIT),
                "SEA" => Some(NflTeamAbbrv::SEA),
                "SF"  => Some(NflTeamAbbrv::SF),
                "TB"  => Some(NflTeamAbbrv::TB),
                "TEN" => Some(NflTeamAbbrv::TEN),
                "WAS" => Some(NflTeamAbbrv::WAS),
                "FA"  => Some(NflTeamAbbrv::FA),
                _     => None,
            }
        }
    }
}

/// Checks the provided rosters for starters who are injured or unlikely to play, returning a list
/// of tuples mapping an OwnerId (String) to their list of bad starters. The list can be empty.
pub fn check_rosters(rosters: &Vec<Roster>, 
                         all_players: &HashMap<PlayerId, NflPlayer>, 
                         current_week: u8
) -> Vec<(OwnerId, BadStarters)> {

    rosters.iter().map(|roster: &Roster|{
        let starter_ids: Vec<PlayerId> = roster.starters.clone();

        // Collect starters into vec of option, because if we filter out the players
        // we can't get from the map, we are removing the ID "0" (empty players),
        // and thus fucking up the order / quantity of starters. We need to preserve order
        // because the order determines which position the starter is in at.
        let starters: Vec<Option<&NflPlayer>> = starter_ids.iter()
            .map(|id| all_players.get(id))
            .collect();

        let empty_starters:   Vec<EmptySpot>      = empties_from_starters(&roster.starters);
        let injured_starters: Vec<InjuredStarter> = injured_from_starters(&starters);
        let bye_starters:     Vec<ByeStarter>     = byes_from_starters(&starters, current_week);

        (roster.owner_id.to_owned(), ( injured_starters, empty_starters, bye_starters ))

    }).collect()
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

/// Given a roster, get a Vector of their injured players (of size 0 if they have none).
/// Swallows errors and filters out the starter / player that produced them.
pub fn injured_from_starters(starters: &Vec<Option<&NflPlayer>>) -> Vec<InjuredStarter> {
    starters.iter()
        .enumerate()
        .filter_map(|(index, opt_player)| {
            match opt_player {
                Some(player) => match InjuryStatus::from_opt_string(player.injury_status.clone()) {
                    Ok(status) => match is_injured(status) {
                        Some(injury) => {
                            Some(InjuredStarter {
                                name: player.full_name.as_ref().unwrap().to_string(),
                                pos: position_from_index(index).unwrap(),
                                status: injury
                            })
                        },
                        None => None
                    },
                    Err(_) => None
                },
                None => None
            }
        }).collect()
}

/// Given a vector of PlayerId's representing the starters,
/// produces a vector of the RosterPositions for which the PlayerId
/// was "0" (signifying empty).
pub fn empties_from_starters(starters: &Vec<PlayerId>) -> Vec<EmptySpot> {
    let mut result: Vec<RosterPosition> = vec![];

    for (index, value) in starters.iter().enumerate() {
        if value == "0" {
            let empty_pos = position_from_index(index).unwrap();
            result.push(empty_pos);
        }
    }

    result
}

/// Produce a list of ByeStarters from one fantasy team's starter list.
pub fn byes_from_starters(starters: &Vec<Option<&NflPlayer>>, current_week: u8) -> Vec<ByeStarter> {
    starters.iter()
        .enumerate()
        .filter_map(|(index, opt_player)| {
            match opt_player {
                Some(player) => match NflTeamAbbrv::new(player.team.clone()) {
                    Some(team) => match team.is_on_bye(current_week) {
                        true => {
                            // Can't use full_name here because defenses don't have
                            // that field. Instead, defenses' first_name is the city and last_name
                            // is the team name. We'll use that instead just to be safe.
                            Some(ByeStarter {
                                name: format!("{} {}", player.first_name, player.last_name),
                                pos:  position_from_index(index).unwrap()
                            })
                        },
                        false => None
                    },
                    // No team, meaning the player is a free agent and therefore won't be playing
                    None => Some(ByeStarter {
                        name: player.full_name.clone().unwrap().to_string(),
                        pos:  position_from_index(index).unwrap()
                    })
                },
                None => None
            }
        }).collect()
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

