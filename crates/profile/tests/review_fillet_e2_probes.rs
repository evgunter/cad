//! **FILLET-E2 review probes (profile side)** — the PATHS `.fillet(r)`
//! door and `Profile::validate` agree about the tangency of the joint
//! the door computes, over every small line × line turn.
//!
//! They did not. The door built a four-vertex loop wherever the corner
//! turned by a few multiples of `√ε` and validation refused it,
//! `TangencyContradicted`, naming the door as the way to make the joint
//! exact. What the measurement behind that finding showed is that the
//! door's carrier is tangent to within a couple of ulps at every one of
//! those turns, and the loss is in the STORED form: a profile holds an
//! arc as a chord and a bulge, and a fillet whose sagitta
//! `r(1 − cos(θ/2))` sits at or below ε is read back as a straight
//! segment, with no arc carrier for the declaration to be about.
//!
//! So the door now reads its own output the way validation will and
//! refuses first. This row is the pin on that: at the turns the
//! disagreement used to span, the door refuses, and nothing reaches
//! validation to be contradicted.
//!
//! **The turn angles ride the run's own tolerance.** The joint's margin
//! is sagitta-like, going as `θ²`, so the window sits at `θ ∝ √ε` and
//! moves with the ε row the gate draws; the multiples below are the
//! ones inside it at 1e-6, 1e-9 and 1e-12 alike.

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
fn small_bends_refuse_at_the_path_door_rather_than_building_what_validate_contradicts() {
    let scale = tol().eps().sqrt();
    for c in [0.1, 0.3, 1.0, 2.0] {
        let theta = c * scale;
        let err = bend(theta, 0.2).err().unwrap_or_else(|| {
            panic!("c = {c}, theta = {theta:e}: the door refuses a fillet it cannot store")
        });
        assert!(
            matches!(err, PathError::FilletArcFlattenedInStorage { .. }),
            "c = {c}, theta = {theta:e}: the refusal is about the stored form, got {err}"
        );
        let shown = err.to_string();
        assert!(
            shown.contains("a chord and a bulge") && shown.contains("turn the corner further"),
            "c = {c}, theta = {theta:e}: the sentence says what was lost and what to move, got \
             {shown}"
        );
    }
    // The two sides agree once the turn is well clear of the band, as
    // they always did — and the loop the door builds there validates.
    let theta = 32.0 * scale;
    let lp = bend(theta, 0.2).expect("the door builds the bend");
    Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol())
        .unwrap_or_else(|e| {
            panic!("and the validator accepts the joint at theta = {theta:e}, got {e}")
        });
}
