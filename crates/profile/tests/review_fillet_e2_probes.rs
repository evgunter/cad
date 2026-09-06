//! **FILLET-E2 review probes (profile side)** — the PATHS `.fillet(r)`
//! door and `Profile::validate` disagree about the tangency of the
//! joint the door itself computed, over a wide range of small line ×
//! line turns. PR 1753 reported the 1e-6 instance as possibly "a
//! legitimate sliver refusal"; it is not a sliver.
//!
//! **The turn angles ride the run's own tolerance.** As filed this row
//! swept fixed decades, 1e-7 to 1e-4 rad, and went red at
//! `CAD_TOLERANCE_EPS=1e-12`, where 1e-5 rad meets a straightness
//! escalation instead. The window is not a fixed angle: the joint's
//! margin is sagitta-like, going as `θ²`, so the band on it puts the
//! disagreement at `θ ∝ √ε`. Measured at 1e-6, 1e-9 and 1e-12, the
//! disagreement holds across `θ ∈ [0.1·√ε, 3·√ε]` at every one — and
//! wider at the default, where it runs from `0.003·√ε`. The row asserts
//! the intersection, so it measures the same phenomenon whichever ε the
//! gate draws.
//!
//! Pinned as a characterization, so the disagreement is measured
//! until `work/fillet/path-fillet-door-validator-tangency-disagree.md`
//! decides which side is right.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::{Open, PathError, Profile, ProfileLoop, SketchPlane, Start};

fn tol() -> Tol {
    Tol::witness()
}

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// The bend `fillet_recourse_followability.rs` builds: the incoming
/// ray runs east from the origin, the corner sits at `(4, 0)`, the
/// arrival leaves it at `theta`, anchored three units along.
fn bend(theta: f64, radius: f64) -> Result<ProfileLoop<f64>, PathError<f64>> {
    let anchor = p2(4.0 + 3.0 * theta.cos(), 3.0 * theta.sin());
    Open.at(p2(0.0, 0.0))
        .angle(0.0, tol())?
        .fillet(radius, tol())?
        .at(anchor, tol())?
        .angle(theta, tol())?
        .line(1.0, tol())?
        .line_to(Start, tol())
        .map(|c| c.loop_)
}

#[test]
fn small_bends_build_at_the_path_door_and_refuse_at_validate_as_transversal() {
    let scale = tol().eps().sqrt();
    for c in [0.1, 0.3, 1.0, 3.0] {
        let theta = c * scale;
        let lp = bend(theta, 0.2).unwrap_or_else(|e| {
            panic!("c = {c}, theta = {theta:e}: the door builds the bend, got {e}")
        });
        let err = Profile::new(SketchPlane::xy(), vec![lp])
            .validate(tol())
            .err()
            .unwrap_or_else(|| {
                panic!(
                    "c = {c}, theta = {theta:e}: today the validator refuses the door's own joint"
                )
            });
        let shown = err.to_string();
        assert!(
            shown.contains("declared tangent") && shown.contains("definitely meet transversally"),
            "c = {c}, theta = {theta:e}: the refusal is about the declared tangency, got {shown}"
        );
    }
    // The two sides agree again once the turn is well clear of the band.
    let theta = 32.0 * scale;
    let lp = bend(theta, 0.2).expect("the door builds the bend");
    Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol())
        .unwrap_or_else(|e| {
            panic!("and the validator accepts the joint at theta = {theta:e}, got {e}")
        });
}
