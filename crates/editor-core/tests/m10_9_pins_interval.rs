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
//! (`work/sym/plate-ceiling-is-now-the-scaffold-pushforward`).
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

/// **One measured document, and everything measured about it** — the
/// single home for the five studies this file and
/// `m10_9_evidence_interval` drive.
///
/// Every field is a MEASUREMENT, not a target: `certifies_at` and
/// `refuses_at` are the two ends of the ceiling's bracket in multiples
/// of ε, and `registered` is how many decisions the registered-identity
/// door discharges on the document at `certifies_at` (ε-independent —
/// the same at `1e-6`, `1e-9` and `1e-12`, and both reviews re-measured
/// it). Three rows read this array; a re-measured ceiling or a moved
/// count is edited once (R1 S1: it used to be typed out three times,
/// magic constants and builders both).
pub(crate) struct Study {
    pub(crate) name: &'static str,
    /// A multiple of ε that certifies whole.
    pub(crate) certifies_at: f64,
    /// A multiple of ε that refuses.
    pub(crate) refuses_at: f64,
    /// `SymCounts::registered` at `certifies_at`, shipped set.
    pub(crate) registered: u64,
    /// `SymCounts::symbolic_zero` at `certifies_at`, shipped set — the
    /// THEOREM count beside the axiom count, pinned since SYM-8.
    ///
    /// Why both: a rule that moves a decision from `symbolic_zero` to
    /// `registered` leaves `registered` looking like a gain and moves
    /// the claim from a theorem the tier proved to an axiom a
    /// constructor stated. SYM-8's rule F did exactly that to four of
    /// the pad's decisions, and with only `registered` pinned the
    /// four lived in a comment (R1's `#651` shape: a claim resting on
    /// a measurement with neither guard nor register). Pinned, a later
    /// change that costs four more theorems reds here.
    pub(crate) symbolic_zero: u64,
    pub(crate) at: Box<dyn Fn(f64) -> ProfileDoc>,
}

/// The five, in the order every row here reports them.
pub(crate) fn measured_studies(tol: Tol) -> [Study; 5] {
    [
        Study {
            name: "two_hole_plate",
            certifies_at: 7.811e2,
            refuses_at: 7.814e2,
            registered: 140,
            // DECIDE-3: eight more THEOREMS (803 -> 811) out of
            // `numeric` (470 -> 462) — comparisons of two rational
            // constants A0 now decides exactly. `registered` unmoved.
            symbolic_zero: 811,
            at: Box::new(move |s: f64| crate::m10_7_plate::plate(5.0e-5 * s, 1.0e-5 * s, tol).0),
        },
        Study {
            name: "r1_annulus",
            certifies_at: 7.805e2,
            refuses_at: 7.810e2,
            registered: 140,
            symbolic_zero: 328,
            at: Box::new(move |s: f64| crate::m10_8_r1_probes_interval::annulus(s, tol).0),
        },
        Study {
            name: "r2_link",
            certifies_at: 4.930e2,
            refuses_at: 4.934e2,
            // DECIDE-3: rule G re-keys the link's roots on their value
            // class, so six more of the rim identity's samples meet
            // the registrant's forms (90 -> 96) and twenty-six more
            // residuals are theorems outright (515 -> 541). Both move
            // UP; nothing was traded. DECIDE-5: the arc's span spelled
            // from the decided turn meets the pushforward's atom, and
            // both move up again, 96 -> 108 and 541 -> 545.
            registered: 108,
            symbolic_zero: 545,
            at: Box::new(move |s: f64| crate::m10_9_r2_probes_interval::link(s, tol).0),
        },
        Study {
            name: "r2_filleted_bracket",
            certifies_at: 3.870e2,
            refuses_at: 3.873e2,
            registered: 144,
            // DECIDE-3: twenty-one more theorems (1083 -> 1104) and
            // seven decisions the read answers, all out of `numeric`
            // (794 -> 766); `registered` is unmoved. Six of the
            // twenty-one are A0's constant fold
            // (`work/decide/a0-leaves-max-and-min-of-constants-opaque`),
            // the rest rule G's.
            symbolic_zero: 1104,
            at: Box::new(move |s: f64| crate::m10_7_r2_probes_interval::bracket(s, tol).0),
        },
        Study {
            name: "r2_rounded_pad",
            certifies_at: 2.083e3,
            refuses_at: 2.084e3,
            // 86 until SYM-5's rule E (`common_factor`). The pad is
            // the one of the five whose `registered` the rule moves,
            // and it moves it UP: with the dial off this replay reads
            // `symbolic_zero: 695, registered: 86, numeric: 1172`, with
            // it on `858 / 104 / 991` — 181 decisions leave `numeric`,
            // 163 of them as theorems and 18 through the door, and
            // `frozen` is 2750 either way. That is the SECOND cause
            // the assertion below names (a changed fold rule that
            // un-freezes a residual the door then recognises) and not
            // the first: `registrations_refused` and
            // `registrations_contradicted` are 0 at both dials, the
            // registrants' own `debug_assert!` never fires, and no
            // ceiling on this document moves. The other four are
            // byte-identical at both dials except for `symbolic_zero`
            // rising against `numeric` (link 485 → 515, bracket
            // 1075 → 1083, plate and annulus unmoved).
            //
            // 104 until SYM-8's rule F (`manifest_sign`), which moves
            // this same document and only this one again, and again
            // the second cause: `without_rule_f` reads
            // `symbolic_zero: 858, registered: 104, numeric: 991`
            // here and the shipped set `854 / 128 / 971` — the same
            // 1953 decisions, 24 of them moving INTO the door, 20 out
            // of `numeric` and FOUR out of `symbolic_zero`. Those four
            // are the unit's disclosed finding: opening an `abs` atom
            // the early walk was cancelling over can cost that walk a
            // theorem the registry then re-takes
            // (`work/sym/coefficient-ring-width-is-not-monotone-in-reach`,
            // the class). No decision is lost, `frozen` is 2750 at
            // both dials, no per-predicate split at any document's
            // nominal moves, and no ceiling on any of the eight
            // measured documents moves by a digit.
            // DECIDE-3: 128 -> 150 and 854 -> 890, with six decisions
            // the read answers; `numeric` 971 -> 907. Every column
            // that moved moved UP.
            registered: 150,
            symbolic_zero: 890,
            at: Box::new(move |s: f64| crate::m10_8_r2_probes_interval::pad(s, tol).0),
        },
    ]
}

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

/// **The door SHIPS, and `shipped_without_the_door` differs from `shipped`
/// in the door and nothing else.**
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
/// The table above is PROSE; `measured_studies` is the checked copy of
/// the same numbers, and it is what the rows below read.
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
    for study in measured_studies(tol) {
        let (name, lo, hi, at) = (study.name, study.certifies_at, study.refuses_at, &study.at);
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

/// **NO REGISTRANT LIES ON A REAL DOCUMENT** — the loud channel the
/// door was missing, at fixture scale, and the three assertions that
/// make it one.
///
/// The five measured documents are replayed at `Sym<Interval>` with the
/// shipped set, at the scale each certifies whole at
/// (`measured_studies`). **The width the row reads at is the ANALYZED
/// BOX** — `analyzed_box(doc, AnalysisPolicy::default())`, the same box
/// the driver replays over — and that width is what the exact witness
/// tests against, so the claim below is about the door's answers over
/// that box and not over a point.
///
/// **What each assertion catches, because no one of them catches
/// everything** (R2 MAJOR-1 / R1 MINOR-4 measured exactly this):
///
/// - **`registered`, pinned PER DOCUMENT.** A registration the exact
///   witness ADMITS but should not is invisible to a refusal count: at
///   `Sym<Interval>` two certified enclosures that still MEET are
///   `Witnessed`, which is the door's contract and not a defect it can
///   see. R2 planted `‖q − c‖ ≡ r · (1 + 2ε)` in `register_rim_identity`
///   and the enclosures still met — but the registry stops discharging,
///   so `registered` collapses (plate 140 → 16, annulus 140 → 16, link
///   90 → 16, bracket 144 → 20, pad 86 → 24 — R2's measurement on the
///   tier as it stood, whose pad base was 86; SYM-5's rule E has since
///   moved that base and not the mechanism) and `numeric` rises
///   (470 → 594 on the plate). **These counts are what a small lie
///   moves**, so they are pinned, and they are ε-independent: the same
///   five numbers at `1e-6`, `1e-9` and `1e-12`.
/// - **The registrants' own `debug_assert!`** (`swept::handle_registration`).
///   A GEOMETRIC lie separates the enclosures, the exact witness answers
///   `Contradicted`, and the registrant aborts — first, and in every CI
///   profile, because this workspace ships `debug-assertions = true` in
///   release too. So a 0.1 % lie reds this row through a panic and never
///   reaches the count below.
/// - **`registrations_refused == 0`.** What is left once those two have
///   had their turn: `Cyclic`, and any FUTURE registrant that binds the
///   refusal without asserting. It is the backstop, not the loud
///   channel, and saying otherwise was the first cut's mistake.
///
/// The ε rows are the suite's: one process per ε, so this row runs at
/// `1e-6`, `1e-9` and `1e-12` and the claim is all three.
#[test]
fn m10_9_no_registrant_lies_on_any_measured_document() {
    let tol = Tol::witness();
    let eps = tol.eps();
    for study in measured_studies(tol) {
        let name = study.name;
        let doc = (study.at)(study.certifies_at * eps);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let (refusal, counts) =
            replay_counts(&doc, &ParamBox::of(&analyzed), SymRules::shipped(), tol);
        println!("   {name} at eps={eps:e}: {counts:?} -> {refusal:?}");
        assert_eq!(
            counts.registered, study.registered,
            "{name} at eps={eps:e}: the door discharges a DIFFERENT number of decisions \
             than the measurement. The cause this pin exists for is a registrant that \
             started stating something slightly false — it stops discharging without \
             ever being refused — but the count moves for other reasons too: a changed \
             fold rule, a new normal-form shortcut that discharges a residual earlier, \
             or a re-cut document. Decide which before re-baselining: {counts:?}"
        );
        assert_eq!(
            counts.symbolic_zero, study.symbolic_zero,
            "{name} at eps={eps:e}: the TIER's own theorems moved. Beside `registered` \
             because the two trade: a rule that costs the early walk a theorem the \
             registry then re-takes leaves `registered` up and this count down, which \
             is a claim WEAKENED from a theorem to an axiom and not a gain. SYM-8's \
             rule F did that to four of the pad's: {counts:?}"
        );
        assert_eq!(
            counts.registrations_refused, 0,
            "{name} at eps={eps:e}: a registration was refused on a real document — at \
             this lane that is `Cyclic`, or an exact-witness refusal from a registrant \
             that binds it instead of asserting: {counts:?}"
        );
    }
}

/// **THE PAD'S FOUR, AT BOTH DIALS** (adopted from SYM-8's review R2,
/// `sym8_r2_the_pads_four_re_taken`). The row above pins the shipped
/// side; this one is the DIFFERENTIAL that says what rule F
/// (`SymRules::manifest_sign`) did to it. At the scale the pad
/// certifies whole at, over its analyzed box, rule F off → on:
/// `symbolic_zero` 858 → 854, `registered` 104 → 128, `numeric`
/// 991 → 971, `frozen` 2750 either way — the same 1953 decisions, 24
/// of them moving into the door, twenty out of `numeric` and FOUR out
/// of `symbolic_zero`.
///
/// No decision is lost and the document certifies whole at both dials,
/// which is asserted here; what moved is the STRENGTH of four claims.
/// The spec's Phase-1.3 stop clause reads on that, and shipping rule F
/// on anyway is a ratified spec deviation, not a disclosure
/// (`work/decide/SYM-8.md`).
///
/// `#[ignore]`d: it is two whole-box replays of the heaviest of the
/// five documents (~2 min locally), on top of the one the gating row
/// above already pays, and the shipped side of it is now pinned there
/// by `Study::symbolic_zero`. Re-take it by running this row.
#[test]
#[ignore = "evidence-only: two whole-box pad replays; the shipped side is pinned by the row above"]
fn m10_9_the_pads_four_at_both_dials() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let study = measured_studies(tol)
        .into_iter()
        .find(|s| s.name == "r2_rounded_pad")
        .expect("the pad is one of the five");
    let doc = (study.at)(study.certifies_at * eps);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let box_ = ParamBox::of(&analyzed);
    let mut got = Vec::new();
    for (label, rules) in [
        ("F-off", SymRules::without_rule_f()),
        ("F-on ", SymRules::shipped()),
    ] {
        let t0 = std::time::Instant::now();
        let (refusal, c) = replay_counts(&doc, &box_, rules, tol);
        println!(
            "   pad {label} eps={eps:e} ({:.1}s): {c:?} -> {refusal:?}",
            t0.elapsed().as_secs_f64()
        );
        assert!(
            refusal.is_none(),
            "{label}: the pad certifies whole at this scale: {refusal:?}"
        );
        got.push((c.symbolic_zero, c.registered, c.numeric, c.frozen));
    }
    assert_eq!(got[0], (858, 104, 991, 2750), "rule F off");
    assert_eq!(got[1], (854, 128, 971, 2750), "rule F on");
    assert_eq!(
        got[0].0 + got[0].1 + got[0].2,
        got[1].0 + got[1].1 + got[1].2,
        "the same decisions, re-attributed: no decision is lost"
    );
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
/// (`work/sym/plate-ceiling-is-now-the-scaffold-pushforward`).
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
    // The three documents a gate can afford to name the set for,
    // selected BY NAME out of the shared table: a positional `take(3)`
    // would let a reorder or an insert change what this row measures
    // while the doc above still named these three.
    for name in ["two_hole_plate", "r1_annulus", "r2_link"] {
        let study = measured_studies(tol)
            .into_iter()
            .find(|study| study.name == name)
            .unwrap_or_else(|| panic!("{name} is not in measured_studies"));
        let (hi, at) = (study.refuses_at, &study.at);
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
                 (work/sym/plate-ceiling-is-now-the-scaffold-pushforward)"
            );
        }
    }
}
