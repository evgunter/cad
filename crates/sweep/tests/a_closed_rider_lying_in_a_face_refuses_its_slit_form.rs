//! **A circle lying in a face, closed or on a torus, refuses its slit
//! form**
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
//!   material. Likewise the slab `[−3, 3] × [−1, 0] × [−3, 3]` less the
//!   full revolve of the triangle `(1, 0), (.5, −.5), (1.5, −.5)`, its
//!   apex sweeping a circle in the slab's top face.
//! - **A rim on a torus.** The solid torus of radii 3 and 1 about `y`
//!   less the revolve of a triangle whose apex `(3, 1)` sweeps the
//!   torus's top parallel, its two cones going down into the tube: the
//!   full revolve's closed rim, and a quarter turn's open arc.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::FRAC_PI_2;

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
    let slab = finished(
        "the slab",
        brick::<f64>((-3.0, 3.0), (-1.0, 0.0), (-3.0, 3.0), tol()),
        tol(),
    );
    let groove = revolved(
        "the groove",
        &[(1.0, 0.0), (0.5, -0.5), (1.5, -0.5)],
        Revolution::Full,
    );
    refuses_curved("the slab less the groove", &slab, &groove);
}

/// **A rim on the torus's top parallel refuses `Curved`**, closed and
/// open.
#[test]
fn a_rim_on_a_torus_refuses_its_slit_form() {
    let torus = finished(
        "the torus",
        revolved_about_y(
            vec![(Point2::new(2.0, 0.0), 1.0), (Point2::new(4.0, 0.0), 1.0)],
            Revolution::Full,
            tol(),
        ),
        tol(),
    );
    let keel = [(3.0, 1.0), (2.8, 0.6), (3.2, 0.6)];
    for (what, rev) in [
        ("the full turn", Revolution::Full),
        ("the quarter turn", Revolution::Partial(FRAC_PI_2)),
    ] {
        let ring = revolved(what, &keel, rev);
        refuses_curved(&format!("the torus less {what}"), &torus, &ring);
    }
}
