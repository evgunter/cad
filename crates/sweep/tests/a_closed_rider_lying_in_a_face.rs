//! **A circle lying in a face, closed or on a torus**
//! (`crates/topo/README.md`, tier 3): where both of an edge's faces
//! leave its line into the material of a face of the other operand, the
//! material about the line is two wedges, and the Boolean parts the
//! edge into two coincident arcs, recorded as an edge–edge pair.
//!
//! - **A closed rim in a plane face builds.** The block
//!   `[−2, 2] × [0, 1] × [−2, 2]` less the full revolve about `y` of the
//!   quadrilateral `(.5, .5), (1, .5), (1, 1), (.6, .6)`: its top vertex
//!   sweeps a closed circle of radius 1 in the block's top face `y = 1`,
//!   and the revolve's two faces at it both leave it into the block's
//!   material. Likewise the slab `[−3, 3] × [−1, 0] × [−3, 3]` less the
//!   full revolve of the triangle `(1, 0), (.5, −.5), (1.5, −.5)`, its
//!   apex sweeping a circle in the slab's top face.
//! - **A closed rim in a bore refuses `CurvedFace`.** The tube of radii
//!   1 and 2 about `y` less the full revolve of the triangle
//!   `(1, .5), (1.2, .7), (1.2, .3)`: its apex sweeps a closed circle in
//!   the bore's cylinder, and its two cones both leave it into the
//!   tube's material, a zero-area ring of a curved face, which is not
//!   measured
//!   (`work/nearpoint/a-zero-area-ring-in-a-cylinder-face-measures-degenerate.md`).
//! - **A rim on a torus refuses `CurvedFace`.** The solid torus of radii
//!   3 and 1 about `y` less the revolve of a triangle whose apex `(3, 1)`
//!   sweeps the torus's top parallel, its two cones going down into the
//!   tube: the full revolve's closed rim, and a quarter turn's open arc.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};

use geom_core::{Point2, Tol};
use sweep::Revolution;
use sweep::test_support::{brick, finished, revolved_about_y};
use topo::{AtRestBody, BooleanError, BooleanResult, EdgeInFaceGap, subtract};

fn tol() -> Tol {
    Tol::witness()
}

fn revolved(what: &str, pts: &[(f64, f64)], rev: Revolution<f64>) -> AtRestBody<f64> {
    let verts = pts.iter().map(|&(x, y)| (Point2::new(x, y), 0.0)).collect();
    finished(what, revolved_about_y(verts, rev, tol()), tol())
}

/// `a ∖ b` refuses `EdgeInFace` (`CurvedFace`).
fn refuses_curved_face(what: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>) {
    match subtract(a, b, tol()) {
        Err(BooleanError::EdgeInFace {
            gap: EdgeInFaceGap::CurvedFace,
            ..
        }) => {}
        other => panic!(
            "{what}: {:?}, not the rider's `CurvedFace` refusal",
            other.map(drop)
        ),
    }
}

/// `a ∖ b` builds one body at `volume` that passes tiers 1, 3 and 3′ on
/// its records, among them one edge–edge record of the rim's two
/// coincident circles. *Breaks if* the parting ships them unrecorded (3′
/// refuses an `EdgeEdgeOverlap`).
fn builds_recorded(what: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>, volume: f64) {
    let r = match subtract(a, b, tol()) {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("{what}: {:?}, not a body", other.map(drop)),
    };
    assert_eq!(topo::validate(&r.body), Ok(()), "{what}: tier 1");
    assert_eq!(
        topo::validate_geometric(&r.body, tol()),
        Ok(()),
        "{what}: tier 3"
    );
    assert_eq!(
        topo::validate_pseudomanifold(&r.body, r.contacts(), tol()),
        Ok(()),
        "{what}: tier 3′ on {:?}",
        r.contacts()
    );
    assert_eq!(
        r.contacts().edge_edge().count(),
        1,
        "{what}: the rim's two circles are one edge–edge record: {:?}",
        r.contacts()
    );
    let got =
        topo::test_support::pinned_volume(what, &finished(what, r.body.clone(), tol()), tol())
            .volume;
    assert!(
        (got - volume).abs() < 1e-9 * volume,
        "{what}: volume {got}, closed form {volume}"
    );
}

/// **The closed rim in the block's top face builds**, the block less
/// the ring's Pappus volume `2π · 5/48`, and the slab less the groove's
/// `2π · 1/4`.
#[test]
fn a_closed_rim_in_a_plane_face_builds_recorded() {
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
    builds_recorded(
        "the block less the ring",
        &block,
        &ring,
        16.0 - 2.0 * PI * 5.0 / 48.0,
    );
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
    builds_recorded("the slab less the groove", &slab, &groove, 36.0 - PI / 2.0);
}

/// **The closed rim in the tube's bore refuses `CurvedFace`**.
#[test]
fn a_closed_rim_in_a_bore_refuses_its_curved_face_slit() {
    let tube = revolved(
        "the tube",
        &[(1.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0)],
        Revolution::Full,
    );
    let keel = revolved(
        "the keel",
        &[(1.0, 0.5), (1.2, 0.7), (1.2, 0.3)],
        Revolution::Full,
    );
    refuses_curved_face("the tube less the keel", &tube, &keel);
}

/// **A rim on the torus's top parallel refuses `CurvedFace`**, closed
/// and open.
#[test]
fn a_rim_on_a_torus_refuses_its_curved_face_slit() {
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
        refuses_curved_face(&format!("the torus less {what}"), &torus, &ring);
    }
}
