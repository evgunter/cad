//! **A cone against a ball, and against another cone, in general pose,
//! in every operation and order.** The cone `K` of base radius `1` at
//! `y = 0` and apex `(0, 2, 0)` (half-angle `atan ½`), its seam
//! meridians on `±z`, against:
//!
//! - the ball of radius `0.4` about `(0.9, 0.6, 0)`, its poles along
//!   `z` (at `(0.9, 0.6, ±0.4)`, off `K`, whose radius is `0.7` there),
//!   off the cone's axis, `0.18` outside its side's meridian line, clear
//!   of its base plane and of its apex: one null loop on the cone's side,
//!   the section value's general cone × sphere pose, marched;
//! - `K` turned a quarter about `z`, its apex moved to `(0.3, 0.8, 0)`
//!   inside `K` and its axis along `+x`: its side leaves `K`'s side in
//!   one loop about its own axis, both apexes clear of the other cone,
//!   the general cone × cone pose, marched.
//!
//! Each run's outcome is pinned at the default ε, `built` or the refusal
//! whole (its operand, face and kind), so the row goes red the day either
//! changes; the nightly reads the same pins at 1e-6 and 1e-12. Every run
//! of both builds, the section loop a ring on `K`'s face that the
//! carried chords close, each body valid at tier 3; where an order's
//! three runs all build, the volumes close as the operations must:
//! `|A ∪ B| + |A ∩ B| = |A| + |B|` and `|A ∖ B| = |A| − |A ∩ B|`, within
//! the measured pads. A desync or a broken invariant is red whatever the
//! pin says.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{ProfileLoop, RawLoop};
use sweep::test_support::{ball_poled_z, finished};
use sweep::{Revolution, revolve};
use topo::AtRestBody;

use crate::revolve_common::{axis_y, validated};

/// The cone `K`, moved by `pose`.
fn cone(pose: Affine3<f64>, tol: Tol) -> AtRestBody<f64> {
    let tri = ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.0),
        Point2::new(0.0, 2.0),
    ]);
    let body = revolve(&validated(vec![tri]), axis_y(), Revolution::Full, tol)
        .unwrap()
        .body;
    let spin = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_y(), FRAC_PI_2);
    finished(
        "the cone",
        topo::transform_rigid(&body, &geom_core::test_support::rigid(&(pose * spin), tol)).unwrap(),
        tol,
    )
}

fn ball(tol: Tol) -> AtRestBody<f64> {
    finished(
        "the ball",
        ball_poled_z(0.4, Vec3::new(0.9, 0.6, 0.0), tol),
        tol,
    )
}

/// `K` turned a quarter about `z` (its apex onto `(−2, 0, 0)`), then
/// moved so the apex stands at `(0.3, 0.8, 0)`.
fn crossing_cone(tol: Tol) -> AtRestBody<f64> {
    let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_z(), FRAC_PI_2);
    let to = Affine3::translation(Vec3::new(2.3, 0.8, 0.0));
    cone(to * turn, tol)
}

/// Each run's outcome, ∪, ∩, ∖ with the cone first, then with the other
/// first.
const PINNED: [&str; 6] = ["built"; 6];

fn volume(body: &AtRestBody<f64>, tol: Tol) -> (f64, f64) {
    let m = topo::mass_properties(body, tol, topo::test_support::pinned_resolution(tol))
        .unwrap_or_else(|e| panic!("the body does not measure: {e:?}"))
        .enclosure;
    (m.volume, m.volume_pad)
}

/// A run's outcome: its measured volume and pad (zero for an empty
/// result) and `built`, the body valid at tier 3, or the refusal whole
/// (its operand, face and kind). A desync, a broken invariant or an
/// invalid body is a defect, and red.
fn outcome(
    label: &str,
    got: Result<topo::BooleanResult<f64>, topo::BooleanError>,
    tol: Tol,
) -> (Option<(f64, f64)>, String) {
    match got {
        Ok(got) => (
            Some(got.body().map_or((0.0, 0.0), |b| {
                topo::validate_geometric(&b.body, tol)
                    .unwrap_or_else(|e| panic!("{label}: the built body is invalid: {e:?}"));
                volume(&b.body, tol)
            })),
            "built".to_owned(),
        ),
        Err(
            e @ (topo::BooleanError::JoinDesync { .. }
            | topo::BooleanError::ClassificationInvariant { .. }),
        ) => panic!("{label}: a defect: {e:?}"),
        Err(e) => {
            let shown = format!("{e:?}");
            println!("{label}: refused: {shown}");
            (None, shown)
        }
    }
}

/// Every op in both orders of `k` and `other`, each run's outcome against
/// `pinned` (∪, ∩, ∖ with the cone first, then with the other first), the
/// volumes closed where an order builds, `∩` a part of each.
fn every_order(
    what: &str,
    (k, other): (&AtRestBody<f64>, &AtRestBody<f64>),
    v_other: f64,
    pinned: [&str; 6],
    tol: Tol,
) {
    let v_cone = 2.0 * PI / 3.0;
    let (vk, pk) = volume(k, tol);
    let (vo, po) = volume(other, tol);
    assert!(
        (vk - v_cone).abs() <= pk + 1e-9 && (vo - v_other).abs() <= po + 1e-9,
        "{what}: the operands measure as their closed forms: {vk} ± {pk}, {vo} ± {po}"
    );
    let mut got = Vec::new();
    for (name, a, b, va, pa) in [
        ("the cone first", k, other, vk, pk),
        ("the other first", other, k, vo, po),
    ] {
        let (u, ou) = outcome(&format!("{what}, {name}: ∪"), topo::union(a, b, tol), tol);
        let (i, oi) = outcome(
            &format!("{what}, {name}: ∩"),
            topo::intersect(a, b, tol),
            tol,
        );
        let (d, od) = outcome(
            &format!("{what}, {name}: ∖"),
            topo::subtract(a, b, tol),
            tol,
        );
        got.extend([ou, oi, od]);
        let (Some((vu, pu)), Some((vi, pi)), Some((vd, pd))) = (u, i, d) else {
            continue;
        };
        assert!(
            vi > 0.0 && vi < vk.min(vo),
            "{what}, {name}: the two cross, so ∩ is a part of each ({vi})"
        );
        assert!(
            (vu + vi - vk - vo).abs() <= pu + pi + pk + po + 1e-9,
            "{what}, {name}: |A ∪ B| + |A ∩ B| = {} misses |A| + |B| = {}",
            vu + vi,
            vk + vo
        );
        assert!(
            (vd - (va - vi)).abs() <= pd + pa + pi + 1e-9,
            "{what}, {name}: |A ∖ B| = {vd} misses |A| − |A ∩ B| = {}",
            va - vi
        );
    }
    assert_eq!(got, pinned, "{what}: each run's outcome");
}

#[test]
fn a_cone_and_a_ball_off_its_axis_build_in_every_order() {
    let tol = Tol::witness();
    every_order(
        "the ball",
        (&cone(Affine3::identity(), tol), &ball(tol)),
        4.0 * PI * 0.4f64.powi(3) / 3.0,
        PINNED,
        tol,
    );
}

#[test]
fn two_cones_in_general_pose_build_in_every_order() {
    let tol = Tol::witness();
    every_order(
        "the crossing cone",
        (&cone(Affine3::identity(), tol), &crossing_cone(tol)),
        2.0 * PI / 3.0,
        PINNED,
        tol,
    );
}
