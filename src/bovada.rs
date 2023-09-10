use std::pin::Pin;

use futures_util::{SinkExt, StreamExt};
use websocket_lite::{Message, Opcode};

use crate::{event_type::EventType, structs::SportsEventCoupon, utilities, ws_error::WsError};

pub struct Bovada {
    slug: String,
}

impl Bovada {
    pub fn new(slug: String) -> Bovada {
        Bovada { slug }
    }

    pub async fn get_event_ids(
        &self,
    ) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        // perform http get
        let http_client = reqwest::Client::new();
        let request = http_client.get(format!(
            "https://www.bovada.lv/services/sports/event/coupon/events/A/description/{}?lang=en",
            self.slug
        ));
        let response = request.send().await?;
        assert_eq!(response.status(), reqwest::StatusCode::OK);
        let stringified_response_body = response.text().await?;
        // parse response into json
        let parsed_response_body =
            serde_json::from_str::<Vec<SportsEventCoupon>>(&stringified_response_body)?;
        // pluck event IDs
        Ok(parsed_response_body[0]
            .events
            .iter()
            .map(|event| event.id.clone())
            .collect())
    }

    async fn send_subscribe_message(
        &self,
        ws_sink: &mut Pin<
            Box<impl futures_util::sink::Sink<Message, Error = websocket_lite::Error>>,
        >,
        event_id: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let timestamp = utilities::current_millis()?;
        let message = Message::text(format!(
            "SUBSCRIBE|A|/events/{}.{}?delta=true",
            event_id, timestamp
        ));
        ws_sink.send(message).await
    }

    async fn handle_incoming_messages(
        &self,
        ws_stream: &mut Pin<
            Box<impl futures_util::stream::Stream<Item = Result<Message, websocket_lite::Error>>>,
        >,
        event_id: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        loop {
            // get next message from stream
            let msg = ws_stream
                .next()
                .await
                .ok_or(WsError::StreamEnded)?
                .map_err(WsError::MessageError)?;
            // parse based on opcode
            match msg.opcode() {
                Opcode::Text => {
                    let timestamp = utilities::current_seconds()?;
                    let msg_data = msg.as_text().ok_or(WsError::TextDecodeError)?;
                    let events = msg_data.split('|').collect::<Vec<_>>();
                    for event in events {
                        let event_type = EventType::from(event);
                        println!("{event_id}\t{timestamp}\t{event_type:?}\t{event}");
                    }
                }
                Opcode::Close => {
                    return Err(Box::new(WsError::CloseOpcodeReceived));
                }
                Opcode::Binary => unimplemented!(),
                Opcode::Ping => unimplemented!(),
                Opcode::Pong => unimplemented!(),
            }
        }
    }

    pub async fn create_event_subscription(
        &self,
        event_id: String,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // connect websocket
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
        // split into sink/stream
        let (ws_sink, ws_stream) = ws.split::<Message>();
        let mut ws_sink = Box::pin(ws_sink);
        let mut ws_stream = Box::pin(ws_stream);
        // send subscribe message
        self.send_subscribe_message(&mut ws_sink, &event_id).await?;
        // print header
        println!("event_id\ttimestamp\tevent_type\tevent");
        // receive and process messages
        self.handle_incoming_messages(&mut ws_stream, &event_id)
            .await
    }
}
