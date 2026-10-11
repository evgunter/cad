//! Second-pass verifier probes for PR 4696's fix pass (e4f977bd).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use crate::review_4696_r2::Cone;
use crate::shared::tol::band;
use crate::shared::topo;
use geom::Curve3;
use geom_brep::props::{
    LoopEdge, PropsError, boundary_material_sign_loops, curved_face_loops,
};
use geom_core::{Point3, Vec3};

const PI: f64 = core::f64::consts::PI;
const TAU: f64 = core::f64::consts::TAU;
type V = [f64; 3];
fn add(a: V, b: V, s: f64) -> V {
    [a[0] + s * b[0], a[1] + s * b[1], a[2] + s * b[2]]
}
fn p(a: V) -> Point3<f64> {
    Point3::new(a[0], a[1], a[2])
}
fn v(a: V) -> Vec3<f64> {
    Vec3::new(a[0], a[1], a[2])
}
fn nrm(a: V) -> V {
    let n = (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt();
    [a[0] / n, a[1] / n, a[2] / n]
}

fn run(c: &Cone, loops: &[Vec<LoopEdge<f64>>]) -> Result<f64, PropsError> {
    let ls: Vec<&[LoopEdge<f64>]> = loops.iter().map(Vec::as_slice).collect();
    curved_face_loops(&c.surf::<f64>(), &ls, true, band()).map(|f| f.area)
}
fn gate(c: &Cone, loops: &[Vec<LoopEdge<f64>>]) -> Result<geom_brep::props::MaterialSign, PropsError> {
    let ls: Vec<&[LoopEdge<f64>]> = loops.iter().map(Vec::as_slice).collect();
    boundary_material_sign_loops(&c.surf::<f64>(), &ls, band())
}
fn tag(r: &Result<f64, PropsError>) -> String {
    match r {
        Ok(_) => "ok".into(),
        Err(PropsError::OffSurface { what }) => format!("OffSurface({what})"),
        Err(e @ PropsError::Escalated { .. }) => {
            let d = format!("{e:?}");
            let p = d.split("predicate: Some(\"").nth(1).and_then(|x| x.split('"').next()).unwrap_or("?").to_string();
            format!("Escalated({p})")
        }
        Err(e) => format!("{e:?}").chars().take(60).collect(),
    }
}
fn off(r: &Result<f64, PropsError>) -> bool {
    matches!(r, Err(PropsError::OffSurface { .. }))
        || matches!(r, Err(PropsError::Escalated { .. }))
}

const ALPHAS: [f64; 5] = [1e-6, 1e-3, 0.5, PI / 2.0 - 1e-3, PI / 2.0 - 1e-6];

/// Wall u∈[0,1], v∈[v0,v0+1]; edges [rim v0, gen u=1, rim v1, gen u=0].
fn wall(c: &Cone, v0: f64) -> Vec<LoopEdge<f64>> {
    c.chart_loop::<f64>(&[(0.0, v0), (1.0, v0), (1.0, v0 + 1.0), (0.0, v0 + 1.0)])
}

/// The top rim (v = v1) perturbed so its farthest point lies `d` metres
/// off the cone, by each incidence mode; and the u=1 generator likewise.
fn perturbed(c: &Cone, mode: &str, d: f64, v1: f64) -> Vec<LoopEdge<f64>> {
    let (s, co) = c.alpha.sin_cos();
    let mut w = wall(c, v1 - 1.0);
    let r = v1 * s;
    let ctr = add(c.apex, c.a, v1 * co);
    let circ = |center: V, axis: V, radius: f64| {
        topo::edge(
            Curve3::Circle { center: p(center), axis: v(axis), radius, u_ref: v(c.e1) },
            1.0,
            0.0,
            2,
            3,
        )
    };
    match mode {
        // radial: meridian-plane distance δr·cos α
        "rim_fit_radius" => w[2] = circ(ctr, c.a, r + d / co),
        // axial: δh·sin α
        "rim_fit_height" => w[2] = circ(add(ctr, c.a, d / s), c.a, r),
        // off axis by o: a point moves radially by ≤ o, distance ≤ o·cos α
        "center_off_axis" => w[2] = circ(add(ctr, c.e1, d / co), c.a, r),
        // tilt θ: a point moves axially by ≤ r·θ, distance ≤ r·θ·sin α
        "tilt" => {
            let th = d / (r * s);
            w[2] = circ(ctr, nrm(add(c.a, c.e2, th.tan())), r)
        }
        // generator u=1 rotated about the apex in its meridian plane by φ;
        // farthest point (t1 = v1) is v1·sin φ off the cone
        "generator_angle" => {
            let phi = (d / v1).asin();
            let g = c.g(1.0);
            let radial = nrm(add(g, c.a, -co));
            // rotate toward the axis
            let dir = add([0.0; 3], g, phi.cos());
            let perp = nrm(add(c.a, radial, 0.0)); // placeholder
            let _ = perp;
            let nrm_in = add(add([0.0; 3], c.a, s), radial, -co); // unit, ⟂ g in meridian plane
            let dir = add(dir, nrm_in, phi.sin());
            w[1] = topo::edge(Curve3::Line { origin: p(c.apex), dir: v(dir) }, v1 - 1.0, v1, 1, 2);
        }
        // generator u=1 shifted along the cone normal by d (misses the apex by d)
        "generator_offset" => {
            let g = c.g(1.0);
            let radial = nrm(add(g, c.a, -co));
            let n = add(add([0.0; 3], radial, co), c.a, -s);
            w[1] = topo::edge(Curve3::Line { origin: p(add(c.apex, n, d)), dir: v(g) }, v1 - 1.0, v1, 1, 2);
        }
        _ => unreachable!(),
    }
    w
}

/// Every incidence decide, perturbed by a known TRUE distance from the
/// cone, at five half-angles: 0.1·zero must admit as on-surface (it may
/// still fail closure etc. — reported), 100·escalate must refuse.
#[test]
fn incidence_decides_meter_true_distance() {
    let b = band();
    let modes = ["rim_fit_radius", "rim_fit_height", "center_off_axis", "tilt", "generator_angle", "generator_offset"];
    let mut bad = Vec::new();
    for &alpha in &ALPHAS {
        for &v1 in &[2.0, 1001.0] {
            let c = Cone::new([0.2, -0.1, -1.0], [0.3, -0.4, 1.0], alpha);
            let base = run(&c, &[wall(&c, v1 - 1.0)]);
            for m in modes {
                let lo = run(&c, &[perturbed(&c, m, 0.1 * b.zero(), v1)]);
                let hi = run(&c, &[perturbed(&c, m, 100.0 * b.escalate(), v1)]);
                eprintln!("alpha {alpha:.3e} v1 {v1} {m:18} base {:12} | d=0.1z {:45} | d=100esc {}", tag(&base), tag(&lo), tag(&hi));
                if base.is_ok() && off(&lo) {
                    bad.push(format!("over-levered: alpha {alpha} v1 {v1} {m}: {}", tag(&lo)));
                }
                if base.is_ok() && hi.is_ok() {
                    bad.push(format!("admitted off-surface: alpha {alpha} v1 {v1} {m}"));
                }
            }
        }
    }
    for x in &bad {
        eprintln!("BAD {x}");
    }
    assert!(bad.is_empty(), "{} bad", bad.len());
}

/// Parity: the gate refuses whatever the flux lane refuses (spiral,
/// sectors past a turn, figure-eight, stated twice, off-surface edges),
/// and answers every legal face.
#[test]
fn gate_flux_parity() {
    let c = Cone::new([0.2, -0.1, -1.0], [0.0, 0.0, 1.0], 0.5);
    let ul = TAU + 1.0;
    let mut cases: Vec<(String, Vec<Vec<LoopEdge<f64>>>)> = vec![
        ("spiral".into(), vec![c.chart_loop(&[(0.0, 0.0), (0.0, 1.0), (4.0, 1.0), (4.0, 1.5), (ul, 1.5), (ul, 0.0)])]),
        ("figure-eight".into(), vec![c.chart_loop(&[(0.0, 1.0), (1.0, 1.0), (1.0, 2.0), (1.0, 3.0), (2.0, 3.0), (2.0, 2.0), (1.0, 2.0), (0.0, 2.0)])]),
    ];
    for du in [TAU + 0.1, TAU + 2.0, TAU + 2.9] {
        cases.push((format!("sector {du:.2}"), vec![c.chart_loop(&[(0.0, 0.0), (du / 2.0, 0.0), (du, 0.0), (du, 1.5), (du / 2.0, 1.5), (0.0, 1.5)])]));
    }
    let mut twice = wall(&c, 1.0);
    twice.extend(wall(&c, 1.0));
    cases.push(("wall twice".into(), vec![twice]));
    let mut sec2 = c.chart_loop(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.5), (0.0, 1.5)]);
    sec2.extend(c.chart_loop(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.5), (0.0, 1.5)]));
    cases.push(("sector twice".into(), vec![sec2]));
    cases.push(("rim twice".into(), vec![vec![c.rim(1.5, 0.0, TAU, 0, 0), c.rim(1.5, 0.0, TAU, 0, 0)]]));
    for m in ["rim_fit_radius", "center_off_axis", "tilt", "generator_angle", "generator_offset"] {
        cases.push((format!("off-surface {m}"), vec![perturbed(&c, m, 1e-3, 2.0)]));
    }
    let mut bad = Vec::new();
    for (name, l) in &cases {
        let f = run(&c, l);
        let g = gate(&c, l);
        eprintln!("parity {name:24} flux {:50} gate {:?}", tag(&f), g.as_ref().map_err(|e| format!("{e:?}").chars().take(70).collect::<String>()));
        if f.is_err() != g.is_err() || (f.is_err() && f.as_ref().err() != g.as_ref().err()) {
            bad.push(name.clone());
        }
    }
    // legal faces: both answer
    for (name, verts) in [
        ("wall", vec![(0.0, 1.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)]),
        ("sector", vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.5), (0.0, 1.5)]),
        ("slit cap", vec![(0.0, 0.0), (0.0, 1.5), (TAU, 1.5), (TAU, 0.0)]),
        ("wide sector", vec![(0.0, 0.0), (5.9, 0.0), (5.9, 1.5), (0.0, 1.5)]),
    ] {
        for fwd in [true, false] {
            let mut vv = verts.clone();
            if !fwd { vv.reverse(); }
            let l = vec![c.chart_loop(&vv)];
            let (f, g) = (run(&c, &l), gate(&c, &l));
            eprintln!("legal {name} fwd {fwd}: flux {} gate {g:?}", tag(&f));
            if f.is_err() || g.is_err() { bad.push(format!("legal {name} {fwd}")); }
        }
    }
    assert!(bad.is_empty(), "{bad:?}");
}
