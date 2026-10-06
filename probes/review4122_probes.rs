//! Reviewer probes for PR #4122 (`boolean::separating::apart`).
//!
//! Each fixture is a curved operand and a planar one whose closest
//! approach is known in closed form as a signed gap `δ` (`δ ≤ 0`:
//! touching or interpenetrating). The pair is turned and moved
//! TOGETHER through pseudo-random rigid poses at three scales, and
//! every op runs in both operand orders. The oracle is the closed-form
//! gap, never the kernel:
//!
//! - `δ ≤ 0` and the op BUILDS as if disjoint (union volume = sum,
//!   `∩` empty, or an `Assembly`) on an unarmed kind: a MAJOR.
//! - `δ > 0` and the op builds: volumes held to the closed forms.
//! - `δ > 0` far above the pad and the op refuses: completeness note.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{brick, revolved_about_y};
use topo::{AtRestBody, Body, BooleanError, BooleanResult};

fn lcg(seed: &mut u64) -> f64 {
    *seed = seed
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407);
    ((*seed >> 11) as f64) / ((1u64 << 53) as f64)
}

/// Identity plus `n` pseudo-random rigid poses (rotation about a random
/// axis through the origin, then a translation of up to `2s`).
fn poses(n: usize, s: f64, seed: u64) -> Vec<(String, Affine3<f64>)> {
    let mut st = seed;
    let mut out = vec![("identity".to_owned(), Affine3::identity())];
    for k in 0..n {
        let ax = Vec3::new(
            lcg(&mut st) - 0.5,
            lcg(&mut st) - 0.5,
            lcg(&mut st) - 0.5,
        )
        .normalize();
        let ang = lcg(&mut st) * 2.0 * PI;
        let t = Vec3::new(
            (lcg(&mut st) - 0.5) * 4.0 * s,
            (lcg(&mut st) - 0.5) * 4.0 * s,
            (lcg(&mut st) - 0.5) * 4.0 * s,
        );
        let m = Affine3::translation(t)
            * Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), ax, ang);
        out.push((format!("pose{k} ang {ang:.3} ax {ax:?}"), m));
    }
    out
}

fn tol() -> Tol {
    Tol::witness()
}

fn posed(body: &Body<f64>, map: &Affine3<f64>) -> Option<AtRestBody<f64>> {
    let b = topo::transform_rigid(body, map, tol()).ok()?;
    use topo::AtRestPolicy;
    f64::gate_at_rest_kept(b, tol()).ok()
}

fn rot_z(theta: f64) -> Affine3<f64> {
    Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), theta)
}

fn place(body: Body<f64>, m: &Affine3<f64>) -> Body<f64> {
    topo::transform_rigid(&body, m, tol()).unwrap()
}

/// The PR's frustum about y: inner cylinder r = 0.2, outer cone
/// r = 0.6 at y = 0 → 0.4 at y = 0.6; `rev` full or partial.
fn frustum(s: f64, rev: Revolution<f64>) -> Body<f64> {
    revolved_about_y(
        [(0.2, 0.0), (0.6, 0.0), (0.4, 0.6), (0.2, 0.6)]
            .into_iter()
            .map(|(r, y)| (Point2::new(r * s, y * s), 0.0))
            .collect(),
        rev,
        tol(),
    )
}

/// A torus about y: tube centre (0.6, 0.3)·s, tube radius 0.2·s.
fn torus(s: f64) -> Body<f64> {
    revolved_about_y(
        vec![
            (Point2::new(0.4 * s, 0.3 * s), 1.0),
            (Point2::new(0.8 * s, 0.3 * s), 1.0),
        ],
        Revolution::Full,
        tol(),
    )
}

fn ball(r: f64) -> Body<f64> {
    revolved_about_y(
        vec![(Point2::new(0.0, -r), 1.0), (Point2::new(0.0, r), 0.0)],
        Revolution::Full,
        tol(),
    )
}

struct Fixture {
    name: &'static str,
    /// Curved operand, planar operand, their volumes.
    build: fn(s: f64, delta: f64) -> (Body<f64>, f64, Body<f64>, f64),
    /// Whether the curved kind is armed (a seam result is then legal
    /// at δ ≤ 0).
    armed: bool,
}

fn fixtures() -> Vec<Fixture> {
    vec![
        // A bar's edge on the cone wall: point touch at (0.5, 0.3, 0)·s;
        // signed gap 3δ/√10.
        Fixture {
            name: "cone point (bar edge on the wall)",
            build: |s, d| {
                let c = frustum(s, Revolution::Full);
                let b = brick::<f64>(
                    (0.5 * s + d * 10f64.sqrt() / 3.0, 1.0 * s),
                    (0.3 * s, 0.5 * s),
                    (-0.2 * s, 0.2 * s),
                    tol(),
                );
                let vb = (0.5 * s - d * 10f64.sqrt() / 3.0) * 0.2 * s * 0.4 * s;
                (c, 0.128 * PI * s.powi(3), b, vb)
            },
            armed: false,
        },
        // A bar's face in the cone's tangent plane along the generator
        // at azimuth 0: line touch; gap δ.
        Fixture {
            name: "cone line (bar face in the tangent plane)",
            build: |s, d| {
                let c = frustum(s, Revolution::Full);
                let l = (0.04f64 + 0.36).sqrt();
                let local = brick::<f64>(
                    (d, d + 0.3 * s),
                    (0.2 * l * s, 0.8 * l * s),
                    (-0.1 * s, 0.1 * s),
                    tol(),
                );
                let theta = (1.0f64).atan2(3.0);
                let m = Affine3::translation(Vec3::new(0.6 * s, 0.0, 0.0)) * rot_z(theta);
                let vb = 0.3 * s * 0.6 * l * s * 0.2 * s;
                (c, 0.128 * PI * s.powi(3), place(local, &m), vb)
            },
            armed: false,
        },
        // A 270° frustum; a bar standing in the missing wedge, its face
        // on the cut cap plane z = 0 side... the cap at azimuth 0 is
        // the plane z = 0 (x > 0); the wedge is azimuth 270°..360°,
        // i.e. z > 0 for x > 0 under the y-axis right-hand rule? The
        // probe places the bar on BOTH sides and lets the oracle be the
        // full-turn gap: the bar is clear of the cone's carrier by δ
        // radially at the cut plane.
        Fixture {
            name: "partial cone, bar radially outside at the cut",
            build: |s, d| {
                let c = frustum(s, Revolution::Partial(1.5 * PI));
                let b = brick::<f64>(
                    (0.5 * s + d * 10f64.sqrt() / 3.0, 1.0 * s),
                    (0.3 * s, 0.5 * s),
                    (-0.05 * s, 0.05 * s),
                    tol(),
                );
                let vb = (0.5 * s - d * 10f64.sqrt() / 3.0) * 0.2 * s * 0.1 * s;
                (c, 0.128 * PI * s.powi(3) * 0.75, b, vb)
            },
            armed: false,
        },
        // A bar's face tangent to a torus at its outer equator: point
        // touch at (0.8, 0.3, 0)·s; gap δ.
        Fixture {
            name: "torus outer equator point",
            build: |s, d| {
                let t = torus(s);
                let b = brick::<f64>(
                    (0.8 * s + d, 1.2 * s),
                    (0.2 * s, 0.4 * s),
                    (-0.1 * s, 0.1 * s),
                    tol(),
                );
                let vb = (0.4 * s - d) * 0.2 * s * 0.2 * s;
                (t, 2.0 * PI * PI * 0.6 * 0.04 * s.powi(3), b, vb)
            },
            armed: false,
        },
        // A bar's face tangent to the torus at 45° up its tube: point
        // (0.6 + 0.2c, 0.3 + 0.2c, 0)·s, normal (c, c, 0); gap δ.
        Fixture {
            name: "torus oblique point",
            build: |s, d| {
                let t = torus(s);
                let c = core::f64::consts::FRAC_1_SQRT_2;
                let local = brick::<f64>(
                    (d, d + 0.3 * s),
                    (-0.05 * s, 0.05 * s),
                    (-0.05 * s, 0.05 * s),
                    tol(),
                );
                let m = Affine3::translation(Vec3::new(
                    (0.6 + 0.2 * c) * s,
                    (0.3 + 0.2 * c) * s,
                    0.0,
                )) * rot_z(PI / 4.0);
                (
                    t,
                    2.0 * PI * PI * 0.6 * 0.04 * s.powi(3),
                    place(local, &m),
                    0.3 * s * 0.01 * s * s,
                )
            },
            armed: false,
        },
        // A brick's corner on the ball along (1,1,1): gap δ.
        Fixture {
            name: "ball corner point",
            build: |s, d| {
                let a = (s + d) / 3f64.sqrt();
                let b = brick::<f64>((a, a + 0.5 * s), (a, a + 0.5 * s), (a, a + 0.5 * s), tol());
                let ball = ball(s);
                (ball, 4.0 / 3.0 * PI * s.powi(3), b, 0.125 * s.powi(3))
            },
            armed: true,
        },
        // A brick's edge (along z) tangent to the ball at (1,1,0)/√2.
        Fixture {
            name: "ball edge point",
            build: |s, d| {
                let a = (s + d) / 2f64.sqrt();
                let b = brick::<f64>((a, a + 0.5 * s), (a, a + 0.5 * s), (-0.5 * s, 0.5 * s), tol());
                let ball = ball(s);
                (ball, 4.0 / 3.0 * PI * s.powi(3), b, 0.25 * s.powi(3))
            },
            armed: true,
        },
    ]
}

#[derive(Default, Debug)]
struct Tally {
    runs: usize,
    refused: usize,
    built: usize,
    wrong: Vec<String>,
    incomplete: Vec<String>,
}

fn verdict(r: &Result<BooleanResult<f64>, BooleanError>) -> String {
    match r {
        Ok(BooleanResult::Empty) => "Empty".into(),
        Ok(BooleanResult::Body(b)) => format!("Body({:?})", b.kind),
        Err(e) => {
            let s = format!("{e:?}");
            s.chars().take(90).collect()
        }
    }
}

fn run_fixture(fx: &Fixture, eps_label: &str, t: &mut Tally) {
    let deltas_rel = [2e-2, 1e-4, 1e-7, 0.0, -1e-7, -1e-4];
    let deltas_abs = [-1e-6, -1e-9, -1e-12];
    for s in [1e-3, 1.0, 1e3] {
        let mut deltas: Vec<f64> = deltas_rel.iter().map(|d| d * s).collect();
        deltas.extend(deltas_abs);
        for &d in &deltas {
            let (curved, vc, flat, vf) = (fx.build)(s, d);
            for (pose, map) in poses(6, s, 0x4122 ^ s.to_bits()) {
                let (Some(a), Some(b)) = (posed(&curved, &map), posed(&flat, &map)) else {
                    t.incomplete.push(format!("{} s {s} δ {d:e} {pose}: transform failed", fx.name));
                    continue;
                };
                let label = format!("[{eps_label}] {} s {s:e} δ {d:e} {pose}", fx.name);
                for (op, out, disjoint_truth) in [
                    ("a∪b", topo::union(&a, &b, tol()), Some(vc + vf)),
                    ("b∪a", topo::union(&b, &a, tol()), Some(vc + vf)),
                    ("a∖b", topo::subtract(&a, &b, tol()), Some(vc)),
                    ("b∖a", topo::subtract(&b, &a, tol()), Some(vf)),
                    ("a∩b", topo::intersect(&a, &b, tol()), None),
                    ("b∩a", topo::intersect(&b, &a, tol()), None),
                ] {
                    t.runs += 1;
                    let v = verdict(&out);
                    match &out {
                        Err(_) => {
                            t.refused += 1;
                            if d >= 1e-4 * s && !fx.armed {
                                t.incomplete.push(format!("{label} {op}: {v}"));
                            }
                        }
                        Ok(res) => {
                            t.built += 1;
                            let vol = match res {
                                BooleanResult::Empty => 0.0,
                                BooleanResult::Body(bb) => {
                                    match topo::mass_properties(&bb.body, tol()) {
                                        Ok(m) => m.volume,
                                        Err(e) => {
                                            t.wrong.push(format!("{label} {op}: built but mass {e:?}"));
                                            continue;
                                        }
                                    }
                                }
                            };
                            let truth = disjoint_truth.unwrap_or(0.0);
                            let off = (vol - truth).abs();
                            if d <= 0.0 && !fx.armed {
                                t.wrong.push(format!(
                                    "{label} {op}: TOUCHING/CROSSING pair built {v}, vol {vol:e} (disjoint truth {truth:e})"
                                ));
                            } else if d <= 0.0 && fx.armed {
                                if matches!(res, BooleanResult::Body(bb) if bb.kind == topo::BooleanResultKind::Assembly)
                                    && d < -1e-6 * s.max(1.0)
                                {
                                    t.wrong.push(format!("{label} {op}: crossing pair built as Assembly"));
                                }
                            } else if d > 1e-6 * s && off > 1e-6 * truth.max(s.powi(3)) {
                                t.wrong.push(format!(
                                    "{label} {op}: {v} vol {vol:e} vs closed form {truth:e}"
                                ));
                            }
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn review4122_touching_pairs_never_clear() {
    let eps = format!("ε {:e}", tol().eps());
    let only = std::env::var("PROBE_ONLY").ok();
    let mut report = String::new();
    let mut bad = false;
    for fx in fixtures() {
        if let Some(o) = &only {
            if !fx.name.contains(o.as_str()) {
                continue;
            }
        }
        let mut t = Tally::default();
        let start = std::time::Instant::now();
        run_fixture(&fx, &eps, &mut t);
        report.push_str(&format!(
            "== {} [{eps}]: runs {} built {} refused {} wrong {} incomplete {} ({:.1}s)\n",
            fx.name,
            t.runs,
            t.built,
            t.refused,
            t.wrong.len(),
            t.incomplete.len(),
            start.elapsed().as_secs_f64()
        ));
        for w in t.wrong.iter().take(12) {
            report.push_str(&format!("   WRONG {w}\n"));
        }
        for w in t.incomplete.iter().take(6) {
            report.push_str(&format!("   incomplete {w}\n"));
        }
        bad |= !t.wrong.is_empty();
    }
    println!("{report}");
    assert!(!bad, "{report}");
}

// ───────────────────────── probe 2: plates at the exact support ─────────

/// The rotation taking x̂ to the unit `d`.
fn x_to(d: Vec3<f64>) -> Affine3<f64> {
    let x = Vec3::new(1.0, 0.0, 0.0);
    let c = x.dot(d);
    let ax = x.cross(d);
    if ax.norm() < 1e-12 {
        return if c > 0.0 {
            Affine3::identity()
        } else {
            Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), PI)
        };
    }
    Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), ax.normalize(), c.clamp(-1.0, 1.0).acos())
}

/// Max over the kept azimuths `u ∈ [0, sweep]` of `dx·cos u − dz·sin u`
/// (points revolve about +y as `(ρ cos u, y, −ρ sin u)`), and its `u`.
fn azimuth_best(d: Vec3<f64>, sweep: f64) -> (f64, f64) {
    let ustar = (-d.z).atan2(d.x).rem_euclid(2.0 * PI);
    let a = |u: f64| d.x * u.cos() - d.z * u.sin();
    if ustar <= sweep {
        (a(ustar), ustar)
    } else if a(0.0) >= a(sweep) {
        (a(0.0), 0.0)
    } else {
        (a(sweep), sweep)
    }
}

fn rev_point(rho: f64, y: f64, u: f64) -> Point3<f64> {
    Point3::new(rho * u.cos(), y, -rho * u.sin())
}

/// (support value, support point) of the frustum solid along `d`.
fn frustum_support(s: f64, sweep: f64, d: Vec3<f64>) -> (f64, Point3<f64>) {
    let (a, u) = azimuth_best(d, sweep);
    let mut best = (f64::NEG_INFINITY, Point3::new(0.0, 0.0, 0.0));
    for (y, rho_out) in [(0.0, 0.6 * s), (0.6 * s, 0.4 * s)] {
        for rho in [rho_out, 0.2 * s] {
            let v = rho * a + d.y * y;
            if v > best.0 {
                best = (v, rev_point(rho, y, u));
            }
        }
    }
    best
}

/// (support value, support point) of the torus solid along `d`.
fn torus_support(s: f64, sweep: f64, d: Vec3<f64>) -> (f64, Point3<f64>) {
    let (big, r, yc) = (0.6 * s, 0.2 * s, 0.3 * s);
    let (a, u) = azimuth_best(d, sweep);
    let v = d.y.atan2(a);
    let rho = big + r * v.cos();
    let p = rev_point(rho, yc + r * v.sin(), u);
    (d.x * p.x + d.y * p.y + d.z * p.z, p)
}

fn torus_at(s: f64, rev: Revolution<f64>) -> Body<f64> {
    revolved_about_y(
        vec![
            (Point2::new(0.4 * s, 0.3 * s), 1.0),
            (Point2::new(0.8 * s, 0.3 * s), 1.0),
        ],
        rev,
        tol(),
    )
}

#[test]
fn review4122_plates_at_the_exact_support() {
    let eps = format!("ε {:e}", tol().eps());
    let band = geom_core::Band::linear(tol()).unwrap();
    let mut wrong = Vec::new();
    let mut lines = Vec::new();
    // Convention check: the 270° torus holds azimuth 0.75π, not 1.75π.
    {
        let t = torus_at(1.0, Revolution::Partial(1.5 * PI));
        let at = AtRestBody::clone(&posed(&t, &Affine3::identity()).unwrap());
        let inside = topo::point_in_solid(&at, rev_point(0.6, 0.3, 0.75 * PI), band, tol()).unwrap();
        let outside = topo::point_in_solid(&at, rev_point(0.6, 0.3, 1.75 * PI), band, tol()).unwrap();
        assert_eq!(
            (inside, outside),
            (topo::SolidContainment::In, topo::SolidContainment::Out),
            "azimuth convention"
        );
    }
    type Support = fn(f64, f64, Vec3<f64>) -> (f64, Point3<f64>);
    let cases: Vec<(&str, f64, Support, bool)> = vec![
        ("frustum full", 2.0 * PI, frustum_support, false),
        ("frustum 270°", 1.5 * PI, frustum_support, false),
        ("torus full", 2.0 * PI, torus_support, true),
        ("torus 270°", 1.5 * PI, torus_support, true),
    ];
    for (name, sweep, support, is_torus) in cases {
        let (mut runs, mut built, mut refused) = (0, 0, 0);
        let mut far_refused = 0;
        for s in [1e-3, 1.0, 1e3] {
            let rev = if sweep < 2.0 * PI {
                Revolution::Partial(sweep)
            } else {
                Revolution::Full
            };
            let curved = if is_torus { torus_at(s, rev) } else { frustum(s, rev) };
            let mut st = 0xC0FFEE ^ s.to_bits() ^ (sweep.to_bits() >> 3);
            for k in 0..16 {
                let d = Vec3::new(lcg(&mut st) - 0.5, lcg(&mut st) - 0.5, lcg(&mut st) - 0.5)
                    .normalize();
                let (h, p) = support(s, sweep, d);
                let pose = &poses(1, s, st)[1].1;
                for delta in [1e-3 * s, 1e-6 * s, 0.0, -1e-6 * s, -1e-9, -1e-12] {
                    let plate = brick::<f64>(
                        (delta, delta + 0.1 * s),
                        (-0.15 * s, 0.15 * s),
                        (-0.15 * s, 0.15 * s),
                        tol(),
                    );
                    let m = Affine3::translation(Vec3::new(p.x, p.y, p.z)) * x_to(d);
                    let plate = place(plate, &m);
                    let (Some(a), Some(b)) = (posed(&curved, pose), posed(&plate, pose)) else {
                        continue;
                    };
                    for (op, out) in [
                        ("a∪b", topo::union(&a, &b, tol())),
                        ("b∪a", topo::union(&b, &a, tol())),
                        ("a∩b", topo::intersect(&a, &b, tol())),
                    ] {
                        runs += 1;
                        let v = verdict(&out);
                        let label = format!(
                            "[{eps}] {name} s {s:e} dir{k} {d:?} h {h:.6e} δ {delta:e} {op}: {v}"
                        );
                        match out {
                            Err(_) => {
                                refused += 1;
                                if delta >= 1e-3 * s {
                                    far_refused += 1;
                                }
                            }
                            Ok(res) => {
                                built += 1;
                                let disjoint = match &res {
                                    BooleanResult::Empty => true,
                                    BooleanResult::Body(bb) => {
                                        bb.kind == topo::BooleanResultKind::Assembly
                                    }
                                };
                                if delta <= 0.0 && (disjoint || !is_torus) {
                                    wrong.push(label.clone());
                                } else if delta <= 0.0 {
                                    lines.push(format!("   built-at-touch {label}"));
                                }
                            }
                        }
                    }
                }
            }
        }
        lines.push(format!(
            "== {name}: runs {runs} built {built} refused {refused} (far δ refused {far_refused})"
        ));
    }
    let mut report = lines.join("\n");
    for w in wrong.iter().take(20) {
        report.push_str(&format!("\n   WRONG {w}"));
    }
    println!("{report}");
    assert!(wrong.is_empty(), "{report}");
}

// ───────── probe 3: every built result sampled against analytic membership ─────────

#[derive(Clone, Copy)]
enum Shape {
    Frustum,
    Torus,
    Annulus,
}

/// Signed membership margin of the local point `p` in the revolved
/// shape (positive inside), scale `s`, kept azimuths `[0, sweep]`.
fn shape_margin(shape: Shape, s: f64, sweep: f64, p: Point3<f64>) -> f64 {
    let rho = (p.x * p.x + p.z * p.z).sqrt();
    let u = (-p.z).atan2(p.x).rem_euclid(2.0 * PI);
    let az = if sweep >= 2.0 * PI {
        f64::INFINITY
    } else {
        // distance-ish margin to the two cut half-planes
        let to0 = if u <= sweep { u.min(sweep - u) } else { -(u - sweep).min(2.0 * PI - u) };
        to0 * rho
    };
    let m = match shape {
        Shape::Frustum => (rho - 0.2 * s)
            .min(((0.6 * s - p.y / 3.0) - rho) * 3.0 / 10f64.sqrt())
            .min(p.y)
            .min(0.6 * s - p.y),
        Shape::Annulus => (rho - 0.2 * s).min(0.6 * s - rho).min(p.y).min(0.6 * s - p.y),
        Shape::Torus => 0.2 * s - ((rho - 0.6 * s).powi(2) + (p.y - 0.3 * s).powi(2)).sqrt(),
    };
    m.min(az)
}

fn shape_body(shape: Shape, s: f64, rev: Revolution<f64>) -> Body<f64> {
    match shape {
        Shape::Frustum => frustum(s, rev),
        Shape::Torus => torus_at(s, rev),
        Shape::Annulus => revolved_about_y(
            [(0.2, 0.0), (0.6, 0.0), (0.6, 0.6), (0.2, 0.6)]
                .into_iter()
                .map(|(r, y)| (Point2::new(r * s, y * s), 0.0))
                .collect(),
            rev,
            tol(),
        ),
    }
}

fn shape_support(shape: Shape, s: f64, sweep: f64, d: Vec3<f64>) -> (f64, Point3<f64>) {
    match shape {
        Shape::Frustum => frustum_support(s, sweep, d),
        Shape::Torus => torus_support(s, sweep, d),
        Shape::Annulus => {
            let (a, u) = azimuth_best(d, sweep);
            let mut best = (f64::NEG_INFINITY, Point3::new(0.0, 0.0, 0.0));
            for y in [0.0, 0.6 * s] {
                for rho in [0.6 * s, 0.2 * s] {
                    let v = rho * a + d.y * y;
                    if v > best.0 {
                        best = (v, rev_point(rho, y, u));
                    }
                }
            }
            best
        }
    }
}

#[test]
fn review4122_built_results_hold_analytic_membership() {
    let eps = format!("ε {:e}", tol().eps());
    let band = geom_core::Band::linear(tol()).unwrap();
    let mut wrong: Vec<String> = Vec::new();
    let mut lines = Vec::new();
    for (shape, name) in [
        (Shape::Annulus, "annulus"),
        (Shape::Frustum, "frustum"),
        (Shape::Torus, "torus"),
    ] {
        for sweep in [2.0 * PI, 1.5 * PI] {
            let (mut runs, mut built, mut sampled, mut far_refused) = (0, 0, 0usize, 0);
            for s in [1e-3, 1.0, 1e3] {
                let rev = if sweep < 2.0 * PI { Revolution::Partial(sweep) } else { Revolution::Full };
                let curved = shape_body(shape, s, rev);
                let mut st = 0xBADA55 ^ s.to_bits() ^ (sweep.to_bits() >> 5);
                for k in 0..8 {
                    let d = Vec3::new(lcg(&mut st) - 0.5, lcg(&mut st) - 0.5, lcg(&mut st) - 0.5)
                        .normalize();
                    let (_, p) = shape_support(shape, s, sweep, d);
                    let pose = poses(1, s, st)[1].1;
                    for delta in [1e-3 * s, 1e-7 * s, 0.0, -1e-6 * s, -2e-2 * s] {
                        let m = Affine3::translation(Vec3::new(p.x, p.y, p.z)) * x_to(d);
                        let plate = place(
                            brick::<f64>(
                                (delta, delta + 0.1 * s),
                                (-0.15 * s, 0.15 * s),
                                (-0.15 * s, 0.15 * s),
                                tol(),
                            ),
                            &m,
                        );
                        let (Some(a), Some(b)) = (posed(&curved, &pose), posed(&plate, &pose)) else {
                            continue;
                        };
                        let inv_a = pose.inverse();
                        let inv_b = (pose * m).inverse();
                        let ma = |q: Point3<f64>| shape_margin(shape, s, sweep, inv_a.transform_point(q));
                        let mb = |q: Point3<f64>| {
                            let l = inv_b.transform_point(q);
                            (l.x - delta)
                                .min(delta + 0.1 * s - l.x)
                                .min(0.15 * s - l.y.abs())
                                .min(0.15 * s - l.z.abs())
                        };
                        let world_p = pose.transform_point(p);
                        for (op, out) in [
                            ("a∪b", topo::union(&a, &b, tol())),
                            ("b∪a", topo::union(&b, &a, tol())),
                            ("a∖b", topo::subtract(&a, &b, tol())),
                            ("b∖a", topo::subtract(&b, &a, tol())),
                            ("a∩b", topo::intersect(&a, &b, tol())),
                        ] {
                            runs += 1;
                            let label = format!(
                                "[{eps}] {name} sweep {sweep:.3} s {s:e} dir{k} δ {delta:e} {op}: {}",
                                verdict(&out)
                            );
                            let res = match out {
                                Err(_) => {
                                    if delta >= 1e-3 * s {
                                        far_refused += 1;
                                    }
                                    continue;
                                }
                                Ok(r) => r,
                            };
                            built += 1;
                            let truth = |q: Point3<f64>| -> Option<bool> {
                                let (x, y) = (ma(q), mb(q));
                                let g = 1e-6 * s;
                                if x.abs() < g || y.abs() < g {
                                    return None;
                                }
                                let (ia, ib) = (x > 0.0, y > 0.0);
                                Some(match op {
                                    "a∪b" | "b∪a" => ia || ib,
                                    "a∖b" => ia && !ib,
                                    "b∖a" => ib && !ia,
                                    _ => ia && ib,
                                })
                            };
                            let mut ps = st ^ 0x5eed;
                            let mut bad = 0;
                            for i in 0..120 {
                                let r = if i < 80 { 0.05 * s } else { 1.0 * s };
                                let q = Point3::new(
                                    world_p.x + (lcg(&mut ps) - 0.5) * 2.0 * r,
                                    world_p.y + (lcg(&mut ps) - 0.5) * 2.0 * r,
                                    world_p.z + (lcg(&mut ps) - 0.5) * 2.0 * r,
                                );
                                let Some(want) = truth(q) else { continue };
                                let got = match &res {
                                    BooleanResult::Empty => Some(false),
                                    BooleanResult::Body(bb) => {
                                        match topo::point_in_solid(&bb.body, q, band, tol()) {
                                            Ok(topo::SolidContainment::In) => Some(true),
                                            Ok(topo::SolidContainment::Out) => Some(false),
                                            _ => None,
                                        }
                                    }
                                };
                                sampled += 1;
                                if let Some(g) = got {
                                    if g != want {
                                        bad += 1;
                                    }
                                }
                            }
                            if bad > 0 {
                                wrong.push(format!("{label}: {bad} sampled points misclassified"));
                            }
                        }
                    }
                }
            }
            lines.push(format!(
                "== {name} sweep {sweep:.3}: runs {runs} built {built} sampled {sampled} far-δ refused {far_refused}"
            ));
        }
    }
    let mut report = lines.join("\n");
    for w in wrong.iter().take(30) {
        report.push_str(&format!("\n   WRONG {w}"));
    }
    println!("{report}");
    assert!(wrong.is_empty(), "{report}");
}
