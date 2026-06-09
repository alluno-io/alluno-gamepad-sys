# alluno-gamepad-sys

Cross-platform virtual gamepad creation.

Windows: creates virtual Xbox 360 controllers through ViGEmBus-compatible bus drivers (games see a real XInput device). Linux: uinput device with the xpad profile. Both accept the same `XGamepad` report and deliver game force-feedback to a callback.

## Requirements

- Windows 10/11 with [ViGEmBus](https://github.com/nefarius/ViGEmBus/releases) installed, or
- Linux with `/dev/uinput` accessible (uinput module loaded, user in the input group)

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
alluno-gamepad-sys = { git = "https://github.com/alluno-io/alluno-gamepad-sys" }
```
`AllunoGamepad::new()`

```rust
use alluno_gamepad_sys::{xbuttons, AllunoGamepad, XGamepad};

let mut pad = AllunoGamepad::new().expect("gamepad backend unavailable");

// Force feedback from games, on a background thread
pad.spawn_notification(|n| {
    println!("rumble: large={} small={}", n.large_motor, n.small_motor);
})
.unwrap();

// Submit input
pad.update(&XGamepad {
    buttons: xbuttons::A | xbuttons::DOWN,
    left_trigger: 255,
    thumb_lx: i16::MAX,
    ..Default::default()
})
.unwrap();
// The virtual controller is unplugged / destroyed on drop.
```

## API

- `AllunoGamepad::new()` / `::with_name(name)`: create + plug in a virtual Xbox 360 controller
- `AllunoGamepad::with_kind(kind, name)`: create an Xbox 360 or DualShock 4 controller
- `AllunoGamepad::is_available()`: backend present (driver installed / uinput accessible)
- `update(&XGamepad)`: submit an XInput-compatible report
- `spawn_notification(callback)`: receive rumble/LED output from games
- `user_index()`: XInput player slot 0-3 (`None` on Linux)
- `kind()`: the controller type being emulated
- `XGamepad` / `xbuttons`: report struct and button flags
- `GamepadKind`: Xbox360 or DualShock4
- `GamepadNotification`: rumble motors + player LED (LED is Windows-only)

## Testing

```powershell
cargo run --bin alluno-gamepad-test   # plugs a pad, sweeps the stick, prints rumble
cargo test -- --nocapture             # skips gracefully without the driver / uinput
```

The Windows backend implements the XUSB IOCTL protocol of the archived [ViGEmBus](https://github.com/nefarius/ViGEmBus) driver (BSD-3-Clause).

## License

MIT
