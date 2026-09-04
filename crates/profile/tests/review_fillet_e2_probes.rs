//! **FILLET-E2 review probes (profile side)** — the PATHS `.fillet(r)`
//! door and `Profile::validate` disagree about the tangency of the
//! joint the door itself computed, for line × line turns from 1e-7 to
//! 1e-4 rad. PR 1753 reported the 1e-6 instance as possibly "a
//! legitimate sliver refusal"; four decades is not a sliver.
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
    for theta in [1e-7, 1e-6, 1e-5, 1e-4] {
        let lp = bend(theta, 0.2)
            .unwrap_or_else(|e| panic!("theta = {theta:e}: the door builds the bend, got {e}"));
        let err = Profile::new(SketchPlane::xy(), vec![lp])
            .validate(tol())
            .err()
            .unwrap_or_else(|| {
                panic!("theta = {theta:e}: today the validator refuses the door's own joint")
            });
        let shown = err.to_string();
        assert!(
            shown.contains("declared tangent") && shown.contains("definitely meet transversally"),
            "theta = {theta:e}: the refusal is about the declared tangency, got {shown}"
        );
    }
    // The two sides agree again at a turn of 1e-3.
    let lp = bend(1e-3, 0.2).expect("the door builds the bend");
    Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol())
        .expect("and the validator accepts the joint at theta = 1e-3");
}
