#[cfg(any(target_os = "windows", target_os = "linux"))]
fn main() {
    use alluno_gamepad_sys::{xbuttons, AllunoGamepad, XGamepad};
    use std::time::{Duration, Instant};

    if !AllunoGamepad::is_available() {
        eprintln!("gamepad backend not available (driver / uinput)");
        std::process::exit(1);
    }
    let mut pad = match AllunoGamepad::new() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("failed to create gamepad: {e}");
            std::process::exit(1);
        }
    };
    println!("virtual gamepad created");
    if let Some(idx) = pad.user_index() {
        println!("XInput user index: {idx}");
    }

    pad.spawn_notification(|n| {
        println!("rumble: large={} small={}", n.large_motor, n.small_motor);
    })
    .expect("spawn_notification failed");

    println!("sweeping left stick + tapping A for 10s — check a gamepad tester or game");
    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(10) {
        let t = start.elapsed().as_secs_f64() * 2.0;
        let report = XGamepad {
            buttons: if (t as u64).is_multiple_of(2) {
                xbuttons::A
            } else {
                0
            },
            thumb_lx: (t.cos() * f64::from(i16::MAX)) as i16,
            thumb_ly: (t.sin() * f64::from(i16::MAX)) as i16,
            ..Default::default()
        };
        pad.update(&report).expect("update failed");
        std::thread::sleep(Duration::from_millis(10));
    }
    println!("done");
}

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
fn main() {
    eprintln!("alluno-gamepad-test supports Windows and Linux only");
}
