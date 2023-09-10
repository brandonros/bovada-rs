mod structs;

use std::pin::Pin;

use futures_util::{SinkExt, StreamExt};
use structs::*;
use websocket_lite::{Message, Opcode};

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

fn main() -> anyhow::Result<()> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()?;
    runtime.block_on(bovada_subscription())?;
    Ok(())
}

async fn bovada_subscription() -> anyhow::Result<()> {
    // Connect
    let subscription_id = uuid::Uuid::new_v4().to_string().to_ascii_uppercase();
    let url = format!(
        "wss://services.bovada.lv/services/sports/subscription/{}",
        subscription_id
    );
    let ws_client = websocket_lite::ClientBuilder::new(&url)?;
    let ws = ws_client
        .async_connect()
        .await
        .map_err(|err| anyhow::anyhow!(err))?;

    // Split the WebSocket
    let (ws_sink, ws_stream) = ws.split::<Message>();
    let mut ws_sink = Box::pin(ws_sink);
    let mut ws_stream = Box::pin(ws_stream);

    // Subscribe
    subscribe_to_event(&mut ws_sink).await?;

    // Print header
    println!("timestamp\tevent_type\tevent");

    // Receive and process messages
    handle_messages(&mut ws_stream).await
}

async fn subscribe_to_event(
    ws_sink: &mut Pin<Box<impl futures_util::sink::Sink<Message, Error = websocket_lite::Error>>>,
) -> anyhow::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    let timestamp = current_millis()?;
    let event_id = &args
        .get(1)
        .ok_or(anyhow::anyhow!("Event ID argument missing"))?;
    ws_sink
        .send(Message::text(format!(
            "SUBSCRIBE|A|/events/{}.{}?delta=true",
            event_id, timestamp
        )))
        .await
        .map_err(|err| anyhow::anyhow!(err))
}

async fn handle_messages(
    ws_stream: &mut Pin<
        Box<impl futures_util::stream::Stream<Item = Result<Message, websocket_lite::Error>>>,
    >,
) -> anyhow::Result<()> {
    loop {
        let msg = ws_stream
            .next()
            .await
            .ok_or(anyhow::anyhow!("Stream ended prematurely"))?
            .map_err(|err| anyhow::anyhow!(err))?;
        match msg.opcode() {
            Opcode::Text => {
                let timestamp = current_seconds()?;
                let msg_data = msg
                    .as_text()
                    .ok_or(anyhow::anyhow!("Failed to get message data as text"))?;
                let events = msg_data.split('|').collect::<Vec<_>>();
                for event in events {
                    let event_type = determine_event_type(event);
                    println!("{timestamp}\t{event_type:?}\t{event}");
                }
            }
            Opcode::Binary => unimplemented!(),
            Opcode::Ping => unimplemented!(),
            Opcode::Pong => unimplemented!(),
            Opcode::Close => {
                return Err(anyhow::anyhow!("Received close opcode"));
            }
        }
    }
}

fn current_millis() -> Result<u128, std::time::SystemTimeError> {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
}

fn current_seconds() -> Result<u64, std::time::SystemTimeError> {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
}
