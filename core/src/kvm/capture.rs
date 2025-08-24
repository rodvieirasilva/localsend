use super::{KvmEvent, MouseButton};
use anyhow::Result;
use tokio::sync::mpsc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// Input capture configuration
#[derive(Debug, Clone)]
pub struct CaptureConfig {
    /// Whether to capture mouse events
    pub capture_mouse: bool,
    /// Whether to capture keyboard events  
    pub capture_keyboard: bool,
    /// Screen dimensions for coordinate normalization
    pub screen_width: u32,
    pub screen_height: u32,
}

impl Default for CaptureConfig {
    fn default() -> Self {
        Self {
            capture_mouse: true,
            capture_keyboard: true,
            screen_width: 1920,
            screen_height: 1080,
        }
    }
}

/// Input capture interface
pub trait InputCapture {
    /// Start capturing input events
    fn start_capture(&mut self, config: CaptureConfig) -> Result<mpsc::Receiver<KvmEvent>>;
    
    /// Stop capturing input events
    fn stop_capture(&mut self) -> Result<()>;
    
    /// Check if currently capturing
    fn is_capturing(&self) -> bool;
    
    /// Get current screen dimensions
    fn get_screen_dimensions(&self) -> Result<(u32, u32)>;
}

/// Mock input capture for testing/development
pub struct MockInputCapture {
    capturing: Arc<AtomicBool>,
    tx: Option<mpsc::Sender<KvmEvent>>,
}

impl Default for MockInputCapture {
    fn default() -> Self {
        Self {
            capturing: Arc::new(AtomicBool::new(false)),
            tx: None,
        }
    }
}

impl InputCapture for MockInputCapture {
    fn start_capture(&mut self, config: CaptureConfig) -> Result<mpsc::Receiver<KvmEvent>> {
        let (tx, rx) = mpsc::channel(100);
        self.tx = Some(tx.clone());
        self.capturing.store(true, Ordering::Relaxed);
        
        // Simulate some input events for testing
        let capturing = self.capturing.clone();
        tokio::spawn(async move {
            let mut counter = 0;
            while capturing.load(Ordering::Relaxed) {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                
                let event = match counter % 3 {
                    0 => KvmEvent::MouseMove { x: 100 + counter, y: 200 + counter },
                    1 => KvmEvent::MouseClick { button: MouseButton::Left, pressed: true },
                    _ => KvmEvent::KeyPress { key: "a".to_string(), pressed: true },
                };
                
                if tx.send(event).await.is_err() {
                    break;
                }
                counter += 1;
                
                if counter > 10 { // Limit for testing
                    break;
                }
            }
        });
        
        Ok(rx)
    }
    
    fn stop_capture(&mut self) -> Result<()> {
        self.capturing.store(false, Ordering::Relaxed);
        self.tx = None;
        Ok(())
    }
    
    fn is_capturing(&self) -> bool {
        self.capturing.load(Ordering::Relaxed)
    }
    
    fn get_screen_dimensions(&self) -> Result<(u32, u32)> {
        Ok((1920, 1080)) // Mock dimensions
    }
}

// Platform-specific implementations will be added here
#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "macos")]
pub mod macos;

/// Create a platform-specific input capture instance
pub fn create_input_capture() -> Box<dyn InputCapture + Send> {
    #[cfg(target_os = "linux")]
    return Box::new(linux::LinuxInputCapture::new());
    
    #[cfg(target_os = "windows")] 
    return Box::new(windows::WindowsInputCapture::new());
    
    #[cfg(target_os = "macos")]
    return Box::new(macos::MacOSInputCapture::new());
    
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    return Box::new(MockInputCapture::default());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_input_capture() {
        let mut capture = MockInputCapture::default();
        assert!(!capture.is_capturing());
        
        let config = CaptureConfig::default();
        let mut rx = capture.start_capture(config).unwrap();
        assert!(capture.is_capturing());
        
        // Should receive some mock events
        let event = rx.recv().await.unwrap();
        match event {
            KvmEvent::MouseMove { x, y } => {
                assert!(x >= 100);
                assert!(y >= 200);
            },
            _ => {},
        }
        
        capture.stop_capture().unwrap();
        assert!(!capture.is_capturing());
    }

    #[test]
    fn test_screen_dimensions() {
        let capture = MockInputCapture::default();
        let (width, height) = capture.get_screen_dimensions().unwrap();
        assert_eq!(width, 1920);
        assert_eq!(height, 1080);
    }
}