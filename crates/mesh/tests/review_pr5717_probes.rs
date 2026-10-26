//! REVIEW PROBES for PR 5717 (`[mesh.sizing]`). Adversarial faces for
//! the strip and cap lemmas, the cone's ρ_max, and the rounding edge,
//! each meshed through the public door and measured by dense sampling
//! against the true surface. `--nocapture` prints one line per row.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::*;
use core::f64::consts::PI;
use geom_core::{Point2, Tol};
use profile::{ProfileLoop, RawLoop};
use profile::test_support::bulge_loop;
use sweep::Revolution;
use topo::Body;

/// Worst dense-sampled deviation over δ, or the refusal.
fn probe(body: &Body<f64>, delta: f64) -> Result<(f64, usize), String> {
    let mesh = mesh::tessellate(body, delta, Tol::witness()).map_err(|e| format!("{e:?}"))?;
    mesh::validate::check_mesh(&mesh).map_err(|e| format!("check_mesh {e:?}"))?;
    let mut worst = 0.0_f64;
    let mut tris = 0;
    for patch in &mesh.patches {
        let face = body.get_face(patch.face).unwrap();
        let surface = body.get_surface(face.surface).unwrap();
        if matches!(surface, geom::Surface::Plane { .. }) {
            continue;
        }
        for t in &patch.triangles {
            tris += 1;
            let tri = [
                mesh.positions[t[0] as usize],
                mesh.positions[t[1] as usize],
                mesh.positions[t[2] as usize],
            ];
            worst = worst.max(sampled_deviation(surface, tri, 24));
        }
    }
    Ok((worst / delta, tris))
}

fn revolve(lp: ProfileLoop<f64>, rev: Revolution<f64>) -> Option<Body<f64>> {
    sweep::revolve(&validated(vec![lp]), axis_y(), rev, Tol::witness())
        .ok()
        .map(|r| r.body)
}

/// A unit-sphere zone between latitudes `a < b` (radians).
fn zone(r: f64, a: f64, b: f64, rev: Revolution<f64>) -> Option<Body<f64>> {
    let p = |t: f64| Point2::new(r * t.cos(), r * t.sin());
    let bulge = ((b - a) / 4.0).tan();
    let mut chain = Vec::new();
    if a > -PI / 2.0 + 1e-12 {
        chain.push((Point2::new(0.0, r * a.sin()), 0.0));
    }
    chain.push((p(a), bulge));
    chain.push((p(b), 0.0));
    if b < PI / 2.0 - 1e-12 {
        chain.push((Point2::new(0.0, r * b.sin()), 0.0));
    }
    revolve(bulge_loop(chain), rev)
}

/// A frustum between radii `r0` (y = 0) and `r1` (y = h).
fn frustum(r0: f64, r1: f64, h: f64, rev: Revolution<f64>) -> Option<Body<f64>> {
    let mut pts = vec![Point2::new(0.0, 0.0)];
    if r0 > 0.0 {
        pts.push(Point2::new(r0, 0.0));
    }
    if r1 > 0.0 {
        pts.push(Point2::new(r1, h));
    }
    pts.push(Point2::new(0.0, h));
    revolve(ProfileLoop::polygon(pts), rev)
}

/// A cylinder r = 1, height 1, capped at the top by a dome of sphere
/// radius `big` (uneven rims: the dome's grid step tightens the top
/// rim, the planar bottom leaves the cylinder's own).
fn domed_cylinder(big: f64, rev: Revolution<f64>) -> Option<Body<f64>> {
    let rise = big - (big * big - 1.0).sqrt();
    let half = (1.0 / big).asin(); // half the dome's arc angle
    let bulge = (half / 4.0).tan();
    let chain = vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(1.0, 0.0), 0.0),
        (Point2::new(1.0, 1.0), bulge),
        (Point2::new(0.0, 1.0 + rise), 0.0),
    ];
    revolve(bulge_loop(chain), rev)
}

struct Tally {
    rows: usize,
    refused: Vec<String>,
    over: Vec<String>,
    worst: f64,
}

impl Tally {
    fn new() -> Self {
        Self {
            rows: 0,
            refused: Vec::new(),
            over: Vec::new(),
            worst: 0.0,
        }
    }
    fn row(&mut self, name: &str, body: Option<Body<f64>>, delta: f64) {
        let Some(body) = body else {
            println!("{name} δ {delta:e}: NO BODY");
            return;
        };
        self.rows += 1;
        match probe(&body, delta) {
            Ok((ratio, tris)) => {
                self.worst = self.worst.max(ratio);
                if ratio > 1.0 + 1e-6 {
                    self.over.push(format!("{name} δ {delta:e}: {ratio:.6}"));
                }
                println!("{name} δ {delta:e}: worst {ratio:.6} δ over {tris} curved tris");
            }
            Err(e) => {
                println!("{name} δ {delta:e}: REFUSED {e}");
                self.refused.push(format!("{name} δ {delta:e}: {e}"));
            }
        }
    }
    fn report(&self, what: &str) {
        println!(
            "== {what}: {} rows, worst {:.6} δ, {} refused, {} over δ",
            self.rows,
            self.worst,
            self.refused.len(),
            self.over.len()
        );
        for r in &self.refused {
            println!("   refused: {r}");
        }
        for r in &self.over {
            println!("   OVER: {r}");
        }
    }
}

const DELTAS: [f64; 7] = [0.1, 0.05, 0.02, 0.01, 0.005, 0.002, 0.001];

#[test]
fn review_sphere_zones_and_wedges() {
    let mut t = Tally::new();
    let bands: [(f64, f64); 7] = [
        (-0.5, 0.5),
        (0.2, 1.3),
        (0.9, 1.5),
        (1.2, PI / 2.0),
        (-PI / 2.0, -1.0),
        (0.0, 0.05),
        (1.45, 1.55),
    ];
    for (a, b) in bands {
        for rev in [
            Revolution::Full,
            Revolution::Partial(0.3),
            Revolution::Partial(1.0),
            Revolution::Partial(PI / 2.0),
            Revolution::Partial(0.05),
        ] {
            for d in DELTAS {
                t.row(&format!("zone [{a:.3},{b:.3}] {rev:?}"), zone(1.0, a, b, rev), d);
            }
        }
    }
    for theta in [0.02, 0.1, 0.4, 1.0, 2.0, 3.0] {
        for d in DELTAS {
            t.row(&format!("sphere_wedge {theta}"), Some(sphere_wedge(theta)), d);
        }
    }
    t.report("sphere");
}

#[test]
fn review_cone_frusta() {
    let mut t = Tally::new();
    let shapes: [(f64, f64, f64); 8] = [
        (1.5, 0.05, 1.7),
        (0.05, 1.5, 1.7),
        (1.0, 0.0, 1.0),
        (1.0, 0.0, 0.05),
        (1.0, 0.0, 20.0),
        (2.0, 1.9, 0.02),
        (0.3, 3.0, 0.4),
        (1.0, 0.98, 5.0),
    ];
    for (r0, r1, h) in shapes {
        for rev in [
            Revolution::Full,
            Revolution::Partial(0.2),
            Revolution::Partial(PI / 6.0),
            Revolution::Partial(2.5),
        ] {
            for d in DELTAS {
                t.row(
                    &format!("frustum {r0}->{r1} h {h} {rev:?}"),
                    frustum(r0, r1, h, rev),
                    d,
                );
            }
        }
    }
    t.report("cone");
}

#[test]
fn review_uneven_cylinder_rims() {
    let mut t = Tally::new();
    for big in [1.05, 1.5, 3.0, 10.0] {
        for rev in [
            Revolution::Full,
            Revolution::Partial(0.4),
            Revolution::Partial(2.0),
        ] {
            for d in DELTAS {
                t.row(&format!("domed big {big} {rev:?}"), domed_cylinder(big, rev), d);
            }
        }
    }
    t.report("uneven cylinder");
}

/// The rounding edge: δ chosen so a full rim's chord count is exactly
/// `n` (a user asking for an n-gon: δ = r·(1 − cos(π/n))), and the
/// cone's / sphere's analogues.
#[test]
fn review_rounding_edge() {
    let mut t = Tally::new();
    for r in [1.0_f64, 0.5, 2.0, 0.1, 3.0] {
        for n in (9..=400).step_by(1) {
            #[allow(clippy::cast_precision_loss)]
            let d = r * (1.0 - (PI / n as f64).cos());
            t.row(&format!("cyl r {r} n {n}"), frustum(r, r, 1.0, Revolution::Full), d);
        }
    }
    // Cone: δ = ρ_max·cos α·(1 − cos(π/n)), radii 1 → 0.5 over height 1.
    let alpha = (0.5_f64).atan();
    for n in 9..=200 {
        #[allow(clippy::cast_precision_loss)]
        let d = alpha.cos() * (1.0 - (PI / n as f64).cos());
        t.row(&format!("cone n {n}"), frustum(1.0, 0.5, 1.0, Revolution::Full), d);
    }
    t.report("rounding edge");
}

/// The rounding edge on ROUND inputs: round radii, round δ, round
/// wedge angles, every curved kind. Counts refusals only.
#[test]
fn review_rounding_edge_round_inputs() {
    let mut rows = 0;
    let mut refused = Vec::new();
    let radii = [0.01, 0.02, 0.05, 0.1, 0.2, 0.25, 0.5, 0.75, 1.0, 1.5, 2.0, 2.5, 3.0, 5.0, 10.0];
    let mut deltas = Vec::new();
    for k in 1..=6 {
        for m in [1.0, 2.0, 2.5, 5.0] {
            deltas.push(m * 10f64.powi(-k));
        }
    }
    let revs = [
        Revolution::Full,
        Revolution::Partial(PI / 2.0),
        Revolution::Partial(PI / 3.0),
        Revolution::Partial(PI / 4.0),
        Revolution::Partial(PI),
        Revolution::Partial(1.0),
    ];
    for &r in &radii {
        for rev in revs {
            let bodies: Vec<(&str, Option<Body<f64>>)> = vec![
                ("cyl", frustum(r, r, r, rev)),
                ("cone", frustum(r, 0.5 * r, r, rev)),
                ("apexcone", frustum(r, 0.0, r, rev)),
                ("ball", zone(r, -PI / 2.0, PI / 2.0, rev)),
                ("donut", {
                    let lp = bulge_loop(vec![
                        (Point2::new(3.0 * r, -r), 1.0),
                        (Point2::new(3.0 * r, r), 1.0),
                    ]);
                    revolve(lp, rev)
                }),
            ];
            for (name, body) in &bodies {
                let Some(body) = body else { continue };
                for &d in &deltas {
                    if d > 0.5 * r || d < 2e-4 * r {
                        continue;
                    }
                    rows += 1;
                    if let Err(e) = mesh::tessellate(body, d, Tol::witness()) {
                        println!("REFUSED {name} r {r} {rev:?} δ {d:e}: {e:?}");
                        refused.push(format!("{name} r {r} {rev:?} δ {d:e}: {e:?}"));
                    }
                }
            }
        }
    }
    println!("== round inputs: {rows} rows, {} refused", refused.len());
    for r in &refused {
        println!("   refused: {r}");
    }
}
