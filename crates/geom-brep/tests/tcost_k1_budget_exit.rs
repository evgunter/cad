//! **TCOST-K1 — the patch lanes' budget exit.** One row per way a
//! round loop ends, on both lanes and both rational arms, and the
//! width the early refusal carries against the width the schedule
//! itself reaches.
//!
//! The rows drive the public door `nurbs_patch_face` with an EXPLICIT
//! ε and band, so each states its claim at a tolerance of its own
//! choosing rather than the run's — the exit is a function of (face,
//! ε, band), and a row that wants the schedule to run out picks the
//! band that makes it.
//!
//! The "schedule's own width" is measured LIVE, not pinned: a face
//! whose last round lands inside a band's coincidence zone is
//! classified zero there and falls through to the ordinary
//! exhaustion refusal, whose payload is the last round's measured
//! width. The band is chosen from the early exit's own payload, so
//! the comparison needs no number from outside the run.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::f64::consts::{FRAC_1_SQRT_2, PI};
use std::time::Instant;

use geom_brep::props::PropsError;
use geom_brep::props::quad::{FaceCutBounds, nurbs_patch_face};
use geom_core::Band;
use geom_core::ring_interval::RingInterval;
use geom_core::spline::KnotVector;

/// The meter's target factor (`QUAD_TARGET_LEN_FACTOR`), restated
/// where the rows read it.
const TARGET_LEN_FACTOR: f64 = 1024.0;
/// The funnel's escalation multiple (K), for a band shaped like the
/// run's own `Band::linear`.
const K: f64 = 10.0;

fn p(x: f64, y: f64, z: f64) -> [RingInterval; 3] {
    [
        RingInterval::point(x),
        RingInterval::point(y),
        RingInterval::point(z),
    ]
}

/// A band shaped like `Band::linear` at an explicit ε.
fn linear_band(eps: f64) -> Band {
    Band::new(eps, K * eps).unwrap()
}

struct Face {
    name: &'static str,
    ku: KnotVector,
    kv: KnotVector,
    net: Vec<[RingInterval; 3]>,
    weights: Vec<f64>,
    perimeter: f64,
}

impl Face {
    /// One call through the public door; returns the outcome and
    /// the seconds it took (printed, never asserted — cost is
    /// reported, not thresholded).
    fn run(&self, eps: f64, band: Band) -> (Result<FaceCutBounds, PropsError>, f64) {
        let (ua, ub) = self.ku.domain();
        let (va, vb) = self.kv.domain();
        let t0 = Instant::now();
        let out = nurbs_patch_face::<f64>(
            &self.ku,
            &self.kv,
            &self.net,
            &self.weights,
            (ua, ub, va, vb),
            self.perimeter,
            0.0,
            eps,
            band,
        );
        let secs = t0.elapsed().as_secs_f64();
        (out, secs)
    }
}

/// The budget refusal's payload, or a labelled panic.
fn budget_width(name: &str, out: &Result<FaceCutBounds, PropsError>) -> (f64, f64) {
    match out {
        Err(PropsError::QuadratureBudget {
            width_len,
            target_len,
        }) => (*width_len, *target_len),
        other => panic!("{name}: expected the typed budget refusal, got {other:?}"),
    }
}

// ---------- carriers ----------

/// Quarter torus (R = 2, r = 0.5), biquadratic rational — the
/// midpoint arm (weights vary in both directions).
fn quarter_torus() -> Face {
    let kv2 = KnotVector::unit_segment(2);
    let (rr, r) = (2.0, 0.5);
    let prof = [(rr + r, 0.0), (rr + r, r), (rr, r)];
    let pw = [1.0, FRAC_1_SQRT_2, 1.0];
    let mut net = Vec::new();
    let mut weights = Vec::new();
    for (k, (x, z)) in prof.iter().enumerate() {
        net.push(p(*x, 0.0, *z));
        net.push(p(*x, *x, *z));
        net.push(p(0.0, *x, *z));
        for wj in [1.0, FRAC_1_SQRT_2, 1.0] {
            weights.push(pw[k] * wj);
        }
    }
    Face {
        name: "quarter torus",
        ku: kv2.clone(),
        kv: kv2,
        net,
        weights,
        perimeter: 2.0 * (PI / 2.0) * (rr + r) + 2.0 * (PI / 2.0) * r,
    }
}

/// A half cylinder (r = 1, h = 2) as the standard two-span quadratic
/// with its double knot at `knot` — the exact-v arm (weights uniform
/// in v). At `knot = 0.5` the knot sits on the schedule's grid and
/// every cell splits evenly; off the grid, the cells abutting it do
/// not, and the remainder decays more slowly than 4× per round there.
fn half_cylinder(name: &'static str, knot: f64) -> Face {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, knot, knot, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::unit_segment(1);
    let h = 2.0;
    let net = vec![
        p(1.0, 0.0, 0.0),
        p(1.0, 0.0, h),
        p(1.0, 1.0, 0.0),
        p(1.0, 1.0, h),
        p(0.0, 1.0, 0.0),
        p(0.0, 1.0, h),
        p(-1.0, 1.0, 0.0),
        p(-1.0, 1.0, h),
        p(-1.0, 0.0, 0.0),
        p(-1.0, 0.0, h),
    ];
    let w2 = FRAC_1_SQRT_2;
    Face {
        name,
        ku,
        kv,
        net,
        weights: vec![1.0, 1.0, w2, w2, 1.0, 1.0, w2, w2, 1.0, 1.0],
        perimeter: 2.0 * h + 2.0 * PI,
    }
}

/// Quarter cylinder (r = 1, h = 2), single span: certifies at the
/// default ε with room (flux = π/2·r²h, area = π/2·r·h).
fn quarter_cylinder() -> Face {
    let (r, h) = (1.0, 2.0);
    let w2 = FRAC_1_SQRT_2;
    Face {
        name: "quarter cylinder",
        ku: KnotVector::unit_segment(2),
        kv: KnotVector::unit_segment(1),
        net: vec![
            p(r, 0.0, 0.0),
            p(r, 0.0, h),
            p(r, r, 0.0),
            p(r, r, h),
            p(0.0, r, 0.0),
            p(0.0, r, h),
        ],
        weights: vec![1.0, 1.0, w2, w2, 1.0, 1.0],
        perimeter: 2.0 * h + PI * r,
    }
}

/// A degree-5 unit-weight patch over the unit square — past the
/// exact per-span rule's degree window, so the INTEGRAL lane's
/// composite rounds run. `z(i, j)` sets the control heights; the
/// abscissae `i/5` make `x(u) = u` and `y(v) = v` exactly.
fn quintic(name: &'static str, z: impl Fn(usize, usize) -> f64) -> Face {
    let kv = KnotVector::clamped(vec![0.0; 6].into_iter().chain([1.0; 6]).collect(), 5).unwrap();
    let mut net = Vec::new();
    for i in 0..6 {
        for j in 0..6 {
            #[allow(clippy::cast_precision_loss)]
            net.push(p(i as f64 / 5.0, j as f64 / 5.0, z(i, j)));
        }
    }
    Face {
        name,
        ku: kv.clone(),
        kv,
        net,
        weights: vec![1.0; 36],
        perimeter: 4.0,
    }
}

// ---------- the rows ----------

/// **Certified early** — the certify exit is untouched: the
/// single-span quarter cylinder certifies at ε = 1e-9 with its closed
/// forms inside both brackets, and the quintic plane (whose Taylor
/// remainder is identically zero) certifies on the integral lane's
/// composite at round 0 containing its exact flux and area.
#[test]
fn certify_exit_is_untouched() {
    let eps = 1e-9;
    let cyl = quarter_cylinder();
    let (out, secs) = cyl.run(eps, linear_band(eps));
    let fb = out.unwrap_or_else(|e| panic!("{}: must certify at ε = 1e-9, got {e}", cyl.name));
    println!("K1 {}: certified in {secs:.3}s", cyl.name);
    let (r, h) = (1.0, 2.0);
    assert!(
        fb.flux.contains(PI / 2.0 * r * r * h),
        "{}: closed-form flux outside the certified bracket",
        cyl.name
    );
    assert!(
        fb.area.contains(PI / 2.0 * r * h),
        "{}: closed-form area outside the certified bracket",
        cyl.name
    );

    let c = 0.75;
    let plane = quintic("quintic plane z = 0.75", |_, _| c);
    let (out, secs) = plane.run(eps, linear_band(eps));
    let fb = out.unwrap_or_else(|e| panic!("{}: must certify, got {e}", plane.name));
    println!("K1 {}: certified in {secs:.3}s", plane.name);
    assert!(
        fb.flux.contains(c),
        "{}: exact flux {c} outside [{:e}, {:e}]",
        plane.name,
        fb.flux.lo(),
        fb.flux.hi()
    );
    assert!(
        fb.area.contains(1.0),
        "{}: exact area 1 outside [{:e}, {:e}]",
        plane.name,
        fb.area.lo(),
        fb.area.hi()
    );
}

/// **Refused early on the last-round bound**: the quarter torus at
/// ε = 1e-9 cannot certify (the schedule's last round reaches
/// 1.12× the target), so the loop refuses after round 0. The typed
/// class is the budget refusal, the target is `1024·ε`, and the
/// payload is the schedule's own last-round width to within 1e-3.
#[test]
fn refused_early_on_the_last_round_bound() {
    let eps = 1e-9;
    let torus = quarter_torus();
    let (out, secs) = torus.run(eps, linear_band(eps));
    let (width, target) = budget_width(torus.name, &out);
    println!(
        "K1 {}: early budget refusal width {width:e} in {secs:.3}s",
        torus.name
    );
    assert!(
        (target - TARGET_LEN_FACTOR * eps).abs() <= target * 1e-12,
        "{}: the refused target must be 1024·ε: {target:e}",
        torus.name
    );
    assert!(
        width.is_finite() && width > target,
        "{}: the payload must be a finite width that really missed: {width:e} vs {target:e}",
        torus.name
    );
    // The schedule's own last-round width on this carrier, as the
    // merge base measured it running all eight rounds (the same number
    // `review_r1_rational_probes` pins as the carrier's floor).
    let schedule_width = 1.1461e-6;
    assert!(
        (width - schedule_width).abs() <= 1e-3 * schedule_width,
        "{}: the early refusal's width {width:e} is not the schedule's own {schedule_width:e}",
        torus.name
    );
}

/// Drive `face` to the early exit at ε = 1e-9, then again under a
/// band whose coincidence zone the last round lands in, so the bound
/// cannot fire and the schedule runs out honestly; return both
/// payloads (early, exhausted) and the two wall times.
fn early_then_exhausted(face: &Face) -> ((f64, f64), (f64, f64)) {
    let eps = 1e-9;
    let (early, secs_early) = face.run(eps, linear_band(eps));
    let (bound, target) = budget_width(face.name, &early);
    assert!(
        bound > target,
        "{}: the early refusal must carry a width over its target: {bound:e} vs {target:e}",
        face.name
    );
    // A target 1e-3 above the bound, with a coincidence zone of 3e-3
    // around it: rounds 0–6 read negative (≥ 4× the last), the bound
    // reads inside the zone (so it never fires), and the last round
    // reads zero — falling through to the exhaustion refusal.
    let target = bound * (1.0 + 1e-3);
    let band = Band::new(3e-3 * bound, 5e-3 * bound).unwrap();
    let (late, secs_late) = face.run(target / TARGET_LEN_FACTOR, band);
    let (measured, _) = budget_width(face.name, &late);
    // The exhaustion row proper: the second refusal must be the
    // schedule's own, not the early exit firing again. The two are
    // the same typed variant, but their payloads cannot coincide —
    // the bound omits the midpoint sum's width and carries a
    // `1 − 2⁻³⁰` factor, so a measured last round is STRICTLY wider.
    assert!(
        measured > bound,
        "{}: the coincidence band was meant to run the schedule out, but the refusal \
         carries the early exit's own bound {bound:e} (measured {measured:e})",
        face.name
    );
    println!(
        "K1 {}: early exit width {bound:e} in {secs_early:.3}s; schedule's own width {measured:e} \
         in {secs_late:.3}s ({:.2e} relative apart)",
        face.name,
        (measured - bound) / measured
    );
    ((bound, secs_early), (measured, secs_late))
}

/// **Refused at exhaustion because the bound never fired**, and the
/// width row in one: on faces that exhaust the schedule, the early
/// refusal's width is NO LARGER than the width the schedule itself
/// reaches (the bound omits only the midpoint sum's width) and is
/// that width to within 1e-3 — on a face whose cells all split
/// evenly (double knot on the grid) and on one whose cells abutting
/// an off-grid knot do not, where a plain `R_0/4⁷` projection would
/// read a quarter low.
#[test]
fn early_refusal_width_is_the_schedules_own() {
    for face in [
        half_cylinder("half cylinder, knot on the grid", 0.5),
        half_cylinder("half cylinder, knot off the grid", 0.3),
    ] {
        let ((bound, _), (measured, _)) = early_then_exhausted(&face);
        assert!(
            bound <= measured,
            "{}: the early refusal's width {bound:e} exceeds the schedule's own {measured:e}",
            face.name
        );
        assert!(
            measured - bound <= 1e-3 * measured,
            "{}: the early refusal's width {bound:e} is more than 1e-3 under the schedule's \
             own {measured:e}",
            face.name
        );
    }
}

/// The same three rows on the INTEGRAL lane's composite (a quintic
/// bump, unit weights, past the exact rule's degree window): refused
/// early at a tight ε, refused at exhaustion under the coincidence
/// band, and the early width is the schedule's own.
#[test]
fn integral_composite_lane_exits_the_same_way() {
    let bump = quintic("quintic bump", |i, j| {
        if (2..=3).contains(&i) && (2..=3).contains(&j) {
            0.6
        } else {
            0.0
        }
    });
    let ((bound, _), (measured, _)) = early_then_exhausted(&bump);
    assert!(
        bound <= measured,
        "{}: the early refusal's width {bound:e} exceeds the schedule's own {measured:e}",
        bump.name
    );
    assert!(
        measured - bound <= 1e-3 * measured,
        "{}: the early refusal's width {bound:e} is more than 1e-3 under the schedule's own \
         {measured:e}",
        bump.name
    );
}
