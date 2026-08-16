//! The shared **wall probe**: a frontier, run live.
//!
//! A scene's "wall" is a shape the model WANTED and the kernel would
//! not state. The rule (lily, `curvedcut::pin_frontier`) is that a
//! wall is never a comment: it is ATTEMPTED for real, every run, and
//! pinned by its own typed refusal — so a findings list cannot quietly
//! rot behind a frontier that moved.
//!
//! Extracted from `lily` unchanged when the Klein bottle grew its own
//! wall list; `lily::wall` is now a one-line wrapper that supplies its
//! scene name, so its twelve call sites are untouched.

/// Runs one wall probe. Three outcomes, and only one of them is
/// silent:
///
/// - the pinned refusal → narrate it, the findings-list entry holds;
/// - a DIFFERENT refusal → panic: the frontier moved underneath the
///   probe, and the findings list would quietly describe the wrong
///   thing;
/// - success → panic: the wall is gone, so the probe and its
///   findings entry must be retired (`retire` says what to do with
///   the scene once it is).
pub fn wall<T, E: core::fmt::Debug>(
    scene: &str,
    n: u32,
    what: &str,
    outcome: Result<T, E>,
    pinned: impl FnOnce(&E) -> bool,
    retire: &str,
) {
    match outcome {
        Err(e) if pinned(&e) => println!("   wall {n} — {what}: REFUSED TYPED, {e:?}"),
        Err(e) => panic!(
            "wall {n} ({what}) still refuses, but NOT with the refusal it pins \
             ({e:?}) — the wall MOVED. Re-derive this probe AND its findings-list \
             entry before trusting either."
        ),
        Ok(_) => panic!(
            "wall {n} ({what}) NO LONGER REFUSES — the {scene} can now say this. \
             Retire the probe and {retire}"
        ),
    }
}
