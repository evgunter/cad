//! FILLET-RIM review probe (r1), interval half — the rim door on
//! carriers whose enclosures are WIDE, which is the case
//! `rim_of_rows_interval.rs` cannot reach: the waisted revolve's
//! dyadic coordinates store point enclosures, so "both bracket ends
//! agree" is exercised there only trivially.
//!
//! A rigid rotation by `π/2` about `x` at the certified scalar turns
//! every carrier's centre and axis into enclosures with `lo != hi`
//! (`cos(π/2)` is an interval around zero). The door's claim is that
//! one rim's arcs still match bit for bit — because the producer
//! applies one deterministic arithmetic to identical stored inputs —
//! so every rim of the rotated body is still two arcs, from either
//! seed, as a rotation.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;

use geom::Curve3;
use geom_core::{Affine3, Bounds, Interval, Point3, Real, Tol, Vec3};
use sweep::test_support::waisted_at;
use topo::query::rim_of;
use topo::{Body, EdgeKey};

fn is_rotation(a: &[EdgeKey], b: &[EdgeKey]) -> bool {
    a.len() == b.len() && (0..a.len()).any(|k| (0..a.len()).all(|i| a[(i + k) % a.len()] == b[i]))
}

/// Every circle edge whose two sides are different surfaces.
fn rim_arcs(body: &Body<Interval>) -> Vec<EdgeKey> {
    let surface_of = |he| {
        let l = body.get_half_edge(he).unwrap().parent_loop;
        body.get_face(body.get_loop(l).unwrap().face)
            .unwrap()
            .surface
    };
    body.edges()
        .filter(|(_, e)| {
            let c = body.get_curve_geom(e.curve).and_then(|g| g.certified());
            matches!(c.map(|c| c.carrier()), Some(Curve3::Circle { .. }))
                && surface_of(e.he_plus) != surface_of(e.he_minus)
        })
        .map(|(k, _)| k)
        .collect()
}

#[test]
fn wide_enclosures_from_one_producer_still_match_bit_for_bit() {
    let tol = Tol::witness();
    let plain = waisted_at::<Interval>(tol);
    let turned = topo::transform_rigid(
        &plain,
        &Affine3::rotation_about_axis(
            Point3::new(Interval::zero(), Interval::zero(), Interval::zero()),
            Vec3::new(Interval::one(), Interval::zero(), Interval::zero()),
            Interval::from_f64(core::f64::consts::FRAC_PI_2),
        ),
        tol,
    )
    .expect("the waisted revolve turns");

    // Not vacuous: the turned carriers really are wide.
    let mut wide = 0usize;
    for (_, e) in turned.edges() {
        if let Some(Curve3::Circle { center, axis, .. }) = turned
            .get_curve_geom(e.curve)
            .and_then(|g| g.certified())
            .map(|c| c.carrier())
        {
            for x in [center.x, center.y, center.z, axis.x, axis.y, axis.z] {
                if x.lo() != x.hi() {
                    wide += 1;
                }
            }
        }
    }
    assert!(
        wide > 0,
        "the rotation widens at least one stored component"
    );

    let arcs = rim_arcs(&turned);
    assert_eq!(arcs.len(), 6, "three seam-split rims, six arcs");
    let mut rims: BTreeSet<Vec<EdgeKey>> = BTreeSet::new();
    for &seed in &arcs {
        let rim = rim_of(&turned, seed)
            .unwrap_or_else(|e| panic!("a turned rim is still one rim from {seed:?}, got {e}"));
        assert_eq!(rim.len(), 2, "still seam-split");
        assert_eq!(rim[0], seed);
        for &other in &rim {
            let from_other = rim_of(&turned, other).unwrap();
            assert!(is_rotation(&rim, &from_other));
        }
        let mut key = rim.clone();
        key.sort();
        rims.insert(key);
    }
    assert_eq!(rims.len(), 3, "three rims");
}
