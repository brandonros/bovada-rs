#![allow(non_snake_case)]

use futures_util::SinkExt;
use futures_util::StreamExt;
use serde::Deserialize;
use websocket_lite::{Message, Opcode};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Price {
    id: String,
    american: String,
    decimal: String,
    fractional: String,
    malay: String,
    indonesian: String,
    hongkong: String
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct HandicapPrice {
    id: String,
    handicap: String,
    american: String,
    decimal: String,
    fractional: String,
    malay: String,
    indonesian: String,
    hongkong: String
}


#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Market {
    id: String,
    descriptionKey: String,
    description: String,
    key: String,
    marketTypeId: String,
    status: String,
    singleOnly: bool,
    notes: String,
    period: Period,
    outcomes: Vec<Outcome>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct HandicapMarket {
    id: String,
    descriptionKey: String,
    description: String,
    key: String,
    marketTypeId: String,
    status: String,
    singleOnly: bool,
    notes: String,
    period: Period,
    outcomes: Vec<HandicapOutcome>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Period {
    id: String,
    description: String,
    abbreviation: String,
    live: bool,
    main: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Outcome {
    id: String,
    description: String,
    status: String,
    r#type: String,
    competitorId: String,
    price: Price,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct HandicapOutcome {
    id: String,
    description: String,
    status: String,
    r#type: String,
    competitorId: String,
    price: HandicapPrice,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Event1 {
    eventId: String,
    target: String,
    r#type: String
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Event2 {
    id: String,
    notes: String,
    singleOnly: bool,
    status: String
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Event3 {
    id: String,
    price: Price
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Event4 {
    eventId: String,
    parentId: String,
    target: String,
    r#type: String
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Event5 {
    id: usize
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Event6 {
    r#type: String,
    eventId: String,
    index: i64,
    target: String
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Event7 {
    id: String,
    status: String
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Event8 {
    id: String,
    status: String,
    description: String
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Event9 {
    id: String,
    numMarkets: usize
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Event10 {
    id: String,
    price: HandicapPrice
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Event11 {
    id: String,
    description: String,
    defaultType: bool,
    alternateType: bool,
    markets: Vec<Market>,
    order: usize,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Event12 {
    r#type: String,
    eventId: String,
    parentId: String,
    mode: String,
    target: String
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Event13 {
    id: String,
    description: String,
    descriptionKey: String,
    key: String,
    marketTypeId: String,
    status: String,
    singleOnly: bool,
    notes: String,
    period: Period,
    outcomes: Vec<HandicapOutcome>
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Event14 {
    id: String,
    description: String,
    status: String,
    r#type: String,
    competitorId: String,
    price: Price
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Event15 {
    id: String,
    description: String,
    defaultType: bool,
    alternateType: bool,
    markets: Vec<HandicapMarket>,
    order: usize,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Event16 {
    id: String,
    link: String,
    status: String,
    startTime: usize,
    live: bool,
    denySameGame: String,
    teaserAllowed: bool,
    notes: String
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Event17 {
    r#type: String,
    eventId: String,
    parentId: String,
    index: i64,
    target: String
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Event18 {
    id: String,
    description: String,
    status: String,
    r#type: String,
    price: HandicapPrice
}

/*#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Event19 {
    id: String,
    description: String,
    r#type: String,
    link: String,
    status: String,
    sport: String,
    startTime: usize,
    live: bool,
    awayTeamFirst: bool,
    denySameGame: String,
    teaserAllowed: bool,
    competitionId: String,
    notes: String,
    numMarkets: usize,
    lastModified: usize,
    competitors: Vec<Competitor>,
    displayGroups: Vec<DisplayGroup>
}*/

fn main() {
    let runtime = tokio::runtime::Builder::new_current_thread().enable_io().build().unwrap();
    runtime.block_on(async {
        // connect
        println!("connecting");
        let subscription_id = uuid::Uuid::new_v4().to_string().to_ascii_uppercase();
        let url = format!("wss://services.bovada.lv/services/sports/subscription/{subscription_id}");
        let ws = websocket_lite::ClientBuilder::new(&url).unwrap().async_connect().await.unwrap();
        println!("connected");
        // split
        let (mut ws_sink, mut ws_stream) = ws.split::<Message>();
        // subscribe
        println!("subscribing");
        let args = std::env::args().collect::<Vec<String>>();
        let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        let event_id = &args[1];
        ws_sink.send(Message::text(format!("SUBSCRIBE|A|/events/{event_id}.{timestamp}?delta=true"))).await.unwrap();
        println!("subscribed");
        // receive
        loop {
            let msg = ws_stream.next().await;
            if msg.is_none() {
                println!("msg.is_none()");
                std::process::exit(1);
            }
            let msg = msg.unwrap();
            if msg.is_err() {
                println!("msg.err() = {}", msg.err().unwrap());
                std::process::exit(1);
            }
            let msg = msg.unwrap();
            if msg.opcode() == Opcode::Ping {
                println!("ping");
                ws_sink.send(Message::pong(msg.into_data())).await.unwrap();
            } else if msg.opcode() == Opcode::Pong {
                println!("pong");
            } else if msg.opcode() == Opcode::Close {
                println!("close");
                std::process::exit(1);   
            } else if msg.opcode() == Opcode::Text {
                let msg_data = msg.as_text().unwrap();
                let events = msg_data.split("|").collect::<Vec<&str>>();
                for event in events {
                    let event1 = serde_json::from_str::<Event1>(event);
                    let event2 = serde_json::from_str::<Event2>(event);
                    let event3 = serde_json::from_str::<Event3>(event);
                    let event4 = serde_json::from_str::<Event4>(event);
                    let event5 = serde_json::from_str::<Event5>(event);
                    let event6 = serde_json::from_str::<Event6>(event);
                    let event7 = serde_json::from_str::<Event7>(event);
                    let event8 = serde_json::from_str::<Event8>(event);
                    let event9 = serde_json::from_str::<Event9>(event);
                    let event10 = serde_json::from_str::<Event10>(event);
                    let event11 = serde_json::from_str::<Event11>(event);
                    let event12 = serde_json::from_str::<Event12>(event);
                    let event13 = serde_json::from_str::<Event13>(event);
                    let event14 = serde_json::from_str::<Event14>(event);
                    let event15 = serde_json::from_str::<Event15>(event);
                    let event16 = serde_json::from_str::<Event16>(event);
                    let event17 = serde_json::from_str::<Event17>(event);
                    let event18 = serde_json::from_str::<Event18>(event);
                    if event1.is_ok() {
                        println!("event1: {}", event);
                    } else if event2.is_ok() {
                        println!("event2: {}", event);
                    } else if event3.is_ok() {
                        println!("event3: {}", event);
                    } else if event4.is_ok() {
                        println!("event4: {}", event);
                    } else if event5.is_ok() {
                        println!("event5: {}", event);
                    } else if event6.is_ok() {
                        println!("event6: {}", event);
                    } else if event7.is_ok() {
                        println!("event7: {}", event);
                    } else if event8.is_ok() {
                        println!("event8: {}", event);
                    } else if event9.is_ok() {
                        println!("event9: {}", event);
                    } else if event10.is_ok() {
                        println!("event10: {}", event);
                    } else if event11.is_ok() {
                        println!("event11: {}", event);
                    } else if event12.is_ok() {
                        println!("event12: {}", event);
                    } else if event13.is_ok() {
                        println!("event13: {}", event);
                    } else if event14.is_ok() {
                        println!("event14: {}", event);
                    } else if event15.is_ok() {
                        println!("event15: {}", event);
                    } else if event16.is_ok() {
                        println!("event16: {}", event);
                    } else if event17.is_ok() {
                        println!("event17: {}", event);
                    } else if event18.is_ok() {
                        println!("event18: {}", event);
                    } else {
                        println!("unk: {}", event);
                    } 
                }
            }
        }
    });
}
