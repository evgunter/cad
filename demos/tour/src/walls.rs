//! The shared **wall probe**: a frontier, run live.
//!
//! A scene's "wall" is a shape the model WANTED and the kernel would
//! not state. The rule (lily, `curvedcut::walls`) is that a
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

/// A wall whose refusal is pinned to the default ε and every tighter
/// one, and whose PASS is asserted at a looser one: for a refusal
/// against a target derived from ε (the quadrature's reporting target
/// `1024·ε`, say), which a loose enough ε clears. Both ends are
/// asserted, so the wall reds if it moves at either.
///
/// ε is compared through `Tol::eps` against `DEFAULT_EPS`, the same
/// spelling `mcchain::certified_box_applies` uses.
#[allow(clippy::too_many_arguments)] // the 8th is the run-tolerance witness
pub fn wall_from_default_eps<T, E: core::fmt::Debug>(
    scene: &str,
    n: u32,
    what: &str,
    outcome: Result<T, E>,
    pinned: impl FnOnce(&E) -> bool,
    retire: &str,
    tol: pncad::geom_core::Tol,
) {
    let eps = tol.eps();
    if eps > pncad::tolerance::DEFAULT_EPS {
        if let Err(e) = outcome {
            panic!(
                "wall {n} ({what}) refuses at ε = {eps:e}, looser than the default: \
                 {e:?} — the wall MOVED; re-derive it"
            );
        }
        println!("   wall {n} — {what}: passes at ε = {eps:e}; refused at the default ε");
        return;
    }
    wall(scene, n, what, outcome, pinned, retire);
}
