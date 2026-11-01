//! **A closed circle lying in a face refuses its slit form**
//! (`crates/topo/README.md`, tier 3): where both of an edge's faces
//! leave its line into the material of a face of the other operand, the material about the line is two wedges, and an arc there
//! would be two coincident arcs no record backs, so the Boolean refuses
//! `EdgeInFace` (`Curved`) rather than ship the one edge.
//!
//! - **A closed rim in a plane face.** The block
//!   `[−2, 2] × [0, 1] × [−2, 2]` less the full revolve about `y` of the
//!   quadrilateral `(.5, .5), (1, .5), (1, 1), (.6, .6)`: its top vertex
//!   sweeps a closed circle of radius 1 in the block's top face `y = 1`,
//!   and the revolve's two faces at it both leave it into the block's
//!   material.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use sweep::Revolution;
use sweep::test_support::{brick, finished, revolved_about_y};
use topo::{AtRestBody, BooleanError, EdgeInFaceGap, subtract};

fn tol() -> Tol {
    Tol::witness()
}

fn revolved(what: &str, pts: &[(f64, f64)], rev: Revolution<f64>) -> AtRestBody<f64> {
    let verts = pts.iter().map(|&(x, y)| (Point2::new(x, y), 0.0)).collect();
    finished(what, revolved_about_y(verts, rev, tol()), tol())
}

/// `a ∖ b` refuses `EdgeInFace` (`Curved`).
fn refuses_curved(what: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>) {
    match subtract(a, b, tol()) {
        Err(BooleanError::EdgeInFace {
            gap: EdgeInFaceGap::Curved,
            ..
        }) => {}
        other => panic!(
            "{what}: {:?}, not the rider's `Curved` refusal",
            other.map(drop)
        ),
    }
}

/// **The closed rim in the block's top face refuses `Curved`**, as its
/// open-arc forms do.
#[test]
fn a_closed_rim_in_a_plane_face_refuses_its_slit_form() {
    let block = finished(
        "the block",
        brick::<f64>((-2.0, 2.0), (0.0, 1.0), (-2.0, 2.0), tol()),
        tol(),
    );
    let ring = revolved(
        "the ring",
        &[(0.5, 0.5), (1.0, 0.5), (1.0, 1.0), (0.6, 0.6)],
        Revolution::Full,
    );
    refuses_curved("the block less the ring", &block, &ring);
}
