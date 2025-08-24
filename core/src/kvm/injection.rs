use super::{KvmEvent, MouseButton};
use anyhow::Result;

/// Input injection interface for the server side
pub trait InputInjection {
    /// Inject a mouse movement event
    fn inject_mouse_move(&mut self, x: i32, y: i32) -> Result<()>;
    
    /// Inject a mouse button event
    fn inject_mouse_click(&mut self, button: MouseButton, pressed: bool) -> Result<()>;
    
    /// Inject a mouse scroll event
    fn inject_mouse_scroll(&mut self, delta_x: i32, delta_y: i32) -> Result<()>;
    
    /// Inject a keyboard event
    fn inject_key_press(&mut self, key: &str, pressed: bool) -> Result<()>;
    
    /// Get current screen dimensions
    fn get_screen_dimensions(&self) -> Result<(u32, u32)>;
    
    /// Check if injection is available/supported
    fn is_available(&self) -> bool;
}

/// Mock input injection for testing/development
#[derive(Default)]
pub struct MockInputInjection {
    pub injected_events: Vec<KvmEvent>,
}

impl InputInjection for MockInputInjection {
    fn inject_mouse_move(&mut self, x: i32, y: i32) -> Result<()> {
        tracing::debug!("Mock inject mouse move: ({}, {})", x, y);
        self.injected_events.push(KvmEvent::MouseMove { x, y });
        Ok(())
    }
    
    fn inject_mouse_click(&mut self, button: MouseButton, pressed: bool) -> Result<()> {
        tracing::debug!("Mock inject mouse click: {:?} pressed={}", button, pressed);
        self.injected_events.push(KvmEvent::MouseClick { button, pressed });
        Ok(())
    }
    
    fn inject_mouse_scroll(&mut self, delta_x: i32, delta_y: i32) -> Result<()> {
        tracing::debug!("Mock inject mouse scroll: ({}, {})", delta_x, delta_y);
        self.injected_events.push(KvmEvent::MouseScroll { delta_x, delta_y });
        Ok(())
    }
    
    fn inject_key_press(&mut self, key: &str, pressed: bool) -> Result<()> {
        tracing::debug!("Mock inject key press: {} pressed={}", key, pressed);
        self.injected_events.push(KvmEvent::KeyPress { key: key.to_string(), pressed });
        Ok(())
    }
    
    fn get_screen_dimensions(&self) -> Result<(u32, u32)> {
        Ok((1920, 1080)) // Mock dimensions
    }
    
    fn is_available(&self) -> bool {
        true
    }
}

/// Inject a KVM event into the system
pub fn inject_event(injector: &mut dyn InputInjection, event: KvmEvent) -> Result<()> {
    match event {
        KvmEvent::MouseMove { x, y } => injector.inject_mouse_move(x, y),
        KvmEvent::MouseClick { button, pressed } => injector.inject_mouse_click(button, pressed),
        KvmEvent::MouseScroll { delta_x, delta_y } => injector.inject_mouse_scroll(delta_x, delta_y),
        KvmEvent::KeyPress { key, pressed } => injector.inject_key_press(&key, pressed),
        KvmEvent::ScreenInfo { .. } => Ok(()), // Screen info is metadata, no injection needed
    }
}

// Platform-specific implementations will be added here
#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "macos")]
pub mod macos;

/// Create a platform-specific input injection instance
pub fn create_input_injection() -> Box<dyn InputInjection + Send> {
    #[cfg(target_os = "linux")]
    return Box::new(linux::LinuxInputInjection::new());
    
    #[cfg(target_os = "windows")]
    return Box::new(windows::WindowsInputInjection::new());
    
    #[cfg(target_os = "macos")]
    return Box::new(macos::MacOSInputInjection::new());
    
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    return Box::new(MockInputInjection::default());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_input_injection() {
        let mut injector = MockInputInjection::default();
        assert!(injector.is_available());
        assert_eq!(injector.get_screen_dimensions().unwrap(), (1920, 1080));
        
        // Test mouse move injection
        injector.inject_mouse_move(100, 200).unwrap();
        assert_eq!(injector.injected_events.len(), 1);
        
        // Test mouse click injection
        injector.inject_mouse_click(MouseButton::Left, true).unwrap();
        assert_eq!(injector.injected_events.len(), 2);
        
        // Test key press injection
        injector.inject_key_press("a", true).unwrap();
        assert_eq!(injector.injected_events.len(), 3);
    }

    #[test]
    fn test_inject_event() {
        let mut injector = MockInputInjection::default();
        
        let event = KvmEvent::MouseMove { x: 50, y: 100 };
        inject_event(&mut injector, event.clone()).unwrap();
        
        assert_eq!(injector.injected_events.len(), 1);
        assert_eq!(injector.injected_events[0], event);
    }
}