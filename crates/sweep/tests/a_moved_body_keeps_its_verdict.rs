//! **A finished body moved by an isometry keeps its verdict**
//! (`topo::FinishedBody::moved`, `[topology.tiers]`): the move carries
//! every verdict and certificate and asks only the session box again,
//! so finishing the moved body afresh agrees with it.
//!
//! Over a body of every surface class the sweeps build (planes, a
//! cylinder, a sphere, a cone, a torus, partial revolves with plane
//! caps, a spline loft, a torus cut to spiric rims) and an offset
//! (`Approx`) cap, under a rigid motion and under reflections along an
//! axis and oblique to every axis:
//!
//! - the move runs no finish (`topo::finishes_on_this_thread`);
//! - `topo::finish` of the moved body's construction state passes where
//!   the move passed;
//! - the volume measured through the carried certificate agrees with the
//!   one measured through the fresh finish's, within their pads.
//!
//! This is the test that `[simple.checks]` puts where a production gate
//! used to re-derive the verdict: a defect in the data map (a sense not
//! flipped, a joint mis-restated) would ship with the source's verdict,
//! and the fresh finish is what catches it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{RigidMotion, Tol, UnitVec3, Vec3};
use topo::{AtRestBody, Body, Isometry};

use crate::a_mirrored_body_is_its_image::{bodies, loft, planes, torus_less_a_brick};

fn tol() -> Tol {
    Tol::witness()
}

/// A turn about an oblique axis, then a shift: rigid, off every axis.
fn motion() -> RigidMotion<f64> {
    let band = geom_core::Band::linear(tol()).unwrap();
    let axis = UnitVec3::new(Vec3::new(1.0, -2.0, 0.5), "a moved body's turn", band).unwrap();
    RigidMotion::rotation(axis, 0.7)
        .unwrap()
        .translated(Vec3::new(3.0, -1.25, 0.5))
        .unwrap()
}

fn isometries() -> Vec<(String, Isometry<f64>)> {
    let mut out = vec![("the oblique motion".to_owned(), Isometry::Proper(motion()))];
    out.extend(
        planes()
            .into_iter()
            .map(|(name, r)| (format!("the mirror across {name}"), Isometry::Improper(r))),
    );
    out
}

fn volume(body: &AtRestBody<f64>) -> (f64, f64) {
    let m = topo::mass_properties(body, tol(), topo::test_support::pinned_resolution(tol()))
        .unwrap_or_else(|e| panic!("the body measures: {e:?}"));
    (m.enclosure.volume, m.enclosure.volume_pad)
}

fn keeps_its_verdict(what: &str, body: Body<f64>) {
    let source = topo::test_support::finished(what, body, tol());
    for (how, iso) in isometries() {
        let at = format!("{what} under {how}");
        let before = topo::finishes_on_this_thread();
        let moved = source
            .moved(&iso, tol())
            .unwrap_or_else(|e| panic!("{at}: the move refused: {e}"));
        assert_eq!(
            topo::finishes_on_this_thread(),
            before,
            "{at}: the move ran a finish"
        );
        let fresh = topo::finish(moved.clone().into_body(), tol())
            .unwrap_or_else(|e| panic!("{at}: the move passed and a fresh finish refused: {e}"));
        let ((v, v_pad), (w, w_pad)) = (volume(&moved), volume(&fresh));
        assert!(
            (v - w).abs() <= v_pad + w_pad + 1e-12 * v.abs(),
            "{at}: the carried certificate measures {v} ± {v_pad}, the fresh one {w} ± {w_pad}"
        );
    }
}

#[test]
fn every_surface_class_keeps_its_verdict_under_a_move() {
    let mut all = bodies();
    all.push(("the spline loft", loft()));
    all.push(("the spiric-capped torus", torus_less_a_brick()));
    for (what, body) in all {
        keeps_its_verdict(what, body);
    }
}

#[test]
fn an_offset_cap_keeps_its_verdict_under_a_move() {
    let (body, _) = crate::common::approx::box_with_approx_cap(0.05, 1e-9);
    keeps_its_verdict("the offset-capped box", body);
}

/// **A move out of the session box refuses there, typed, and does not
/// panic**: a brick at 950 m moved 900 m further reaches past the box,
/// which the move asks again.
#[test]
fn a_move_out_of_the_box_refuses_at_the_box() {
    let brick = topo::test_support::finished(
        "the far brick",
        topo::test_support::brick::<f64>((950.0, 951.0), (0.0, 1.0), (0.0, 1.0), tol()),
        tol(),
    );
    let out = RigidMotion::translation(Vec3::new(900.0, 0.0, 0.0)).unwrap();
    let refusal = brick
        .moved(&Isometry::Proper(out), tol())
        .expect_err("a body moved past the box refuses");
    let shown = refusal.to_string();
    assert!(
        matches!(
            &refusal.into_findings()[..],
            [topo::ValidationError::OutsideSessionBox { .. }]
        ),
        "the session box is the move's one refusal: {shown}"
    );
}
