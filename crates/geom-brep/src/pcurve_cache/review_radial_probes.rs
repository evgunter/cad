//! Reviewer probes for PR 7370's radial readers (`wall_distance`,
//! `ruling_direction`, `geom_core::radius_gap`). Each prints what it
//! measured; run with `--nocapture`. `review_` rows are evidence for the
//! review, not part of the gate.

use geom::{Curve3, Surface};
use geom_core::k_stats::decide_reported;
use geom_core::sym::with_session;
use geom_core::{
    Bounds, Dual, Interval, Margin, ParamSymbol, Point3, Real, Sign, Sym, SymBudget, Vec3,
};

use super::harmonic_sup_rows::band;
use super::{chart_distance, chart_pcurve};

type S = Sym<Interval>;

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 4096,
        max_degree: 64,
    }
}

fn boxed(name: &str, lo: f64, hi: f64) -> S {
    Sym::param_over(
        ParamSymbol::new(test_utils::symbol_id(name)),
        Interval::from_bounds(lo, hi),
        lo,
        hi,
    )
}

fn lit(x: f64) -> S {
    S::from_f64(x)
}

fn lit3(v: Vec3<f64>) -> Vec3<S> {
    Vec3::new(lit(v.x), lit(v.y), lit(v.z))
}

fn litp(p: Point3<f64>) -> Point3<S> {
    Point3::new(lit(p.x), lit(p.y), lit(p.z))
}

fn read<T: geom_core::Decide>(d: T) -> String {
    match decide_reported("review_radial_probe", Margin::of(d), band()) {
        Ok(dd) => format!("{:?} proven={}", dd.sign, dd.margin.proven()),
        Err(e) => format!("refused {e:?}"),
    }
}

/// The old spelling, for a direct comparison.
fn old_cyl<T: Real>(radius: T, w: Vec3<T>) -> T {
    (w.norm() - radius).abs()
}

/// P1/P2: a point built at `c + r·x` against a cylinder of radius `r`,
/// over a box on `r` that is wholly negative, or straddles zero. The
/// real distance `|‖w‖ − r|` is `2|r|` where `r < 0`.
#[test]
fn review_sym_negative_and_straddling_radius_boxes() {
    let axis = Vec3::new(0.0, 0.0, 1.0);
    let u_ref = Vec3::new(0.0, 1.0, 0.0);
    let c = Point3::new(0.5, -0.25, 2.0);
    for (lo, hi) in [(-0.4, -0.2), (-0.1, 0.3), (0.2, 0.4)] {
        with_session(budget(), || {
            let r = boxed("review_neg_radius", lo, hi);
            for (kind, sphere) in [("cylinder", false), ("sphere", true)] {
                let frame = geom::Frame::new(litp(c), lit3(axis), lit3(u_ref));
                let s = if sphere {
                    Surface::Sphere { frame, radius: r }
                } else {
                    Surface::Cylinder { frame, radius: r }
                };
                let p = litp(c) + lit3(u_ref) * r;
                let d = chart_distance(&s, p);
                let o = old_cyl(r, p - litp(c));
                eprintln!(
                    "PROBE P1 r in [{lo}, {hi}] {kind}: new {} value [{}, {}]; old {} value [{}, {}]",
                    read(d),
                    d.value.lo(),
                    d.value.hi(),
                    read(o),
                    o.value.lo(),
                    o.value.hi()
                );
            }
        });
    }
}

/// P3: plain `Interval` and `f64` at the degenerate data: radius 0 and a
/// point on the axis; a radius interval straddling 0 with the point
/// built at it; a negative radius on its mirror wall.
#[test]
fn review_interval_and_f64_degenerate_data() {
    let iv = Interval::from_f64;
    let ivb = Interval::from_bounds;
    let cyl_iv = |radius: Interval| Surface::Cylinder {
        frame: geom::Frame::new(
            Point3::new(iv(0.0), iv(0.0), iv(0.0)),
            Vec3::new(iv(0.0), iv(0.0), iv(1.0)),
            Vec3::new(iv(1.0), iv(0.0), iv(0.0)),
        ),
        radius,
    };
    let cyl = |radius: f64| Surface::Cylinder {
        frame: geom::Frame::new(
            Point3::origin(),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
        ),
        radius,
    };
    let on_axis = Point3::new(iv(0.0), iv(0.0), iv(0.3));
    let d = chart_distance(&cyl_iv(iv(0.0)), on_axis);
    eprintln!("PROBE P3 Interval r=0, on axis: {} [{}, {}]", read(d), d.lo(), d.hi());
    let d = chart_distance(&cyl(0.0), Point3::new(0.0, 0.0, 0.3));
    eprintln!("PROBE P3 f64 r=0, on axis: {} {d}", read(d));
    let d = chart_distance(&cyl(0.0), Point3::new(1e-3, 0.0, 0.3));
    eprintln!("PROBE P3 f64 r=0, off axis by 1e-3: {} {d}", read(d));
    let r = ivb(-0.1, 0.3);
    let p = Point3::new(r, iv(0.0), iv(0.3));
    let d = chart_distance(&cyl_iv(r), p);
    eprintln!("PROBE P3 Interval r=[-0.1,0.3], built at r: {} [{}, {}]", read(d), d.lo(), d.hi());
    let dd = old_cyl(r, Vec3::new(r, iv(0.0), iv(0.0)));
    eprintln!("PROBE P3 Interval old same: {} [{}, {}]", read(dd), dd.lo(), dd.hi());
    let r = ivb(0.2, 0.4);
    let p = Point3::new(r, iv(0.0), iv(0.3));
    let d = chart_distance(&cyl_iv(r), p);
    let dd = old_cyl(r, Vec3::new(r, iv(0.0), iv(0.0)));
    eprintln!(
        "PROBE P3 Interval r=[0.2,0.4] built at r: new [{}, {}] old [{}, {}]",
        d.lo(),
        d.hi(),
        dd.lo(),
        dd.hi()
    );
    // A negative radius at f64, points on the mirror wall at many angles:
    // how many read a finite value, how many decide Zero.
    let mut counts = std::collections::BTreeMap::<String, u32>::new();
    let mut examples = Vec::new();
    for radius in [0.7_f64, 1.0e-3, 1.0e3, 0.123_456_789, 3.3, 12.5] {
        for k in 0..2000 {
            let a = f64::from(k) * 0.001_234_567 * core::f64::consts::TAU;
            let p = Point3::new(radius * a.cos(), radius * a.sin(), 0.25);
            let d = chart_distance(&cyl(-radius), p);
            let tag = format!(
                "{} {}",
                if d.is_nan() { "NaN" } else if d == 0.0 { "zero" } else { "finite" },
                read(d).split_whitespace().next().unwrap_or("").to_owned()
            );
            if d.is_finite() && examples.len() < 6 {
                examples.push((radius, a, d));
            }
            *counts.entry(tag).or_default() += 1;
        }
    }
    eprintln!("PROBE P3 f64 mirror wall readings (cylinder): {counts:?}; finite examples {examples:?}");
}

/// P4: the segment premise at f64 on the mirror wall.
#[test]
fn review_segment_mirror_wall() {
    // `segment_off_cylinder` is private to props; probed in its own file.
}

/// P5: the derivative of the quotient against the difference's, at
/// `Dual<f64>`, along a direction moving the point and the radius.
#[test]
fn review_dual_derivative() {
    let mut worst: f64 = 0.0;
    for (radius, reach, dr, dw) in [
        (0.7, 1.0, 0.3, 1.1),
        (0.7, 0.5, -0.2, 0.4),
        (1.0e3, 1.0 + 1e-9, 1.0, -2.0),
        (1.0e-3, 2.0, 0.0, 1.0),
        (0.7, 1e-8, 0.0, 1.0),
        (0.7, 0.0, 1.0, 1.0),
    ] {
        let r = Dual::new(radius, dr);
        let x = Vec3::new(0.6, 0.8, 0.0);
        let w = Vec3::new(
            Dual::new(x.x * radius * reach, x.x * dw),
            Dual::new(x.y * radius * reach, x.y * dw),
            Dual::new(0.0, 0.0),
        );
        let new = geom_core::radius_gap(w.dot(w), w.norm(), r);
        let old = w.norm() - r;
        eprintln!(
            "PROBE P5 R={radius} reach={reach}: new ({}, {}) old ({}, {})",
            new.value, new.deriv, old.value, old.deriv
        );
        if new.deriv.is_finite() {
            worst = worst.max((new.deriv - old.deriv).abs());
        }
    }
    // Near the mirror wall's limit: a radius going to −‖w‖.
    for eps in [1e-4, 1e-8, 1e-12, 0.0] {
        let r = Dual::new(-1.0 + eps, 1.0);
        let w = Vec3::new(Dual::new(1.0, 0.0), Dual::new(0.0, 0.0), Dual::new(0.0, 0.0));
        let new = geom_core::radius_gap(w.dot(w), w.norm(), r);
        eprintln!("PROBE P5 mirror eps={eps}: new ({}, {})", new.value, new.deriv);
    }
    eprintln!("PROBE P5 worst |d new − d old| = {worst}");
}

/// P6: spellings of "built on the wall" a document produces, at
/// `Sym<Interval>` over a box on the radius: the carrier's own `eval`, a
/// tilted literal axis, a frame from a boxed angle, a circle carrier's
/// point at a parameter.
#[test]
fn review_sym_spellings_that_may_not_fold() {
    with_session(budget(), || {
        let r = boxed("review_spelling_radius", 0.2, 0.4);
        let c = Point3::new(0.5, -0.25, 2.0);
        let frames: Vec<(&str, Vec3<f64>, Vec3<f64>)> = vec![
            ("straight", Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 1.0, 0.0)),
            ("tilted (0.6, 0, 0.8)", Vec3::new(0.6, 0.0, 0.8), Vec3::new(0.0, 1.0, 0.0)),
            (
                "tilted normalized",
                Vec3::new(0.1, -0.2, 1.0).normalize(),
                {
                    let a = Vec3::new(0.1, -0.2, 1.0).normalize();
                    let raw = Vec3::new(1.0, 0.0, -0.1);
                    (raw - a * raw.dot(a)).normalize()
                },
            ),
        ];
        for (fname, axis, u_ref) in frames {
            let frame = geom::Frame::new(litp(c), lit3(axis), lit3(u_ref));
            let cyl = Surface::Cylinder { frame, radius: r };
            let sph = Surface::Sphere { frame, radius: r };
            // The carrier's own eval at a literal and at a boxed angle.
            let ang = boxed("review_spelling_angle", 0.3, 0.9);
            for (pname, u) in [("u=0.7", lit(0.7)), ("u boxed", ang)] {
                let p = cyl.eval(u, lit(0.4));
                eprintln!("PROBE P6 {fname} cylinder eval {pname}: new {} old {}", read(chart_distance(&cyl, p)), {
                    let (_, w) = crate::implicit::axial_radial(p, frame.origin(), frame.axis());
                    read(old_cyl(r, w))
                });
                let p = sph.eval(u, lit(0.3));
                eprintln!("PROBE P6 {fname} sphere eval {pname}: new {} old {}", read(chart_distance(&sph, p)), read(old_cyl(r, p - frame.origin())));
            }
            // Written as c + r·x + h·axis with x = cross(axis, u_ref).
            let x = lit3(axis.cross(u_ref));
            let p = litp(c) + x * r + lit3(axis) * lit(0.9);
            eprintln!("PROBE P6 {fname} cylinder c + r·(axis×u_ref) + h·axis: new {}", read(chart_distance(&cyl, p)));
            // A circle carrier of radius r about the axis, read at a parameter.
            let circle = geom::Circle::new(geom::Frame::new(litp(c), lit3(axis), lit3(u_ref)), r);
            let p = Curve3::Circle(circle).eval(lit(1.1));
            eprintln!("PROBE P6 {fname} circle carrier eval: cylinder new {} / sphere new {}", read(chart_distance(&cyl, p)), read(chart_distance(&sph, p)));
        }
    });
}

/// P7: incidence near the band's edge at plain `Interval` on a tilted
/// cylinder far from its axis: points at a distance straddling the
/// band's edge, read both ways; list where the two decisions differ.
#[test]
fn review_interval_band_edge_flips() {
    let tol = band().zero();
    eprintln!("PROBE P7 band zero = {tol:e}");
    let iv = Interval::from_f64;
    let axis = Vec3::new(0.1, -0.2, 1.0).normalize();
    let raw = Vec3::new(1.0, 0.0, -0.1);
    let u_ref = (raw - axis * raw.dot(axis)).normalize();
    let cv = axis.cross(u_ref);
    let c = Point3::new(0.4, 0.1, -0.3);
    let iv3 = |v: Vec3<f64>| Vec3::new(iv(v.x), iv(v.y), iv(v.z));
    let ivp = |p: Point3<f64>| Point3::new(iv(p.x), iv(p.y), iv(p.z));
    let mut flips = Vec::new();
    let mut total = 0;
    for radius in [0.7_f64, 12.0, 1.0e3] {
        let s = Surface::Cylinder {
            frame: geom::Frame::new(ivp(c), iv3(axis), iv3(u_ref)),
            radius: iv(radius),
        };
        for k in 0..400 {
            let theta = 0.37 + f64::from(k) * 0.0157;
            let x = u_ref * theta.cos() + cv * theta.sin();
            for j in -60..=60 {
                let off = tol * (1.0 + f64::from(j) * 1e-7 * radius.max(1.0));
                for sign in [1.0, -1.0] {
                    let p = c + x * (radius + sign * off) + axis * 0.37;
                    let new = chart_distance(&s, ivp(p));
                    let q = ivp(p) - ivp(c);
                    let w = q - iv3(axis) * q.dot(iv3(axis));
                    let old = old_cyl(iv(radius), w);
                    let cat = |x: String| x.split_whitespace().next().unwrap_or("").to_owned();
                    let (a, b) = (cat(read(new)), cat(read(old)));
                    total += 1;
                    if a != b {
                        flips.push(format!("R={radius} θ={theta:.4} off={off:e} sgn={sign}: head {a} base {b}"));
                    }
                }
            }
        }
    }
    eprintln!("PROBE P7 {} of {total} near-edge readings decide differently", flips.len());
    let mut kinds = std::collections::BTreeMap::<String, u32>::new();
    for f in &flips {
        *kinds.entry(f.split(": ").nth(1).unwrap_or("").to_owned()).or_default() += 1;
    }
    eprintln!("PROBE P7 kinds {kinds:?}");
    for f in flips.iter().take(12) {
        eprintln!("PROBE P7   {f}");
    }
}

/// P8: the ruling's phase on a cylinder whose radius is negative (at
/// `f64`), and the image's eval against the line.
#[test]
fn review_ruling_on_negative_radius() {
    let axis = Vec3::new(0.0, 0.0, 1.0);
    let u_ref = Vec3::new(0.0, 1.0, 0.0);
    let c = Point3::new(0.5, -0.25, 2.0);
    for radius in [0.3, -0.3] {
        let s = Surface::Cylinder {
            frame: geom::Frame::new(c, axis, u_ref),
            radius,
        };
        let line = geom::Line {
            origin: c + u_ref * radius,
            dir: axis,
        };
        match chart_pcurve(&Curve3::Line(line), &s, band()) {
            Ok(img) => {
                let q = img.eval(0.5);
                let back = s.eval(q.x, q.y);
                eprintln!(
                    "PROBE P8 R={radius}: phase {} ; eval back {:?} vs line {:?}",
                    q.x,
                    back,
                    line.origin + line.dir * 0.5
                );
            }
            Err(e) => eprintln!("PROBE P8 R={radius}: refused {e:?}"),
        }
    }
    let _ = Sign::Zero;
}

/// P9: the readers' cost, head against the base spelling, at `f64` and
/// `Interval`, `chart_distance` on a tilted cylinder and a sphere.
#[test]
fn review_cost_of_the_readers() {
    use std::hint::black_box;
    use std::time::Instant;
    let axis = Vec3::new(0.1, -0.2, 1.0).normalize();
    let raw = Vec3::new(1.0, 0.0, -0.1);
    let u_ref = (raw - axis * raw.dot(axis)).normalize();
    let c = Point3::new(0.4, 0.1, -0.3);
    let n = 400_000;
    let pts: Vec<Point3<f64>> = (0..n)
        .map(|k| {
            let a = f64::from(k) * 0.001;
            c + (u_ref * a.cos() + axis.cross(u_ref) * a.sin()) * (0.7 + 1e-9 * f64::from(k % 7)) + axis * 0.3
        })
        .collect();
    let iv = Interval::from_f64;
    let ivp = |p: Point3<f64>| Point3::new(iv(p.x), iv(p.y), iv(p.z));
    let ipts: Vec<Point3<Interval>> = pts.iter().map(|&p| ivp(p)).collect();
    for kind in ["cylinder", "sphere"] {
        let mk = |sph: bool| {
            if sph {
                Surface::Sphere { frame: geom::Frame::new(c, axis, u_ref), radius: 0.7 }
            } else {
                Surface::Cylinder { frame: geom::Frame::new(c, axis, u_ref), radius: 0.7 }
            }
        };
        let s = mk(kind == "sphere");
        let si = if kind == "sphere" {
            Surface::Sphere { frame: geom::Frame::new(ivp(c), Vec3::new(iv(axis.x), iv(axis.y), iv(axis.z)), Vec3::new(iv(u_ref.x), iv(u_ref.y), iv(u_ref.z))), radius: iv(0.7) }
        } else {
            Surface::Cylinder { frame: geom::Frame::new(ivp(c), Vec3::new(iv(axis.x), iv(axis.y), iv(axis.z)), Vec3::new(iv(u_ref.x), iv(u_ref.y), iv(u_ref.z))), radius: iv(0.7) }
        };
        for round in 0..3 {
            let t = Instant::now();
            let mut acc = 0.0;
            for &p in &pts {
                acc += black_box(chart_distance(black_box(&s), p));
            }
            let f = t.elapsed();
            let t = Instant::now();
            let mut acci = 0.0;
            for &p in &ipts {
                acci += black_box(chart_distance(black_box(&si), p)).hi();
            }
            let fi = t.elapsed();
            eprintln!(
                "PROBE P9 {kind} mode={} round {round}: f64 {:.1} ns/call, Interval {:.1} ns/call ({acc:.3e} {acci:.3e})",
                crate::review_mut(),
                f.as_nanos() as f64 / f64::from(n),
                fi.as_nanos() as f64 / f64::from(n)
            );
        }
    }
}
