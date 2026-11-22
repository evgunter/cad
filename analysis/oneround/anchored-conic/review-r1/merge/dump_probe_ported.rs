//! Review probes for PR 7203 (`oneround/anchored-conic`), reviewer r1.
//! Not rows for the suite: each prints what it measured and asserts only
//! where it says so. Run with `--nocapture`, `--run-ignored all`.

use geom::{Conic, Curve3, Surface};
use geom::cover::ParamWindow;
use geom_brep::Reach;
use geom_brep::intersect::{PlaneConeSection, plane_cone_section};
use geom_core::{Band, Bounds, Interval, Point3, Real, Vec3};

fn band() -> Band {
    Band::new(1e-9, 1e-8).unwrap()
}

fn plane(origin: Point3<f64>, normal: Vec3<f64>) -> Surface<f64> {
    let n = normal.normalize();
    let u = if n.x.abs() < 0.9 { Vec3::unit_x() } else { Vec3::unit_y() };
    Surface::Plane {
        frame: geom::Frame::new(origin, n, (u - n * u.dot(n)).normalize()),
    }
}

fn hex(x: f64) -> String {
    format!("{:016x}", x.to_bits())
}
fn hexp(p: Point3<f64>) -> String {
    format!("{} {} {}", hex(p.x), hex(p.y), hex(p.z))
}
fn hexv(p: Vec3<f64>) -> String {
    format!("{} {} {}", hex(p.x), hex(p.y), hex(p.z))
}

/// Far-centred cone sections: dump the stored data, the f64 point, the
/// running bound and the Interval enclosure at parameters near the
/// anchor, a quarter turn and a half turn from it, for an exact oracle
/// in Python (`review_oracle.py`).
#[test]
#[ignore]
fn dump_far_cone_sections() {
    let out = std::env::var("R1_DUMP").unwrap_or("/tmp/r1_dump.txt".into());
    let mut lines = Vec::new();
    let mut rng = 0x9E37_79B9_7F4A_7C15_u64;
    let mut next = || {
        rng ^= rng << 13;
        rng ^= rng >> 7;
        rng ^= rng << 17;
        (rng >> 11) as f64 / (1u64 << 53) as f64
    };
    let half = 0.4_f64;
    let axis = Vec3::new(0.1, -0.2, -1.0).normalize();
    let apex = Point3::new(0.2, 0.1, 2.0);
    let cone_frame = geom::Frame::new(apex, axis, Vec3::new(1.0, 0.0, 0.1));
    let cone = Surface::Cone {
        frame: cone_frame,
        half_angle: half,
    };
    let parabolic = core::f64::consts::FRAC_PI_2 - half;
    for off in [1e-2, 1e-3, 1e-4, 1e-5, 1e-6, 1e-7] {
        for _ in 0..8 {
            let side = Vec3::new(next() - 0.5, next() - 0.5, next() - 0.5);
            let side = (side - axis * side.dot(axis)).normalize();
            let tilt = parabolic * (1.0 - off * (0.5 + next()));
            let n = axis * tilt.cos() + side * tilt.sin();
            let at = apex + axis * (1.0 + next());
            let cut = plane(at, n);
            let Ok(PlaneConeSection::TiltedEllipse(c)) =
                plane_cone_section(&cut, &cone, &Reach::fixture(at, 1.0), band())
            else {
                continue;
            };
            let Curve3::Ellipse(e0) = c.clone() else { unreachable!() };
            for c in [c.clone(), Curve3::Ellipse(e0.seamed_near(core::f64::consts::FRAC_PI_2)), Curve3::Ellipse(e0.seamed_near(core::f64::consts::PI)), Curve3::Ellipse(e0.seamed_near(-core::f64::consts::FRAC_PI_2).reversed())] {
            let k = c.conic().unwrap();
            let Some(geom::DatumValue::Point(anchor)) = c.data().named(geom::CurveDatum::Anchor) else {
                panic!("cone section not anchored")
            };
            let Some(geom::DatumValue::Direction(ph)) = c.data().named(geom::CurveDatum::Phase) else { panic!("no phase") };
            let f = k.frame();
            let (a, b) = k.semi_axes();
            let (c0, s0) = (ph.x, ph.y);
            let Surface::Plane { frame: pf } = cut else { unreachable!() };
            lines.push(format!(
                "C {} {} {} {} {} {} {} {} {} {} {} {}",
                hexp(anchor),
                hexv(f.axis()),
                hexv(f.u_ref()),
                hex(a),
                hex(b),
                hex(c0),
                hex(s0),
                hexp(apex),
                hexv(cone_frame.axis()),
                hex(half),
                hexp(pf.origin()),
                hexv(pf.axis()),
            ));
            let ci = c.map_scalar(Interval::from_f64);
            let ki = ci.conic().unwrap();
            let th0 = s0.atan2(c0);
            for (tag, t0) in [("near", th0), ("quarter", th0 + core::f64::consts::FRAC_PI_2), ("half", th0 + core::f64::consts::PI)] {
                for j in -4..=4 {
                    let t = t0 + f64::from(j) * 20.0 / a;
                    let p = c.eval(t);
                    let run = k.running_point(t);
                    let pi = ki.point_at(Interval::from_f64(t));
                    lines.push(format!(
                        "P {tag} {} {} {} {} {} {} {} {} {} {} {}",
                        hex(t),
                        hexp(p),
                        hex(run[0].error),
                        hex(run[1].error),
                        hex(run[2].error),
                        hex(pi.x.lo()),
                        hex(pi.x.hi()),
                        hex(pi.y.lo()),
                        hex(pi.y.hi()),
                        hex(pi.z.lo()),
                        format!("{} {}", hex(pi.z.hi()), hexp(f.origin())),
                    ));
                }
            }
            }
        }
    }
    std::fs::write(&out, lines.join("\n")).unwrap();
    println!("wrote {} lines to {out}", lines.len());
}

/// C7: does `cover_over` of a short arc near the anchor contain the
/// evaluator's points? Anchored versus centre-born at the derived centre
/// (the centre-born path is the base's code, bit for bit by C6).
#[test]
#[ignore]
fn the_arc_cover_holds_the_arcs_points() {
    let dirs = [
        Vec3::unit_x(),
        Vec3::unit_y(),
        Vec3::unit_z(),
        Vec3::new(1.0, 2.0, -0.5).normalize(),
        Vec3::new(-0.3, 0.7, 0.9).normalize(),
    ];
    let axis = Vec3::new(0.05, -0.1, 1.0).normalize();
    let toward = Vec3::new(1.0, 0.3, 0.0);
    let anchor = Point3::new(41.3, 7.7, 3.1);
    for far in [1e2, 1e3, 1e4, 1e5, 1e6, 1e7] {
        let anchored = Curve3::Ellipse(geom::Ellipse::anchored(anchor, axis, toward, far, 0.8 * far));
        let centre = anchored.conic().unwrap().frame();
        let centred = Curve3::Ellipse(geom::Ellipse::new(centre, far, 0.8 * far));
        for (name, c) in [("anchored", &anchored), ("centred", &centred)] {
            for w in [1.0 / far, 10.0 / far] {
                let cover = c.cover_over(ParamWindow::new(-w, w)).unwrap();
                let mut worst = 0.0_f64;
                for k in 0..=200 {
                    let t = -w + 2.0 * w * f64::from(k) / 200.0;
                    let p = c.eval(t);
                    for d in dirs {
                        let (lo, hi) = cover.range_along(d);
                        let x = (p - Point3::origin()).dot(d);
                        worst = worst.max(lo - x).max(x - hi);
                    }
                }
                // At Interval: the enclosure of the point must meet the
                // cover's range (the exact point lies in both).
                let ci = c.map_scalar(Interval::from_f64);
                let wi = Interval::from_f64(w);
                let cov_i = ci.cover_over(ParamWindow::new(-wi, wi)).unwrap();
                let mut worst_i = f64::NEG_INFINITY;
                for k in 0..=200 {
                    let t = -w + 2.0 * w * f64::from(k) / 200.0;
                    let p = ci.eval(Interval::from_f64(t));
                    for d in dirs {
                        let di = Vec3::new(Interval::from_f64(d.x), Interval::from_f64(d.y), Interval::from_f64(d.z));
                        let (lo, hi) = cov_i.range_along(di);
                        let x = (p - Point3::origin()).dot(di);
                        // gap > 0: the point's enclosure lies wholly outside.
                        worst_i = worst_i.max(lo.lo() - x.hi()).max(x.lo() - hi.hi());
                    }
                }
                println!(
                    "far {far:e} {name:9} window ±{w:e}: f64 worst excess {worst:e}  Interval worst gap {worst_i:e}"
                );
            }
        }
    }
}

