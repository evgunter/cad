//! **Two radius-1 rods on crossing axes, one mitred by their bisector
//! plane.**
//!
//! - `A`: a rod about `z` over `z ∈ [−3, 3]` less the half-space under
//!   `z = x`, so its bottom rim is the ellipse `z = x` on its wall;
//! - `B`: a rod about `x` over `x ∈ [x0, 3]`. Its wall holds `A`'s rim,
//!   since `z = x` is a bisector plane of the two walls; its end cap at
//!   `x0 > 0` keeps the second ellipse `z = −x` and the walls' tangent
//!   points `(0, ±1, 0)` out of both solids.
//!
//! Each rod is spun about its own axis, which moves its seam rulings.
//! The closed forms: `vol A = 3π`, `vol B = (3 − x0)π`, and over
//! `x ∈ [x0, 1]` the two share the part of `B`'s disc above `z = x`,
//! `acos x − x√(1 − x²)`, whose integral is
//! `[x·acos x − √(1 − x²) + (1 − x²)^{3/2}/3]` from `x0` to `1`
//! ([`shared`]).
//!
//! The germ runs along `A`'s ellipse edge inside `B`'s wall, so the join
//! reads the walls' axes, finds them apart, and hands the germ to the
//! edge-plane lane rather than the parallel pair's radical plane. That
//! lane copies its edge on the edge's own side, which it does for a line
//! or a circle and not for an ellipse
//! (`cleave/the-edge-plane-lane-copies-no-ellipse-edge`): every op
//! refuses there, and is held to its closed form once it builds.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};

use crate::common::differential::outcome;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::test_support::{brick, finished};
use topo::{AtRestBody, BooleanError, BooleanResult, SplitJoinError};

/// A rod of radius 1 about `z` over `z ∈ [z0, z1]`, spun `spin` about
/// `z`.
fn rod(spin: f64, (z0, z1): (f64, f64)) -> AtRestBody<f64> {
    let tol = Tol::witness();
    let rod = sweep::test_support::prism_at(
        vec![(Point2::new(-1.0, 0.0), 1.0), (Point2::new(1.0, 0.0), 1.0)],
        z0,
        z1 - z0,
        tol,
    );
    let rod = finished("the rod", rod, tol);
    let m = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_z(), spin);
    finished(
        "the spun rod",
        topo::transform_rigid(&rod, &m, tol).unwrap(),
        tol,
    )
}

/// `A`: [`rod`] over `z ∈ [−3, 3]` less the half-space under `z = x`.
fn mitred(spin: f64) -> AtRestBody<f64> {
    let tol = Tol::witness();
    let tilt = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_y(), -PI / 4.0);
    let slab = brick((-50.0, 50.0), (-50.0, 50.0), (-100.0, 0.0), tol);
    let under = finished(
        "the half-space under z = x",
        topo::transform_rigid(&slab, &tilt, tol).unwrap(),
        tol,
    );
    match topo::subtract(&rod(spin, (-3.0, 3.0)), &under, tol) {
        Ok(BooleanResult::Body(b)) => b.body,
        other => panic!("the mitred rod builds: {other:?}"),
    }
}

/// `B`: [`rod`] over `[x0, 3]` turned onto `x`.
fn crossing(spin: f64, x0: f64) -> AtRestBody<f64> {
    let tol = Tol::witness();
    let m = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_y(), FRAC_PI_2);
    finished(
        "the crossing rod",
        topo::transform_rigid(&rod(spin, (x0, 3.0)), &m, tol).unwrap(),
        tol,
    )
}

/// The volume `A` and `B` share for `B` from `x0`.
fn shared(x0: f64) -> f64 {
    let f = |x: f64| x * x.acos() - (1.0 - x * x).sqrt() + (1.0 - x * x).powf(1.5) / 3.0;
    f(1.0) - f(x0)
}

/// What the edge-plane lane's copy of `A`'s ellipse edge refuses
/// (`chord_join::along_edge_spec`).
const NO_ELLIPSE_COPY: &str = "a section segment along an edge whose carrier has no chord lane";

/// The lines of the six ops at `(spin A, spin B, x0)` that neither build
/// SOUND at their closed form nor refuse at the edge-plane lane's copy.
fn wrong((sa, sb, x0): (f64, f64, f64)) -> Vec<String> {
    let tol = Tol::witness();
    let (a, b) = (mitred(sa), crossing(sb, x0));
    let (va, vb, s) = (3.0 * PI, (3.0 - x0) * PI, shared(x0));
    let mut lines = Vec::new();
    for (op, r, want) in [
        ("A ∪ B", topo::union(&a, &b, tol), va + vb - s),
        ("B ∪ A", topo::union(&b, &a, tol), va + vb - s),
        ("A ∩ B", topo::intersect(&a, &b, tol), s),
        ("B ∩ A", topo::intersect(&b, &a, tol), s),
        ("A ∖ B", topo::subtract(&a, &b, tol), va - s),
        ("B ∖ A", topo::subtract(&b, &a, tol), vb - s),
    ] {
        let at_the_copy = matches!(
            &r,
            Err(BooleanError::Join(SplitJoinError::SectionInvariant { what, .. }))
                if what.starts_with(NO_ELLIPSE_COPY)
        );
        let line = outcome(r, want, tol);
        if !(line.starts_with("OK SOUND") || at_the_copy) {
            lines.push(format!(
                "spins ({sa}, {sb}), B from x = {x0} | {op} | {line}"
            ));
        }
    }
    lines
}

/// **The crossing pair is not read as parallel**: at the row's pose
/// every op in both orders builds SOUND at its closed form or refuses at
/// the edge-plane lane's copy, and none reaches the parallel pair's
/// radical plane (`JoinDesync`).
#[test]
fn crossing_rods_take_the_edge_plane_lane() {
    let bad = wrong((0.4, 0.3, 0.2));
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// [`crossing_rods_take_the_edge_plane_lane`] at two more spin pairs,
/// both seam rulings at `±x` among them, and with `B` from `x = 0.5`.
#[test]
fn crossing_rods_take_the_edge_plane_lane_at_every_spin() {
    let bad: Vec<String> = [(1.0, -0.7, 0.2), (0.0, 0.0, 0.2), (0.4, 0.3, 0.5)]
        .into_iter()
        .flat_map(wrong)
        .collect();
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// **The shared volume's closed form agrees with a slicing**: Simpson
/// over `x` of the area of `B`'s unit disc above `z = x`, itself
/// integrated by Simpson over `z`.
#[test]
fn the_shared_closed_form_agrees_with_a_slicing() {
    let simpson = |f: &dyn Fn(f64) -> f64, lo: f64, hi: f64| {
        let n = 2000;
        let h = (hi - lo) / f64::from(n);
        let sum: f64 = (0..=n)
            .map(|i| {
                let w = if i == 0 || i == n {
                    1.0
                } else if i % 2 == 1 {
                    4.0
                } else {
                    2.0
                };
                w * f(lo + h * f64::from(i))
            })
            .sum();
        sum * h / 3.0
    };
    for x0 in [0.2, 0.5] {
        let area = |x: f64| simpson(&|z: f64| 2.0 * (1.0 - z * z).max(0.0).sqrt(), x, 1.0);
        let sliced = simpson(&area, x0, 1.0);
        assert!(
            (sliced - shared(x0)).abs() < 1e-5,
            "x0 {x0}: sliced {sliced}, closed form {}",
            shared(x0)
        );
    }
}
