use super::{InputInjection, KvmEvent, MouseButton};
use anyhow::Result;

/// Linux-specific input injection implementation
pub struct LinuxInputInjection;

impl LinuxInputInjection {
    pub fn new() -> Self {
        Self
    }
}

impl InputInjection for LinuxInputInjection {
    fn inject_mouse_move(&mut self, x: i32, y: i32) -> Result<()> {
        // TODO: Implement actual Linux input injection using X11/Wayland
        tracing::debug!("Linux inject mouse move: ({}, {}) - not yet implemented", x, y);
        Ok(())
    }
    
    fn inject_mouse_click(&mut self, button: MouseButton, pressed: bool) -> Result<()> {
        // TODO: Implement actual Linux input injection
        tracing::debug!("Linux inject mouse click: {:?} pressed={} - not yet implemented", button, pressed);
        Ok(())
    }
    
    fn inject_mouse_scroll(&mut self, delta_x: i32, delta_y: i32) -> Result<()> {
        // TODO: Implement actual Linux input injection
        tracing::debug!("Linux inject mouse scroll: ({}, {}) - not yet implemented", delta_x, delta_y);
        Ok(())
    }
    
    fn inject_key_press(&mut self, key: &str, pressed: bool) -> Result<()> {
        // TODO: Implement actual Linux input injection
        tracing::debug!("Linux inject key press: {} pressed={} - not yet implemented", key, pressed);
        Ok(())
    }
    
    fn get_screen_dimensions(&self) -> Result<(u32, u32)> {
        // TODO: Get actual screen dimensions on Linux
        Ok((1920, 1080))
    }
    
    fn is_available(&self) -> bool {
        // TODO: Check if X11/Wayland APIs are available
        true
    }
}