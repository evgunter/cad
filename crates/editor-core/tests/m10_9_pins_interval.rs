//! **M10-9's positive pins** — the registered-identity door
//! (ERROR-DESIGN E12's provenance reserve), asserted as the measured
//! STATE it is: what the door discharges, what it costs the value
//! channel (nothing), and the five ceilings it does NOT move.
//!
//! The gating half; `m10_9_evidence_interval` is the evidence half
//! (`#[ignore]`d) and `m10_8_harness` the shared probe. Every number
//! here is ε-RELATIVE — the ceilings are the numeric channel's and
//! scale with `CAD_TOLERANCE_EPS` to within one bisection step at all
//! three rows — and every ceiling row asserts BOTH ends of the measured
//! bracket, so a `false == false` cannot pass for a measurement.
//!
//! **What these rows re-cut.** M10-8 pinned the plate at `7.81e2 · ε`
//! bounded by `carrier_endpoint_start`, and R2 measured the pad at
//! `2.083e-6` bounded by a declared tangency. Neither NUMBER moved — so
//! `m10_8_pins_interval`'s ceiling rows still hold, and they are left
//! standing rather than duplicated — and neither did the BOUND. The arc
//! carrier's builder registers both of its same-object identities (the
//! rim `‖q − c‖ = r` and the span `carrier.eval(param_end) = q_to`),
//! both discharge outright, and what bounds every measured document is
//! `carrier_matches_mapped_source` — the carrier against the scaffold
//! pushforward, an identity between two independently built objects
//! — door open and door SHUT alike
//! (`work/m10/plate-ceiling-is-now-the-scaffold-pushforward`).
//!
//! **A bound is a SET, read at ceiling + δ.** The first cut of this
//! file read one drive's first refusal at twice the ceiling and
//! reported a bound that had moved; at twice the ceiling several
//! predicates are over the band and evaluation order picks the name. The
//! rows below assert the bracket at both ends and the over-band SET at
//! the refusing end, which is the pair that cannot be satisfied by an
//! artefact.
//!
//! **Which tier these rows pin.** M10-9's — the shipped set with the
//! form-level algebra OFF (`SymRules::without_the_algebra`), which is
//! that tier bit for bit and the differential the algebra is measured
//! against; the shipped set now carries rules A/B per node and rule D
//! on top of the door, and its state is `m10_10_pins_interval`'s.
//! Holding at every row here is what "the algebra off reproduces
//! M10-9" means in assertable form.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::ProfileDoc;
use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::drive::{DriveConfig, drive};
use geom_core::{SymRules, Tol};

use crate::m10_8_harness::{certifies_whole, dials};

/// A named study, as a function of the SCALE of its real study, with ONE
/// measured scale — a multiple of ε.
type StudyAtCeiling<'a> = (&'static str, f64, &'a dyn Fn(f64) -> ProfileDoc);

/// The same, with the measured ceiling as the BRACKET it is: a multiple
/// of ε that certifies whole and one that refuses, both asserted.
type Bracketed<'a> = (&'static str, f64, f64, &'a dyn Fn(f64) -> ProfileDoc);

/// One whole-box replay at `Sym<Interval>`: the session's counts and the
/// first node that refused.
///
/// **Deliberately NOT `m10_8_arc_family_interval::replay`**, and the
/// difference is cost, not taste: that one installs the SHAPE REPORT,
/// which renders the plain normal form of every residual the numeric
/// channel could not decide — hundreds of them per replay, each a
/// page-long rational function. That is what an evidence probe wants
/// and what a gate cannot afford; these rows read the refusal's own
/// message instead, which already names the predicate.
fn replay_counts(
    doc: &ProfileDoc,
    box_: &ParamBox,
    rules: SymRules,
    tol: Tol,
) -> (Option<String>, geom_core::SymCounts) {
    use std::sync::Arc;

    use editor_core::{CancelToken, EvalOptions, NodeResult, ProfileLift, evaluate};

    let opts = EvalOptions {
        param_box: Some(Arc::new(box_.clone())),
        profile_lift: ProfileLift::Guided,
        ..EvalOptions::default()
    };
    let budget = geom_core::SymBudget {
        max_terms: editor_core::drive::DEFAULT_SYM_MAX_TERMS,
        max_degree: editor_core::drive::DEFAULT_SYM_MAX_DEGREE,
    };
    geom_core::sym::with_session_rules(budget, rules, || {
        let ev: editor_core::Evaluation<geom_core::Sym<geom_core::Interval>> =
            evaluate(doc, None, &CancelToken::new(), &opts, tol);
        ev.order.iter().find_map(|id| match ev.result(*id) {
            Some(NodeResult::Failed(e)) => Some(format!("node {} — {}", id.0, e.kind)),
            _ => None,
        })
    })
}

/// M10-9's tier exactly — the shipped set with the algebra off.
fn opened() -> SymRules {
    SymRules::without_the_algebra()
}

/// M10-8's tier exactly — M10-9's with the door shut, and the
/// differential every row here is taken against.
fn closed() -> SymRules {
    SymRules {
        registered: false,
        ..opened()
    }
}

/// **The door SHIPS, and it is the only difference from M10-8's set.**
#[test]
fn m10_9_the_shipped_set_carries_the_door() {
    let s = SymRules::shipped();
    assert_eq!(SymRules::default(), s, "one default");
    assert!(s.registered, "the registered-identity door ships");
    assert!(
        s.early,
        "and it rides the early walk, so the plain form — asked first — never sees the registry"
    );
    let off = SymRules::shipped_without_the_door();
    assert!(!off.registered);
    assert_eq!(
        SymRules {
            registered: true,
            ..off
        },
        s,
        "`shipped_without_the_door` differs from `shipped` in the door and in nothing else"
    );
    assert_eq!(
        SymRules {
            registered: true,
            ..closed()
        },
        opened(),
        "and M10-9's tier is M10-8's plus the door"
    );
}

/// **The door is INERT on straight geometry**: the M10-3 slab has no
/// arc, so no constructor registers anything, the registry stays empty
/// and no third walk is ever built — the drive serializes byte for byte
/// what M10-8's tier serialized.
#[test]
fn m10_9_the_door_is_inert_on_straight_geometry() {
    use crate::m10_3_driver_interval::slab;
    let tol = Tol::witness();
    let doc = slab(1.0, 0.25);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let run = |rules: SymRules| {
        drive(
            &doc,
            &analyzed,
            &DriveConfig {
                max_leaves: 8,
                symbolic: dials(rules),
                ..DriveConfig::default()
            },
            tol,
        )
        .expect("the slab builds")
        .serialize()
    };
    assert_eq!(
        run(opened()),
        run(closed()),
        "straight geometry: the door changes nothing, down to the receipt line"
    );
}

/// **THE MECHANISM, in the receipt.** A replay of the plate past its
/// ceiling with the door SHUT registers nothing; the same replay with
/// the door OPEN discharges through the registry — BOTH endpoint
/// pinnings (the rim identity `‖q − c‖ = r` and the span identity
/// `carrier.eval(param_end) = q_to`, the arc carrier's two same-object
/// identities) — and only out of `numeric`: `symbolic_zero` and
/// `sign_gated` are identical door open and shut.
///
/// The name the drive stops on is deliberately NOT read: past the
/// ceiling several predicates are over the band at once and the name
/// is evaluation order. The BOUND is the over-band set at ceiling + δ
/// (`m10_9_the_bound_at_ceiling_plus_delta_is_the_scaffold_pushforward`).
#[test]
fn m10_9_the_rim_registrant_discharges_the_plates_endpoint_identity() {
    let tol = Tol::witness();
    let eps = tol.eps();
    // Past the measured ceiling (`[7.811e2, 7.814e2] · ε`, both ends
    // asserted by the ceiling row below).
    let doc = crate::m10_7_plate::plate(5.0e-5 * 1.6e3 * eps, 1.0e-5 * 1.6e3 * eps, tol).0;
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let box_ = ParamBox::of(&analyzed);
    let (shut_refusal, shut) = replay_counts(&doc, &box_, closed(), tol);
    let (open_refusal, open) = replay_counts(&doc, &box_, opened(), tol);
    println!(
        "   door shut {shut:?} -> {shut_refusal:?}\n   door open {open:?} -> {open_refusal:?}"
    );
    assert_eq!(shut.registered, 0, "M10-8's tier registers nothing");
    assert!(
        open.registered > 0,
        "the rim registrant discharges on the plate: {open:?}"
    );
    assert_eq!(
        (open.symbolic_zero, open.sign_gated),
        (shut.symbolic_zero, shut.sign_gated),
        "the door moves decisions out of `numeric` and out of nothing else: {open:?} vs {shut:?}"
    );
    // NOT "and `numeric` shrank": a REFUSING replay stops at its first
    // refusal, so the two runs do not decide the same population — the
    // door lets this one get further, and the decisions past the old
    // refusal are decisions the shut run never reached. The
    // out-of-`numeric`-and-nothing-else claim is pinned where every
    // site decides: at the scalar
    // (`geom_core::sym`'s `a_registered_identity_decides_zero_and_is_counted_apart`)
    // and at each document's nominal (the census's table).
    assert!(
        open.decisions() >= shut.decisions(),
        "the door can only carry a replay further, never less far: {open:?} vs {shut:?}"
    );
    assert!(
        shut_refusal.is_some() && open_refusal.is_some(),
        "past its ceiling the plate refuses with the door shut and open alike"
    );
}

/// **The value channel is untouched, at document scale**: over a box
/// the plate certifies whole, the door-on and door-off drives serialize
/// identically except for the receipt line that reports the door. Same
/// leaves, same verdict vectors, same masses, same bits.
#[test]
fn m10_9_the_value_channel_is_untouched_on_a_certifying_box() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let doc = crate::m10_7_plate::plate(5.0e-5 * 1.0e2 * eps, 1.0e-5 * 1.0e2 * eps, tol).0;
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let run = |rules: SymRules| {
        drive(
            &doc,
            &analyzed,
            &DriveConfig {
                max_leaves: 4,
                symbolic: dials(rules),
                ..DriveConfig::default()
            },
            tol,
        )
        .expect("the plate builds")
        .serialize()
    };
    let strip = |s: String| {
        s.lines()
            .filter(|l| !l.starts_with("decisions "))
            .map(str::to_owned)
            .collect::<Vec<_>>()
            .join("\n")
    };
    let open = run(opened());
    let shut = run(closed());
    assert_eq!(
        strip(open.clone()),
        strip(shut.clone()),
        "every line but the receipt is byte-identical door on and off"
    );
    assert_ne!(open, shut, "and the receipt line itself DID move");
}

/// **THE CEILINGS, UNMOVED — pinned to the BISECTION BRACKET at both
/// ends — AND THE BOUND, NAMED as the over-band SET at ceiling + δ.**
///
/// Two claims, and the second is the one M10-9's fix pass had to
/// re-measure. A ceiling is a bracket, so both of its ends are asserted
/// and the bracket is the measured one (a few parts in ten thousand
/// wide), not a `0.5×`/`2×` gesture that a 4× move could pass. And a
/// BOUND is the SET of predicates over the band at the refusing end —
/// never the single name a drive reports, because a drive stops at its
/// first refusal and evaluation ORDER (validation before certification)
/// picks which of several simultaneously-over-band predicates that is.
///
/// The measured state, door open and door shut, at ε = 1e-6, 1e-9 and
/// 1e-12 (each ceiling ∝ ε to within one bisection step; the bracket
/// below is the union over the three rows):
///
/// | document | certifies at | refuses at | over-band set at ceiling + δ |
/// | --- | --- | --- | --- |
/// | two-hole plate | `7.811e2 · ε` | `7.814e2 · ε` | `{carrier_matches_mapped_source}`, `[0, 1.0001 · ε]` |
/// | R1 annulus | `7.805e2 · ε` | `7.810e2 · ε` | `{carrier_matches_mapped_source}`, `[0, 1.0001 · ε]` |
/// | R2 link | `4.930e2 · ε` | `4.934e2 · ε` | `{carrier_matches_mapped_source}`, `[0, 1.0002 · ε]` |
/// | R2 filleted bracket | `3.870e2 · ε` | `3.873e2 · ε` | `{carrier_matches_mapped_source}`, `[0, 1.0001 · ε]` |
/// | R2 rounded pad | `2.083e3 · ε` | `2.084e3 · ε` | `{carrier_matches_mapped_source}`, `[0, 1.0001 · ε]` |
///
/// **Twenty drives, and they were profiled before they were written**
/// ([[test-suite-cost]]: cost concentrates savagely). The pad is ~80%
/// of this file, and it is bought deliberately — it is the document
/// whose ceiling R2 measured and whose expectation this unit was cut
/// against, and a table that measured it and a gate that skipped it
/// would be the shape this repository calls a silent skip. The
/// over-band SET is asserted for the three cheap documents only, in
/// the row below, and NOT for the bracket and the pad: naming the set
/// needs the shape report, which renders every blocked residual, and
/// on the pad that is minutes. Their sets are measured in
/// `m10_9_evidence_interval` and tabled above; what gates here is
/// their bracket.
#[test]
fn m10_9_the_ceilings_are_unmoved_and_both_ends_are_the_measured_bracket() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let docs: [Bracketed<'_>; 5] = [
        ("two_hole_plate", 7.811e2, 7.814e2, &|s: f64| {
            crate::m10_7_plate::plate(5.0e-5 * s, 1.0e-5 * s, tol).0
        }),
        ("r1_annulus", 7.805e2, 7.810e2, &|s: f64| {
            crate::m10_8_r1_probes_interval::annulus(s, tol).0
        }),
        ("r2_link", 4.930e2, 4.934e2, &|s: f64| {
            crate::m10_9_r2_probes_interval::link(s, tol).0
        }),
        ("r2_filleted_bracket", 3.870e2, 3.873e2, &|s: f64| {
            crate::m10_7_r2_probes_interval::bracket(s, tol).0
        }),
        ("r2_rounded_pad", 2.083e3, 2.084e3, &|s: f64| {
            crate::m10_8_r2_probes_interval::pad(s, tol).0
        }),
    ];
    for (name, lo, hi, at) in docs {
        for (rules, label) in [(opened(), "open"), (closed(), "shut")] {
            assert!(
                certifies_whole(&at(lo * eps), rules, tol),
                "{name}, door {label}: {lo:e}·ε is inside the measured bracket and must \
                 certify whole at eps={eps:e} — if this fails the ceiling FELL"
            );
            assert!(
                !certifies_whole(&at(hi * eps), rules, tol),
                "{name}, door {label}: {hi:e}·ε is past the measured bracket and must \
                 refuse at eps={eps:e} — if this passes the ceiling ROSE and the table \
                 above is stale"
            );
        }
    }
}

/// **AND THE BOUND IS ONE PREDICATE, door open or shut** — the
/// over-band set at ceiling + δ on the three documents a gate can
/// afford to name it for.
///
/// This is the unit's finding in its assertable form. The door
/// discharges both of the arc carrier's endpoint identities outright,
/// and the predicate that BOUNDS each document does not move, because
/// it was never one of them: `carrier_matches_mapped_source`, the
/// carrier against the `MappedCurve` pushforward at the certifier's own
/// samples — an identity between two INDEPENDENTLY BUILT objects, which
/// is the line E12's reserve draws
/// (`work/m10/plate-ceiling-is-now-the-scaffold-pushforward`).
///
/// Asserted as a SET, so a second predicate joining it is a failure
/// rather than a silent change of subject.
#[test]
fn m10_9_the_bound_at_ceiling_plus_delta_is_the_scaffold_pushforward() {
    use geom_core::sym::report::ShapeOutcome;

    use crate::m10_8_arc_family_interval::replay;

    let tol = Tol::witness();
    let eps = tol.eps();
    // The refusing end of each measured bracket (the row above asserts
    // that these refuse; this one says WHAT is over the band there).
    let docs: [StudyAtCeiling<'_>; 3] = [
        ("two_hole_plate", 7.814e2, &|s: f64| {
            crate::m10_7_plate::plate(5.0e-5 * s, 1.0e-5 * s, tol).0
        }),
        ("r1_annulus", 7.810e2, &|s: f64| {
            crate::m10_8_r1_probes_interval::annulus(s, tol).0
        }),
        ("r2_link", 4.934e2, &|s: f64| {
            crate::m10_9_r2_probes_interval::link(s, tol).0
        }),
    ];
    for (name, hi, at) in docs {
        for (rules, label) in [(opened(), "open"), (closed(), "shut")] {
            let doc = at(hi * eps);
            let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
            let (shapes, _, _) = replay(&doc, &ParamBox::of(&analyzed), rules, tol);
            let mut over: Vec<&'static str> = shapes
                .iter()
                .filter(|sh| {
                    matches!(
                        sh.outcome,
                        ShapeOutcome::Indeterminate | ShapeOutcome::Invalid
                    )
                })
                .map(|sh| sh.predicate)
                .collect();
            over.sort_unstable();
            over.dedup();
            assert_eq!(
                over,
                ["carrier_matches_mapped_source"],
                "{name}, door {label}: at ceiling + δ exactly one predicate is over the \
                 band, and it is the scaffold pushforward — not an identity the door \
                 could reach, and not a real margin \
                 (work/m10/plate-ceiling-is-now-the-scaffold-pushforward)"
            );
        }
    }
}
