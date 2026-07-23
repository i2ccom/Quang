//! Shared event bus — WebSocket + Signal-driven event distribution.
//!
//! Provides a reactive event bus that bridges server-sent events (SSE)
//! to Dioxus Signals, enabling live-updating UI across all modules.

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

/// A typed event envelope received from the QuangHub event bus.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusEvent {
    pub id: String,
    pub topic: String,
    pub event_type: String,
    pub payload: serde_json::Value,
    pub timestamp: String,
}

/// Reactive event bus signal.
///
/// Components subscribe to topics and reactively re-render when
/// matching events arrive via SSE or WebSocket.
#[derive(Clone)]
pub struct EventBusSignal {
    /// All events, keyed by topic
    pub events: Signal<HashMap<String, Vec<BusEvent>>>,
    /// Latest event per topic
    pub latest: Signal<HashMap<String, BusEvent>>,
    /// Connection status
    pub connected: Signal<bool>,
}

impl EventBusSignal {
    pub fn new() -> Self {
        Self {
            events: Signal::new(HashMap::new()),
            latest: Signal::new(HashMap::new()),
            connected: Signal::new(false),
        }
    }

    /// Connect to an SSE stream for a topic.
    #[cfg(target_arch = "wasm32")]
    pub fn connect(&self, topic: &str, url: &str) {
        let topic = topic.to_string();
        let mut events = self.events;
        let mut latest = self.latest;
        let mut connected = self.connected;

        let endpoint = format!("{}/api/events/subscribe?topic={}", url, topic);
        let event_source =
            web_sys::EventSource::new(&endpoint).expect("Failed to create EventSource");

        let onmessage =
            Closure::<dyn FnMut(web_sys::MessageEvent)>::new(move |msg: web_sys::MessageEvent| {
                if let Ok(data) = msg.data().dyn_into::<js_sys::JsString>() {
                    let data_str: String = data.into();
                    if let Ok(event) = serde_json::from_str::<BusEvent>(&data_str) {
                        let mut map = events.write();
                        map.entry(topic.clone()).or_default().push(event.clone());
                        latest.write().insert(topic.clone(), event);
                    }
                }
            });

        event_source.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
        connected.set(true);
        onmessage.forget();
    }

    /// Get all events for a topic.
    pub fn events_for(&self, topic: &str) -> Vec<BusEvent> {
        self.events.read().get(topic).cloned().unwrap_or_default()
    }

    /// Get the latest event for a topic.
    pub fn latest_for(&self, topic: &str) -> Option<BusEvent> {
        self.latest.read().get(topic).cloned()
    }

    /// Whether the event bus is connected.
    pub fn is_connected(&self) -> bool {
        *self.connected.read()
    }
}

impl Default for EventBusSignal {
    fn default() -> Self {
        Self::new()
    }
}
