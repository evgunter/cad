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
//! 2. Each saddle wall, offset alone at the shell thickness, reaches
//!    the fit, and the fit's reach at that `d` is [`SADDLE_FIT_REACH`].
//!    At a looser ε the fit certifies and the door refuses
//!    structurally (the fitted chart's boundary, O4); at a tighter one
//!    the fit refuses.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::OffsetFitError;
use geom_core::Tol;
use topo::{Body, FaceKey, ReplaceFaceError, ShellError};

use crate::common::approx::{band, twisted_loft};

/// The wall thickness these rows shell and offset at, in metres: 2.5%
/// of the 2 m section, a thickness a user would ask for.
const THICKNESS: f64 = 0.05;

/// The tightest ε at which a saddle wall's offset fit certifies at
/// `|d| =` [`THICKNESS`], measured on this fixture: the loop certifies
/// at 1e-6 in 3 rounds (sup bound 4.28e-7) and refuses at 1e-9 and at
/// 1e-12 with the round budget spent on a 27×17 grid at an achieved sup
/// bound of 4.1427e-9, identical at both signs of `d`. A fit refusal
/// whose achieved bound exceeds this is the engine reaching less far
/// than it does today; a certificate at an ε below it is the engine
/// reaching further. Either moves the boundary and reds.
const SADDLE_FIT_REACH: f64 = 4.2e-9;

fn is_spline_wall(body: &Body<f64>, face: FaceKey) -> bool {
    matches!(
        body.get_face(face).and_then(|f| body.get_surface(f.surface)),
        Some(geom::Surface::Nurbs(n)) if !n.is_placeholder()
    )
}

/// The shell of the curved loft refuses at a CAP, re-anchoring a
/// wall-to-wall seam, before the fit of any wall runs.
#[test]
fn shelling_the_curved_loft_refuses_at_a_wall_seam_before_any_fit() {
    let body = twisted_loft(0.3);
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
        ReplaceFaceError::CarrierLaneUnsupported { edge, .. } => {
            let halves = body.get_edge(*edge).expect("the refused edge resolves");
            let sides: Vec<FaceKey> = [halves.he_plus, halves.he_minus]
                .into_iter()
                .filter_map(|he| body.face_of_half_edge(he))
                .collect();
            assert!(
                sides.len() == 2 && sides.iter().all(|k| is_spline_wall(&body, *k)),
                "the refused edge is not a seam between two spline walls: {e}"
            );
        }
        other => panic!("expected the wall seam's carrier-lane refusal, got {other}"),
    }
}

/// Each saddle wall at the shell thickness: the fit runs, and which side
/// of [`SADDLE_FIT_REACH`] the run's ε sits on decides the refusal.
#[test]
fn a_saddle_walls_offset_at_shell_thickness_reaches_its_measured_bound() {
    let body = twisted_loft(0.3);
    let wall = body
        .faces()
        .map(|(k, _)| k)
        .find(|k| is_spline_wall(&body, *k))
        .expect("the loft has spline walls");
    let eps = Tol::witness().eps();
    for d in [THICKNESS, -THICKNESS] {
        let mut b = body.clone();
        let e = topo::replace_face_offset(&mut b, wall, d, band(), Tol::witness())
            .expect_err("a fitted wall's boundary cannot follow it");
        match e {
            ReplaceFaceError::FittedBoundaryUnsupported { .. } => assert!(
                eps >= SADDLE_FIT_REACH,
                "d = {d}: the fit certified at ε = {eps:e}, below its measured reach \
                 {SADDLE_FIT_REACH:e} — the engine reaches further now; re-baseline"
            ),
            ReplaceFaceError::Fit { error, .. } => {
                assert!(
                    eps < SADDLE_FIT_REACH,
                    "d = {d}: the fit refused at ε = {eps:e}, where it certifies today: {error}"
                );
                let achieved = match error {
                    OffsetFitError::BudgetExhausted { achieved, .. }
                    | OffsetFitError::SampleCapReached { achieved, .. }
                    | OffsetFitError::RefinementStalled { achieved, .. } => achieved,
                    other => panic!("d = {d}: expected a refinement refusal, got {other}"),
                };
                assert!(
                    achieved > eps && achieved <= SADDLE_FIT_REACH,
                    "d = {d}: achieved {achieved:e} against ε = {eps:e} and the measured \
                     reach {SADDLE_FIT_REACH:e}"
                );
            }
            other => panic!("d = {d}: expected the fit or the fitted boundary, got {other}"),
        }
    }
}
