use serde::{Deserialize, Serialize};
use std::{
    any::{Any, TypeId},
    boxed::Box,
    fmt::Debug,
};

#[repr(u8)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "op")]
pub enum OpCodes {
    EventCreate = 0,
    Hello = 1,
    Identify = 2,
    HeartBeat = 3,
    HeartBeatAck = 4,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebsocketMessage {
    pub op: OpCodes,
    pub event: Option<Box<dyn WebsocketEvent + Send + Sync>>,
}

#[typetag::serde]
pub trait WebsocketEvent: erased_serde::Serialize + Debug + Send + Sync {
    fn as_any(&self) -> &dyn Any;
}

impl WebsocketMessage {
    pub fn downcast_event<T: 'static>(&self) -> Option<&T> {
        let Some(event) = &self.event else {
            return None;
        };
        let event_any = event.as_any();
        if event_any.type_id() != TypeId::of::<T>() {
            // The event type did not match the type we wanted
            return None;
        }
        event_any.downcast_ref::<T>()
    }
}
