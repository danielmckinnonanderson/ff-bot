use std::collections::HashMap;
use std::fmt;
use http::Version;
use serde::Deserialize;

pub type LeagueId = String;

#[derive(Debug, Deserialize)]
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
    pub league_id: String,
    pub last_transation_id: Option<String>,
    pub last_read_id: Option<String>,
    pub last_pinned_message_id: Option<String>,
    pub las_message_time: u64,
    pub last_message_text_mape: Option<String>,
    pub last_message_id: Option<String>,
    pub last_message_attachment: Option<String>,
    pub last_author_is_bot: Option<bool>,
    pub last_author_id: Option<String>,
    pub last_author_display_name: Option<String>,
    pub last_author_avatar: Option<String>,
    pub group_id: Option<String>,
    pub draft_id: Option<String>,
    pub display_order: u8,
    pub company_id: Option<String>,
    pub bracket_id: Option<String>,
    pub avatar: String,
}

#[derive(Debug, Deserialize)]
pub struct LeagueMetadata {
    pub latest_league_winner_roster_id: Option<String>,
    pub keeper_deadline: String,
    pub auto_contineu: String,
}

#[derive(Debug, Deserialize)]
pub struct LeagueSettings {
    pub daily_waivers_last_run: u16,
    pub reserve_allow_cov: u8,
    pub reserve_slots: u8,
    pub leg: u8,
    pub offseason_adds: u8,
    pub bench_lock: u8, 
    pub trade_review_days: u8,
    pub league_average_match: u8,
    pub waiver_type: u8,
    pub max_keepers: u8,
    #[serde(rename(serialize = "type", deserialize = "_type"))]
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

#[derive(Debug, serde::Deserialize)]
pub struct ScoringSettings {
    pub st_ff: i8,
    pub pts_allow_7_13: i8,
    pub def_st_ff: i8,
    pub rec_yd: f64,
    pub fum_rec_td: i8,
    pub pts_allow_35p: i8,
    pub pts_allow_28_34: i8,
    pub fum: i8,
    pub rush_yd: f64,
    pub pass_td: i8,
    pub blk_kick: i8,
    pub pass_yd: f64,
    pub safe: i8,
    pub def_td: i8,
    pub fgm_50p: i8,
    pub def_st_td: i8,
    pub fum_rec: i8,
    pub rush_2pt: i8,
    pub xpm: i8,
    pub pts_allow_21_27: i8,
    pub fgm_20_29: i8,
    pub pts_allow_1_6: i8,
    pub fum_lost: i8,
    pub def_st_fum_rec: i8,
    pub int: i8,
    pub def_kr_td: i8,
    pub fgm_0_19: i8,
    pub pts_allow_14_20: i8,
    pub rec: i8,
    pub ff: i8,
    pub fgmiss: i8,
    pub st_fum_rec: i8,
    pub rec_2pt: i8,
    pub def_pr_td: i8,
    pub rush_td: i8,
    pub xpmiss: i8,
    pub fgm_30_39: i8,
    pub rec_td: i8,
    pub st_td: i8,
    pub pass_2pt: i8,
    pub pts_allow_0: i8,
    pub pass_int: i8,
    pub fgm_40_49: i8,
    pub sack: i8,
}

#[derive(Debug, Deserialize)]
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

// TODO!
#[derive(Debug)]
pub struct Error {}

pub struct Client {
    base_url: String,
    client: reqwest::blocking::Client
}

impl Client {
    pub fn new() -> Self {
        Client {
            base_url: String::from("https://api.sleeper.app/v1"),
            client: reqwest::blocking::Client::builder()
                .https_only(true)
                .build()
                .unwrap()
        }
    }

    pub fn get_league(&self, id: &str) -> Result<League, Error> {
        let url = format!("{}/league/{}", &self.base_url, &id);

        let res = match self.client.get(&url).send() {
            Ok(res) => res,
            Err(_) => {
                eprintln!("Error while sending GET request to '{}'", &url);
                return Result::Err(Error {});
            }
        };

        let league: League = match res.json() {
            Ok(league) => league,
            Err(_) => {
                eprintln!("Error while deserializing League from response body");
                return Result::Err(Error {});
            }
        };

        Ok(league)
    }
}

mod tests {
    #[test]
    fn test_sleeper_client_works() -> () {
        let c = super::Client::new();
    }
}

