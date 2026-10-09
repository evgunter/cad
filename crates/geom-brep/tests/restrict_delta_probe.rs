//! Delta-review probes for PR 4441's fix pass (a1848c4..5d393d8):
//! measurement rows, not gates. Each prints `PROBE` lines.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::TAU;

use crate::shared::interval::iv;
use geom_brep::{MappedCurve, SweepRange};
use geom_core::{Affine3, Bounds, Interval, Mat3, Point2, Point3, Vec3};

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

/// main's `restrict`: compose the s0 motion into `place`, scale the span.
fn old_restrict(c: &MappedCurve<Interval>, s0: Interval, s1: Interval) -> MappedCurve<Interval> {
    let MappedCurve::RevolvedPoint {
        point,
        place,
        axis_origin,
        axis_dir,
        angles,
    } = *c
    else {
        unreachable!()
    };
    assert_eq!((angles.start().lo(), angles.start().hi()), (0.0, 0.0));
    let angle = angles.span();
    MappedCurve::RevolvedPoint {
        point,
        place: Affine3::rotation_about_axis(axis_origin, axis_dir, s0 * angle) * place,
        axis_origin,
        axis_dir,
        angles: SweepRange::from_zero((s1 - s0) * angle),
    }
}

/// The alternative form: the head's start + span, but in the
/// normalized parameter of the ORIGINAL curve, the sweep angle applied
/// once at evaluation: `orig.eval(u0 + du·s)`.
#[derive(Clone, Copy)]
struct SUnits {
    u0: Interval,
    du: Interval,
}

impl SUnits {
    fn restrict(self, s0: Interval, s1: Interval) -> Self {
        SUnits {
            u0: self.u0 + self.du * s0,
            du: self.du * (s1 - s0),
        }
    }
}

/// The endpoint form in angle units: `from·(1−s) + to·s` (a1848c4).
#[derive(Clone, Copy)]
struct Endpoint {
    from: Interval,
    to: Interval,
}

impl Endpoint {
    fn at(self, s: Interval) -> Interval {
        self.from * (iv(1.0) - s) + self.to * s
    }
    fn restrict(self, s0: Interval, s1: Interval) -> Self {
        Endpoint {
            from: self.at(s0),
            to: self.at(s1),
        }
    }
}

const SAMPLES: [f64; 3] = [0.0, 0.5, 1.0];

fn sampled(c: &MappedCurve<Interval>) -> f64 {
    SAMPLES
        .into_iter()
        .map(|s| pw(c.eval(iv(s))))
        .fold(0.0, f64::max)
}

fn sampled_by(at: [f64; 3], angle: impl Fn(Interval) -> Interval) -> f64 {
    let MappedCurve::RevolvedPoint {
        point,
        place,
        axis_origin,
        axis_dir,
        ..
    } = rim_at(at)
    else {
        unreachable!()
    };
    let p = place.transform_point(Point3::new(point.x, point.y, iv(0.0)));
    SAMPLES
        .into_iter()
        .map(|s| {
            pw(Affine3::rotation_about_axis(axis_origin, axis_dir, angle(iv(s))).transform_point(p))
        })
        .fold(0.0, f64::max)
}

fn widen(x: f64, d: f64) -> Interval {
    Interval::from_bounds(x - d, x + d)
}

fn step(pattern: &str, k: usize) -> (Interval, Interval) {
    #[allow(clippy::cast_precision_loss)]
    let a = 0.37 + 0.011 * ((k * 7) % 5) as f64;
    let alt = |x: Interval| if k % 2 == 0 { (x, iv(1.0)) } else { (iv(0.0), x) };
    match pattern {
        "(0,1/2)" => (iv(0.0), iv(0.5)),
        "(1/2,1)" => (iv(0.5), iv(1.0)),
        "(1/4,3/4)" => (iv(0.25), iv(0.75)),
        "(0.3,0.7)" => (iv(0.3), iv(0.7)),
        "keep-far (a,1)" => (iv(a), iv(1.0)),
        "keep-near (0,a)" => (iv(0.0), iv(a)),
        "alternate" => alt(iv(a)),
        "alternate a±1e-13" => alt(widen(a, 1e-13)),
        "keep-far a±1e-13" => (widen(a, 1e-13), iv(1.0)),
        "keep-near a±1e-13" => (iv(0.0), widen(a, 1e-13)),
        "alternate a=t/span" => alt(iv(a * 3.0) / iv(3.0)),
        "(0.3,0.7) quotients" => (iv(0.9) / iv(3.0), iv(2.1) / iv(3.0)),
        _ => unreachable!(),
    }
}

#[test]
fn probe_width_table_delta() {
    let patterns = [
        "(0,1/2)",
        "(1/2,1)",
        "(1/4,3/4)",
        "(0.3,0.7)",
        "keep-far (a,1)",
        "keep-near (0,a)",
        "alternate",
        "alternate a±1e-13",
        "keep-far a±1e-13",
        "keep-near a±1e-13",
        "alternate a=t/span",
        "(0.3,0.7) quotients",
    ];
    let checkpoints = [1usize, 4, 16, 52, 64];
    for (name, at) in [("near", [0.0, 0.0, 3.0]), ("far", [1000.0, -700.0, 300.0])] {
        for pat in patterns {
            let mut old = rim_at(at);
            let mut new = rim_at(at);
            let mut su = SUnits {
                u0: iv(0.0),
                du: iv(1.0),
            };
            let mut ep = Endpoint {
                from: iv(0.0),
                to: iv(TAU),
            };
            let mut worst_ratio: f64 = 0.0;
            let mut row = format!("PROBE {name:4} {pat:22}");
            for n in 1..=64usize {
                let (s0, s1) = step(pat, n - 1);
                old = old_restrict(&old, s0, s1);
                new = new.restrict(s0, s1);
                su = su.restrict(s0, s1);
                ep = ep.restrict(s0, s1);
                let (wm, wh) = (sampled(&old), sampled(&new));
                worst_ratio = worst_ratio.max(wh / wm);
                if checkpoints.contains(&n) {
                    let ws = sampled_by(at, |s| iv(TAU) * (su.u0 + su.du * s));
                    let we = sampled_by(at, |s| ep.at(s));
                    row += &format!(
                        " | N={n}: main {wm:.2e} head {wh:.2e} sunits {ws:.2e} endpt {we:.2e}"
                    );
                }
            }
            row += &format!(" | worst head/main {worst_ratio:.2}");
            println!("{row}");
        }
    }
}

/// `offset_axial::reauthor`'s revolved arm, both spellings, outside the
/// crate: head reads the moved corner as `place⁻¹(p − (I − R(−θ))(p −
/// q))` and stores `θ` as the range's start; main composed `R(θ)` into
/// the placement and read `(R(θ)·place)⁻¹ p`. Measured: the stored
/// point's width at Interval, and at f64 how far `eval(0)` of the
/// re-authored description lands from the corner it was read from.
#[test]
fn probe_reauthor_turn_back() {
    // A tilted, far placement and a tilted axis.
    let mk_place = |at: [f64; 3]| {
        Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.2, 1.0, -0.4), 0.7)
            * Affine3::translation(Vec3::new(at[0], at[1], at[2]))
    };
    let n = Vec3::new(0.3, -0.2, 1.0);
    for at in [[0.0, 0.0, 3.0], [1000.0, -700.0, 300.0], [1.0e5, 3.0e4, -2.0e4]] {
        let place = mk_place(at);
        // the axis passes near the placed sketch origin
        let q = place.transform_point(Point3::new(0.5, 1.0, 0.0));
        let pt = Point2::new(2.0, 2.0);
        let mut worst = (0.0f64, 0.0f64);
        let mut differs = 0;
        for k in 0..200 {
            let theta = -3.0 + 6.0 * f64::from(k) / 199.0;
            // the moved corner: the placed point turned by theta
            let p = Affine3::rotation_about_axis(q, n, theta)
                .transform_point(place.transform_point(Point3::new(pt.x, pt.y, 0.0)));
            // head
            let inv = place.inverse();
            let turn = Mat3::identity_minus_rotation_about(n, -theta);
            let qh = inv.transform_point(p - turn * (p - q));
            let eh = Affine3::rotation_about_axis(q, n, theta)
                .transform_point(place.transform_point(Point3::new(qh.x, qh.y, 0.0)));
            // main
            let pm = Affine3::rotation_about_axis(q, n, theta) * place;
            let qm = pm.inverse().transform_point(p);
            let em = Affine3::rotation_about_axis(q, n, 0.0)
                .transform_point(pm.transform_point(Point3::new(qm.x, qm.y, 0.0)));
            let dh = (eh - p).norm_inf();
            let dm = (em - p).norm_inf();
            worst = (worst.0.max(dh), worst.1.max(dm));
            if qh.x.to_bits() != qm.x.to_bits() || qh.y.to_bits() != qm.y.to_bits() {
                differs += 1;
            }
        }
        println!(
            "PROBE reauthor f64 at {at:?}: worst |eval(0) - corner| head {:.3e} main {:.3e}; \
             stored point bits differ at {differs}/200 angles",
            worst.0, worst.1
        );
        // theta = 0 exactly, f64: is the head's stored point the main one?
        let p0 = place.transform_point(Point3::new(pt.x, pt.y, 0.0));
        let qh0 = place.inverse().transform_point(
            p0 - Mat3::identity_minus_rotation_about(n, 0.0) * (p0 - q),
        );
        let qm0 = place.inverse().transform_point(p0);
        println!(
            "PROBE reauthor f64 theta=0 at {at:?}: head {:?} main {:?} bit-equal {}",
            qh0,
            qm0,
            qh0.x.to_bits() == qm0.x.to_bits()
                && qh0.y.to_bits() == qm0.y.to_bits()
                && qh0.z.to_bits() == qm0.z.to_bits()
        );
    }
}

/// The Interval side: at the exact zero start, does `I − R(−0)` reach
/// the stored point? And at a moved start, head vs main widths.
#[test]
fn probe_reauthor_interval_widths() {
    let lift = |p: Point3<f64>| Point3::new(iv(p.x), iv(p.y), iv(p.z));
    for at in [[0.0, 0.0, 3.0], [1000.0, -700.0, 300.0]] {
        let place = Affine3::translation(Vec3::new(iv(at[0]), iv(at[1]), iv(at[2])));
        let q = Point3::new(iv(1.0 + at[0]), iv(2.0 + at[1]), iv(at[2]));
        let n = Vec3::new(iv(0.0), iv(0.0), iv(1.0));
        let p = lift(Point3::new(2.0 + at[0], 2.0 + at[1], at[2]));
        let m0 = Mat3::identity_minus_rotation_about(n, iv(0.0));
        let dust = m0 * (p - q);
        let inv = place.inverse();
        let head = inv.transform_point(p - dust);
        let plain = inv.transform_point(p);
        println!(
            "PROBE theta=0 at {at:?}: (I-R(0))(p-q) = [{:e},{:e}] x; p - it width {:.3e}; \
             stored head {:.3e} plain place^-1 {:.3e}",
            dust.x.lo(),
            dust.x.hi(),
            pw(p - dust),
            pw(head),
            pw(plain)
        );
        for theta in [0.3f64, 2.0] {
            let pt = Affine3::rotation_about_axis(q, n, iv(theta)).transform_point(p);
            let h = inv.transform_point(
                pt - Mat3::identity_minus_rotation_about(n, -iv(theta)) * (pt - q),
            );
            let m = (Affine3::rotation_about_axis(q, n, iv(theta)) * place)
                .inverse()
                .transform_point(pt);
            println!(
                "PROBE theta={theta} at {at:?}: stored width head {:.3e} main {:.3e}",
                pw(h),
                pw(m)
            );
        }
    }
}
