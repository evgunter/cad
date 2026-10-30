//! R1 adversarial review probes for PR #309 (M8-3 half 2): the
//! rational patch-flux enclosure. Every probe pits the delivered
//! enclosure against an INDEPENDENT dense oracle (plain-f64 Cox-de
//! Boor evaluation + composite Gauss-Legendre), and where a closed
//! form exists, against that too. The contract under test: the true
//! flux/area lie INSIDE the returned brackets, or the call refuses
//! typed. A returned bracket that EXCLUDES the truth is the failure.
//!
//! **The non-answers are held to a contract too** — they are not merely
//! printed. A lane that stops short of the target must stop at its own
//! floor or work wall, still contain the truth, carry a FINITE width
//! that really missed the run's own `1024·ε` target, and land on the
//! carrier's PINNED width (neither stop moves with ε, only the target
//! does); a refusal must be a typed outcome this lane is allowed to
//! produce. A probe that only prints its `Err` asserts nothing: that is how
//! a convergence meter dividing by a cancelled lever reported an `inf`
//! displacement — for a flux enclosure that was still narrowing —
//! through eight rounds and two ε rows of a green suite.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::needless_range_loop,
    clippy::useless_vec
)]

use crate::shared::patch::{QUAD_TARGET_LEN_FACTOR, Unmet, at_target, patch_net, to_target};
use core::num::NonZeroUsize;
use geom_brep::props::PropsError;
use geom_brep::props::quad::{
    LaneEnd, LaneRun, LaneStop, mean_boundary_displacement, nurbs_patch_face,
};
use geom_core::Bounds;
use geom_core::Interval;
use geom_core::Tol;
use geom_core::spline::KnotVector;

use crate::shared::patch::{dbasis_over, dense_over};
use crate::shared::ring::p3 as p;
use crate::shared::tol::band;

/// Relative-ulp overshoot of `v` outside `[lo, hi]`; `0.0` when inside.
///
/// A DIAGNOSTIC, printed on every probe row so that a carrier drifting
/// toward its enclosure's edge is visible before it becomes a failure.
/// The containment assertions themselves stay EXACT — see the note at
/// their call site for why a tolerance would have been the wrong fix.
fn overshoot_ulps(lo: f64, hi: f64, v: f64) -> f64 {
    let mag = lo.abs().max(hi.abs()).max(v.abs()).max(f64::MIN_POSITIVE);
    ((lo - v).max(v - hi).max(0.0)) / (mag * f64::EPSILON)
}

/// The corpus's declared ambient uncertainty — the ε the fixed
/// quadrature schedule (D9) is dimensioned for, and the boundary the
/// ε-keyed anti-vacuity claim below is pinned against (`props/quad/`'s own
/// unit tests use the same constant for the same reason).
const CORPUS_EPS: f64 = 1e-9;

// ---------------------------------------------------------------
// The measured refusal floors, one per carrier.
//
// Each is the mean boundary displacement (m) of the narrowest
// enclosure the lane reaches for that carrier before it stops, at its
// floor or its work wall — the width it hands back when the run's
// `1024·ε` target sits below it. Neither stop moves with ε, only the
// target does, so these are ε-independent. A carrier whose floor is
// under the run's target certifies instead, and the pin stands down
// (see `pin_floor`).
//
// They are pinned because "it refused, typed" is a contract a broken
// engine also satisfies: without a number, a refusal that got 10×
// worse still reads as green.
// ---------------------------------------------------------------

// Every width below is set by the flux's Taylor remainder, whose
// `f_uu`/`f_vv` hulls are the quotient statement's
// (`geom_core::spline::cell_measure_quotients`), at the round the
// lane's work wall stops it: the composite arm's round 7, the exact-v
// arm's round 12 (`props::quad`'s `FACE_WORK_WALL`, per span cell).

/// Möbius-reparameterized quarter cylinder (weight ratio 100), exact-v.
const MOEBIUS_FLOOR: f64 = 6.151_875_578_118e-9;
/// Warped bilinear with 1e-1/1e1 weights. Round 7's own width, read
/// against the area pass's 32 cells a span: the meter divides the flux
/// width by the area's midpoint, which that pass sets.
const HYPAR_FLOOR: f64 = 9.418_122_349_238e-6;
/// Quarter torus, R = 2 m, r = 0.5 m. Round 7's own width.
const QUARTER_TORUS_FLOOR: f64 = 1.617_645_992_203e-7;
/// Bilinear unit square with a 1e-3/1e3 weight spread: the enclosure
/// is uselessly wide, but it is a WIDTH rather than a degeneracy (see
/// the row for why). Round 7's own width.
const SQUARE_FLOOR: f64 = 6.357_819_244_047e5;
/// Unit sphere octant (degenerate pole row). Round 7's own width.
const SPHERE_OCTANT_FLOOR: f64 = 1.064_253_751_623e-7;
/// Quarter cylinder at r = 1 km: the floor is a LENGTH, so it scales
/// with the part (1e3 × the metre-scale quarter cylinder's).
const HUGE_CYLINDER_FLOOR: f64 = 2.047_260_586_885e-8;
/// The outcomes a probe is allowed to have on the run's ε — two
/// honest postures plus the degenerate-face refusal, which is sound
/// but is a capability gap and so is pinned per carrier.
///
/// This is `props/quad/rational.rs`'s own `eps_posture` contract, ported: the target
/// is `1024·ε` while the lane's floor and work wall do not move with
/// ε, so the same patch that certifies at ε = 1e-6 genuinely cannot at
/// ε = 1e-12. A probe that merely PRINTS its non-answer asserts
/// nothing about it — that is how an `inf` width and a mis-typed
/// escalation rode along under this suite for two ε rows.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Posture {
    /// The enclosure certified — and contains every truth in hand
    /// (checked in the `Ok` arm of [`probe`]).
    Certified,
    /// The lane stopped (its floor or its work wall) with its
    /// narrowest enclosure DEFINITELY wider than the run's target.
    /// Carries that enclosure's width.
    Stopped(f64),
    /// [`PropsError::DegenerateFace`]: the area enclosure does not
    /// certify positive extent, so the convergence meter has no lever
    /// and the face-extent gate would refuse the same way one step
    /// later. On a carrier whose true area IS positive this is a
    /// FALSE NEGATIVE — a capability gap, never an unsound answer —
    /// and it is pinned per carrier below so it cannot spread quietly.
    Degenerate,
}

/// Pin one carrier's measured refusal floor.
///
/// The lane's floor and work wall do not move with ε, only the target
/// does, so a carrier that stops short of the target stops with the
/// SAME width on every ε row; the pinned number is that width, as
/// measured by this suite. A carrier whose floor sits under the run's
/// target certifies instead (the ε row working as designed), so only
/// the stopped arm is pinned — the anti-vacuity test below is what
/// keeps "everything stops short" from being green.
#[track_caller]
fn pin_floor(name: &str, posture: Posture, floor: f64) {
    match posture {
        Posture::Stopped(width_len) => assert!(
            (width_len - floor).abs() <= 1e-3 * floor,
            "{name}: the stopping width MOVED: {width_len:e} against the pinned \
             {floor:e}. The floor and the work wall are fixed, so a moved width is a \
             changed enclosure — re-measure and re-pin deliberately, never widen this bound"
        ),
        Posture::Certified => {}
        Posture::Degenerate => panic!(
            "{name}: the engine now refuses this carrier as DEGENERATE — its area \
             enclosure stopped certifying positive extent. That is a capability \
             regression, not an ε row"
        ),
    }
}

// NOTE: there is no `pin_degenerate` helper any more. No carrier in
// this file reaches `DegenerateFace`: the last one that did — the
// 1e-3/1e3 weight square — now refuses with a measured width, because
// the area rule intersects its symmetric pad with a magnitude hull
// that cannot be negative. `Posture::Degenerate` is still a posture
// the engine can return and `posture_of` still classifies it; a
// carrier that starts reaching it again wants the helper back and, more
// importantly, wants explaining.

// ---------------------------------------------------------------
// The independent oracle: plain-f64 B-spline basis + derivatives
// (Cox-de Boor, no kernel spline code), rational surface point and
// partials through the quotient rule, composite Gauss-Legendre.
// ---------------------------------------------------------------

/// All basis values N_{i,p}(t) for a clamped knot vector.
///
/// **This copy stays, and is the only part of this oracle that does.**
/// It seeds the Cox-de Boor recursion by clamping `t` into the last
/// nonzero span at the domain end, where
/// `crates/geom-brep/tests/shared/patch.rs`'s takes a separate
/// `t >= knots[n]` branch. That seeding is the part of an oracle a
/// `props::quad` defect could plausibly be mirrored by, so the crate
/// keeps more than one of it on purpose; the recurrence and the
/// quadrature loop underneath carry no such risk and are shared.
///
/// It is character-identical to `cert5_r2_probes.rs`'s. Those two are
/// one derivation spelled twice, and collapsing them is an
/// adjudication about which suite owns it rather than a dedup — left
/// for TCOST-8, not decided here.
fn basis(knots: &[f64], degree: usize, ncp: usize, t: f64) -> Vec<f64> {
    let n = knots.len() - 1;
    let mut nn = vec![0.0f64; n]; // degree-0
    let d1 = knots[n - degree];
    for i in 0..n {
        let inside = if t >= d1 {
            // clamp into the last nonzero span
            knots[i] < knots[i + 1] && t >= knots[i] && t <= knots[i + 1]
        } else {
            knots[i] <= t && t < knots[i + 1]
        };
        if inside {
            nn[i] = 1.0;
        }
    }
    // at t == d1 several degenerate spans may have matched; keep only
    // the last nonzero span
    if t >= d1 {
        let mut last = None;
        for i in 0..n {
            if nn[i] == 1.0 {
                last = Some(i);
            }
        }
        for i in 0..n {
            nn[i] = 0.0;
        }
        if let Some(i) = last {
            nn[i] = 1.0;
        }
    }
    for pdeg in 1..=degree {
        let mut out = vec![0.0f64; n];
        for i in 0..(n - pdeg) {
            let da = knots[i + pdeg] - knots[i];
            let db = knots[i + pdeg + 1] - knots[i + 1];
            let a = if da > 0.0 {
                (t - knots[i]) / da * nn[i]
            } else {
                0.0
            };
            let b = if db > 0.0 {
                (knots[i + pdeg + 1] - t) / db * nn[i + 1]
            } else {
                0.0
            };
            out[i] = a + b;
        }
        nn = out;
    }
    nn.truncate(ncp);
    nn
}

/// Basis derivatives N'_{i,p}(t) — the shared divided difference,
/// taken over THIS file's `basis` above, so the derivative path keeps
/// the seeding that makes this oracle independent.
fn dbasis(knots: &[f64], degree: usize, ncp: usize, t: f64) -> Vec<f64> {
    dbasis_over(basis, knots, degree, ncp, t)
}

struct Patch {
    ku: Vec<f64>,
    kv: Vec<f64>,
    du: usize,
    dv: usize,
    nu: usize,
    nv: usize,
    /// homogeneous control: (w*x, w*y, w*z, w), row-major [i*nv + j]
    cp: Vec<[f64; 4]>,
}

impl Patch {
    /// (S, S_u, S_v) by the quotient rule on the homogeneous sums.
    /// **Deliberately not `shared::patch::Patch::eval`**, which is this
    /// text: it calls this file's OWN `basis`, whose span seeding is
    /// where this suite's independence lives (see this module's `basis`).
    /// The quadrature loop under it IS shared — `patch::dense_over`.
    fn eval(&self, u: f64, v: f64) -> ([f64; 3], [f64; 3], [f64; 3]) {
        let bu = basis(&self.ku, self.du, self.nu, u);
        let bv = basis(&self.kv, self.dv, self.nv, v);
        let dbu = dbasis(&self.ku, self.du, self.nu, u);
        let dbv = dbasis(&self.kv, self.dv, self.nv, v);
        let mut a = [0.0f64; 4];
        let mut au = [0.0f64; 4];
        let mut av = [0.0f64; 4];
        for i in 0..self.nu {
            for j in 0..self.nv {
                let c = self.cp[i * self.nv + j];
                for k in 0..4 {
                    a[k] += bu[i] * bv[j] * c[k];
                    au[k] += dbu[i] * bv[j] * c[k];
                    av[k] += bu[i] * dbv[j] * c[k];
                }
            }
        }
        let w = a[3];
        let s = [a[0] / w, a[1] / w, a[2] / w];
        let su = [
            (au[0] - s[0] * au[3]) / w,
            (au[1] - s[1] * au[3]) / w,
            (au[2] - s[2] * au[3]) / w,
        ];
        let sv = [
            (av[0] - s[0] * av[3]) / w,
            (av[1] - s[1] * av[3]) / w,
            (av[2] - s[2] * av[3]) / w,
        ];
        (s, su, sv)
    }

    /// Dense (flux, area) at `cells` cells per span per axis: the
    /// shared composite Gauss-Legendre loop over THIS file's `eval`.
    /// The ladder this suite runs it at is 24/48; why two rungs a
    /// factor of two apart settle the oracle's own error is
    /// [`crate::shared::patch::dense_over`]'s doc, which is that
    /// argument's one home.
    fn dense(&self, cells: usize) -> (f64, f64) {
        dense_over(&self.ku, &self.kv, cells, |u, v| self.eval(u, v))
    }
}

/// Build the oracle patch from the same data handed to the kernel.
fn patch(ku: &KnotVector, kv: &KnotVector, control: &[[Interval; 3]], weights: &[f64]) -> Patch {
    let nu = ku.control_count();
    let nv = kv.control_count();
    let cp = control
        .iter()
        .zip(weights)
        .map(|(c, w)| [c[0].lo() * w, c[1].lo() * w, c[2].lo() * w, *w])
        .collect();
    Patch {
        ku: ku.knots().to_vec(),
        kv: kv.knots().to_vec(),
        du: ku.degree(),
        dv: kv.degree(),
        nu,
        nv,
        cp,
    }
}

/// One probe: run the engine; if it answers, the dense oracle (and
/// any closed forms) must lie inside both brackets; if it refuses, the
/// refusal must satisfy the [`Posture`] contract. Returns the posture
/// so the caller can pin the carrier's floor.
#[allow(clippy::too_many_arguments)]
fn probe(
    name: &str,
    ku: &KnotVector,
    kv: &KnotVector,
    control: &[[Interval; 3]],
    weights: &[f64],
    perimeter: f64,
    closed_flux: Option<f64>,
    closed_area: Option<f64>,
) -> Posture {
    let pa = patch(ku, kv, control, weights);
    let (of1, oa1) = pa.dense(24);
    let (of2, oa2) = pa.dense(48);
    // A boundary-layer patch (extreme weights) can defeat uniform
    // Gauss cells; with a closed form in hand the oracle is advisory
    // there, and the closed-form containment below is the real check.
    let oracle_ok = (of1 - of2).abs() < 1e-7 * (1.0 + of2.abs())
        && (oa1 - oa2).abs() < 1e-7 * (1.0 + oa2.abs());
    assert!(
        oracle_ok || (closed_flux.is_some() && closed_area.is_some()),
        "{name}: oracle did not converge: flux {of1} vs {of2}, area {oa1} vs {oa2}"
    );
    if !oracle_ok {
        println!("{name}: WARN oracle unconverged ({of1} vs {of2}); closed forms carry the probe");
    }
    if oracle_ok {
        if let Some(cf) = closed_flux {
            assert!(
                (of2 - cf).abs() < 1e-6 * (1.0 + cf.abs()),
                "{name}: oracle flux {of2} disagrees with closed form {cf}"
            );
        }
        if let Some(ca) = closed_area {
            assert!(
                (oa2 - ca).abs() < 1e-6 * (1.0 + ca.abs()),
                "{name}: oracle area {oa2} disagrees with closed form {ca}"
            );
        }
    }
    let out = nurbs_patch_face::<f64>(
        &patch_net(ku, kv, control, weights),
        {
            let (a, b) = ku.domain();
            let (c, d) = kv.domain();
            (a, b, c, d)
        },
        perimeter,
        0.0,
        band(),
        &mut to_target(Tol::witness().get().eps),
    );
    match out {
        Ok(LaneRun {
            bounds: fb,
            end: LaneEnd::Accepted(_),
            ..
        }) => {
            println!(
                "{name}: flux [{:.12e}, {:.12e}] oracle {:.12e}; area [{:.12e}, {:.12e}] oracle {:.12e}",
                fb.flux.lo(),
                fb.flux.hi(),
                of2,
                fb.area.lo(),
                fb.area.hi(),
                oa2
            );
            // Always reported, so drift is visible before it is fatal.
            let (fo, ao) = (
                overshoot_ulps(fb.flux.lo(), fb.flux.hi(), of2),
                overshoot_ulps(fb.area.lo(), fb.area.hi(), oa2),
            );
            println!("{name}: sampled-oracle overshoot flux {fo:.2} ulps, area {ao:.2} ulps");
            // The SAMPLED oracle is authoritative only where there is no
            // exact value to compare against.
            //
            // `oracle_ok` above establishes that the coarse and fine
            // grids AGREE — convergence, not accuracy. Both share the
            // same O(n) summation drift, so a bias common to both is
            // invisible to that test. That is harmless while the
            // enclosure is orders wider than the drift, which holds for
            // every hard carrier here. It inverts for an exactly
            // integrable one: the weight-1 unit square encloses
            // [0.9999999999999908, 1.000000000000012] (width ~2e-14,
            // containing the true 1.0) against a *converged* oracle of
            // 1.0000000000002953 — wrong by 1276 relative ulps, an order
            // wider than the enclosure judging it. Asserting containment
            // of the approximation would report the oracle's own drift as
            // a kernel exclusion.
            //
            // So where a closed form is supplied it carries the claim,
            // and it is the STRONGER check: it is the true value, not an
            // estimate of it, and it is asserted exactly just below.
            if oracle_ok && (closed_flux.is_none() || closed_area.is_none()) {
                assert!(
                    fo == 0.0,
                    "{name}: FLUX EXCLUSION — enclosure [{}, {}] excludes oracle {} by {fo:.2} relative ulps",
                    fb.flux.lo(),
                    fb.flux.hi(),
                    of2
                );
                assert!(
                    ao == 0.0,
                    "{name}: AREA EXCLUSION — enclosure [{}, {}] excludes oracle {} by {ao:.2} relative ulps",
                    fb.area.lo(),
                    fb.area.hi(),
                    oa2
                );
            }
            if let Some(cf) = closed_flux {
                assert!(
                    fb.flux.lo() <= cf && cf <= fb.flux.hi(),
                    "{name}: FLUX EXCLUSION of closed form {cf}: [{}, {}]",
                    fb.flux.lo(),
                    fb.flux.hi()
                );
            }
            if let Some(ca) = closed_area {
                assert!(
                    fb.area.lo() <= ca && ca <= fb.area.hi(),
                    "{name}: AREA EXCLUSION of closed form {ca}: [{}, {}]",
                    fb.area.lo(),
                    fb.area.hi()
                );
            }
            Posture::Certified
        }
        // A lane that stopped short asserts the SAME contract as the
        // crate's own `eps_posture` (props/quad/rational.rs): its narrowest enclosure
        // still holds every truth in hand, and its width really missed
        // the run's own `1024·ε` target.
        Ok(run) => {
            let target = QUAD_TARGET_LEN_FACTOR * Tol::witness().get().eps;
            let fb = run.bounds;
            let width_len = mean_boundary_displacement(fb.flux, fb.area).unwrap();
            println!("{name}: stopped short (sound): {run:?}");
            if let Some(cf) = closed_flux {
                assert!(
                    fb.flux.lo() <= cf && cf <= fb.flux.hi(),
                    "{name}: FLUX EXCLUSION of closed form {cf} at the stop: [{}, {}]",
                    fb.flux.lo(),
                    fb.flux.hi()
                );
            }
            if let Some(ca) = closed_area {
                assert!(
                    fb.area.lo() <= ca && ca <= fb.area.hi(),
                    "{name}: AREA EXCLUSION of closed form {ca} at the stop: [{}, {}]",
                    fb.area.lo(),
                    fb.area.hi()
                );
            }
            assert!(
                matches!(run.end, LaneEnd::Floor | LaneEnd::Wall),
                "{name}: a whole-schedule call ends at the lane's own stop: {run:?}"
            );
            assert!(
                width_len.is_finite() && width_len > target,
                "{name}: a lane stopped short must carry a width that really \
                 missed: {width_len:e} vs {target:e}"
            );
            println!("FLOOR {name} {width_len:.12e} target {target:.6e} {run:?}");
            Posture::Stopped(width_len)
        }
        Err(e) => {
            println!("{name}: typed refusal (sound): {e}");
            match e {
                // A face whose AREA enclosure does not certify positive
                // extent: the convergence meter has no lever, and the
                // face-extent gate would read the same `area.lo()` the
                // same way. Sound (it answers nothing), but on a
                // carrier with a positive true area it is a false
                // negative, so no probe gets it by default — the only
                // carrier allowed to reach it says so with
                // `pin_degenerate`, and `pin_floor` panics on it.
                PropsError::DegenerateFace => Posture::Degenerate,
                other => panic!("{name}: unsound outcome {other:?}"),
            }
        }
    }
}

const W2: f64 = core::f64::consts::FRAC_1_SQRT_2;
const PI: f64 = core::f64::consts::PI;

/// Sphere octant (unit radius, degenerate pole row): |flux| = area =
/// pi/2. Signed flux from the oracle.
#[test]
fn probe_sphere_octant() {
    let kv2 = KnotVector::unit_segment(const { NonZeroUsize::new(2).unwrap() });
    let net = [
        p(0.0, 0.0, 1.0),
        p(0.0, 0.0, 1.0),
        p(0.0, 0.0, 1.0),
        p(1.0, 0.0, 1.0),
        p(1.0, 1.0, 1.0),
        p(0.0, 1.0, 1.0),
        p(1.0, 0.0, 0.0),
        p(1.0, 1.0, 0.0),
        p(0.0, 1.0, 0.0),
    ];
    let weights = [1.0, W2, 1.0, W2, 0.5, W2, 1.0, W2, 1.0];
    let posture = probe(
        "sphere-octant",
        &kv2,
        &kv2,
        &net,
        &weights,
        3.0 * PI / 2.0,
        None,
        Some(PI / 2.0),
    );
    pin_floor("sphere-octant", posture, SPHERE_OCTANT_FLOOR);
}

/// Quarter torus patch (R=2, r=0.5): dense oracle only.
#[test]
fn probe_quarter_torus() {
    let kv2 = KnotVector::unit_segment(const { NonZeroUsize::new(2).unwrap() });
    let (rr, r) = (2.0, 0.5);
    // tube quarter arc in xz-plane: (R+r,0,0) -> (R+r,0,r) -> (R,0,r)
    let prof = [(rr + r, 0.0), (rr + r, r), (rr, r)];
    let pw = [1.0, W2, 1.0];
    let mut net = Vec::new();
    let mut weights = Vec::new();
    for (k, (x, z)) in prof.iter().enumerate() {
        net.push(p(*x, 0.0, *z));
        net.push(p(*x, *x, *z));
        net.push(p(0.0, *x, *z));
        for wj in [1.0, W2, 1.0] {
            weights.push(pw[k] * wj);
        }
    }
    let posture = probe(
        "quarter-torus",
        &kv2,
        &kv2,
        &net,
        &weights,
        2.0 * (PI / 2.0) * (rr + r) + 2.0 * (PI / 2.0) * r,
        None,
        None,
    );
    pin_floor("quarter-torus", posture, QUARTER_TORUS_FLOOR);
}

/// Moebius-reparameterized quarter cylinder (weights scaled by
/// lambda^i, lambda = 10 -> same locus): flux = area = pi exactly.
#[test]
fn probe_moebius_quarter_cylinder() {
    let ku = KnotVector::unit_segment(const { NonZeroUsize::new(2).unwrap() });
    let kv = KnotVector::unit_segment(NonZeroUsize::MIN);
    let h = 2.0;
    let net = [
        p(1.0, 0.0, 0.0),
        p(1.0, 0.0, h),
        p(1.0, 1.0, 0.0),
        p(1.0, 1.0, h),
        p(0.0, 1.0, 0.0),
        p(0.0, 1.0, h),
    ];
    let l = 10.0;
    let weights = [1.0, 1.0, W2 * l, W2 * l, l * l, l * l];
    let posture = probe(
        "moebius-quarter-cylinder",
        &ku,
        &kv,
        &net,
        &weights,
        4.0 + PI,
        Some(PI),
        Some(PI),
    );
    pin_floor("moebius-quarter-cylinder", posture, MOEBIUS_FLOOR);
}

/// Bilinear rational reparameterized unit square at z=1 with EXTREME
/// mixed corner weights (1e-3 / 1e3): flux = area = 1 exactly.
#[test]
fn probe_extreme_weight_square() {
    let kv1 = KnotVector::unit_segment(NonZeroUsize::MIN);
    let net = [
        p(0.0, 0.0, 1.0),
        p(0.0, 1.0, 1.0),
        p(1.0, 0.0, 1.0),
        p(1.0, 1.0, 1.0),
    ];
    let weights = [1e-3, 1e3, 1e3, 1e-3];
    let posture = probe(
        "extreme-weight-square",
        &kv1,
        &kv1,
        &net,
        &weights,
        4.0,
        Some(1.0),
        Some(1.0),
    );
    // The 1e-3/1e3 weight spread still makes the area rule's Lipschitz
    // pad astronomically larger than the true area of 1 m². What it no
    // longer does is straddle zero: the pad is symmetric, but the rule
    // now intersects it with the cell's own hull of `g = |S_u×S_v|`,
    // and a magnitude's hull has a non-negative lower end. So the face
    // has a positive certified extent, the meter has a lever, and the
    // carrier refuses with a MEASURED width instead of as degenerate.
    //
    // That is a refusal changing class, and in this direction it is
    // the honest one: the patch really is a unit square, the engine
    // really cannot bracket it usefully, and a stop carrying its width
    // says exactly that, where `DegenerateFace` said the
    // face had no extent — which was false of it. The enclosure is
    // still uselessly wide, so the capability gap this carrier exists
    // to hold still is intact; only its spelling improved.
    pin_floor("extreme-weight-square", posture, SQUARE_FLOOR);
}

/// Warped bilinear (hyperbolic paraboloid) with mixed extreme
/// weights: dense oracle only.
#[test]
fn probe_extreme_weight_hypar() {
    let kv1 = KnotVector::unit_segment(NonZeroUsize::MIN);
    let net = [
        p(0.0, 0.0, 0.0),
        p(0.0, 1.0, 1.0),
        p(1.0, 0.0, 1.0),
        p(1.0, 1.0, 0.0),
    ];
    let weights = [1e-1, 1.0, 1.0, 1e1];
    let posture = probe(
        "extreme-weight-hypar",
        &kv1,
        &kv1,
        &net,
        &weights,
        6.0,
        None,
        None,
    );
    pin_floor("extreme-weight-hypar", posture, HYPAR_FLOOR);
}

/// The quarter-cylinder carrier at radius `r`, height `h` (the scale
/// row's shape; shared with the anti-vacuity test below).
/// Closed forms: flux = (pi/2) r^2 h, area = (pi/2) r h.
fn quarter_cylinder_probe(name: &str, r: f64, h: f64) -> Posture {
    let ku = KnotVector::unit_segment(const { NonZeroUsize::new(2).unwrap() });
    let kv = KnotVector::unit_segment(NonZeroUsize::MIN);
    let net = [
        p(r, 0.0, 0.0),
        p(r, 0.0, h),
        p(r, r, 0.0),
        p(r, r, h),
        p(0.0, r, 0.0),
        p(0.0, r, h),
    ];
    let weights = [1.0, 1.0, W2, W2, 1.0, 1.0];
    probe(
        name,
        &ku,
        &kv,
        &net,
        &weights,
        2.0 * h + PI * r,
        Some(PI / 2.0 * r * r * h),
        Some(PI / 2.0 * r * h),
    )
}

/// Scale extremes: quarter cylinders at r = 1e-6 and r = 1e3 — the two
/// ends of the LENGTH meter.
///
/// The 1 µm carrier's convergence floor scales with the part (1e-9× the
/// metre-scale quarter cylinder's), so convergence is never its
/// problem. What it runs into instead is the FACE-EXTENT gate, whose
/// lever is also a length: the face's mean width is `area/perimeter` ≈
/// 0.22 µm, which is above ε = 1e-9 but BELOW ε = 1e-6 — so at the
/// coarse row the run's own tolerance says this face is degenerate,
/// and the engine refuses it as such. That is the gate working, it is
/// ε-keyed rather than waived here, and it predates the meter guard
/// (verified: the pre-guard engine refuses the same row identically).
///
/// The 1 km carrier's width is 1e3× the metre-scale one's: it
/// certifies at 1e-6 and 1e-9 and stops short at 1e-12.
#[test]
fn probe_scale_extremes() {
    let tiny = quarter_cylinder_probe("tiny-cylinder", 1e-6, 2e-6);
    let expected = if Tol::witness().get().eps <= CORPUS_EPS {
        Posture::Certified
    } else {
        Posture::Degenerate
    };
    assert_eq!(
        tiny,
        expected,
        "the 1 µm quarter cylinder's posture is ε-keyed: certified at the corpus ε \
         and below, degenerate above it (its 0.22 µm mean width is under the run's \
         own tolerance there) — got {tiny:?} at ε = {:e}",
        Tol::witness().get().eps
    );
    pin_floor(
        "huge-cylinder",
        quarter_cylinder_probe("huge-cylinder", 1e3, 2e3),
        HUGE_CYLINDER_FLOOR,
    );
}

/// ANTI-VACUITY: a probe in this suite must still CERTIFY on the run's
/// ε — and at the corpus ε, a RATIONAL one must.
///
/// Every other assertion here is conditional on the outcome class — a
/// refusal discharges its contract by being typed, finite and honest —
/// so a change that made the engine refuse everything would leave the
/// whole file green. This is the floor under that, in two parts,
/// because one claim alone is either vacuous or false:
///
/// - **Every ε row**: the weight-1 unit square rides the exact
///   per-span Newton–Cotes lane, whose enclosure width is ring
///   rounding only. That is ε-INDEPENDENT, so it must certify at every
///   ε in the matrix; if it ever refuses, the engine has stopped being
///   able to answer at all.
/// - **At the corpus ε and coarser**: the rational quarter cylinder
///   must certify, so the rational lane is covered too. Below the
///   corpus ε this is not available and pretending otherwise would be
///   the ε-blindness the [`Posture`] docs describe: the rational
///   target is ε-coupled against a stop that is not, so at ε = 1e-12
///   a rational carrier here may honestly stop short.
///
/// (Containment is checked inside [`probe`]: certifying with a bracket
/// that excluded the truth fails there, not here.)
#[test]
fn probe_suite_still_certifies_something() {
    let kv1 = KnotVector::unit_segment(NonZeroUsize::MIN);
    let flat = [
        p(0.0, 0.0, 1.0),
        p(0.0, 1.0, 1.0),
        p(1.0, 0.0, 1.0),
        p(1.0, 1.0, 1.0),
    ];
    assert_eq!(
        probe(
            "anti-vacuity-unit-square",
            &kv1,
            &kv1,
            &flat,
            &[1.0; 4],
            4.0,
            Some(1.0),
            Some(1.0),
        ),
        Posture::Certified,
        "ANTI-VACUITY: the exact-lane unit square no longer certifies at ε = {:e}. \
         Nothing in this file certifies any more, so every refusal assertion in it \
         is vacuously satisfiable",
        Tol::witness().get().eps
    );
    if Tol::witness().get().eps >= CORPUS_EPS {
        assert_eq!(
            quarter_cylinder_probe("anti-vacuity-quarter-cylinder", 1.0, 2.0),
            Posture::Certified,
            "ANTI-VACUITY: no RATIONAL carrier certifies at ε = {:e}, which is the \
             corpus ε or coarser — the lane has stopped answering, not just stopped \
             reaching a tighter target",
            Tol::witness().get().eps
        );
    }
}

/// Half cylinder as a TWO-SPAN quadratic (interior double knot at
/// 1/2, non-uniform): flux = pi r^2 h = 2 pi, area = pi r h = 2 pi.
#[test]
fn probe_half_cylinder_interior_knot() {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::unit_segment(NonZeroUsize::MIN);
    let h = 2.0;
    let net = [
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
    let weights = [1.0, 1.0, W2, W2, 1.0, 1.0, W2, W2, 1.0, 1.0];
    let posture = probe(
        "half-cylinder-2span",
        &ku,
        &kv,
        &net,
        &weights,
        2.0 * h + 2.0 * PI,
        Some(2.0 * PI),
        Some(2.0 * PI),
    );
    // Its exact-v arm reaches 4.27e-10 m at the work wall, under the
    // tightest row's 1.024e-9 m target, so it certifies on every row.
    assert_eq!(posture, Posture::Certified, "half-cylinder-2span");
}

/// A C0 CORNER inside the domain (double knot, degree 2, kink not on
/// any cell grid: knot at 1/3): a genuinely discontinuous surface
/// derivative. The area pass has no straddle rule, so the honest
/// outcomes are containment or a typed refusal — a returned bracket
/// excluding the truth is the unsound outcome this probe hunts.
#[test]
fn probe_c0_kink_area() {
    let ku =
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0 / 3.0, 1.0 / 3.0, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::unit_segment(NonZeroUsize::MIN);
    // profile in the xy-plane: straight (0,0)->(1,0), corner, then
    // straight (1,0)->(1,4); extruded in z by 1.
    let prof = [(0.0, 0.0), (0.5, 0.0), (1.0, 0.0), (1.0, 2.0), (1.0, 4.0)];
    let mut net = Vec::new();
    for (x, y) in prof {
        net.push(p(x, y, 0.0));
        net.push(p(x, y, 1.0));
    }
    let weights = [0.9; 10];
    let posture = probe(
        "c0-kink-wall",
        &ku,
        &kv,
        &net,
        &weights,
        2.0 * 5.0 + 2.0,
        // `S·(S_u × S_v)` is `y·dx = 0` on the y = 0 leg and
        // `x·dy = 1·4` on the x = 1 leg: flux 4, exactly.
        Some(4.0),
        Some(5.0),
    );
    // Both legs are straight, so the knot-aligned cells carry no
    // remainder and the enclosure is the arithmetic's: it certifies on
    // every row.
    assert_eq!(posture, Posture::Certified, "c0-kink-wall");
}

/// DIAGNOSIS: is certified knot refinement exact on the two-span
/// half-circle? Refine the HOMOGENEOUS 1-D net with the kernel's own
/// refine_plan chain (weights-1 plans, plain lerp — exactly what
/// RefinedNet does) and compare rational curve points before/after
/// with the independent evaluator.
#[test]
fn diag_refine_half_circle() {
    use geom_core::spline::algebra::{domain_grid_points, refine_plan_homogeneous};
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let pts = [(1.0, 0.0), (1.0, 1.0), (0.0, 1.0), (-1.0, 1.0), (-1.0, 0.0)];
    let ws = [1.0, W2, 1.0, W2, 1.0];
    // homogeneous 3-vectors (x*w, y*w, w)
    let mut hom: Vec<[f64; 3]> = pts
        .iter()
        .zip(&ws)
        .map(|((x, y), w)| [x * w, y * w, *w])
        .collect();
    let add = domain_grid_points(&kv, 16);
    let plans = refine_plan_homogeneous(&kv, &add).unwrap();
    let mut cur_kv = kv.clone();
    for plan in &plans {
        hom = plan.apply_points(&hom, [f64::NAN; 3], |x, y, l: f64| {
            [
                x[0] + (y[0] - x[0]) * l,
                x[1] + (y[1] - x[1]) * l,
                x[2] + (y[2] - x[2]) * l,
            ]
        });
        cur_kv = plan.knots().clone();
    }
    // evaluate both with the independent basis
    let eval = |kvv: &KnotVector, net: &[[f64; 3]], t: f64| -> (f64, f64) {
        let b = basis(kvv.knots(), 2, kvv.control_count(), t);
        let mut a = [0.0f64; 3];
        for (i, c) in net.iter().enumerate() {
            for k in 0..3 {
                a[k] += b[i] * c[k];
            }
        }
        (a[0] / a[2], a[1] / a[2])
    };
    let orig: Vec<[f64; 3]> = pts
        .iter()
        .zip(&ws)
        .map(|((x, y), w)| [x * w, y * w, *w])
        .collect();
    let mut worst = 0.0f64;
    for k in 0..=200 {
        let t = k as f64 / 200.0;
        let (x0, y0) = eval(&kv, &orig, t);
        let (x1, y1) = eval(&cur_kv, &hom, t);
        let d = ((x1 - x0).powi(2) + (y1 - y0).powi(2)).sqrt();
        if d > worst {
            worst = d;
            if d > 1e-12 {
                println!("t={t} orig=({x0},{y0}) refined=({x1},{y1}) d={d}");
            }
        }
    }
    println!("worst refinement deviation: {worst:e}");
    assert!(
        worst < 1e-12,
        "refinement is NOT locus-preserving: {worst:e}"
    );
}

/// ISOLATION: uniform weights w = 0.5 make the rational lane's
/// integrand identical to the weight-1 polynomial patch (w is
/// constant, f = N/w^3 reduces exactly). The integral lane certifies
/// the weight-1 twin; the two enclosures must OVERLAP (both contain
/// the same truth). Two-span half-cylinder net.
#[test]
fn diag_uniform_weight_twins() {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::unit_segment(NonZeroUsize::MIN);
    let h = 2.0;
    let net = [
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
    let run = |ws: &[f64]| {
        at_target(nurbs_patch_face::<f64>(
            &patch_net(&ku, &kv, &net, ws),
            (0.0, 1.0, 0.0, 1.0),
            2.0 * h + 2.0 * PI,
            0.0,
            band(),
            &mut to_target(Tol::witness().get().eps),
        ))
    };
    let poly = run(&[1.0; 10]).expect("integral lane");
    println!(
        "poly-first flux [{:.12}, {:.12}] area [{:.12}, {:.12}]",
        poly.flux.lo(),
        poly.flux.hi(),
        poly.area.lo(),
        poly.area.hi()
    );
    // The INTEGRAL twin rides the exact per-span Newton-Cotes rule, so
    // it certifies on every ε row, and #313's closed form is available:
    // the half cylinder's flux is exactly 20/3.
    let truth = 20.0 / 3.0;
    assert!(
        poly.flux.lo() <= truth && truth <= poly.flux.hi(),
        "the integral twin excludes the closed form 20/3: [{}, {}]",
        poly.flux.lo(),
        poly.flux.hi()
    );
    let pa = patch(&ku, &kv, &net, &[0.5; 10]);
    let (of, _) = pa.dense(48);
    println!("oracle flux {of:.12}");
    // The RATIONAL twin's target is ε-coupled against a lane whose stop
    // is not, so below the corpus ε its honest outcome may be to stop
    // short. Both
    // arms are pinned; a wide or wrong answer is what is ruled out.
    match run(&[0.5; 10]) {
        Ok(rat) => {
            println!(
                "poly flux [{:.12}, {:.12}]  rational flux [{:.12}, {:.12}]",
                poly.flux.lo(),
                poly.flux.hi(),
                rat.flux.lo(),
                rat.flux.hi()
            );
            // Read against the closed form, not the dense oracle: the
            // same locus, and the oracle's own error (2e-13 here) is
            // wider than this enclosure.
            assert!(
                rat.flux.lo() <= truth && truth <= rat.flux.hi(),
                "rational lane excludes the closed form 20/3: [{}, {}] (dense oracle {of})",
                rat.flux.lo(),
                rat.flux.hi()
            );
        }
        Err(e) => {
            println!(
                "rational twin refuses typed @ eps={:e}: {e:?}",
                Tol::witness().get().eps
            );
            assert!(
                matches!(
                    e,
                    Unmet::Stopped(LaneRun {
                        end: LaneEnd::Floor | LaneEnd::Wall,
                        ..
                    })
                ),
                "the rational twin must stop at the lane's own stop: {e:?}"
            );
        }
    }
}

/// Genericity spot check: the rational lane driven by the certified
/// Interval decision scalar must agree with the f64 lane bit-for-bit
/// on the returned enclosure (the Interval arithmetic is shared;
/// only decisions route through T).
#[test]
fn probe_interval_scalar_agrees() {
    use geom_core::Interval;
    let ku = KnotVector::unit_segment(const { NonZeroUsize::new(2).unwrap() });
    let kv = KnotVector::unit_segment(NonZeroUsize::MIN);
    let h = 2.0;
    let net = [
        p(1.0, 0.0, 0.0),
        p(1.0, 0.0, h),
        p(1.0, 1.0, 0.0),
        p(1.0, 1.0, h),
        p(0.0, 1.0, 0.0),
        p(0.0, 1.0, h),
    ];
    let weights = [1.0, 1.0, W2, W2, 1.0, 1.0];
    let rect = (0.0, 1.0, 0.0, 1.0);
    let eps = Tol::witness().get().eps;
    let a = at_target(nurbs_patch_face::<f64>(
        &patch_net(&ku, &kv, &net, &weights),
        rect,
        4.0 + PI,
        0.0,
        band(),
        &mut to_target(eps),
    ));
    let b = at_target(nurbs_patch_face::<Interval>(
        &patch_net(&ku, &kv, &net, &weights),
        rect,
        4.0 + PI,
        0.0,
        band(),
        &mut to_target(eps),
    ));
    // The agreement claim is ε-independent and covers the stops too:
    // below the corpus ε the ε-coupled target may be out of the lane's
    // reach, and then BOTH scalars must stop short, identically.
    match (a, b) {
        (Ok(a), Ok(b)) => {
            assert_eq!(a.flux.lo().to_bits(), b.flux.lo().to_bits());
            assert_eq!(a.flux.hi().to_bits(), b.flux.hi().to_bits());
            assert_eq!(a.area.lo().to_bits(), b.area.lo().to_bits());
            assert_eq!(a.area.hi().to_bits(), b.area.hi().to_bits());
            assert!(a.flux.lo() <= PI && PI <= a.flux.hi());
        }
        (Err(a), Err(b)) => {
            println!("INTERVAL-AGREE refusal @ eps={eps:e}: {a:?}");
            assert!(
                matches!(
                    a,
                    Unmet::Stopped(LaneRun {
                        end: LaneEnd::Floor | LaneEnd::Wall,
                        ..
                    })
                ),
                "an ε row must stop at the lane's own stop: {a:?}"
            );
            assert_eq!(
                format!("{a:?}"),
                format!("{b:?}"),
                "the two decision scalars disagree on the refusal"
            );
        }
        (x, y) => panic!("f64 and Interval disagree on the OUTCOME: {x:?} vs {y:?}"),
    }
}

/// The quarter torus `(R, r)` over `θ, φ ∈ [0, π/2]` as `spans × spans`
/// degree-2 conic spans (each a rational quadratic arc of
/// `π/(2·spans)`, joined at double knots): the surface of revolution of
/// the minor arc about `z` (Piegl–Tiller), with weights `w_i·w_j`.
fn conic_quarter_torus(
    rr: f64,
    r: f64,
    spans: usize,
) -> (KnotVector, Vec<[Interval; 3]>, Vec<f64>) {
    #[allow(clippy::cast_precision_loss)]
    let a = core::f64::consts::FRAC_PI_2 / spans as f64;
    let mut knots = vec![0.0; 3];
    for k in 1..spans {
        #[allow(clippy::cast_precision_loss)]
        let t = k as f64 / spans as f64;
        knots.extend([t, t]);
    }
    knots.extend([1.0; 3]);
    let kv = KnotVector::clamped(knots, 2).expect("a clamped conic knot vector");
    // The unit arc's control points and weights over [0, π/2].
    let mut unit: Vec<((f64, f64), f64)> = vec![((1.0, 0.0), 1.0)];
    for k in 0..spans {
        #[allow(clippy::cast_precision_loss)]
        let (a0, a1) = (a * k as f64, a * (k + 1) as f64);
        let m = 0.5 * (a0 + a1);
        let c = (0.5 * a).cos();
        unit.push(((m.cos() / c, m.sin() / c), c));
        unit.push(((a1.cos(), a1.sin()), 1.0));
    }
    let mut net = Vec::new();
    let mut weights = Vec::new();
    // u: the minor (profile) arc, `(d, z) = (R + r·cos φ, r·sin φ)`;
    // v: the major arc about z.
    for &((cx, sx), wu) in &unit {
        let (d, z) = (rr + r * cx, r * sx);
        for &((ca, sa), wv) in &unit {
            net.push(p(d * ca, d * sa, z));
            weights.push(wu * wv);
        }
    }
    (kv, net, weights)
}

/// The quarter torus's area and its flux `∫ S·n dA` about the origin
/// (outward normal), closed form.
fn quarter_torus_closed(rr: f64, r: f64) -> (f64, f64) {
    let h = core::f64::consts::FRAC_PI_2;
    let area = h * r * (rr * h + r);
    let flux = h * r * (rr * rr + rr * r * core::f64::consts::FRAC_PI_4 + rr * r * h + r * r);
    (flux, area)
}

/// **A patch costs per span, so more spans do not stop it short.** The
/// quarter torus at 12× (R = 24 m, r = 6 m) on 9×9 conic spans (PR
/// 5589's first reviewer's witness) stopped at the work wall after round
/// 6 while its schedule was laid over the face: its rounds cost more
/// cells than the calibrating face's, and each span got about a ninth of
/// the cells a one-span patch gets. Laid per span, with the wall per span
/// cell, it meets the reporting target at round 3, and that round's
/// bracket holds the closed-form flux and area. The flux is read about
/// the origin with the patch's normal inward, so its value is
/// `−∫ S·n dA`.
#[test]
fn a_conic_torus_on_nine_spans_a_direction_meets_its_target_holding_its_closed_form() {
    use geom_core::interval::certification::Certification;
    let (rr, r) = (24.0, 6.0);
    let (flux, area) = quarter_torus_closed(rr, r);
    let (kv, net, w) = conic_quarter_torus(rr, r, 9);
    let run = nurbs_patch_face::<f64>(
        &patch_net(&kv, &kv, &net, &w),
        (0.0, 1.0, 0.0, 1.0),
        2.0 * core::f64::consts::FRAC_PI_2 * (rr + r) + 2.0 * core::f64::consts::FRAC_PI_2 * r,
        0.0,
        crate::shared::tol::band(),
        &mut to_target(CORPUS_EPS),
    )
    .expect("the conic torus has an enclosure");
    assert_eq!(
        (run.end, run.rounds),
        (LaneEnd::Accepted(LaneStop::Refinable), 4),
        "{run:?}"
    );
    assert!(
        run.bounds.flux.contains(-flux),
        "flux {:?} misses {}",
        run.bounds.flux,
        -flux
    );
    assert!(
        run.bounds.area.contains(area),
        "area {:?} misses {area}",
        run.bounds.area
    );
}
