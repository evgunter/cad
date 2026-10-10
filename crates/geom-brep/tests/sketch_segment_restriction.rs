//! A placed sketch segment's restriction, pinned at the certified
//! scalar: a split narrows the description's range and never touches
//! the authored segment, so however often an edge is split its
//! evaluation reads the authored `a`, `b` and carrier once.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use crate::shared::interval::iv;
use geom_brep::{MappedCurve, MappedSource, SketchSegment};
use geom_core::{Affine3, Arc2, Bounds, Interval, Point2, Point3, Real, Sym, Vec3};

fn width(e: Interval) -> f64 {
    e.hi() - e.lo()
}

fn point_width(p: Point3<Interval>) -> f64 {
    width(p.x).max(width(p.y)).max(width(p.z))
}

/// The near centre and the far one, a thousand metres out.
const NEAR: [f64; 2] = [0.0, 0.0];
const FAR: [f64; 2] = [1000.0, -700.0];

/// The half-turn unit arc about `c` from `c + (1, 0)` to `c − (1, 0)`,
/// or the chord between the same two points, under the identity
/// placement — every input an exact point.
fn segment(kind: &str, c: [f64; 2]) -> MappedCurve<Interval> {
    let p = |x: f64, y: f64| Point2::new(iv(c[0] + x), iv(c[1] + y));
    let segment = match kind {
        "line" => SketchSegment::Line {
            a: p(1.0, 0.0),
            b: p(-1.0, 0.0),
        },
        "arc" => SketchSegment::Arc {
            a: p(1.0, 0.0),
            b: p(-1.0, 0.0),
            arc: Arc2 {
                centre: p(0.0, 0.0),
                radius: iv(1.0),
                sweep: Interval::pi(),
            },
        },
        _ => unreachable!("an unnamed segment"),
    };
    MappedCurve::whole(MappedSource::PlacedSegment {
        segment,
        place: Affine3::translation(Vec3::new(iv(0.0), iv(0.0), iv(0.0))),
    })
}

/// The widest of the samples `s = 0, ½, 1`.
fn sampled_width(c: &MappedCurve<Interval>) -> f64 {
    [0.0, 0.5, 1.0]
        .into_iter()
        .map(|s| point_width(c.eval(iv(s))))
        .fold(0.0, f64::max)
}

/// A split parameter an intersection hands `restrict`, non-dyadic and
/// varying along the chain: `0.37 … 0.414`.
fn crossing(k: usize) -> f64 {
    #[allow(clippy::cast_precision_loss)]
    let wobble = ((k * 7) % 5) as f64;
    0.37 + 0.011 * wobble
}

/// The `k`-th split of the chain `name`, as `restrict`'s parameters.
fn chain(name: &str, k: usize) -> (Interval, Interval) {
    match name {
        "(0.3, 0.7)" => (iv(0.3), iv(0.7)),
        "(a, 1)" => (iv(crossing(k)), iv(1.0)),
        "(0, 1/2)" => (iv(0.0), iv(0.5)),
        _ => unreachable!("an unnamed chain"),
    }
}

/// `sampled_width` before any split and after each of 64.
fn widths_along(mut c: MappedCurve<Interval>, name: &str) -> Vec<f64> {
    let mut widths = vec![sampled_width(&c)];
    for k in 0..64 {
        let (s0, s1) = chain(name, k);
        c = c.restrict(s0, s1);
        widths.push(sampled_width(&c));
    }
    widths
}

/// **A segment's split chain stays at one evaluation's width**, for a
/// line and an arc, near and far, over 64 nested splits of three
/// chains: each ceiling is 4× the worst width this form measures over
/// its chain, rounded up. A segment re-derived through its own
/// evaluation at each split compounds: the line's `lerp` re-reads both
/// ends, so its width multiplies per split (9e-11 after 8 `(0.3, 0.7)`
/// splits far out, metres after 64), and the arc's re-turned start adds
/// two ulps of the coordinates per split (1.5e-11 far out after 64).
#[test]
fn nested_segment_splits_stay_at_one_evaluations_width() {
    let rows: [(&str, [f64; 2], &str, f64); 12] = [
        ("line", NEAR, "(0.3, 0.7)", 7e-14),
        ("line", NEAR, "(a, 1)", 1.2e-13),
        ("line", NEAR, "(0, 1/2)", 1.4e-15),
        ("line", FAR, "(0.3, 0.7)", 9.1e-13),
        ("line", FAR, "(a, 1)", 1.4e-12),
        ("line", FAR, "(0, 1/2)", 9.1e-13),
        ("arc", NEAR, "(0.3, 0.7)", 1.3e-13),
        ("arc", NEAR, "(a, 1)", 1.9e-13),
        ("arc", NEAR, "(0, 1/2)", 1.3e-14),
        ("arc", FAR, "(0.3, 0.7)", 1.4e-12),
        ("arc", FAR, "(a, 1)", 1.4e-12),
        ("arc", FAR, "(0, 1/2)", 9.1e-13),
    ];
    for (kind, at, name, ceiling) in rows {
        let widths = widths_along(segment(kind, at), name);
        let worst = widths.iter().copied().fold(0.0, f64::max);
        println!(
            "{kind} {at:?} {name}: N=0 {:e}, N=1 {:e}, N=8 {:e}, N=64 {:e}, worst {worst:e} (ceiling {ceiling:e})",
            widths[0], widths[1], widths[8], widths[64]
        );
        for (n, &w) in widths.iter().enumerate() {
            assert!(
                w <= ceiling,
                "{kind} at {at:?}, after {n} splits of {name} the stored width is {w:e}, over \
                 {ceiling:e}"
            );
        }
    }
}

/// Every whole source at `lift`'s scalar, each beside the unrestricted
/// form main evaluated it in, spelled out here: a placed segment is
/// the placement of the segment's own `eval(s)`, a revolved point the
/// rotation by `s·angle` of its placed point, an extruded one its
/// placed point plus `vec·s`. Answers the samples whose two spellings
/// `same` tells apart.
fn whole_range_mismatches<T: Real>(
    lift: impl Fn(f64) -> T,
    same: impl Fn(T, T) -> bool,
) -> Vec<String> {
    let mut mismatches = Vec::new();
    for at in [[0.0, 0.0, 0.0], [1000.0, -700.0, 300.0], [-3.0, 0.0, 0.0]] {
        let p2 = |x: f64, y: f64| Point2::new(lift(x), lift(y));
        let v3 = |x: f64, y: f64, z: f64| Vec3::new(lift(x), lift(y), lift(z));
        let place = Affine3::rotation_about_axis(
            Point3::new(lift(0.0), lift(0.0), lift(0.0)),
            v3(0.2, 1.0, -0.4),
            lift(0.7),
        ) * Affine3::translation(v3(at[0], at[1], at[2]));
        let placed = |p: Point2<T>| place.transform_point(Point3::new(p.x, p.y, lift(0.0)));
        let line = SketchSegment::Line {
            a: p2(1.0, 0.0),
            b: p2(-1.0, 0.5),
        };
        let arc = SketchSegment::Arc {
            a: p2(1.0, 0.0),
            b: p2(-1.0, 0.0),
            arc: Arc2 {
                centre: p2(0.0, 0.0),
                radius: lift(1.0),
                sweep: lift(PI),
            },
        };
        let (q, n, pt, angle, v) = (
            Point3::new(lift(at[0] + 1.0), lift(at[1] + 2.0), lift(at[2])),
            v3(0.3, -0.2, 1.0),
            p2(2.0, 2.0),
            lift(-1.9),
            v3(0.5, -1.5, 3.0),
        );
        for i in 0..=64 {
            let s = if i == 7 {
                lift(1.0) / lift(3.0)
            } else {
                lift(f64::from(i) / 64.0)
            };
            let rows = [
                (
                    "line",
                    MappedSource::PlacedSegment {
                        segment: line,
                        place,
                    },
                    placed(line.eval(s)),
                ),
                (
                    "arc",
                    MappedSource::PlacedSegment {
                        segment: arc,
                        place,
                    },
                    placed(arc.eval(s)),
                ),
                (
                    "rim",
                    MappedSource::RevolvedPoint {
                        point: pt,
                        place,
                        axis_origin: q,
                        axis_dir: n,
                        angle,
                    },
                    Affine3::rotation_about_axis(q, n, s * angle).transform_point(placed(pt)),
                ),
                (
                    "strut",
                    MappedSource::ExtrudedPoint {
                        point: pt,
                        place,
                        vec: v,
                    },
                    placed(pt) + v * s,
                ),
            ];
            for (name, source, want) in rows {
                let got = MappedCurve::whole(source).eval(s);
                if !(same(got.x, want.x) && same(got.y, want.y) && same(got.z, want.z)) {
                    mismatches.push(format!("{name} at {at:?}, sample {i}"));
                }
            }
        }
    }
    mismatches
}

/// **A whole range evaluates as the unrestricted source, bit for
/// bit**, at `f64`, at `Interval` and at `Sym<Interval>` (its numeric
/// channel and its expression node both), for a placed line, a placed
/// arc, a revolved and an extruded point, on a tilted placement near
/// and far: the whole range's `at(s)` is `s` itself, so nothing a body
/// builds unrestricted moves.
#[test]
fn a_whole_range_evaluates_as_the_unrestricted_source_bit_for_bit() {
    let at_f64 = whole_range_mismatches(|x| x, |a: f64, b: f64| a.to_bits() == b.to_bits());
    assert!(at_f64.is_empty(), "f64: {at_f64:?}");
    let bits = |a: Interval, b: Interval| {
        a.lo().to_bits() == b.lo().to_bits() && a.hi().to_bits() == b.hi().to_bits()
    };
    let at_interval = whole_range_mismatches(iv, bits);
    assert!(at_interval.is_empty(), "Interval: {at_interval:?}");
    let budget = geom_core::SymBudget {
        max_terms: 8,
        max_degree: 2,
    };
    let (at_sym, _) = geom_core::sym::with_session(budget, || {
        whole_range_mismatches(
            Sym::<Interval>::from_f64,
            |a: Sym<Interval>, b: Sym<Interval>| bits(a.value, b.value) && a.node() == b.node(),
        )
    });
    assert!(at_sym.is_empty(), "Sym<Interval>: {at_sym:?}");
}
