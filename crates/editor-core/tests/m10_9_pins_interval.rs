//! **M10-9's positive pins** — the registered-identity door
//! (ERROR-DESIGN E12's provenance reserve), asserted as the measured
//! STATE it is: what the door discharges, and what it costs the value
//! channel (nothing).
//!
//! The gating half; `m10_9_evidence_interval` is the evidence half
//! (`#[ignore]`d) and `m10_8_harness` the shared probe. The five
//! measured documents and their ε-relative ceiling brackets live in
//! [`measured_studies`], which the evidence half and the exact-channel
//! rows (`sym11_exact_channel_rows`, `decide_1_self_dot_interval`) read.
//! The arc carrier's builder registers both of its same-object
//! identities (the rim `‖q − c‖ = r` and the span
//! `carrier.eval(param_end) = q_to`), both discharge outright, and what
//! bounds every measured document is `carrier_matches_mapped_source` —
//! the carrier against the scaffold pushforward, an identity between two
//! independently built objects — door open and door SHUT alike
//! (`work/sym/plate-ceiling-is-now-the-scaffold-pushforward`).
//!
//! **Which tier these rows pin.** M10-9's — the shipped set with the
//! form-level algebra OFF (`SymRules::without_the_algebra`), which is
//! that tier bit for bit and the differential the algebra is measured
//! against; the shipped set now carries rules A/B per node and rule D
//! on top of the door, and its state is `m10_10_pins_interval`'s.
//! Holding at every row here is what "the algebra off reproduces
//! M10-9" means in assertable form.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::ProfileDoc;
use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::drive::{DriveConfig, drive};
use geom_core::{SymRules, Tol};

use crate::m10_8_harness::dials;

/// **One measured document, and everything measured about it** — the
/// single home for the five studies this file and
/// `m10_9_evidence_interval` drive.
///
/// Every field is a MEASUREMENT, not a target: `certifies_at` and
/// `refuses_at` are the two ends of the ceiling's bracket in multiples
/// of ε, and `registered` is how many decisions the registered-identity
/// door discharges on the document at `certifies_at` (ε-independent —
/// the same at `1e-6`, `1e-9` and `1e-12`, and both reviews re-measured
/// it). Every reader reads this array; a re-measured ceiling or a moved
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
            symbolic_zero: 803,
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
            // 90 / 515 until the carrier's span was spelled as the
            // stored sweep signed by the decided turn (`swept::arc_span`)
            // instead of `4·atan|b|`: the span then shares the
            // pushforward's `atan b` atom, and of the link's 1102
            // decisions 20 move INTO the door — 10 out of `numeric` and
            // TEN out of `symbolic_zero` (505 now). Those ten are the
            // same class as the pad's four above (a residual the early
            // walk was cancelling over reshaped, then re-taken by the
            // registry). `frozen` is 1060 either way, nothing is
            // refused or contradicted, and no ceiling moves.
            registered: 110,
            symbolic_zero: 505,
            at: Box::new(move |s: f64| crate::m10_9_r2_probes_interval::link(s, tol).0),
        },
        Study {
            name: "r2_filleted_bracket",
            certifies_at: 3.870e2,
            refuses_at: 3.873e2,
            registered: 144,
            symbolic_zero: 1083,
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
            //
            // 854 until the must-carry rule gated each station
            // first-order (`geom_brep::must_carry_over_edge` asks
            // `classify_dihedral` before the second-order jet). That
            // is none of the three causes above but a fourth: the
            // document asks MORE decisions, and no decision moves.
            // The pad reaches the rule on 16 edges, each read at the
            // 7 interior stations, and every station's
            // `dihedral_wedge` is one new decision — 112, of which
            // 28 are theorems and 84 numeric; `registered` and
            // `frozen` do not move, and with the gate's call removed
            // this replay reads 854 again. The link (8 edges) and
            // the bracket (4) take the same 7 per edge, all numeric,
            // so their pins here hold. Every rule-reached edge on the
            // five reads `JetDeterminate` with the gate and without,
            // so the document the replay builds is the same one.
            registered: 128,
            symbolic_zero: 882,
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
/// is evaluation order.
#[test]
fn m10_9_the_rim_registrant_discharges_the_plates_endpoint_identity() {
    let tol = Tol::witness();
    let eps = tol.eps();
    // Past the measured ceiling (`[7.811e2, 7.814e2] · ε`,
    // `measured_studies`).
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
/// - **`theorems_disputed == 0`**, the `Interval` twin of SYM-11's
///   dispute count. At an EXACT witness a definite non-zero sign is a
///   certified bracket that excludes zero, so no form over the
///   parameters can be the zero polynomial under it; the
///   contradiction is asserted at this scalar rather than counted, and
///   a count here says the const on `Interval` moved. The same claim
///   past each document's ceiling — where the residuals are the widest
///   the corpus produces — is `sym11_exact_channel_rows`.
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
        assert_eq!(
            counts.theorems_disputed, 0,
            "{name} at eps={eps:e}: what this pins is the CHARGE'S ARM at this lane. \
             `Interval::WITNESS` is `Exact`, which routes a theorem-vs-numeric \
             contradiction to the `debug_assert!` and never to this column, and that \
             assertion is live in every profile this workspace builds — so a count here \
             is the const having moved, and the soundness question it would otherwise \
             raise has already been answered by the panic that did not happen: {counts:?}"
        );
    }
}

/// **THE PAD'S FOUR, AT BOTH DIALS** (adopted from SYM-8's review R2,
/// `sym8_r2_the_pads_four_re_taken`). The row above pins the shipped
/// side; this one is the DIFFERENTIAL that says what rule F
/// (`SymRules::manifest_sign`) did to it. At the scale the pad
/// certifies whole at, over its analyzed box, rule F off → on:
/// `symbolic_zero` 886 → 882, `registered` 104 → 128, `numeric`
/// 1075 → 1055, `frozen` 2750 either way — the same 2065 decisions, 24
/// of them moving into the door, twenty out of `numeric` and FOUR out
/// of `symbolic_zero`. Those are the STORED tuples; the replay
/// currently measures `numeric` 8 higher and `frozen` 2722 at both
/// dials, a drift that predates the stored values and is not yet
/// attributed
/// (`work/sym/ignored-sym-receipt-rows-drifted-red-on-main-unattributed`),
/// so this row is red until it is.
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
    // Each side carries +28 `symbolic_zero` and +84 `numeric` from the
    // must-carry rule's per-station dihedral gate.
    assert_eq!(got[0], (886, 104, 1075, 2750), "rule F off");
    assert_eq!(got[1], (882, 128, 1055, 2750), "rule F on");
    assert_eq!(
        got[0].0 + got[0].1 + got[0].2,
        got[1].0 + got[1].1 + got[1].2,
        "the same decisions, re-attributed: no decision is lost"
    );
}
