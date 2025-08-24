use super::{KvmEvent, MouseButton};
use anyhow::Result;

/// Event handler trait for KVM events
pub trait EventHandler {
    /// Handle a KVM event
    fn handle_event(&mut self, event: KvmEvent) -> Result<()>;
    
    /// Check if handler is ready to process events
    fn is_ready(&self) -> bool;
}

/// Basic event handler that logs events (for testing)
#[derive(Default)]
pub struct LoggingEventHandler {
    pub events: Vec<KvmEvent>,
}

impl EventHandler for LoggingEventHandler {
    fn handle_event(&mut self, event: KvmEvent) -> Result<()> {
        tracing::debug!("KVM Event: {:?}", event);
        self.events.push(event);
        Ok(())
    }
    
    fn is_ready(&self) -> bool {
        true
    }
}

/// Convert platform-specific coordinates to normalized coordinates (0.0-1.0)
pub fn normalize_coordinates(x: i32, y: i32, screen_width: u32, screen_height: u32) -> (f64, f64) {
    let norm_x = (x as f64 / screen_width as f64).clamp(0.0, 1.0);
    let norm_y = (y as f64 / screen_height as f64).clamp(0.0, 1.0);
    (norm_x, norm_y)
}

/// Convert normalized coordinates back to platform-specific coordinates
pub fn denormalize_coordinates(norm_x: f64, norm_y: f64, screen_width: u32, screen_height: u32) -> (i32, i32) {
    let x = (norm_x * screen_width as f64) as i32;
    let y = (norm_y * screen_height as f64) as i32;
    (x.max(0).min(screen_width as i32 - 1), y.max(0).min(screen_height as i32 - 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_coordinates() {
        // Center of 1920x1080 screen
        let (norm_x, norm_y) = normalize_coordinates(960, 540, 1920, 1080);
        assert!((norm_x - 0.5).abs() < 0.01);
        assert!((norm_y - 0.5).abs() < 0.01);
        
        // Top-left corner
        let (norm_x, norm_y) = normalize_coordinates(0, 0, 1920, 1080);
        assert_eq!(norm_x, 0.0);
        assert_eq!(norm_y, 0.0);
        
        // Bottom-right corner
        let (norm_x, norm_y) = normalize_coordinates(1919, 1079, 1920, 1080);
        assert!((norm_x - 1.0).abs() < 0.01);
        assert!((norm_y - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_denormalize_coordinates() {
        // Center of screen
        let (x, y) = denormalize_coordinates(0.5, 0.5, 1920, 1080);
        assert_eq!(x, 960);
        assert_eq!(y, 540);
        
        // Top-left corner
        let (x, y) = denormalize_coordinates(0.0, 0.0, 1920, 1080);
        assert_eq!(x, 0);
        assert_eq!(y, 0);
    }

    #[test]
    fn test_logging_event_handler() {
        let mut handler = LoggingEventHandler::default();
        assert!(handler.is_ready());
        
        let event = KvmEvent::MouseMove { x: 100, y: 200 };
        handler.handle_event(event.clone()).unwrap();
        
        assert_eq!(handler.events.len(), 1);
        assert_eq!(handler.events[0], event);
    }
}