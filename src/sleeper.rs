use std::collections::HashMap;

use serde::Deserialize;
use thiserror::Error;

pub type LeagueId = String;
pub type PlayerId = String;

#[derive(Debug, Deserialize, serde::Serialize)]
pub struct League {
    pub total_rosters: u8,
    pub status: String,
    pub sport: String,
    pub shard: u32,
    pub settings: LeagueSettings,
    pub season_type: String,
    pub season: String,
    pub scoring_settings: ScoringSettings,
    pub roster_positions: Vec<RosterPosition>,
    pub previous_league_id: String,
    pub name: String,
    pub metadata: LeagueMetadata,
    pub loser_bracket_id: Option<String>,
    pub league_id: LeagueId,
    pub last_transation_id: Option<String>,
    pub last_read_id: Option<String>,
    pub last_pinned_message_id: Option<String>,
    pub last_message_time: u64,
    pub last_message_text_mape: Option<String>,
    pub last_message_id: Option<String>,
    pub last_message_attachment: Option<String>,
    pub last_author_is_bot: Option<bool>,
    pub last_author_id: Option<String>,
    pub last_author_display_name: Option<String>,
    pub last_author_avatar: Option<String>,
    pub group_id: Option<String>,
    pub draft_id: Option<String>,
    pub company_id: Option<String>,
    pub bracket_id: Option<String>,
    pub avatar: String,
}

#[derive(Debug, Deserialize, serde::Serialize)]
pub struct LeagueMetadata {
    pub latest_league_winner_roster_id: Option<String>,
    pub keeper_deadline: String,
    pub auto_continue: String,
}

#[derive(Debug, Deserialize, serde::Serialize)]
pub struct LeagueSettings {
    pub daily_waivers_last_ran: u16,
    pub reserve_allow_cov: u8,
    pub reserve_slots: u8,
    pub leg: u8,
    pub offseason_adds: u8,
    pub bench_lock: u8, 
    pub trade_review_days: u8,
    pub league_average_match: u8,
    pub waiver_type: u8,
    pub max_keepers: u8,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub _type: u8,
    pub pick_trading: u8,
    pub disable_trades: u8,
    pub daily_waivers: u8,
    pub taxi_years: u8,
    pub trade_deadline: u8,
    pub veto_show_votes: u8,
    pub reserve_allow_sus: u8,
    pub reserve_allow_out: u8,
    pub playoff_round_type: u8,
    pub waiver_day_of_week: u8,
    pub taxi_allow_vets: u8,
    pub reserve_allow_dnr: u8,
    pub veto_auto_poll: u8,
    pub commissioner_direct_invite: u8,
    pub reserve_allow_doubtful: u8,
    pub waiver_clear_days: u8,
    pub playoff_week_start: u8,
    pub daily_waivers_days: u16,
    pub taxi_slots: u8,
    pub playoff_type: u8,
    pub daily_waivers_hour: u32,
    pub num_teams: u8,
    pub squads: u8,
    pub veto_votes_needed: u8,
    pub playoff_teams: u8,
    pub playoff_seed_type: u8,
    pub start_week: u8,
    pub reserve_allow_na: u8,
    pub draft_rounds: u8,
    pub taxi_deadline: u8,
    pub capacity_override: u8,
    pub disable_adds: u8,
    pub waiver_budget: u8,
    pub best_ball: u8
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ScoringSettings {
    pub st_ff: f64,
    pub pts_allow_7_13: f64,
    pub def_st_ff: f64,
    pub rec_yd: f64,
    pub fum_rec_td: f64,
    pub pts_allow_35p: f64,
    pub pts_allow_28_34: f64,
    pub fum: f64,
    pub rush_yd: f64,
    pub pass_td: f64,
    pub blk_kick: f64,
    pub pass_yd: f64,
    pub safe: f64,
    pub def_td: f64,
    pub fgm_50p: f64,
    pub def_st_td: f64,
    pub fum_rec: f64,
    pub rush_2pt: f64,
    pub xpm: f64,
    pub pts_allow_21_27: f64,
    pub fgm_20_29: f64,
    pub pts_allow_1_6: f64,
    pub fum_lost: f64,
    pub def_st_fum_rec: f64,
    pub int: f64,
    pub def_kr_td: f64,
    pub fgm_0_19: f64,
    pub pts_allow_14_20: f64,
    pub rec: f64,
    pub ff: f64,
    pub fgmiss: f64,
    pub st_fum_rec: f64,
    pub rec_2pt: f64,
    pub def_pr_td: f64,
    pub rush_td: f64,
    pub xpmiss: f64,
    pub fgm_30_39: f64,
    pub rec_td: f64,
    pub st_td: f64,
    pub pass_2pt: f64,
    pub pts_allow_0: f64,
    pub pass_int: f64,
    pub fgm_40_49: f64,
    pub sack: f64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum RosterPosition {
    QB,
    RB,
    WR,
    TE,
    FLEX,
    K,
    DEF,
    BN,
    IDP
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Roster {
    pub taxi: Option<String>,
    pub starters: Vec<String>,
    pub settings: RosterSettings,
    pub roster_id: u8,
    pub reserve: Option<String>,
    pub players: Vec<String>,
    pub player_map: Option<PlayerId>,
    pub owner_id: String,
    pub metadata: Option<HashMap<String, String>>,
    pub league_id: LeagueId,
    pub keepers: Option<Vec<String>>,
    pub co_owners: Option<Vec<String>>
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct RosterSettings {
    pub wins: u8,
    pub waiver_position: u8,
    pub waiver_budget_used: u8,
    pub total_moves: u16,
    pub ties: u8,
    pub losses: u8,
    pub fpts: u8,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct SleeperUser {
    pub user_id: String,
    pub username: Option<String>,
    pub settings: Option<HashMap<String, String>>,
    // TODO - metadata has one important field "team_name that should be communicated"
    pub metadata: Option<HashMap<String, String>>, 
    pub is_owner: bool,
    pub is_bot: bool,
    pub league_id: LeagueId,
    pub display_name: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Error, Debug)]
pub enum SleeperError {
    #[error("could not deserialize Sleeper response into provided type")]
    DeserializationError(String),

    #[error("request to Sleeper failed with status code {0}")]
    NetworkError(http::StatusCode)
}

pub struct Client {
    base_url: String,
    client: reqwest::Client
}

impl Client {
    pub fn new() -> Self {
        Client {
            base_url: String::from("https://api.sleeper.app/v1"),
            client: reqwest::Client::builder()
                .https_only(true)
                .build()
                .unwrap()
        }
    }

    pub async fn get_league(&self, id: &str) -> Result<League, SleeperError> {
        let url = format!("{}/league/{}", &self.base_url, &id);

        let res = match self.client.get(&url).send().await {
            Ok(res) => res,
            Err(e) => {
                return Result::Err(SleeperError::NetworkError(e.status().unwrap()));
            }
        };

        let league: League = match res.json::<League>().await {
            Ok(league) => league,
            Err(_) => {
                return Result::Err(SleeperError::DeserializationError(String::from("League")));
            }
        };

        Ok(league)
    }

    pub async fn get_rosters(&self, league_id: &str) -> Result<Vec<Roster>, SleeperError> {
        let url = format!("{}/league/{}/rosters", &self.base_url, &league_id);

        let res = match self.client.get(&url).send().await {
            Ok(res) => res,
            Err(e) => {
                return Result::Err(SleeperError::NetworkError(e.status().unwrap()));
            }
        };

        let rosters: Vec<Roster> = match res.json().await {

            Ok(rost) => rost,
            Err(_) => {
                return Result::Err(SleeperError::DeserializationError(String::from("Roster")));
            }
        };

        Ok(rosters)
    }

    pub async fn get_users_in_league(&self, league_id: &str) -> Result<Vec<SleeperUser>, SleeperError> {
        let url = format!("{}/league/{}/users", &self.base_url, &league_id);

        let res = match self.client.get(&url).send().await {
            Ok(res) => res,
            Err(e) => {
                return Result::Err(SleeperError::NetworkError(e.status().unwrap()));
            }
        };

        let users: Vec<SleeperUser> = match res.json().await {
            Ok(users) => users,
            Err(_) => {
                return Result::Err(SleeperError::DeserializationError(String::from("SleeperUser")));
            }
        };

        Ok(users)
    }
}

