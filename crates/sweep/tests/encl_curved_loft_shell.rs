//! **A body with genuinely curved NURBS walls, against the shell verb
//! and the offset door it runs per face.**
//!
//! The twisted loft (`common::approx::twisted_loft`) is lofted through
//! `sweep::loft_body` between a square and the same square turned
//! 0.3 rad: two planar caps and four bilinear SADDLE walls, so every
//! wall's offset is genuinely not a NURBS and has to be fitted. It is
//! the natural operand for the question "what does a shell of a
//! spline-walled body cost at the run's ε", and these rows pin why that
//! cost cannot be taken yet, at the thickness a user would ask for.
//!
//! Two boundaries, each measured and each able to move:
//!
//! 1. `topo::shell` refuses before any wall is fitted: a cap's inward
//!    offset moves the cap's corners, so the seam between two walls
//!    that ends at a moved corner has to be re-anchored, and that
//!    seam's carrier is a lofted spline where the face-replacement
//!    door's re-anchor lane carries only lines and circles.
//! 2. A saddle wall, offset alone at the shell thickness, reaches the
//!    fit, and the fit's budget-limited reach at that `d` is
//!    [`SADDLE_FIT_REACH`]. At a looser ε the fit certifies and the
//!    door refuses structurally (the fitted chart's boundary, O4); at a
//!    tighter one the fit refuses.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::OffsetFitError;
use geom_core::Tol;
use topo::{FaceKey, ReplaceFaceError, ShellError};

use crate::common::approx::{band, nurbs_walls, twisted_loft};

/// The wall thickness these rows shell and offset at, in metres: 2.5%
/// of the 2 m section, a thickness a user would ask for.
const THICKNESS: f64 = 0.05;

/// A saddle wall's **budget-limited** reach at `|d| =` [`THICKNESS`]:
/// the smallest sup bound the fit loop reaches under the shipped round
/// budget and sample cap — its last round's, since the bound is still
/// falling there — where it refuses `BudgetExhausted` on a 27×17 grid. Measured on this fixture at
/// 4.1427e-9 at both signs of `d` (every wall agrees to 5 digits); the
/// loop certifies at 1e-6 in 3 rounds (sup bound 4.28e-7).
///
/// It is a reach under the loop's current budget, not the fit's
/// arithmetic floor: with the budget raised the same wall's bound keeps
/// falling for two more rounds before it turns (to 1.15e-9). A change
/// that moves the rounds this loop runs moves this number, and the row
/// reds in either direction beyond [`REACH_BAND`].
const SADDLE_FIT_REACH: f64 = 4.1427e-9;

/// The relative band around [`SADDLE_FIT_REACH`] inside which a
/// refusal's best bound counts as the same reach: wide enough for
/// the last-digit drift an unrelated rounding change produces (the
/// four walls and both signs spread by 1e-5 relative), narrow enough
/// that any real change in how far the loop reaches reds.
const REACH_BAND: f64 = 1e-2;

fn is_spline_wall(walls: &[(FaceKey, impl Sized)], face: FaceKey) -> bool {
    walls.iter().any(|(k, _)| *k == face)
}

/// The shell of the curved loft refuses at a CAP, re-anchoring a
/// wall-to-wall seam, before the fit of any wall runs.
#[test]
fn shelling_the_curved_loft_refuses_at_a_wall_seam_before_any_fit() {
    let body = twisted_loft(0.3);
    let walls = nurbs_walls(&body);
    let e = topo::shell(&body, THICKNESS, Tol::witness())
        .expect_err("a spline-walled body does not shell today");
    let ShellError::Face { face, error } = &e else {
        panic!("expected a per-face offset refusal, got {e}");
    };
    assert!(
        matches!(
            body.get_face(*face)
                .and_then(|f| body.get_surface(f.surface)),
            Some(geom::Surface::Plane { .. })
        ),
        "the refusing face is not a cap: {e}"
    );
    match error.as_ref() {
        ReplaceFaceError::CarrierLaneUnsupported { edge, what } => {
            // The re-anchor lane's own refusal, not any of the door's
            // other carrier-lane sites.
            assert_eq!(
                *what, "a re-anchored carrier that is neither a line nor a circle",
                "the carrier-lane refusal came from another site: {e}"
            );
            let halves = body.get_edge(*edge).expect("the refused edge resolves");
            let sides: Vec<FaceKey> = [halves.he_plus, halves.he_minus]
                .into_iter()
                .filter_map(|he| body.face_of_half_edge(he))
                .collect();
            assert!(
                sides.len() == 2 && sides.iter().all(|k| is_spline_wall(&walls, *k)),
                "the refused edge is not a seam between two spline walls: {e}"
            );
        }
        other => panic!("expected the wall seam's carrier-lane refusal, got {other}"),
    }
}

/// One saddle wall at the shell thickness, both signs: the fit runs, and
/// which side of [`SADDLE_FIT_REACH`] the run's ε sits on decides the
/// refusal.
#[test]
fn a_saddle_walls_offset_at_shell_thickness_reaches_its_measured_bound() {
    let body = twisted_loft(0.3);
    let (wall, _) = *nurbs_walls(&body)
        .first()
        .expect("the loft has spline walls");
    let eps = Tol::witness().eps();
    let (lo, hi) = (
        SADDLE_FIT_REACH * (1.0 - REACH_BAND),
        SADDLE_FIT_REACH * (1.0 + REACH_BAND),
    );
    assert!(
        eps < lo || eps > hi,
        "ε = {eps:e} sits inside the reach band [{lo:e}, {hi:e}], where this row cannot \
         say which arm is right"
    );
    for d in [THICKNESS, -THICKNESS] {
        let mut b = body.clone();
        let e = topo::replace_face_offset(&mut b, wall, d, band(), Tol::witness())
            .expect_err("a fitted wall's boundary cannot follow it");
        match e {
            ReplaceFaceError::FittedBoundaryUnsupported { .. } => assert!(
                eps > hi,
                "d = {d}: the fit certified at ε = {eps:e}, below its measured reach \
                 {SADDLE_FIT_REACH:e} — the loop reaches further now; re-baseline"
            ),
            ReplaceFaceError::Fit { error, .. } => {
                assert!(
                    eps < lo,
                    "d = {d}: the fit refused at ε = {eps:e}, where it certifies today: {error}"
                );
                // The reach is the smallest bound any round reached: the
                // schedule does not read ε, so the loop certifies exactly
                // when ε is at least that.
                let best = match error {
                    OffsetFitError::BudgetExhausted { best, .. }
                    | OffsetFitError::SampleCapReached { best, .. }
                    | OffsetFitError::RefinementStalled { best, .. } => best,
                    other => panic!("d = {d}: expected a refinement refusal, got {other}"),
                };
                assert!(
                    (lo..=hi).contains(&best),
                    "d = {d}: best bound {best:e}, outside the measured reach \
                     {SADDLE_FIT_REACH:e} ± {REACH_BAND} — the loop's reach moved; \
                     re-baseline"
                );
            }
            other => panic!("d = {d}: expected the fit or the fitted boundary, got {other}"),
        }
    }
}
