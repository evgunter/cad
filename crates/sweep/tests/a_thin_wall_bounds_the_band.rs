//! **Where no support bends toward the ball, the wall it stands in
//! bounds the radius.** A convex corner of a thin wall whose supports
//! all turn away from the ball sets `fillet3_radius_headroom` no limit,
//! so past some radius the band's deepest point leaves the material
//! through the wall's far face. Predicate 2's reach meter refuses that
//! band against the far face (`FaceClearance`); the body is never built.
//!
//! The fixture is the Klein bottle's neck→flare in miniature: a revolved
//! wall 0.05 thick, a bore of radius 0.225 turning 30° out into a cone.
//! The inner corner's wedge is 150°, so the band's deepest point lies on
//! the bisector at `r/sin 75° − r` from the corner, and it crosses the
//! outer wall once `r > 1.466`.
//!
//! The cone-foot row pins why `radius_headroom`'s cone read is not
//! reachable end to end at the ball's foot: every cone arm is coaxial
//! with a circular spine, the ball's centre sits at `ρ_c = ρ_f − r·cos α`,
//! and spine regularity's `ρ_c > r` refuses before the foot's own bend
//! could matter.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Sign, Tol};
use sweep::Revolution;
use sweep::blend::battery::Convexity;
use sweep::blend::{BlendError, fillet_edges};
use sweep::test_support::{corners, revolved_about_y, rim_arcs_at};
use topo::validate_geometric;

const WALL: f64 = 0.05;
const BORE: f64 = 0.225;

/// The thin flared wall's meridian: down the bore to the corner
/// `K = (BORE, 0)`, down-out along the 30° cone, across the wall, and
/// back up the outer wall. Returns the corners and the outer corner.
fn thin_flare() -> (Vec<(f64, f64)>, (f64, f64)) {
    let (s, c) = (0.5_f64, 0.75_f64.sqrt());
    let (dir, n) = ((s, -c), (c, s));
    let k = (BORE, 0.0);
    let e = (k.0 + 2.0 * dir.0, k.1 + 2.0 * dir.1);
    let eo = (e.0 + WALL * n.0, e.1 + WALL * n.1);
    let t = (BORE + WALL - (k.0 + WALL * n.0)) / dir.0;
    let ko = (BORE + WALL, k.1 + WALL * n.1 + t * dir.1);
    (vec![(BORE, 1.0), k, e, eo, ko, (BORE + WALL, 1.0)], ko)
}

/// The band's deepest point, on the inner corner's bisector.
fn bisector_point(r: f64) -> (f64, f64) {
    let b = (15.0_f64.to_radians().cos(), 15.0_f64.to_radians().sin());
    let d = r / 75.0_f64.to_radians().sin() - r;
    (BORE + d * b.0, d * b.1)
}

#[test]
fn a_band_past_a_thin_walls_far_face_refuses_at_the_reach_meter() {
    let tol = Tol::witness();
    let (pts, ko) = thin_flare();
    let body = revolved_about_y(corners(&pts), Revolution::Full, tol);
    let edges = rim_arcs_at(&body, BORE, 0.0);
    assert!(!edges.is_empty(), "the inner corner's rim");
    let past = |r: f64| {
        let p = bisector_point(r);
        p.0 > BORE + WALL && p.1 > ko.1
    };

    assert!(!past(1.4), "r = 1.4: the band stays inside the wall");
    let near = fillet_edges(&sweep::test_support::at_rest(&body, tol), &edges, 1.4, tol)
        .unwrap_or_else(|e| panic!("r = 1.4 rolls inside the wall, got {:?}", e.error));
    assert_eq!(topo::validate(&near.body), Ok(()), "r = 1.4: tier 1");
    assert_eq!(topo::validate_closed(&near.body), Ok(()), "r = 1.4: tier 2");
    assert_eq!(
        validate_geometric(&near.body, tol),
        Ok(()),
        "r = 1.4: tier 3"
    );

    for r in [1.5, 1.6] {
        assert!(
            past(r),
            "r = {r}: the band's deepest point is past the outer wall"
        );
        match fillet_edges(&sweep::test_support::at_rest(&body, tol), &edges, r, tol) {
            Err(e) => match e.error {
                BlendError::FaceClearance {
                    chain: Convexity::Convex,
                    margin,
                    ..
                } => assert_eq!(margin.sign, Sign::Negative, "r = {r}: a definite refusal"),
                other => panic!("r = {r}: the reach meter refuses, got {other:?}"),
            },
            Ok(_) => panic!("r = {r}: a band that leaves through the far wall built"),
        }
    }
}

#[test]
fn a_cone_foot_past_its_bend_never_reaches_the_headroom() {
    let tol = Tol::witness();
    let t30 = (core::f64::consts::PI / 6.0).tan();
    // A flat (60° half-angle) frustum's base rim: the ball's foot sits up
    // the cone, where `ρ` is smaller than at the edge, and the foot's own
    // bend is past the ball from `r ≈ 0.268`.
    let body = revolved_about_y(
        corners(&[(0.1, 0.0), (1.0, 0.0), (0.2, 0.8 * t30), (0.1, 0.8 * t30)]),
        Revolution::Full,
        tol,
    );
    let edges = rim_arcs_at(&body, 1.0, 0.0);
    for r in [0.22, 0.27, 0.3, 0.5] {
        match fillet_edges(&sweep::test_support::at_rest(&body, tol), &edges, r, tol) {
            Err(e) => assert!(
                matches!(
                    e.error,
                    BlendError::SpineIrregular { .. } | BlendError::FaceClearanceUncertified { .. }
                ),
                "r = {r}: spine regularity or clearance refuses first, got {:?}",
                e.error
            ),
            Ok(_) => panic!("r = {r}: a band whose foot is past the cone's bend built"),
        }
    }
}
