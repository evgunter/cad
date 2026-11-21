//! REVIEW PROBE (r2), not for merge: a differential of the session box's
//! verdict against a sampled oracle, over revolved bodies under random
//! rigid motions that stand them near the box's faces and corners.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, RigidMotion, Tol, UnitVec3, Vec3};
use sweep::Revolution;
use sweep::test_support::try_revolved_about_y_at;

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
    fn sym(&mut self) -> f64 {
        2.0 * self.next() - 1.0
    }
}

/// Dense points of the bulge loop, in the sketch.
fn profile_points(verts: &[(Point2<f64>, f64)], n: usize) -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    for i in 0..verts.len() {
        let (p0, b) = verts[i];
        let p1 = verts[(i + 1) % verts.len()].0;
        let (dx, dy) = (p1.x - p0.x, p1.y - p0.y);
        if b == 0.0 {
            for k in 0..=n {
                let t = k as f64 / n as f64;
                out.push((p0.x + dx * t, p0.y + dy * t));
            }
        } else {
            let theta = 4.0 * b.atan();
            let d = (dx * dx + dy * dy).sqrt();
            let r = d / (2.0 * (theta / 2.0).sin());
            // centre: to the left of travel for a positive bulge.
            let h = (d / 2.0) / (theta / 2.0).tan();
            let (mx, my) = ((p0.x + p1.x) / 2.0, (p0.y + p1.y) / 2.0);
            let (lx, ly) = (-dy / d, dx / d);
            let (cx, cy) = (mx + lx * h, my + ly * h);
            let a0 = (p0.y - cy).atan2(p0.x - cx);
            for k in 0..=n {
                let a = a0 + theta * (k as f64 / n as f64);
                out.push((cx + r.abs() * a.cos(), cy + r.abs() * a.sin()));
            }
        }
    }
    out
}

/// The largest |coordinate| of the revolved surface under `m`, sampled:
/// a lower bound on the body's, short of it by under a millimetre at
/// these sizes.
fn oracle(profile: &[(f64, f64)], sweep: (f64, f64), m: &RigidMotion<f64>) -> f64 {
    let nphi = 3000;
    let mut far = 0.0_f64;
    for &(x, y) in profile {
        for k in 0..=nphi {
            let phi = sweep.0 + (sweep.1 - sweep.0) * (k as f64) / (nphi as f64);
            let q = m.transform_point(Point3::new(x * phi.cos(), y, -x * phi.sin()));
            far = far.max(q.x.abs()).max(q.y.abs()).max(q.z.abs());
        }
    }
    far
}

fn box_word(errors: &[topo::ValidationError]) -> String {
    match errors {
        [topo::ValidationError::OutsideSessionBox { .. }] => "PAST".into(),
        [topo::ValidationError::SessionBoxUnshown { .. }] => "UNSHOWN".into(),
        [topo::ValidationError::SessionBoxUncovered { .. }] => "UNCOVERED".into(),
        other => format!("OTHER({})", other.len()),
    }
}

fn shapes() -> Vec<(&'static str, Vec<(Point2<f64>, f64)>)> {
    let p = Point2::new;
    let t8 = (core::f64::consts::PI / 8.0).tan();
    let t16 = (core::f64::consts::PI / 16.0).tan();
    let t12 = (core::f64::consts::PI / 12.0).tan();
    vec![
        ("ball r=300", vec![(p(0.0, -300.0), 1.0), (p(0.0, 300.0), 0.0)]),
        (
            "torus R=450 r=400 seams top/bottom",
            vec![(p(450.0, -400.0), 1.0), (p(450.0, 400.0), 1.0)],
        ),
        (
            "torus R=500 r=100 seams in/out",
            vec![(p(400.0, 0.0), 1.0), (p(600.0, 0.0), 1.0)],
        ),
        (
            "torus R=300 r=250 seams oblique",
            vec![(p(300.0 + 150.0, 200.0), 1.0), (p(300.0 - 150.0, -200.0), 1.0)],
        ),
        (
            "D outer bulge (torus outer part + cylinder)",
            vec![(p(300.0, -200.0), 0.6), (p(300.0, 200.0), 0.0)],
        ),
        (
            "D inner bulge (cylinder + torus inner part)",
            vec![(p(500.0, 200.0), 0.5), (p(500.0, -200.0), 0.0)],
        ),
        (
            "lens off axis (two torus parts)",
            vec![(p(400.0, -250.0), 0.4), (p(400.0, 250.0), 0.4)],
        ),
        (
            "dome r=400",
            vec![
                (p(200.0, 0.0), 0.0),
                (p(400.0, 0.0), t16),
                (
                    p(
                        400.0 * core::f64::consts::FRAC_1_SQRT_2,
                        400.0 * core::f64::consts::FRAC_1_SQRT_2,
                    ),
                    0.0,
                ),
                (p(200.0, 400.0 * core::f64::consts::FRAC_1_SQRT_2), 0.0),
            ],
        ),
        (
            "spool x200",
            vec![
                (p(100.0, 0.0), 0.0),
                (p(400.0, 0.0), t12),
                (p(350.0, 50.0 * 3.0f64.sqrt()), 0.0),
                (p(100.0, 50.0 * 3.0f64.sqrt()), 0.0),
            ],
        ),
        (
            "lemon (spindle)",
            vec![(p(0.0, -300.0), 0.5), (p(0.0, 300.0), 0.0)],
        ),
        (
            "apple (spindle outer)",
            vec![(p(0.0, -200.0), 1.6), (p(0.0, 200.0), 0.0)],
        ),
        (
            "cone on axis",
            vec![(p(0.0, 0.0), 0.0), (p(300.0, 0.0), 0.0), (p(0.0, 400.0), 0.0)],
        ),
        (
            "frustum + sphere cap",
            vec![
                (p(0.0, -100.0), 0.0),
                (p(300.0, -100.0), 0.0),
                (p(200.0, 200.0), t8 * 0.8),
                (p(0.0, 320.0), 0.0),
            ],
        ),
        (
            "fat torus quarter-in arcs R=350 r=300",
            vec![
                (p(350.0, -300.0), t8),
                (p(650.0, 0.0), t8),
                (p(350.0, 300.0), t8),
                (p(50.0, 0.0), t8),
            ],
        ),
    ]
}

#[test]
fn review_r2_differential_of_the_box_against_a_sampled_oracle() {
    let tol = Tol::witness();
    let band = geom_core::Band::linear(tol).unwrap();
    let mut rng = Lcg(0x5eed_0002);
    let mut tally: std::collections::BTreeMap<String, usize> = Default::default();
    let mut bad = Vec::new();
    let partials = [
        None,
        Some(1.1_f64),
        Some(-2.3),
        Some(4.4),
    ];
    for (name, verts) in shapes() {
        for partial in partials {
            let rev = partial.map_or(Revolution::Full, Revolution::Partial);
            let body = match try_revolved_about_y_at::<f64>(verts.clone(), rev, tol) {
                Ok(b) => b,
                Err(e) => {
                    println!("SKIP {name} {partial:?}: revolve refuses: {e:?}");
                    continue;
                }
            };
            if let Err(e) = topo::validate_geometric(&body, tol) {
                println!("SKIP {name} {partial:?}: at home not finished: {}", box_word(&e));
                continue;
            }
            let profile = profile_points(&verts, 400);
            // The partial revolve's direction: whichever sign's sampled
            // sweep holds every vertex of the body.
            let sweeps: Vec<(f64, f64)> = match partial {
                None => vec![(0.0, core::f64::consts::TAU)],
                Some(a) => vec![(0.0, a), (0.0, -a)],
            };
            let sweep_of = sweeps
                .iter()
                .copied()
                .find(|s| {
                    // every vertex lies on the sampled sweep (to the sampling)
                    body.vertex_points().all(|(_, q)| {
                        let rho = (q.x * q.x + q.z * q.z).sqrt();
                        if rho < 1e-6 {
                            return true;
                        }
                        let phi = (-q.z).atan2(q.x);
                        let (lo, hi) = if s.1 >= s.0 { (s.0, s.1) } else { (s.1, s.0) };
                        let tau = core::f64::consts::TAU;
                        [phi, phi + tau, phi - tau]
                            .into_iter()
                            .any(|f| f >= lo - 1e-6 && f <= hi + 1e-6)
                    })
                });
            let Some(sweep) = sweep_of else {
                println!("SKIP {name} {partial:?}: no sweep sign holds the vertices");
                continue;
            };
            for case in 0..14 {
                let axis = Vec3::new(rng.sym(), rng.sym(), rng.sym());
                let angle = if case == 0 { 0.0 } else { rng.sym() * 3.1 };
                let Ok(axis) = UnitVec3::new(axis, "probe axis", band) else {
                    continue;
                };
                let turn = RigidMotion::rotation(axis, angle).unwrap();
                // Stand it so its sampled far coordinate is 1000 + delta
                // along one axis, or all three (a corner).
                let delta = {
                    let d = rng.sym() * 40.0;
                    if d.abs() < 0.5 { 0.5_f64.copysign(d) } else { d }
                };
                let corner = case % 4 == 3;
                // extremes of the turned body per axis
                let mut lo = [f64::INFINITY; 3];
                let mut hi = [f64::NEG_INFINITY; 3];
                for &(x, y) in &profile {
                    for k in 0..=600 {
                        let phi = sweep.0 + (sweep.1 - sweep.0) * (k as f64) / 600.0;
                        let q = turn.transform_point(Point3::new(x * phi.cos(), y, -x * phi.sin()));
                        for (i, c) in [q.x, q.y, q.z].into_iter().enumerate() {
                            lo[i] = lo[i].min(c);
                            hi[i] = hi[i].max(c);
                        }
                    }
                }
                let which = (rng.next() * 3.0) as usize % 3;
                let neg = rng.next() < 0.5;
                let mut t = [0.0; 3];
                for i in 0..3 {
                    if i == which || corner {
                        let want = 1000.0 + if i == which { delta } else { -rng.next() * 30.0 - 1.0 };
                        t[i] = if neg { -want - lo[i] } else { want - hi[i] };
                    } else {
                        t[i] = -(lo[i] + hi[i]) / 2.0;
                    }
                }
                let at = RigidMotion::translation(Vec3::new(t[0], t[1], t[2]))
                    .unwrap()
                    .compose(turn)
                    .unwrap();
                let truth = oracle(&profile, sweep, &at);
                if (truth - 1000.0).abs() < 0.05 {
                    continue;
                }
                let image = topo::transform_rigid(&body, &at).unwrap();
                let gate = match topo::validate_geometric(&image, tol) {
                    Ok(()) => "OK".to_string(),
                    Err(e) => box_word(&e),
                };
                let door = match topo::session_box_at(&body, &at, tol) {
                    Ok(()) => "OK".to_string(),
                    Err(topo::MoveError::NotFinished(found)) => box_word(found.as_slice()),
                    Err(other) => format!("MOVEERR({other:?})"),
                };
                let side = if truth > 1000.0 { "truly-past" } else { "truly-in" };
                *tally.entry(format!("{side} gate={gate} door={door}")).or_default() += 1;
                let wrong = (truth > 1000.0 && (gate == "OK" || door == "OK"))
                    || (truth < 1000.0 && (gate == "PAST" || door == "PAST"))
                    || gate != door;
                let limit = gate == "UNSHOWN" || gate == "UNCOVERED" || gate.starts_with("OTHER");
                if wrong || limit {
                    let line = format!(
                        "{} {name} {partial:?} case {case}: truth {truth:.4} gate {gate} door {door} (corner {corner})",
                        if wrong { "WRONG" } else { "limit" }
                    );
                    println!("{line}");
                    if wrong {
                        bad.push(line);
                    }
                }
            }
        }
    }
    println!("TALLY:");
    for (k, v) in &tally {
        println!("  {v:5}  {k}");
    }
    assert!(bad.is_empty(), "{} wrong verdicts:\n{}", bad.len(), bad.join("\n"));
}
