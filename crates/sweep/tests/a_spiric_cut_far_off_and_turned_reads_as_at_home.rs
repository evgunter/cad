//! **A spiric cut far off and turned reads as it does at home.** The
//! torus `R = 2, r = 0.5` cut by the half-space `x ≥ 0.3` meets the
//! plane in a spiric: its caps are planes bounded by spirics and its
//! walls tori, each measured by quadrature. A rigid motion preserves
//! volume, and moving a body moves no face's width: every face's flux
//! is read about a vertex of its shell's first quadrature face
//! (`topo::props`' `shell_reference`), so the lever a width scales with
//! is the extent of the shell's quadrature faces wherever the body sits.
//!
//! Read about the world origin, the same cut carried `s = 100` along
//! `(8, −3, 5)` and turned by 0.7 rad about a skew axis stood ~990 m
//! off: each spiric face's vector area kept ~7e-11 of width per
//! component at the last round, the lever made the flux ~8.6e-8 wide,
//! and `T ∩ P`, `P ∩ T` and `T ∖ P` refused `QuadratureBudget` at every
//! ε (width 3.6e-8 against 1024ε = 1.024e-9 at 1e-12). Carried without
//! the turn, the faces' widths off their normals stayed zero, so the
//! translation alone built.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point3, Tol, Vec3};
use sweep::test_support::finished;
use topo::AtRestBody;

use crate::common::operands::framed_bar;
use crate::germ_plane_torus_chord::half;
use crate::germ_plane_torus_frame::{six, torus};

/// The torus and the half-space carried `s` along `(8, −3, 5)` after a
/// turn of 0.7 rad about the skew axis `(0.3, −0.5, 0.8)` through the
/// origin.
fn posed(s: f64) -> (AtRestBody<f64>, AtRestBody<f64>) {
    let tol = Tol::witness();
    let axis = Vec3::new(0.3, -0.5, 0.8).normalize();
    let m = Affine3::translation(Vec3::new(8.0, -3.0, 5.0) * s)
        * Affine3::rotation_about_axis(Point3::origin(), axis, 0.7);
    let motion = geom_core::test_support::rigid(&m, tol);
    let mv = |b: &AtRestBody<f64>, who: &str| {
        finished(who, topo::transform_rigid(b, &motion, tol).unwrap(), tol)
    };
    (
        mv(&torus(2.0, 0.5), "the torus"),
        mv(
            &half(Point3::new(0.3, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0)).body,
            "the half-space",
        ),
    )
}

/// Each op's result at `s`, built.
fn built(s: f64) -> Vec<(&'static str, AtRestBody<f64>)> {
    let (t, p) = posed(s);
    six(&t, &p)
        .into_iter()
        .map(|(label, got)| {
            let got = got.unwrap_or_else(|e| panic!("s = {s}, {label}: {e:?}"));
            let body = got
                .body()
                .unwrap_or_else(|| panic!("s = {s}, {label}: empty"))
                .body
                .clone();
            (label, body)
        })
        .collect()
}

/// Every op builds far off and turned, and measures the volume it
/// measures at home: the two enclosures overlap.
#[test]
fn every_op_far_off_and_turned_measures_its_volume_at_home() {
    let tol = Tol::witness();
    for ((label, home), (_, far)) in built(0.0).into_iter().zip(built(100.0)) {
        let at = |b: &AtRestBody<f64>, s: f64| {
            topo::mass_properties(b, tol)
                .unwrap_or_else(|e| panic!("s = {s}, {label}: the volume: {e:?}"))
        };
        let (home, far) = (at(&home, 0.0), at(&far, 100.0));
        assert!(
            (far.volume - home.volume).abs() <= far.volume_pad + home.volume_pad,
            "{label}: far off {} ± {} against at home {} ± {}",
            far.volume,
            far.volume_pad,
            home.volume,
            home.volume_pad,
        );
    }
}

/// Every quadrature face, a spiric cap and a torus wall alike, meets
/// the reporting target at the same round far off and turned as at
/// home: its width does not depend on where the body sits.
#[test]
fn every_spiric_face_meets_its_target_at_the_round_it_does_at_home() {
    let tol = Tol::witness();
    let rounds = |b: &AtRestBody<f64>| {
        let mut rounds: Vec<(String, Option<usize>)> =
            topo::test_support::quadrature_rounds(b, tol)
                .unwrap()
                .into_iter()
                .map(|(_, kind, round)| (format!("{kind:?}"), round))
                .collect();
        rounds.sort_unstable();
        rounds
    };
    for ((label, home), (_, far)) in built(0.0).into_iter().zip(built(100.0)) {
        let (home, far) = (rounds(&home), rounds(&far));
        for kind in ["Plane", "Torus"] {
            assert!(
                home.iter().any(|(k, _)| k == kind),
                "{label}: a {kind} face is measured by quadrature: {home:?}"
            );
        }
        assert!(
            home.iter().all(|(_, round)| round.is_some()),
            "{label}: every face meets its target at home: {home:?}"
        );
        assert_eq!(far, home, "{label}: the rounds far off and turned");
    }
}

/// **What a point of the shell buys: a far-off body's pads are its pads
/// at home.** Read about a vertex of the shell's first quadrature face,
/// a quadrature face's lever is at most the extent of the shell's
/// quadrature faces wherever the body sits, so a body carries the width
/// it carries at home. Read about the
/// world origin, this pose's spiric faces' widths grew with the
/// placement's 990 m, to 35 times the target at 1e-12, and three ops
/// refused. Far off and turned, every op's pad is its pad at home to the
/// rounding of the moved coordinates: measured at most 0.21% more at
/// 1e-12, 2e-6 relative at 1e-9 and 1e-6.
#[test]
fn every_op_far_off_and_turned_keeps_its_pads_at_home() {
    let tol = Tol::witness();
    for ((label, home), (_, far)) in built(0.0).into_iter().zip(built(100.0)) {
        let pad = |b: &AtRestBody<f64>| topo::mass_properties(b, tol).unwrap().volume_pad;
        let (home, far) = (pad(&home), pad(&far));
        assert!(
            home > 0.0 && far <= home * 1.01,
            "{label}: the pad far off and turned {far:e} against at home {home:e}"
        );
    }
}

/// **One solid stored in two face orders carries one pad.** `T ∪ P` and
/// `P ∪ T` with a 900 m bar are one solid in two face orders. Each face
/// is read about the start of its shell's first quadrature face, a vertex
/// beside the torus in either order, so the quadrature faces' lever is
/// the torus's extent and not the bar's. Measured 1.04× apart at every ε
/// row (4.27e-6 against 4.10e-6 at 1e-9). Read about the shell's first
/// face's start, the far end of the bar in one order, they sat 14.3×
/// apart at 1e-9, 15.9× at 1e-12 and 911× at 1e-6.
#[test]
fn one_solid_in_two_face_orders_carries_one_pad() {
    let tol = Tol::witness();
    let t = torus(2.0, 0.5);
    let p = finished(
        "a 900 m bar",
        framed_bar(
            Point3::new(0.3, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            0.0,
            900.0,
            12.0,
        ),
        tol,
    );
    let pad = |x: &AtRestBody<f64>, y: &AtRestBody<f64>| {
        let got = topo::union(x, y, tol).unwrap();
        topo::mass_properties(&got.body().unwrap().body, tol)
            .unwrap()
            .volume_pad
    };
    let (tp, pt) = (pad(&t, &p), pad(&p, &t));
    let ratio = tp.max(pt) / tp.min(pt);
    assert!(
        ratio <= 2.0,
        "T ∪ P pad {tp:e}, P ∪ T pad {pt:e}: {ratio} apart"
    );
}
