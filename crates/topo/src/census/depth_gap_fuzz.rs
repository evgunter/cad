//! **The depth bounds are bounds**: [`depth_gap`]'s closed forms for a
//! depth's greatest and least value over a ring or a segment, against
//! the exact depth of points sampled on it.
//!
//! A shape-1 counterexample search: random rings and segments of every
//! pose against random axes, centres and core circles, half the rings
//! coaxial with the depth's own axis, the pose the census meets most
//! and where the bounds are tight. A ring is sampled inside its tube
//! and on the tube's surface, where its depths are extreme, and a
//! quarter of the draws stand 1e3 to 1e4 from the origin. A sample past
//! a bound is a clear the census would grant across a face it does not
//! hold.
//!
//! Two lanes. In `f64` a bound may round past a sample by at most
//! [`REL`] of the draw's scale. In `Interval` the bound encloses the
//! closed form's exact value, so only the samples' own rounding stands
//! between them.

#![allow(clippy::unwrap_used)]

test_utils::gated_to![
    "crates/topo/src/census/depth_gap.rs",
    "crates/topo/src/census.rs",
    "crates/geom-core/src/real.rs",
    "crates/geom-core/src/interval.rs",
    "crates/geom-core/src/linalg/",
];

use geom_core::{Bounds, Interval, Point3, Vec3};
use test_utils::fuzz::Rng;

use super::depth_gap::{Depth, Piece, Ring, exact, inf_on, split, sup_on};

/// How far an `f64` bound may round past a sample, relative to the
/// draw's scale (`1 +` its largest coordinate, plus its reach). The
/// worst of four runs at effort 400 stood 1.6e-15 past.
const REL: f64 = 1e-14;

/// How far a sample's own rounding may carry it off its piece,
/// relative to the draw's scale, in the `Interval` lane: the worst of
/// four runs at effort 400 (240 000 draws each) stood 1.1e-15 past, at
/// seed `0x7925458d5bba5dab`; a run at effort 1 typically sees ~1e-17.
const SAMPLE_REL: f64 = 5e-15;

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

fn centre(r: &mut Rng, far: Vec3<f64>) -> Point3<f64> {
    p(r.range(-2.0, 2.0), r.range(-2.0, 2.0), r.range(-2.0, 2.0)) + far
}

/// One draw: a depth, a piece, points sampled on the piece, and the
/// draw's scale.
fn draw(r: &mut Rng, i: usize) -> (Depth<f64>, Piece<f64>, Vec<Point3<f64>>, f64) {
    let far = if i % 4 == 3 {
        let mut c = || r.range(1.0e3, 1.0e4) * if r.unit() < 0.5 { -1.0 } else { 1.0 };
        Vec3::new(c(), c(), c())
    } else {
        Vec3::new(0.0, 0.0, 0.0)
    };
    let scale = 1.0 + far.x.abs().max(far.y.abs()).max(far.z.abs()) + 4.0;
    let n = unit(r);
    let coaxial = i.is_multiple_of(2);
    let axis = if coaxial { n } else { unit(r) } * r.range(0.5, 2.0);
    let origin = centre(r, far);
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
        let (a, b) = (centre(r, far), centre(r, far));
        let samples = (0..=64)
            .map(|k| a + (b - a) * (f64::from(k) / 64.0))
            .collect();
        return (depth, Piece::Segment(a, b), samples, scale);
    }
    let ring = Ring {
        center: if coaxial {
            origin + n * r.range(-1.0, 1.0)
        } else {
            centre(r, far)
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
    let samples = (0..128)
        .map(|k| {
            let th = f64::from(k / 2) * core::f64::consts::TAU / 64.0 + r.range(0.0, 0.1);
            let radial = e1 * th.cos() + e2 * th.sin();
            let on = ring.center + radial * ring.major;
            if k % 2 == 0 {
                on + unit(r) * (ring.minor * r.unit())
            } else {
                // On the tube's surface, in the circle's meridian plane.
                let phi = r.range(0.0, core::f64::consts::TAU);
                on + (radial * phi.cos() + n * phi.sin()) * ring.minor
            }
        })
        .collect();
    (depth, Piece::Ring(ring), samples, scale)
}

fn iv(x: f64) -> Interval {
    Interval::from_bounds(x, x)
}

fn ip(x: Point3<f64>) -> Point3<Interval> {
    Point3::new(iv(x.x), iv(x.y), iv(x.z))
}

fn ivec(x: Vec3<f64>) -> Vec3<Interval> {
    x.map(iv)
}

fn lift_depth(d: Depth<f64>) -> Depth<Interval> {
    match d {
        Depth::Axis { origin, axis } => Depth::Axis {
            origin: ip(origin),
            axis: ivec(axis),
        },
        Depth::Center { center } => Depth::Center { center: ip(center) },
        Depth::Core {
            center,
            axis,
            major,
        } => Depth::Core {
            center: ip(center),
            axis: ivec(axis),
            major: iv(major),
        },
    }
}

fn lift_piece(piece: Piece<f64>) -> Piece<Interval> {
    match piece {
        Piece::Ring(ring) => Piece::Ring(Ring {
            center: ip(ring.center),
            normal: ivec(ring.normal),
            major: iv(ring.major),
            minor: iv(ring.minor),
        }),
        Piece::Segment(a, b) => Piece::Segment(ip(a), ip(b)),
    }
}

#[test]
fn no_sampled_depth_lies_past_its_bounds() {
    let mut r = test_utils::fuzz::start("census_depth_gap_bounds");
    let (mut over, mut under) = (f64::NEG_INFINITY, f64::NEG_INFINITY);
    let mut sampled = f64::NEG_INFINITY;
    for i in 0..test_utils::fuzz::scaled(600) {
        let (depth, piece, samples, scale) = draw(&mut r, i);
        let hi = sup_on(depth, &piece);
        if hi.is_none() {
            assert!(
                matches!((depth, piece), (Depth::Core { .. }, Piece::Segment(..))),
                "draw {i}: no upper bound on {piece:?} for {depth:?}; {}",
                test_utils::fuzz::replay()
            );
        }
        let lo = inf_on(depth, &piece);
        let (idepth, ipiece) = (lift_depth(depth), lift_piece(piece));
        let (ihi, ilo) = (sup_on(idepth, &ipiece), inf_on(idepth, &ipiece));
        assert_eq!(
            (ihi.is_some(), ilo.is_some()),
            (hi.is_some(), lo.is_some()),
            "draw {i}: the lanes claim alike; {}",
            test_utils::fuzz::replay()
        );
        for x in samples {
            let d = exact(depth, x);
            let id = exact(idepth, ip(x));
            if let (Some(hi), Some(ihi)) = (hi, ihi) {
                over = over.max((d - hi) / scale);
                assert!(
                    d - hi <= REL * scale,
                    "draw {i}: {x:?} on {piece:?} is {:e} deeper than the bound {hi} on \
                     {depth:?}; {}",
                    d - hi,
                    test_utils::fuzz::replay()
                );
                sampled = sampled.max((id.lo() - ihi.hi()) / scale);
                assert!(
                    id.lo() - ihi.hi() <= SAMPLE_REL * scale,
                    "draw {i}, Interval: {x:?} on {piece:?} is {:e} deeper than the bound \
                     {ihi:?} on {depth:?}; {}",
                    id.lo() - ihi.hi(),
                    test_utils::fuzz::replay()
                );
            }
            if let (Some(lo), Some(ilo)) = (lo, ilo) {
                under = under.max((lo - d) / scale);
                assert!(
                    lo - d <= REL * scale,
                    "draw {i}: {x:?} on {piece:?} is {:e} shallower than the bound {lo} on \
                     {depth:?}; {}",
                    lo - d,
                    test_utils::fuzz::replay()
                );
                sampled = sampled.max((ilo.lo() - id.hi()) / scale);
                assert!(
                    ilo.lo() - id.hi() <= SAMPLE_REL * scale,
                    "draw {i}, Interval: {x:?} on {piece:?} is {:e} shallower than the bound \
                     {ilo:?} on {depth:?}; {}",
                    ilo.lo() - id.hi(),
                    test_utils::fuzz::replay()
                );
            }
        }
    }
    println!(
        "[fuzz] the worst samples stood {over:e} past the upper and {under:e} past the lower, \
         and {sampled:e} past an Interval bound, relative to their draws' scale"
    );
}
