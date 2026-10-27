//! **A block resting a Zero-band gap above another unions.** `B` stands
//! on `A = [0, 1]³` with its bottom a gap `g` inside the Zero band above
//! `A`'s top, or overlapping it by as little: the glue decides the two
//! planes one carrier and keeps the first operand's description for the
//! merged face, so the result is the true union to within `g` over the
//! glued area. Each union builds in both operand orders, holds tiers 3
//! and 3′ and one solid, measures at the union's closed form to within
//! that, and holds the volume relations
//! (`topo::test_support::volume_relations`). The same holds for a wedge
//! standing on a block's top with its face tilted inside the Zero band
//! over it.
//!
//! Gaps are spelled in ε, so each row is the same pose at every ε; two
//! more put `B`'s bottom one and four ulp above `A`'s top.

#![allow(clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use topo::test_support::{assert_volume_relations, brick, finished, wedge_on_a_block_top};
use topo::{AtRestBody, BooleanOp, BooleanResult};

/// `B`'s height.
const H: f64 = 0.5;

/// A footprint: its name and its `x` and `y` spans.
type Footprint = (&'static str, (f64, f64), (f64, f64));

/// The footprints `B` stands on: inside `A`'s top, its whole top, and
/// overhanging it by half.
const FOOTPRINTS: [Footprint; 3] = [
    ("inside", (0.25, 0.75), (0.25, 0.75)),
    ("whole", (0.0, 1.0), (0.0, 1.0)),
    ("overhanging", (-0.5, 0.5), (0.25, 0.75)),
];

/// The union of `x` and `y` as built, held to tiers 3 and 3′ and to one
/// solid, its volume relations held, and its volume.
fn unites(what: &str, x: &AtRestBody<f64>, y: &AtRestBody<f64>, tol: Tol) -> (f64, f64) {
    let bb = match topo::union(x, y, tol) {
        Ok(BooleanResult::Body(bb)) => bb,
        other => panic!("{what}: the union builds, got {:?}", other.map(|_| ())),
    };
    topo::validate_geometric(&bb.body, tol).unwrap_or_else(|e| panic!("{what}: tier 3: {e:?}"));
    topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol)
        .unwrap_or_else(|e| panic!("{what}: tier 3′: {e:?}"));
    assert_eq!(bb.body.solids().count(), 1, "{what}: one solid");
    assert_volume_relations(what, BooleanOp::Union, x, y, Some(&bb.body), tol);
    let m = topo::test_support::pinned_volume(what, &bb.body, tol);
    (m.volume, m.surface_area)
}

#[test]
fn a_block_resting_a_zero_band_gap_above_another_unions_at_its_volume() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let a = finished("A", brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol), tol);
    let ulp = |n: u64| f64::from_bits(1.0f64.to_bits() + n);
    let bottoms = [1e-4, 1e-3, 1e-2, 0.1, 0.5, -1e-3, -0.1]
        .map(|k| (format!("g = {k}ε"), 1.0 + k * eps))
        .into_iter()
        .chain([1, 4].map(|n| (format!("{n} ulp"), ulp(n))));
    let mut built = 0;
    for (gap, bottom) in bottoms {
        for (name, fx, fy) in FOOTPRINTS {
            let b = finished("B", brick(fx, fy, (bottom, bottom + H), tol), tol);
            let area = (fx.1 - fx.0) * (fy.1 - fy.0);
            // The footprint's overlap with `A`'s top: the area glued.
            let glued = (fx.1.min(1.0) - fx.0.max(0.0)) * (fy.1.min(1.0) - fy.0.max(0.0));
            // The true union, and what the glue may move it by.
            let want = 1.0 + area * H - glued * (1.0 - bottom).max(0.0);
            let moved = glued * (bottom - 1.0).abs();
            for (order, x, y) in [("A ∪ B", &a, &b), ("B ∪ A", &b, &a)] {
                let what = format!("{order}, {name}, {gap}");
                let (v, _) = unites(&what, x, y, tol);
                assert!(
                    (v - want).abs() <= moved + 1e-12,
                    "{what}: {v} against the union's {want}, moved by at most {moved:e}"
                );
                built += 1;
            }
        }
    }
    assert_eq!(built, 54, "every cell built");
}

/// **A wedge standing on a block's top, its face tilted inside the Zero
/// band** (`θ = k·ε`, `k ∈ {2e-3, 0.25}`), unions in both orders at
/// every opening, at the block's volume plus the wedge's `sin φ`, to
/// within ε over the result's boundary.
#[test]
fn a_wedge_tilted_inside_the_zero_band_on_a_block_unions_at_its_volume() {
    let tol = Tol::witness();
    let eps = tol.eps();
    for k in [2e-3, 0.25] {
        for opening in [2.0, 5.0, 20.0, 60.0, 90.0] {
            let (block, wedge) = wedge_on_a_block_top(opening, k * eps, 1.0, 0.0, tol);
            let (block, wedge) = (finished("block", block, tol), finished("wedge", wedge, tol));
            let want = 18.0 + f64::to_radians(opening).sin();
            for (order, x, y) in [
                ("block ∪ wedge", &block, &wedge),
                ("wedge ∪ block", &wedge, &block),
            ] {
                let what = format!("{order} at {opening}°, θ = {k}ε");
                let (v, area) = unites(&what, x, y, tol);
                assert!(
                    (v - want).abs() <= eps * area,
                    "{what}: {v} against {want}, beyond ε over the area {area}"
                );
            }
        }
    }
}
