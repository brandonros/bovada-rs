#![allow(non_snake_case)]

use serde::Deserialize;

#[derive(Deserialize)]
pub enum Status {
    O,
    D,
    S,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Price {
    pub id: String,
    pub handicap: Option<String>,
    pub american: String,
    pub decimal: String,
    pub fractional: String,
    pub malay: String,
    pub indonesian: String,
    pub hongkong: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Market {
    pub id: String,
    pub descriptionKey: String,
    pub description: String,
    pub key: String,
    pub marketTypeId: String,
    pub status: Status,
    pub singleOnly: bool,
    pub notes: String,
    pub period: Period,
    pub outcomes: Vec<Outcome>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Period {
    pub id: String,
    pub description: String,
    pub abbreviation: String,
    pub live: bool,
    pub main: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Outcome {
    pub id: String,
    pub description: String,
    pub status: Status,
    pub r#type: String,
    pub competitorId: Option<String>,
    pub price: Price,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Competitor {
    pub id: String,
    pub name: String,
    pub home: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DisplayGroup {
    pub id: String,
    pub description: String,
    pub defaultType: bool,
    pub alternateType: bool,
    pub markets: Vec<Market>,
    pub order: i64,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EmptyEvent {
    pub id: usize,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NumMarketsEvent {
    pub id: String,
    pub numMarkets: usize,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TargetEvent {
    pub eventId: String,
    pub r#type: String,
    pub target: String,
    pub parentId: Option<String>,
    pub index: Option<i64>,
    pub mode: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PriceEvent {
    pub id: String,
    pub price: Price,
    pub description: Option<String>,
    pub status: Option<String>,
    pub r#type: Option<String>,
    pub competitorId: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StatusEvent {
    pub id: String,
    pub status: Status,
    pub description: Option<String>,
    pub notes: Option<String>,
    pub singleOnly: Option<bool>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MarketsEvent {
    pub id: String,
    pub description: String,
    pub defaultType: bool,
    pub alternateType: bool,
    pub markets: Vec<Market>,
    pub order: usize,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OutcomesEvent {
    pub id: String,
    pub description: String,
    pub descriptionKey: String,
    pub key: String,
    pub marketTypeId: String,
    pub status: Status,
    pub singleOnly: bool,
    pub notes: String,
    pub period: Period,
    pub outcomes: Vec<Outcome>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DisplayGroupsEvent {
    pub id: String,
    pub description: String,
    pub r#type: String,
    pub link: String,
    pub status: Status,
    pub sport: String,
    pub startTime: i64,
    pub live: bool,
    pub awayTeamFirst: bool,
    pub denySameGame: String,
    pub teaserAllowed: bool,
    pub competitionId: String,
    pub notes: String,
    pub numMarkets: i64,
    pub lastModified: i64,
    pub competitors: Vec<Competitor>,
    pub displayGroups: Vec<DisplayGroup>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Path {
    pub id: String,
    pub link: String,
    pub description: String,
    pub r#type: String,
    pub sportCode: String,
    pub order: i64,
    pub leaf: bool,
    pub current: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Event {
    pub id: String
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SportsEventCoupon {
    pub path: Vec<Path>,
    pub events: Vec<DisplayGroupsEvent>
}
