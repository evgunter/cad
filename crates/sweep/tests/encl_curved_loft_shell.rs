//! **A body with genuinely curved NURBS walls, against the shell verb
//! and the offset door it runs per face.**
//!
//! The twisted loft (`common::approx::twisted_loft`) is lofted through
//! `sweep::loft_body` between a square and the same square turned
//! 0.3 rad: two planar caps and four bilinear SADDLE walls, so every
//! wall's offset is genuinely not a NURBS and has to be fitted. It is
//! the natural operand for the question "what does a shell of a
//! spline-walled body cost at the run's ε", and this row pins why that
//! cost cannot be taken yet, at the thickness a user would ask for:
//! `topo::shell` refuses before any wall is fitted. A cap's inward
//! offset moves the cap's corners, so the seam between two walls that
//! ends at a moved corner has to be re-anchored, and that seam's
//! carrier is a lofted spline where the face-replacement door's
//! re-anchor lane carries only lines and circles.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use topo::{FaceKey, ReplaceFaceError, ShellError};

use crate::common::approx::{nurbs_walls, twisted_loft};

/// The wall thickness this row shells at, in metres: 2.5%
/// of the 2 m section, a thickness a user would ask for.
const THICKNESS: f64 = 0.05;

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
