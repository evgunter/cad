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
        other => {
            let first = other.first().map(|e| format!("{e:?}")).unwrap_or_default();
            format!("OTHER({} {})", other.len(), first.chars().take(60).collect::<String>())
        }
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
                    || (gate != door && !gate.starts_with("OTHER"));
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

/// Mean microseconds of `f` over `n` runs after a warm-up.
fn mean_us(n: u32, mut f: impl FnMut()) -> f64 {
    for _ in 0..3 {
        f();
    }
    let t = std::time::Instant::now();
    for _ in 0..n {
        f();
    }
    t.elapsed().as_secs_f64() * 1e6 / f64::from(n)
}

/// REVIEW PROBE: the box's ask alone (the placing door under the
/// identity) and the structural gate, timed. Planted at the merge base
/// too; the figures are compared across the two builds.
#[test]
fn review_r2_timing_of_the_ask() {
    use crate::common::interval::iv;
    use geom_core::Interval;
    let tol = Tol::witness();
    let id = RigidMotion::<f64>::identity();
    let p = Point2::new;
    let bodies: Vec<(&str, topo::Body<f64>)> = vec![
        (
            "brick",
            sweep::test_support::brick::<f64>((0.0, 1.0), (0.0, 2.0), (0.0, 0.5), tol),
        ),
        (
            "ball r=50",
            sweep::test_support::ball_poled_y::<f64>(50.0, Vec3::new(0.0, 0.0, 0.0), tol),
        ),
        (
            "torus 450/400",
            sweep::test_support::revolved_about_y(
                vec![(p(450.0, -400.0), 1.0), (p(450.0, 400.0), 1.0)],
                Revolution::Full,
                tol,
            ),
        ),
        ("loft prism", sweep::test_support::loft_prism(tol)),
    ];
    for (name, body) in &bodies {
        let ask = mean_us(400, || {
            topo::session_box_at(body, &id, tol).unwrap();
        });
        let gate = mean_us(20, || {
            let _ = topo::validate_geometric_structural(body, tol);
        });
        println!("TIMING f64 {name}: ask {ask:.1} us, structural gate {gate:.1} us");
    }
    // The ask where the carriers reach past the box: a sphere wedge
    // inside the box whose ball pokes out of it (stages 1, 2 and 3), a
    // dome whose zone is windowed (stages 1 and 2), and the same two
    // 300 m nearer the origin (stage 1 alone).
    let band = geom_core::Band::linear(tol).unwrap();
    let turn = RigidMotion::rotation(
        UnitVec3::new(Vec3::new(0.0, 1.0, 0.0), "y", band).unwrap(),
        core::f64::consts::PI,
    )
    .unwrap();
    let wedge = try_revolved_about_y_at::<f64>(
        vec![(p(0.0, -300.0), 1.0), (p(0.0, 300.0), 0.0)],
        Revolution::Partial(1.1),
        tol,
    )
    .unwrap();
    let dome = sweep::test_support::dome(400.0, tol);
    for (name, body, motion) in [
        ("wedge near", &wedge, RigidMotion::translation(Vec3::new(600.0, 0.0, 0.0)).unwrap().compose(turn).unwrap()),
        ("wedge at the wall", &wedge, RigidMotion::translation(Vec3::new(900.0, 0.0, 0.0)).unwrap().compose(turn).unwrap()),
        ("dome near", &dome, RigidMotion::translation(Vec3::new(0.0, -700.0, 0.0)).unwrap()),
        ("dome at the wall", &dome, RigidMotion::translation(Vec3::new(0.0, -990.0, 0.0)).unwrap()),
    ] {
        let word = match topo::session_box_at(body, &motion, tol) {
            Ok(()) => "admitted".to_string(),
            Err(e) => format!("{e}").chars().take(40).collect(),
        };
        let ask = mean_us(200, || {
            let _ = topo::session_box_at(body, &motion, tol);
        });
        println!("TIMING f64 {name}: ask {ask:.1} us ({word})");
    }
    let id_iv = RigidMotion::<Interval>::identity();
    let ball = sweep::test_support::revolved_about_y_at::<Interval>(
        vec![
            (Point2::new(iv(0.0), iv(-50.0)), iv(1.0)),
            (Point2::new(iv(0.0), iv(50.0)), iv(0.0)),
        ],
        Revolution::Full,
        tol,
    );
    let torus = sweep::test_support::revolved_about_y_at::<Interval>(
        vec![
            (Point2::new(iv(450.0), iv(-400.0)), iv(1.0)),
            (Point2::new(iv(450.0), iv(400.0)), iv(1.0)),
        ],
        Revolution::Full,
        tol,
    );
    for (name, body) in [("ball r=50", &ball), ("torus 450/400", &torus)] {
        let ask = mean_us(100, || {
            topo::session_box_at(body, &id_iv, tol).unwrap();
        });
        let gate = mean_us(5, || {
            let _ = topo::validate_geometric_structural(body, tol);
        });
        println!("TIMING Interval {name}: ask {ask:.1} us, structural gate {gate:.1} us");
    }
}

/// REVIEW PROBE: how the limit is said, at the gate and at the placing
/// door, alone and beside another family's finding.
#[test]
fn review_r2_how_the_limit_is_said() {
    let tol = Tol::witness();
    let prism = sweep::test_support::loft_prism(tol);
    let at = RigidMotion::translation(Vec3::new(998.5, 0.0, 0.0)).unwrap();
    let placed = topo::transform_rigid(&prism, &at).unwrap();
    let errors = topo::validate_geometric(&placed, tol).unwrap_err();
    for e in &errors {
        println!("GATE SAYS: {e}");
    }
    match topo::session_box_at(&prism, &at, tol) {
        Err(e) => println!("DOOR SAYS: {e}"),
        Ok(()) => println!("DOOR admits"),
    }
    // A sphere wedge inside the box whose carrier ball pokes out of it.
    let wedge = try_revolved_about_y_at::<f64>(
        vec![(Point2::new(0.0, -300.0), 1.0), (Point2::new(0.0, 300.0), 0.0)],
        Revolution::Partial(1.1),
        tol,
    )
    .unwrap();
    // The wedge spans azimuth [0, 1.1] from +x toward -z (or +z); stand
    // its axis at x = 900 so that the ball reaches x = 1200 and the
    // wedge, turned a half turn about y to face -x, reaches 900.
    let turn = RigidMotion::rotation(
        UnitVec3::new(Vec3::new(0.0, 1.0, 0.0), "y", geom_core::Band::linear(tol).unwrap())
            .unwrap(),
        core::f64::consts::PI,
    )
    .unwrap();
    let at = RigidMotion::translation(Vec3::new(900.0, 0.0, 0.0))
        .unwrap()
        .compose(turn)
        .unwrap();
    let image = topo::transform_rigid(&wedge, &at).unwrap();
    let far = image
        .vertex_points()
        .map(|(_, q)| q.x.abs().max(q.y.abs()).max(q.z.abs()))
        .fold(0.0, f64::max);
    println!("WEDGE vertices reach {far}");
    match topo::validate_geometric(&image, tol) {
        Ok(()) => println!("WEDGE gate admits"),
        Err(errors) => {
            for e in &errors {
                println!("WEDGE GATE SAYS: {e}");
            }
        }
    }
}

fn gate_word(body: &topo::Body<f64>, tol: Tol) -> String {
    match topo::validate_geometric(body, tol) {
        Ok(()) => "OK".into(),
        Err(e) => format!("{} {:?}", box_word(&e), e.iter().map(ToString::to_string).collect::<Vec<_>>()),
    }
}

/// REVIEW PROBE: sphere faces a boolean trims, near the box's top. A
/// dimple's carrier ball pokes out of the box where the body does not; a
/// boss's top is past it.
#[test]
fn review_r2_boolean_trimmed_sphere_faces_near_the_box() {
    use sweep::test_support::{ball_poled_y, ball_poled_z, brick, finished};
    let tol = Tol::witness();
    for (pole, ball) in [
        ("z", ball_poled_z as fn(f64, Vec3<f64>, Tol) -> topo::Body<f64>),
        ("y", ball_poled_y::<f64>),
    ] {
        // A dimple: slab top at z = 990, ball r = 50 centred at z = 1000
        // (its top at 1050). The body's far coordinate is 990.
        let slab = finished(
            "slab",
            brick::<f64>((-100.0, 100.0), (-100.0, 100.0), (900.0, 990.0), tol),
            tol,
        );
        let hi = ball(50.0, Vec3::new(0.0, 0.0, 1000.0), tol);
        println!("DIMPLE pole {pole}: the ball alone at the gate: {}", gate_word(&hi, tol));
        // The ball itself is past the box, so it cannot be an operand at
        // rest there; build the dimple low and lift it.
        let slab0 = finished(
            "slab0",
            brick::<f64>((-100.0, 100.0), (-100.0, 100.0), (-90.0, 0.0), tol),
            tol,
        );
        let ball0 = finished("ball0", ball(50.0, Vec3::new(0.0, 0.0, 10.0), tol), tol);
        match topo::subtract(&slab0, &ball0, tol) {
            Ok(topo::BooleanResult::Body(b)) => {
                println!("DIMPLE pole {pole}: built at home: {}", gate_word(&b.body, tol));
                for lift in [900.0, 950.0, 985.0, 989.0, 995.0, 1005.0] {
                    let at = RigidMotion::translation(Vec3::new(0.0, 0.0, lift)).unwrap();
                    let image = topo::transform_rigid(&b.body, &at).unwrap();
                    let door = match topo::session_box_at(&b.body, &at, tol) {
                        Ok(()) => "OK".to_string(),
                        Err(e) => format!("{e}"),
                    };
                    println!(
                        "DIMPLE pole {pole} top at {} (ball's top {}): gate {} | door {}",
                        lift,
                        lift + 60.0,
                        gate_word(&image, tol),
                        door
                    );
                }
                // Turned so that the dimple opens toward +x.
                let band = geom_core::Band::linear(tol).unwrap();
                let turn = RigidMotion::rotation(
                    UnitVec3::new(Vec3::new(0.0, 1.0, 0.0), "y", band).unwrap(),
                    core::f64::consts::FRAC_PI_2,
                )
                .unwrap();
                for lift in [950.0, 985.0, 995.0] {
                    let at = RigidMotion::translation(Vec3::new(lift, 0.0, 0.0))
                        .unwrap()
                        .compose(turn)
                        .unwrap();
                    let image = topo::transform_rigid(&b.body, &at).unwrap();
                    let far = image
                        .vertex_points()
                        .map(|(_, q)| q.x.abs().max(q.y.abs()).max(q.z.abs()))
                        .fold(0.0, f64::max);
                    println!(
                        "DIMPLE pole {pole} turned to +x, vertices reach {far:.3}: gate {}",
                        gate_word(&image, tol)
                    );
                }
            }
            other => println!("DIMPLE pole {pole}: subtract gave {other:?}"),
        }
        // A boss: slab top at z = 0, ball r = 50 centred on it.
        let ball1 = finished("ball1", ball(50.0, Vec3::new(0.0, 0.0, 0.0), tol), tol);
        match topo::union(&slab0, &ball1, tol) {
            Ok(topo::BooleanResult::Body(b)) => {
                for lift in [940.0, 949.0, 951.0, 960.0] {
                    let at = RigidMotion::translation(Vec3::new(0.0, 0.0, lift)).unwrap();
                    let image = topo::transform_rigid(&b.body, &at).unwrap();
                    println!(
                        "BOSS pole {pole} crest at {}: gate {}",
                        lift + 50.0,
                        gate_word(&image, tol)
                    );
                }
            }
            other => println!("BOSS pole {pole}: union gave {other:?}"),
        }
    }
}

/// REVIEW PROBE: a slender frustum, whose cone's apex stands far past
/// the box while the body sits at the origin. Stage 1 hulls the apex.
#[test]
fn review_r2_a_slender_frustum_at_the_origin() {
    let tol = Tol::witness();
    let id = RigidMotion::<f64>::identity();
    for (what, taper) in [("apex 100 m off", 0.1), ("apex 10 km off", 1e-3), ("apex 1000 km off", 1e-5)] {
        for rev in [Revolution::Full, Revolution::Partial(2.0)] {
            // radius 1 at y = 0, radius 1 + taper at y = 10: the apex is
            // 10 / taper below.
            let got = try_revolved_about_y_at::<f64>(
                vec![
                    (Point2::new(0.5, 0.0), 0.0),
                    (Point2::new(1.0, 0.0), 0.0),
                    (Point2::new(1.0 + taper, 10.0), 0.0),
                    (Point2::new(0.5, 10.0), 0.0),
                ],
                rev,
                tol,
            );
            match got {
                Ok(body) => {
                    let gate = gate_word(&body, tol);
                    let ask = mean_us(200, || {
                        let _ = topo::session_box_at(&body, &id, tol);
                    });
                    println!("FRUSTUM {what} {rev:?}: gate {} | ask {ask:.1} us", gate.chars().take(150).collect::<String>());
                }
                Err(e) => println!("FRUSTUM {what} {rev:?}: revolve refuses: {}", format!("{e}").chars().take(260).collect::<String>()),
            }
        }
    }
}

/// DELTA PROBE: a ring torus cut by a bar. The piece's torus faces are
/// bounded by spiric edges; does such a face read a window? Stood so the
/// piece is inside the box and the torus's tube pokes out of it.
#[test]
fn review_r2_delta_a_bar_cut_ring_torus_piece_near_the_wall() {
    use sweep::test_support::{brick, finished, revolved_about_y};
    let tol = Tol::witness();
    let (big_r, r) = (700.0, 50.0);
    let torus = revolved_about_y(
        vec![
            (Point2::new(big_r - r, 0.0), 1.0),
            (Point2::new(big_r + r, 0.0), 1.0),
        ],
        Revolution::Full,
        tol,
    );
    let moved = topo::transform_rigid(
        &torus,
        &RigidMotion::translation(Vec3::new(0.0, 300.0, 0.0)).unwrap(),
    )
    .unwrap();
    let torus = finished("the torus", moved, tol);
    let bar = finished(
        "the bar",
        brick::<f64>((100.0, 400.0), (200.0, 400.0), (400.0, 800.0), tol),
        tol,
    );
    let cut = match topo::intersect(&torus, &bar, tol) {
        Ok(topo::BooleanResult::Body(b)) => b.body,
        other => {
            println!("CUT: intersect gave {other:?}");
            return;
        }
    };
    let far = |b: &topo::Body<f64>| {
        b.vertex_points()
            .map(|(_, q)| q.x.abs().max(q.y.abs()).max(q.z.abs()))
            .fold(0.0, f64::max)
    };
    println!("CUT at home: vertices reach {:.3}; gate {}", far(&cut), gate_word(&cut, tol));
    // The piece lies in the bar: x in [100, 400], y in [200, 400], z in
    // [400, 800]. The torus's tube reaches |x|, |z| <= 750, y in [250, 350].
    for (dx, dz) in [(0.0, 0.0), (200.0, 0.0), (300.0, 0.0), (400.0, 0.0), (0.0, 150.0), (0.0, 195.0)] {
        let at = RigidMotion::translation(Vec3::new(dx, 0.0, dz)).unwrap();
        let image = topo::transform_rigid(&cut, &at).unwrap();
        println!(
            "CUT moved ({dx}, 0, {dz}): the bar's far coordinate {}, the tube's {}, vertices {:.3}: gate {}",
            (400.0 + dx).max(800.0 + dz),
            (750.0 + dx).max(750.0 + dz),
            far(&image),
            gate_word(&image, tol).chars().take(170).collect::<String>()
        );
    }
}

/// DELTA PROBE: the slender frustum and seamless-band claims, at the gate.
#[test]
fn review_r2_delta_how_an_uncovered_face_is_said() {
    let tol = Tol::witness();
    // A ball wedge as before, the sentence of the limit.
    let wedge = try_revolved_about_y_at::<f64>(
        vec![(Point2::new(0.0, -30.0), 1.0), (Point2::new(0.0, 30.0), 0.0)],
        Revolution::Partial(1.1),
        tol,
    )
    .unwrap();
    let band = geom_core::Band::linear(tol).unwrap();
    let turn = RigidMotion::rotation(
        UnitVec3::new(Vec3::new(0.0, 1.0, 0.0), "y", band).unwrap(),
        core::f64::consts::PI,
    )
    .unwrap();
    let at = RigidMotion::translation(Vec3::new(990.0, 0.0, 0.0)).unwrap().compose(turn).unwrap();
    let image = topo::transform_rigid(&wedge, &at).unwrap();
    println!("WEDGE: {}", gate_word(&image, tol));
}

/// DELTA 2 PROBE: a slender cone ring cut by planes parallel to its axis
/// (hyperbola sections), and a ball cut by a plane tilted to its poles.
#[test]
fn review_r2_delta2_cone_cut_along_its_axis_and_a_tilted_ball_cut() {
    use sweep::test_support::{ball_poled_y, brick, finished};
    let tol = Tol::witness();
    let band = geom_core::Band::linear(tol).unwrap();
    let ring = try_revolved_about_y_at::<f64>(
        vec![
            (Point2::new(0.09, 0.0), 0.0),
            (Point2::new(0.1, 0.0), 0.0),
            (Point2::new(0.1001, 0.1), 0.0),
            (Point2::new(0.09, 0.1), 0.0),
        ],
        Revolution::Full,
        tol,
    )
    .unwrap();
    let at = RigidMotion::translation(Vec3::new(980.0, 0.0, 0.0))
        .unwrap()
        .compose(
            RigidMotion::rotation(
                UnitVec3::new(Vec3::new(0.0, 0.0, 1.0), "z", band).unwrap(),
                core::f64::consts::FRAC_PI_2,
            )
            .unwrap(),
        )
        .unwrap();
    for (what, slab) in [
        ("slab along the axis, off it", brick::<f64>((0.03, 0.2), (-0.2, 0.3), (-0.2, 0.2), tol)),
        ("slab along the axis, through it", brick::<f64>((-0.02, 0.2), (-0.2, 0.3), (-0.2, 0.2), tol)),
        ("thin slab along the axis", brick::<f64>((0.095, 0.2), (-0.2, 0.3), (-0.2, 0.2), tol)),
        ("corner", brick::<f64>((0.02, 0.2), (0.03, 0.3), (0.01, 0.2), tol)),
    ] {
        let r = finished("ring", ring.clone(), tol);
        let s = finished("slab", slab, tol);
        for (op, got) in [("intersect", topo::intersect(&r, &s, tol)), ("subtract", topo::subtract(&r, &s, tol))] {
            match got {
                Ok(topo::BooleanResult::Body(b)) => {
                    let image = topo::transform_rigid(&b.body, &at).unwrap();
                    println!(
                        "CONECUT {what} {op}: home {} | apex 100 m past the wall: {}",
                        gate_word(&b.body, tol).chars().take(60).collect::<String>(),
                        gate_word(&image, tol).chars().take(150).collect::<String>()
                    );
                }
                other => println!("CONECUT {what} {op}: {}", format!("{other:?}").chars().take(160).collect::<String>()),
            }
        }
    }
    // A ball poled on y cut by a z-plane and by a tilted slab.
    let ball = finished("ball", ball_poled_y::<f64>(50.0, Vec3::new(0.0, 0.0, 0.0), tol), tol);
    let tilted = topo::transform_rigid(
        &brick::<f64>((-100.0, 100.0), (-100.0, 100.0), (-100.0, 10.0), tol),
        &RigidMotion::rotation(UnitVec3::new(Vec3::new(1.0, 0.0, 0.0), "x", band).unwrap(), 0.4).unwrap(),
    )
    .unwrap();
    for (what, cutter) in [
        ("z-plane cut", brick::<f64>((-100.0, 100.0), (-100.0, 100.0), (-100.0, 10.0), tol)),
        ("tilted cut", tilted),
    ] {
        match topo::intersect(&ball, &finished("cutter", cutter, tol), tol) {
            Ok(topo::BooleanResult::Body(b)) => {
                // The kept part lies below the cut; stand it so the cut-off
                // cap's side of the ball pokes out along +z.
                for lift in [940.0, 960.0, 985.0] {
                    let m = RigidMotion::translation(Vec3::new(0.0, 0.0, lift)).unwrap();
                    let image = topo::transform_rigid(&b.body, &m).unwrap();
                    let far = image
                        .vertex_points()
                        .map(|(_, q)| q.x.abs().max(q.y.abs()).max(q.z.abs()))
                        .fold(0.0, f64::max);
                    println!(
                        "BALLCUT {what} lifted {lift} (ball's top {}), vertices reach {far:.2}: {}",
                        lift + 50.0,
                        gate_word(&image, tol).chars().take(140).collect::<String>()
                    );
                }
            }
            other => println!("BALLCUT {what}: {}", format!("{other:?}").chars().take(200).collect::<String>()),
        }
    }
}
