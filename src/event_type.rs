use crate::structs::*;

macro_rules! try_parse_event {
    ($event_str:expr, $event_type:ident) => {
        if serde_json::from_str::<$event_type>($event_str).is_ok() {
            return EventType::$event_type;
        }
    };
}

#[derive(Debug)]
pub enum EventType {
    TargetEvent,
    PriceEvent,
    EmptyEvent,
    StatusEvent,
    NumMarketsEvent,
    MarketsEvent,
    OutcomesEvent,
    DisplayGroupsEvent,
}

impl EventType {
    pub fn from(event: &str) -> Self {
        try_parse_event!(event, TargetEvent);
        try_parse_event!(event, PriceEvent);
        try_parse_event!(event, EmptyEvent);
        try_parse_event!(event, StatusEvent);
        try_parse_event!(event, NumMarketsEvent);
        try_parse_event!(event, MarketsEvent);
        try_parse_event!(event, OutcomesEvent);
        try_parse_event!(event, DisplayGroupsEvent);

        panic!("unable to determine event type {}", event)
    }
}
