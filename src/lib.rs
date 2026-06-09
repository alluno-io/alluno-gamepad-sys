pub mod xbuttons {
    pub const UP: u16 = 0x0001;
    pub const DOWN: u16 = 0x0002;
    pub const LEFT: u16 = 0x0004;
    pub const RIGHT: u16 = 0x0008;
    pub const START: u16 = 0x0010;
    pub const BACK: u16 = 0x0020;
    pub const LTHUMB: u16 = 0x0040;
    pub const RTHUMB: u16 = 0x0080;
    pub const LB: u16 = 0x0100;
    pub const RB: u16 = 0x0200;
    pub const GUIDE: u16 = 0x0400;
    pub const A: u16 = 0x1000;
    pub const B: u16 = 0x2000;
    pub const X: u16 = 0x4000;
    pub const Y: u16 = 0x8000;
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct XGamepad {
    pub buttons: u16,
    pub left_trigger: u8,
    pub right_trigger: u8,
    pub thumb_lx: i16,
    pub thumb_ly: i16,
    pub thumb_rx: i16,
    pub thumb_ry: i16,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GamepadNotification {
    pub large_motor: u8,
    pub small_motor: u8,
    pub led_number: u8,
}

/// Which virtual controller to emulate.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GamepadKind {
    /// Xbox 360 (XInput).
    #[default]
    Xbox360,
    /// Sony DualShock 4 (DirectInput).
    DualShock4,
}

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::{Error, Result};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::{Error, Result};

#[cfg(any(target_os = "windows", target_os = "linux"))]
mod device;
#[cfg(any(target_os = "windows", target_os = "linux"))]
pub use device::AllunoGamepad;
