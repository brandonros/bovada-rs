#![allow(non_snake_case)]

use serde::Deserialize;

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
    pub hongkong: String
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Market {
    pub id: String,
    pub descriptionKey: String,
    pub description: String,
    pub key: String,
    pub marketTypeId: String,
    pub status: String,
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
    pub status: String,
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
pub struct Event1 {
    pub eventId: String,
    pub target: String,
    pub r#type: String
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Event2 {
    pub id: String,
    pub notes: String,
    pub singleOnly: bool,
    pub status: String
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Event3 {
    pub id: String,
    pub price: Price
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Event4 {
    pub eventId: String,
    pub parentId: String,
    pub target: String,
    pub r#type: String
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Event5 {
    pub id: usize
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Event6 {
    pub r#type: String,
    pub eventId: String,
    pub index: i64,
    pub target: String
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Event7 {
    pub id: String,
    pub status: String
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Event8 {
    pub id: String,
    pub status: String,
    pub description: String
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Event9 {
    pub id: String,
    pub numMarkets: usize
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Event10 {
    pub id: String,
    pub description: String,
    pub defaultType: bool,
    pub alternateType: bool,
    pub markets: Vec<Market>,
    pub order: usize,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Event11 {
    pub r#type: String,
    pub eventId: String,
    pub parentId: String,
    pub mode: String,
    pub target: String
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Event12 {
    pub id: String,
    pub description: String,
    pub descriptionKey: String,
    pub key: String,
    pub marketTypeId: String,
    pub status: String,
    pub singleOnly: bool,
    pub notes: String,
    pub period: Period,
    pub outcomes: Vec<Outcome>
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Event13 {
    pub id: String,
    pub description: String,
    pub status: String,
    pub r#type: String,
    pub competitorId: Option<String>,
    pub price: Price
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Event14 {
    pub r#type: String,
    pub eventId: String,
    pub parentId: String,
    pub index: i64,
    pub target: String
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Event15 {
    pub id: String,
    pub description: String,
    pub r#type: String,
    pub link: String,
    pub status: String,
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