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
//! lane copies its edge on the edge's own side, the ellipse's own curve.
//! Past the copy, at spins `(0, 0)` every op builds a sound body at its
//! closed form, but its certified enclosure is wider than
//! `differential`'s budget; at every other pose every op refuses
//! `UnpairedLooseEnds`
//! (`honejoin/the-edge-plane-lane-copies-no-ellipse-edge`). The pins
//! name those endings exactly, so a fix to either moves them.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};

use crate::common::differential::{VOLUME_CLAIM, outcome};
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
        topo::transform_rigid(&rod, &geom_core::test_support::rigid(&m, tol), tol).unwrap(),
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
        topo::transform_rigid(&slab, &geom_core::test_support::rigid(&tilt, tol), tol).unwrap(),
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
        topo::transform_rigid(
            &rod(spin, (x0, 3.0)),
            &geom_core::test_support::rigid(&m, tol),
            tol,
        )
        .unwrap(),
        tol,
    )
}

/// The volume `A` and `B` share for `B` from `x0`.
fn shared(x0: f64) -> f64 {
    let f = |x: f64| x * x.acos() - (1.0 - x * x).sqrt() + (1.0 - x * x).powf(1.5) / 3.0;
    f(1.0) - f(x0)
}

/// How one op ends past the edge-plane lane's copy of `A`'s ellipse
/// edge.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Ending {
    /// A body passing tiers 2 and 3′, the at-rest certificate and the
    /// legal-operand check, the closed form within its certified volume
    /// enclosure ([`sound`]).
    Builds,
    /// The join's loose ends left unpaired.
    LooseEnds,
}

/// Whether `bb` passes tiers 2 and 3′, the at-rest certificate and the
/// legal-operand check (it unites with a far brick), with `volume`
/// within its certified enclosure.
fn sound(bb: &topo::BooleanBody<f64>, volume: f64, tol: Tol) -> bool {
    let far = finished(
        "the far brick",
        brick((50.0, 51.0), (50.0, 51.0), (50.0, 51.0), tol),
        tol,
    );
    let m = topo::test_support::pinned_volume("the result", &bb.body, tol);
    topo::validate_closed(&bb.body).is_ok()
        && topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol).is_ok()
        && topo::validate_geometric_certificate(&bb.body, tol).is_ok()
        && topo::union(&bb.body, &far, tol).is_ok()
        && (m.volume - volume).abs() <= VOLUME_CLAIM + m.volume_pad
}

/// The lines of the six ops at `(spin A, spin B, x0)` that do not end
/// as `want`, each with what it did against its closed form.
fn not_ending((sa, sb, x0): (f64, f64, f64), want: Ending) -> Vec<String> {
    let tol = Tol::witness();
    let (a, b) = (mitred(sa), crossing(sb, x0));
    let (va, vb, s) = (3.0 * PI, (3.0 - x0) * PI, shared(x0));
    let mut lines = Vec::new();
    for (op, r, volume) in [
        ("A ∪ B", topo::union(&a, &b, tol), va + vb - s),
        ("B ∪ A", topo::union(&b, &a, tol), va + vb - s),
        ("A ∩ B", topo::intersect(&a, &b, tol), s),
        ("B ∩ A", topo::intersect(&b, &a, tol), s),
        ("A ∖ B", topo::subtract(&a, &b, tol), va - s),
        ("B ∖ A", topo::subtract(&b, &a, tol), vb - s),
    ] {
        let ending = match &r {
            Err(BooleanError::Join(j))
                if matches!(j.error(), SplitJoinError::UnpairedLooseEnds { .. }) =>
            {
                Some(Ending::LooseEnds)
            }
            Ok(BooleanResult::Body(bb)) if sound(bb, volume, tol) => Some(Ending::Builds),
            _ => None,
        };
        if ending != Some(want) {
            lines.push(format!(
                "spins ({sa}, {sb}), B from x = {x0} | {op} | {}",
                outcome(r, volume, tol)
            ));
        }
    }
    lines
}

/// **The crossing pair is not read as parallel**: at the row's pose
/// every op in both orders gets past the edge-plane lane's copy of the
/// ellipse edge and refuses `UnpairedLooseEnds`, a refusal on valid
/// input that `honejoin/the-edge-plane-lane-copies-no-ellipse-edge`
/// owns, and none at the parallel pair's radical plane (`JoinDesync`).
/// An op that builds, or refuses otherwise, moves this pin with that
/// row.
#[test]
fn crossing_rods_end_past_the_ellipse_edge_copy() {
    let moved = not_ending((0.4, 0.3, 0.2), Ending::LooseEnds);
    assert!(
        moved.is_empty(),
        "the-edge-plane-lane-copies-no-ellipse-edge: every op leaves loose ends\n{}",
        moved.join("\n")
    );
}

/// [`crossing_rods_end_past_the_ellipse_edge_copy`] at two more spin
/// pairs, both seam rulings at `±x` among them, and with `B` from
/// `x = 0.5`. With both seams at `±x`, every op builds.
#[test]
fn crossing_rods_end_past_the_ellipse_edge_copy_at_every_spin() {
    let moved: Vec<String> = [
        ((1.0, -0.7, 0.2), Ending::LooseEnds),
        ((0.0, 0.0, 0.2), Ending::Builds),
        ((0.4, 0.3, 0.5), Ending::LooseEnds),
    ]
    .into_iter()
    .flat_map(|(pose, want)| not_ending(pose, want))
    .collect();
    assert!(
        moved.is_empty(),
        "the-edge-plane-lane-copies-no-ellipse-edge: each pose ends as pinned\n{}",
        moved.join("\n")
    );
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
