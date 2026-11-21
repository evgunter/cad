//! REVIEW PROBE (reviewer r2, PR 7203). Not part of the suite: its own
//! `[[test]]` target, written against the API both the frozen head and
//! its base share, so one file runs on each tree. It prints rows that
//! `analysis/oneround/anchored-conic/r2/oracle.py` reads.
#![allow(clippy::all, clippy::pedantic, missing_docs)]

use geom::cover::ParamWindow;
use geom::{Curve3, Surface};
use geom_brep::{PlaneConeSection, Reach, plane_cone_section};
use geom_core::{Band, Bounds, Interval, Point3, Real, Vec3};

fn lp(p: Point3<f64>) -> Point3<Interval> {
    Point3::new(Interval::from_f64(p.x), Interval::from_f64(p.y), Interval::from_f64(p.z))
}
fn lv(p: Vec3<f64>) -> Vec3<Interval> {
    Vec3::new(Interval::from_f64(p.x), Interval::from_f64(p.y), Interval::from_f64(p.z))
}

fn band() -> Band {
    Band::new(1e-9, 1e-8).unwrap()
}

fn plane(origin: Point3<f64>, normal: Vec3<f64>) -> Surface<f64> {
    let n = normal.normalize();
    let u = if n.x.abs() < 0.9 {
        Vec3::unit_x()
    } else {
        Vec3::unit_y()
    };
    Surface::Plane {
        frame: geom::Frame::new(origin, n, (u - n * u.dot(n)).normalize()),
    }
}

fn h(x: f64) -> String {
    format!("{:016x}", x.to_bits())
}
fn hv(v: Vec3<f64>) -> String {
    format!("{} {} {}", h(v.x), h(v.y), h(v.z))
}
fn hp(p: Point3<f64>) -> String {
    hv(p - Point3::origin())
}

struct Section {
    cut: Surface<f64>,
    cone: Surface<f64>,
    curve: Curve3<f64>,
    curve_i: Option<Curve3<Interval>>,
}

/// Far-centred cone sections: two cones, planes a part in 1e3..1e8 short
/// of the parabola.
fn sections(seed: u64) -> Vec<Section> {
    let mut rng = seed;
    let mut next = || {
        rng ^= rng << 13;
        rng ^= rng >> 7;
        rng ^= rng << 17;
        (rng >> 11) as f64 / (1u64 << 53) as f64
    };
    let mut out = Vec::new();
    for (half, axis, apex) in [
        (
            0.4_f64,
            Vec3::new(0.1, -0.2, -1.0).normalize(),
            Point3::new(0.2, 0.1, 2.0),
        ),
        (
            0.9_f64,
            Vec3::new(1.0, 0.3, 0.2).normalize(),
            Point3::new(-3.0, 4.0, 1.5),
        ),
        (
            0.15_f64,
            Vec3::new(0.0, 0.0, 1.0),
            Point3::new(120.0, -75.0, 30.0),
        ),
    ] {
        let cone = Surface::Cone {
            frame: geom::Frame::new(apex, axis, Vec3::new(1.0, 0.0, 0.1)),
            half_angle: half,
        };
        let parabolic = core::f64::consts::FRAC_PI_2 - half;
        for off in [1e-2, 1e-3, 1e-4, 1e-5, 1e-6, 1e-7, 1e-8] {
            for _ in 0..6 {
                let side = Vec3::new(next() - 0.5, next() - 0.5, next() - 0.5);
                let side = (side - axis * side.dot(axis)).normalize();
                let tilt = parabolic * (1.0 - off * (0.5 + next()));
                let n = axis * tilt.cos() + side * tilt.sin();
                let at = apex + axis * (1.0 + next());
                let cut = plane(at, n);
                let reach = Reach::of_frame(at, (1.0, 1.0));
                if let Ok(PlaneConeSection::TiltedEllipse(c)) =
                    plane_cone_section(&cut, &cone, &reach, band())
                {
                    let lift = Interval::from_f64;
                    let reach_i = Reach::of_frame(lp(at), (lift(1.0), lift(1.0)));
                    let curve_i = match plane_cone_section(
                        &cut.map_scalar(lift),
                        &cone.map_scalar(lift),
                        &reach_i,
                        band(),
                    ) {
                        Ok(PlaneConeSection::TiltedEllipse(c)) => Some(c),
                        _ => None,
                    };
                    out.push(Section {
                        cut: cut.clone(),
                        cone: cone.clone(),
                        curve: c,
                        curve_i,
                    });
                }
            }
        }
    }
    out
}

fn surf_row(s: &Section) -> String {
    let (Surface::Plane { frame: pf, .. }, Surface::Cone { frame: cf, half_angle, .. }) =
        (&s.cut, &s.cone)
    else {
        panic!()
    };
    let f = s.curve.frame().unwrap();
    let (a, b) = s.curve.conic().unwrap().semi_axes();
    format!(
        "S {} {} {} {} {} {} {} {} {} {}",
        hp(pf.origin()),
        hv(pf.axis()),
        hp(cf.origin()),
        hv(cf.axis()),
        h(*half_angle),
        hp(f.origin()),
        hv(f.axis()),
        hv(f.u_ref()),
        h(a),
        h(b)
    )
}

fn params(a: f64) -> Vec<f64> {
    let mut ts = vec![0.0];
    for d in [1e-3, 1e-2, 0.1, 1.0, 10.0, 80.0, 400.0] {
        for s in [-1.0, 1.0] {
            ts.push(s * d / a);
            ts.push(s * d * 0.7310585786 / a);
        }
    }
    for base in [
        core::f64::consts::FRAC_PI_2,
        -core::f64::consts::FRAC_PI_2,
        core::f64::consts::PI,
        0.3,
        -1.1,
        2.5,
    ] {
        for d in [-1.0, 0.0, 3.0] {
            ts.push(base + d / a);
        }
    }
    ts
}

/// C1: the evaluator's points, its running bound and its Interval
/// enclosures, for the Python oracle.
#[test]
fn dump_points() {
    for s in sections(0x9E37_79B9_7F4A_7C15) {
        let conic = s.curve.conic().unwrap();
        let (a, _) = conic.semi_axes();
        if (conic.center() - Point3::origin()).norm() < 1e2 {
            continue;
        }
        println!("{}", surf_row(&s));
        let lifted = s.curve.map_scalar(Interval::from_f64);
        for t in params(a) {
            let p = s.curve.eval(t);
            let r = conic.running_point(t);
            let e = lifted.eval(Interval::from_f64(t));
            println!(
                "P {} {} {} {} {} {} {} {} {} {} {} {} {}",
                h(t),
                hp(p),
                h(r[0].error),
                h(r[1].error),
                h(r[2].error),
                h(e.x.lo()),
                h(e.y.lo()),
                h(e.z.lo()),
                h(e.x.hi()),
                h(e.y.hi()),
                h(e.z.hi()),
                h(r[0].value),
                h(conic.param_in(p, (t - 0.3, t + 0.3))),
            );
            if let Some(ci) = &s.curve_i {
                let e = ci.eval(Interval::from_f64(t));
                println!(
                    "I {} {} {} {} {} {} {}",
                    h(t),
                    h(e.x.lo()),
                    h(e.y.lo()),
                    h(e.z.lo()),
                    h(e.x.hi()),
                    h(e.y.hi()),
                    h(e.z.hi()),
                );
            }
        }
    }
}

fn dirs(f: &geom::Frame<f64>) -> Vec<Vec3<f64>> {
    let mut d = vec![f.u_ref(), f.v_ref(), f.axis()];
    for (x, y, z) in [
        (1.0, 0.0, 0.0),
        (0.0, 1.0, 0.0),
        (0.0, 0.0, 1.0),
        (0.6, 0.8, 0.0),
        (0.0, -0.6, 0.8),
        (2.0 / 3.0, 2.0 / 3.0, 1.0 / 3.0),
        (-1.0 / 3.0, 2.0 / 3.0, -2.0 / 3.0),
    ] {
        d.push(Vec3::new(x, y, z));
    }
    let n = d.len();
    for i in 0..n {
        let v = d[i];
        d.push(Vec3::new(-v.x, -v.y, -v.z));
    }
    d
}

/// C7: does `cover_over` of a short arc near the near vertex hold the
/// evaluator's own points? At `f64` and at `Interval`. Prints the worst
/// miss per semi-axis decade and arc length, in metres and in `u·a`.
#[test]
fn cover_holds_the_arcs_points() {
    let u = f64::EPSILON * 0.5;
    let mut worst = std::collections::BTreeMap::<(i32, i32), (f64, f64, f64, usize)>::new();
    for seed in [0x9E37_79B9_7F4A_7C15_u64, 0x1234_5678_9ABC_DEF1, 0xDEAD_BEEF_CAFE_F00D] {
        for s in sections(seed) {
            let conic = s.curve.conic().unwrap();
            let (a, _) = conic.semi_axes();
            let f = s.curve.frame().unwrap();
            if (f.origin() - Point3::origin()).norm() < 1e2 {
                continue;
            }
            let lifted = s.curve.map_scalar(Interval::from_f64);
            for (li, len) in [0.01_f64, 1.0, 50.0].into_iter().enumerate() {
                for mid in [0.0, 0.5 * len / a, -3.0 * len / a] {
                    let (lo, hi) = (mid - len / a, mid + len / a);
                    let cover = s.curve.cover_over(ParamWindow::new(lo, hi)).unwrap();
                    let cover_i = lifted
                        .cover_over(ParamWindow::new(Interval::from_f64(lo), Interval::from_f64(hi)))
                        .unwrap();
                    let pts: Vec<_> = (0..=64)
                        .map(|k| s.curve.eval(lo + (hi - lo) * f64::from(k) / 64.0))
                        .collect();
                    let (mut m, mut mi) = (f64::NEG_INFINITY, f64::NEG_INFINITY);
                    for d in dirs(&f) {
                        let (rlo, rhi) = cover.range_along(d);
                        let di = lv(d);
                        let (ilo, ihi) = cover_i.range_along(di);
                        for p in &pts {
                            // The dot product as an enclosure, so a miss
                            // is a miss whatever the dot's own rounding.
                            let pd = (lp(*p) - Point3::origin()).dot(di);
                            m = m.max(pd.lo() - rhi).max(rlo - pd.hi());
                            mi = mi.max(pd.lo() - ihi.hi()).max(ilo.lo() - pd.hi());
                        }
                    }
                    let key = (a.log10().floor() as i32, li as i32);
                    let e = worst.entry(key).or_insert((f64::NEG_INFINITY, f64::NEG_INFINITY, 0.0, 0));
                    if m > e.0 {
                        e.0 = m;
                        e.2 = m / (u * a);
                    }
                    e.1 = e.1.max(mi);
                    e.3 += 1;
                }
            }
        }
    }
    println!("COVER decade arc_len_idx(0.01,1,50 m)  worst_miss_f64[m]  worst_miss_interval[m]  f64 miss/(u*a)  windows");
    for ((dec, li), (m, mi, ua, n)) in worst {
        println!("COVER 1e{dec} {li} {m:+.3e} {mi:+.3e} {ua:+.2} {n}");
    }
}

/// Interval transcendentals at the turn's landmarks.
#[test]
fn interval_sines_at_landmarks() {
    for t in [
        core::f64::consts::PI,
        core::f64::consts::FRAC_PI_2,
        -core::f64::consts::FRAC_PI_2,
        core::f64::consts::PI * 0.5,
    ] {
        let (s, c) = Interval::from_f64(t).sin_cos();
        println!(
            "LAND t {:e} sin [{:e}, {:e}] cos [{:e}, {:e}] f64 sin {:e} cos {:e}",
            t,
            s.lo(),
            s.hi(),
            c.lo(),
            c.hi(),
            t.sin(),
            t.cos()
        );
    }
}
