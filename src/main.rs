mod structs;

use futures_util::{SinkExt, StreamExt};
use websocket_lite::{Message, Opcode};
use structs::*;

#[derive(Debug)]
enum EventType {
    TargetEvent,
    PriceEvent,
    EmptyEvent,
    StatusEvent,
    NumMarketsEvent,
    MarketsEvent,
    OutcomesEvent,
    DisplayGroupsEvent,
}

fn determine_event_type(event: &str) -> EventType {
    if serde_json::from_str::<TargetEvent>(event).is_ok() {
        return EventType::TargetEvent;
    } else if serde_json::from_str::<PriceEvent>(event).is_ok() {
        return EventType::PriceEvent;
    } else if serde_json::from_str::<EmptyEvent>(event).is_ok() {
        return EventType::EmptyEvent;
    } else if serde_json::from_str::<StatusEvent>(event).is_ok() {
        return EventType::StatusEvent;
    } else if serde_json::from_str::<NumMarketsEvent>(event).is_ok() {
        return EventType::NumMarketsEvent;
    } else if serde_json::from_str::<MarketsEvent>(event).is_ok() {
        return EventType::MarketsEvent;
    } else if serde_json::from_str::<OutcomesEvent>(event).is_ok() {
        return EventType::OutcomesEvent;
    } else if serde_json::from_str::<DisplayGroupsEvent>(event).is_ok() {
        return EventType::DisplayGroupsEvent;
    } else {
        panic!("unable to determine event type {event}");
    } 
}

fn main() {
    let runtime = tokio::runtime::Builder::new_current_thread().enable_io().build().unwrap();
    runtime.block_on(async {
        // connect
        let subscription_id = uuid::Uuid::new_v4().to_string().to_ascii_uppercase();
        let url = format!("wss://services.bovada.lv/services/sports/subscription/{subscription_id}");
        let ws_client = websocket_lite::ClientBuilder::new(&url).unwrap();
        let ws = ws_client.async_connect().await.unwrap();
        // split
        let (mut ws_sink, mut ws_stream) = ws.split::<Message>();
        // subscribe
        let args = std::env::args().collect::<Vec<String>>();
        let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        let event_id = &args[1];
        ws_sink.send(Message::text(format!("SUBSCRIBE|A|/events/{event_id}.{timestamp}?delta=true"))).await.unwrap();
        // print header
        println!("timestamp\tevent_type\tevent");
        // receive
        loop {
            let msg = ws_stream.next().await;
            if msg.is_none() {
                panic!("msg.is_none()");
            }
            let msg = msg.unwrap();
            if msg.is_err() {
                panic!("msg.err() = {}", msg.err().unwrap());
            }
            let msg = msg.unwrap();
            match msg.opcode() {
                Opcode::Text => {
                    let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
                    let msg_data = msg.as_text().unwrap();
                    let events = msg_data.split("|").collect::<Vec<&str>>();
                    for event in events {
                        let event_type = determine_event_type(event);
                        println!("{timestamp}\t{event_type:?}\t{event}");
                    }
                },
                Opcode::Binary => unimplemented!(),
                Opcode::Close => {
                    panic!("close");
                },
                Opcode::Ping => {
                    println!("pong");
                },
                Opcode::Pong => {
                    println!("ping");
                    ws_sink.send(Message::pong(msg.into_data())).await.unwrap();
                },
            }
        }
    });
}
