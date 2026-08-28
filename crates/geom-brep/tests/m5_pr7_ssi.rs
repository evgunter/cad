//! **M5 PR 7 acceptance** — rung 3 of the C1 ladder: march-then-certify
//! with in-op exhaustiveness (spec §6).
//!
//! The rows, in the spec's order:
//!
//! 1. **Shape (iv), the milestone's signature.** A planted fixture whose
//!    whole intersection is two *interior* loops that touch no domain
//!    boundary: boundary-curve seeding provably cannot reach either, and
//!    a marcher launched from one of them finds exactly one. Both are
//!    found — because the subdivision enumerated them — and both certify.
//! 2. **The found-AND-floor-refused variant.** The same fixture with the
//!    accounting floor clamped above the certified tube radius: the
//!    branches are still found, and the operation nonetheless refuses
//!    `SsiExhaustivenessInconclusive` rather than claim they are all of
//!    them. Never silence, in both directions.
//! 3. **Shape (iii) substrate.** A directly-authored NURBS wall cut by a
//!    plane: marched in ℝ⁴, fitted, all three limbs.
//! 4. **The OQ4 demonstration.** The ℝ⁴ trace's two pcurves are
//!    coordinate projections of one parameterized object, so
//!    `|S(P(t)) − C(t)| ≤ ε` holds on the **carrier's own parameter** at
//!    the PR 6 cache schedule — the parameter-identity contract, checked
//!    the way PR 6's door checks it.
//! 5. **The corrupted-cache limb rows.** Each limb refuses separately.
//! 6. **The σ₂-sliver row**, refusing toward C7.
//! 7. **The closure trio**, exercised as verdicts: a closed loop, a
//!    boundary-terminated branch, and the tangent-match arm.
//! 8. **Idealized vs realized**, the T4 differential pin.
//!
//! Beside them, and not in the spec: **the sweeps' never-silence
//! doors** — the refusal sites in `ssi/exhaust.rs` and the chart-speed
//! guard in `ssi.rs` that makes the sweep's floor meaningful. Their own
//! block below carries the grid of doors, which rows sit in which cell,
//! and which cells still have none.
//!
//! # Two SSI operations, not ten (the test-cost audit)
//!
//! Rows 1, 5 and 8 — plus the fit-budget row, the accounting receipt and
//! the dedup row — are all questions about ONE `cylinder_sphere_ssi`
//! call on the planted fixture, and rows 3 and the corrupted-pcurve half
//! of row 5 are questions about ONE `plane_nurbs_ssi` call on the
//! substrate wall. nextest runs one process per test, so the `OnceLock`s
//! that claimed to compute each fixture "once per process" shared
//! nothing and the suite paid each operation once PER ROW. There is now
//! one test per fixture:
//! [`the_planted_fixture_is_found_certified_limbed_accounted_and_deduplicated`]
//! and
//! [`shape_iii_the_wall_cut_certifies_all_three_limbs_and_refuses_a_corrupted_pcurve`].
//! Each labels every assertion with the property it belongs to, so a
//! merged failure still names what broke; each retired row's name is
//! recorded on the merged row's doc comment. [`shape_iii_bit_replay`]
//! deliberately keeps its own test — it runs the operation TWICE and the
//! second run is its whole content.
//!
//! **What the never-silence block costs, since that audit binds
//! whoever adds to this file.** Six rows, nine SSI operations, four
//! of them on the substrate wall the audit merged rows to stop paying
//! for. They are not mergeable on that fixture, and the reason is the
//! audit's own criterion: the merged rows above are *several questions
//! about one outcome*, so one call answers them all, whereas each row
//! below is a question about **which door answered**, and a door
//! answers by ending the operation. Two rows cannot share a call when
//! each one's content is that the call stopped somewhere different.
//! The mode pins are the only genuinely shared work, and the two
//! cell-budget rows now share one helper rather than one call.
//!
//! Every ε stand-down in this file is LOUD **and PROVED**: the run
//! prints, by name ([`test_utils::vacuity::stood_down`]), the coverage it
//! did not deliver, and it first asserts that the excuse is the one it
//! claims — D9's `SSI_MAX_FIT_SAMPLES`, genuinely overrun, at an ε finer
//! than the compiled default. A stand-down that is only announced is
//! still a row that greens without entering its own mode the day the
//! budget starts firing everywhere. The retired `fixture_or_return!` /
//! `carrier_or_return!` macros returned green in silence, which is the
//! honesty gap `docs/M5-EXIT-WALK.md` row 15 recorded.
//!
//! **Planted quantities are stated in metres, not in multipliers.** The
//! accounting floor is `SSI_FLOOR · band.zero() · floor_scale`, so a
//! literal `floor_scale` names a different width at every ε. Every floor
//! fixture in this file states its width and converts through
//! [`SsiDomain::floor_scale_for`] — the inverse of `SsiDomain::floor`,
//! which is where the identity lives rather than in a comment.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unreachable
)]

use geom::{Curve3, NurbsCurve3};
use geom::{NurbsSurface, Surface};
use geom_brep::CERT_SAMPLES;
use geom_brep::ssi::BranchEnd;
use geom_brep::ssi::{
    self, SSI_FLOOR, SSI_MAX_CELLS, SSI_MAX_FIT_SAMPLES, SSI_SEED_FLOOR, SSI_TUBE_RADIUS,
    SsiDomain, SsiError, SsiLimb, SsiOperand, TubeScale,
};
use geom_core::Tol;
use geom_core::spline::KnotVector;
use geom_core::tolerance::DEFAULT_EPS;
use geom_core::{Band, Margin, Point3, Vec3};
use test_utils::vacuity;

/// The accounting floor the floor-clamped fixture plants, **in metres**
/// — far wider than any certifiable tube radius on that pair, and the
/// same width at every ε of the battery.
const FLOOR_CLAMP_METRES: f64 = 0.1;

fn eps() -> f64 {
    Tol::witness().get().eps
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// A margin the resolved band calls **definitely positive**, at any ε.
///
/// The battery runs this suite at ε ∈ {1e-6, 1e-9, 1e-12} and at the
/// interval scalar, so every probe value and every planted corruption
/// has to be placed *relative to the band the run resolved*, never at a
/// literal that happens to straddle the default one. `Band::linear(Tol::witness())`
/// puts `zero` at ε and `escalate` at K·ε; these three helpers name the
/// three regions.
fn definitely_positive() -> f64 {
    band().escalate() * 100.0
}

/// A margin inside the F6 escalation band at any ε — strictly between
/// `zero` and `escalate`, placed at their midpoint so no rounding of K
/// can push it out either side.
fn inside_the_band() -> f64 {
    0.5 * (band().zero() + band().escalate())
}

/// The distance from `p` to the carrier **as a set**, measured honestly
/// on a closed curve.
///
/// `NurbsCurve3::project` converges to a *stationary* point of the
/// distance and says so in its own docs; on a closed carrier a foot
/// near the seam can settle at the clamped domain end and report a
/// residual three orders too large (5.6e-4 m where the true distance is
/// ~0, observed at ε = 1e-6). That is the projection being honest, not
/// wrong — but a differential row that took it at face value would be
/// measuring the projection, not the two steppers. So: a fixed coarse
/// parameter scan picks the seed, Newton refines from there, and the
/// scan's own minimum is kept as the fallback bound.
fn distance_to_carrier(carrier: &NurbsCurve3<f64>, p: Point3<f64>) -> f64 {
    let (t0, t1) = carrier.domain();
    let n = 8 * carrier.control().len();
    let mut best = (t0, f64::INFINITY);
    for i in 0..=n {
        #[allow(clippy::cast_precision_loss)]
        let t = t0 + (t1 - t0) * (i as f64 / n as f64);
        let d = (carrier.eval(t) - p).norm();
        if d < best.1 {
            best = (t, d);
        }
    }
    match carrier.project_from_seed(p, best.0) {
        Ok(pr) => pr.distance.min(best.1),
        Err(_) => best.1,
    }
}

// ---------------------------------------------------------------------
// Shape (iv): the planted two-interior-loop fixture
// ---------------------------------------------------------------------

/// The unit sphere at the origin.
fn sphere() -> Surface<f64> {
    Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: 1.0,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// A thin cylinder threaded through the sphere, offset from the axis so
/// the locus is a genuine quartic and not a pair of exact circles.
///
/// Because `radius + offset = 0.11 < 1`, the cylinder wall never reaches
/// the sphere's equator: the intersection is **two disjoint small loops**
/// near the poles (`z ≈ ±0.996`, each about 0.5 m long), and *nothing*
/// else. Neither touches the session slab's boundary, so
/// boundary-curve×surface seeding — the only seeding a marcher gets for
/// free — finds nothing at all. This is the classic silent disaster the
/// banked principle names, planted deliberately.
fn threaded_cylinder() -> Surface<f64> {
    Surface::Cylinder {
        origin: Point3::new(0.03, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 0.08,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

fn slab() -> SsiDomain {
    SsiDomain {
        center: Point3::new(0.0, 0.0, 0.0),
        half_extent: 1.5,
        extent: 2.0,
        floor_scale: 1.0,
    }
}

/// **The planted fixture, built ONCE, and every row that reads it.**
///
/// Eight rows until the test-cost audit, all of them questions about
/// the SAME `cylinder_sphere_ssi` call on the SAME planted shape:
///
/// | retired row | block below |
/// |---|---|
/// | `the_fit_sample_budget_refuses_typed_rather_than_grinding` | `BUDGET` |
/// | `shape_iv_both_interior_loops_are_found_and_certified` | `SHAPE-IV` |
/// | `the_accounting_receipt_is_bounded_and_reported` | `RECEIPT` |
/// | `a_good_carrier_certifies_all_three_limbs` | `LIMBS` |
/// | `corrupting_a_carrier_grossly_fails_the_on_locus_limb` | `LIMB-1` |
/// | `a_between_samples_excursion_is_caught_by_the_hull_limb_alone` | `LIMB-2` |
/// | `a_seed_refining_onto_a_found_branch_does_not_mint_a_duplicate` | `DEDUP` |
/// | `idealized_and_realized_steppers_agree_on_the_locus_they_trace` | `DIFFERENTIAL` |
///
/// # Why one row and not eight
///
/// A rung-3 intersection at ε = 1e-9 legitimately produces a carrier
/// with several hundred control points (the geometry's own requirement:
/// a cubic needs that many spans to stay inside ε on a loop of 0.08 m
/// radius), and the interpolation solve is cubic in that — measured
/// ~3.2 s per call at the default ε. Two `OnceLock`s (`fixture_or_\
/// budget` and `good_carrier`) used to say that cost was paid "once per
/// process". It was: nextest runs ONE PROCESS PER TEST, so the memo
/// shared nothing across the eight rows and the suite paid the
/// operation eight times per ε row for one operation's worth of
/// coverage. The memos are gone with the split that needed them.
///
/// What the split bought and a merged row cannot is failure ISOLATION:
/// eight independent properties now surface under one test id. So every
/// assertion NAMES its property — `BUDGET`, `SHAPE-IV`, `RECEIPT`,
/// `LIMBS`, `LIMB-1`, `LIMB-2`, `DEDUP`, `DIFFERENTIAL` — and the
/// message alone says which one broke. Keep that discipline when adding
/// assertions here.
///
/// # The ε stand-down, said out loud
///
/// The battery runs at ε ∈ {1e-6, 1e-9, 1e-12}. The step rule spaces
/// samples as `ε^{−1/4}`, so this 0.08 m loop wants ~126 samples at
/// 1e-6, ~570 at 1e-9 and ~4000 at 1e-12 — and the fit's solve is cubic
/// in that. At the finest row the operation therefore refuses
/// [`SsiError::FitSampleBudget`], which is the kernel behaving exactly
/// as designed (a named budget, a typed refusal, never a carrier fitted
/// from too coarse a set).
///
/// This is a **skip gated on a typed kernel refusal**, not on an ε
/// literal. Scaling the fixture instead would mean holding `r/ε`
/// constant — an 80 m loop at 1e-6 and an 0.08 mm one at 1e-12 — which
/// stops being the planted small-loop shape the row exists to test.
///
/// The retired `fixture_or_return!` / `carrier_or_return!` macros made
/// that stand-down a bare `return`, so a row that asserted NOTHING
/// reported green and nothing in the log said which it had been — the
/// honesty gap `docs/M5-EXIT-WALK.md` row 15 records. The `BUDGET` arm
/// below still pins the refusal typed, and then SAYS, by name, every
/// property this run did not cover.
#[test]
fn the_planted_fixture_is_found_certified_limbed_accounted_and_deduplicated() {
    let (s, c) = (sphere(), threaded_cylinder());
    // BUDGET: whichever ε the run resolved, the operation either
    // produces certified branches or says — in one typed error, naming
    // the fix — that this tolerance and this curvature need more
    // control points than the fit can afford. It never truncates the
    // sample set.
    let out = match ssi::cylinder_sphere_ssi(&c, &s, slab(), band()) {
        Ok(o) => {
            assert_eq!(
                o.branches.len(),
                2,
                "BUDGET: an operation that fits at all fits both loops"
            );
            o
        }
        Err(SsiError::FitSampleBudget { samples, budget }) => {
            assert!(samples > budget, "BUDGET: {samples} vs {budget}");
            let msg = format!("{}", SsiError::FitSampleBudget { samples, budget });
            assert!(msg.contains("fit budget"), "BUDGET: {msg}");
            assert!(msg.contains("raise the tolerance"), "BUDGET: {msg}");
            vacuity::stood_down(
                &format!("planted fixture, eps = {:e}", eps()),
                &format!(
                    "the SSI door refused typed on its named fit-sample budget \
                     ({samples} samples vs {budget}). THIS RUN CONTRIBUTES NO \
                     SHAPE-(iv) COVERAGE — no found-and-certified row, no three-limb \
                     row, no limb-1/limb-2 separation, no accounting receipt, no dedup \
                     row, no idealized-vs-realized differential. Only the BUDGET \
                     assertions above executed."
                ),
            );
            return;
        }
        Err(e) => panic!("unexpected: {e}"),
    };
    println!(
        "EPS-ROW planted fixture @ eps = {:e}: CERTIFIED — {} branches, {} seeds",
        eps(),
        out.branches.len(),
        out.seeds
    );

    // ---- SHAPE-IV: both interior loops are found AND certified -----
    assert_eq!(
        out.branches.len(),
        2,
        "SHAPE-IV: expected two interior loops, got {} (exhaustiveness {:?})",
        out.branches.len(),
        out.exhaustiveness
    );
    for b in out.branches.iter() {
        assert_eq!(
            b.end,
            BranchEnd::Closed,
            "SHAPE-IV: an interior loop must close"
        );
        // The full three-limb certificate rode along.
        assert!(
            b.certificate.on_locus_max <= eps(),
            "SHAPE-IV: {:?}",
            b.certificate
        );
        assert!(
            b.certificate.hull_sup <= eps(),
            "SHAPE-IV: {:?}",
            b.certificate
        );
        assert!(
            b.certificate.tube_boxes >= 1,
            "SHAPE-IV: {:?}",
            b.certificate
        );
        // MARCH-TOL: the tolerance the carrier was actually GENERATED
        // at, read off the branch the door returned.
        //
        // The end-to-end counterpart of the `MarchTol` unit rows. Those
        // test the derivation as a pure function; this one tests the
        // door, which is where the divergence would be reintroduced —
        // a certifying door minting its own decoupled generator
        // tolerance is one line, changes no public signature, and every
        // other assertion in this file stays green through it. This
        // receipt does not, and the seam refuses before it is even
        // reached.
        assert_eq!(
            b.march_tol,
            band().zero(),
            "MARCH-TOL: the carrier was generated at {:e} m while the run is banded \
             at {:e} m — the certificate and the generator disagree about ε",
            b.march_tol,
            band().zero()
        );
        // TUBE-FLOOR: the certified tube is stated in the RUN's ε.
        //
        // This row exists to go red when the *guarantee degrades*, not
        // only when it is violated. The tube ladder's floor is
        // `SSI_TUBE_RADIUS · ε`, and ε here means the band's — the same
        // number the limbs' trileans decide against. Feed the ladder a
        // tolerance finer than the run's and the floor drops with it:
        // a thinner tube certifies, the uniqueness theorem shipped
        // under `SsiCertificate` gets weaker, and every existing
        // assertion above still passes, because they are all monotone
        // in the easy direction. This one is not.
        assert!(
            b.certificate.tube_radius >= SSI_TUBE_RADIUS * band().zero(),
            "TUBE-FLOOR: a tube of {:e} m is below the run band's own floor \
             ({} · {:e} m) — the certificate was obtained at a finer tolerance \
             than the one it is banded at: {:?}",
            b.certificate.tube_radius,
            SSI_TUBE_RADIUS,
            band().zero(),
            b.certificate
        );
        assert!(
            b.certificate.tube_transversality > 0.0,
            "SHAPE-IV: the uniqueness tube must have positive headroom: {:?}",
            b.certificate
        );
        // The witness is carrier(mid), unchanged from M2.
        let Curve3::Nurbs(ref n) = b.carrier else {
            panic!("a rung-3 carrier is a NURBS curve");
        };
        let (t0, t1) = n.domain();
        let mid = n.eval(0.5 * (t0 + t1));
        assert!(
            (b.witness - mid).norm() < 1e-15,
            "SHAPE-IV: the witness is carrier(mid)"
        );
    }
    // The two loops are near opposite poles, so they are genuinely
    // distinct components and not the same one found twice.
    let z: Vec<f64> = out.branches.iter().map(|b| b.witness.z).collect();
    assert!(
        z[0] * z[1] < 0.0,
        "SHAPE-IV: expected one loop per pole, got {z:?}"
    );
    // The never-silence receipt.
    assert!(
        out.exhaustiveness.excluded > 0,
        "SHAPE-IV: {:?}",
        out.exhaustiveness
    );
    assert!(
        out.exhaustiveness.accounted > 0,
        "SHAPE-IV: {:?}",
        out.exhaustiveness
    );
    assert!(out.seeds >= 2, "SHAPE-IV: seeds = {}", out.seeds);

    // ---- RECEIPT: the exhaustiveness contract is also AFFORDABLE ----
    //
    // A subdivision that must refine to ε along the whole locus is a
    // hang, not a proof. What makes it terminate is the certified tube
    // radius — cells are accounted at the geometry's own scale — so
    // this block pins both the accounting numbers and the radius they
    // depend on, and would fail loudly if the tube ladder ever started
    // bottoming out.
    let e = out.exhaustiveness;
    println!("exhaustiveness = {e:?}, seeds = {}", out.seeds);
    for b in out.branches.iter() {
        println!("certificate = {:?}", b.certificate);
    }
    assert!(e.examined > 0, "RECEIPT: {e:?}");
    assert_eq!(
        e.examined,
        e.excluded + e.accounted + e.refined,
        "RECEIPT: the receipt must add up: every cell excluded, accounted, or split"
    );
    assert!(
        e.excluded > 0 && e.accounted > 0,
        "RECEIPT: both terminal states must be exercised: {e:?}"
    );
    assert!(
        e.examined < 100_000,
        "RECEIPT: the accounting should terminate at the tube's scale, not at ε: {e:?}"
    );

    // ---- The corrupted-cache limb rows, on the fixture's own carrier
    let Curve3::Nurbs(ref first) = out.branches[0].carrier else {
        panic!("a rung-3 carrier is a NURBS curve");
    };
    let carrier: NurbsCurve3<f64> = (**first).clone();

    // LIMBS: the carrier the operation produced re-certifies, all three.
    let cert = certify_against(&carrier).expect("LIMBS: the branch this came from certified");
    assert_eq!(cert.samples, CERT_SAMPLES, "LIMBS: the PR 6 schedule");
    assert!(
        cert.hull_sup <= eps(),
        "LIMBS: hull sup {:e}",
        cert.hull_sup
    );
    assert!(
        cert.tube_transversality > 0.0,
        "LIMBS: tube margin {:e}",
        cert.tube_transversality
    );

    // LIMB-1: a gross corruption fails the on-locus limb.
    //
    // A hundred escalation-bands' worth of displacement: whatever ε the
    // run resolved, the schedule sees this immediately. A literal
    // (a micron, say) is definitely-outside only at the default ε and
    // silently *inside* limb 1 at ε = 1e-6, where the row would then be
    // asserting the kernel is wrong for being right.
    let n = carrier.control().len() / 2;
    let bad = displaced(&carrier, n, definitely_positive());
    match certify_against(&bad) {
        Err(SsiError::CertificateLimb { limb, value }) => {
            assert_eq!(limb, SsiLimb::OnLocus, "LIMB-1: value = {value}");
        }
        other => panic!("LIMB-1: expected limb 1 to refuse, got {other:?}"),
    }

    // LIMB-2 (C2.2's whole reason to exist): a displacement small
    // enough that the nine-point schedule still passes, but large
    // enough that the certified control-hull bound does not. The
    // sampled max steers; the hull bound certifies.
    //
    // Deliberately NOT the middle: a control point at the parameter
    // midpoint sits on a schedule sample, so its bump would be seen by
    // limb 1 and the row would be testing nothing. Three sixteenths
    // lands between samples 1 and 2 of the nine.
    let n = carrier.control().len() * 3 / 16;
    let mut found = None;
    // A deterministic ascending scan — the planted excursion is looked
    // for, not guessed at.
    for k in 1..=200u32 {
        let d = eps() * 0.05 * f64::from(k);
        let bad = displaced(&carrier, n, d);
        match certify_against(&bad) {
            Err(SsiError::CertificateLimb {
                limb: SsiLimb::HullSup,
                value,
            }) => {
                found = Some((d, value));
                break;
            }
            // A hull bound that lands just ABOVE ε is inside the
            // escalation band, so limb 2 speaks as an F6 escalation
            // rather than a definite refusal. Same limb, same meaning —
            // and the predicate name is how they are told apart.
            Err(SsiError::Escalated(ref diag)) if diag.predicate == Some("ssi_hull_sup") => {
                found = Some((d, f64::NAN));
                break;
            }
            // Still inside both limbs, or already gross enough to trip
            // limb 1 — keep scanning / stop.
            Ok(_) => {}
            Err(SsiError::CertificateLimb {
                limb: SsiLimb::OnLocus,
                ..
            }) => break,
            // The scan walks a displacement across ε, so the schedule's
            // own trilean legitimately lands in the escalation band on
            // the way past: that is limb 1 speaking, and the scan is
            // over.
            Err(SsiError::Escalated(ref diag)) if diag.predicate == Some("ssi_on_locus") => {
                break;
            }
            Err(e) => panic!("LIMB-2: unexpected refusal while scanning: {e}"),
        }
    }
    let (d, value) = found.expect(
        "LIMB-2: no displacement made the hull bound refuse while the sampled schedule \
         still passed — either the hull bound is not tighter than the schedule, or the \
         scan range is wrong; both are defects worth failing on",
    );
    assert!(
        value.is_nan() || value > eps(),
        "LIMB-2: the hull bound must exceed ε: {value}"
    );
    // And the same displacement passes limb 1 on its own schedule.
    let bad = displaced(&carrier, n, d);
    let (t0, t1) = bad.domain();
    for i in 0..CERT_SAMPLES {
        let t = t0 + (t1 - t0) * (f64::from(i) / f64::from(CERT_SAMPLES - 1));
        let r = geom_brep::implicit_residual(&c, bad.eval(t)).abs();
        assert!(
            r <= eps(),
            "LIMB-2: the schedule must NOT see it: sample {i} = {r}"
        );
    }

    // ---- DEDUP: the dedup guards the LANDING point, not the seed ----
    //
    // This block builds the case that distinguishes them: a point well
    // outside every tube (so the old location test would have let it
    // through) whose Newton refinement lands squarely on a branch
    // already found.
    assert_eq!(
        out.branches.len(),
        2,
        "DEDUP: the fixture has two components"
    );
    let radius = out.branches[0].certificate.tube_radius;
    // A point off the locus by several tube radii — a tube box is the
    // span hull padded by exactly `radius`, so nothing contains this.
    let off = out.branches[0].witness + Vec3::new(0.0, 0.0, 3.0 * radius);
    assert!(
        carrier.project(off).unwrap().distance > radius,
        "DEDUP: the probe must start outside every tube or the row proves nothing"
    );
    // Marched from there, it re-traces the branch that already exists:
    // Newton pulls the seed onto the locus first, and the component it
    // lands on is the one already found. Under the old seed-LOCATION
    // dedup this seed passed the filter and this trace became a second
    // `SsiBranch` for one component.
    let (pts, end) = ssi::idealized_trace_r3(&c, &s, off, slab(), band())
        .expect("DEDUP: the off-locus probe refines onto the locus");
    assert_eq!(
        end,
        BranchEnd::Closed,
        "DEDUP: it re-traced a closed component"
    );
    for (i, p) in pts.iter().enumerate() {
        assert!(
            carrier.project(*p).unwrap().distance <= radius,
            "DEDUP: sample {i} of the duplicate trace left the found branch's tube"
        );
    }
    // And the operation minted no duplicate: two components, two
    // branches, witnesses on opposite poles.
    let z: Vec<f64> = out.branches.iter().map(|b| b.witness.z).collect();
    assert!(
        z[0] * z[1] < 0.0,
        "DEDUP: one branch per component, got {z:?}"
    );

    // ---- DIFFERENTIAL: the T4 / PERF-PLAN §4.4 pin ------------------
    //
    // The two steppers place different samples at different arc lengths
    // by construction, so the pin is on the LOCUS: same branch
    // topology, and every idealized sample within the realized branch's
    // own certified band of the realized carrier.
    for b in out.branches.iter() {
        let Curve3::Nurbs(ref realized) = b.carrier else {
            panic!("a rung-3 carrier is a NURBS curve");
        };
        let seed = b.witness;
        let (pts, end) = ssi::idealized_trace_r3(&c, &s, seed, slab(), band())
            .expect("DIFFERENTIAL: the idealized trace");
        assert_eq!(
            end, b.end,
            "DIFFERENTIAL: the two steppers must agree on branch topology"
        );
        // The band the realized branch actually earned, plus a term for
        // the idealized stepper's own Newton residual — both scale with
        // the resolved ε, so this row means the same thing at every row
        // of the battery.
        let tol = b.certificate.hull_sup + 2.0 * eps();
        for (i, p) in pts.iter().enumerate() {
            let d = distance_to_carrier(realized, *p);
            assert!(
                d <= tol,
                "DIFFERENTIAL: idealized sample {i} is {d:e} m off the realized carrier \
                 (band {tol:e})"
            );
        }
    }
}

#[test]
fn a_single_seeded_march_finds_only_one_of_the_two_loops() {
    // The row that makes row 1 mean something: marching is a candidate
    // generator, and a candidate generator launched once returns one
    // candidate. Found-ness comes from the subdivision, not from luck.
    let (s, c) = (sphere(), threaded_cylinder());
    let seed = Point3::new(0.11, 0.0, 0.994);
    let (pts, end) = ssi::idealized_trace_r3(&c, &s, seed, slab(), band()).expect("a seeded trace");
    assert_eq!(end, BranchEnd::Closed);
    // Every sample of the single trace sits on ONE pole's loop.
    assert!(
        pts.iter().all(|p| p.z > 0.0),
        "one march cannot reach the other component"
    );
}

/// The planted fixture with the accounting floor clamped far above any
/// certifiable tube radius: cells along the locus can be neither
/// excluded nor accounted, so the operation refuses instead of
/// reporting an intersection it cannot prove complete.
///
/// **The name states only what the row checks.** It was
/// `..._even_though_branches_were_found`, whose premise the body never
/// verifies and structurally cannot: the row only ever holds an `Err`,
/// and a refusal carries no branch count. Branch-found and branch-free
/// runs reach the identical refusal, so from inside this row the
/// premise is not merely unchecked but undecidable — the reason the
/// all-seeds-fail mode needed its own fixture rather than a widening of
/// this one (see
/// [`an_unseeded_run_refuses_typed_rather_than_receipting_an_unprovable_domain`]).
#[test]
fn the_floor_clamped_planted_fixture_refuses_typed() {
    let (s, c) = (sphere(), threaded_cylinder());
    // One loop's neighbourhood: the row is about the floor, not about
    // finding both components, and a rung-3 op is not cheap.
    let mut d = SsiDomain {
        center: Point3::new(0.03, 0.0, 0.996),
        half_extent: 0.2,
        extent: 0.4,
        floor_scale: 1.0,
    };
    // The clamp is stated in METRES, through the same door as this
    // file's other three floor fixtures. A literal multiplier states a
    // different width at every ε: a fixed `1.0e8` reads 0.1 m only at
    // the compiled default, and at ε = 1e-12 it is 1e-4 m — under which
    // the same fixture, shrunk, returns `Ok` instead of refusing. The
    // premise this row rests on is the floor's width.
    d.floor_scale = SsiDomain::floor_scale_for(FLOOR_CLAMP_METRES, band());
    let err = ssi::cylinder_sphere_ssi(&c, &s, d, band()).expect_err("must refuse");
    let msg = format!("{err}");
    match err {
        SsiError::ExhaustivenessInconclusive {
            cell_width, floor, ..
        } => {
            assert!(cell_width <= floor, "{cell_width} vs {floor}");
            // The accounting floor is stated in the RUN band's ε and
            // in nothing else — exact equality, so the row goes red the
            // moment a second tolerance is reachable here.
            let expect = SSI_FLOOR * band().zero() * d.floor_scale;
            assert!(
                (floor - expect).abs() <= f64::EPSILON * expect,
                "FLOOR-TIE: the accounting floor is {floor:e} m but the run band's \
                 own floor is {expect:e} m"
            );
            // The refusal says what it means and what to do.
            assert!(msg.contains("exhaustiveness inconclusive"), "{msg}");
            assert!(msg.contains("refuses"), "{msg}");
        }
        // At a fine enough ε the fit budget fires before any branch is
        // fitted, so the floor never gets its turn — and no fixture
        // fixes that, because a domain small enough to fit inside the
        // budget at ε = 1e-12 holds no branch to find, which is the
        // OTHER row's mode. So this row stands down there. What it must
        // not do is stand down on trust: the stand-down is only
        // legitimate for D9's own fit budget, exceeded, at an ε finer
        // than the compiled default. Assert all three, so a fit budget
        // that starts firing at the default ε — or a second budget
        // wearing this variant's name — reds here instead of printing
        // SKIPPED and passing.
        SsiError::FitSampleBudget { samples, budget } => {
            assert_eq!(
                budget, SSI_MAX_FIT_SAMPLES,
                "the stand-down is D9's fit budget or it is not a stand-down"
            );
            assert!(samples > budget, "{samples} of {budget} is not an overrun");
            assert!(
                eps() < DEFAULT_EPS,
                "the fit budget fired at ε = {:e}, which is not finer than the compiled \
                 default {DEFAULT_EPS:e} — the floor claim is REACHABLE here and this row \
                 owes it, not a stand-down",
                eps()
            );
            vacuity::stood_down(
                &format!("the floor-clamped refusal, eps = {:e}", eps()),
                &format!(
                    "the fit budget ({samples} of {budget} samples) refused before any \
                     branch was fitted, so THIS RUN ASSERTS NEITHER that the clamped \
                     {FLOOR_CLAMP_METRES} m floor refuses NOR what its refusal says — only \
                     that the refusal is D9's budget, overrun, at a finer-than-default ε"
                ),
            );
        }
        other => panic!("expected the exhaustiveness refusal, got {other}"),
    }
}

/// **The mode the row above cannot reach**: no branch is found at all,
/// so the accounting pass runs on an *empty* tube set.
///
/// The fixture is a near-miss pair — a cylinder whose wall clears the
/// unit sphere by 1 mm, so the locus is genuinely **empty** — run
/// TWICE, at two accounting floors.
///
/// 1. **At a healthy floor** the enclosures separate the two surfaces
///    and the run certifies the domain empty: `Ok`, with **zero
///    branches** and **zero accounted cells**, off a seed set that is
///    not itself empty. That is this row pinning its own mode. The
///    subdivision seeds, every seeded march fails to refine (there is
///    no root to refine onto), and no uniqueness tube is ever banked.
/// 2. **At a floor clamped two orders above the clearance** the same
///    seeding and the same marching happen — `floor_scale` feeds
///    `SsiDomain::floor` and nothing else, while the seed floor is a
///    fraction of the extent — so the accounting call is reached with
///    that same empty tube set, and now no enclosure at the floor can
///    separate the surfaces either. Nothing is proved about the domain,
///    so nothing may be claimed about it: the operation refuses.
///
/// Run 1 is what keeps run 2 honest. Post-fix a branch-FOUND run and a
/// branch-FREE run produce the identical `ExhaustivenessInconclusive`,
/// so without run 1 a fixture that drifted into finding a branch would
/// leave this row green as a second copy of the row above. Run 1 goes
/// red the moment that drift happens.
///
/// An `Ok` from run 2 is precisely the silent incompleteness this
/// module exists to prevent: zero branches reported *together with* an
/// exhaustiveness receipt.
///
/// **This row covers the ℝ³ lane only.** The chart lane reaches the
/// same empty tube set by its own road, off a fixture this pair of
/// surfaces cannot express, and has its own row:
/// [`an_unseeded_chart_run_refuses_typed_rather_than_receipting_an_unprovable_domain`].
#[test]
fn an_unseeded_run_refuses_typed_rather_than_receipting_an_unprovable_domain() {
    let s = sphere();
    // |d − r| = 1 + clearance > 1: the wall clears the unit sphere, so
    // the pair does not intersect at all. The clearance sits above the
    // escalation threshold at every battery ε, so the within-pair
    // tangency trilean passes and the run reaches the subdivision.
    let clearance = 1.0e-3;
    assert!(
        clearance > band().escalate(),
        "the fixture's clearance must be a definite sign at this ε"
    );
    let c = Surface::Cylinder {
        origin: Point3::new(1.5 + clearance, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 0.5,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    // `floor_m` is the accounting floor in METRES, at every ε — the
    // width this fixture's premise is about, through the door that
    // states it (`SsiDomain::floor_scale_for`, the inverse of
    // `SsiDomain::floor`).
    let domain = |floor_m: f64| SsiDomain {
        center: Point3::new(1.0, 0.0, 0.0),
        half_extent: 0.2,
        extent: 0.4,
        floor_scale: SsiDomain::floor_scale_for(floor_m, band()),
    };

    // ---- Run 1, the MODE PIN: a floor an order finer than the
    // clearance, where the enclosures do separate the surfaces.
    let out = ssi::cylinder_sphere_ssi(&c, &s, domain(1.0e-4), band())
        .expect("MODE: the near-miss domain certifies empty at a healthy floor");
    assert_eq!(
        out.branches.len(),
        0,
        "MODE: the pair does not intersect, so nothing may be certified — and a \
         banked tube would take the accounting call out of the mode this row exists \
         to cover: {:?}",
        out.exhaustiveness
    );
    assert!(
        out.seeds > 0,
        "MODE: the subdivision must actually SEED here — a run with no seeds would \
         reach the accounting call by a different road than the all-seeds-fail one"
    );
    assert_eq!(
        out.exhaustiveness.accounted, 0,
        "MODE: no tube exists, so no cell can have been accounted by one: {:?}",
        out.exhaustiveness
    );
    let e = out.exhaustiveness;
    assert_eq!(
        e.examined,
        e.excluded + e.accounted + e.refined,
        "MODE RECEIPT: the receipt must add up even on the empty domain: {e:?}"
    );

    // ---- Run 2, the CLAIM: the same seeding, the same empty tube set,
    // a floor no enclosure can beat.
    match ssi::cylinder_sphere_ssi(&c, &s, domain(0.1), band()) {
        Err(
            ref e @ SsiError::ExhaustivenessInconclusive {
                cell_width, floor, ..
            },
        ) => {
            // As the floor-clamped row above: the width/floor relation
            // is `sweep`'s own guard read back out and can only catch a
            // mis-populated refusal, so the refusal's TEXT is the part
            // with content.
            assert!(cell_width <= floor, "{cell_width} vs {floor}");
            let msg = format!("{e}");
            assert!(msg.contains("exhaustiveness inconclusive"), "{msg}");
            assert!(msg.contains("refuses"), "{msg}");
        }
        Err(other) => panic!("expected the exhaustiveness refusal, got {other}"),
        Ok(out) => panic!(
            "SILENT: an unprovable domain returned Ok with {} branches \
             and an exhaustiveness receipt {:?}",
            out.branches.len(),
            out.exhaustiveness
        ),
    }
}

// ---------------------------------------------------------------------
// The σ₂-sliver row
// ---------------------------------------------------------------------

#[test]
fn a_tangent_pair_refuses_toward_the_c7_regime_and_never_desingularizes() {
    // A cylinder of the sphere's own radius, coaxial: the two surfaces
    // are TANGENT along the equator. Transversality dies on the whole
    // candidate locus, which is C7's construction, not a locus to march.
    let s = sphere();
    let c = Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let err = ssi::cylinder_sphere_ssi(&c, &s, slab(), band()).expect_err("must refuse");
    let msg = format!("{err}");
    match err {
        SsiError::TransversalityBand { sin_theta, .. } => {
            assert!(sin_theta < 1.0e-6, "sin θ = {sin_theta}");
        }
        // The tube's own straddle is the same verdict reached one stage
        // later; either is a correct refusal toward C7.
        SsiError::TubeStraddles { .. } | SsiError::CertificateLimb { .. } => {}
        other => panic!("expected a tangency-shaped refusal, got {other}"),
    }
    // The refusal names the regime it points at.
    assert!(
        msg.contains("TangentIntersection") || msg.contains("tube"),
        "{msg}"
    );
}

#[test]
fn the_uniqueness_tube_margin_dies_on_a_tangent_pair() {
    // The limb-3 mechanism in isolation: on a tangent pair the graph
    // criterion's enclosure straddles zero at every box size, because
    // the gradients are parallel there. This is what makes the tube
    // refuse rather than certify a component it cannot separate.
    let s = sphere();
    let c = Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    // A carrier that IS the tangency circle (the equator).
    // Interpolated, not approximated, and densely: the row is about
    // limb 3, so limbs 1 and 2 must pass on their own merits. A cubic
    // interpolant through 400 exact circle points deviates by
    // A quarter arc, not the whole circle: the row is about limb 3, so
    // limbs 1 and 2 must pass on their own merits, and a short arc
    // reaches the same interpolation accuracy with a quarter of the
    // samples — which matters, because the interpolation solve is cubic
    // in the sample count.
    //
    // The count is DERIVED from the resolved ε, not fixed: a cubic
    // interpolant's error is `h⁴/384` on a unit circle, and what limb 2
    // actually reports is the control-hull bound over it — conservative
    // by ~20× on these fixtures — so the design point is `ε/200`, not
    // `ε/10`: `n = ((π/2)⁴ / (384·0.005·ε))^{1/4}`. At the finest ε that
    // lands past the fit-sample budget, and the row stands down for the
    // same reason the fixture rows do.
    let need = (std::f64::consts::FRAC_PI_2.powi(4) / (384.0 * 0.005 * eps()))
        .sqrt()
        .sqrt()
        .ceil();
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let n = (need as usize).max(64);
    if n > geom_brep::ssi::SSI_MAX_FIT_SAMPLES {
        vacuity::stood_down(
            &format!("equator interpolant, eps = {:e}", eps()),
            &format!(
                "limbs 1 and 2 would need {n} samples against a fit budget of {} — THIS \
                 RUN DOES NOT EXERCISE THE LIMB-3 TUBE REFUSAL ON A TANGENT PAIR. The \
                 refusal itself is still reached end-to-end by \
                 `a_tangent_pair_refuses_toward_the_c7_regime_and_never_desingularizes`; \
                 what is absent is the isolated limb-3 statement.",
                SSI_MAX_FIT_SAMPLES
            ),
        );
        return;
    }
    let pts: Vec<Point3<f64>> = (0..=n)
        .map(|i| {
            #[allow(clippy::cast_precision_loss)]
            let t = std::f64::consts::FRAC_PI_2 * (i as f64 / n as f64);
            Point3::new(t.cos(), t.sin(), 0.0)
        })
        .collect();
    let carrier =
        NurbsCurve3::<f64>::interpolate(&pts, 3).expect("a cubic interpolant of the equator");
    let err = ssi::certify_rung3(
        &carrier,
        None,
        &SsiOperand::Analytic(&c),
        &SsiOperand::Analytic(&s),
        TubeScale::split(1.0, 2.0),
        band(),
    )
    .expect_err("a tangency cannot certify a uniqueness tube");
    match err {
        SsiError::TubeStraddles { .. } => {}
        SsiError::CertificateLimb { limb, .. } => {
            assert_eq!(limb, SsiLimb::Tube, "the tube limb is the one that dies");
        }
        // A coarse fit of the equator may trip an earlier limb first;
        // that is still a refusal, but it is not the row's point, so
        // name it loudly if it happens.
        other => panic!("expected a limb-3 refusal, got {other}"),
    }
}

// ---------------------------------------------------------------------
// The corrupted-cache limb rows' helpers (the rows themselves are the
// `LIMBS` / `LIMB-1` / `LIMB-2` blocks of the planted-fixture row above,
// which already has the carrier they corrupt)
// ---------------------------------------------------------------------

/// The same carrier with control point `i` displaced by `d` metres in
/// `+z` — a planted corruption.
///
/// `+z` deliberately: the fixture's loops sit near the sphere's poles,
/// where the sphere normal is essentially `+z`, so this displacement is
/// *normal* to the locus and shows up in the residual at full size. An
/// `+x` displacement there is nearly tangent to the sphere and would
/// plant a corruption the certificate is right not to see.
fn displaced(curve: &NurbsCurve3<f64>, i: usize, d: f64) -> NurbsCurve3<f64> {
    let mut control = curve.control().to_vec();
    control[i] = control[i] + Vec3::new(0.0, 0.0, d);
    NurbsCurve3::new(curve.knots().clone(), control, curve.weights().to_vec())
        .expect("structure unchanged")
}

fn certify_against(carrier: &NurbsCurve3<f64>) -> Result<geom_brep::SsiCertificate<f64>, SsiError> {
    let (s, c) = (sphere(), threaded_cylinder());
    ssi::certify_rung3(
        carrier,
        None,
        &SsiOperand::Analytic(&c),
        &SsiOperand::Analytic(&s),
        TubeScale::split(0.08, 2.0),
        band(),
    )
}

// ---------------------------------------------------------------------
// Shape (iii): the NURBS wall, the ℝ⁴ trace, and OQ4
// ---------------------------------------------------------------------

/// **The one wall builder.** Every directly-authored NURBS wall in this
/// file is the same patch — cubic × linear, four control columns in `u`
/// and two rows in `v`, extruded 0.8 m along `z` — and differs only in
/// its section, so the section is the only thing a caller states.
///
/// Loft/sweep *definitions* are PR 10; these are authored control nets.
fn wall_from_cols(cols: [(f64, f64); 4]) -> NurbsSurface<f64> {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let mut control = Vec::with_capacity(8);
    for (x, y) in cols {
        control.push(Point3::new(x, y, 0.0));
        control.push(Point3::new(x, y, 0.8));
    }
    NurbsSurface::new(ku, kv, control, vec![1.0; 8]).unwrap()
}

/// The wall the ℝ⁴ rows march: curved in `x`–`y`, extruded in `z`. The
/// cutting plane meets it in a single open branch that runs wall-edge to
/// wall-edge.
///
/// Gently curved: a wall whose curvature *swings* violently makes ‖C⁗‖
/// far exceed κ³ and the step rule's fit budget (which assumes
/// slowly-varying curvature) then understates what the fit needs. The
/// acceptance shape wants a NURBS wall, not a pathological one.
fn nurbs_wall() -> NurbsSurface<f64> {
    wall_from_cols([(0.0, 0.0), (0.35, 0.18), (0.70, -0.12), (1.05, 0.04)])
}

/// The wall the substrate row CERTIFIES: same construction, section
/// curvature one-signed and slowly varying. `nurbs_wall`'s section
/// inflects (y swings 0.18 → −0.12), and at an inflection the step
/// rule's fit rung `h_fit ∝ (ε/κ³)^¼` unbinds (κ → 0), so the realized
/// between-samples deviation of the two independent fits genuinely
/// exceeds ε there (~3.8ε measured) — which PR 7b's tight bound now
/// SEES and refuses honestly (its own row below). The certified
/// acceptance wall is the one the step rule's own documented
/// assumption (slowly-varying curvature) actually covers.
fn certifiable_wall() -> NurbsSurface<f64> {
    wall_from_cols([(0.0, 0.0), (0.35, 0.14), (0.70, 0.24), (1.05, 0.30)])
}

/// A plane slicing the wall at mid height, tilted so the cut is not a
/// parameter line.
fn cutting_plane() -> Surface<f64> {
    let n = Vec3::new(0.0, 0.25, 1.0);
    let n = n / n.norm();
    // u_ref must be unit and ⊥ n.
    let u = Vec3::new(1.0, 0.0, 0.0);
    let u = (u - n * u.dot(n)) / (u - n * u.dot(n)).norm();
    Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.4),
        normal: n,
        u_ref: u,
    }
}

fn wall_domain() -> SsiDomain {
    SsiDomain {
        center: Point3::new(0.5, 0.0, 0.4),
        half_extent: 2.0,
        extent: 1.5,
        floor_scale: 1.0,
    }
}

/// The certified outcome for the substrate wall (the operation runs a
/// full march + exhaustiveness sweep). `None` when the fit budget
/// refuses at this ε — pinned by its own row, and the budget's demand
/// is march-side, which PR 7b deliberately did not touch.
///
/// **No memo.** This was a `OnceLock` "once per process"; nextest is
/// process-per-test, so it shared nothing and each reader paid the
/// march. The two readers that only wanted the finished outcome are one
/// row now; `shape_iii_bit_replay` calls this AND re-runs the operation
/// independently, which is the content of its claim, not duplication.
fn wall_outcome() -> Option<geom_brep::SsiOutcome> {
    let (p, w) = (cutting_plane(), certifiable_wall());
    match ssi::plane_nurbs_ssi(&p, &w, wall_domain(), band()) {
        Ok(o) => Some(o),
        Err(SsiError::FitSampleBudget { .. }) => None,
        Err(e) => panic!("shape (iii) substrate must certify: {e}"),
    }
}

/// The wall fixture's stand-down, said out loud: which row stood down,
/// and what it therefore did NOT cover. A bare `return` here reports
/// coverage the run does not have (`docs/M5-EXIT-WALK.md` row 15).
///
/// One argument's worth of local vocabulary over
/// [`test_utils::vacuity::stood_down`], not a second implementation of
/// it: every caller stands down for the same reason, so the reason is
/// written once here rather than at each `return`.
fn wall_stand_down(row: &str, absent: &str) {
    vacuity::stood_down(
        &format!("{row}, eps = {:e}", eps()),
        &format!(
            "the plane×NURBS march wants more samples than the SSI fit budget allows, \
             so the shape-(iii) wall never fitted at this ε — {absent}"
        ),
    );
}

/// **The shape-(iii) substrate, built ONCE, with both rows that read
/// it.** `a_corrupted_pcurve_is_caught_by_the_hull_limb_alone` is the
/// `CORRUPT-PCURVE` block below; it consumed the SAME `wall_outcome()`
/// march and, under nextest's process-per-test isolation, paid it a
/// second time. Every assertion NAMES its block — `SUBSTRATE`,
/// `TABLE`, `CORRUPT-PCURVE` — so a merged failure still says which
/// property broke. `shape_iii_bit_replay` deliberately stays out: it
/// runs the operation TWICE and the second run is its content.
#[test]
fn shape_iii_the_wall_cut_certifies_all_three_limbs_and_refuses_a_corrupted_pcurve() {
    // THE SUBSTRATE ROW (M5-PR7-SPEC §6, left unmet at PR 7 as its
    // deviation 1; exit-gating for shape (iii)): a directly-authored
    // NURBS wall cut by a plane — rung-3 marched + fitted + certified,
    // ALL THREE limbs, through the retired arm. Limb 2 is the tensor
    // composite bound; every pin scales from the resolved band.
    let Some(out) = wall_outcome() else {
        wall_stand_down(
            "shape (iii) substrate",
            "THIS RUN ASSERTS NEITHER the three-limb certification of the wall cut NOR \
             the limb-2-alone refusal of a corrupted pcurve NOR this row's C5 table \
             read (that last claim is ε-independent and is also stated \
             unconditionally by `the_c5_table_retires_the_arm_whose_proof_is_complete`)",
        );
        return;
    };
    assert_eq!(
        out.branches.len(),
        1,
        "SUBSTRATE: one open branch, edge to edge"
    );
    // The receipt identity, on the chart lane's own accounting pass:
    // every leaf excluded or accounted, every interior node split.
    let e = out.exhaustiveness;
    assert_eq!(
        e.examined,
        e.excluded + e.accounted + e.refined,
        "SUBSTRATE RECEIPT: the receipt must add up: {e:?}"
    );
    // Deliberately no `excluded > 0 && accounted > 0` here, though the
    // ℝ³ receipt block demands exactly that of its own lane. On THIS
    // fixture both are already entailed and neither could go red: a
    // cell holding a locus point can never be excluded, so with no tube
    // banked it reaches the floor and the sweep refuses before this
    // line — `Ok` with one branch IS the statement that
    // `pcurve_windows` banked a rectangle and `UvRect::contained_in`
    // consumed it. The chart lane's tube arm has no
    // degraded-but-still-`Ok` regime, so an assertion here would
    // document rather than test.
    let b = &out.branches[0];
    let cert = &b.certificate;
    assert_eq!(cert.samples, CERT_SAMPLES, "SUBSTRATE: the PR 6 schedule");
    // Limb 1 and limb 2 certified within the band's zero region — and
    // limb 2 is the number that certifies (C2.2), so it is the pin
    // that matters: the composite bound reaches the fit's own scale.
    assert!(
        cert.on_locus_max <= eps(),
        "SUBSTRATE: {:e}",
        cert.on_locus_max
    );
    assert!(cert.hull_sup <= eps(), "SUBSTRATE: {:e}", cert.hull_sup);
    // The measured bound-improvement, order-of-magnitude (spec §5):
    // where the first-order enclosure reported span-width scale
    // (~1e-2 m at any ε), the composite lands ~1e-8·(ε/1e-9) — an
    // order above the certified fixture's measured 4.3e-11 at the
    // default ε, band-relative so the row means the same at 1e-6.
    assert!(
        cert.hull_sup <= 10.0 * eps(),
        "SUBSTRATE: the composite bound lost its tightness: {:e}",
        cert.hull_sup
    );
    // Limb 3: a real tube with a real margin.
    assert!(
        cert.tube_radius > 0.0 && cert.tube_boxes > 0,
        "SUBSTRATE: limb 3 has a real tube"
    );
    assert!(
        cert.tube_transversality > 0.0,
        "SUBSTRATE: limb 3 has a real margin"
    );
    // The ℝ⁴ shape's products: both pcurves, on the carrier's own
    // parameter (the OQ4 identity the certified path now consumes).
    assert!(
        b.pcurve_a.is_some() && b.pcurve_b.is_some(),
        "SUBSTRATE: the ℝ⁴ arm fits both pcurves"
    );
    // And the table says RETIRED where a caller reads it (C12.1: the
    // arm retires WITH its proof, and the note records the date).
    assert_c5_plane_nurbs_retired();

    // ---- CORRUPT-PCURVE: exclusion-cannot-lie against the NEW bound,
    // and limb separation.
    //
    // A pcurve corruption leaves limb 1 clean — the foot-point check
    // re-projects from the corrupted warm start and converges to the
    // true foot, so on-locus distance and orthogonality stay in band —
    // while limb 2, which consumes the pcurve AS the parameter map,
    // must see |S(P(t)) − C(t)| at the corruption's full size. The
    // displacement scales from the resolved band (definitely positive
    // at any battery ε).
    let Curve3::Nurbs(ref carrier) = b.carrier else {
        panic!("a rung-3 carrier is a NURBS curve");
    };
    let pb = b
        .pcurve_b
        .as_ref()
        .expect("CORRUPT-PCURVE: the ℝ⁴ arm fits both pcurves");
    let mut control = pb.control().to_vec();
    let mid = control.len() / 2;
    let d = definitely_positive();
    control[mid] = geom_core::Point2::new(control[mid].x + d, control[mid].y);
    let bad = geom::NurbsCurve2::new(pb.knots().clone(), control, pb.weights().to_vec())
        .expect("structure unchanged");
    let (p, w) = (cutting_plane(), certifiable_wall());
    let err = ssi::certify_rung3(
        carrier,
        Some(&bad),
        &SsiOperand::Analytic(&p),
        &SsiOperand::Nurbs(&w),
        TubeScale::uniform(wall_domain().extent),
        band(),
    )
    .expect_err("CORRUPT-PCURVE: a corrupted parameter map cannot certify");
    match err {
        SsiError::CertificateLimb {
            limb: SsiLimb::HullSup,
            value,
        } => assert!(value > eps(), "CORRUPT-PCURVE: {value:e}"),
        other => panic!("CORRUPT-PCURVE: expected limb 2 alone, got {other}"),
    }
}

/// The routing table's plane×NURBS row, read the way a caller reads it.
/// Pure table lookup — no geometry, no ε — so it is stated whether or not
/// the wall fixture fitted at this ε.
fn assert_c5_plane_nurbs_retired() {
    let r = geom_brep::route(geom_brep::SurfaceKind::Plane, geom_brep::SurfaceKind::Nurbs);
    assert!(r.implemented, "TABLE: plane×NURBS is implemented");
    assert!(
        r.note.contains("certifies the whole chain"),
        "TABLE: the note must claim the FULL certificate, not a partial one: {}",
        r.note
    );
    assert!(
        r.note.contains("Bernstein composition"),
        "TABLE: {}",
        r.note
    );
}

#[test]
fn shape_iii_bit_replay() {
    // The same operation twice is the same certificate and the same
    // carrier to the BIT (D9) — the certified path includes the ring
    // composite, so this replays the whole PR 7b pipeline.
    let Some(a) = wall_outcome() else {
        wall_stand_down(
            "shape (iii) bit replay",
            "THIS RUN DOES NOT REPLAY the PR 7b pipeline — no certificate or control-point \
             bit comparison was made at this ε",
        );
        return;
    };
    let (p, w) = (cutting_plane(), certifiable_wall());
    let b = ssi::plane_nurbs_ssi(&p, &w, wall_domain(), band()).expect("replay");
    assert_eq!(a.branches.len(), b.branches.len());
    for (x, y) in a.branches.iter().zip(b.branches.iter()) {
        assert_eq!(
            x.certificate.hull_sup.to_bits(),
            y.certificate.hull_sup.to_bits()
        );
        assert_eq!(
            x.certificate.on_locus_max.to_bits(),
            y.certificate.on_locus_max.to_bits()
        );
        assert_eq!(
            x.certificate.tube_transversality.to_bits(),
            y.certificate.tube_transversality.to_bits()
        );
        let (Curve3::Nurbs(cx), Curve3::Nurbs(cy)) = (&x.carrier, &y.carrier) else {
            panic!("rung-3 carriers are NURBS curves");
        };
        for (u, v) in cx.control().iter().zip(cy.control().iter()) {
            assert_eq!(u.x.to_bits(), v.x.to_bits());
            assert_eq!(u.y.to_bits(), v.y.to_bits());
            assert_eq!(u.z.to_bits(), v.z.to_bits());
        }
    }
}

#[test]
fn an_inflected_wall_refuses_in_band_at_the_hull_limb_honestly() {
    // PR 7's original wall inflects (κ crosses zero along the section),
    // where the step rule's fit rung unbinds: the relative rungs price
    // the step there, so the realized fit pair carries a deviation at
    // the crossing, measured at ~3.8e-9 m at march-ε = 1e-9. That
    // deviation is PHASE-DEPENDENT AND NON-MONOTONE in march-ε, not a
    // constant cap (review measurement: 4× tighter march-ε → 4.36×
    // better, 16× → 16.92× better reaching 2.25e-10 m, 64× → only
    // 6.83× — where samples land relative to the crossing decides;
    // regression witness: review_m5_pr7b_ssi.rs `deviation2b`). The
    // shipped configuration's number stands, so the verdict honestly
    // FORKS on the resolved band: a band whose zero sits well above
    // the measured deviation certifies (ε = 1e-6), the default band
    // catches it in-band at the very predicate whose old bound (~1e-2
    // m, span-width-scaled at every ε) could never say anything this
    // precise, and the finest ε is preempted by the fit budget.
    const MEASURED_DEVIATION: f64 = 3.9e-9;
    let (p, w) = (cutting_plane(), nurbs_wall());
    match ssi::plane_nurbs_ssi(&p, &w, wall_domain(), band()) {
        Ok(out) => {
            assert!(
                band().zero() > 10.0 * MEASURED_DEVIATION,
                "certified at a band that should have seen the ~{MEASURED_DEVIATION:e} m deviation"
            );
            assert!(out.branches[0].certificate.hull_sup <= eps());
        }
        Err(SsiError::Escalated(ref d)) if d.predicate == Some("ssi_hull_sup_chart") => {
            assert!(
                band().zero() <= 10.0 * MEASURED_DEVIATION,
                "in-band refusal where the band is far above the measured deviation"
            );
        }
        Err(SsiError::CertificateLimb {
            limb: SsiLimb::HullSup,
            value,
        }) => {
            // Definite refusal is also honest — but only just above
            // the band; anything at the old bound's scale means the
            // cancellation was lost.
            assert!(value <= definitely_positive(), "{value:e}");
        }
        // The finest ε: the march demands more samples than the fit
        // budget affords, pinned by its own row. Same discipline as the
        // floor row's stand-down — it is D9's budget, overrun, at a
        // finer-than-default ε, or it is not this arm.
        Err(SsiError::FitSampleBudget { samples, budget }) => {
            assert_eq!(budget, SSI_MAX_FIT_SAMPLES);
            assert!(samples > budget, "{samples} of {budget} is not an overrun");
            assert!(
                eps() < DEFAULT_EPS,
                "the fit budget fired at ε = {:e}",
                eps()
            );
            vacuity::stood_down(
                &format!("the limb-2 in-band row, eps = {:e}", eps()),
                &format!(
                    "the fit budget ({samples} of {budget} samples) refused before limb 2 \
                     was reached, so this run asserts nothing about the composite bound"
                ),
            );
        }
        Err(other) => panic!("expected limb 2 in-band, got {other}"),
    }
}

#[test]
fn the_composite_bound_tracks_dense_scan_truth_on_the_pr7_fixture() {
    // THE MEASURED IMPROVEMENT ROW (spec §5): on the very fixture where
    // the first-order enclosure reported ~1e-2 m, the composite bound
    // is compared against a 1e5-sample dense scan of the true residual
    // |S(P(t)) − C(t)| on the SAME fitted triple: it must dominate it
    // (a bound) and track it (the cancellation) — measured 1.009× at
    // the default ε, asserted ≤ 2× for headroom. At the default ε the
    // spec's conservative order-of-magnitude ceiling (≤ 1e-8 m, seven
    // orders under the old report) is pinned literally.
    let (p, w) = (cutting_plane(), nurbs_wall());
    let (carrier, _pa, pb) = match ssi::trace_plane_nurbs_uncertified(
        &p,
        &w,
        (0.5, 0.5),
        wall_domain(),
        band().zero(),
        band(),
    ) {
        Ok(t) => t,
        Err(SsiError::FitSampleBudget { .. }) => {
            wall_stand_down(
                "composite bound vs dense scan",
                "THIS RUN COMPARES NOTHING against the 1e5-sample dense scan — the \
                     measured-improvement claim (spec §5) is unstated at this ε",
            );
            return;
        }
        Err(e) => panic!("the ℝ⁴ trace: {e}"),
    };
    use geom_core::spline::compose::{CurveRingData, tensor};
    let scoords = w.ring_coords();
    let sdata =
        tensor::SurfaceRingData::new(w.knots_u(), w.knots_v(), w.weights(), &scoords).unwrap();
    let ccoords = carrier.ring_coords();
    let cdata = CurveRingData::new(carrier.knots(), carrier.weights(), &ccoords).unwrap();
    let pcoords = pb.ring_coords();
    let pdata = CurveRingData::new(pb.knots(), pb.weights(), &pcoords).unwrap();
    let sup = tensor::surface_curve_residual(&sdata, &pdata, &cdata, &[])
        .unwrap()
        .sup_bound();
    let (t0, t1) = carrier.domain();
    let samples = 100_000u32;
    let mut max = 0.0f64;
    for i in 0..=samples {
        let t = t0 + (t1 - t0) * (f64::from(i) / f64::from(samples));
        let q = pb.eval(t);
        let r = (w.eval(q.x, q.y) - carrier.eval(t)).norm();
        if r > max {
            max = r;
        }
    }
    assert!(sup >= max, "not a bound: {sup:e} < dense max {max:e}");
    assert!(
        sup <= 2.0 * max,
        "the cancellation was lost: bound {sup:e} vs truth {max:e}"
    );
    if (eps() - 1e-9).abs() < f64::EPSILON {
        assert!(sup <= 1e-8, "the measured-improvement ceiling: {sup:e}");
    }
}

#[test]
fn oq4_the_two_pcurves_share_the_carriers_own_parameter() {
    // THE OQ4-discharge demonstration, and it does not depend on the
    // arm being retired: the ℝ⁴ trace yields the 3-D curve and both
    // pcurves as projections of ONE parameterized object, so
    // `S(P(t)) = C(t)` holds at the SAME t — the parameter-identity
    // contract PR 6 ratified for caches — by construction rather than
    // by coincidence. Checked on PR 6's own schedule (`CERT_SAMPLES`
    // over the carrier's interval), which is the statement
    // `PcurveCache::certify` makes.
    let (p, w) = (cutting_plane(), nurbs_wall());
    let (carrier, pa, pb) = match ssi::trace_plane_nurbs_uncertified(
        &p,
        &w,
        (0.5, 0.5),
        wall_domain(),
        band().zero(),
        band(),
    ) {
        Ok(t) => t,
        // Same budget stand-down as the ℝ³ fixture: at ε = 1e-12
        // the wall's cut wants more samples than the fit affords,
        // and the refusal is pinned by its own row.
        Err(SsiError::FitSampleBudget { .. }) => {
            wall_stand_down(
                "OQ4 parameter identity",
                "THIS RUN MAKES NO OQ4 DEMONSTRATION — neither pcurve was checked \
                     against the carrier's own parameter on the PR 6 schedule at this ε",
            );
            return;
        }
        Err(e) => panic!("the ℝ⁴ trace: {e}"),
    };
    let (t0, t1) = carrier.domain();
    // Same parameter interval, not merely the same shape.
    assert!((pa.domain().0 - t0).abs() < 1e-15 && (pa.domain().1 - t1).abs() < 1e-15);
    assert!((pb.domain().0 - t0).abs() < 1e-15 && (pb.domain().1 - t1).abs() < 1e-15);
    let Surface::Plane {
        origin,
        normal,
        u_ref,
    } = p
    else {
        unreachable!()
    };
    let v_ref = normal.cross(u_ref);
    for i in 0..CERT_SAMPLES {
        let t = t0 + (t1 - t0) * (f64::from(i) / f64::from(CERT_SAMPLES - 1));
        let c = carrier.eval(t);
        // The wall chart, through the NURBS map.
        let q = pb.eval(t);
        let on_wall = (w.eval(q.x, q.y) - c).norm();
        assert!(on_wall <= eps(), "wall pcurve, sample {i}: {on_wall:e}");
        // The plane chart, through the affine map.
        let a = pa.eval(t);
        let on_plane = ((origin + u_ref * a.x + v_ref * a.y) - c).norm();
        assert!(on_plane <= eps(), "plane pcurve, sample {i}: {on_plane:e}");
    }
    // And the identity is not vacuous: the pcurves genuinely move.
    let a0 = pb.eval(t0);
    let a1 = pb.eval(t1);
    assert!(
        (a1.x - a0.x).abs() > 0.5,
        "the wall pcurve must span its chart"
    );
}

// ---------------------------------------------------------------------
// The chart lane's empty-tube accounting
// ---------------------------------------------------------------------

/// A NURBS wall whose true surface **misses the cutting plane inside its
/// own control-net hull slack**.
///
/// [`wall_from_cols`] again — the file's one patch, differing only in
/// its section — cut by the plane `y = 0`, so the signed distance to the
/// plane is the section's own Bézier polynomial in `u` alone. Its
/// section values are `[0.158, −0.05, −0.05, 0.158]`: the control net
/// reaches **0.05 m past** the plane while the curve itself comes no
/// closer than `(0.158 − 3·0.05)/4 = 0.002 m`. The hull says the wall
/// may touch the plane; the surface does not.
///
/// That 25:1 gap between hull and truth is the whole fixture, and both
/// halves of it are asserted in the row below rather than trusted here.
/// The chart lane's exclusion rule reads a first-order box — midpoint
/// value ⊕ derivative hull × half-width, and the section derivative's
/// hull is `3·(0.158 + 0.05) = 0.624` — so a cell over the near miss
/// stays unexcludable until it narrows past `2·0.002/0.624 ≈ 6.4e−3` in
/// `u`, and is excluded once it does. The seed floor lands at 3.84e−2
/// and the healthy accounting floor at 8.19e−4, either side of it,
/// which is what puts the two runs on opposite sides of one enclosure.
/// That ordering is a fact about lengths, not about ε.
///
/// [`certifiable_wall`] cannot serve: it genuinely meets its plane, and
/// its hull is tight exactly where the near miss would have to sit.
fn hull_slack_wall() -> NurbsSurface<f64> {
    wall_from_cols([(0.0, 0.158), (0.35, -0.05), (0.70, -0.05), (1.05, 0.158)])
}

/// The plane [`hull_slack_wall`] grazes without touching: `y = 0`,
/// across the section the other walls' [`cutting_plane`] would cut.
fn grazing_plane() -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// **The chart lane's twin of
/// [`an_unseeded_run_refuses_typed_rather_than_receipting_an_unprovable_domain`]**:
/// no branch is found at all, so the accounting pass runs on an *empty*
/// tube set — the ℝ⁴ arm's own road into the mode, not the ℝ³ one's.
///
/// [`hull_slack_wall`] misses its plane by 0.002 m while its control
/// net crosses the plane by 0.05 m. The subdivision's enclosures are
/// first-order boxes over that net, so cells straddling the near miss
/// survive exclusion all the way down to the seed floor, and every seed
/// the subdivision then hands the marcher refuses to refine onto a
/// locus that does not exist. All seeds fail, no tube is banked, and
/// the accounting call is reached with `&[]`.
///
/// Run twice, at two accounting floors, in the ℝ³ row's shape:
///
/// 1. **At a healthy floor** the enclosures do separate wall from plane
///    and the run certifies the chart empty: `Ok`, zero branches, zero
///    accounted cells, off a seed set that is not empty. That `Ok` is
///    also the *certified* form of this fixture's premise — the sweep
///    proved every leaf solution-free by interval arithmetic, which the
///    sampled clearance above cannot do.
/// 2. **At a floor two orders coarser** the seeding and the marching
///    are unchanged — `floor_scale` feeds `SsiDomain::floor` and
///    nothing else, while the seed floor is a fraction of the extent —
///    so the accounting call is provably reached with that same empty
///    tube set, and now no enclosure at the floor separates the
///    surfaces either. Nothing is proved about the domain, so nothing
///    may be claimed about it: the operation refuses.
///
/// Run 1 is what keeps run 2 honest. A branch-found run and a
/// branch-free run reach the identical `ExhaustivenessInconclusive`, so
/// a refusal-only row whose fixture drifted into finding a branch would
/// stay green as a second spelling of the floor-clamped row rather than
/// covering this mode at all.
///
/// Neither run depends on ε: `floor_scale` comes from
/// [`SsiDomain::floor_scale_for`], the seed floor is a fraction of the
/// extent, and the clearance is checked against the resolved band.
#[test]
fn an_unseeded_chart_run_refuses_typed_rather_than_receipting_an_unprovable_domain() {
    let (p, w) = (grazing_plane(), hull_slack_wall());

    // ---- The fixture's two halves, asserted rather than described.
    // The hull must CROSS the plane, or nothing survives exclusion and
    // there are no seeds to fail; the surface must MISS it, or a branch
    // is found and the tube set is not empty. Both are this row's
    // premise, and either one drifting takes the run out of its mode.
    let dip = w
        .control()
        .iter()
        .map(|c| c.y)
        .fold(f64::INFINITY, f64::min);
    assert!(
        dip < 0.0,
        "FIXTURE: the control net must reach past the plane: {dip:e}"
    );
    let mut clearance = f64::INFINITY;
    for i in 0..=512 {
        for j in 0..=4 {
            let q = w.eval(f64::from(i) / 512.0, f64::from(j) / 4.0);
            clearance = clearance.min(q.y);
        }
    }
    // Against the top of the resolved band, the ℝ³ row's own threshold
    // for the same premise. What this guards is the MARCHER, which is
    // where the mode is ε-relative: a clearance the refinement could
    // close would find a branch and bank a tube. The enclosure ordering
    // that makes the two runs differ is a fact about lengths, not about
    // ε, and it is asserted behaviourally below — run 1's `Ok` says the
    // healthy floor is under the unexcludable width, run 1's seed count
    // says the seed floor is over it, run 2's refusal says the clamped
    // floor is over it.
    assert!(
        clearance > band().escalate(),
        "FIXTURE: the true surface must miss the plane by a definite margin at this \
         ε — sampled clearance {clearance:e} against a net dipping to {dip:e}"
    );

    // `floor_m` is the accounting floor in METRES, at every ε — the
    // width this fixture's premise is about, through the door that
    // states it (`SsiDomain::floor_scale_for`, the inverse of
    // `SsiDomain::floor`).
    let domain = |floor_m: f64| SsiDomain {
        center: Point3::new(0.5, 0.0, 0.4),
        half_extent: 2.0,
        extent: 3.0,
        floor_scale: SsiDomain::floor_scale_for(floor_m, band()),
    };

    // ---- Run 1, the MODE PIN: a floor fine enough that the first-order
    // enclosures resolve the near miss.
    let out = ssi::plane_nurbs_ssi(&p, &w, domain(1.0e-3), band())
        .expect("MODE: the near-miss wall certifies its chart empty at a healthy floor");
    assert_eq!(
        out.branches.len(),
        0,
        "MODE: the wall does not meet the plane, so nothing may be certified — and a \
         banked tube would take the accounting call out of the mode this row exists \
         to cover: {:?}",
        out.exhaustiveness
    );
    assert!(
        out.seeds > 0,
        "MODE: the subdivision must actually SEED here — the hull slack is what keeps \
         cells alive to the seed floor, and a run with no seeds would reach the \
         accounting call by a different road than the all-seeds-fail one"
    );
    assert_eq!(
        out.exhaustiveness.accounted, 0,
        "MODE: no tube exists, so no cell can have been accounted by one: {:?}",
        out.exhaustiveness
    );
    let e = out.exhaustiveness;
    assert_eq!(
        e.examined,
        e.excluded + e.accounted + e.refined,
        "MODE RECEIPT: the receipt must add up even on the empty domain: {e:?}"
    );

    // ---- Run 2, the CLAIM: the same seeding, the same empty tube set,
    // a floor no enclosure can beat.
    match ssi::plane_nurbs_ssi(&p, &w, domain(0.1), band()) {
        Err(
            ref e @ SsiError::ExhaustivenessInconclusive {
                cell_width, floor, ..
            },
        ) => {
            // The width/floor relation is `sweep`'s own guard read back
            // out, so it can only catch a mis-populated refusal — the
            // content is the refusal's TEXT, which is what a caller
            // acts on. Same two phrases the ℝ³ floor rows demand.
            assert!(cell_width <= floor, "{cell_width} vs {floor}");
            let msg = format!("{e}");
            assert!(msg.contains("exhaustiveness inconclusive"), "{msg}");
            assert!(msg.contains("refuses"), "{msg}");
        }
        Err(other) => panic!("expected the exhaustiveness refusal, got {other}"),
        Ok(out) => panic!(
            "SILENT: an unprovable chart domain returned Ok with {} branches \
             and an exhaustiveness receipt {:?}",
            out.branches.len(),
            out.exhaustiveness
        ),
    }
}

// ---------------------------------------------------------------------
// The sweeps' other never-silence doors
// ---------------------------------------------------------------------

// **The grid these rows sit in, and the cells that still have none.**
//
// `exhaust.rs` has three refusal sites and `ssi.rs` one guard that
// makes the sweep's floor meaningful. What multiplies them is not the
// site count: the budget check and both poison arms live in the ONE
// shared recursion, which runs under BOTH of `SweepDuty`'s values, on
// separate calls with separate floors. Duty is therefore an axis, and
// it is the axis the original bug lived on — `sweep` used to read its
// duty off `tubes.is_empty()`. The floor refusal is the one site that
// exists under one duty only (seeding banks the survivor where
// accounting refuses), and it is crossed with the tube set instead.
//
//     floor refusal   × lane × {empty, non-empty tubes}   4 cells
//     cell budget     × lane × {Seed, Account}            4 cells
//     poison arm      × lane × {Seed, Account}            4 cells
//     chart-speed guard (pre-sweep, chart only)           1 cell
//                                                       13 cells
//
// Three had rows before this block: the floor refusal in ℝ³ with both
// tube sets, and in the chart lane with an empty one. Five more do
// now — the floor refusal's fourth cell, and the cell budget and the
// poison arm in each lane under the **Seed** duty. Every row here
// names the duty it drives, because the duty is not visible in the
// call: it is which of the two `exhaust` entry points the operation
// reached first, and on both lanes seeding runs before accounting.
//
// **Five cells still have no row**, and this block does not close
// them:
//
//   - the cell budget and the poison arm under the **Account** duty,
//     in both lanes (four cells). Reachable in principle — the
//     accounting floor is routinely orders finer than the seeding one
//     (measured on the substrate wall: 8.8e-10 against 2.1e-2 in chart
//     units) — but the naive road in does not get there: with
//     `floor_scale` at 0, 1e-12 and 1e-9 both fixtures' accounting
//     passes still terminate `Ok`, because exclusion and the banked
//     tubes between them resolve every cell above any floor. What is
//     needed is a fixture leaving a region neither excluded nor
//     accounted at every width — a new fixture, not a new assertion.
//     Scheduled as §D row C18 in `docs/SMELL-SCAN-2026-08.md`, which
//     carries this negative result so the next taker does not repeat
//     it.
//   - the chart-speed guard itself. Both of its arms are unreachable
//     as written, and the hole beside them is a LIVE source defect,
//     open as issue #762 — see
//     `an_infinite_chart_speed_refuses_rather_than_receipting`.
//
// Every cell here is a claim in `exhaust.rs`'s module docs — "a typed
// refusal, never a silent truncation of the search" — that no fixture
// drove.

/// The substrate wall's **certified chart speed** — the quantity
/// `plane_nurbs_ssi` divides its two floors by, so that a floor stated
/// in meters means the same thing in both lanes.
///
/// It is a bound on `|∂S/∂u|` and `|∂S/∂v|` over the wall's whole knot
/// domain, taken from ring boxes over the control net. **No ε enters
/// it**, which is why one literal serves the whole battery — and why
/// pinning it is the chart lane's form of the ℝ³ rows' FLOOR-TIE. It is
/// a measurement of this fixture on this tree, not a tuned constant: if
/// `NurbsBoxes::deriv_box` is ever tightened, the certified floor
/// translation genuinely moves, the assertion below goes red with both
/// numbers in its message, and this literal is re-measured rather than
/// widened.
const WALL_CHART_SPEED: f64 = 1.130_884_609_498_248;

/// **FLOOR-TIE, the chart lane's form.** The reported floor is in
/// chart units; multiplied back by the one scale that translated it,
/// it must be the meters floor the caller asked for and nothing else.
/// A second tolerance entering the translation moves it.
///
/// Slack of a few ulps: the caller's floor is reconstituted as
/// `SSI_FLOOR · ε · (meters/ε)` and then divided by the speed, so two
/// roundings stand between the literal and the receipt.
fn assert_floor_is_the_meters_floor_over_the_chart_speed(floor: f64, meters: f64, which: &str) {
    let recovered = floor * WALL_CHART_SPEED;
    assert!(
        (recovered - meters).abs() <= 4.0 * f64::EPSILON * meters,
        "FLOOR-TIE ({which}): the sweep reported a floor of {floor:e} in chart units, \
         which is {recovered:e} m at the certified chart speed {WALL_CHART_SPEED:e} — \
         the row asked for {meters:e} m, so a second scale entered the translation"
    );
}

/// **The fourth cell of the {lane} × {tube set} cross product**: the
/// chart lane refusing at the floor with a tube set that is NOT empty —
/// the twin of [`the_floor_clamped_planted_fixture_refuses_typed`], one
/// lane over.
///
/// The substrate wall genuinely meets its plane, so a branch is found,
/// certified, and its pcurve's span windows are banked as tubes. The
/// accounting floor is then clamped above the cell width at which the
/// sweep resolves the domain, and the operation refuses rather than
/// hand back a receipt for a domain it did not finish proving.
///
/// **What is new here is not the shared floor arm** — that is one
/// generic `sweep`, exercised by all three rows above. It is
/// `SweepDuty::accounts` running against a NON-empty chart tube set:
/// `UvRect::contained_in`, over the rectangles `pcurve_windows` builds.
/// Every other chart row reaches that predicate only in the direction
/// where accepting MORE cells keeps the run green — an
/// over-permissive containment turns cells nobody proved anything
/// about into "accounted", which is the silent completeness claim
/// `pcurve_windows`' own doc warns about, and before this row no
/// assertion in the workspace went red when it did.
///
/// **The asymmetry, stated like for like.** Each lane's accounting
/// predicate reaches the sweep through one `SweepCell::contained_in`
/// forwarder, and those two forwarders are the comparable pair. Making
/// the chart one return `true` unconditionally reddens **one** row —
/// this one. Making the ℝ³ one return `true` unconditionally reddens
/// **three**: the two ℝ³ floor rows in this file and
/// `review_m5_pr7_adversarial`'s `the_tiny_pair_floor_variant_refuses_typed`.
/// One against three is the gap this row closes; a caller with no
/// guard at all is what it was.
///
/// Two runs, in the shape the two unseeded rows use:
///
/// 1. **The mode pin, at a healthy floor.** `Ok`, with a branch, and
///    with `accounted > 0` — so the tube set is non-empty AND the
///    accounting pass actually consumed it. Without this the row could
///    refuse for the empty-tube reason and read as a third spelling of
///    the rows above.
/// 2. **The claim, at a clamped floor.** Same seeding, same marching,
///    same tubes — `floor_scale` feeds `SsiDomain::floor` and nothing
///    else — and the operation refuses.
///
/// The two floors sit either side of a **dyadic** boundary, not a
/// tuned one: the sweep resolves this domain by cell width 0.125 in
/// chart units, so it is `Ok` for every floor below that and refuses
/// for every floor at or above the next admitted width, 0.25 — a
/// half-line in each direction rather than an interval. Stated in
/// meters through [`SsiDomain::floor_scale_for`], so both stay put at
/// every battery ε.
///
/// **No ε stand-down, deliberately.** Every other row on this wall
/// carries a `FitSampleBudget` arm because `nurbs_wall()` outruns the
/// fit budget at the fine end of the battery; `certifiable_wall()`
/// does not, and this row was measured with both arms replaced by a
/// panic at ε ∈ {1e-6, 1e-9, 1e-12} — the battery CI runs — and never
/// took either. A stand-down that cannot happen is an escape hatch on
/// the workspace's only guard for `UvRect::contained_in`, so there is
/// none: if the fit budget ever does preempt this row, it fails and
/// someone looks.
///
/// **The floor tie is exact, in both runs.** The ℝ³ twin asserts its
/// refusal floor equals `SSI_FLOOR · ε · floor_scale` outright. Here a
/// second scale sits in the expression — the certified chart speed the
/// floor is divided by — and the test cannot recompute it from the
/// public API, so it is pinned as `WALL_CHART_SPEED` and both floors
/// are carried back to meters through it. Run 1's receipt and run 2's
/// refusal each have to come out at the meters floor this row asked
/// for. A ratio between the two runs would NOT do: any scale shared by
/// both translations cancels out of it, which is the mutation that
/// showed the weaker form has no teeth.
#[test]
fn the_floor_clamped_chart_run_refuses_typed_with_a_banked_tube_set() {
    let (p, w) = (cutting_plane(), certifiable_wall());
    // `floor_m` is the accounting floor in METRES, at every ε — the
    // width this fixture's premise is about, through the door that
    // states it (`SsiDomain::floor_scale_for`, the inverse of
    // `SsiDomain::floor`).
    let domain = |floor_m: f64| SsiDomain {
        floor_scale: SsiDomain::floor_scale_for(floor_m, band()),
        ..wall_domain()
    };

    // ---- Run 1, the MODE PIN: a floor under the width at which the
    // sweep resolves this domain.
    // No `FitSampleBudget` arm: measured never taken at any battery ε
    // on this fixture (see the doc comment). A refusal here is a
    // failure, because this row is the only guard on the predicate.
    let out = ssi::plane_nurbs_ssi(&p, &w, domain(0.05), band())
        .expect("the substrate wall must certify at a healthy floor");
    assert_eq!(
        out.branches.len(),
        1,
        "MODE: the wall meets the plane in one branch, whose pcurve windows are the \
         tube set this row exists to account against: {:?}",
        out.exhaustiveness
    );
    assert!(
        out.exhaustiveness.accounted > 0,
        "MODE: the accounting pass must actually CONSUME the banked tubes — with no \
         accounted cell this row would be covering the empty-tube mode the rows above \
         already cover: {:?}",
        out.exhaustiveness
    );
    let e = out.exhaustiveness;
    assert_eq!(
        e.examined,
        e.excluded + e.accounted + e.refined,
        "MODE RECEIPT: {e:?}"
    );
    // FLOOR-TIE, run 1: the receipt's floor is the meters floor this
    // row asked for, divided by the certified chart speed and by
    // nothing else. See `WALL_CHART_SPEED`.
    assert_floor_is_the_meters_floor_over_the_chart_speed(e.floor, 0.05, "MODE RECEIPT");

    // ---- Run 2, the CLAIM: the same branch, the same tubes, a floor
    // above the width at which the sweep resolves the domain.
    match ssi::plane_nurbs_ssi(&p, &w, domain(0.5), band()) {
        Err(
            ref err @ SsiError::ExhaustivenessInconclusive {
                cell_width, floor, ..
            },
        ) => {
            // As the rows above: the width/floor relation is `sweep`'s
            // own guard read back out, so the content is the refusal's
            // TEXT, which is what a caller acts on.
            assert!(cell_width <= floor, "{cell_width} vs {floor}");
            assert_floor_is_the_meters_floor_over_the_chart_speed(floor, 0.5, "CLAIM");
            let msg = format!("{err}");
            assert!(msg.contains("exhaustiveness inconclusive"), "{msg}");
            assert!(msg.contains("refuses"), "{msg}");
        }
        Err(other) => panic!("expected the exhaustiveness refusal, got {other}"),
        Ok(out) => panic!(
            "SILENT: a domain the sweep could not finish proving returned Ok with {} \
             branches and an exhaustiveness receipt {:?}",
            out.branches.len(),
            out.exhaustiveness
        ),
    }
}

/// **The cell-budget claim, written once for both lanes.**
///
/// The two lanes reach `SSI_MAX_CELLS` through different exclusion
/// rules over differently shaped cells, but the shape of the row is
/// identical — a mode pin at a feature extent proportionate to the
/// slab, then the claim at an extent orders finer — so it is written
/// here rather than twice. `run` is the lane's whole operation as a
/// function of the caller's named feature extent; everything else the
/// two rows differ in is in `run`'s closure.
///
/// **This helper's contract is a claim about both callers.** It asserts
/// the mode is entered, that the refusal is the budget and not some
/// other typed door, that the budget it names is the module's own
/// constant, and that the message carries the recourse sentence — and
/// it panics with the receipt on `Ok`, which is the silence.
fn the_cell_budget_refuses_at_an_unaffordable_seed_floor(
    lane: &str,
    pin_extent: f64,
    claim_extent: f64,
    run: impl Fn(f64) -> Result<geom_brep::SsiOutcome, SsiError>,
) {
    // ---- The MODE PIN. Without it the row would stay green on a
    // fixture that had drifted into refusing for any reason at all.
    assert!(
        !matches!(run(pin_extent), Err(SsiError::CellBudget { .. })),
        "MODE ({lane}): at a feature extent proportionate to the slab this fixture \
         does not exhaust the budget — a row whose fixture refused here would be \
         pinning the fixture rather than the floor that drives it"
    );

    // ---- The CLAIM.
    match run(claim_extent) {
        Err(ref err @ SsiError::CellBudget { budget }) => {
            assert_eq!(
                budget, SSI_MAX_CELLS,
                "({lane}) the refusal must name the module's own budget"
            );
            let msg = format!("{err}");
            assert!(msg.contains("refused rather than truncated"), "{msg}");
        }
        Err(other) => panic!("({lane}) expected the cell-budget refusal, got {other}"),
        Ok(out) => panic!(
            "SILENT ({lane}): a subdivision that cannot afford its own floor returned \
             Ok with {} branches, {} seeds and a receipt {:?}",
            out.branches.len(),
            out.seeds,
            out.exhaustiveness
        ),
    }
}

/// **The cell budget under the Seed duty, ℝ³ lane** —
/// `SSI_MAX_CELLS`, whose whole docstring is *"exceeding it is a typed
/// refusal, never a silent truncation of the search"*, and which no
/// fixture reached.
///
/// The road in is the **seeding** floor, and that is the point twice
/// over. It is the caller's named feature `extent`, not ε, that sizes
/// it (`seed_floor = SSI_SEED_FLOOR · extent`) — and because seeding
/// runs before accounting on this lane (`seed_r3` precedes
/// `account_r3` in `cylinder_sphere_ssi`), an unaffordable seeding
/// floor is answered under the **Seed** duty and the accounting call is
/// never reached. Measured, by instrumenting both calls: the refusal
/// arrives from `seed_r3`. The Account-duty cell of this same door has
/// no row; the block header above says what stands in its way.
///
/// A caller who names a feature three orders finer than the slab it
/// asked to be searched asks the subdivision for a tree it cannot
/// afford, and the answer is the named budget rather than a truncated
/// seed set — which would be silence of the exact kind this module
/// exists to prevent, since a seed set truncated mid-enumeration loses
/// whole components and nothing downstream could tell.
///
/// **ε-free by construction**, unlike every other refusal row in this
/// file: the seeding floor is a fraction of the extent, the exclusion
/// rule is interval arithmetic over the operands, and neither reads
/// the band. Only the mode pin's outcome moves with ε.
#[test]
fn an_unaffordable_seed_floor_refuses_the_cell_budget_typed() {
    let (s, c) = (sphere(), threaded_cylinder());
    let domain = |extent: f64| SsiDomain {
        center: Point3::new(0.03, 0.0, 0.996),
        half_extent: 0.2,
        extent,
        floor_scale: 1.0,
    };
    // The slab is 0.4 m across and the claim's named feature is 1 mm,
    // so the seeding floor is 1.56e-5 m: the subdivision would have to
    // enumerate the locus at that width, and there are more such cells
    // than the budget allows.
    let claim_extent = 1.0e-3;
    assert!(
        2.0 * 0.2 / (SSI_SEED_FLOOR * claim_extent) > 1.0e4,
        "FIXTURE: the named feature must be orders finer than the slab, which is what \
         drives the seeding tree past what the budget can hold — the seeding floor is \
         {:e} m across a 0.4 m slab",
        SSI_SEED_FLOOR * claim_extent
    );
    the_cell_budget_refuses_at_an_unaffordable_seed_floor("ℝ³", 0.4, claim_extent, |extent| {
        ssi::cylinder_sphere_ssi(&c, &s, domain(extent), band())
    });
}

/// **The cell budget under the Seed duty, chart lane** — the same door
/// by the ℝ⁴ arm's own road, and not the same code above it: the
/// seeding floor is translated through the certified chart speed
/// (`domain.seed_floor() / speed`), and the cells being enumerated are
/// parameter rectangles enclosed by first-order boxes over the control
/// net rather than boxes in ℝ³.
///
/// Same duty and the same ε-freedom as
/// [`an_unaffordable_seed_floor_refuses_the_cell_budget_typed`] —
/// instrumented here too: `seed_chart_plane` refuses at
/// `seed_floor/speed = 1.38e-5` and `account_chart_plane` is never
/// called.
#[test]
fn an_unaffordable_chart_seed_floor_refuses_the_cell_budget_typed() {
    let (p, w) = (cutting_plane(), certifiable_wall());
    let domain = |extent: f64| SsiDomain {
        extent,
        ..wall_domain()
    };
    the_cell_budget_refuses_at_an_unaffordable_seed_floor("chart", 1.5, 1.0e-3, |extent| {
        ssi::plane_nurbs_ssi(&p, &w, domain(extent), band())
    });
}

/// **The ℝ³ sweep's poison arm**: an operand whose certified implicit
/// enclosure cannot be formed at all, so no cell can be excluded and
/// the domain cannot be proved exhausted by any amount of refinement.
///
/// The fixture is a **zero-radius sphere** — a point, which the
/// `Surface` enum admits (its radius is documented "positive by
/// convention", and the convention is unchecked). The sphere's
/// enclosure divides by `2r`, and the ring refuses a divisor that
/// touches zero, so the very first cell poisons.
///
/// **What the refusal is reached by is not what its text describes**,
/// and the row says so rather than hiding it: the arm's message names
/// a surface KIND with no ring-computable implicit form (cone, torus,
/// NURBS), and no such kind can get here — `cylinder_sphere_ssi`
/// refuses `WrongLane` for anything but a cylinder and a sphere. The
/// reachable cause is a degenerate INSTANCE of a supported kind. Both
/// are the same obligation — an enclosure that cannot be formed is a
/// typed refusal, never a sweep that quietly excludes nothing — and
/// pinning the text is what keeps this row from passing on some other
/// `UnsupportedCertificate`, of which the certificate stack has many.
///
/// **Which duty**: the **Seed** one. `cylinder_sphere_ssi` calls
/// `seed_r3` before `account_r3`, the poison arm lives in the closure
/// the shared recursion runs under either duty, and the first cell
/// poisons — so the refusal arrives during seeding and accounting is
/// never reached (instrumented). The Account-duty cell of this door
/// has no row.
///
/// Without the arm the sweep does not go wrong quietly in one step: a
/// poisoned enclosure excludes nothing, so every cell refines and
/// **another door answers** — measured, on this fixture, the cell
/// budget. The caller is then told the search was too big, when the
/// truth is that this operand has no certificate at all and no budget
/// would have helped. That is what this arm exists to prevent: not a
/// wrong answer, a wrong DIAGNOSIS. The same substitution happens for
/// real, today, one guard over — see
/// [`an_infinite_chart_speed_refuses_rather_than_receipting`], where
/// there is no arm and the budget does answer in its place.
#[test]
fn a_degenerate_r3_operand_refuses_the_enclosure_typed() {
    let point_sphere = Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: 0.0,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let c = threaded_cylinder();
    match ssi::cylinder_sphere_ssi(&c, &point_sphere, slab(), band()) {
        Err(ref err @ SsiError::UnsupportedCertificate { what }) => {
            // The sweep's own arm, not one of the certificate stack's:
            // this phrase appears at exactly one site in the kernel.
            assert!(
                what.contains("ring-computable implicit enclosure"),
                "the refusal must be the SWEEP's poison arm: {what}"
            );
            assert!(what.contains("cannot be proved exhausted"), "{what}");
            assert!(format!("{err}").starts_with("ssi: "), "{err}");
        }
        Err(other) => panic!("expected the enclosure refusal, got {other}"),
        Ok(out) => panic!(
            "SILENT: a domain no enclosure could be formed over returned Ok with {} \
             branches and a receipt {:?}",
            out.branches.len(),
            out.exhaustiveness
        ),
    }
}

/// **The chart sweep's poison arm**: the same obligation in the ℝ⁴
/// lane, where the enclosure is a first-order box over the control net
/// rather than an implicit residual.
///
/// The fixture is a wall of **finite** control points with **finite**
/// weights whose homogeneous products are not finite: the net sits at
/// `1e308` m and the weights run 1, 2, 3, 4, so `w·P` overflows and the
/// enclosure of `S` over the first cell poisons. The distinction is the
/// one `projection.rs`'s `mid` doc already carries in this workspace —
/// **finite inputs, non-finite arithmetic** — and the row asserts the
/// input half rather than describing it, because a fixture that had
/// drifted into holding an infinity would be testing the constructor
/// instead of the sweep.
///
/// The wall is absurd as geometry and that is not a weakness of the
/// row: the arm is a certificate obligation, and a certificate that
/// cannot be formed must say so at any magnitude a caller can build.
///
/// **Which duty**: the **Seed** one, as in the ℝ³ twin —
/// `seed_chart_plane` runs first and the first cell poisons
/// (instrumented). Worth recording: this net also drives the certified
/// chart speed to `+∞`, so it passes through the same guard hole
/// [`an_infinite_chart_speed_refuses_rather_than_receipting`] is about;
/// the poison arm simply answers first, which is the ordering that
/// makes this row a poison-arm row and not a second copy of that one.
#[test]
fn a_poisoning_control_net_refuses_the_enclosure_typed() {
    let h = 1.0e308;
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let mut control = Vec::with_capacity(8);
    let mut weights = Vec::with_capacity(8);
    for ((x, y), wt) in [(0.0, 0.0), (h, h), (h, h), (h, h)]
        .into_iter()
        .zip([1.0, 2.0, 3.0, 4.0])
    {
        control.push(Point3::new(x, y, 0.0));
        control.push(Point3::new(x, y, 0.8));
        weights.push(wt);
        weights.push(wt);
    }
    assert!(
        control
            .iter()
            .all(|p| p.x.is_finite() && p.y.is_finite() && p.z.is_finite())
            && weights.iter().all(|w: &f64| w.is_finite()),
        "FIXTURE: every input is finite — it is the ring arithmetic over them that is \
         not, and a fixture holding an infinity would be testing the constructor"
    );
    let w = NurbsSurface::new(ku, kv, control, weights).expect("a wall a caller can build");
    match ssi::plane_nurbs_ssi(&cutting_plane(), &w, wall_domain(), band()) {
        Err(SsiError::UnsupportedCertificate { what }) => {
            assert!(
                what.contains("control-net enclosure poisoned"),
                "the refusal must be the CHART sweep's poison arm: {what}"
            );
        }
        Err(other) => panic!("expected the enclosure refusal, got {other}"),
        Ok(out) => panic!(
            "SILENT: a chart domain no enclosure could be formed over returned Ok with \
             {} branches and a receipt {:?}",
            out.branches.len(),
            out.exhaustiveness
        ),
    }
}

/// **A live defect, made executable — not a door row.**
///
/// This row covers **none** of the thirteen cells the block header
/// enumerates. It is the regression guard attached to a source defect
/// that is open as **issue #762**, and it is here so the defect is
/// executable rather than only written down.
///
/// **The defect.** `plane_nurbs_ssi` translates BOTH of its floors —
/// seeding and accounting — from meters into the wall's parameter
/// domain by dividing by a certified chart speed, and guards that
/// translation with `speed.is_nan() || speed <= 0.0`. A speed of
/// **+∞** passes: `floor / ∞` is exactly `0`, in both floors, so
/// neither sweep can terminate at its floor; and the certified tube
/// padding is `tube_radius / speed`, so every banked tube would be
/// zero-width as well. Measured on this fixture: `speed = inf`,
/// `seed_floor/speed = 0e0`, `floor/speed = 0e0`.
///
/// **What answers instead, and why that is the bug.** Seeding runs
/// first, so the refusal comes from `seed_chart_plane` and the cell
/// budget — the same door and the same duty
/// [`an_unaffordable_chart_seed_floor_refuses_the_cell_budget_typed`]
/// already covers, reached by another road. The caller is told its
/// search was too big when the truth is that this wall has no usable
/// chart speed, which is exactly the substitution
/// [`a_degenerate_r3_operand_refuses_the_enclosure_typed`]'s arm
/// exists to prevent one lane over: not a wrong answer, a wrong
/// DIAGNOSIS. This row does not endorse that disposition. It pins the
/// one thing that is true today and must stay true — the operation
/// never hands back a receipt — and names which door answered, so that
/// when the guard is widened to refuse a non-finite speed the row
/// moves to the other arm instead of going red.
///
/// Latent beside it, and part of the same defect: `mag(du).max(mag(dv))`
/// **drops a lone `NaN`** (`f64::max` returns the non-NaN operand), so
/// the guard's `is_nan` arm cannot fire from a single poisoned
/// derivative box. Nothing reaches it today.
///
/// The fixture is a net at `1e200` m: the derivative boxes are finite
/// intervals, their magnitudes overflow when squared, and the speed
/// comes out `+∞`.
#[test]
fn an_infinite_chart_speed_refuses_rather_than_receipting() {
    let m = 1.0e200;
    let w = wall_from_cols([
        (0.0, 0.0),
        (0.35 * m, 0.14 * m),
        (0.70 * m, 0.24 * m),
        (1.05 * m, 0.30 * m),
    ]);
    match ssi::plane_nurbs_ssi(&cutting_plane(), &w, wall_domain(), band()) {
        Err(SsiError::CellBudget { budget }) => {
            assert_eq!(budget, SSI_MAX_CELLS);
            println!(
                "the infinite chart speed was answered by the CELL BUDGET, under the \
                 SEEDING duty: both floors translated to 0 and the sweep ran until the \
                 budget stopped it — the wrong diagnosis, and the reason this row is a \
                 defect record rather than a door row"
            );
        }
        Err(SsiError::UnsupportedCertificate { what }) if what.contains("chart speed") => {
            println!("the infinite chart speed was answered by the CHART-SPEED GUARD");
        }
        Err(other) => panic!("expected the budget refusal or the chart-speed refusal, got {other}"),
        Ok(out) => panic!(
            "SILENT: a floor that translated to zero returned Ok with {} branches and \
             a receipt {:?}",
            out.branches.len(),
            out.exhaustiveness
        ),
    }
}

// ---------------------------------------------------------------------
// The closure trio and the differential pin
// ---------------------------------------------------------------------

#[test]
fn a_clipped_domain_ends_the_branch_on_the_boundary() {
    // The third closure trilean: `ssi_branch_open_end`. The slab is
    // shrunk so the loop cannot stay inside it, and the branch ends
    // open on the named domain instead of closing.
    let (s, c) = (sphere(), threaded_cylinder());
    let mut d = slab();
    d.center = Point3::new(0.06, 0.0, 0.996);
    d.half_extent = 0.05;
    d.extent = 0.2;
    let out = ssi::cylinder_sphere_ssi(&c, &s, d, band());
    match out {
        Ok(o) => {
            assert!(!o.branches.is_empty());
            assert!(
                o.branches.iter().all(|b| b.end != BranchEnd::Closed),
                "a clipped slab cannot contain a closed loop"
            );
        }
        // A slab that clips this tightly may also fail to prove itself
        // exhausted; that is a typed refusal and equally acceptable —
        // what must never happen is a silently-closed loop.
        Err(SsiError::ExhaustivenessInconclusive { .. }) => {}
        Err(e) => panic!("unexpected: {e}"),
    }
}

#[test]
fn the_c5_table_retires_the_arm_whose_proof_is_complete() {
    use geom_brep::{Rung, SurfaceKind, route};
    // Retired: all three C2 limbs certify.
    for (a, b) in [
        (SurfaceKind::Cylinder, SurfaceKind::Sphere),
        (SurfaceKind::Sphere, SurfaceKind::Cylinder),
    ] {
        let r = route(a, b);
        assert_eq!(r.rung, Rung::General);
        assert!(r.implemented, "{a:?}×{b:?} should be retired by PR 7");
        assert!(r.note.contains("IMPLICIT PAIR"), "{}", r.note);
    }
    // The ℝ⁴ arm is live with all three limbs, and the note claims the
    // FULL certificate rather than a partial one — an arm goes
    // implemented only WITH its proof.
    for (a, b) in [
        (SurfaceKind::Plane, SurfaceKind::Nurbs),
        (SurfaceKind::Nurbs, SurfaceKind::Plane),
    ] {
        let r = route(a, b);
        assert_eq!(r.rung, Rung::General);
        assert!(r.implemented, "{a:?}×{b:?} is implemented");
        assert!(r.note.contains("PARAMETRIC PAIR"), "{}", r.note);
        assert!(r.note.contains("certifies the whole chain"), "{}", r.note);
        assert!(r.note.contains("Bernstein composition"), "{}", r.note);
    }
    // Everything else on the general rung still refuses, and now names
    // its trace shape rather than a bare "unimplemented".
    for (a, b) in [
        (SurfaceKind::Cylinder, SurfaceKind::Torus),
        (SurfaceKind::Nurbs, SurfaceKind::Nurbs),
        (SurfaceKind::Cone, SurfaceKind::Sphere),
    ] {
        let r = route(a, b);
        assert!(!r.implemented, "{a:?}×{b:?} must stay refused");
        assert!(
            r.note.contains("TRACE") || r.note.contains("trace shape"),
            "{a:?}×{b:?} must cite its trace shape: {}",
            r.note
        );
    }
}

// ---------------------------------------------------------------------
// Per-predicate trios, the closure-tangent arm, and the K-funnel
// ---------------------------------------------------------------------

/// The three verdicts of one named predicate at a chosen margin — the
/// `intersect_table` trio convention, applied to the SSI funnel.
fn trio(name: &'static str) -> (geom_core::Sign, geom_core::Sign, bool) {
    let b = band();
    let definite = geom_core::k_stats::decide(name, Margin::of(definitely_positive()), b)
        .expect("definitely positive");
    let degenerate = geom_core::k_stats::decide(name, Margin::of(0.0), b).expect("exactly zero");
    // Between `zero` and `escalate`: the F6 band, placed relative to
    // the band this run resolved (module note on `inside_the_band`).
    let escalated = geom_core::k_stats::decide(name, Margin::of(inside_the_band()), b).is_err();
    (definite, degenerate, escalated)
}

#[test]
fn ssi_transversality_trio() {
    // Positive ⇒ march; Zero ⇒ the sliver band, refused toward C7
    // (`TransversalityBand`); in-band ⇒ F6 escalation. A definite
    // NEGATIVE is unreachable — the margin is `sin θ · arm`, a
    // magnitude — and the marcher folds it into the same refusal as
    // Zero rather than leaving an unhandled arm.
    let (d, z, e) = trio("ssi_transversality");
    assert_eq!(d, geom_core::Sign::Positive);
    assert_eq!(z, geom_core::Sign::Zero);
    assert!(e, "the F6 band must escalate");
}

#[test]
fn ssi_closure_return_trio() {
    // Margin is `h − ‖x − x₀‖`: Positive ⇒ the next step would step
    // over the seed (returned); Negative ⇒ still away; Zero ⇒ in-band,
    // and the marcher treats it as a return (the conservative reading —
    // closing a loop that then has to certify is safer than marching
    // past a seed forever).
    let (d, z, e) = trio("ssi_closure_return");
    assert_eq!(d, geom_core::Sign::Positive);
    assert_eq!(z, geom_core::Sign::Zero);
    assert!(e);
    let neg = geom_core::k_stats::decide(
        "ssi_closure_return",
        Margin::of(-definitely_positive()),
        band(),
    )
    .unwrap();
    assert_eq!(neg, geom_core::Sign::Negative, "still away from the seed");
}

#[test]
fn ssi_tube_transversality_trio() {
    // Positive ⇒ the chain proves one arc; Zero ⇒ the enclosure
    // straddles, which is a genuine sliver (`TubeStraddles`, F6), not a
    // resolution to refine away; in-band ⇒ escalation.
    let (d, z, e) = trio("ssi_tube_transversality");
    assert_eq!(d, geom_core::Sign::Positive);
    assert_eq!(z, geom_core::Sign::Zero);
    assert!(e);
}

#[test]
fn the_closure_tangent_arm_that_refuses_a_cusp_or_crossing() {
    // `ssi_closure_tangent`'s margin is `cos φ · arc`, and its
    // Zero/Negative arm is the one no end-to-end fixture reaches: from
    // the kinds this PR retires (cylinder × sphere) every closed
    // component is a smooth loop, so the trace always returns running
    // the way it left. The arm is exercised at the predicate, and the
    // refusal it produces is pinned by its payload and message — which
    // is what a consumer actually sees.
    let b = band();
    let closed =
        geom_core::k_stats::decide("ssi_closure_tangent", Margin::of(definitely_positive()), b)
            .unwrap();
    assert_eq!(closed, geom_core::Sign::Positive, "a genuine closure");
    let perpendicular =
        geom_core::k_stats::decide("ssi_closure_tangent", Margin::of(0.0), b).unwrap();
    assert_eq!(perpendicular, geom_core::Sign::Zero, "a crossing");
    let reversed =
        geom_core::k_stats::decide("ssi_closure_tangent", Margin::of(-definitely_positive()), b)
            .unwrap();
    assert_eq!(reversed, geom_core::Sign::Negative, "a cusp / retrace");
    // Both non-Positive arms produce this refusal, and it says so.
    let err = SsiError::SelfCrossingLocus {
        cos_phi: -0.87,
        arc_length: 0.42,
    };
    let msg = format!("{err}");
    assert!(msg.contains("wrong way"), "{msg}");
    assert!(msg.contains("cusps or crosses itself"), "{msg}");
}

#[test]
fn the_ssi_predicates_reach_the_k_funnel() {
    // Telemetry from birth (T5): every SSI decision goes through the
    // one funnel, so the verdict log — which records at f64, unlike the
    // `Probe` margin sink — sees them by name. This is the row that
    // would catch a raw comparison sneaking into the marcher.
    use geom_core::k_stats::{start_verdict_log, take_verdict_log};
    let (s, c) = (sphere(), threaded_cylinder());
    let mut d = SsiDomain {
        center: Point3::new(0.03, 0.0, 0.996),
        half_extent: 0.2,
        extent: 0.4,
        floor_scale: 1.0,
    };
    d.floor_scale = 1.0;
    start_verdict_log();
    let outcome = ssi::cylinder_sphere_ssi(&c, &s, d, band());
    let v = take_verdict_log();
    // The marching predicates run before anything is fitted, so they
    // are recorded at every ε; the certificate's only run once a branch
    // was actually fitted, which the fit-sample budget can prevent at
    // the finest row.
    let mut expected = vec!["ssi_cs_tangency", "ssi_transversality", "ssi_step_progress"];
    if !matches!(outcome, Err(SsiError::FitSampleBudget { .. })) {
        expected.extend(["ssi_on_locus", "ssi_hull_sup", "ssi_tube_transversality"]);
    }
    for name in expected {
        assert!(
            v.iter().any(|x| x.predicate == name),
            "{name} never reached the funnel (recorded: {:?})",
            v.iter()
                .map(|x| x.predicate)
                .collect::<std::collections::BTreeSet<_>>()
        );
    }
}

#[test]
fn zz_temp_probe_poison_vs_speed() {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let build = |cols: [(f64, f64); 4], ws: [f64; 4]| {
        let mut control = Vec::with_capacity(8);
        let mut weights = Vec::with_capacity(8);
        for ((x, y), w) in cols.into_iter().zip(ws) {
            control.push(Point3::new(x, y, 0.0));
            control.push(Point3::new(x, y, 0.8));
            weights.push(w);
            weights.push(w);
        }
        NurbsSurface::new(ku.clone(), kv.clone(), control, weights)
    };
    let h = 1.0e308;
    let cases: Vec<(&str, [(f64, f64); 4], [f64; 4])> = vec![
        ("A orig 1e308 w=1,2,3,4", [(0.0, 0.0), (h, h), (h, h), (h, h)], [1.0, 2.0, 3.0, 4.0]),
        ("B 1e308 w=1", [(0.0, 0.0), (h, h), (h, h), (h, h)], [1.0, 1.0, 1.0, 1.0]),
        ("C modest pts, huge weights", [(0.0, 0.0), (0.35, 0.14), (0.70, 0.24), (1.05, 0.30)], [1.0e308, 9.0e307, 8.0e307, 7.0e307]),
        ("D modest pts, one huge weight", [(0.0, 0.0), (0.35, 0.14), (0.70, 0.24), (1.05, 0.30)], [1.0, 1.0e308, 1.0, 1.0]),
        ("E 1e200 net", [(0.0, 0.0), (0.35e200, 0.14e200), (0.70e200, 0.24e200), (1.05e200, 0.30e200)], [1.0, 1.0, 1.0, 1.0]),
        ("F 1e160 pts, 1e160 weights", [(0.0, 0.0), (1.0e160, 0.4e160), (2.0e160, 0.7e160), (3.0e160, 0.9e160)], [1.0e160, 1.0e160, 1.0e160, 1.0e160]),
    ];
    for (name, cols, ws) in cases {
        match build(cols, ws) {
            Err(e) => println!("[probe] {name}: constructor refused: {e:?}"),
            Ok(w) => match ssi::plane_nurbs_ssi(&cutting_plane(), &w, wall_domain(), band()) {
                Err(SsiError::UnsupportedCertificate { what }) => {
                    println!("[probe] {name}: UnsupportedCertificate: {what}");
                }
                Err(other) => println!("[probe] {name}: {other}"),
                Ok(o) => println!("[probe] {name}: Ok, {} branches", o.branches.len()),
            },
        }
    }
}
