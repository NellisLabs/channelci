use std::any::Any;

use serde::{Deserialize, Serialize};

use crate::opcodes::WebsocketEvent;

macro_rules! impl_websocket_event {
    ($name:ident) => {
        #[typetag::serde]
        impl WebsocketEvent for $name {
            fn as_any(&self) -> &dyn Any {
                self
            }
        }
    };
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hello {
    pub heartbeat: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Identify {
    pub name: String,
    pub token: String,
}

impl_websocket_event!(Hello);
impl_websocket_event!(Identify);
