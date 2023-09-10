mod structs;
mod event_type;
mod ws_error;

use std::pin::Pin;

use futures_util::{SinkExt, StreamExt};
use websocket_lite::{Message, Opcode};
use crate::{event_type::EventType, ws_error::WsError, structs::SportsEventCoupon};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>>  {
    let args = std::env::args().collect::<Vec<String>>();
    let slug = args
        .get(1)
        .ok_or(WsError::InvalidArgumentsError)?;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .build()?;
    runtime.block_on(async {
        let event_ids = get_event_ids(&slug).await?;
        let handles = event_ids.into_iter().map(|event_id| {
            tokio::task::spawn(create_bovada_event_subscription(event_id.clone()))
        }).collect::<Vec<_>>();
        futures::future::join_all(handles).await;
        Ok(())
    })
}

async fn get_event_ids(slug: &str) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let http_client = reqwest::Client::new();
    let request = http_client.get(format!("https://www.bovada.lv/services/sports/event/coupon/events/A/description/{slug}?lang=en"));
    let response = request.send().await?;
    assert_eq!(response.status(), reqwest::StatusCode::OK);
    let stringified_response_body = response.text().await?;
    let parsed_response_body = serde_json::from_str::<Vec<SportsEventCoupon>>(&stringified_response_body)?;
    Ok(parsed_response_body[0].events.iter().map(|event| event.id.clone()).collect())
}

async fn create_bovada_event_subscription(event_id: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>>  {
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
        .map_err(|err| WsError::ConnectError(err))?;
    // Split the WebSocket
    let (ws_sink, ws_stream) = ws.split::<Message>();
    let mut ws_sink = Box::pin(ws_sink);
    let mut ws_stream = Box::pin(ws_stream);

    // Subscribe
    subscribe_to_event(&mut ws_sink, &event_id).await?;

    // Print header
    println!("event_id\ttimestamp\tevent_type\tevent");

    // Receive and process messages
    handle_messages(&mut ws_stream, &event_id).await
}

async fn subscribe_to_event(
    ws_sink: &mut Pin<Box<impl futures_util::sink::Sink<Message, Error = websocket_lite::Error>>>,
    event_id: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>>  {
    let timestamp = current_millis()?;
    ws_sink
        .send(Message::text(format!(
            "SUBSCRIBE|A|/events/{}.{}?delta=true",
            event_id, timestamp
        )))
        .await
}

async fn handle_messages(
    ws_stream: &mut Pin<
        Box<impl futures_util::stream::Stream<Item = Result<Message, websocket_lite::Error>>>,
    >,
    event_id: &str
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    loop {
        let msg = ws_stream
            .next()
            .await
            .ok_or(WsError::StreamEnded)?
            .map_err(WsError::MessageError)?;
        match msg.opcode() {
            Opcode::Text => {
                let timestamp = current_seconds()?;
                let msg_data = msg
                    .as_text()
                    .ok_or(WsError::TextDecodeError)?;
                let events = msg_data.split('|').collect::<Vec<_>>();
                for event in events {
                    let event_type = EventType::from(event);
                    println!("{event_id}\t{timestamp}\t{event_type:?}\t{event}");
                }
            }
            Opcode::Binary => unimplemented!(),
            Opcode::Ping => unimplemented!(),
            Opcode::Pong => unimplemented!(),
            Opcode::Close => {
                return Err(Box::new(WsError::CloseOpcodeReceived));
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
