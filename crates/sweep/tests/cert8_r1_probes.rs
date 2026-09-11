//! CERT-8 review probes (reviewer lane 8r1; not part of the unit).
//!
//! Executes the PR body's claimed digits and the inf-bound soundness
//! on the unit's own acceptance fixtures: sampled `|S_u|`, `|S_v|`,
//! `|S_u x S_v|` against `chart_stretch_inf`'s certified floors, and
//! the assembled arms against the sampled metric.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::cast_precision_loss
)]

use geom::Surface;
use geom_core::Tol;
use geom_core::{Affine3, Point2, Vec3};
use profile::{ProfileLoop, ProfileVertex, RawLoop};
use topo::{Body, FaceKey};

fn at_z(zs: &[f64]) -> Vec<Affine3<f64>> {
    zs.iter()
        .map(|z| Affine3::translation(Vec3::new(0.0, 0.0, *z)))
        .collect()
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

/// The assembly `certified_arms` performs, reproduced from the PR's
/// published formula so the probe can check it against samples.
fn assemble(inf: &geom_brep::ChartStretchInf<f64>) -> (f64, f64, f64) {
    let (iu, iv) = (inf.inf_u, inf.inf_v);
    let t = (inf.sup_u / iu).powi(2) + (inf.sup_v / iv).powi(2);
    let d = (inf.area_inf / (iu * iv)).powi(2);
    let root = (t.powi(2) - 4.0 * d).max(0.0).sqrt();
    let rho = (2.0 * d / (t + root)).sqrt().min(1.0);
    (iu * rho, iv * rho, rho)
}

/// Sampled floors: min |S_u|, min |S_v|, min |S_u x S_v|, and the
/// worst ratio of sampled metric length to the arms' claimed floor
/// over chart directions (must stay >= 1 for soundness).
fn sample(surface: &Surface<f64>, arm_u: f64, arm_v: f64) -> (f64, f64, f64, f64) {
    let n = 160;
    let (mut mu, mut mv, mut ma, mut worst) = (f64::MAX, f64::MAX, f64::MAX, f64::MAX);
    for i in 0..=n {
        for j in 0..=n {
            let (u, v) = (f64::from(i) / f64::from(n), f64::from(j) / f64::from(n));
            let su = surface.deriv_u(u, v);
            let sv = surface.deriv_v(u, v);
            mu = mu.min(su.norm());
            mv = mv.min(sv.norm());
            ma = ma.min(su.cross(sv).norm());
            for k in 0..8 {
                let th = core::f64::consts::PI * f64::from(k) / 8.0;
                let (du, dv) = (th.cos(), th.sin());
                let model = (su * du + sv * dv).norm();
                let floor = ((arm_u * du).powi(2) + (arm_v * dv).powi(2)).sqrt();
                worst = worst.min(model / floor);
            }
        }
    }
    (mu, mv, ma, worst)
}

fn probe(name: &str, surface: &Surface<f64>) {
    let inf = geom_brep::chart_stretch_inf(surface);
    let sup = geom_brep::chart_stretch_sup(surface);
    let (arm_u, arm_v, rho) = assemble(&inf);
    let mw = 2.0 * (arm_u * arm_v) / (2.0 * (arm_u + arm_v));
    println!(
        "{name}: inf=({:.6},{:.6}) sup=({:.6},{:.6}) area_inf={:.6} rho={:.6} \
         arms=({:.6},{:.6}) unit-square mean width={:.6}",
        inf.inf_u, inf.inf_v, sup.0, sup.1, inf.area_inf, rho, arm_u, arm_v, mw
    );
    let (mu, mv, ma, worst) = sample(surface, arm_u, arm_v);
    println!(
        "{name}: sampled min|S_u|={mu:.6} min|S_v|={mv:.6} min|SuxSv|={ma:.6} \
         worst metric/floor={worst:.6}"
    );
    assert!(
        inf.inf_u <= mu + 1e-9,
        "{name}: inf_u overstates sampled floor"
    );
    assert!(
        inf.inf_v <= mv + 1e-9,
        "{name}: inf_v overstates sampled floor"
    );
    assert!(
        inf.area_inf <= ma + 1e-9,
        "{name}: area_inf overstates sampled floor"
    );
    assert!(
        worst >= 1.0 - 1e-9,
        "{name}: arms overstate the sampled metric"
    );
}

#[test]
fn probe_loft_wall_digits_and_sampled_soundness() {
    // The bowed IsoLine wall, the unit's polynomial acceptance row.
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
        .expect("the offset square prism builds")
        .body;
    let wall = nurbs_wall(&body);
    let sref = body
        .get_surface(body.get_face(wall).unwrap().surface)
        .unwrap();
    probe("iso-line-wall", sref);

    // The bulged IsoArc (rational) wall, the rational acceptance row.
    let vb = |x: f64, y: f64, bulge: f64| ProfileVertex::new(Point2::new(x, y), bulge);
    let bulged = || {
        vec![ProfileLoop::new(vec![
            vb(0.0, 0.0, 0.0),
            vb(2.0, 0.0, 0.4),
            vb(2.0, 2.0, 0.0),
            vb(0.0, 2.0, 0.0),
        ])]
    };
    let sections = vec![bulged(), bulged()];
    let body = sweep::loft_body::<f64>(&sections, &at_z(&[0.0, 1.0]), 1, Tol::witness())
        .expect("the bulged prism builds")
        .body;
    for (_, face) in body.faces() {
        if let Some(s @ Surface::Nurbs(p)) = body.get_surface(face.surface)
            && !p.is_placeholder()
        {
            let inf = geom_brep::chart_stretch_inf(s);
            if (inf.inf_u - 1.0488).abs() < 0.01 {
                probe("iso-arc-wall", s);
            }
        }
    }
}

#[test]
fn probe_swap_chart_assembly_bounds() {
    use geom::NurbsSurface;
    use geom_core::Point3;
    use geom_core::spline::KnotVector;
    use std::sync::Arc;
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(0.25, 0.0, 0.0),
        Point3::new(0.25, 1.0, 0.0),
        Point3::new(4.25, 0.0, 0.0),
        Point3::new(4.25, 1.0, 0.0),
    ];
    let s: Surface<f64> = Surface::Nurbs(Arc::new(
        NurbsSurface::new(ku, kv, control, vec![1.0; 6]).unwrap(),
    ));
    let inf = geom_brep::chart_stretch_inf(&s);
    let (arm_u, _arm_v, rho) = assemble(&inf);
    // The PR-body claim under test: "the assembled arm sits in
    // (0, inf_u] — a swap cannot hide inside the assembly". The
    // sup-swapped assembly lands INSIDE the same pin:
    let swapped_u = inf.sup_u * rho;
    println!(
        "swap-chart: inf_u={} sup_u={} rho={rho} arm_u={arm_u} \
         sup-swapped arm={swapped_u} pin-upper={}",
        inf.inf_u, inf.sup_u, inf.inf_u
    );
    assert!(
        swapped_u <= inf.inf_u,
        "if this fails, the pin would in fact catch the assembly swap"
    );
    probe("swap-chart", &s);
}
