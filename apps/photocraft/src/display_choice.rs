//! Linux: run the window natively on Wayland or under Xwayland (pen tablets, #79).
//!
//! winit 0.30 doesn't implement the Wayland tablet protocol (`zwp_tablet_v2`), so a native
//! Wayland window only sees a pen if the compositor emulates a pointer for it, without pressure.
//! Since Plasma 6.3 KWin no longer does that for clients that don't bind the tablet protocol, so
//! on KDE a pen can't paint at all in a native Wayland window. Xwayland does bind the protocol and
//! hands the pen to X clients as XInput2 devices with pressure, tilt and an eraser end, which
//! `photocraft-tablet` reads.
//!
//! So on a Wayland session with Xwayland available, the app asks Xwayland whether it sees a pen
//! tablet, and if it does, runs the window under Xwayland. `PHOTOCRAFT_DISPLAY` overrides this:
//! `wayland` keeps the native Wayland window (e.g. for crisp fractional scaling when no pen is
//! used), `x11` (or `xwayland`) always uses X11, `auto` (the default) decides as above.
//!
//! The decision is pure ([`decide`]) and tested on every platform; only [`choose`] touches the
//! environment and the X server, on Linux.
#![cfg_attr(not(target_os = "linux"), allow(dead_code))]

use crate::linux_libs::DisplaySession;

/// What `PHOTOCRAFT_DISPLAY` asks for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Requested {
    Auto,
    Wayland,
    X11,
}

/// Parse `PHOTOCRAFT_DISPLAY` (case-insensitive). An unknown value is reported and treated as
/// `auto`.
pub fn requested(value: Option<&str>) -> (Requested, Option<String>) {
    let Some(v) = value.map(str::trim).filter(|v| !v.is_empty()) else { return (Requested::Auto, None) };
    match v.to_ascii_lowercase().as_str() {
        "auto" => (Requested::Auto, None),
        "wayland" => (Requested::Wayland, None),
        "x11" | "xwayland" => (Requested::X11, None),
        _ => (Requested::Auto, Some(format!("PHOTOCRAFT_DISPLAY={v} is not one of auto, wayland, x11; using auto"))),
    }
}

/// The display server to start on and why.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Choice {
    /// The session winit will connect to (what the library preflight checks).
    pub session: DisplaySession,
    /// Make winit use X11 although `WAYLAND_DISPLAY` is set.
    pub force_x11: bool,
    /// Who decided: `PHOTOCRAFT_DISPLAY` or the pen probe.
    pub requested: Requested,
    /// One line for stderr, `None` when nothing worth saying happened.
    pub note: Option<String>,
}

impl Choice {
    fn keep(session: DisplaySession, requested: Requested, note: Option<String>) -> Self {
        Self { session, force_x11: false, requested, note }
    }
    fn x11(requested: Requested, note: String) -> Self {
        Self { session: DisplaySession::X11, force_x11: true, requested, note: Some(note) }
    }

    /// Back to the native Wayland window (when the X11 libraries turn out to be missing).
    pub fn native_wayland(requested: Requested, note: String) -> Self {
        Self::keep(DisplaySession::Wayland, requested, Some(note))
    }
}

/// Decide where the window goes. `x_display`: `DISPLAY` is set (Xwayland is reachable);
/// `pens` asks the X server for pen devices (only called when the answer matters).
pub fn decide(session: DisplaySession, x_display: bool, requested: Requested, pens: impl FnOnce() -> Result<Vec<String>, String>) -> Choice {
    if session != DisplaySession::Wayland {
        return Choice::keep(session, requested, None);
    }
    match requested {
        Requested::Wayland => Choice::keep(session, requested, None),
        Requested::X11 if x_display => Choice::x11(requested, "PHOTOCRAFT_DISPLAY=x11: running under Xwayland".into()),
        Requested::X11 => Choice::keep(session, requested, Some("PHOTOCRAFT_DISPLAY=x11, but DISPLAY isn't set (no Xwayland): staying on Wayland".into())),
        Requested::Auto if !x_display => {
            Choice::keep(session, requested, Some("pen tablets need Xwayland (DISPLAY isn't set): a pen won't paint in this native Wayland window".into()))
        }
        Requested::Auto => match pens() {
            Ok(names) if !names.is_empty() => Choice::x11(
                requested,
                format!("pen tablet found ({}): running under Xwayland for pen pressure; PHOTOCRAFT_DISPLAY=wayland keeps native Wayland", names.join(", ")),
            ),
            Ok(_) => Choice::keep(session, requested, None),
            Err(e) => Choice::keep(session, requested, Some(format!("couldn't ask Xwayland for pen tablets ({e}): staying on Wayland"))),
        },
    }
}

/// Read the environment, probe Xwayland if needed (bounded by `timeout`), and decide.
#[cfg(target_os = "linux")]
pub fn choose(timeout: std::time::Duration) -> Choice {
    let var = |k: &str| std::env::var(k).ok();
    let session = crate::linux_libs::session_from_env(var);
    let x_display = var("DISPLAY").is_some_and(|v| !v.is_empty());
    let (req, warning) = requested(var("PHOTOCRAFT_DISPLAY").as_deref());
    if let Some(w) = warning {
        eprintln!("photocraft: {w}");
    }
    decide(session, x_display, req, || probe_pens(timeout))
}

/// Pen device names Xwayland reports. Connecting can start Xwayland on demand, so the probe runs
/// on its own thread and gives up after `timeout` (the thread then ends by itself).
#[cfg(target_os = "linux")]
fn probe_pens(timeout: std::time::Duration) -> Result<Vec<String>, String> {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::Builder::new()
        .name("photocraft-pen-probe".into())
        .spawn(move || {
            let r = photocraft_tablet::x11::pen_devices(None).map(|d| d.into_iter().map(|d| d.name).collect::<Vec<_>>()).map_err(|e| e.to_string());
            let _ = tx.send(r);
        })
        .map_err(|e| format!("cannot start the probe: {e}"))?;
    rx.recv_timeout(timeout).map_err(|_| format!("no answer within {} ms", timeout.as_millis()))?
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pen() -> Result<Vec<String>, String> {
        Ok(vec!["xwayland-tablet stylus:16".into(), "xwayland-tablet eraser:16".into()])
    }
    fn none() -> Result<Vec<String>, String> {
        Ok(Vec::new())
    }
    fn unreachable_probe() -> Result<Vec<String>, String> {
        panic!("the pen probe must not run here")
    }

    #[test]
    fn parses_photocraft_display() {
        assert_eq!(requested(None), (Requested::Auto, None));
        assert_eq!(requested(Some("  ")), (Requested::Auto, None));
        assert_eq!(requested(Some("Auto")), (Requested::Auto, None));
        assert_eq!(requested(Some("WAYLAND")), (Requested::Wayland, None));
        assert_eq!(requested(Some("x11")), (Requested::X11, None));
        assert_eq!(requested(Some("xwayland")), (Requested::X11, None));
        let (r, w) = requested(Some("mir"));
        assert_eq!(r, Requested::Auto);
        assert!(w.is_some_and(|w| w.contains("mir")));
    }

    #[test]
    fn a_pen_on_wayland_moves_the_window_to_xwayland() {
        let c = decide(DisplaySession::Wayland, true, Requested::Auto, pen);
        assert!(c.force_x11);
        assert_eq!(c.session, DisplaySession::X11);
        assert!(c.note.is_some_and(|n| n.contains("xwayland-tablet stylus:16")));
    }

    #[test]
    fn no_pen_or_no_answer_keeps_native_wayland() {
        let c = decide(DisplaySession::Wayland, true, Requested::Auto, none);
        assert_eq!((c.force_x11, c.session, c.note), (false, DisplaySession::Wayland, None));
        let c = decide(DisplaySession::Wayland, true, Requested::Auto, || Err("no answer within 1500 ms".into()));
        assert!(!c.force_x11);
        assert!(c.note.is_some_and(|n| n.contains("1500 ms")));
    }

    #[test]
    fn photocraft_display_overrides_the_probe() {
        let c = decide(DisplaySession::Wayland, true, Requested::Wayland, unreachable_probe);
        assert_eq!((c.force_x11, c.session), (false, DisplaySession::Wayland));
        let c = decide(DisplaySession::Wayland, true, Requested::X11, unreachable_probe);
        assert_eq!((c.force_x11, c.session), (true, DisplaySession::X11));
        // Without Xwayland there is nothing to switch to.
        let c = decide(DisplaySession::Wayland, false, Requested::X11, unreachable_probe);
        assert_eq!((c.force_x11, c.session), (false, DisplaySession::Wayland));
        let c = decide(DisplaySession::Wayland, false, Requested::Auto, unreachable_probe);
        assert!(!c.force_x11 && c.note.is_some());
    }

    #[test]
    fn x11_and_headless_sessions_are_left_alone() {
        for s in [DisplaySession::X11, DisplaySession::None] {
            for r in [Requested::Auto, Requested::Wayland, Requested::X11] {
                let c = decide(s, true, r, unreachable_probe);
                assert_eq!((c.session, c.force_x11, c.note), (s, false, None), "{s:?} {r:?}");
            }
        }
    }
}
