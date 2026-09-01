//! CERT-8 review probes (reviewer lane 8r2). Not for merge.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::Tol;
use geom_core::{Affine3, Band, Point2, Vec3};
use profile::{ProfileLoop, ProfileVertex, RawLoop};
use topo::{Body, FaceKey};

fn band() -> Band {
    Band::new(1e-9, 1e-8).unwrap()
}

fn nurbs_wall(body: &Body<f64>) -> FaceKey {
    body.faces()
        .find(|(_, face)| {
            matches!(
                body.get_surface(face.surface),
                Some(Surface::Nurbs(payload)) if !payload.is_placeholder()
            )
        })
        .map(|(key, _)| key)
        .expect("a loft has described NURBS walls")
}

fn report(tag: &str, s: &Surface<f64>) {
    let i = geom_brep::chart_stretch_inf(s);
    let (su, sv) = geom_brep::chart_stretch_sup(s);
    let t = (i.sup_u / i.inf_u).powi(2) + (i.sup_v / i.inf_v).powi(2);
    let d = (i.area_inf / (i.inf_u * i.inf_v)).powi(2);
    let root = (t * t - 4.0 * d).max(0.0).sqrt();
    let rho = (2.0 * d / (t + root)).sqrt().min(1.0);
    println!(
        "{tag}: inf=({:.6}, {:.6}) sup=({:.6}, {:.6}) [chart_stretch_sup=({:.6},{:.6})] area_inf={:.6} T={:.6} D={:.6} rho={:.6} arms=({:.6},{:.6})",
        i.inf_u, i.inf_v, i.sup_u, i.sup_v, su, sv, i.area_inf, t, d, rho, i.inf_u * rho, i.inf_v * rho
    );
    // mean width of the unit square scaled by the arms
    let (au, av) = (i.inf_u * rho, i.inf_v * rho);
    println!("{tag}: unit-square mean width 2A/P = {:.6}", 2.0 * au * av / (2.0 * (au + av)));
}

#[test]
fn probe_loft_wall_digits() {
    let v = |x: f64, y: f64| ProfileVertex::new(Point2::new(x, y), 0.0);
    let square = || {
        vec![ProfileLoop::new(vec![
            v(-1.0, -1.0),
            v(1.0, -1.0),
            v(1.0, 1.0),
            v(-1.0, 1.0),
        ])]
    };
    let sections = vec![square(), square(), square()];
    let places = vec![
        Affine3::identity(),
        Affine3::translation(Vec3::new(0.5, 0.0, 1.0)),
        Affine3::translation(Vec3::new(0.0, 0.0, 2.0)),
    ];
    let body = sweep::loft_body::<f64>(&sections, &places, 2, Tol::witness())
        .expect("builds")
        .body;
    let wall = nurbs_wall(&body);
    let s = body
        .get_surface(body.get_face(wall).unwrap().surface)
        .unwrap()
        .clone();
    report("BOWED-ISOLINE", &s);
    let _ = band();
}

#[test]
fn probe_rational_wall_digits() {
    let v = |x: f64, y: f64, bulge: f64| ProfileVertex::new(Point2::new(x, y), bulge);
    let bulged = || {
        vec![ProfileLoop::new(vec![
            v(0.0, 0.0, 0.0),
            v(2.0, 0.0, 0.4),
            v(2.0, 2.0, 0.0),
            v(0.0, 2.0, 0.0),
        ])]
    };
    let sections = vec![bulged(), bulged()];
    let places = vec![
        Affine3::identity(),
        Affine3::translation(Vec3::new(0.0, 0.0, 1.0)),
    ];
    let body = sweep::loft_body::<f64>(&sections, &places, 1, Tol::witness())
        .expect("builds")
        .body;
    for (key, face) in body.faces() {
        if let Some(Surface::Nurbs(p)) = body.get_surface(face.surface) {
            if p.is_placeholder() {
                continue;
            }
            let w = p.weights();
            let rational = w.iter().any(|x| (x - w[0]).abs() > 1e-15);
            let s = body.get_surface(face.surface).unwrap().clone();
            report(
                if rational { "RATIONAL-WALL" } else { "poly-wall" },
                &s,
            );
            let _ = key;
        }
    }
}

/// **Sampled falsification of the certified inf/arm claims.** Evaluate
/// the real surface on a grid, finite-difference the partials, and
/// check every claim `chart_stretch_inf` + the `certified_arms`
/// assembly makes: `inf_u <= |S_u|`, `inf_v <= |S_v|`,
/// `area_inf <= |S_u x S_v|`, and the metric contraction
/// `|J w| >= |(arm_u du, arm_v dv)|` for arbitrary chart directions.
fn sample_check(tag: &str, s: &Surface<f64>) {
    let Surface::Nurbs(p) = s else { return };
    let i = geom_brep::chart_stretch_inf(s);
    let t = (i.sup_u / i.inf_u).powi(2) + (i.sup_v / i.inf_v).powi(2);
    let d = (i.area_inf / (i.inf_u * i.inf_v)).powi(2);
    let root = (t * t - 4.0 * d).max(0.0).sqrt();
    let rho = (2.0 * d / (t + root)).sqrt().min(1.0);
    let (au, av) = (i.inf_u * rho, i.inf_v * rho);
    let (u0, u1) = p.knots_u().domain();
    let (v0, v1) = p.knots_v().domain();
    let h = 1e-6;
    let (mut min_su, mut min_sv, mut min_area, mut worst_ratio) =
        (f64::MAX, f64::MAX, f64::MAX, f64::MAX);
    let (mut max_su, mut max_sv) = (0.0_f64, 0.0_f64);
    let n = 60;
    for a in 1..n {
        for b in 1..n {
            let u = u0 + (u1 - u0) * (a as f64) / (n as f64);
            let v = v0 + (v1 - v0) * (b as f64) / (n as f64);
            let su = (p.eval(u + h, v) - p.eval(u - h, v)) * (1.0 / (2.0 * h));
            let sv = (p.eval(u, v + h) - p.eval(u, v - h)) * (1.0 / (2.0 * h));
            min_su = min_su.min(su.norm());
            min_sv = min_sv.min(sv.norm());
            max_su = max_su.max(su.norm());
            max_sv = max_sv.max(sv.norm());
            min_area = min_area.min(su.cross(sv).norm());
            for k in 0..24 {
                let th = core::f64::consts::TAU * (k as f64) / 24.0;
                let (du, dv) = (th.cos(), th.sin());
                let model = (su * du + sv * dv).norm();
                let scaled = ((au * du).powi(2) + (av * dv).powi(2)).sqrt();
                worst_ratio = worst_ratio.min(model / scaled);
            }
        }
    }
    println!(
        "{tag} SAMPLED: |S_u| in [{min_su:.6},{max_su:.6}] vs inf {:.6}/sup {:.6}; \
         |S_v| in [{min_sv:.6},{max_sv:.6}] vs inf {:.6}/sup {:.6}; \
         |SuxSv| min {min_area:.6} vs area_inf {:.6}; \
         arms=({au:.6},{av:.6}) worst |Jw|/|scaled w| = {worst_ratio:.6}",
        i.inf_u, i.sup_u, i.inf_v, i.sup_v, i.area_inf
    );
    assert!(min_su >= i.inf_u - 1e-6, "{tag}: inf_u is NOT a lower bound");
    assert!(min_sv >= i.inf_v - 1e-6, "{tag}: inf_v is NOT a lower bound");
    assert!(max_su <= i.sup_u + 1e-6, "{tag}: sup_u is NOT an upper bound");
    assert!(max_sv <= i.sup_v + 1e-6, "{tag}: sup_v is NOT an upper bound");
    assert!(
        min_area >= i.area_inf - 1e-6,
        "{tag}: area_inf is NOT a lower bound"
    );
    assert!(
        worst_ratio >= 1.0 - 1e-9,
        "{tag}: the arms are NOT a metric contraction (ratio {worst_ratio})"
    );
}

#[test]
fn probe_sampled_bounds_hold_on_the_acceptance_walls() {
    let v = |x: f64, y: f64, bulge: f64| ProfileVertex::new(Point2::new(x, y), bulge);
    let bulged = || {
        vec![ProfileLoop::new(vec![
            v(0.0, 0.0, 0.0),
            v(2.0, 0.0, 0.4),
            v(2.0, 2.0, 0.0),
            v(0.0, 2.0, 0.0),
        ])]
    };
    let body = sweep::loft_body::<f64>(
        &[bulged(), bulged()],
        &[
            Affine3::identity(),
            Affine3::translation(Vec3::new(0.0, 0.0, 1.0)),
        ],
        1,
        Tol::witness(),
    )
    .expect("builds")
    .body;
    for (_, face) in body.faces() {
        if let Some(s @ Surface::Nurbs(p)) = body.get_surface(face.surface) {
            if p.is_placeholder() {
                continue;
            }
            let w = p.weights();
            let rational = w.iter().any(|x| (x - w[0]).abs() > 1e-15);
            sample_check(if rational { "RATIONAL" } else { "poly" }, s);
        }
    }
    // The bowed IsoLine wall too.
    let sq = |x: f64, y: f64| ProfileVertex::new(Point2::new(x, y), 0.0);
    let square = || {
        vec![ProfileLoop::new(vec![
            sq(-1.0, -1.0),
            sq(1.0, -1.0),
            sq(1.0, 1.0),
            sq(-1.0, 1.0),
        ])]
    };
    let body2 = sweep::loft_body::<f64>(
        &[square(), square(), square()],
        &[
            Affine3::identity(),
            Affine3::translation(Vec3::new(0.5, 0.0, 1.0)),
            Affine3::translation(Vec3::new(0.0, 0.0, 2.0)),
        ],
        2,
        Tol::witness(),
    )
    .expect("builds")
    .body;
    let wall = nurbs_wall(&body2);
    let s = body2
        .get_surface(body2.get_face(wall).unwrap().surface)
        .unwrap();
    sample_check("BOWED", s);
}

/// Deterministic LCG, so the hunt below is reproducible.
struct Lcg(u64);
impl Lcg {
    fn next_f64(&mut self, lo: f64, hi: f64) -> f64 {
        self.0 = self.0.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
        lo + (hi - lo) * (((self.0 >> 11) as f64) / ((1u64 << 53) as f64))
    }
}

/// **Randomised soundness hunt for the inf-arm assembly.** Build
/// random (rational and polynomial) charts, run the exact
/// `chart_stretch_inf` + `certified_arms` assembly, and sample the
/// real surface for a violation of any of the four claims.
#[test]
fn probe_random_charts_never_break_the_certified_arms() {
    use geom::NurbsSurface;
    use geom_core::spline::KnotVector;
    use geom_core::Point3;
    let mut r = Lcg(0x9E37_79B9_7F4A_7C15);
    let mut checked = 0usize;
    let mut worst_overall = f64::MAX;
    let mut worst_case = String::new();
    for trial in 0..4000 {
        let rational = trial % 2 == 0;
        let (pu, nu) = (2usize, 3usize);
        let (pv, nv) = (2usize, 3usize);
        let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], pu).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], pv).unwrap();
        let mut ctl = Vec::new();
        for i in 0..nu {
            for j in 0..nv {
                ctl.push(Point3::new(
                    i as f64 + r.next_f64(-0.4, 0.4),
                    j as f64 + r.next_f64(-0.4, 0.4),
                    r.next_f64(-0.6, 0.6),
                ));
            }
        }
        let w: Vec<f64> = (0..nu * nv)
            .map(|_| if rational { r.next_f64(0.3, 3.0) } else { 1.0 })
            .collect();
        let Ok(surf) = NurbsSurface::new(ku, kv, ctl, w) else {
            continue;
        };
        let s: Surface<f64> = Surface::Nurbs(std::sync::Arc::new(surf));
        let i = geom_brep::chart_stretch_inf(&s);
        if !(i.inf_u > 1e-6 && i.inf_v > 1e-6) {
            continue;
        }
        let t = (i.sup_u / i.inf_u).powi(2) + (i.sup_v / i.inf_v).powi(2);
        let d = (i.area_inf / (i.inf_u * i.inf_v)).powi(2);
        let root = (t * t - 4.0 * d).max(0.0).sqrt();
        let rho = (2.0 * d / (t + root)).sqrt().min(1.0);
        if !(rho > 1e-12) {
            continue;
        }
        let (au, av) = (i.inf_u * rho, i.inf_v * rho);
        let Surface::Nurbs(p) = &s else { unreachable!() };
        let h = 1e-6;
        let n = 25;
        checked += 1;
        for a in 1..n {
            for b in 1..n {
                let (u, v) = ((a as f64) / (n as f64), (b as f64) / (n as f64));
                let su = (p.eval(u + h, v) - p.eval(u - h, v)) * (1.0 / (2.0 * h));
                let sv = (p.eval(u, v + h) - p.eval(u, v - h)) * (1.0 / (2.0 * h));
                assert!(
                    su.norm() >= i.inf_u - 1e-5,
                    "trial {trial}: inf_u {} > |S_u| {} at ({u},{v})",
                    i.inf_u,
                    su.norm()
                );
                assert!(
                    sv.norm() >= i.inf_v - 1e-5,
                    "trial {trial}: inf_v {} > |S_v| {}",
                    i.inf_v,
                    sv.norm()
                );
                assert!(
                    su.cross(sv).norm() >= i.area_inf - 1e-5,
                    "trial {trial}: area_inf {} > |SuxSv| {}",
                    i.area_inf,
                    su.cross(sv).norm()
                );
                for k in 0..32 {
                    let th = core::f64::consts::TAU * (k as f64) / 32.0;
                    let (du, dv) = (th.cos(), th.sin());
                    let model = (su * du + sv * dv).norm();
                    let scaled = ((au * du).powi(2) + (av * dv).powi(2)).sqrt();
                    let ratio = model / scaled;
                    if ratio < worst_overall {
                        worst_overall = ratio;
                        worst_case = format!("trial {trial} rational={rational} at ({u},{v})");
                    }
                    assert!(
                        ratio >= 1.0 - 1e-7,
                        "trial {trial} rational={rational}: NOT a contraction, \
                         ratio {ratio} at ({u},{v}) arms ({au},{av})"
                    );
                }
            }
        }
    }
    println!("checked {checked} random charts; worst contraction ratio {worst_overall} at {worst_case}");
    assert!(checked > 200, "the hunt must actually build charts, got {checked}");
}
