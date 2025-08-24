use flutter_rust_bridge::frb;
use localsend::kvm::{KvmEvent, MouseButton};
use localsend::kvm::capture::{create_input_capture, CaptureConfig};
use localsend::kvm::injection::{create_input_injection, inject_event};
use anyhow::Result;

/// Check if KVM is supported
#[frb]
pub fn kvm_is_supported() -> bool {
    true // For now, return true for all platforms
}

/// Get current screen dimensions
#[frb]
pub fn kvm_get_screen_dimensions() -> Result<(u32, u32)> {
    // Use mock dimensions for now
    Ok((1920, 1080))
}

/// Handle incoming KVM event on server
#[frb]
pub fn kvm_handle_server_event(event: KvmEvent) -> Result<()> {
    let mut injection = create_input_injection();
    inject_event(injection.as_mut(), event)?;
    Ok(())
}

/// Create a simple test event to verify KVM functionality
#[frb]
pub fn kvm_create_test_event() -> KvmEvent {
    KvmEvent::MouseMove { x: 100, y: 200 }
}

/// Test input capture functionality
#[frb]
pub fn kvm_test_capture() -> Result<bool> {
    let _capture = create_input_capture();
    // For now just return true if we can create the capture
    Ok(true)
}

/// Test input injection functionality
#[frb] 
pub fn kvm_test_injection() -> Result<bool> {
    let _injection = create_input_injection();
    // For now just return true if we can create the injection
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kvm_support() {
        assert!(kvm_is_supported());
    }

    #[test]
    fn test_screen_dimensions() {
        let dimensions = kvm_get_screen_dimensions().unwrap();
        assert_eq!(dimensions, (1920, 1080));
    }

    #[test]
    fn test_create_test_event() {
        let event = kvm_create_test_event();
        match event {
            KvmEvent::MouseMove { x, y } => {
                assert_eq!(x, 100);
                assert_eq!(y, 200);
            },
            _ => panic!("Wrong event type"),
        }
    }

    #[test]
    fn test_handle_server_event() {
        let event = KvmEvent::MouseMove { x: 50, y: 75 };
        let result = kvm_handle_server_event(event);
        assert!(result.is_ok());
    }
}