//! Delta-verifier probes for PR 5225's fix pass (geom-brep).
#![allow(clippy::unwrap_used, clippy::panic)]
use core::f64::consts::{FRAC_PI_2, PI, TAU};
use geom::{Curve3, Surface};
use geom_brep::{chart_pcurve_over, BranchMiss, Period, PcurveCache};
use geom_core::{Band, Margin, Point3, Tol, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}
fn win(u: f64) -> bool {
    -1.0 < u && u <= TAU - 1.0
}
fn circle(center: Point3<f64>, axis: Vec3<f64>, radius: f64, u_ref: Vec3<f64>) -> Curve3<f64> {
    Curve3::Circle { center, axis, radius, u_ref }
}
fn villarceau(major: f64, minor: f64, phi: f64, family: f64, psi: f64) -> (Surface<f64>, Curve3<f64>) {
    let (axis, u_ref) = (Vec3::unit_z(), Vec3::unit_x());
    let torus = Surface::Torus { center: Point3::new(0.0, 0.0, 0.0), axis, major_radius: major, minor_radius: minor, u_ref };
    let d = u_ref * phi.cos() + axis.cross(u_ref) * phi.sin();
    let tilt = (minor / major).asin();
    let lean = d.cross(axis) * tilt.cos() + axis * (family * tilt.sin());
    let c = circle(Point3::new(0.0, 0.0, 0.0) + d * minor, d.cross(lean), major, d * psi.cos() + lean * psi.sin());
    (torus, c)
}

/// The image evaluated through the chart lands on the carrier, and the
/// midpoint's channels are in their windows.
fn check(name: &str, surface: &Surface<f64>, c: &Curve3<f64>, a: f64, b: f64, v_angle: bool, bad: &mut Vec<String>) -> Option<geom_core::Point2<f64>> {
    let img = match chart_pcurve_over(c, a, b, surface, band()) {
        Ok(i) => i,
        Err(e) => { bad.push(format!("{name} [{a:.3},{b:.3}]: refused {e:?}")); return None; }
    };
    for s in [0.0, 0.3, 0.5, 0.8, 1.0] {
        let t = a + s * (b - a);
        let q = img.eval(t);
        let p = surface.eval(q.x, q.y);
        if (p - c.eval(t)).norm() > 1e-9 { bad.push(format!("{name} [{a:.3},{b:.3}] at {s}: off the carrier by {:e}", (p - c.eval(t)).norm())); }
    }
    let m = img.eval(0.5 * (a + b));
    if !win(m.x) { bad.push(format!("{name} [{a:.3},{b:.3}]: u {} out of window", m.x)); }
    if v_angle && !win(m.y) { bad.push(format!("{name} [{a:.3},{b:.3}]: v {} out of window", m.y)); }
    if let Err(e) = PcurveCache::certify(img.clone(), a, b, c, surface, band()) { bad.push(format!("{name} [{a:.3},{b:.3}]: certify {e:?}")); }
    Some(m)
}

/// F2: Villarceau circles on many tori, azimuths, families, starts and
/// spans, restated whole periods off AND restated with the start moved
/// (u_ref rotated by δ, span moved by −δ: the same arc). Both channels
/// must agree pointwise.
#[test]
fn g1_villarceau_every_start() {
    let mut bad = Vec::new();
    let mut n = 0;
    for (major, minor) in [(2.0, 0.7), (3.0, 0.5), (1.5, 1.2)] {
        for phi in [0.0, 0.7, 2.0, -2.5] {
            for family in [1.0, -1.0] {
                for (t0, t1) in [(0.4, 1.6), (-0.5, 0.2), (2.5, 4.0), (5.0, 6.0)] {
                    let (torus, base_c) = villarceau(major, minor, phi, family, 0.3);
                    let label = format!("R{major} r{minor} phi{phi} fam{family} [{t0},{t1}]");
                    let Some(m0) = check(&label, &torus, &base_c, t0, t1, true, &mut bad) else { continue };
                    for k in [-7.0, -1.0, 1.0, 4.0] {
                        n += 1;
                        if let Some(m) = check(&format!("{label} k{k}"), &torus, &base_c, t0 + k * TAU, t1 + k * TAU, true, &mut bad) {
                            if (m.x - m0.x).abs() > 1e-9 || (m.y - m0.y).abs() > 1e-9 { bad.push(format!("{label} k{k}: {m:?} vs {m0:?}")); }
                        }
                    }
                    for delta in [0.9, -2.2, 3.1, 5.5] {
                        n += 1;
                        let (_, c) = villarceau(major, minor, phi, family, 0.3 + delta);
                        if let Some(m) = check(&format!("{label} psi+{delta}"), &torus, &c, t0 - delta, t1 - delta, true, &mut bad) {
                            let du = (m.x - m0.x).abs();
                            let dv = (m.y - m0.y).abs();
                            if du > 1e-9 || dv > 1e-9 { bad.push(format!("{label} psi+{delta}: mid {m:?} vs {m0:?}")); }
                        }
                    }
                }
            }
        }
    }
    eprintln!("g1: {n} restatements, {} bad", bad.len());
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// F2: the spiric wall over sub-spans, whole periods off.
#[test]
fn g2_spiric_wall_sub_spans() {
    let (r, rr, d) = (0.09375, 0.0703125, 0.0078125);
    let o = Point3::new(0.0, 0.0, 0.0);
    let c = Curve3::Spiric { center: o, axis: Vec3::unit_y(), u_ref: Vec3::unit_x(), major_radius: r, minor_radius: rr, offset: d };
    let torus = Surface::Torus { center: o, axis: Vec3::unit_y(), major_radius: r, minor_radius: rr, u_ref: Vec3::unit_x() };
    let w = 0.8911225078866529;
    let mut bad = Vec::new();
    for (t0, t1) in [(-w, w), (-w, 0.0), (0.1, w), (-0.3, 0.3)] {
        let label = format!("spiric [{t0:.3},{t1:.3}]");
        let Some(m0) = check(&label, &torus, &c, t0, t1, true, &mut bad) else { continue };
        for k in [-20.0, -3.0, -1.0, 1.0, 2.0, 20.0] {
            if let Some(m) = check(&format!("{label} k{k}"), &torus, &c, t0 + k * TAU, t1 + k * TAU, true, &mut bad) {
                if (m.x - m0.x).abs() > 1e-9 || (m.y - m0.y).abs() > 1e-9 { bad.push(format!("{label} k{k}: {m:?} vs {m0:?}")); }
            }
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// F3: a meridian's span anywhere on its great circle, including through
/// either pole and on the far half; and the same edge restated from the
/// antipodal start.
#[test]
fn g3_sphere_meridian_anywhere() {
    let sphere = Surface::Sphere { center: Point3::new(0.0, 0.0, 0.0), radius: 1.0, axis: Vec3::unit_z(), u_ref: Vec3::unit_x() };
    let c = circle(Point3::new(0.0, 0.0, 0.0), -Vec3::unit_y(), 1.0, Vec3::unit_x());
    let far = circle(Point3::new(0.0, 0.0, 0.0), -Vec3::unit_y(), 1.0, -Vec3::unit_x());
    let mut bad = Vec::new();
    let mut lines = Vec::new();
    for i in 0..48 {
        let mid = -PI + TAU * f64::from(i) / 48.0 + 0.01;
        let (a, b) = (mid - 0.3, mid + 0.3);
        let label = format!("meridian mid {mid:.3}");
        let Some(m0) = check(&label, &sphere, &c, a, b, false, &mut bad) else { continue };
        if !(-PI < m0.y && m0.y <= PI) { bad.push(format!("{label}: v {} out of (−π, π]", m0.y)); }
        for k in [-5.0, 1.0, 9.0] {
            if let Some(m) = check(&format!("{label} k{k}"), &sphere, &c, a + k * TAU, b + k * TAU, false, &mut bad) {
                if (m.x - m0.x).abs() > 1e-9 || (m.y - m0.y).abs() > 1e-9 { bad.push(format!("{label} k{k}: {m:?} vs {m0:?}")); }
            }
        }
        let m1 = check(&format!("{label} from the antipode"), &sphere, &far, a - PI, b - PI, false, &mut bad);
        lines.push(format!("{label}: mid {m0:?}  antipode-start {m1:?}"));
        if let Some(m1) = m1 {
            let valid0 = m0.y.abs() <= FRAC_PI_2;
            let valid1 = m1.y.abs() <= FRAC_PI_2;
            if mid.abs() + 0.3 < FRAC_PI_2 && !(valid0 && valid1) {
                bad.push(format!("{label}: an edge clear of both poles stored past a pole: {m0:?} / {m1:?}"));
            }
        }
    }
    eprintln!("{}", lines.join("\n"));
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// F7: can the `unreachable!` arm's premise (OnMark with no astride
/// pair) occur?
#[test]
fn g4_on_mark_without_astride() {
    let gap = (f64::from(i32::MAX) + 0.5) * TAU;
    let at = Period::new(TAU).decide("probe", gap, |g| Margin::levered(g, 1.0), band());
    eprintln!("decide at the i32::MAX + ½ mark: {at:?}");
    assert!(!matches!(at, Err(ref s) if matches!(s.miss, BranchMiss::OnMark) && s.astride.is_none()), "{at:?}");
}

/// F6: levers of zero: a cone ruling whose span is centred on the apex.
#[test]
fn g5_cone_ruling_through_the_apex() {
    let cone = Surface::Cone { apex: Point3::new(0.0, 0.0, 0.0), axis: Vec3::unit_z(), half_angle: 0.4, u_ref: Vec3::unit_x() };
    let dir = Vec3::new(0.4_f64.sin(), 0.0, 0.4_f64.cos());
    let line = Curve3::Line { origin: Point3::new(0.0, 0.0, 0.0), dir };
    let mut out = Vec::new();
    for (a, b) in [(0.5, 1.5), (-1.5, -0.5), (0.0, 1.0), (-1.0, 0.0), (-1.0, 1.0)] {
        let r = chart_pcurve_over(&line, a, b, &cone, band());
        out.push(format!("[{a},{b}]: {:?}", r.map(|i| i.eval(0.5 * (a + b)))));
    }
    eprintln!("{}", out.join("\n"));
}

/// F7: reach the `unreachable!` through `chart_pcurve_over`: a cylinder
/// rim whose span's midpoint puts the placement's gap on the mark
/// `i32::MAX + ½` turns up.
#[test]
fn g6_unreachable_through_the_public_door() {
    let cyl = Surface::Cylinder { origin: Point3::new(0.0, 0.0, 0.0), axis: Vec3::unit_z(), radius: 1.0, u_ref: Vec3::unit_x() };
    let rim = circle(Point3::new(0.0, 0.0, 0.0), Vec3::unit_z(), 1.0, Vec3::unit_x());
    let centre = PI + (-1.0);
    let target = (f64::from(i32::MAX) + 0.5) * TAU + centre;
    let mut m = target;
    for _ in 0..64 { m = f64::from_bits(m.to_bits() - 1); }
    let mut hits = Vec::new();
    for _ in 0..128 {
        let gap = (0.0 + m) - centre;
        let on = gap - (f64::from(i32::MAX) + 0.5) * TAU == 0.0;
        let r = std::panic::catch_unwind(|| chart_pcurve_over(&rim, m, m, &cyl, band()).map(|_| ()));
        if on || r.is_err() { hits.push(format!("m {m:e} on-mark {on} panicked {}", r.is_err())); }
        m = f64::from_bits(m.to_bits() + 1);
    }
    eprintln!("{}", hits.join("\n"));
    assert!(!hits.iter().any(|h| h.contains("panicked true")), "{}", hits.join("\n"));
}

/// G1: g5's reachers at Interval: does Interval name f64's copy?
#[test]
fn g5i_cone_ruling_through_the_apex_both_scalars() {
    use geom_core::{Bounds, Interval, Real};
    let f = Interval::from_f64;
    let cone = Surface::Cone { apex: Point3::new(0.0, 0.0, 0.0), axis: Vec3::unit_z(), half_angle: 0.4, u_ref: Vec3::unit_x() };
    let mut bad = Vec::new();
    for phi in [0.0_f64, 2.3, -1.0, 5.0] {
        let dir = Vec3::new(0.4_f64.sin() * phi.cos(), 0.4_f64.sin() * phi.sin(), 0.4_f64.cos());
        for origin_t in [0.0_f64, 0.7] {
            let line = Curve3::Line { origin: Point3::new(0.0, 0.0, 0.0) + dir * origin_t, dir };
            for (a, b) in [(0.5, 1.5), (-1.5, -0.5), (-1.0, 1.0), (-0.7, 1.0)] {
                let (a, b) = (a - origin_t, b - origin_t);
                let r64 = chart_pcurve_over(&line, a, b, &cone, band());
                let ri = chart_pcurve_over(&line.map_scalar(f), f(a), f(b), &cone.map_scalar(f), band());
                let m = 0.5 * (a + b);
                match (&r64, &ri) {
                    (Ok(p), Ok(q)) => {
                        let (pm, qm) = (p.eval(m), q.eval(f(m)));
                        if !(qm.x.lo() <= pm.x && pm.x <= qm.x.hi()) {
                            bad.push(format!("phi {phi} o {origin_t} [{a},{b}]: f64 {pm:?} vs Interval {qm:?}"));
                        }
                    }
                    (Err(_), Err(_)) => {}
                    _ => bad.push(format!("phi {phi} o {origin_t} [{a},{b}]: f64 {:?} vs Interval {:?}", r64.as_ref().map(|p| p.eval(m)), ri.as_ref().map(|_| ()))),
                }
            }
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// G1: a zero-lever edge (a pole's zero-radius parallel, a cone apex's)
/// restated whole periods off: is it placed in the window, or kept as read?
#[test]
fn g7_zero_lever_edge_restated_periods_off() {
    let sphere = Surface::Sphere { center: Point3::new(0.0, 0.0, 0.0), radius: 1.0, axis: Vec3::unit_z(), u_ref: Vec3::unit_x() };
    let cone = Surface::Cone { apex: Point3::new(0.0, 0.0, 0.0), axis: Vec3::unit_z(), half_angle: 0.4, u_ref: Vec3::unit_x() };
    let pole = circle(Point3::new(0.0, 0.0, 1.0), Vec3::unit_z(), 0.0, Vec3::unit_x());
    let apex = circle(Point3::new(0.0, 0.0, 0.0), Vec3::unit_z(), 0.0, Vec3::unit_x());
    let mut lines = Vec::new();
    let mut bad = Vec::new();
    for (name, s, c) in [("sphere pole", &sphere, &pole), ("cone apex", &cone, &apex)] {
        for k in [0.0, 1.0, -3.0, 10.0] {
            for (t0, t1) in [(0.0, TAU), (0.3, 1.3)] {
                let (a, b) = (t0 + k * TAU, t1 + k * TAU);
                let r = chart_pcurve_over(c, a, b, s, band());
                let m = r.as_ref().map(|i| i.eval(0.5 * (a + b)));
                lines.push(format!("{name} k {k} [{t0},{t1}]: {m:?}"));
                if let Ok(m) = m { if !win(m.x) { bad.push(format!("{name} k {k} [{t0},{t1}]: u {} out of window", m.x)); } }
            }
        }
    }
    eprintln!("{}", lines.join("\n"));
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// G2: a meridian whose midpoint is on (or within band of) its pole,
/// started near and started far: same image, or twins?
#[test]
fn g8_meridian_midpoint_at_the_pole_restarted() {
    let sphere = Surface::Sphere { center: Point3::new(0.0, 0.0, 0.0), radius: 1.0, axis: Vec3::unit_z(), u_ref: Vec3::unit_x() };
    let near = circle(Point3::new(0.0, 0.0, 0.0), -Vec3::unit_y(), 1.0, Vec3::unit_x());
    let far = circle(Point3::new(0.0, 0.0, 0.0), -Vec3::unit_y(), 1.0, -Vec3::unit_x());
    let mut lines = Vec::new();
    for off in [0.0, 1e-13, 1e-10, 1e-8, 1e-6, 1e-3] {
        for pole in [FRAC_PI_2, -FRAC_PI_2] {
            let mid = pole + off;
            let (a, b) = (mid - 0.3, mid + 0.3);
            let p = chart_pcurve_over(&near, a, b, &sphere, band()).map(|i| i.eval(mid));
            let q = chart_pcurve_over(&far, a - PI, b - PI, &sphere, band()).map(|i| i.eval(mid - PI));
            let same = matches!((&p, &q), (Ok(p), Ok(q)) if (p.x - q.x).abs() < 1e-9 && (p.y - q.y).abs() < 1e-9);
            lines.push(format!("pole {pole:.3} off {off:e}: near {p:?} far {q:?} same {same}"));
        }
    }
    eprintln!("{}", lines.join("\n"));
}
