//! R1 adversarial review probes for PR #309 (M8-3 half 2): the
//! rational patch-flux enclosure. Every probe pits the delivered
//! enclosure against an INDEPENDENT dense oracle (plain-f64 Cox-de
//! Boor evaluation + composite Gauss-Legendre), and where a closed
//! form exists, against that too. The contract under test: the true
//! flux/area lie INSIDE the returned brackets, or the call refuses
//! typed. A returned bracket that EXCLUDES the truth is the failure.
//!
//! **The refusals are held to a contract too** — they are not merely
//! printed. Each one must be a typed outcome this lane is allowed to
//! produce, must carry a FINITE width that really missed the run's own
//! `1024·ε` target, and must land on the carrier's PINNED floor (the
//! refinement schedule is fixed by D9, so only the target moves with
//! ε). A probe that only prints its `Err` asserts nothing: that is how
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

use geom_brep::props::PropsError;
use geom_brep::props::quad::nurbs_patch_face;
use geom_core::Tol;
use geom_core::spline::KnotVector;
use geom_core::{Band, MarginDiag, RingInterval};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// The engine's convergence target as a multiple of ε
/// (`QUAD_TARGET_LEN_FACTOR`, crate-private — mirrored here so a
/// refusal's carried target can be checked against the run's ε).
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

const TARGET_LEN_FACTOR: f64 = 1024.0;

/// The corpus's declared ambient uncertainty — the ε the fixed
/// quadrature schedule (D9) is dimensioned for, and the boundary the
/// ε-keyed anti-vacuity claim below is pinned against (`quad.rs`'s own
/// unit tests use the same constant for the same reason).
const CORPUS_EPS: f64 = 1e-9;

// ---------------------------------------------------------------
// The measured refusal floors, one per carrier.
//
// Each is the mean boundary displacement (m) the fixed schedule (D9)
// bottoms out at for that carrier — the number a
// `PropsError::QuadratureBudget` carries when the run's `1024·ε`
// target sits below it. The SCHEDULE is fixed and only the target
// moves with ε, so these are ε-independent: measured identical on the
// 1e-9 and 1e-12 rows. A carrier whose floor is under the run's
// target certifies instead, and the pin stands down (see `pin_floor`).
//
// They are pinned because "it refused, typed" is a contract a broken
// engine also satisfies: without a number, a refusal that got 10×
// worse still reads as green.
// ---------------------------------------------------------------

// **Why the extreme-weight floors moved, and by how much.** These are
// AREA-dominated carriers: their symmetric Lipschitz pad is what the
// enclosure is made of, and the area rule now intersects that pad with
// the cell's own hull of `g = |S_u×S_v|` instead of choosing between
// them. Both bound the cell's mean, so the intersection is sound and
// can only narrow — and on exactly these carriers, where the pad is
// enormous and the hull is merely wide, it narrows by an order.
// Nothing about the flux side moved: the carriers whose enclosure is
// flux-dominated (the cylinders below) moved in their seventh digit,
// which is the cut arithmetic, not the area rule.

/// Möbius-reparameterized quarter cylinder (weight ratio 100).
/// Was 5.165e-2 before the area rule intersected its two bounds.
const MOEBIUS_FLOOR: f64 = 2.877_754_878_307e-3;
/// Warped bilinear with 1e-1/1e1 weights. Was 1.916e-2, same cause.
const HYPAR_FLOOR: f64 = 2.362_832_410_525e-3;
/// Quarter torus, R = 2 m, r = 0.5 m.
const QUARTER_TORUS_FLOOR: f64 = 1.146e-6;
/// Two-span half cylinder, r = 1 m, h = 2 m — the row that misses the
/// DEFAULT ε (1.024e-6) by 27%. Re-measured at 1.303_516_100_912e-6,
/// which its 1e-3 relative tolerance already covered; kept at three
/// digits because that is the precision this row's claim needs.
const HALF_CYLINDER_FLOOR: f64 = 1.304e-6;
/// Bilinear unit square with a 1e-3/1e3 weight spread: the enclosure
/// is uselessly wide, but it is now a WIDTH rather than a degeneracy
/// (see the row for why the class changed).
const SQUARE_FLOOR: f64 = 3.415_290_727_405e6;
/// C0-kink extruded wall (5 m profile).
const C0_KINK_FLOOR: f64 = 4.785e-4;
/// Unit sphere octant (degenerate pole row).
const SPHERE_OCTANT_FLOOR: f64 = 9.683e-7;
/// Quarter cylinder at r = 1 km: the floor is a LENGTH, so it scales
/// with the part (1e3 × the metre-scale quarter cylinder's).
const HUGE_CYLINDER_FLOOR: f64 = 1.533_469_017_207e-4;
/// The determinism carrier: single-span quarter cylinder, r = 1 m,
/// h = 2 m, driven at a millionfold-tighter target.
///
/// This one is pinned to thirteen digits against a 1e-3 relative
/// tolerance, so it is the row that notices a flux-side change at all:
/// it moved from `1.535_131_804_305_385e-7` when the cells became
/// knot-aligned (an outward-rounded cell width in place of a rounded
/// float, on a carrier with no interior knots to align to) and again
/// in its seventh digit when the hull blocks did. Neither is the area
/// rule — this carrier's enclosure is flux-dominated.
const QUARTER_CYLINDER_FLOOR: f64 = 1.533_466_684_469_612e-7;

/// The outcomes a probe is allowed to have on the run's ε — three
/// honest postures plus the degenerate-face refusal, which is sound
/// but is a capability gap and so is pinned per carrier.
///
/// This is `quad.rs`'s own `eps_posture` contract, ported: the target
/// is `1024·ε` while the refinement schedule is FIXED (D9), so the
/// same patch that certifies at ε = 1e-6 genuinely cannot at
/// ε = 1e-12. A probe that merely PRINTS its refusal asserts nothing
/// about it — that is how an `inf` width and a mis-typed escalation
/// rode along under this suite for two ε rows.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Posture {
    /// The enclosure certified — and contains every truth in hand
    /// (checked in the `Ok` arm of [`probe`]).
    Certified,
    /// [`PropsError::QuadratureBudget`]: the fixed schedule's floor is
    /// DEFINITELY above the run's target. Carries the width the
    /// schedule's LAST round reaches: measured, when the schedule ran
    /// out, or its lower bound, when the loop proved after round 0
    /// that the last round could not certify and refused without
    /// running it. The two differ by the midpoint sum's own rounding
    /// width — at most 4e-4 relative on every carrier below, inside
    /// the pins' 1e-3 window — so a pin reads either the same way.
    Budget(f64),
    /// The same shortfall, in-band for the run's `Band{ε, Kε}` — so
    /// `props_quad_converged` escalates through the funnel before the
    /// budget can run out.
    Escalated,
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
/// The refinement schedule is fixed by D9 and only the target moves
/// with ε, so a carrier that refuses at the budget refuses with the
/// SAME width on every ε row; the pinned number is that width, as
/// measured by this suite. The refusal's payload is that width or its
/// lower bound ([`Posture::Budget`]) — every pin below was re-read
/// against the bound, none moved past its window, and no pinned
/// number changed. A carrier whose floor sits under the run's
/// target certifies instead (the ε row working as designed), so only
/// the budget arm is pinned — the anti-vacuity test below is what
/// keeps "everything refuses" from being green.
#[track_caller]
fn pin_floor(name: &str, posture: Posture, floor: f64) {
    match posture {
        Posture::Budget(width_len) => assert!(
            (width_len - floor).abs() <= 1e-3 * floor,
            "{name}: the refusal floor MOVED: {width_len:e} against the pinned \
             {floor:e}. The schedule is fixed (D9), so a moved floor is a changed \
             enclosure — re-measure and re-pin deliberately, never widen this bound"
        ),
        Posture::Certified | Posture::Escalated => {}
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

fn p(x: f64, y: f64, z: f64) -> [RingInterval; 3] {
    [
        RingInterval::point(x),
        RingInterval::point(y),
        RingInterval::point(z),
    ]
}

// ---------------------------------------------------------------
// The independent oracle: plain-f64 B-spline basis + derivatives
// (Cox-de Boor, no kernel spline code), rational surface point and
// partials through the quotient rule, composite Gauss-Legendre.
// ---------------------------------------------------------------

/// All basis values N_{i,p}(t) for a clamped knot vector.
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

/// Basis derivatives N'_{i,p}(t).
fn dbasis(knots: &[f64], degree: usize, ncp: usize, t: f64) -> Vec<f64> {
    let n = knots.len() - 1;
    // degree-(p-1) values over the full index range
    let mut low = basis(knots, degree - 1, n - (degree - 1), t);
    low.resize(n, 0.0);
    let mut out = vec![0.0f64; ncp];
    let pf = degree as f64;
    for (i, o) in out.iter_mut().enumerate() {
        let da = knots[i + degree] - knots[i];
        let db = knots[i + degree + 1] - knots[i + 1];
        let a = if da > 0.0 { pf / da * low[i] } else { 0.0 };
        let b = if db > 0.0 { pf / db * low[i + 1] } else { 0.0 };
        *o = a - b;
    }
    out
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

    /// Dense (flux, area) by composite 5-pt Gauss-Legendre per span
    /// rectangle, `cells` cells per span per axis.
    fn dense(&self, cells: usize) -> (f64, f64) {
        let gx = [
            -0.906_179_845_938_664,
            -0.538_469_310_105_683,
            0.0,
            0.538_469_310_105_683,
            0.906_179_845_938_664,
        ];
        let gw = [
            0.236_926_885_056_189,
            0.478_628_670_499_366,
            0.568_888_888_888_889,
            0.478_628_670_499_366,
            0.236_926_885_056_189,
        ];
        let spans = |knots: &[f64]| -> Vec<(f64, f64)> {
            let mut s = Vec::new();
            for w in knots.windows(2) {
                if w[1] > w[0] {
                    s.push((w[0], w[1]));
                }
            }
            s
        };
        let su = spans(&self.ku);
        let sv = spans(&self.kv);
        let mut flux = 0.0;
        let mut area = 0.0;
        for (ua, ub) in &su {
            for (va, vb) in &sv {
                for cu in 0..cells {
                    for cv in 0..cells {
                        let hu = (ub - ua) / cells as f64;
                        let hv = (vb - va) / cells as f64;
                        let u0 = ua + cu as f64 * hu;
                        let v0 = va + cv as f64 * hv;
                        for a in 0..5 {
                            for b in 0..5 {
                                let u = u0 + hu * 0.5 * (1.0 + gx[a]);
                                let v = v0 + hv * 0.5 * (1.0 + gx[b]);
                                let (s, sud, svd) = self.eval(u, v);
                                let cx = [
                                    sud[1] * svd[2] - sud[2] * svd[1],
                                    sud[2] * svd[0] - sud[0] * svd[2],
                                    sud[0] * svd[1] - sud[1] * svd[0],
                                ];
                                let f = s[0] * cx[0] + s[1] * cx[1] + s[2] * cx[2];
                                let g = (cx[0] * cx[0] + cx[1] * cx[1] + cx[2] * cx[2]).sqrt();
                                let wq = gw[a] * gw[b] * hu * hv * 0.25;
                                flux += wq * f;
                                area += wq * g;
                            }
                        }
                    }
                }
            }
        }
        (flux, area)
    }
}

/// Build the oracle patch from the same data handed to the kernel.
fn patch(
    ku: &KnotVector,
    kv: &KnotVector,
    control: &[[RingInterval; 3]],
    weights: &[f64],
) -> Patch {
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
    control: &[[RingInterval; 3]],
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
        ku,
        kv,
        control,
        weights,
        {
            let (a, b) = ku.domain();
            let (c, d) = kv.domain();
            (a, b, c, d)
        },
        perimeter,
        0.0,
        Tol::witness().get().eps,
        band(),
    );
    match out {
        Ok(fb) => {
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
        // The refusal arm asserts the SAME contract as the crate's own
        // `eps_posture` (quad.rs): a budget refusal carries a finite
        // width that really missed the run's own `1024·ε` target, an
        // escalation is the convergence predicate with a finite in-band
        // margin, and anything else is an unsound outcome.
        Err(e) => {
            println!("{name}: typed refusal (sound): {e}");
            let target = TARGET_LEN_FACTOR * Tol::witness().get().eps;
            match e {
                PropsError::QuadratureBudget {
                    width_len,
                    target_len,
                } => {
                    assert!(
                        width_len.is_finite() && width_len > target_len,
                        "{name}: a budget refusal must carry a width that really \
                         missed: {width_len:e} vs {target_len:e}"
                    );
                    assert!(
                        (target_len - target).abs() <= target * 1e-12,
                        "{name}: the refused target must BE 1024·ε for this run: \
                         {target_len:e} vs {target:e}"
                    );
                    println!("FLOOR {name} {width_len:.12e} target {target_len:.6e}");
                    Posture::Budget(width_len)
                }
                PropsError::Escalated { cause } => {
                    assert_eq!(
                        cause.predicate,
                        Some("props_quad_converged"),
                        "{name}: only the convergence predicate may escalate here: \
                         {cause:?}"
                    );
                    assert!(
                        matches!(cause.margin, MarginDiag::Value(m) if m.is_finite()),
                        "{name}: the escalation must carry a finite in-band margin: \
                         {cause:?}"
                    );
                    Posture::Escalated
                }
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
    let kv2 = KnotVector::unit_segment(2);
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
    let kv2 = KnotVector::unit_segment(2);
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
    let ku = KnotVector::unit_segment(2);
    let kv = KnotVector::unit_segment(1);
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
    let kv1 = KnotVector::unit_segment(1);
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
    // really cannot bracket it usefully, and a budget refusal carrying
    // its width says exactly that, where `DegenerateFace` said the
    // face had no extent — which was false of it. The enclosure is
    // still uselessly wide, so the capability gap this carrier exists
    // to hold still is intact; only its spelling improved.
    pin_floor("extreme-weight-square", posture, SQUARE_FLOOR);
}

/// Warped bilinear (hyperbolic paraboloid) with mixed extreme
/// weights: dense oracle only.
#[test]
fn probe_extreme_weight_hypar() {
    let kv1 = KnotVector::unit_segment(1);
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
    let ku = KnotVector::unit_segment(2);
    let kv = KnotVector::unit_segment(1);
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
/// The 1 km carrier's floor is 1e3× the metre-scale one and refuses on
/// every row of the matrix.
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
///   target is ε-coupled against a FIXED schedule (D9), so at
///   ε = 1e-12 every rational carrier here honestly refuses.
///
/// (Containment is checked inside [`probe`]: certifying with a bracket
/// that excluded the truth fails there, not here.)
#[test]
fn probe_suite_still_certifies_something() {
    let kv1 = KnotVector::unit_segment(1);
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
    let kv = KnotVector::unit_segment(1);
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
    pin_floor("half-cylinder-2span", posture, HALF_CYLINDER_FLOOR);
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
    let kv = KnotVector::unit_segment(1);
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
        None,
        Some(5.0),
    );
    pin_floor("c0-kink-wall", posture, C0_KINK_FLOOR);
}

/// D9 determinism: the same rational flux computed twice must be
/// BIT-identical (also printed for the debug/release cross-check).
#[test]
fn probe_determinism_bits() {
    let ku = KnotVector::unit_segment(2);
    let kv = KnotVector::unit_segment(1);
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
    let run = || {
        nurbs_patch_face::<f64>(
            &ku,
            &kv,
            &net,
            &weights,
            (0.0, 1.0, 0.0, 1.0),
            4.0 + PI,
            0.0,
            Tol::witness().get().eps,
            band(),
        )
    };
    // Determinism is the claim, and it holds on EVERY ε row: below the
    // corpus ε the fixed schedule cannot meet the ε-coupled target and
    // the honest outcome is a typed refusal, which must ITSELF be
    // reproduced identically. Only a changed outcome class, or an
    // untyped error, is a failure.
    match (run(), run()) {
        (Ok(a), Ok(b)) => {
            println!(
                "DETBITS flux {:016x} {:016x} area {:016x} {:016x}",
                a.flux.lo().to_bits(),
                a.flux.hi().to_bits(),
                a.area.lo().to_bits(),
                a.area.hi().to_bits()
            );
            assert_eq!(a.flux.lo().to_bits(), b.flux.lo().to_bits());
            assert_eq!(a.flux.hi().to_bits(), b.flux.hi().to_bits());
            assert_eq!(a.area.lo().to_bits(), b.area.lo().to_bits());
            assert_eq!(a.area.hi().to_bits(), b.area.hi().to_bits());
            // the meter, as claimed: width(flux)/(3*area_mid)
            let meter = a.flux.width() / (3.0 * (a.area.lo() + a.area.hi()) * 0.5);
            println!(
                "METER quarter-cylinder {:.6e} target {:.6e}",
                meter,
                1024.0 * Tol::witness().get().eps
            );
        }
        (Err(a), Err(b)) => {
            println!(
                "DETBITS refusal @ eps={:e}: {a:?}",
                Tol::witness().get().eps
            );
            assert!(
                matches!(
                    a,
                    PropsError::QuadratureBudget { .. } | PropsError::Escalated { .. }
                ),
                "an ε row must refuse TYPED, not otherwise: {a:?}"
            );
            assert_eq!(
                format!("{a:?}"),
                format!("{b:?}"),
                "the refusal is not reproduced identically"
            );
        }
        (x, y) => panic!("a same-process re-run changed outcome class: {x:?} vs {y:?}"),
    }
    // budget-defeat check lives in probe_budget_refusal below
    let refused = nurbs_patch_face::<f64>(
        &ku,
        &kv,
        &net,
        &weights,
        (0.0, 1.0, 0.0, 1.0),
        4.0 + PI,
        0.0,
        Tol::witness().get().eps * 1e-6,
        band(),
    );
    // A million-fold-tighter target must REFUSE, never answer — but
    // which typed refusal is ε-dependent, because the funnel band is
    // built from the run's AMBIENT ε while the target came from the
    // scaled one. When the shortfall is outside `Band{ε, Kε}` the
    // schedule runs out and the budget refuses; when the ambient band
    // is coarse enough to swallow it (measured: ambient 1e-6 gives
    // margin −9.8e-6 inside `Band{1e-6, 1e-5}`), `props_quad_converged`
    // escalates first. Both are honest and both are pinned; `Ok` is
    // the only outcome ruled out.
    match refused {
        Err(PropsError::QuadratureBudget {
            width_len,
            target_len,
        }) => {
            println!("BUDGET width_len {width_len:.15e} target {target_len:.6e}");
            assert!(width_len.is_finite() && width_len > target_len);
            // The schedule is fixed (D9), so this carrier bottoms out
            // at the same displacement whatever the target is. The
            // payload is the last round's lower bound (the loop refuses
            // after round 0 once it has proved the schedule cannot
            // certify): 2.0e-5 relative under the pinned measurement,
            // which the window below covers without a digit moving.
            assert!(
                (width_len - QUARTER_CYLINDER_FLOOR).abs() <= 1e-3 * QUARTER_CYLINDER_FLOOR,
                "the quarter cylinder's refusal floor MOVED: {width_len:e} against \
                 the pinned {QUARTER_CYLINDER_FLOOR:e}"
            );
        }
        Err(PropsError::Escalated { cause }) => {
            println!("ESCALATED {cause:?}");
            assert_eq!(
                cause.predicate,
                Some("props_quad_converged"),
                "only the convergence predicate may escalate here: {cause:?}"
            );
        }
        other => panic!("a 1e-6-scaled eps must refuse typed, got {other:?}"),
    }
}

/// DIAGNOSIS: is certified knot refinement exact on the two-span
/// half-circle? Refine the HOMOGENEOUS 1-D net with the kernel's own
/// refine_plan chain (weights-1 plans, plain lerp — exactly what
/// refine_dir does) and compare rational curve points before/after
/// with the independent evaluator.
#[test]
fn diag_refine_half_circle() {
    use geom_core::spline::algebra::refine_plan;
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let pts = [(1.0, 0.0), (1.0, 1.0), (0.0, 1.0), (-1.0, 1.0), (-1.0, 0.0)];
    let ws = [1.0, W2, 1.0, W2, 1.0];
    // homogeneous 3-vectors (x*w, y*w, w)
    let mut hom: Vec<[f64; 3]> = pts
        .iter()
        .zip(&ws)
        .map(|((x, y), w)| [x * w, y * w, *w])
        .collect();
    let add: Vec<f64> = (1..16)
        .map(|k| k as f64 / 16.0)
        .filter(|t| !kv.knots().contains(t))
        .collect();
    let plans = refine_plan(&kv, &vec![1.0; 5], &add).unwrap();
    let mut cur_kv = kv.clone();
    for plan in &plans {
        hom = plan.apply_points(&hom, [f64::NAN; 3], |x, y, l| {
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
    let kv = KnotVector::unit_segment(1);
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
        nurbs_patch_face::<f64>(
            &ku,
            &kv,
            &net,
            ws,
            (0.0, 1.0, 0.0, 1.0),
            2.0 * h + 2.0 * PI,
            0.0,
            Tol::witness().get().eps,
            band(),
        )
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
    // The RATIONAL twin's target is ε-coupled against a FIXED schedule,
    // so below the corpus ε its honest outcome is a typed refusal. Both
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
            assert!(
                rat.flux.lo() <= of && of <= rat.flux.hi(),
                "rational lane excludes the oracle: [{}, {}] vs {of}",
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
                    PropsError::QuadratureBudget { .. } | PropsError::Escalated { .. }
                ),
                "the rational twin must refuse TYPED: {e:?}"
            );
        }
    }
}

/// Genericity spot check: the rational lane driven by the certified
/// Interval decision scalar must agree with the f64 lane bit-for-bit
/// on the returned enclosure (the RingInterval arithmetic is shared;
/// only decisions route through T).
#[cfg(feature = "interval")]
#[test]
fn probe_interval_scalar_agrees() {
    use geom_core::Interval;
    let ku = KnotVector::unit_segment(2);
    let kv = KnotVector::unit_segment(1);
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
    let a = nurbs_patch_face::<f64>(&ku, &kv, &net, &weights, rect, 4.0 + PI, 0.0, eps, band());
    let b =
        nurbs_patch_face::<Interval>(&ku, &kv, &net, &weights, rect, 4.0 + PI, 0.0, eps, band());
    // The agreement claim is ε-independent and covers the refusals too:
    // below the corpus ε the ε-coupled target is out of the fixed
    // schedule's reach and BOTH scalars must refuse, identically.
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
                    PropsError::QuadratureBudget { .. } | PropsError::Escalated { .. }
                ),
                "an ε row must refuse TYPED: {a:?}"
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
