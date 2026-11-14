//! **A conic arc's span reach is never short of its farthest point**
//! (`Reach::lever_from`), hunted over random circles and ellipses
//! (eccentricities to 0.999999, a negative or swapped semi-axis), spans
//! from 1e-6 rad to two turns either way, and pivots on the arc, at its
//! centre, along and against its crests, far off its plane and just off
//! it. The farthest point is sampled and each local maximum refined by
//! golden section. The reach is never short of it to rounding, a
//! circle's never past it either and an ellipse's never by more than a
//! quarter's bulge.

#![allow(clippy::panic, clippy::cast_precision_loss)]

test_utils::gated_to!["crates/geom-brep/src/extent.rs"];

use geom::Curve3;
use geom_brep::Reach;
use geom_core::{Point3, Vec3};
use test_utils::fuzz;

fn log(g: &mut fuzz::Rng, a: f64, b: f64) -> f64 {
    (a.ln() + (b.ln() - a.ln()) * g.unit()).exp()
}

fn unit(g: &mut fuzz::Rng) -> Vec3<f64> {
    loop {
        let v = Vec3::new(g.range(-1., 1.), g.range(-1., 1.), g.range(-1., 1.));
        let n = v.norm();
        if n > 0.2 && n < 1.0 {
            return v / n;
        }
    }
}

/// The farthest point of the arc from `p`: 1,000 samples, then each local
/// maximum refined by golden section.
fn farthest(c: &Curve3<f64>, t0: f64, t1: f64, p: Point3<f64>) -> f64 {
    let n: usize = 1000;
    let d = |t: f64| (c.eval(t) - p).norm();
    let ts: Vec<f64> = (0..=n)
        .map(|k| t0 + (t1 - t0) * k as f64 / n as f64)
        .collect();
    let ds: Vec<f64> = ts.iter().map(|t| d(*t)).collect();
    let mut best = ds.iter().copied().fold(0.0, f64::max);
    for k in 0..=n {
        let (lo, hi) = (k.saturating_sub(1), (k + 1).min(n));
        if ds[k] >= ds[lo] && ds[k] >= ds[hi] {
            let (mut a, mut b) = (ts[lo], ts[hi]);
            for _ in 0..80 {
                let m1 = a + (b - a) * (1.0 - geom_core::GOLDEN_SECTION);
                let m2 = a + (b - a) * geom_core::GOLDEN_SECTION;
                if d(m1) < d(m2) {
                    a = m1;
                } else {
                    b = m2;
                }
            }
            best = best.max(d(0.5 * (a + b)));
        }
    }
    best
}

#[test]
fn a_conic_arcs_span_reach_is_never_short_of_its_farthest_point() {
    let mut g = fuzz::start("span_reach_hunt");
    for _ in 0..fuzz::scaled(100) {
        let r = [1e-3, 1.0, 1e3][g.below(3)];
        let center = Point3::origin() + unit(&mut g) * [0.0, 1.0, 1e3][g.below(3)];
        let axis = unit(&mut g);
        let u_ref = {
            let v = unit(&mut g);
            (v - axis * v.dot(axis)).normalize()
        };
        let ecc: f64 = [0.0, 0.5, 0.9, 0.99, 0.999, 0.999_999][g.below(6)];
        let (mut a, mut b) = (r, r * (1.0 - ecc * ecc).sqrt());
        if g.unit() < 0.2 {
            core::mem::swap(&mut a, &mut b);
        }
        let carrier = if ecc == 0.0 && g.unit() < 0.5 {
            Curve3::circle(center, axis, r, u_ref)
        } else {
            Curve3::Ellipse(geom::Ellipse::new(
                geom::Frame::new(center, axis, u_ref),
                a,
                b,
            ))
        };
        let big = a.max(b);
        let span = if g.unit() < 0.1 {
            g.range(6.0, 13.0)
        } else {
            log(&mut g, 1e-6, 6.3)
        };
        let t0 = g.range(-10.0, 10.0);
        let t1 = if g.unit() < 0.3 { t0 - span } else { t0 + span };
        let tm = t0 + (t1 - t0) * g.unit();
        let pivots = [
            carrier.eval(t0),
            carrier.eval(t1),
            carrier.eval(tm),
            center,
            center + (carrier.eval(tm) - center) * g.range(-50.0, 50.0),
            center - (carrier.eval(tm) - center) * log(&mut g, 1e-3, 1e6),
            center + unit(&mut g) * big * log(&mut g, 1e-3, 1e3),
            carrier.eval(tm) + axis * big * log(&mut g, 1e-6, 1e3),
            carrier.eval(tm) + u_ref * big * g.range(-1e-4, 1e-4),
        ];
        for p in pivots {
            let reach = Reach::Span {
                carrier: carrier.clone(),
                t0,
                t1,
            };
            let got = reach.lever_from(p).length();
            let far = farthest(&carrier, t0, t1, p);
            let scale = (center - Point3::origin()).norm() + (p - Point3::origin()).norm() + big;
            let tol = 16.0 * f64::EPSILON * scale;
            let slack = match carrier {
                Curve3::Circle(..) => 0.0,
                _ => (1.0 - ((t1 - t0).abs().min(core::f64::consts::TAU) / 8.0).cos()) * big,
            };
            let label = format!("{carrier:?} over [{t0}, {t1}] from {p:?}");
            assert!(
                got >= far - tol,
                "{label}: {got} falls short of the farthest point {far}; {}",
                fuzz::replay()
            );
            assert!(
                got <= far + slack + tol,
                "{label}: {got} passes the farthest point {far} by more than {slack}; {}",
                fuzz::replay()
            );
        }
    }
}

/// **Arcs whose span cover rounds past the whole turn**, read from the
/// conic's centre: the window's piece alone reads its far distance an
/// ulp above the whole conic's, and the arc's reach reads the lesser.
/// The hunt found each (seeds `0xe429b8c6a93f17ee` and
/// `0x52cefed7fc20b9ed`).
#[test]
fn an_arc_read_from_its_centre_is_never_past_the_whole_turn() {
    let ellipse = |origin: [f64; 3], axis: [f64; 3], u_ref: [f64; 3], a: f64, b: f64| {
        let [x, y, z] = origin;
        let v = |[x, y, z]: [f64; 3]| Vec3::new(x, y, z);
        (
            Curve3::Ellipse(geom::Ellipse::new(
                geom::Frame::new(Point3::new(x, y, z), v(axis), v(u_ref)),
                a,
                b,
            )),
            Point3::new(x, y, z),
        )
    };
    let rows = [
        (
            ellipse(
                [
                    0.830_304_973_648_282_8,
                    -0.416_467_848_902_350_56,
                    0.370_335_228_631_537_84,
                ],
                [
                    -0.828_978_939_411_948_3,
                    0.480_638_571_787_519,
                    -0.285_972_868_156_221,
                ],
                [
                    0.307_146_392_996_351_8,
                    0.818_559_802_054_640_1,
                    0.485_408_017_784_625_2,
                ],
                866.025_403_784_438_6,
                1000.0,
            ),
            (-4.616_193_293_956_75, -3.767_850_344_482_187_4),
        ),
        (
            ellipse(
                [0.0, 0.0, 0.0],
                [
                    0.414_840_037_039_107_05,
                    -0.735_167_300_097_304_7,
                    -0.536_131_312_774_241_9,
                ],
                [
                    0.119_112_848_247_196_9,
                    -0.540_275_439_858_399,
                    0.833_015_353_080_754_3,
                ],
                1000.0,
                141.067_359_796_658_93,
            ),
            (-4.180_645_483_569_217, 8.234_684_407_467_041),
        ),
    ];
    for ((carrier, centre), (t0, t1)) in rows {
        let reach = |t0, t1| {
            Reach::Span {
                carrier: carrier.clone(),
                t0,
                t1,
            }
            .lever_from(centre)
            .length()
        };
        let (got, whole) = (reach(t0, t1), reach(0.0, core::f64::consts::TAU));
        assert!(
            got <= whole,
            "the arc over [{t0}, {t1}]: its span reach {got} passes the whole turn {whole}"
        );
    }
}
