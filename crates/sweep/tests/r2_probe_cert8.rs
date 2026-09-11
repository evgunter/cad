//! CERT-8 review probes (reviewer lane 8r2).
//!
//! What survives here asserts: the sampled soundness of
//! `chart_stretch_inf`'s certified floors and ceilings and of the
//! `certified_arms` metric contraction, on the unit's acceptance walls
//! and on randomly drawn charts. The digit-dump rows that printed the
//! same walls' `inf`/`sup`/`rho` figures for a one-shot comparison are
//! gone: they held no assertion, so they could not gate, and every
//! bound they printed is asserted by
//! `probe_sampled_bounds_hold_on_the_acceptance_walls` below and by
//! `cert8_r1_probes::probe_loft_wall_digits_and_sampled_soundness` on
//! the same two walls.
//!
//! The randomised half is SPLIT in two, per `memories/test-suite-cost.md`:
//! `probe_written_charts_never_break_the_certified_arms` is the witness
//! set — five charts written down, each of which must enter the mode
//! every run — and `probe_random_charts_never_break_the_certified_arms`
//! is a pure counterexample search on a varying seed with no floor
//! bolted on.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::Tol;
use geom_core::{Affine3, Point2, Vec3};
use profile::{ProfileLoop, ProfileVertex, RawLoop};
use topo::{Body, FaceKey};

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
    assert!(
        min_su >= i.inf_u - 1e-6,
        "{tag}: inf_u is NOT a lower bound"
    );
    assert!(
        min_sv >= i.inf_v - 1e-6,
        "{tag}: inf_v is NOT a lower bound"
    );
    assert!(
        max_su <= i.sup_u + 1e-6,
        "{tag}: sup_u is NOT an upper bound"
    );
    assert!(
        max_sv <= i.sup_v + 1e-6,
        "{tag}: sup_v is NOT an upper bound"
    );
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

/// One chart's worth of the soundness check, shared by the written
/// row below and the randomised one after it.
///
/// Builds the clamped biquadratic net, runs the exact
/// `chart_stretch_inf` + `certified_arms` assembly, and samples the
/// real surface for a violation of any of the four claims. Returns the
/// worst contraction ratio seen, or `None` when the chart never entered
/// the mode — a degenerate net, a floor at machine zero, or a poisoned
/// assembly is a SKIP, not a counterexample.
///
/// `note` names the caller's chart in every message: the written row
/// passes its tag, the randomised one its trial and replay line.
fn check_certified_arms(ctl: &[[f64; 3]; 9], w: &[f64; 9], note: &str) -> Option<f64> {
    use geom::NurbsSurface;
    use geom_core::Point3;
    use geom_core::spline::KnotVector;
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let points: Vec<Point3<f64>> = ctl.iter().map(|c| Point3::new(c[0], c[1], c[2])).collect();
    let surf = NurbsSurface::new(ku, kv, points, w.to_vec()).ok()?;
    let s: Surface<f64> = Surface::Nurbs(std::sync::Arc::new(surf));
    let i = geom_brep::chart_stretch_inf(&s);
    if !(i.inf_u > 1e-6 && i.inf_v > 1e-6) {
        return None;
    }
    let t = (i.sup_u / i.inf_u).powi(2) + (i.sup_v / i.inf_v).powi(2);
    let d = (i.area_inf / (i.inf_u * i.inf_v)).powi(2);
    let root = (t * t - 4.0 * d).max(0.0).sqrt();
    let rho = (2.0 * d / (t + root)).sqrt().min(1.0);
    // NaN must fall into this arm: a chart whose assembly went poison
    // is not a counterexample, it is a skip. Spelled positively so the
    // NaN case is visible rather than hiding behind a negated partial
    // comparison.
    if rho <= 1e-12 || rho.is_nan() {
        return None;
    }
    let (au, av) = (i.inf_u * rho, i.inf_v * rho);
    let Surface::Nurbs(p) = &s else {
        unreachable!()
    };
    let h = 1e-6;
    let n = 25;
    let mut worst = f64::MAX;
    for a in 1..n {
        for b in 1..n {
            let (u, v) = (f64::from(a) / f64::from(n), f64::from(b) / f64::from(n));
            let su = (p.eval(u + h, v) - p.eval(u - h, v)) * (1.0 / (2.0 * h));
            let sv = (p.eval(u, v + h) - p.eval(u, v - h)) * (1.0 / (2.0 * h));
            assert!(
                su.norm() >= i.inf_u - 1e-5,
                "{note}: inf_u {} > |S_u| {} at ({u},{v})",
                i.inf_u,
                su.norm()
            );
            assert!(
                sv.norm() >= i.inf_v - 1e-5,
                "{note}: inf_v {} > |S_v| {} at ({u},{v})",
                i.inf_v,
                sv.norm()
            );
            assert!(
                su.cross(sv).norm() >= i.area_inf - 1e-5,
                "{note}: area_inf {} > |SuxSv| {} at ({u},{v})",
                i.area_inf,
                su.cross(sv).norm()
            );
            for k in 0..32 {
                let th = core::f64::consts::TAU * f64::from(k) / 32.0;
                let (du, dv) = (th.cos(), th.sin());
                let model = (su * du + sv * dv).norm();
                let scaled = ((au * du).powi(2) + (av * dv).powi(2)).sqrt();
                let ratio = model / scaled;
                worst = worst.min(ratio);
                assert!(
                    ratio >= 1.0 - 1e-7,
                    "{note}: NOT a contraction, ratio {ratio} at ({u},{v}) arms ({au},{av})"
                );
            }
        }
    }
    Some(worst)
}

/// **The witness set, written down.** Five biquadratic charts —
/// polynomial and rational, flat and bowed, orthogonal and sheared —
/// that the inf-arm assembly must survive on EVERY run.
///
/// This is the anti-vacuity half of the old single sweep, and it is
/// written out rather than searched for because
/// `memories/test-suite-cost.md` says so: *at least K of class C*, with
/// C concisely constructible, is a witness you can WRITE DOWN, and
/// bolting such a floor onto a counterexample search makes one row
/// carry two obligations of which only one is safe to cut. So the floor
/// lives here, on charts a reader can see, and the search next door
/// carries none.
/// A written chart: its tag, the 3x3 control net, and the nine
/// weights.
type WrittenChart = (&'static str, [[f64; 3]; 9], [f64; 9]);

const WRITTEN_CHARTS: [WrittenChart; 5] = [
    (
        "flat-polynomial",
        [
            [0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 2.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [1.0, 2.0, 0.0],
            [2.0, 0.0, 0.0],
            [2.0, 1.0, 0.0],
            [2.0, 2.0, 0.0],
        ],
        [1.0; 9],
    ),
    (
        "bowed-polynomial",
        [
            [0.0, 0.0, 0.0],
            [0.0, 1.0, 0.3],
            [0.0, 2.0, 0.0],
            [1.0, 0.0, 0.3],
            [1.0, 1.0, -0.4],
            [1.0, 2.0, 0.3],
            [2.0, 0.0, 0.0],
            [2.0, 1.0, 0.3],
            [2.0, 2.0, 0.0],
        ],
        [1.0; 9],
    ),
    (
        "flat-rational",
        [
            [0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 2.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [1.0, 2.0, 0.0],
            [2.0, 0.0, 0.0],
            [2.0, 1.0, 0.0],
            [2.0, 2.0, 0.0],
        ],
        [1.0, 0.5, 1.0, 2.0, 1.0, 0.4, 1.0, 3.0, 1.0],
    ),
    (
        "bowed-rational",
        [
            [0.0, 0.0, 0.0],
            [0.0, 1.0, 0.3],
            [0.0, 2.0, 0.0],
            [1.0, 0.0, 0.3],
            [1.0, 1.0, -0.4],
            [1.0, 2.0, 0.3],
            [2.0, 0.0, 0.0],
            [2.0, 1.0, 0.3],
            [2.0, 2.0, 0.0],
        ],
        [1.0, 0.5, 1.0, 2.0, 1.0, 0.4, 1.0, 3.0, 1.0],
    ),
    (
        // Sheared, so u and v are not orthogonal anywhere: this is the
        // chart the rho contraction is actually about.
        "sheared-rational",
        [
            [0.0, 0.0, 0.0],
            [0.7, 1.0, 0.1],
            [1.4, 2.0, 0.0],
            [1.0, 0.2, 0.2],
            [1.7, 1.2, -0.3],
            [2.4, 2.2, 0.2],
            [2.0, 0.4, 0.0],
            [2.7, 1.4, 0.1],
            [3.4, 2.4, 0.0],
        ],
        [1.0, 0.8, 1.0, 1.6, 1.0, 0.6, 1.0, 2.2, 1.0],
    ),
];

#[test]
fn probe_written_charts_never_break_the_certified_arms() {
    for (tag, ctl, w) in &WRITTEN_CHARTS {
        let worst =
            check_certified_arms(ctl, w, &format!("written chart {tag}")).unwrap_or_else(|| {
                panic!(
                    "written chart {tag} must ENTER the mode — a chart the assembly \
                     declines to floor tests nothing, and this row is the witness set"
                )
            });
        println!("written chart {tag}: worst contraction ratio {worst}");
    }
}

/// **Randomised soundness hunt for the inf-arm assembly.** Build
/// random (rational and polynomial) charts and check the same four
/// claims on each.
///
/// # A counterexample search, and nothing else
///
/// The shape is *for all sampled charts, P(chart)*
/// (`memories/test-suite-cost.md`'s first shape), so the seed VARIES
/// and is logged unconditionally by `fuzz::start`, `CAD_FUZZ_SEED`
/// replays an exact draw, and the replay line is in every message. The
/// count is a multiple of the workspace `CAD_FUZZ_EFFORT` dial rather
/// than a private constant.
///
/// **It carries no anti-vacuity floor.** A floor here would make one
/// row both a counterexample search and a witness claim at once — the
/// mixing the memory names as the trap — and would tie the search's
/// count, which is safe to cut, to a floor which is not.
/// `probe_written_charts_never_break_the_certified_arms` above holds
/// the witness set on charts that are written down and cannot be
/// missed by a draw.
///
/// It ships at the depth it ran at before the harness, because the
/// change filter cannot yet gate a suite to the code it tests; when it
/// can, this row's marker names `geom-brep`'s chart-stretch lane and
/// the count moves to the gated level.
#[test]
fn probe_random_charts_never_break_the_certified_arms() {
    use test_utils::fuzz;
    let mut r = fuzz::start("cert8-random-charts");
    let replay = fuzz::replay();
    let mut entered = 0usize;
    let mut worst_overall = f64::MAX;
    let mut worst_case = String::new();
    for trial in 0..fuzz::scaled(4000) {
        let rational = trial % 2 == 0;
        let mut ctl = [[0.0f64; 3]; 9];
        for (n, slot) in ctl.iter_mut().enumerate() {
            let (i, j) = (n / 3, n % 3);
            #[allow(clippy::cast_precision_loss)]
            let base = (i as f64, j as f64);
            *slot = [
                base.0 + r.range(-0.4, 0.4),
                base.1 + r.range(-0.4, 0.4),
                r.range(-0.6, 0.6),
            ];
        }
        let mut w = [1.0f64; 9];
        if rational {
            for slot in &mut w {
                *slot = r.range(0.3, 3.0);
            }
        }
        let note = format!("trial {trial} rational={rational} — {replay}");
        if let Some(worst) = check_certified_arms(&ctl, &w, &note) {
            entered += 1;
            if worst < worst_overall {
                worst_overall = worst;
                worst_case = note;
            }
        }
    }
    println!(
        "checked {entered} random charts; worst contraction ratio {worst_overall} at {worst_case}"
    );
}
