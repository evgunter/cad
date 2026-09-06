//! **M10-9's positive pins** — the registered-identity door
//! (ERROR-DESIGN E12's provenance reserve), asserted as the measured
//! STATE it is: what the door discharges, what it costs the value
//! channel (nothing), and the four ceilings it does NOT move.
//!
//! The gating half; `m10_9_evidence_interval` is the evidence half
//! (`#[ignore]`d) and `m10_8_harness` the shared probe. Every number
//! here is ε-RELATIVE — the ceilings are the numeric channel's and
//! scale exactly with `CAD_TOLERANCE_EPS`, measured `7.787e2 · ε`
//! (plate, annulus), `3.865e2 · ε` (bracket) and `2.083e3 · ε` (pad) at
//! all three rows — and every ceiling row asserts BOTH of its ends so a
//! `false == false` cannot pass for a measurement.
//!
//! **What these rows re-cut.** M10-8 pinned the plate at `7.81e2 · ε`
//! bounded by `carrier_endpoint_start`, and R2 measured the pad at
//! `2.083e-6` bounded by a declared tangency. The plate's NUMBER is
//! unmoved — so `m10_8_pins_interval`'s ceiling rows still hold, and
//! they are left standing rather than duplicated — but the PREDICATE
//! that bounds it has changed, because the door discharges
//! `carrier_endpoint_start` outright. That is the positive statement
//! this file makes, and the reason the miss is a predicate rather than
//! prose (`work/m10/plate-ceiling-is-now-the-arc-span-identity`).
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::ProfileDoc;
use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::drive::{DriveConfig, drive};
use geom_core::{SymRules, Tol};

use crate::m10_8_harness::{certifies_whole, dials};

/// A named study, as a function of the SCALE of its real study, with the
/// ceiling last measured for it.
type StudyAtCeiling<'a> = (&'static str, f64, &'a dyn Fn(f64) -> ProfileDoc);

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

/// M10-8's tier exactly — the shipped set with the door shut, and the
/// differential every row here is taken against.
fn closed() -> SymRules {
    SymRules::shipped_without_the_door()
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
    let off = closed();
    assert!(!off.registered);
    assert_eq!(
        SymRules {
            registered: true,
            ..off
        },
        s,
        "`shipped_without_the_door` differs from `shipped` in the door and in nothing else"
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
        run(SymRules::shipped()),
        run(closed()),
        "straight geometry: the door changes nothing, down to the receipt line"
    );
}

/// **THE MECHANISM, on the plate's own ceiling predicate.** Just past
/// the widest whole-certifying box, a replay with the door SHUT refuses
/// on `carrier_endpoint_start` — the rim identity `‖q − c‖ = r` — and
/// the same replay with the door OPEN refuses on `carrier_endpoint_end`
/// instead, because the rim identity is discharged and the arc's SPAN
/// identity is a different theorem.
///
/// Three claims in one drive pair, and each is labelled: the door
/// discharges (`registered > 0`), it discharges only out of `numeric`
/// (`symbolic_zero` and `sign_gated` identical), and the predicate that
/// bounds the document has moved while the width has not.
#[test]
fn m10_9_the_rim_registrant_discharges_the_plates_endpoint_identity() {
    let tol = Tol::witness();
    let eps = tol.eps();
    // Just past the measured ceiling of `7.787e2 · ε` (both ends of
    // that bracket are asserted by the ceiling row below).
    let doc = crate::m10_7_plate::plate(5.0e-5 * 1.6e3 * eps, 1.0e-5 * 1.6e3 * eps, tol).0;
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let box_ = ParamBox::of(&analyzed);
    let (shut_refusal, shut) = replay_counts(&doc, &box_, closed(), tol);
    let (open_refusal, open) = replay_counts(&doc, &box_, SymRules::shipped(), tol);
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
    let shut_refusal = shut_refusal.expect("past its ceiling the plate refuses with the door shut");
    let open_refusal = open_refusal.expect("and with the door open");
    assert!(
        shut_refusal.contains("carrier_endpoint_start"),
        "M10-8's bound is the rim identity: {shut_refusal}"
    );
    assert!(
        open_refusal.contains("carrier_endpoint_end"),
        "with the rim identity registered, the bound is the arc's SPAN identity \
         (work/m10/plate-ceiling-is-now-the-arc-span-identity): {open_refusal}"
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
    let open = run(SymRules::shipped());
    let shut = run(closed());
    assert_eq!(
        strip(open.clone()),
        strip(shut.clone()),
        "every line but the receipt is byte-identical door on and off"
    );
    assert_ne!(open, shut, "and the receipt line itself DID move");
}

/// **The four ceilings, unmoved, both ends asserted, at whichever ε row
/// this process was built with.**
///
/// Each document is probed at half its measured ceiling (must certify)
/// and at twice it (must refuse), with the door open and shut — four
/// drives per document, and no bisection, because the claim is that the
/// ceiling is where it was and not what its tenth digit is.
///
/// **Sixteen drives, and they were profiled before they were written**
/// ([[test-suite-cost]]: cost concentrates savagely). The whole file
/// runs in 46 s in a debug build and in 10 s without the pad's four —
/// one whole-box leaf of the pad costs 1.2 s in release — so the pad is
/// 80% of this file's cost, and it is bought deliberately: it is the
/// document whose ceiling R2 measured and whose expectation this unit
/// was cut against, and a table that measured it and a gate that
/// skipped it would be the shape this repository calls a silent skip.
///
/// The measured brackets, identical at ε = 1e-6, 1e-9 and 1e-12 (so
/// every ceiling scales exactly with ε; none of them stopped, which is
/// the E12 claim this unit does NOT get to make):
///
/// | document | ceiling | first refusal beyond it, door shut → open | enclosure |
/// | --- | --- | --- | --- |
/// | two-hole plate | `[7.787e2, 7.817e2] · ε` | `carrier_endpoint_start` → `carrier_endpoint_end` | `[0, 1.246 · ε]` |
/// | R2 filleted bracket | `[3.865e2, 3.880e2] · ε` | `line_span` (both) | `[-1.095 · ε, 1.095 · ε]` |
/// | R1 annulus | `[7.787e2, 7.817e2] · ε` | `carrier_matches_mapped_source` (both) | `[0, 1.050 · ε]` |
/// | R2 rounded pad | `[2.083e3, 2.091e3] · ε` | `line_span` (both) | `[-1.666 · ε, 1.666 · ε]` |
#[test]
fn m10_9_the_four_ceilings_are_unmoved_by_the_door() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let docs: [StudyAtCeiling<'_>; 4] = [
        ("two_hole_plate", 7.787e2 * eps, &|s: f64| {
            crate::m10_7_plate::plate(5.0e-5 * s, 1.0e-5 * s, tol).0
        }),
        ("r2_filleted_bracket", 3.865e2 * eps, &|s: f64| {
            crate::m10_7_r2_probes_interval::bracket(s, tol).0
        }),
        ("r1_annulus", 7.787e2 * eps, &|s: f64| {
            crate::m10_8_r1_probes_interval::annulus(s, tol).0
        }),
        ("r2_rounded_pad", 2.083e3 * eps, &|s: f64| {
            crate::m10_8_r2_probes_interval::pad(s, tol).0
        }),
    ];
    for (name, ceiling, at) in docs {
        for (rules, label) in [(SymRules::shipped(), "open"), (closed(), "shut")] {
            assert!(
                certifies_whole(&at(0.5 * ceiling), rules, tol),
                "{name}, door {label}: half the measured ceiling ({:e}) must certify whole \
                 at eps={eps:e}",
                0.5 * ceiling
            );
            assert!(
                !certifies_whole(&at(2.0 * ceiling), rules, tol),
                "{name}, door {label}: twice the measured ceiling ({:e}) must refuse at \
                 eps={eps:e} — if this passes the ceiling MOVED and the table above is stale",
                2.0 * ceiling
            );
        }
    }
}
