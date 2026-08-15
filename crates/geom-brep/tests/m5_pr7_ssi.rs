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
//! Every ε stand-down in this file is LOUD: the run prints, by name, the
//! coverage it did not deliver. The retired `fixture_or_return!` /
//! `carrier_or_return!` macros returned green in silence, which is the
//! honesty gap `docs/M5-EXIT-WALK.md` row 15 recorded.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unreachable
)]

use geom_brep::CERT_SAMPLES;
use geom_brep::ssi::BranchEnd;
use geom_brep::ssi::{self, SsiDomain, SsiError, SsiLimb, SsiOperand};
use geom_core::spline::KnotVector;
use geom_core::{Band, Margin, Point3, Tolerance, Vec3};
use geom_curves::{Curve3, NurbsCurve3};
use geom_surfaces::{NurbsSurface, Surface};

fn eps() -> f64 {
    Tolerance::get().eps
}

fn band() -> Band {
    Band::linear().unwrap()
}

/// A margin the resolved band calls **definitely positive**, at any ε.
///
/// The battery runs this suite at ε ∈ {1e-6, 1e-9, 1e-12} and at the
/// interval scalar, so every probe value and every planted corruption
/// has to be placed *relative to the band the run resolved*, never at a
/// literal that happens to straddle the default one. `Band::linear()`
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
        eps: eps(),
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
            println!(
                "SKIPPED (planted fixture, eps = {:e}): the SSI door refused typed on its \
                 named fit-sample budget ({samples} samples vs {budget}). THIS RUN \
                 CONTRIBUTES NO SHAPE-(iv) COVERAGE — no found-and-certified row, no \
                 three-limb row, no limb-1/limb-2 separation, no accounting receipt, no \
                 dedup row, no idealized-vs-realized differential. Only the BUDGET \
                 assertions above executed.",
                eps()
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

#[test]
fn the_floor_clamped_variant_refuses_typed_even_though_branches_were_found() {
    // Same fixture, accounting floor clamped far above any certifiable
    // tube radius: cells along the locus can be neither excluded nor
    // accounted, so the operation refuses instead of reporting an
    // intersection it cannot prove complete.
    let (s, c) = (sphere(), threaded_cylinder());
    // One loop's neighbourhood: the row is about the floor, not about
    // finding both components, and a rung-3 op is not cheap.
    let mut d = SsiDomain {
        center: Point3::new(0.03, 0.0, 0.996),
        half_extent: 0.2,
        extent: 0.4,
        eps: eps(),
        floor_scale: 1.0,
    };
    d.floor_scale = 1.0e8; // floor = 0.1 m, far wider than any tube
    let err = ssi::cylinder_sphere_ssi(&c, &s, d, band()).expect_err("must refuse");
    let msg = format!("{err}");
    match err {
        SsiError::ExhaustivenessInconclusive {
            cell_width, floor, ..
        } => {
            assert!(cell_width <= floor, "{cell_width} vs {floor}");
            // The refusal says what it means and what to do.
            assert!(msg.contains("exhaustiveness inconclusive"), "{msg}");
            assert!(msg.contains("refuses"), "{msg}");
        }
        // At the finest ε the fit budget fires before any branch is
        // fitted, so the floor never gets its turn. Both are typed
        // refusals of the same operation and neither is silence, which
        // is the property this row exists to hold.
        SsiError::FitSampleBudget { .. } => {}
        other => panic!("expected the exhaustiveness refusal, got {other}"),
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
        println!(
            "SKIPPED (equator interpolant, eps = {:e}): limbs 1 and 2 would need {n} samples \
             against a fit budget of {} — THIS RUN DOES NOT EXERCISE THE LIMB-3 TUBE \
             REFUSAL ON A TANGENT PAIR. The refusal itself is still reached end-to-end by \
             `a_tangent_pair_refuses_toward_the_c7_regime_and_never_desingularizes`; what \
             is absent is the isolated limb-3 statement.",
            eps(),
            geom_brep::ssi::SSI_MAX_FIT_SAMPLES
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
        1.0,
        2.0,
        eps(),
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
        0.08,
        2.0,
        eps(),
        band(),
    )
}

// ---------------------------------------------------------------------
// Shape (iii): the NURBS wall, the ℝ⁴ trace, and OQ4
// ---------------------------------------------------------------------

/// A directly-authored NURBS wall (loft/sweep *definitions* are PR 10):
/// a bicubic × linear patch, curved in `x`–`y`, extruded in `z`. The
/// cutting plane meets it in a single open branch that runs wall-edge to
/// wall-edge.
fn nurbs_wall() -> NurbsSurface<f64> {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    // Four control columns in u, two rows in v (height).
    // Gently curved: a wall whose curvature *swings* violently makes
    // ‖C⁗‖ far exceed κ³ and the step rule's fit budget (which assumes
    // slowly-varying curvature) then understates what the fit needs.
    // The acceptance shape wants a NURBS wall, not a pathological one.
    let cols = [(0.0, 0.0), (0.35, 0.18), (0.70, -0.12), (1.05, 0.04)];
    let mut control = Vec::with_capacity(8);
    for (x, y) in cols {
        control.push(Point3::new(x, y, 0.0));
        control.push(Point3::new(x, y, 0.8));
    }
    NurbsSurface::new(ku, kv, control, vec![1.0; 8]).unwrap()
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
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let cols = [(0.0, 0.0), (0.35, 0.14), (0.70, 0.24), (1.05, 0.30)];
    let mut control = Vec::with_capacity(8);
    for (x, y) in cols {
        control.push(Point3::new(x, y, 0.0));
        control.push(Point3::new(x, y, 0.8));
    }
    NurbsSurface::new(ku, kv, control, vec![1.0; 8]).unwrap()
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
        eps: eps(),
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
fn wall_stand_down(row: &str, absent: &str) {
    println!(
        "SKIPPED ({row}, eps = {:e}): the plane×NURBS march wants more samples than the \
         SSI fit budget allows, so the shape-(iii) wall never fitted at this ε — {absent}",
        eps()
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
    let bad = geom_curves::NurbsCurve2::new(pb.knots().clone(), control, pb.weights().to_vec())
        .expect("structure unchanged");
    let (p, w) = (cutting_plane(), certifiable_wall());
    let err = ssi::certify_rung3(
        carrier,
        Some(&bad),
        &SsiOperand::Analytic(&p),
        &SsiOperand::Nurbs(&w),
        wall_domain().extent,
        wall_domain().extent,
        eps(),
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

/// The C5 table's plane×NURBS row, read the way a caller reads it. Pure
/// table lookup — no geometry, no ε — so it is stated whether or not
/// the wall fixture fitted at this ε.
fn assert_c5_plane_nurbs_retired() {
    let r = geom_brep::route(geom_brep::SurfaceKind::Plane, geom_brep::SurfaceKind::Nurbs);
    assert!(r.implemented, "TABLE: plane×NURBS is retired");
    assert!(r.note.contains("RETIRED 2026-07-31"), "TABLE: {}", r.note);
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
        // budget affords, pinned by its own row.
        Err(SsiError::FitSampleBudget { .. }) => {}
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
    let (carrier, _pa, pb) =
        match ssi::trace_plane_nurbs_uncertified(&p, &w, (0.5, 0.5), wall_domain(), band()) {
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
    let (carrier, pa, pb) =
        match ssi::trace_plane_nurbs_uncertified(&p, &w, (0.5, 0.5), wall_domain(), band()) {
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
    // Retired by PR 7b: the ℝ⁴ arm, all three limbs, and the note
    // records the retirement and its date (C12.1: WITH its proof).
    for (a, b) in [
        (SurfaceKind::Plane, SurfaceKind::Nurbs),
        (SurfaceKind::Nurbs, SurfaceKind::Plane),
    ] {
        let r = route(a, b);
        assert_eq!(r.rung, Rung::General);
        assert!(r.implemented, "{a:?}×{b:?} retired by PR 7b");
        assert!(r.note.contains("PARAMETRIC PAIR"), "{}", r.note);
        assert!(r.note.contains("RETIRED 2026-07-31"), "{}", r.note);
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
        eps: eps(),
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
