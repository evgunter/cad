//! **The depth bounds are bounds**: [`depth_gap`]'s closed forms for a
//! depth's greatest and least value over a ring or a segment, against
//! the exact depth of points sampled on it.
//!
//! A shape-1 counterexample search: random rings and segments of every
//! pose against random axes, centres and core circles, half the rings
//! coaxial with the depth's own axis, the pose the census meets most
//! and where the bounds are tight. A sample past a bound is a clear the
//! census would grant across a face it does not hold.

#![allow(clippy::unwrap_used)]

test_utils::gated_to![
    "crates/topo/src/census.rs",
    "crates/geom-core/src/real.rs",
    "crates/geom-core/src/linalg/",
];

use geom_core::{Point3, Vec3};
use test_utils::fuzz::Rng;

use super::depth_gap::{Depth, Piece, Ring, exact, inf_on, split, sup_on};

fn p(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

fn unit(r: &mut Rng) -> Vec3<f64> {
    loop {
        let w = Vec3::new(r.range(-1.0, 1.0), r.range(-1.0, 1.0), r.range(-1.0, 1.0));
        if w.norm() > 0.1 && w.norm() < 1.0 {
            return w / w.norm();
        }
    }
}

fn centre(r: &mut Rng) -> Point3<f64> {
    p(r.range(-2.0, 2.0), r.range(-2.0, 2.0), r.range(-2.0, 2.0))
}

/// One draw: a depth and a piece, and points sampled on the piece.
fn draw(r: &mut Rng, i: usize) -> (Depth<f64>, Piece<f64>, Vec<Point3<f64>>) {
    let n = unit(r);
    let coaxial = i.is_multiple_of(2);
    let axis = if coaxial { n } else { unit(r) } * r.range(0.5, 2.0);
    let origin = centre(r);
    let depth = match i % 3 {
        0 => Depth::Axis { origin, axis },
        1 => Depth::Center { center: origin },
        _ => Depth::Core {
            center: origin,
            axis,
            major: r.range(0.0, 2.0),
        },
    };
    if i % 7 == 6 {
        let (a, b) = (centre(r), centre(r));
        let samples = (0..=64)
            .map(|k| a + (b - a) * (f64::from(k) / 64.0))
            .collect();
        return (depth, Piece::Segment(a, b), samples);
    }
    let ring = Ring {
        center: if coaxial {
            origin + n * r.range(-1.0, 1.0)
        } else {
            centre(r)
        },
        normal: n * r.range(0.5, 2.0),
        major: r.range(0.0, 2.0),
        minor: if i.is_multiple_of(5) {
            0.0
        } else {
            r.range(0.0, 0.5)
        },
    };
    let e1 = split(unit(r), n).0;
    let e1 = e1 / e1.norm();
    let e2 = n.cross(e1);
    let samples = (0..64)
        .map(|k| {
            let th = f64::from(k) * core::f64::consts::TAU / 64.0 + r.range(0.0, 0.1);
            let on = ring.center + (e1 * th.cos() + e2 * th.sin()) * ring.major;
            on + unit(r) * (ring.minor * r.unit())
        })
        .collect();
    (depth, Piece::Ring(ring), samples)
}

#[test]
fn no_sampled_depth_lies_past_its_bounds() {
    let mut r = test_utils::fuzz::start("census_depth_gap_bounds");
    let (mut over, mut under) = (f64::NEG_INFINITY, f64::NEG_INFINITY);
    for i in 0..test_utils::fuzz::scaled(600) {
        let (depth, piece, samples) = draw(&mut r, i);
        let hi = sup_on(depth, &piece);
        if hi.is_none() {
            assert!(
                matches!((depth, piece), (Depth::Core { .. }, Piece::Segment(..))),
                "draw {i}: no upper bound on {piece:?} for {depth:?}; {}",
                test_utils::fuzz::replay()
            );
        }
        let lo = inf_on(depth, &piece);
        for x in samples {
            let d = exact(depth, x);
            if let Some(hi) = hi {
                over = over.max(d - hi);
                assert!(
                    d - hi <= 1e-12 * (1.0 + hi.abs()),
                    "draw {i}: {x:?} on {piece:?} is {:e} deeper than the bound {hi} on \
                     {depth:?}; {}",
                    d - hi,
                    test_utils::fuzz::replay()
                );
            }
            if let Some(lo) = lo {
                under = under.max(lo - d);
                assert!(
                    lo - d <= 1e-12 * (1.0 + lo.abs()),
                    "draw {i}: {x:?} on {piece:?} is {:e} shallower than the bound {lo} on \
                     {depth:?}; {}",
                    lo - d,
                    test_utils::fuzz::replay()
                );
            }
        }
    }
    println!("[fuzz] the worst samples stood {over:e} past the upper and {under:e} past the lower");
}
