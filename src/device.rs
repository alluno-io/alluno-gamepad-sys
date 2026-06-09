use crate::{GamepadKind, GamepadNotification, Result, XGamepad};

#[cfg(target_os = "linux")]
use crate::linux::UinputGamepad;
#[cfg(target_os = "windows")]
use crate::windows::{Ds4Target, GamepadBus, XusbTarget};

#[cfg(target_os = "windows")]
enum Inner {
    Xbox360(XusbTarget),
    Ds4(Ds4Target),
}

/// A virtual game controller.
pub struct AllunoGamepad {
    #[cfg(target_os = "windows")]
    inner: Inner,
    #[cfg(target_os = "linux")]
    inner: UinputGamepad,
}

impl AllunoGamepad {
    /// Create and plug in a virtual Xbox 360 controller.
    pub fn new() -> Result<Self> {
        Self::with_kind(GamepadKind::Xbox360, "Alluno Virtual Gamepad")
    }

    /// Create an Xbox 360 controller with a specific device name.
    pub fn with_name(name: &str) -> Result<Self> {
        Self::with_kind(GamepadKind::Xbox360, name)
    }

    /// Create a controller of the given kind with a specific device name.
    pub fn with_kind(kind: GamepadKind, name: &str) -> Result<Self> {
        #[cfg(target_os = "windows")]
        {
            let _ = name;
            let bus = std::sync::Arc::new(GamepadBus::connect()?);
            let inner = match kind {
                GamepadKind::Xbox360 => {
                    let target = XusbTarget::plugin(bus)?;
                    target.wait_ready()?;
                    Inner::Xbox360(target)
                }
                GamepadKind::DualShock4 => {
                    let target = Ds4Target::plugin(bus)?;
                    target.wait_ready()?;
                    Inner::Ds4(target)
                }
            };
            Ok(Self { inner })
        }
        #[cfg(target_os = "linux")]
        {
            Ok(Self {
                inner: UinputGamepad::create(name, kind)?,
            })
        }
    }

    /// Whether the platform backend is present.
    #[cfg(target_os = "windows")]
    pub fn is_available() -> bool {
        GamepadBus::connect().is_ok()
    }

    /// Whether the platform backend is present.
    #[cfg(target_os = "linux")]
    pub fn is_available() -> bool {
        UinputGamepad::is_available()
    }

    /// The controller type this gamepad is emulating.
    pub fn kind(&self) -> GamepadKind {
        #[cfg(target_os = "windows")]
        {
            match &self.inner {
                Inner::Xbox360(_) => GamepadKind::Xbox360,
                Inner::Ds4(_) => GamepadKind::DualShock4,
            }
        }
        #[cfg(target_os = "linux")]
        {
            self.inner.kind()
        }
    }

    /// Submit an XInput-compatible input report.
    pub fn update(&mut self, report: &XGamepad) -> Result<()> {
        #[cfg(target_os = "windows")]
        {
            match &mut self.inner {
                Inner::Xbox360(t) => t.update(report),
                Inner::Ds4(t) => t.update(report),
            }
        }
        #[cfg(target_os = "linux")]
        {
            self.inner.update(report)
        }
    }

    /// Stream force-feedback from games to a callback on a dedicated thread.
    pub fn spawn_notification<F>(&self, callback: F) -> Result<()>
    where
        F: FnMut(GamepadNotification) + Send + 'static,
    {
        #[cfg(target_os = "windows")]
        {
            match &self.inner {
                Inner::Xbox360(t) => t.spawn_notification(callback),
                Inner::Ds4(t) => t.spawn_notification(callback),
            }
        }
        #[cfg(target_os = "linux")]
        {
            self.inner.spawn_notification(callback)
        }
    }

    /// XInput player slot. None for DualShock 4 and on Linux.
    pub fn user_index(&mut self) -> Option<u32> {
        #[cfg(target_os = "windows")]
        {
            match &mut self.inner {
                Inner::Xbox360(t) => t.get_user_index().ok(),
                Inner::Ds4(_) => None,
            }
        }
        #[cfg(target_os = "linux")]
        {
            None
        }
    }
}
