//! The directional extremum of a conic arc
//! (`geom::curves::boxes::conic_arc_extent_along` and its certified
//! twin): agreement with the coordinate box doors on the world axes,
//! exactness against dense sampling along oblique directions, and
//! containment at the interval scalar over wide brackets.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/geom/src/curves/",
    "crates/geom/src/curves.rs",
    "crates/geom-core/src/interval.rs",
    "crates/geom-core/src/real.rs",
    "crates/geom-core/src/dual.rs",
    "crates/geom-core/src/linalg/",
    "crates/bvh/src/aabb.rs",
];

use core::f64::consts::{PI, TAU};

use geom::Curve3;
use geom::curves::boxes::{
    circle_arc_aabb, conic_arc_extent_along, conic_arc_extent_along_bracketed, ellipse_arc_aabb,
};
use geom_core::{Bounds, Dual64, Interval, Point3, Real, Vec3};

fn circle(center: Point3<f64>, axis: Vec3<f64>, radius: f64, u_ref: Vec3<f64>) -> Curve3<f64> {
    Curve3::Circle {
        center,
        axis,
        radius,
        u_ref,
    }
}

fn ellipse(
    center: Point3<f64>,
    axis: Vec3<f64>,
    semi_u: f64,
    semi_v: f64,
    u_ref: Vec3<f64>,
) -> Curve3<f64> {
    Curve3::Ellipse {
        center,
        axis,
        semi_u,
        semi_v,
        u_ref,
    }
}

/// A tilted plane's normal and an in-plane reference direction.
fn tilted() -> (Vec3<f64>, Vec3<f64>) {
    let axis = Vec3::new(1.0, 2.0, 3.0).normalize();
    (axis, axis.orthonormal_basis().0)
}

/// The carriers every row runs, all in a tilted plane: a circle and a
/// 3:1 ellipse off the origin, a 5000:1 ellipse, and a micron circle
/// far from the origin.
fn carriers() -> [(&'static str, Curve3<f64>); 4] {
    let (axis, u) = tilted();
    [
        ("circle", circle(Point3::new(0.4, -1.1, 2.3), axis, 1.7, u)),
        (
            "ellipse",
            ellipse(Point3::new(-0.6, 0.9, -0.2), axis, 2.4, 0.8, u),
        ),
        (
            "flat ellipse",
            ellipse(Point3::new(-1.0, 0.5, 2.0), axis, 5.0, 1e-3, u),
        ),
        (
            "tiny far circle",
            circle(Point3::new(100.0, -50.0, 25.0), axis, 1e-6, u),
        ),
    ]
}

/// `(P(t) − origin)·n`.
fn reading(c: &Curve3<f64>, t: f64, origin: Point3<f64>, n: Vec3<f64>) -> f64 {
    (c.eval(t) - origin).dot(n)
}

/// The harmonic `p·cos t + q·sin t` the arc reads along `n`, and its
/// crest angle `atan2(q, p)`.
fn crest_angle(c: &Curve3<f64>, n: Vec3<f64>) -> f64 {
    let (axis, u, a, b) = match *c {
        Curve3::Circle {
            axis,
            radius,
            u_ref,
            ..
        } => (axis, u_ref, radius, radius),
        Curve3::Ellipse {
            axis,
            semi_u,
            semi_v,
            u_ref,
            ..
        } => (axis, u_ref, semi_u, semi_v),
        _ => unreachable!(),
    };
    let v = axis.cross(u);
    (b * v.dot(n)).atan2(a * u.dot(n))
}

/// The sample count of [`sampled`].
const N: u32 = 8_000;

/// The densely sampled `(min, max)` of the reading over `[t0, t1]`.
fn sampled(c: &Curve3<f64>, (t0, t1): (f64, f64), origin: Point3<f64>, n: Vec3<f64>) -> (f64, f64) {
    (0..=N)
        .map(|i| reading(c, t0 + (t1 - t0) * f64::from(i) / f64::from(N), origin, n))
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), x| {
            (lo.min(x), hi.max(x))
        })
}

/// **Agreement on the world axes.** Along `x̂`, `ŷ`, `ẑ` the directional
/// door is the coordinate box: the certified twin reads the box's own
/// bounds (the box is that extent hulled with endpoint seeds the
/// extent already contains, to an ulp), and the carrier-scalar door
/// reads them to rounding. Along `−x̂` it reads the same box negated,
/// which no axis-only bug survives.
#[test]
fn the_directional_door_on_the_world_axes_is_the_coordinate_box() {
    let spans = [
        (0.2, 1.3),
        (1.3, 0.2),
        (-2.0, 2.5),
        (0.4, 0.4 + TAU),
        (3.0, 9.0),
    ];
    for (what, c) in carriers() {
        for (t0, t1) in spans {
            let (e0, e1) = (c.eval(t0), c.eval(t1));
            let b = match c {
                Curve3::Circle { .. } => circle_arc_aabb(&c, t0, t1, e0, e1),
                _ => ellipse_arc_aabb(&c, t0, t1, e0, e1),
            }
            .unwrap();
            let o = Point3::origin();
            for (name, n, (lo, hi)) in [
                ("x", Vec3::unit_x(), (b.min_x, b.max_x)),
                ("y", Vec3::unit_y(), (b.min_y, b.max_y)),
                ("z", Vec3::unit_z(), (b.min_z, b.max_z)),
                ("-x", -Vec3::unit_x(), (-b.max_x, -b.min_x)),
            ] {
                let scale = 1.0 + lo.abs().max(hi.abs());
                let ulps = 4.0 * f64::EPSILON * scale;
                let (blo, bhi) = conic_arc_extent_along_bracketed(&c, t0, t1, o, n).unwrap();
                assert!(
                    (blo - lo).abs() <= ulps && (bhi - hi).abs() <= ulps,
                    "{what} [{t0}, {t1}] along {name}: certified door [{blo}, {bhi}] vs box \
                     [{lo}, {hi}]"
                );
                let (glo, ghi) = conic_arc_extent_along(&c, t0, t1, o, n).unwrap();
                assert!(
                    (glo - lo).abs() <= 1e-12 * scale && (ghi - hi).abs() <= 1e-12 * scale,
                    "{what} [{t0}, {t1}] along {name}: f64 door [{glo}, {ghi}] vs box [{lo}, {hi}]"
                );
            }
        }
    }
}

/// **Exact along oblique directions.** Against [`N`] + 1 samples the
/// door must contain the arc's reading and sit no further outside it
/// than the samples' own spacing allows (`A·(Δt/2)²/2` at a crest):
/// a door that read the full turn, dropped a crest, or misplaced the
/// span's middle fails one side or the other. The spans are built per
/// direction from its crest angle `t*`, so each shape is the one
/// named: the crest strictly inside, the crest exactly at an end, a
/// monotone run with no crest, both crests, a full turn, more than a
/// full turn, `0.999` of a turn with the crest in the gap (`h` just
/// below π, so the crest's reading tops the ends' by `A·(1 − cos δ)`,
/// far past the samples' bulge), and a descending run.
#[test]
fn the_directional_door_is_the_sampled_extent_along_oblique_directions() {
    let dirs = [
        Vec3::new(0.3, -0.8, 0.5).normalize(),
        Vec3::new(-0.9, 0.1, 0.4).normalize(),
        Vec3::new(0.2, 0.2, -0.95).normalize(),
        // Not unit: the extent is linear in `n`.
        Vec3::new(2.0, -1.0, 0.5),
    ];
    let origin = Point3::new(0.7, 0.3, -0.4);
    for (what, c) in carriers() {
        for n in dirs {
            let ts = crest_angle(&c, n);
            let spans = [
                ("crest inside", (ts - 0.6, ts + 0.4)),
                ("crest at the end", (ts - 1.1, ts)),
                ("crest at the start", (ts, ts + 0.9)),
                ("monotone", (ts + 0.3, ts + 2.5)),
                ("both crests", (ts - 0.2, ts + PI + 0.3)),
                ("full turn", (ts + 0.77, ts + 0.77 + TAU)),
                ("past a turn", (ts - 1.0, ts + 8.0)),
                (
                    "near-full, crest in the gap",
                    (ts + 0.0005 * TAU, ts + 0.9995 * TAU),
                ),
                ("descending", (ts + 0.4, ts - 0.6)),
            ];
            for (shape, (t0, t1)) in spans {
                let (slo, shi) = sampled(&c, (t0, t1), origin, n);
                let amp = (c.eval(ts) - c.eval(ts + PI)).norm() * n.norm();
                let spacing = (t1 - t0).abs() / f64::from(N);
                let bulge = amp * (spacing / 2.0).powi(2) / 2.0 + 1e-12 * (1.0 + amp);
                for (door, (lo, hi)) in [
                    (
                        "f64",
                        conic_arc_extent_along(&c, t0, t1, origin, n).unwrap(),
                    ),
                    (
                        "certified",
                        conic_arc_extent_along_bracketed(&c, t0, t1, origin, n).unwrap(),
                    ),
                ] {
                    let tag = format!("{what} {shape} along {n:?} ({door})");
                    assert!(
                        lo <= slo + 1e-12 && hi >= shi - 1e-12,
                        "{tag}: [{lo}, {hi}] does not contain the samples [{slo}, {shi}]"
                    );
                    assert!(
                        slo - lo <= bulge && hi - shi <= bulge,
                        "{tag}: [{lo}, {hi}] is looser than the samples [{slo}, {shi}] \
                         allow ({bulge:e})"
                    );
                }
            }
        }
    }
}

/// **Sound at the interval scalar.** A circle whose radius, frame and
/// span are wide brackets describes a family of arcs; the door at
/// [`Interval`] (and its certified twin, which lifts the brackets the
/// same way) must contain every member's sampled reading, at zero
/// slack beyond the sample's own rounding. The spans put the crest of
/// the oblique direction inside, at the end and outside, so the
/// undecided in-span question is exercised where a bracket straddles
/// it.
#[test]
fn the_interval_door_contains_every_realization_of_a_wide_bracket() {
    let iv = <Interval as Real>::from_f64;
    let n = Vec3::new(0.6, -0.3, 0.74);
    let probe = circle(Point3::origin(), Vec3::unit_z(), 1.0, Vec3::unit_x());
    let ts = crest_angle(&probe, n);
    for w in [1e-9, 1e-4, 1e-2] {
        for (t0, t1) in [(ts - 0.5, ts + 0.5), (ts - 0.9, ts), (ts + 0.2, ts + 1.4)] {
            let wide = |x: f64| Interval::from_bounds(x - w, x + w);
            let ci: Curve3<Interval> = Curve3::Circle {
                center: Point3::new(iv(0.3), iv(-0.2), iv(0.1)),
                axis: Vec3::new(iv(0.0), iv(0.0), iv(1.0)),
                radius: wide(1.5),
                u_ref: Vec3::new(iv(1.0), wide(0.0), iv(0.0)),
            };
            let origin = Point3::new(iv(0.0), iv(0.0), iv(0.0));
            let ni = Vec3::new(iv(n.x), iv(n.y), iv(n.z));
            let (glo, ghi) = conic_arc_extent_along(&ci, wide(t0), wide(t1), origin, ni).unwrap();
            let (blo, bhi) =
                conic_arc_extent_along_bracketed(&ci, wide(t0), wide(t1), origin, ni).unwrap();
            for (door, lo, hi) in [("interval", glo.lo(), ghi.hi()), ("certified", blo, bhi)] {
                for (dr, du, d0, d1) in [
                    (-w, -w, -w, -w),
                    (w, w, w, w),
                    (w, -w, -w, w),
                    (-w, w, w, -w),
                    (0.0, 0.0, 0.0, 0.0),
                ] {
                    let member = circle(
                        Point3::new(0.3, -0.2, 0.1),
                        Vec3::unit_z(),
                        1.5 + dr,
                        Vec3::new(1.0, du, 0.0),
                    );
                    let (slo, shi) = sampled(&member, (t0 + d0, t1 + d1), Point3::origin(), n);
                    let slack = 4.0 * f64::EPSILON * 4.0;
                    assert!(
                        lo <= slo + slack && hi >= shi - slack,
                        "{door} w={w:e} [{t0}, {t1}] member ({dr:e}, {du:e}, {d0:e}, {d1:e}): \
                         [{lo}, {hi}] does not contain [{slo}, {shi}]"
                    );
                }
            }
        }
    }
}

/// **A dual's extent is its value channel's** (D9), and poison reads
/// as poison: a NaN radius poisons both ends of both doors rather than
/// narrowing to the end readings.
#[test]
fn the_dual_reads_the_value_channel_and_poison_stays_loud() {
    let (axis, u) = tilted();
    let c = circle(Point3::new(0.4, -1.1, 2.3), axis, 1.7, u);
    let k = Dual64::constant;
    let cd: Curve3<Dual64> = Curve3::Circle {
        center: Point3::new(k(0.4), Dual64::variable(-1.1), k(2.3)),
        axis: Vec3::new(k(axis.x), k(axis.y), k(axis.z)),
        radius: Dual64::variable(1.7),
        u_ref: Vec3::new(k(u.x), k(u.y), k(u.z)),
    };
    let n = Vec3::new(0.3, -0.8, 0.5);
    let nd = Vec3::new(k(n.x), k(n.y), k(n.z));
    let od = Point3::new(k(0.0), k(0.0), k(0.0));
    for (t0, t1) in [(0.1, 1.2), (2.0, 5.5), (0.0, TAU)] {
        let (lo, hi) = conic_arc_extent_along(&c, t0, t1, Point3::origin(), n).unwrap();
        let (dlo, dhi) = conic_arc_extent_along(&cd, k(t0), k(t1), od, nd).unwrap();
        assert!(
            dlo.lo().to_bits() == lo.to_bits() && dhi.hi().to_bits() == hi.to_bits(),
            "[{t0}, {t1}]: dual [{}, {}] vs f64 [{lo}, {hi}]",
            dlo.lo(),
            dhi.hi()
        );
    }
    let poisoned = circle(Point3::origin(), axis, f64::NAN, u);
    let (lo, hi) = conic_arc_extent_along(&poisoned, 0.1, 1.2, Point3::origin(), n).unwrap();
    assert!(
        lo.is_nan() && hi.is_nan(),
        "f64 door narrowed poison: [{lo}, {hi}]"
    );
    let (lo, hi) =
        conic_arc_extent_along_bracketed(&poisoned, 0.1, 1.2, Point3::origin(), n).unwrap();
    assert!(
        lo.is_nan() && hi.is_nan(),
        "certified door narrowed poison: [{lo}, {hi}]"
    );
    let line = Curve3::Line {
        origin: Point3::origin(),
        dir: Vec3::unit_x(),
    };
    assert!(conic_arc_extent_along(&line, 0.0, 1.0, Point3::origin(), n).is_none());
}

/// **An arc in a coordinate plane reads its plane along the normal.**
/// With `û = ŷ` and `v̂ = ẑ`, the arc's `x` harmonic is `p = q = 0`
/// exactly: no crest angle exists, and both doors must read the
/// centre's own `x` with no width, whatever the span.
#[test]
fn an_arc_in_a_coordinate_plane_reads_its_plane_along_the_normal() {
    let c = circle(
        Point3::new(0.75, -1.5, 2.0),
        Vec3::unit_x(),
        1.3,
        Vec3::unit_y(),
    );
    let o = Point3::new(0.25, 0.0, 0.0);
    for (t0, t1) in [(0.2, 1.3), (1.3, 0.2), (0.4, 0.4 + TAU), (3.0, 9.0)] {
        for n in [Vec3::unit_x(), -Vec3::unit_x() * 3.0] {
            let want = (c.eval(t0) - o).dot(n);
            let (lo, hi) = conic_arc_extent_along(&c, t0, t1, o, n).unwrap();
            assert!(
                lo == want && hi == want,
                "[{t0}, {t1}] along {n:?}: the f64 door reads [{lo}, {hi}], not {want}"
            );
            let (blo, bhi) = conic_arc_extent_along_bracketed(&c, t0, t1, o, n).unwrap();
            let ulps = 4.0 * f64::EPSILON * (1.0 + want.abs());
            assert!(
                blo <= want && bhi >= want && bhi - blo <= ulps,
                "[{t0}, {t1}] along {n:?}: the certified door reads [{blo}, {bhi}], not {want}"
            );
        }
    }
}
