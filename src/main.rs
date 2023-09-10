mod structs;

use futures_util::{SinkExt, StreamExt};
use websocket_lite::{Message, Opcode};
use structs::*;

#[derive(Debug)]
enum EventType {
    Event1,
    Event2,
    Event3,
    Event4,
    Event5,
    Event6,
    Event7,
    Event8,
    Event9,
    Event10,
    Event11,
    Event12,
    Event13,
    Event14,
    Event15,
}

fn determine_event_type(event: &str) -> EventType {
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
    if event1.is_ok() {
        return EventType::Event1;
    } else if event2.is_ok() {
        return EventType::Event2;
    } else if event3.is_ok() {
        return EventType::Event3;
    } else if event4.is_ok() {
        return EventType::Event4;
    } else if event5.is_ok() {
        return EventType::Event5;
    } else if event6.is_ok() {
        return EventType::Event6;
    } else if event7.is_ok() {
        return EventType::Event7;
    } else if event8.is_ok() {
        return EventType::Event8;
    } else if event9.is_ok() {
        return EventType::Event9;
    } else if event10.is_ok() {
        return EventType::Event10;
    } else if event11.is_ok() {
        return EventType::Event11;
    } else if event12.is_ok() {
        return EventType::Event12;
    } else if event13.is_ok() {
        return EventType::Event13;
    } else if event14.is_ok() {
        return EventType::Event14;
    } else if event15.is_ok() {
        return EventType::Event15;
    } else {
        panic!("unable to determine event type");
    } 
}

fn main() {
    let runtime = tokio::runtime::Builder::new_current_thread().enable_io().build().unwrap();
    runtime.block_on(async {
        // connect
        println!("connecting");
        let subscription_id = uuid::Uuid::new_v4().to_string().to_ascii_uppercase();
        let url = format!("wss://services.bovada.lv/services/sports/subscription/{subscription_id}");
        let ws_client = websocket_lite::ClientBuilder::new(&url).unwrap();
        let ws = ws_client.async_connect().await.unwrap();
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
                        println!("{timestamp}: {event_type:?} {event}");
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
