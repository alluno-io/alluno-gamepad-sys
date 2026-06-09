use crate::{GamepadNotification, Result, XGamepad};

#[cfg(target_os = "linux")]
use crate::linux::UinputGamepad;
#[cfg(target_os = "windows")]
use crate::windows::{GamepadBus, XusbTarget};

/// A virtual Xbox 360 controller — the unified cross-platform device.
///
/// Windows plugs a ViGEmBus XUSB target; Linux creates a uinput device. Either
/// way the OS sees a real XInput gamepad. The controller is unplugged /
/// destroyed automatically when the value is dropped.
pub struct AllunoGamepad {
    #[cfg(target_os = "windows")]
    inner: XusbTarget,
    #[cfg(target_os = "linux")]
    inner: UinputGamepad,
}

impl AllunoGamepad {
    /// Create and plug in a virtual Xbox 360 controller.
    pub fn new() -> Result<Self> {
        Self::with_name("Alluno Virtual Gamepad")
    }

    /// Create with a specific device name (used as the Linux uinput device name;
    /// ignored on Windows).
    pub fn with_name(name: &str) -> Result<Self> {
        #[cfg(target_os = "windows")]
        {
            let _ = name;
            let bus = std::sync::Arc::new(GamepadBus::connect()?);
            let inner = XusbTarget::plugin(bus)?;
            inner.wait_ready()?;
            Ok(Self { inner })
        }
        #[cfg(target_os = "linux")]
        {
            Ok(Self {
                inner: UinputGamepad::create(name)?,
            })
        }
    }

    /// Whether the platform backend is present (bus driver installed on Windows,
    /// `/dev/uinput` accessible on Linux).
    #[cfg(target_os = "windows")]
    pub fn is_available() -> bool {
        GamepadBus::connect().is_ok()
    }

    /// Whether the platform backend is present (bus driver installed on Windows,
    /// `/dev/uinput` accessible on Linux).
    #[cfg(target_os = "linux")]
    pub fn is_available() -> bool {
        UinputGamepad::is_available()
    }

    /// Submit an XInput-compatible input report.
    pub fn update(&mut self, report: &XGamepad) -> Result<()> {
        self.inner.update(report)
    }

    /// Stream force-feedback (rumble/LED) from games to `callback` on a
    /// dedicated thread. The callback fires only when a game changes output.
    pub fn spawn_notification<F>(&self, callback: F) -> Result<()>
    where
        F: FnMut(GamepadNotification) + Send + 'static,
    {
        self.inner.spawn_notification(callback)
    }

    /// XInput player slot (0–3) assigned to this controller. `None` on Linux.
    #[cfg(target_os = "windows")]
    pub fn user_index(&mut self) -> Option<u32> {
        self.inner.get_user_index().ok()
    }

    /// XInput player slot (0–3) assigned to this controller. `None` on Linux.
    #[cfg(target_os = "linux")]
    pub fn user_index(&mut self) -> Option<u32> {
        None
    }
}
