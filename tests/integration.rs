#![cfg(any(target_os = "windows", target_os = "linux"))]

use alluno_gamepad_sys::{xbuttons, AllunoGamepad, XGamepad};

fn make_pad() -> Option<AllunoGamepad> {
    if !AllunoGamepad::is_available() {
        eprintln!("skipped: gamepad backend not available");
        return None;
    }
    match AllunoGamepad::new() {
        Ok(p) => Some(p),
        Err(e) => {
            eprintln!("skipped: {e}");
            None
        }
    }
}

// The first reports after creation can fail until the OS starts polling the
// device, so updates are retried briefly before asserting.
fn update_with_retry(pad: &mut AllunoGamepad, report: &XGamepad) {
    let start = std::time::Instant::now();
    loop {
        match pad.update(report) {
            Ok(()) => return,
            Err(e) if start.elapsed() < std::time::Duration::from_secs(2) => {
                eprintln!("update not ready yet: {e}");
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
            Err(e) => panic!("update failed: {e}"),
        }
    }
}

#[test]
fn test_available() {
    let _ = AllunoGamepad::is_available();
}

#[test]
fn test_create_drop() {
    let Some(pad) = make_pad() else {
        return;
    };
    drop(pad);
}

#[test]
fn test_submit_report() {
    let Some(mut pad) = make_pad() else {
        return;
    };
    let report = XGamepad {
        buttons: xbuttons::A | xbuttons::UP,
        left_trigger: 128,
        thumb_lx: i16::MAX,
        ..Default::default()
    };
    update_with_retry(&mut pad, &report);
    update_with_retry(&mut pad, &XGamepad::default());
}

#[test]
fn test_user_index() {
    let Some(mut pad) = make_pad() else {
        return;
    };
    if let Some(idx) = pad.user_index() {
        assert!(idx < 4);
    }
}

#[test]
fn test_spawn_notification() {
    let Some(pad) = make_pad() else {
        return;
    };
    pad.spawn_notification(|_| {}).expect("spawn failed");
}
