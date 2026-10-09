//! Review probes for PR 4441 (`nurbs/restrict-in-the-parameter`):
//! measurement rows, not gates. Each prints its table and asserts only
//! what the review states as found.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::TAU;

use crate::shared::interval::iv;
use geom_brep::{MappedCurve, SweepRange};
use geom_core::{Affine3, Bounds, Interval, Point2, Point3, Vec3};

fn width(e: Interval) -> f64 {
    e.hi() - e.lo()
}

fn pw(p: Point3<Interval>) -> f64 {
    width(p.x).max(width(p.y)).max(width(p.z))
}

fn rim_at(at: [f64; 3]) -> MappedCurve<Interval> {
    let [x, y, z] = at;
    MappedCurve::RevolvedPoint {
        point: Point2::new(iv(2.0), iv(2.0)),
        place: Affine3::translation(Vec3::new(iv(x), iv(y), iv(z))),
        axis_origin: Point3::new(iv(1.0 + x), iv(2.0 + y), iv(z)),
        axis_dir: Vec3::new(iv(0.0), iv(0.0), iv(1.0)),
        angles: SweepRange::from_zero(iv(TAU)),
    }
}

/// main's `restrict`, emulated on a `from_zero` range: compose the s0
/// motion into `place`, scale the span. `eval` on a `from_zero` range
/// is main's `eval` bit for bit (claim 1, checked below).
fn old_restrict(c: &MappedCurve<Interval>, s0: Interval, s1: Interval) -> MappedCurve<Interval> {
    match *c {
        MappedCurve::RevolvedPoint {
            point,
            place,
            axis_origin,
            axis_dir,
            angles,
        } => {
            assert_eq!(angles.from.lo(), 0.0);
            assert_eq!(angles.from.hi(), 0.0);
            let angle = angles.to;
            MappedCurve::RevolvedPoint {
                point,
                place: Affine3::rotation_about_axis(axis_origin, axis_dir, s0 * angle) * place,
                axis_origin,
                axis_dir,
                angles: SweepRange::from_zero((s1 - s0) * angle),
            }
        }
        _ => unreachable!(),
    }
}

fn sampled(c: &MappedCurve<Interval>) -> f64 {
    [0.0, 0.5, 1.0]
        .into_iter()
        .map(|s| pw(c.eval(iv(s))))
        .fold(0.0, f64::max)
}

/// A split step: the parameters handed to `restrict`.
type Step = (Interval, Interval);

fn widen(x: f64, d: f64) -> Interval {
    Interval::from_bounds(x - d, x + d)
}

/// The step sequence for a named pattern at step k.
fn step(pattern: &str, k: usize) -> Step {
    // a non-dyadic "intersection parameter", varying per split
    let a = 0.37 + 0.011 * ((k * 7) % 5) as f64;
    match pattern {
        "(0,1/2)" => (iv(0.0), iv(0.5)),
        "(1/2,1)" => (iv(0.5), iv(1.0)),
        "(0.3,0.7)" => (iv(0.3), iv(0.7)),
        "keep-far (a,1)" => (iv(a), iv(1.0)),
        "keep-near (0,a)" => (iv(0.0), iv(a)),
        "alternate (a,1)/(0,a)" => {
            if k % 2 == 0 {
                (iv(a), iv(1.0))
            } else {
                (iv(0.0), iv(a))
            }
        }
        // the split parameter as an enclosure, as split_specs builds it
        // from an intersection parameter t: a = (t - t0)/span
        "alternate, a ± 1e-13" => {
            if k % 2 == 0 {
                (widen(a, 1e-13), iv(1.0))
            } else {
                (iv(0.0), widen(a, 1e-13))
            }
        }
        "keep-far, a ± 1e-13" => (widen(a, 1e-13), iv(1.0)),
        // a = (t - t0)/span for a point t: an ulp-wide quotient
        "alternate, a = t/span" => {
            let q = iv(a * 3.0) / iv(3.0);
            if k % 2 == 0 { (q, iv(1.0)) } else { (iv(0.0), q) }
        }
        "(0.3,0.7) as quotients" => (iv(0.9) / iv(3.0), iv(2.1) / iv(3.0)),
        _ => unreachable!(),
    }
}

/// The composed parameter range of the ORIGINAL curve after `n` steps
/// (f64 midpoints; only used to place an ulp-wide enclosure for the
/// "exact range, one rounding at eval" estimate).
fn composed(pattern: &str, n: usize) -> (f64, f64) {
    let (mut lo, mut hi) = (0.0f64, 1.0f64);
    for k in 0..n {
        let (s0, s1) = step(pattern, k);
        let m = |x: Interval| 0.5 * (x.lo() + x.hi());
        let (a, b) = (m(s0), m(s1));
        let span = hi - lo;
        let (nlo, nhi) = (lo + span * a, lo + span * b);
        lo = nlo;
        hi = nhi;
    }
    (lo, hi)
}

#[test]
fn probe_width_table() {
    let patterns = [
        "(0,1/2)",
        "(1/2,1)",
        "(0.3,0.7)",
        "keep-far (a,1)",
        "keep-near (0,a)",
        "alternate (a,1)/(0,a)",
        "alternate, a ± 1e-13",
        "keep-far, a ± 1e-13",
        "alternate, a = t/span",
        "(0.3,0.7) as quotients",
    ];
    let checkpoints = [1usize, 4, 16, 64];
    for (name, at) in [("near", [0.0, 0.0, 3.0]), ("far", [1000.0, -700.0, 300.0])] {
        for pat in patterns {
            let mut old = rim_at(at);
            let mut new = rim_at(at);
            let mut row = format!("PROBE {name:4} {pat:24}");
            for n in 1..=64usize {
                let (s0, s1) = step(pat, n - 1);
                old = old_restrict(&old, s0, s1);
                new = new.restrict(s0, s1);
                if checkpoints.contains(&n) {
                    // exact-range estimate: the original curve evaluated
                    // at an ulp enclosure of the composed parameter, plus
                    // (for enclosure patterns) nothing for the parameter
                    // width — so it is a floor, not an emulation
                    let (lo, hi) = composed(pat, n);
                    let orig = rim_at(at);
                    let exact = [0.0, 0.5, 1.0]
                        .into_iter()
                        .map(|s| {
                            let x = lo + (hi - lo) * s;
                            pw(orig.eval(Interval::from_bounds(x.next_down(), x.next_up())))
                        })
                        .fold(0.0, f64::max);
                    row += &format!(
                        " | N={n}: main {:.2e} head {:.2e} exact~ {:.2e}",
                        sampled(&old),
                        sampled(&new),
                        exact
                    );
                }
            }
            println!("{row}");
        }
    }
}

/// Claim 1 at f64 and Interval: a whole range's eval against main's
/// spelling `rotation_about_axis(q, n, s·angle)`, bit for bit.
#[test]
fn probe_unrestricted_bits() {
    let mut mismatches = Vec::new();
    for angle in [TAU, -TAU, 1.0, -1.0, 1e-8, -0.3, core::f64::consts::PI] {
        for at in [[0.0, 0.0, 0.0], [1000.0, -700.0, 300.0], [-3.0, 0.0, 0.0]] {
            let place = Affine3::translation(Vec3::new(at[0], at[1], at[2]));
            let (q, n) = (Point3::new(at[0] + 1.0, at[1] + 2.0, at[2]), Vec3::new(0.3, -0.2, 1.0));
            let pt = Point2::new(2.0, 2.0);
            let c = MappedCurve::RevolvedPoint {
                point: pt,
                place,
                axis_origin: q,
                axis_dir: n,
                angles: SweepRange::from_zero(angle),
            };
            for i in 0..=64 {
                let s = f64::from(i) / 64.0;
                let s = if i == 7 { 1.0 / 3.0 } else { s };
                let new = c.eval(s);
                let p = place.transform_point(Point3::new(pt.x, pt.y, 0.0));
                let old = Affine3::rotation_about_axis(q, n, s * angle).transform_point(p);
                for (a, b) in [(new.x, old.x), (new.y, old.y), (new.z, old.z)] {
                    if a.to_bits() != b.to_bits() {
                        mismatches.push(format!("f64 angle {angle} at {at:?} s {s}: {a:e} vs {b:e}"));
                    }
                }
                // Interval
                let ci = MappedCurve::RevolvedPoint {
                    point: Point2::new(iv(2.0), iv(2.0)),
                    place: Affine3::translation(Vec3::new(iv(at[0]), iv(at[1]), iv(at[2]))),
                    axis_origin: Point3::new(iv(q.x), iv(q.y), iv(q.z)),
                    axis_dir: Vec3::new(iv(n.x), iv(n.y), iv(n.z)),
                    angles: SweepRange::from_zero(iv(angle)),
                };
                let si = if i == 7 { iv(1.0) / iv(3.0) } else { iv(s) };
                let newi = ci.eval(si);
                let pi = Affine3::translation(Vec3::new(iv(at[0]), iv(at[1]), iv(at[2])))
                    .transform_point(Point3::new(iv(2.0), iv(2.0), iv(0.0)));
                let oldi = Affine3::rotation_about_axis(
                    Point3::new(iv(q.x), iv(q.y), iv(q.z)),
                    Vec3::new(iv(n.x), iv(n.y), iv(n.z)),
                    si * iv(angle),
                )
                .transform_point(pi);
                for (a, b) in [(newi.x, oldi.x), (newi.y, oldi.y), (newi.z, oldi.z)] {
                    if a.lo().to_bits() != b.lo().to_bits() || a.hi().to_bits() != b.hi().to_bits() {
                        mismatches.push(format!(
                            "Interval angle {angle} at {at:?} s {s}: [{:e},{:e}] vs [{:e},{:e}]",
                            a.lo(),
                            a.hi(),
                            b.lo(),
                            b.hi()
                        ));
                    }
                }
            }
            // strut
            let v = Vec3::new(0.5, -1.5, angle);
            let st = MappedCurve::ExtrudedPoint {
                point: pt,
                place,
                vec: v,
                stations: SweepRange::unit(),
            };
            for i in 0..=16 {
                let s = f64::from(i) / 16.0;
                let new = st.eval(s);
                let old = place.transform_point(Point3::new(pt.x, pt.y, 0.0)) + v * s;
                for (a, b) in [(new.x, old.x), (new.y, old.y), (new.z, old.z)] {
                    if a.to_bits() != b.to_bits() {
                        mismatches.push(format!("strut f64 {angle} at {at:?} s {s}: {a:e} vs {b:e}"));
                    }
                }
            }
        }
    }
    for m in mismatches.iter().take(40) {
        println!("PROBE MISMATCH {m}");
    }
    println!("PROBE mismatches total {}", mismatches.len());
}

/// offset_axial::reauthor's stored point at Interval: main read
/// `place.inverse()` when the start did not turn; the head reads
/// `(rotation_about_axis(q, n, angles.from) * place).inverse()`, and
/// `angles.from` is the exact `[0, 0]` for a whole range.
#[test]
fn probe_reauthor_stored_point_width() {
    for at in [[0.0, 0.0, 3.0], [1000.0, -700.0, 300.0]] {
        let place = Affine3::translation(Vec3::new(iv(at[0]), iv(at[1]), iv(at[2])));
        let q = Point3::new(iv(1.0 + at[0]), iv(2.0 + at[1]), iv(at[2]));
        let n = Vec3::new(iv(0.0), iv(0.0), iv(1.0));
        let p_start = Point3::new(iv(2.0 + at[0]), iv(2.0 + at[1]), iv(at[2]));
        let old = place.inverse().transform_point(p_start);
        let new = (Affine3::rotation_about_axis(q, n, iv(0.0)) * place)
            .inverse()
            .transform_point(p_start);
        // and the re-authored description's start sample
        let mk = |pt: Point3<Interval>| MappedCurve::RevolvedPoint {
            point: Point2::new(pt.x, pt.y),
            place,
            axis_origin: q,
            axis_dir: n,
            angles: SweepRange::from_zero(iv(TAU)),
        };
        println!(
            "PROBE reauthor at {at:?}: stored q width main {:.3e} head {:.3e}; eval(0) main {:.3e} head {:.3e}",
            pw(old),
            pw(new),
            pw(mk(old).eval(iv(0.0))),
            pw(mk(new).eval(iv(0.0)))
        );
    }
}
