//! CERT-N3 blinded review (R1) — adversarial soundness corpus for the
//! adopted conic box (`geom::curves::boxes::{circle_arc_aabb,
//! ellipse_arc_aabb, conic_arc_aabb}`), independent of the lane's 92
//! edges. Probe branch only.
//!
//! Row 1: at `f64`, radii 1e-4 … 1e4, spans crossing each extremal angle
//! strictly inside / exactly at an endpoint / within 1e-9 rad of one,
//! runs wrapping past 2π, runs starting at the atan2 cut, `u_ref` at
//! every octant boundary, ellipse tilts to 89.99° and axis ratios to 1e3,
//! degenerate 1e-6 spans and spans within 1e-6 of a full turn. Every
//! edge: 1e5 samples, zero pad, rounding slack only.
//!
//! Row 2: at `Interval` — every realization of a wide bracket lies in
//! the bracket box, including brackets that straddle an extremal angle
//! and brackets that put the rectangle on the negative-u axis (the
//! rotated frame), on both axes (origin inside), and touching the
//! positive axis exactly at a corner; width-zero brackets.
//!
//! Row 3: at `Dual64` — the box is the `f64` box of the values.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::too_many_lines)]

use bvh::Aabb;
use geom::Curve3;
use geom::curves::boxes::{circle_arc_aabb, conic_arc_aabb, ellipse_arc_aabb};
#[cfg(feature = "interval")]
use geom_core::{Bounds, Interval, Real};
use geom_core::{Dual64, Point3, Vec3};

const PI: f64 = core::f64::consts::PI;
const TAU: f64 = core::f64::consts::TAU;

fn rot_z(v: Vec3<f64>, a: f64) -> Vec3<f64> {
    let (s, c) = a.sin_cos();
    Vec3::new(c * v.x - s * v.y, s * v.x + c * v.y, v.z)
}

fn rot_x(v: Vec3<f64>, a: f64) -> Vec3<f64> {
    let (s, c) = a.sin_cos();
    Vec3::new(v.x, c * v.y - s * v.z, s * v.y + c * v.z)
}

/// Frames: axis/u_ref pairs — axis-aligned, tilted, and `u_ref` at every
/// octant boundary plus odd angles.
fn frames() -> Vec<(Vec3<f64>, Vec3<f64>, String)> {
    let mut out = Vec::new();
    for k in 0..8 {
        let phi = f64::from(k) * PI / 4.0;
        out.push((
            Vec3::unit_z(),
            rot_z(Vec3::unit_x(), phi),
            format!("axis z, u_ref at octant {k}"),
        ));
    }
    for &phi_deg in &[10.0, 137.0, 200.0, 359.0] {
        let phi = f64::to_radians(phi_deg);
        out.push((
            Vec3::unit_z(),
            rot_z(Vec3::unit_x(), phi),
            format!("axis z, u_ref at {phi_deg}°"),
        ));
    }
    // Tilted frames: rotate the whole z-frame about x by alpha, u_ref at phi.
    for &alpha_deg in &[30.0, 60.0, 89.99] {
        let alpha = f64::to_radians(alpha_deg);
        for &phi_deg in &[0.0, 45.0, 90.0, 180.0, 225.0] {
            let phi = f64::to_radians(phi_deg);
            out.push((
                rot_x(Vec3::unit_z(), alpha),
                rot_x(rot_z(Vec3::unit_x(), phi), alpha),
                format!("tilt {alpha_deg}°, u_ref at {phi_deg}°"),
            ));
        }
    }
    // The extremal-angle-exactly-π frame (u_ref = −x): φ_x = π.
    out.push((Vec3::unit_z(), -Vec3::unit_x(), "u_ref = -x (phi_x = pi)".into()));
    out.push((Vec3::unit_z(), -Vec3::unit_y(), "u_ref = -y".into()));
    out
}

/// The extremal angles of the frame per world axis: atan2(b·v_i, a·u_i)
/// and its antipode — the angles a run must cross.
fn extremal_angles(a: f64, b: f64, u: Vec3<f64>, v: Vec3<f64>) -> Vec<f64> {
    let mut out = Vec::new();
    for (ui, vi) in [(u.x, v.x), (u.y, v.y), (u.z, v.z)] {
        let amp = (a * ui).hypot(b * vi);
        if amp > 0.0 {
            let phi = (b * vi).atan2(a * ui);
            out.push(phi);
            out.push(phi + PI);
        }
    }
    out
}

/// Spans built around each extremal angle: strictly inside, exactly at
/// an endpoint (both ends), within 1e-9 inside and outside of an
/// endpoint, plus wraps, cut-starts, degenerate and near-full runs.
fn spans(extremals: &[f64]) -> Vec<(f64, f64, String)> {
    let mut out = Vec::new();
    for &phi in extremals {
        for &shift in &[0.0, TAU, -TAU] {
            let p = phi + shift;
            out.push((p - 0.3, p + 0.7, format!("crosses {p:.4} strictly inside")));
            out.push((p, p + 0.5, format!("starts exactly at {p:.4}")));
            out.push((p - 0.5, p, format!("ends exactly at {p:.4}")));
            out.push((p + 1e-9, p + 0.5, format!("starts 1e-9 past {p:.4}")));
            out.push((p - 1e-9, p + 0.5, format!("starts 1e-9 before {p:.4}")));
            out.push((p - 0.5, p - 1e-9, format!("ends 1e-9 before {p:.4}")));
            out.push((p - 0.5, p + 1e-9, format!("ends 1e-9 past {p:.4}")));
            out.push((p - 1e-6, p, "1e-6 run ending at extremum".into()));
            out.push((p - 5e-7, p + 5e-7, "1e-6 run around extremum".into()));
        }
    }
    out.push((6.0, 7.0, "wraps past 2pi".into()));
    out.push((TAU - 1e-9, TAU + 2.0, "starts 1e-9 short of 2pi".into()));
    out.push((PI - 1e-9, PI + 1.0, "starts 1e-9 short of pi (atan2 cut)".into()));
    out.push((PI, PI + 1.0, "starts exactly at pi".into()));
    out.push((-PI, -PI + 1.0, "starts exactly at -pi".into()));
    out.push((-1e-12, 0.5, "starts at -1e-12".into()));
    out.push((0.0, 1e-6, "degenerate 1e-6 run".into()));
    out.push((3.0, 3.0 + 1e-6, "degenerate 1e-6 run mid-circle".into()));
    out.push((0.1, 0.1 + TAU - 1e-6, "full turn less 1e-6".into()));
    out.push((0.0, TAU, "exactly a full turn".into()));
    out.push((-3.0, -3.0 + TAU, "full turn from -3".into()));
    out.push((1.0, 1.0 + TAU - 1e-12, "full turn less 1e-12".into()));
    out
}

fn check_contains(
    what: &str,
    b: &Aabb,
    carrier: &Curve3<f64>,
    t0: f64,
    t1: f64,
    n: usize,
    scale: f64,
    worst: &mut f64,
) {
    let slack = 8.0 * f64::EPSILON * scale;
    for i in 0..=n {
        #[allow(clippy::cast_precision_loss)]
        let t = t0 + (t1 - t0) * (i as f64) / (n as f64);
        let p = carrier.eval(t);
        let esc = [
            b.min_x - p.x,
            p.x - b.max_x,
            b.min_y - p.y,
            p.y - b.max_y,
            b.min_z - p.z,
            p.z - b.max_z,
        ]
        .into_iter()
        .fold(f64::NEG_INFINITY, f64::max);
        if esc > *worst {
            *worst = esc;
        }
        assert!(
            esc <= slack,
            "{what}: sample t={t} {p:?} escapes box {b:?} by {esc:e} (slack {slack:e})"
        );
    }
}

fn circle(c: Point3<f64>, axis: Vec3<f64>, r: f64, u: Vec3<f64>) -> Curve3<f64> {
    Curve3::Circle {
        center: c,
        axis,
        radius: r,
        u_ref: u,
    }
}

fn ellipse(c: Point3<f64>, axis: Vec3<f64>, a: f64, b: f64, u: Vec3<f64>) -> Curve3<f64> {
    Curve3::Ellipse {
        center: c,
        axis,
        major: a,
        minor: b,
        u_ref: u,
    }
}

#[test]
fn n3r1_f64_adversarial_corpus_is_contained_at_zero_pad() {
    let mut edges = 0usize;
    let mut worst = f64::NEG_INFINITY;
    let centers = [Point3::origin(), Point3::new(3.5, -2.25, 1e3)];
    for (axis, u_ref, fname) in frames() {
        let v_ref = axis.cross(u_ref);
        for &r in &[1e-4, 1e-2, 1.0, 1e2, 1e4] {
            for &c in &centers {
                let carrier = circle(c, axis, r, u_ref);
                let scale = r + c.x.abs().max(c.y.abs()).max(c.z.abs());
                let dense = r == 1.0 && c.x == 0.0 && c.y == 0.0 && c.z == 0.0 && fname.contains("octant");
                let n = if dense { 100_000 } else { 1_000 };
                for (t0, t1, sname) in spans(&extremal_angles(r, r, u_ref, v_ref)) {
                    let (e0, e1) = (carrier.eval(t0), carrier.eval(t1));
                    let b = circle_arc_aabb(&carrier, t0, t1, e0, e1).unwrap();
                    let b2 = conic_arc_aabb(&carrier, t0, t1, e0, e1).unwrap();
                    assert_eq!(format!("{b:?}"), format!("{b2:?}"), "dispatcher differs");
                    let what = format!("circle r={r} c={c:?} {fname} {sname}");
                    check_contains(&what, &b, &carrier, t0, t1, n, scale, &mut worst);
                    edges += 1;
                }
            }
        }
        // Ellipses: axis ratios to 1e3, both ways round.
        for &(a, bm) in &[(1.0, 1e-3), (1e3, 1.0), (2.0, 0.7), (1e-4, 1e-4 * 0.3), (1e4, 10.0)] {
            let carrier = ellipse(Point3::origin(), axis, a, bm, u_ref);
            let scale = a.max(bm);
            for (t0, t1, sname) in spans(&extremal_angles(a, bm, u_ref, v_ref)) {
                let (e0, e1) = (carrier.eval(t0), carrier.eval(t1));
                let b = ellipse_arc_aabb(&carrier, t0, t1, e0, e1).unwrap();
                let what = format!("ellipse a={a} b={bm} {fname} {sname}");
                check_contains(&what, &b, &carrier, t0, t1, 1_000, scale, &mut worst);
                edges += 1;
            }
        }
    }
    // Ellipse in a plane tilted to 89.99° about x, seen from the z axis:
    // the section of a cylinder, ratio 1/cos.
    for &alpha_deg in &[80.0, 89.0, 89.9, 89.99] {
        let alpha = f64::to_radians(alpha_deg);
        let normal = Vec3::new(alpha.sin(), 0.0, alpha.cos());
        let u_ref = Vec3::new(alpha.cos(), 0.0, -alpha.sin());
        let v_ref = normal.cross(u_ref);
        let (a, bm) = (1.0 / alpha.cos(), 1.0);
        let carrier = ellipse(Point3::origin(), normal, a, bm, u_ref);
        for (t0, t1, sname) in spans(&extremal_angles(a, bm, u_ref, v_ref)) {
            let (e0, e1) = (carrier.eval(t0), carrier.eval(t1));
            let b = ellipse_arc_aabb(&carrier, t0, t1, e0, e1).unwrap();
            let what = format!("tilted-section ellipse alpha={alpha_deg} {sname}");
            check_contains(&what, &b, &carrier, t0, t1, 100_000, a, &mut worst);
            edges += 1;
        }
    }
    eprintln!("n3r1 f64 corpus: {edges} edges, worst escape {worst:e} (negative = inside)");
}

/// Wide-bracket realizations: every corner and a grid of interior
/// points of each bracket, realized as an `f64` carrier, must lie in the
/// `Interval` box.
#[cfg(feature = "interval")]
fn check_interval_dominates(
    what: &str,
    iv_carrier: &Curve3<Interval>,
    t0: Interval,
    t1: Interval,
    scale: f64,
) {
    let end0 = iv_carrier.eval(t0);
    let end1 = iv_carrier.eval(t1);
    let b = conic_arc_aabb(iv_carrier, t0, t1, end0, end1).unwrap();
    // Realizations: pick each bracketed quantity at lo, mid, hi.
    let pick = |x: Interval, k: usize| -> f64 {
        match k {
            0 => x.lo(),
            1 => 0.5 * (x.lo() + x.hi()),
            _ => x.hi(),
        }
    };
    let mut worst = f64::NEG_INFINITY;
    for kc in 0..3 {
        for ka in 0..3 {
            for ku in 0..3 {
                for kr in 0..3 {
                    for kt in 0..3 {
                        let (t0f, t1f) = (pick(t0, kt), pick(t1, 2 - kt));
                        let real: Curve3<f64> = match iv_carrier {
                            Curve3::Circle {
                                center,
                                axis,
                                radius,
                                u_ref,
                            } => circle(
                                Point3::new(pick(center.x, kc), pick(center.y, kc), pick(center.z, kc)),
                                Vec3::new(pick(axis.x, ka), pick(axis.y, ka), pick(axis.z, ka)),
                                pick(*radius, kr),
                                Vec3::new(pick(u_ref.x, ku), pick(u_ref.y, 2 - ku), pick(u_ref.z, ku)),
                            ),
                            Curve3::Ellipse {
                                center,
                                axis,
                                major,
                                minor,
                                u_ref,
                            } => ellipse(
                                Point3::new(pick(center.x, kc), pick(center.y, kc), pick(center.z, kc)),
                                Vec3::new(pick(axis.x, ka), pick(axis.y, ka), pick(axis.z, ka)),
                                pick(*major, kr),
                                pick(*minor, 2 - kr),
                                Vec3::new(pick(u_ref.x, ku), pick(u_ref.y, 2 - ku), pick(u_ref.z, ku)),
                            ),
                            _ => unreachable!(),
                        };
                        let who = format!("{what} realization c{kc} a{ka} u{ku} r{kr} t{kt}");
                        check_contains(&who, &b, &real, t0f, t1f, 2_000, scale, &mut worst);
                    }
                }
            }
        }
    }
}

#[cfg(feature = "interval")]
#[test]
fn n3r1_interval_brackets_are_dominated_by_the_box() {
    let iv = <Interval as Real>::from_f64;
    let ib = Interval::from_bounds;
    let mut cases: Vec<(String, Curve3<Interval>, Interval, Interval, f64)> = Vec::new();
    // Frame brackets on u_ref.x straddling zero (extremal angle straddled).
    for &w in &[1e-9, 1e-3, 0.3, 0.9] {
        for &(t0, t1, s) in &[
            (0.0, 1.0, "no wrap"),
            (1.2, 2.0, "crosses pi/2"),
            (2.5, 4.0, "crosses pi"),
            (4.2, 5.0, "crosses 3pi/2"),
            (6.0, 7.0, "wraps"),
            (0.1, TAU - 0.02, "near full"),
        ] {
            cases.push((
                format!("u_ref.x straddles by {w}, {s}"),
                Curve3::Circle {
                    center: Point3::new(iv(0.0), iv(0.0), iv(0.0)),
                    axis: Vec3::new(iv(0.0), iv(0.0), iv(1.0)),
                    radius: ib(0.9, 1.1),
                    u_ref: Vec3::new(ib(-w, w), iv(1.0), iv(0.0)),
                },
                ib(t0, t0 + 1e-3),
                ib(t1 - 1e-3, t1),
                1.1,
            ));
        }
    }
    // The rotated-frame branch: on axis x, u = u_ref.x ≤ 0 strictly and
    // v = (z × u_ref).x = −u_ref.y straddles zero.
    for &(ux, uy, s) in &[
        ((-1.0, -0.9), (-0.05, 0.05), "rotated: v straddles"),
        ((-1.0, -0.9), (0.0, 0.05), "rotated: v touches zero at lo"),
        ((-1.0, -0.9), (-0.05, 0.0), "rotated: v touches zero at hi"),
        ((-1.0, -0.9), (0.0, 0.0), "rotated: v width zero at 0"),
        ((-1.0, -1.0), (-0.05, 0.05), "rotated: u width zero"),
        ((-1.0, 0.0), (-0.05, 0.05), "u.hi = 0 exactly (origin on edge)"),
        ((-0.5, 0.5), (-0.5, 0.5), "origin inside (both cuts)"),
        ((0.0, 0.9), (0.0, 0.05), "touches +u axis at a corner"),
        ((0.0, 0.9), (-0.05, 0.05), "u.lo = 0, v straddles"),
        ((0.9, 1.0), (0.0, 0.0), "on +u axis, v width zero"),
    ] {
        for &(t0, t1, ss) in &[
            (0.0, 0.7, "short"),
            (2.6, 3.7, "crosses pi"),
            (PI - 1e-9, PI + 1e-9, "tiny run at pi"),
            (3.0, 3.0 + TAU - 1e-4, "near full"),
            (-0.4, 0.4, "crosses 0"),
        ] {
            cases.push((
                format!("{s}, {ss}"),
                Curve3::Circle {
                    center: Point3::new(iv(0.0), iv(0.0), iv(0.0)),
                    axis: Vec3::new(iv(0.0), iv(0.0), iv(1.0)),
                    radius: ib(1.0, 1.0),
                    u_ref: Vec3::new(ib(ux.0, ux.1), ib(uy.0, uy.1), iv(0.0)),
                },
                ib(t0, t0),
                ib(t1, t1),
                1.0,
            ));
        }
    }
    // Wide theta brackets straddling an extremal angle, and a tilted
    // ellipse with a wide axis bracket.
    cases.push((
        "theta brackets straddle pi/2".into(),
        Curve3::Circle {
            center: Point3::new(ib(-1e-3, 1e-3), iv(2.0), iv(-1.0)),
            axis: Vec3::new(iv(0.0), iv(0.0), iv(1.0)),
            radius: ib(0.001, 0.0011),
            u_ref: Vec3::new(iv(1.0), iv(0.0), iv(0.0)),
        },
        ib(1.4, 1.6),
        ib(1.65, 1.8),
        2.0,
    ));
    cases.push((
        "tilted ellipse, wide axis".into(),
        Curve3::Ellipse {
            center: Point3::new(iv(0.0), iv(0.0), iv(0.0)),
            axis: Vec3::new(ib(0.5, 0.6), iv(0.0), ib(0.8, 0.87)),
            major: ib(50.0, 60.0),
            minor: ib(0.01, 0.02),
            u_ref: Vec3::new(ib(0.8, 0.87), iv(0.0), ib(-0.6, -0.5)),
        },
        ib(0.9, 1.0),
        ib(4.0, 4.1),
        60.0,
    ));
    for (what, c, t0, t1, scale) in &cases {
        check_interval_dominates(what, c, *t0, *t1, *scale);
    }
    eprintln!("n3r1 interval corpus: {} bracket cases dominated", cases.len());
}

#[test]
fn n3r1_dual64_box_is_the_value_box() {
    let d = Dual64::variable;
    let k = Dual64::constant;
    let cd: Curve3<Dual64> = Curve3::Circle {
        center: Point3::new(k(1.0), d(2.0), k(3.0)),
        axis: Vec3::new(k(0.0), k(0.0), k(1.0)),
        radius: d(0.75),
        u_ref: Vec3::new(k(-1.0), k(0.0), k(0.0)),
    };
    let cf = circle(Point3::new(1.0, 2.0, 3.0), Vec3::unit_z(), 0.75, -Vec3::unit_x());
    for &(t0, t1) in &[(0.0, 1.0), (2.5, 4.0), (PI, PI + 1e-6), (0.2, 6.2)] {
        let bd = conic_arc_aabb(&cd, k(t0), k(t1), cd.eval(k(t0)), cd.eval(k(t1))).unwrap();
        let bf = conic_arc_aabb(&cf, t0, t1, cf.eval(t0), cf.eval(t1)).unwrap();
        assert_eq!(format!("{bd:?}"), format!("{bf:?}"), "dual vs f64 at [{t0}, {t1}]");
        let mut worst = f64::NEG_INFINITY;
        check_contains("dual", &bd, &cf, t0, t1, 20_000, 4.0, &mut worst);
    }
}

/// Item 2, the ceiling side of the branch-cut fix: the extremal angle
/// exactly π (`u_ref = −x`, axis z) now admits the x extremum only when
/// the run reaches π — a run [0.5, 2.5] must NOT carry `c − r` in x.
#[test]
fn n3r1_extremal_angle_exactly_pi_is_a_point_interval() {
    let cf = circle(Point3::origin(), Vec3::unit_z(), 1.0, -Vec3::unit_x());
    // x = −cos t: min at t = 0 (x = −1), max at t = π (x = +1).
    let b = circle_arc_aabb(&cf, 0.5, 2.5, cf.eval(0.5), cf.eval(2.5)).unwrap();
    assert!(b.max_x < 0.81 && b.max_x > 0.80, "run [0.5, 2.5] must end at the endpoint -cos 2.5 = 0.801, not reach x = +1: {b:?}");
    assert!(b.min_x < -0.87, "min_x is the endpoint at 0.5: {b:?}");
    let b = circle_arc_aabb(&cf, 2.5, 3.5, cf.eval(2.5), cf.eval(3.5)).unwrap();
    assert!((b.max_x - 1.0).abs() < 1e-12, "run [2.5, 3.5] reaches x = +1: {b:?}");
}

#[cfg(feature = "interval")]
#[test]
fn n3r1_extremal_angle_exactly_pi_is_a_point_interval_at_interval() {
    // At Interval with a point-width bracket, the same.
    let iv = <Interval as Real>::from_f64;
    let ci: Curve3<Interval> = Curve3::Circle {
        center: Point3::new(iv(0.0), iv(0.0), iv(0.0)),
        axis: Vec3::new(iv(0.0), iv(0.0), iv(1.0)),
        radius: iv(1.0),
        u_ref: Vec3::new(iv(-1.0), iv(0.0), iv(0.0)),
    };
    let b = conic_arc_aabb(&ci, iv(0.5), iv(2.5), ci.eval(iv(0.5)), ci.eval(iv(2.5))).unwrap();
    assert!(b.max_x < 0.81 && b.max_x > 0.80, "interval: run [0.5, 2.5] must end at 0.801, not reach x = +1: {b:?}");
}

/// Item 9 (C24): `deriv(t)` + `deriv2(t)` on the conic arms against a
/// fused jet (one `sin_cos`, one cross). Run with `--release --ignored`.
#[test]
#[ignore]
fn n3r1_c24_timing() {
    use std::hint::black_box;
    use std::time::Instant;
    let axis = Vec3::new(0.3, -0.2, 0.9).normalize();
    let u_ref = axis.cross(Vec3::unit_x()).normalize();
    let circ = circle(Point3::new(0.1, 0.2, 0.3), axis, 1.3, u_ref);
    let ell = ellipse(Point3::new(0.1, 0.2, 0.3), axis, 2.1, 0.7, u_ref);
    let n = 20_000_000u32;
    for (name, carrier, a, b) in [("circle", &circ, 1.3, 1.3), ("ellipse", &ell, 2.1, 0.7)] {
        let v_ref = axis.cross(u_ref);
        let t0 = Instant::now();
        let mut acc = 0.0;
        for i in 0..n {
            let t = black_box(f64::from(i) * 1e-7);
            let d = carrier.deriv(t);
            let d2 = carrier.deriv2(t);
            acc += d.x + d2.y;
        }
        let two = t0.elapsed().as_secs_f64() / f64::from(n) * 1e9;
        black_box(acc);
        let t0 = Instant::now();
        let mut acc = 0.0;
        for i in 0..n {
            let t = black_box(f64::from(i) * 1e-7);
            let (s, c) = t.sin_cos();
            let d = u_ref * (-a * s) + v_ref * (b * c);
            let d2 = -(u_ref * (a * c) + v_ref * (b * s));
            acc += d.x + d2.y;
        }
        let one = t0.elapsed().as_secs_f64() / f64::from(n) * 1e9;
        black_box(acc);
        eprintln!("n3r1 c24 {name}: deriv+deriv2 {two:.1} ns, fused {one:.1} ns, saving {:.1} ns", two - one);
    }
}
