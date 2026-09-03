//! R2 review probes for BOOL-2 (issue 1011, cone half). Independent
//! attacks on the ray×cone arm: an analytic ORACLE sweep over three
//! cone bodies (differently shaped from the delivered rows, which are
//! hand-picked points), the apex posture, the trim-window edges, a
//! fixture built so a schedule ray is exactly generator-parallel, the
//! probe-offset clamp's saturation, and an e2e through public doors.
//!
//! These are reviewer probes, not acceptance rows.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common;

use geom_core::{Point3, Tol, Vec3};
use profile::{ProfileLoop, RawLoop};
use revolve_common::*;
use sweep::{Extrusion, Revolution, extrude, revolve};
use topo::{Body, PointInSolidError, SolidContainment, point_in_solid};

use profile::{Profile, SketchPlane};

fn band() -> geom_core::Band {
    geom_core::Band::linear(Tol::witness()).unwrap()
}

fn triangle() -> ProfileLoop<f64> {
    ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(0.0, 1.0)])
}

fn cone() -> Body<f64> {
    revolve(
        &validated(vec![triangle()]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

fn frustum() -> Body<f64> {
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(0.5, 1.0), p2(0.0, 1.0)]);
    revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

fn quarter_cone() -> Body<f64> {
    revolve(
        &validated(vec![triangle()]),
        axis_y(),
        Revolution::Partial(core::f64::consts::FRAC_PI_2),
        Tol::witness(),
    )
    .unwrap()
    .body
}

fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    let lp = ProfileLoop::polygon([p2(x.0, y.0), p2(x.1, y.0), p2(x.1, y.1), p2(x.0, y.1)]);
    let plane = SketchPlane::new(geom_core::Affine3::translation(Vec3::new(0.0, 0.0, z.0)));
    extrude(
        &Profile::new(plane, vec![lp])
            .validate(Tol::witness())
            .unwrap(),
        Extrusion::Distance(z.1 - z.0),
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// A deterministic low-discrepancy sample of a box (golden-ratio
/// additive recurrence): a differently-SHAPED sweep from the delivered
/// rows' hand-chosen points, and reproducible without a seed knob.
fn samples(n: usize, lo: (f64, f64, f64), hi: (f64, f64, f64)) -> Vec<Point3<f64>> {
    let a = [
        0.754_877_666_246_692_8_f64,
        0.569_840_290_998_020_3,
        0.902_836_195_192_641_2,
    ];
    let mut out = Vec::with_capacity(n);
    for i in 1..=n {
        let f = |k: usize| ((i as f64) * a[k]).fract();
        out.push(Point3::new(
            lo.0 + (hi.0 - lo.0) * f(0),
            lo.1 + (hi.1 - lo.1) * f(1),
            lo.2 + (hi.2 - lo.2) * f(2),
        ));
    }
    out
}

/// Signed clearance of `q` from the boundary of the FULL cone
/// (apex (0,1,0), base disc radius 1 at y=0): positive inside.
fn clear_cone(q: Point3<f64>) -> f64 {
    let rho = (q.x * q.x + q.z * q.z).sqrt();
    // wall `rho + y - 1 = 0`, |grad| = sqrt(2); base `y = 0`.
    let wall = (1.0 - q.y - rho) / 2.0_f64.sqrt();
    wall.min(q.y)
}

/// Same for the frustum (base radius 1 at y=0, top radius 0.5 at y=1).
fn clear_frustum(q: Point3<f64>) -> f64 {
    let rho = (q.x * q.x + q.z * q.z).sqrt();
    let wall = (1.0 - q.y / 2.0 - rho) / 1.25_f64.sqrt();
    wall.min(q.y).min(1.0 - q.y)
}

/// Same for the quarter cone: the full cone intersected with the
/// quadrant `x > 0, z < 0` (fan walls `x = 0` and `z = 0`).
fn clear_quarter(q: Point3<f64>) -> f64 {
    clear_cone(q).min(q.x).min(-q.z)
}

/// Runs the oracle over one body and reports mismatches and
/// escalations. `margin` is the band of points skipped as too close to
/// the boundary for the oracle to be a fair judge.
/// Returns `(judged, escalations, WRONG answers, escalation notes)`.
///
/// **The two failure kinds are kept apart, and only one of them is a
/// disagreement.** A wrong verdict is the arm answering the oracle's
/// question differently from the oracle: that is the claim this sweep
/// exists to test, and it must be empty. An escalation is the arm
/// DECLINING to answer, which at a coarse ε is correct behaviour — a
/// ray can be near-tangent to the cone while the query point sits far
/// from the boundary, because the discriminant's band is a property of
/// the RAY, not of the point's clearance. Folding those into one bucket
/// and asserting it empty makes the row assert "never escalates", which
/// is not true of any margined arm and is not what the sweep measured:
/// at ε = 1e-6 three of 294 points on the flat cone escalate on
/// `bool_ray_cone_disc` at clearances of 0.14–0.22, with no wrong
/// answer anywhere. The callers assert emptiness of the WRONG list and
/// a bound on the escalation RATE, so a real degradation — the arm
/// escalating everywhere — still goes red.
fn oracle(
    name: &str,
    body: &Body<f64>,
    pts: &[Point3<f64>],
    truth: impl Fn(Point3<f64>) -> f64,
    margin: f64,
) -> (usize, usize, Vec<String>, Vec<String>) {
    let mut tested = 0usize;
    let mut escalated = 0usize;
    let mut bad = Vec::new();
    let mut notes = Vec::new();
    for &q in pts {
        let c = truth(q);
        if c.abs() < margin {
            continue;
        }
        tested += 1;
        match point_in_solid(body, q, band(), Tol::witness()) {
            Ok(v) => {
                let want = if c > 0.0 {
                    SolidContainment::In
                } else {
                    SolidContainment::Out
                };
                if v != want {
                    bad.push(format!(
                        "{name}: {q:?} clearance {c:+.4} -> {v:?}, oracle says {want:?}"
                    ));
                }
            }
            Err(e) => {
                escalated += 1;
                // Reported, never counted as a disagreement — see the
                // header. Worth seeing at a point this far out.
                if escalated <= 5 {
                    notes.push(format!(
                        "{name}: ESCALATED at {q:?} clearance {c:+.4}: {e:?}"
                    ));
                }
            }
        }
    }
    (tested, escalated, bad, notes)
}

/// **Claim 1, by execution.** The quadratic + nappe + trim, judged
/// against a closed-form oracle over a low-discrepancy cloud rather
/// than at hand-chosen points.
#[test]
fn r2_oracle_sweep_over_three_cone_bodies() {
    let pts = samples(600, (-1.6, -0.6, -1.6), (1.6, 1.8, 1.6));
    let mut all = Vec::new();
    let mut all_notes: Vec<String> = Vec::new();
    let mut total = 0usize;
    let mut esc = 0usize;
    for (name, body, truth) in [
        (
            "cone",
            cone(),
            Box::new(clear_cone) as Box<dyn Fn(Point3<f64>) -> f64>,
        ),
        ("frustum", frustum(), Box::new(clear_frustum)),
        ("quarter", quarter_cone(), Box::new(clear_quarter)),
    ] {
        let (t, e, bad, notes) = oracle(name, &body, &pts, truth, 0.02);
        total += t;
        esc += e;
        all.extend(bad);
        all_notes.extend(notes);
    }
    println!(
        "r2 oracle: {total} points judged, {esc} escalations, {} wrong answers",
        all.len()
    );
    for m in all.iter().take(25) {
        println!("  {m}");
    }
    for m in all_notes.iter().take(10) {
        println!("  {m}");
    }
    assert!(
        esc * 20 <= total,
        "{esc} of {total} points escalated — the arm is declining to answer at a \
         rate that is no longer a near-tangent ray here and there"
    );
    assert!(all.is_empty(), "{} oracle disagreements", all.len());
}

/// **Claim 2, by execution.** The apex posture: ON the apex, and near
/// it inside and outside, at several scales.
#[test]
fn r2_apex_posture() {
    let body = cone();
    let at_apex = point_in_solid(&body, Point3::new(0.0, 1.0, 0.0), band(), Tol::witness());
    println!("r2 apex: exactly at the apex -> {at_apex:?}");
    for d in [1e-1_f64, 1e-2, 1e-3, 1e-4, 1e-5, 1e-6, 1e-7, 1e-8] {
        let below = point_in_solid(
            &body,
            Point3::new(0.0, 1.0 - d, 0.0),
            band(),
            Tol::witness(),
        );
        let above = point_in_solid(
            &body,
            Point3::new(0.0, 1.0 + d, 0.0),
            band(),
            Tol::witness(),
        );
        // Off-axis, just inside the wall near the tip.
        let rho = (1.0 - (1.0 - d)) * 0.5;
        let side = point_in_solid(
            &body,
            Point3::new(rho, 1.0 - d, 0.0),
            band(),
            Tol::witness(),
        );
        println!("r2 apex d={d:e}: below={below:?} above={above:?} justinside={side:?}");
    }
    // The PR's stated posture for the query point AT the apex.
    assert_eq!(
        at_apex.as_ref().ok(),
        Some(&SolidContainment::OnBoundary),
        "the PR states the pre-pass reads an apex hit as OnBoundary"
    );
}

/// **Claim 3, by execution.** Is the generator-parallel leading
/// coefficient reachable? Build a cone whose half-angle makes schedule
/// member `[0.5, 0.25, 1.0]` exactly parallel to a generator:
/// `cos²α = 0.25² / (0.5² + 0.25² + 1²) = 1/21`, which a base radius 1
/// and apex height `1/√20` produce.
#[test]
fn r2_generator_parallel_schedule_member() {
    let h = 1.0 / 20.0_f64.sqrt();
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(0.0, h)]);
    let body = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body;
    // The arithmetic the arm does: A = (d·â)² − cos²α for that member.
    let d = Vec3::new(0.5, 0.25, 1.0).normalize();
    let cos2 = h * h / (1.0 + h * h);
    let a2 = d.y * d.y - cos2;
    println!(
        "r2 generator-parallel: A = {a2:e} (band ~ {:e})",
        10.0 * Tol::witness().get().eps
    );
    assert!(
        a2.abs() < 1e-12,
        "the fixture must make that schedule member generator-parallel: A = {a2:e}"
    );
    // The door must still answer correctly: the arm abandons that ray
    // and the schedule retries.
    let pts = samples(300, (-1.6, -0.4, -1.6), (1.6, 0.8, 1.6));
    let truth = |q: Point3<f64>| {
        let rho = (q.x * q.x + q.z * q.z).sqrt();
        // wall through (1,0) and (0,h): rho/1 + y/h = 1.
        let wall = (1.0 - q.y / h - rho) / (1.0 + 1.0 / (h * h)).sqrt();
        wall.min(q.y)
    };
    let (t, e, bad, notes) = oracle("flat-cone", &body, &pts, truth, 0.02);
    println!(
        "r2 generator-parallel body: {t} judged, {e} escalations, {} wrong answers",
        bad.len()
    );
    for m in bad.iter().chain(notes.iter()).take(10) {
        println!("  {m}");
    }
    assert!(
        bad.is_empty(),
        "{} disagreements on the flat cone",
        bad.len()
    );
    // Escalations are correct behaviour on a near-tangent ray and are
    // ε-dependent: none at the default row, three of 294 at 1e-6. The
    // RATE is what a degradation would move.
    assert!(
        e * 20 <= t,
        "{e} of {t} points escalated on the flat cone — a rate, not a \
         near-tangent ray here and there"
    );
}

/// **Claim 3/1, by execution.** Points exactly ON the trim window's
/// edges — the frustum's two rims and the quarter cone's fan seams.
#[test]
fn r2_trim_window_edges() {
    let f = frustum();
    for (y, rho) in [(0.0_f64, 1.0_f64), (1.0, 0.5)] {
        for phi in [0.0_f64, 0.9, 2.4, -1.7] {
            let (s, c) = phi.sin_cos();
            let q = Point3::new(rho * c, y, rho * s);
            let r = point_in_solid(&f, q, band(), Tol::witness());
            println!("r2 rim y={y} phi={phi:.2} -> {r:?}");
            assert_eq!(
                r.as_ref().ok(),
                Some(&SolidContainment::OnBoundary),
                "a frustum rim point is on the boundary"
            );
        }
    }
    let qc = quarter_cone();
    // The fan seams of the quarter cone, on the cone wall.
    for (x, y, z) in [(0.5_f64, 0.5_f64, 0.0_f64), (0.0, 0.5, -0.5)] {
        let r = point_in_solid(&qc, Point3::new(x, y, z), band(), Tol::witness());
        println!("r2 quarter seam ({x},{y},{z}) -> {r:?}");
        assert_eq!(
            r.as_ref().ok(),
            Some(&SolidContainment::OnBoundary),
            "a fan-seam point on the cone wall is on the boundary"
        );
    }
}

/// **Claim 4, by execution.** What the delivered rows' probe offset
/// actually is at this ε point — the clamp's saturation.
#[test]
fn r2_probe_offset_clamp_saturation() {
    let eps = Tol::witness().get().eps;
    let raw = 1e6 * eps;
    let clamped = raw.clamp(1e-3, 0.1);
    println!(
        "r2 clamp: eps={eps:e} raw=1e6*eps={raw:e} clamped={clamped:e} saturated={}",
        (raw - clamped).abs() > f64::EPSILON * raw.max(1.0)
    );
    // The band-relative derivation is live only on eps in (1e-9, 1e-7).
    println!(
        "r2 clamp: the derivation governs only for eps in (1e-9, 1e-7); \
         this run's eps is {}",
        if (1e-9..=1e-7).contains(&eps) {
            "INSIDE that window"
        } else {
            "OUTSIDE it — the offset is a constant"
        }
    );
}

/// **Claim 7, by execution.** The out-of-unit PLANAR finding: a
/// revolved rectangle is a cylinder with no cone in it, and a point in
/// the INTERIOR of its base cap.
#[test]
fn r2_planar_base_cap_interior_out_of_unit() {
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(1.0, 1.0), p2(0.0, 1.0)]);
    let cyl = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body;
    for q in [
        Point3::new(0.3, 0.0, 0.2),
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.5, 0.0, 0.0),
        Point3::new(0.0, 0.0, 0.5),
        Point3::new(-0.3, 0.0, -0.2),
        Point3::new(0.3, 1.0, 0.2),
    ] {
        let r = point_in_solid(&cyl, q, band(), Tol::witness());
        println!("r2 planar cap {q:?} -> {r:?}");
    }
    let r = point_in_solid(&cyl, Point3::new(0.3, 0.0, 0.2), band(), Tol::witness());
    assert_eq!(
        r.as_ref().ok(),
        Some(&SolidContainment::Out),
        "reproducing the PR's reported out-of-unit misread (truth is OnBoundary)"
    );
}

/// **e2e through public doors**: revolve a coned solid, boolean it,
/// tessellate the result.
#[test]
fn r2_e2e_cone_boolean_tessellate() {
    let a = quarter_cone();
    let b = brick((5.0, 6.0), (0.0, 1.0), (-1.0, 0.0));
    let out = topo::union(&a, &b, Tol::witness()).expect("the cone arm unlocks this union");
    let result = out.body().expect("a disjoint union is not empty");
    assert_eq!(topo::validate_closed(&result.body), Ok(()));
    let m = mesh::tessellate(&result.body, 0.05, Tol::witness()).expect("tessellates");
    println!(
        "r2 e2e: {} triangles, {} vertices",
        m.patches.iter().map(|p| p.triangles.len()).sum::<usize>(),
        m.positions.len()
    );
    assert!(m.patches.iter().any(|p| !p.triangles.is_empty()));
    assert_eq!(mesh::validate::check_mesh(&m), Ok(()));
}

/// **Claim 3.** Does any escalation site silently answer? Sweep a dense
/// shell of points straddling the cone wall and count the outcomes; a
/// site that answered where it should graze shows up as a wrong side.
#[test]
fn r2_wall_straddle_no_silent_answer() {
    let body = cone();
    let mut counts = (0usize, 0usize, 0usize, 0usize);
    let mut wrong = Vec::new();
    for i in 0..240 {
        let phi = (i as f64) * 0.261_799_387_799_149_4;
        let y = 0.05 + 0.9 * ((i as f64) * 0.754_877_666_246_692_8).fract();
        let rho0 = 1.0 - y;
        for k in [-3.0_f64, -1.0, 1.0, 3.0] {
            let off = k * 1e-3;
            let rho = rho0 + off;
            let (s, c) = phi.sin_cos();
            let q = Point3::new(rho * c, y, rho * s);
            match point_in_solid(&body, q, band(), Tol::witness()) {
                Ok(SolidContainment::In) => {
                    counts.0 += 1;
                    if off > 0.0 {
                        wrong.push(format!("In outside the wall at {q:?} off={off:e}"));
                    }
                }
                Ok(SolidContainment::Out) => {
                    counts.1 += 1;
                    if off < 0.0 {
                        wrong.push(format!("Out inside the wall at {q:?} off={off:e}"));
                    }
                }
                Ok(SolidContainment::OnBoundary) => counts.2 += 1,
                Err(_) => counts.3 += 1,
            }
        }
    }
    println!(
        "r2 straddle: In={} Out={} OnBoundary={} Escalated={}",
        counts.0, counts.1, counts.2, counts.3
    );
    for w in wrong.iter().take(10) {
        println!("  {w}");
    }
    assert!(wrong.is_empty(), "{} wrong-side answers", wrong.len());
}

/// **Claim 3.** Ray exactly THROUGH the apex: pick query points on the
/// axis below the base so that a schedule ray runs straight through the
/// tip, and see that the door does not fabricate a parity.
#[test]
fn r2_rays_through_the_apex() {
    let body = cone();
    for y in [-0.25_f64, -0.5, -1.0, -2.0] {
        let q = Point3::new(0.0, y, 0.0);
        let r = point_in_solid(&body, q, band(), Tol::witness());
        println!("r2 through-apex from {q:?} -> {r:?}");
        // On the axis below the base the +y schedule ray passes through
        // the base disc's centre and exits exactly at the apex.
        match r {
            Ok(SolidContainment::Out) => {}
            Ok(other) => panic!("a point below the base is outside, got {other:?}"),
            Err(PointInSolidError::RayExhausted) => {
                println!("  (escalated RayExhausted — the honest outcome)");
            }
            Err(e) => panic!("unexpected escalation {e:?}"),
        }
    }
}

/// **Claim 4, sharpened.** The near-apex outcome as a function of
/// distance from the apex, at whatever eps this run carries. The
/// delivered rows probe this neighbourhood at `away()`, whose floor the
/// PR justifies as "five orders clear of the widest escalate band".
/// What actually bounds it is the disc margin's QUADRATIC decay toward
/// the apex, which is a much larger radius than the band.
#[test]
fn r2_apex_escalation_shell_vs_the_clamp_floor() {
    let body = cone();
    let eps = Tol::witness().get().eps;
    let away = (1e6 * eps).clamp(1e-3, 0.1);
    let mut answered_from: Option<f64> = None;
    let mut d = 1e-2_f64;
    // Walk outward in eighth-decades and find where answers resume.
    let mut rows = Vec::new();
    for k in 0..40 {
        d = 1e-9 * 10f64.powf(k as f64 / 4.0);
        let r = point_in_solid(
            &body,
            Point3::new(0.0, 1.0 - d, 0.0),
            band(),
            Tol::witness(),
        );
        let tag = match &r {
            Ok(v) => format!("{v:?}"),
            Err(PointInSolidError::RayExhausted) => "RayExhausted".to_string(),
            Err(PointInSolidError::Escalated { diag, .. }) => {
                format!("Escalated({:?})", diag.predicate)
            }
            Err(e) => format!("{e:?}"),
        };
        rows.push(format!("  d={d:9.3e} -> {tag}"));
        if r.is_ok() && matches!(r, Ok(SolidContainment::In)) {
            answered_from = Some(answered_from.unwrap_or(d).min(d));
        } else {
            answered_from = None; // reset: we want the LAST resumption
        }
    }
    for r in &rows {
        println!("{r}");
    }
    println!(
        "r2 apex shell: eps={eps:e} escalate band={:e} away()={away:e} \
         definite-In resumes at {:?}; clearance = away()/resume = {:?}",
        10.0 * eps,
        answered_from,
        answered_from.map(|a| away / a)
    );
    let _ = d;
}

/// **Claim 7, widened.** The PR reports the planar misread on a
/// revolved cylinder's BASE cap. Does it also hit the top cap, and does
/// it hit the cone bodies this unit ships rows for?
#[test]
fn r2_planar_cap_misread_blast_radius() {
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(1.0, 1.0), p2(0.0, 1.0)]);
    let cyl = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body;
    let cone_body = cone();
    let frus = frustum();
    let qc = quarter_cone();
    // Interior points of each planar cap, at a generic azimuth
    // (z != 0, so off the revolve seam).
    for (name, body, pts) in [
        (
            "cylinder",
            &cyl,
            vec![Point3::new(0.3, 0.0, 0.2), Point3::new(0.3, 1.0, 0.2)],
        ),
        ("cone", &cone_body, vec![Point3::new(0.3, 0.0, 0.2)]),
        (
            "frustum",
            &frus,
            vec![Point3::new(0.3, 0.0, 0.2), Point3::new(0.2, 1.0, 0.15)],
        ),
        ("quarter-cone", &qc, vec![Point3::new(0.3, 0.0, -0.2)]),
    ] {
        for q in pts {
            let r = point_in_solid(body, q, band(), Tol::witness());
            println!("r2 cap-misread {name} {q:?} -> {r:?} (truth: OnBoundary)");
        }
    }
}

/// **Claim 6, attacked.** The surviving `KindUnsupported` raise site
/// (`solid_contain.rs:341`, `face_plane`) is dispositioned "a cone
/// reaching it means a caller asked a non-plane face for a plane" — but
/// `join.rs` asks `face_plane(body, region_face)` on a region face
/// during role resolution, which is one of the containment doors issue
/// 1011 names. The delivered rows only exercise the DISJOINT
/// no-crossings fallback. Try genuinely CROSSING cone booleans and see
/// what comes back.
#[test]
fn r2_crossing_cone_booleans_and_the_surviving_raise_site() {
    let mut saw_cone_kind_refusal = Vec::new();
    let cases: Vec<(&str, Body<f64>, Body<f64>)> = vec![
        (
            "quarter-cone ∪ overlapping brick",
            quarter_cone(),
            brick((0.0, 0.6), (0.0, 0.5), (-0.6, 0.0)),
        ),
        (
            "quarter-cone ∪ straddling brick",
            quarter_cone(),
            brick((-0.5, 0.5), (0.2, 0.8), (-0.5, 0.5)),
        ),
        (
            "cone ∪ overlapping brick",
            cone(),
            brick((-0.4, 0.4), (0.1, 0.6), (-0.4, 0.4)),
        ),
        (
            "frustum ∪ overlapping brick",
            frustum(),
            brick((-0.4, 0.4), (0.2, 0.7), (-0.4, 0.4)),
        ),
        (
            "quarter-cone ∪ tall brick",
            quarter_cone(),
            brick((0.1, 0.3), (-0.5, 1.5), (-0.3, -0.1)),
        ),
    ];
    for (name, a, b) in cases {
        let r = topo::union(&a, &b, Tol::witness());
        let tag = match &r {
            Ok(_) => "Ok(assembled)".to_string(),
            Err(topo::BooleanError::Containment(e)) => {
                let s = format!("Containment({e:?})");
                if s.contains("KindUnsupported") && s.contains("Cone") {
                    saw_cone_kind_refusal.push(format!("{name}: {s}"));
                }
                s
            }
            Err(other) => format!("{other:?}"),
        };
        println!("r2 crossing: {name} -> {tag}");
    }
    for s in &saw_cone_kind_refusal {
        println!("r2 CONE KIND REFUSAL STILL REACHABLE: {s}");
    }
    assert!(
        saw_cone_kind_refusal.is_empty(),
        "KindUnsupported{{Cone}} is still reachable from a containment door: {saw_cone_kind_refusal:?}"
    );
}
