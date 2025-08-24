use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::time::{SystemTime, UNIX_EPOCH};

pub mod events;
pub mod capture;
pub mod injection;

/// KVM input event types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum KvmEvent {
    /// Mouse movement event
    MouseMove { x: i32, y: i32 },
    /// Mouse button click event  
    MouseClick { button: MouseButton, pressed: bool },
    /// Mouse scroll event
    MouseScroll { delta_x: i32, delta_y: i32 },
    /// Keyboard key event
    KeyPress { key: String, pressed: bool },
    /// Screen size info for coordinate normalization
    ScreenInfo { width: u32, height: u32 },
}

/// Mouse button types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]  
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

/// KVM session message exchanged between client and server
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum KvmMessage {
    /// Request to start KVM session
    StartRequest {
        client_screen_width: u32,
        client_screen_height: u32,
    },
    /// Response to start request
    StartResponse {
        accepted: bool,
        server_screen_width: u32,
        server_screen_height: u32,
        session_id: String,
    },
    /// Input event during active session
    InputEvent {
        session_id: String,
        event: KvmEvent,
        timestamp: u64,
    },
    /// End KVM session
    EndSession {
        session_id: String,
    },
    /// Heartbeat to keep session alive
    Heartbeat {
        session_id: String,
    },
}

/// KVM session state
#[derive(Debug, Clone)]
pub struct KvmSession {
    pub session_id: String,
    pub client_screen_size: (u32, u32),
    pub server_screen_size: (u32, u32),
    pub active: bool,
    pub created_at: SystemTime,
}

impl KvmSession {
    pub fn new(
        session_id: String,
        client_screen_size: (u32, u32),
        server_screen_size: (u32, u32),
    ) -> Self {
        Self {
            session_id,
            client_screen_size,
            server_screen_size,
            active: true,
            created_at: SystemTime::now(),
        }
    }

    pub fn age(&self) -> Result<std::time::Duration> {
        Ok(SystemTime::now().duration_since(self.created_at)?)
    }
}

impl KvmMessage {
    /// Get current timestamp in milliseconds
    pub fn timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }

    /// Create a new input event message
    pub fn input_event(session_id: String, event: KvmEvent) -> Self {
        Self::InputEvent {
            session_id,
            event,
            timestamp: Self::timestamp(),
        }
    }
}

/// Scale coordinates from client screen to server screen
pub fn scale_coordinates(
    x: i32,
    y: i32,
    from_size: (u32, u32),
    to_size: (u32, u32),
) -> (i32, i32) {
    let scale_x = to_size.0 as f64 / from_size.0 as f64;
    let scale_y = to_size.1 as f64 / from_size.1 as f64;
    
    let scaled_x = (x as f64 * scale_x) as i32;
    let scaled_y = (y as f64 * scale_y) as i32;
    
    (scaled_x.max(0), scaled_y.max(0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_coordinates() {
        // Test 1:1 scaling
        assert_eq!(scale_coordinates(100, 200, (1920, 1080), (1920, 1080)), (100, 200));
        
        // Test 2:1 scaling
        assert_eq!(scale_coordinates(100, 200, (1920, 1080), (3840, 2160)), (200, 400));
        
        // Test 1:2 scaling  
        assert_eq!(scale_coordinates(200, 400, (1920, 1080), (960, 540)), (100, 200));
        
        // Test negative coordinates clamped to 0
        assert_eq!(scale_coordinates(-10, -20, (1920, 1080), (1920, 1080)), (0, 0));
    }

    #[test]
    fn test_kvm_message_serialization() {
        let msg = KvmMessage::StartRequest {
            client_screen_width: 1920,
            client_screen_height: 1080,
        };
        
        let json = serde_json::to_string(&msg).unwrap();
        let deserialized: KvmMessage = serde_json::from_str(&json).unwrap();
        
        match deserialized {
            KvmMessage::StartRequest { client_screen_width, client_screen_height } => {
                assert_eq!(client_screen_width, 1920);
                assert_eq!(client_screen_height, 1080);
            },
            _ => panic!("Wrong message type"),
        }
    }

    #[test]
    fn test_kvm_event_serialization() {
        let event = KvmEvent::MouseMove { x: 100, y: 200 };
        let json = serde_json::to_string(&event).unwrap();
        let deserialized: KvmEvent = serde_json::from_str(&json).unwrap();
        
        assert_eq!(event, deserialized);
    }
}