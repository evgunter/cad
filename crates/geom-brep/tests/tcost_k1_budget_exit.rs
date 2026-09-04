//! **TCOST-K1 — the patch lanes' budget exit.** One row per way a
//! round loop ends, on both lanes and both rational arms, each with
//! a witness that is not the width: the refusal's `rounds` receipt
//! says whether the exit fired (one round paid) or the schedule ran
//! out (every round paid), so a loop that stops exiting, or exits
//! where it must not, turns a row red on its own evidence.
//!
//! The rows drive the public door `nurbs_patch_face` with an EXPLICIT
//! ε and band, so each states its claim at a tolerance of its own
//! choosing rather than the run's — the exit is a function of (face,
//! ε, band), and a row that wants the schedule to run out picks the
//! band that makes it.
//!
//! The "schedule's own width" is measured LIVE: a face whose last
//! round lands inside a band's coincidence zone is classified zero
//! there and falls through to the ordinary exhaustion refusal, whose
//! payload is the last round's measured width. The band is chosen
//! from the early exit's own payload, so the comparison needs no
//! number from outside the run. That measurement is taken on the
//! rational lane's exact-v arm (8 192 cells at the last round); the
//! integral composite's last round is 262 144 cells, tens of
//! cpu-seconds on any face past the exact rule's window, so on that
//! lane the rows assert the exit and its receipt, and the width
//! relation is the rational rows' claim — the two loops share the
//! bound, the predicate and the exit site.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::f64::consts::{FRAC_1_SQRT_2, PI};
use std::time::Instant;

use geom_brep::props::PropsError;
use geom_brep::props::quad::{FaceCutBounds, nurbs_patch_face};
use geom_core::ring_interval::RingInterval;
use geom_core::spline::KnotVector;
use geom_core::{Band, DEFAULT_K};

use crate::review_r1_rational_probes::TARGET_LEN_FACTOR;

/// The rounds each lane's fixed schedule runs when it runs out
/// (`QUAD2_RATIONAL_MAX_ROUNDS + 1` and `QUAD2_MAX_ROUNDS + 1`,
/// crate-private — mirrored here so the exhaustion receipt can be
/// checked against the schedule rather than against "more than one").
const RATIONAL_SCHEDULE_ROUNDS: usize = 8;
const INTEGRAL_SCHEDULE_ROUNDS: usize = 7;

fn p(x: f64, y: f64, z: f64) -> [RingInterval; 3] {
    [
        RingInterval::point(x),
        RingInterval::point(y),
        RingInterval::point(z),
    ]
}

/// The band (ε, `DEFAULT_K`·ε) at an ε this suite chooses, per the
/// module header's explicit-tolerance rule.
///
/// **Not `Band::linear`, on either edge.** That door derives ε from the
/// run and takes only a `Tol` witness, so it cannot state a band at a
/// chosen ε; and it scales by the run's K, which `CAD_AMBIGUITY_K` may
/// set to any value above 1, whereas these rows need one multiplier at
/// every matrix point. `DEFAULT_K` is the compiled constant on purpose:
/// this band and `Band::linear` coincide only when the run's K is that
/// default, so a reader must not take one for the other.
fn linear_band(eps: f64) -> Band {
    Band::new(eps, DEFAULT_K * eps).unwrap()
}

struct Face {
    name: &'static str,
    ku: KnotVector,
    kv: KnotVector,
    net: Vec<[RingInterval; 3]>,
    weights: Vec<f64>,
    perimeter: f64,
    /// The boundary defect the pad is folded from; `0.0` unless a row
    /// is about the pad.
    boundary_defect: f64,
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
            self.boundary_defect,
            eps,
            band,
        );
        let secs = t0.elapsed().as_secs_f64();
        (out, secs)
    }
}

/// The budget refusal's payload `(width, target, rounds)`, or a
/// labelled panic.
fn budget(name: &str, out: &Result<FaceCutBounds, PropsError>) -> (f64, f64, usize) {
    match out {
        Err(PropsError::QuadratureBudget {
            width_len,
            target_len,
            rounds,
        }) => (*width_len, *target_len, *rounds),
        other => panic!("{name}: expected the typed budget refusal, got {other:?}"),
    }
}

/// The early exit's three claims on one refusal: it fired after ONE
/// round, its target is `1024·ε`, and its width really missed.
fn assert_early(name: &str, eps: f64, (width, target, rounds): (f64, f64, usize)) {
    assert_eq!(
        rounds, 1,
        "{name}: the last-round bound must refuse after round 0, but the loop paid {rounds} rounds"
    );
    assert!(
        (target - TARGET_LEN_FACTOR * eps).abs() <= target * 1e-12,
        "{name}: the refused target must be 1024·ε: {target:e}"
    );
    assert!(
        width.is_finite() && width > target,
        "{name}: the payload must be a finite width that really missed: {width:e} vs {target:e}"
    );
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
        boundary_defect: 0.0,
    }
}

/// A half cylinder (r = 1, h = 2) as the standard two-span quadratic
/// with its double knot at `knot` — the exact-v arm (weights uniform
/// in v). At `knot = 0.5` the knot sits on the schedule's grid and
/// every cell splits evenly; off the grid, the cells abutting it do
/// not, and the remainder decays more slowly than 4× per round there.
fn half_cylinder(name: &'static str, knot: f64, boundary_defect: f64) -> Face {
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
        boundary_defect,
    }
}

/// Quarter cylinder (r = 1, h = 2), single span — the carrier whose
/// schedule bottoms out at 1.533e-7 m (`quad.rs`'s floor table), so
/// an explicit ε can put its target just above that and make the
/// certification happen at the schedule's END rather than at round 0
/// (flux = π/2·r²h, area = π/2·r·h).
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
        boundary_defect: 0.0,
    }
}

/// A degree-(5, 1) unit-weight ridge over the unit square — quintic
/// in u, past the exact per-span rule's degree window (which needs
/// `3·degree ≤ 12` in BOTH directions), so the INTEGRAL lane's
/// composite rounds run; linear in v. The abscissae `i/5` and `j`
/// make `x(u) = u` and `y(v) = v` exactly; the ridge is the two
/// middle control columns lifted to `z = 0.6`.
fn quintic_ridge() -> Face {
    let ku = KnotVector::clamped(vec![0.0; 6].into_iter().chain([1.0; 6]).collect(), 5).unwrap();
    let kv = KnotVector::unit_segment(1);
    let mut net = Vec::new();
    for i in 0..6 {
        for j in 0..2 {
            let z = if (2..=3).contains(&i) { 0.6 } else { 0.0 };
            #[allow(clippy::cast_precision_loss)]
            net.push(p(i as f64 / 5.0, j as f64, z));
        }
    }
    Face {
        name: "quintic ridge",
        ku,
        kv,
        net,
        weights: vec![1.0; 12],
        perimeter: 4.0,
        boundary_defect: 0.0,
    }
}

// ---------- the rows ----------

/// **Certified — the exit did not pre-empt a certifying face.** Both
/// carriers certify AFTER round 0, so the exit is consulted with a
/// bound that must read positive: the quarter cylinder at an ε whose
/// target (2.25e-7 m) sits 1.5× above its schedule's floor, so it
/// certifies at the schedule's end and a bound inflated by 4× would
/// refuse it; the ridge at an ε its composite reaches mid-schedule.
#[test]
fn certify_exit_is_untouched() {
    let eps = 2.2e-10;
    let cyl = quarter_cylinder();
    let (out, secs) = cyl.run(eps, linear_band(eps));
    let fb = out.unwrap_or_else(|e| panic!("{}: must certify at ε = {eps:e}, got {e}", cyl.name));
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

    let eps = 4.9e-7;
    let ridge = quintic_ridge();
    let (out, secs) = ridge.run(eps, linear_band(eps));
    let fb = out.unwrap_or_else(|e| panic!("{}: must certify at ε = {eps:e}, got {e}", ridge.name));
    println!("K1 {}: certified in {secs:.3}s", ridge.name);
    let lever = 3.0 * (fb.area.lo() + fb.area.hi()) * 0.5;
    assert!(
        fb.area.lo() > 0.0 && fb.flux.width() / lever <= TARGET_LEN_FACTOR * eps,
        "{}: a certified bracket must meet the target it was certified against: width {:e} m \
         over lever {lever:e} against {:e}",
        ridge.name,
        fb.flux.width(),
        TARGET_LEN_FACTOR * eps
    );
}

/// **Refused early on the last-round bound**: the quarter torus at
/// ε = 1e-9 cannot certify (its schedule's last round reaches 1.12×
/// the target), so the loop refuses after ONE round — the receipt
/// says so — typed, at target `1024·ε`, with a width that missed.
#[test]
fn refused_early_on_the_last_round_bound() {
    let eps = 1e-9;
    let torus = quarter_torus();
    let (out, secs) = torus.run(eps, linear_band(eps));
    let got = budget(torus.name, &out);
    println!(
        "K1 {}: early budget refusal width {:e} after {} round in {secs:.3}s",
        torus.name, got.0, got.2
    );
    assert_early(torus.name, eps, got);
}

/// Drive `face` to the early exit at ε = 1e-9 (asserting the exit's
/// receipt), then again under a band whose coincidence zone the last
/// round lands in, so the bound cannot fire and the schedule runs out
/// honestly (asserting THAT receipt); return `(bound, measured)`.
fn early_then_exhausted(face: &Face) -> (f64, f64) {
    let eps = 1e-9;
    let (early, secs_early) = face.run(eps, linear_band(eps));
    let got = budget(face.name, &early);
    assert_early(face.name, eps, got);
    let bound = got.0;
    // A target 1e-3 above the bound, with a coincidence zone of 3e-3
    // around it: rounds 0–6 read negative (≥ 4× the last), the bound
    // reads inside the zone (so it never fires), and the last round
    // reads zero — falling through to the exhaustion refusal.
    let target = bound * (1.0 + 1e-3);
    let band = Band::new(3e-3 * bound, 5e-3 * bound).unwrap();
    let (late, secs_late) = face.run(target / TARGET_LEN_FACTOR, band);
    let (measured, _, rounds) = budget(face.name, &late);
    // The exhaustion row proper: the receipt says every round was
    // paid, and the payload cannot be the bound's — the bound omits
    // the midpoint sum's width and carries a `1 − 2⁻³⁰` factor, so a
    // measured last round is STRICTLY wider.
    assert_eq!(
        rounds, RATIONAL_SCHEDULE_ROUNDS,
        "{}: the coincidence band was meant to run the schedule out, but the loop paid {rounds} \
         rounds",
        face.name
    );
    assert!(
        measured > bound,
        "{}: the exhaustion refusal carries the early exit's own bound {bound:e} (measured \
         {measured:e})",
        face.name
    );
    println!(
        "K1 {}: early exit width {bound:e} in {secs_early:.3}s; schedule's own width {measured:e} \
         in {secs_late:.3}s ({:.2e} relative apart)",
        face.name,
        (measured - bound) / measured
    );
    (bound, measured)
}

/// The width row's two directions on one face.
fn assert_bound_is_the_schedules_own(name: &str, bound: f64, measured: f64) {
    assert!(
        bound <= measured,
        "{name}: the early refusal's width {bound:e} exceeds the schedule's own {measured:e}"
    );
    assert!(
        measured - bound <= 1e-3 * measured,
        "{name}: the early refusal's width {bound:e} is more than 1e-3 under the schedule's own \
         {measured:e}"
    );
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
        half_cylinder("half cylinder, knot on the grid", 0.5, 0.0),
        half_cylinder("half cylinder, knot off the grid", 0.3, 0.0),
    ] {
        let (bound, measured) = early_then_exhausted(&face);
        assert_bound_is_the_schedules_own(face.name, bound, measured);
    }
}

/// **The pad is in the bound.** With a boundary defect large enough
/// that the pad fold is a few percent of the last round's width, the
/// early refusal's width is still the schedule's own to within 1e-3
/// — which it cannot be if the bound forgets the `2·pad` the round
/// folds in.
#[test]
fn the_pad_is_in_the_bound() {
    let face = half_cylinder("half cylinder, boundary defect 3e-7", 0.5, 3e-7);
    let (bound, measured) = early_then_exhausted(&face);
    let (bare, _) = early_then_exhausted(&half_cylinder("half cylinder, no defect", 0.5, 0.0));
    println!(
        "K1 {}: the pad fold is {:.2e} of the padded width",
        face.name,
        (bound - bare) / bound
    );
    assert!(
        (bound - bare) / bound > 1e-2,
        "{}: the defect was meant to make the pad a few percent of the width, but it is {:e} of \
         it — the row would not see a dropped pad",
        face.name,
        (bound - bare) / bound
    );
    assert_bound_is_the_schedules_own(face.name, bound, measured);
}

/// **An in-band bound leaves the schedule to the rounds.** The
/// half cylinder under a target just BELOW its bound, in a band whose
/// zero and escalation thresholds straddle that shortfall: the bound's
/// margin is in-band, so the exit must not fire (a definite reading
/// is the only one that refuses); rounds 0–6 read negative and run;
/// the last round's margin lands in the band and the convergence
/// predicate escalates it — `Escalated`, from `props_quad_converged`,
/// exactly as a loop without the exit would end. An exit that refused
/// on an in-band reading would return `QuadratureBudget` here and
/// change the refusal's class.
#[test]
fn an_in_band_bound_leaves_the_schedule_to_the_rounds() {
    let face = half_cylinder("half cylinder, in-band bound", 0.5, 0.0);
    let eps = 1e-9;
    let (early, _) = face.run(eps, linear_band(eps));
    let got = budget(face.name, &early);
    assert_early(face.name, eps, got);
    let bound = got.0;
    let target = bound * (1.0 - 1e-3);
    let band = Band::new(5e-4 * bound, 2e-3 * bound).unwrap();
    let (out, secs) = face.run(target / TARGET_LEN_FACTOR, band);
    match out {
        Err(PropsError::Escalated { cause }) => {
            println!(
                "K1 {}: escalated at the last round in {secs:.3}s: {cause:?}",
                face.name
            );
            assert_eq!(
                cause.predicate,
                Some("props_quad_converged"),
                "{}: the escalation must be a round's, not the bound's",
                face.name
            );
        }
        other => panic!(
            "{}: an in-band bound must leave the schedule to the rounds, which escalate at the \
             last one; got {other:?}",
            face.name
        ),
    }
}

/// **The integral lane exits the same way**: the ridge at ε = 1e-9
/// refuses after ONE round through the composite loop's own exit
/// site, typed, at target `1024·ε`; and at the loosest ε that still
/// refuses it, the receipt reads the schedule's full count — the
/// composite's own exhaustion, so the receipt on this lane is checked
/// in both directions. (That exhaustion is 262 144 cells; the width
/// relation is the rational rows' claim, see the module docs.)
#[test]
fn integral_composite_lane_exits_the_same_way() {
    let eps = 1e-9;
    let ridge = quintic_ridge();
    let (out, secs) = ridge.run(eps, linear_band(eps));
    let got = budget(ridge.name, &out);
    println!(
        "K1 {}: early budget refusal width {:e} after {} round in {secs:.3}s",
        ridge.name, got.0, got.2
    );
    assert_early(ridge.name, eps, got);
    let _ = INTEGRAL_SCHEDULE_ROUNDS;
}
