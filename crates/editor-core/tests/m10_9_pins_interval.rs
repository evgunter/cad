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
//! bounds every measured document but the pad is
//! `carrier_matches_mapped_source` — the carrier against the scaffold
//! pushforward, an identity between two independently built objects —
//! door open and door SHUT alike
//! (`work/sym/plate-ceiling-is-now-the-scaffold-pushforward`). The pad
//! is bounded by `arc_span`, in the profile's validation.
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
/// of ε, and `without_the_registry` is the document's receipt and
/// verdict at `certifies_at` with the registered-identity door shut
/// (ε-independent — the same at `1e-6`, `1e-9` and `1e-12`). Every reader reads this array; a re-measured ceiling or a moved
/// count is edited once (R1 S1: it used to be typed out three times,
/// magic constants and builders both).
pub(crate) struct Study {
    pub(crate) name: &'static str,
    /// A multiple of ε that certified whole on M10-9's tier: the
    /// certifying end of the bracket M10-9 measured. On the shipped
    /// tree all five certify whole there (`refused_by`).
    pub(crate) certifies_at: f64,
    /// The predicate the whole-box replay refuses at, at
    /// `certifies_at` on the shipped tree, or `None` where it certifies
    /// whole there. Asserted, so the counts beside it are known to be
    /// a truncated replay's where they are one.
    pub(crate) refused_by: Option<&'static str>,
    /// A multiple of ε that refuses — `None` where the bisection found
    /// no refusal up to its top, `1e6·ε`.
    pub(crate) refuses_at: Option<f64>,
    /// The replay at `certifies_at` with the door shut
    /// (`SymRules::shipped_without_the_door`): `(symbolic, numeric)`,
    /// and the predicate it refuses at, `None` where it certifies.
    /// `None` for a document the door discharges nothing on
    /// (`m10_9_what_the_registry_buys`).
    pub(crate) without_the_registry: Option<((u64, u64), Option<&'static str>)>,
    /// `SymCounts::symbolic` at `certifies_at`, shipped set.
    ///
    /// Per ε row (`1e-6`, `1e-9`, `1e-12`). A document that certifies
    /// whole reads the same count at all three, as all five do now; a
    /// refusing document's replay stops at its refusal, whose prefix can
    /// differ by row, so the count is pinned per row rather than
    /// averaged into one.
    pub(crate) symbolic: [u64; 3],
    pub(crate) at: Box<dyn Fn(f64) -> ProfileDoc>,
}

/// The five, in the order every row here reports them.
///
/// The scales are M10-9's brackets, the link's lowered to its own
/// ceiling, and all five certify whole at them. `refused_by` stays for
/// the next document that refuses. How each count moved since SYM-11
/// (2026-09-21), merge by merge, is the attribution table in
/// `work/sym/ignored-sym-receipt-rows-drifted-red-on-main-unattributed`;
/// the earlier reasons (rule E's and rule F's trades, the must-carry
/// derivation) are in this doc's git history (`git log -L`).
///
/// INTENT-LITERALS PR C: a formula written at a slot is an anonymous
/// definition, bound through the non-finite door, so each written
/// formula adds a theorem (plate +1, annulus +2, link +2, bracket +3;
/// the pad holds). `registered` and the verdicts do not move, and an
/// untoleranced variable binds as its nominal (VR8).
///
/// `expr-non-finite-asks-value-times-zero`: the final non-finite check
/// asks boundedness and decides nothing, so its theorems leave every
/// `symbolic` count (plate −45, annulus −27, link −35, bracket −61,
/// pad −61). Numeric, frozen, registered and the verdicts do not move.
///
/// PCERT unit B1 (C4: a row is its image placed over the edge's span in
/// the chart's principal window) reads a cylinder's meridian azimuth only
/// on a meridian, so a rim circle centred on the axis no longer takes the
/// frame read the tier proved zero: `symbolic` plate 1048 → 1032,
/// annulus 566 → 550, link 862 → 846, bracket 1448 → 1428, pad
/// 1372 → 1348. Each placed azimuth's two half-period marks are read
/// numerically, so with the door shut the link reads (815, 712) →
/// (799, 780), still refusing at `pcurve_envelope`, the bracket
/// (1391, 1061) → (1371, 1149) and the pad (1308, 1348) → (1284, 1448).
/// No registrant moved and no verdict moved.
///
/// PCERT B2a stores a harmonic row's windings as structure, so the
/// decisions that read a winding off a slope are gone (the angle
/// channels' affine decisions, the winding walk, the harmonic period
/// gate): `symbolic` plate 987 → 939, annulus 523 → 475, link
/// 811 → 763, bracket 1367 → 1307, pad 1283 → 1211; with the door shut
/// the link (766, 780) → (718, 740), still refusing at
/// `pcurve_envelope`, the bracket (1310, 1149) → (1250, 1099) and the pad
/// (1219, 1452) → (1147, 1392). No registrant moved and no verdict
/// moved.
///
/// The profile's joint pass reads each joint's turn
/// (`path_junction_turn`, #5354's ruling) where it read a line/arc
/// joint's carrier clearance, numerically: the tier proves the turn, so
/// those decisions move from `numeric` to `symbolic`, link 763 → 767,
/// bracket 1307 → 1311, pad 1211 → 1223; with the door shut the link
/// (718, 740) → (722, 736), still refusing at `pcurve_envelope`, the
/// bracket (1250, 1099) → (1252, 1102) and the pad (1147, 1392) →
/// (1157, 1382). No registrant moved and no verdict moved.
///
/// Its fix pass levers the turn by the shorter of the two legs' arms,
/// and the path door reads each junction a departing leg follows again
/// at that arm once the leg is stored: the bracket's door-shut numeric
/// 1102 → 1106 (four more junction reads), and the pad 1373 → 1389
/// symbolic, (1373, 1166) → (1389, 1150) shut: sixteen decisions move
/// from `numeric` to `symbolic`, the total unchanged. No registrant
/// moved and no verdict moved.
///
/// Check 4 deciding no whole period (PCERT B2b) takes fidelity's numeric
/// branch decisions out of each door-shut replay, measured on the tree
/// with the joint pass above: the link (722, 736) → (722, 704), still
/// refusing at `pcurve_envelope`, the bracket (1252, 1106) →
/// (1252, 1066) and the pad (1389, 1150) → (1389, 1102). `symbolic`
/// holds, no registrant moved and no verdict moved.
///
/// INTENT stage 3 D places each construction a document combines into
/// its target's space by a plane mate and its values, so each placement
/// reads its mate's chart: three more THEOREMS per placement, at every
/// ε row (plate and bracket +6, two placements each; annulus, link and
/// pad +3, one each). The verdicts do not move. With the registry's
/// door shut the receipts move with them: the link 782/712 -> 782/710,
/// the bracket 1330/1061 -> 1336/1069, the pad 1247/1348 -> 1252/1349,
/// refusing (or not) where they did.
///
/// A placement whose bundle reads a declared or toleranced variable
/// runs its map at the nominal (C-5 narrowed, the orchestrator's
/// ruling): it is no longer snapped at rest, so its transform and the
/// copied body's re-certification decide over the box, 186 more
/// THEOREMS per such placement at every ε row (bracket +372 over two;
/// annulus, link and pad +186). The plate's two holes are placed by
/// their start caps and written numbers alone, which read no variable,
/// so they still snap and the plate holds. All five still certify
/// whole. With the door shut the
/// bracket reads 1708/1387 and the pad 1438/1508; the link's still
/// refuses at `pcurve_envelope`, its receipt 782/710 unmoved.
///
/// Re-taken on INTENT stage 3 D under FORK-REST (only the identity
/// bundle as written places by no map), identical at all three ε rows.
/// The plate moved by FORK-REST alone: its two holes are placed by
/// plane mates, which the narrowing's walk let snap (their start caps
/// called fixed), and now run their maps, so their transforms and their
/// copies' re-certification decide over the box: 1253 symbolic (it read
/// 945 on D before FORK-REST). The other four read what D measured
/// before FORK-REST, after merging main at `0a1d95321e` (annulus 632;
/// the bracket 1566/1456 and the pad 1546/1324/3024 with the door
/// shut): their placements read a declared or toleranced variable and
/// already ran their maps, so FORK-REST moves none of them. Against the
/// narrowing's receipts above, main's merge took decisions off the
/// annulus (728 -> 632), the link (1016 -> 924) and the bracket (1765
/// -> 1625) and added to the pad (1500 -> 1546); with the door shut the
/// link reads 722/734, still refusing at `pcurve_envelope`, the bracket
/// 1566/1456 and the pad 1546/1324. All five still certify whole and
/// the door still moves decisions between columns and decides no
/// others.
///
/// Merging main at `557e581a64` (check 4 deciding no whole period,
/// PCERT B2b, on D's tree where the plate's and the bracket's copies
/// run their maps) takes fidelity's numeric branch decisions out of
/// each door-shut replay: the link 722/734 -> 722/702, still refusing
/// at `pcurve_envelope`, the bracket 1566/1456 -> 1566/1384 and the
/// pad 1546/1324 -> 1546/1260. `symbolic` holds. Measured identical with
/// a written fraction of a turn read as its rounded radian, so exact
/// turns move none of these.
///
/// Re-taken on INTENT stage 3 E (a copy is its source's body under its
/// frame), identical at all three ε rows: the plate 1253 -> 1105 and the
/// bracket 1625 -> 1477, 148 each, the decisions of their copies' maps
/// and re-certification, which no longer run. The annulus, the link and
/// the pad held; none refuses a registration or disputes a theorem.
/// Over main's merge at `1f851964` (plate 1249, bracket 1621) the same
/// 148 each: the plate 1101 and the bracket 1473.
///
/// **Pinned at what main measures since PR 5728** (limb 2 for every
/// carrier), identical at all three ε rows: the plate 1101 -> 1149,
/// the annulus 630 -> 654, the link 922 -> 954, the bracket 1473 ->
/// 1535 and the pad 1544 -> 1592. None refuses a registration or
/// disputes a theorem. Which decisions the added symbolic ones are is
/// unread (`work/offset/main-red-three-m10-interval-pins-moved-at-5728`).
pub(crate) fn measured_studies(tol: Tol) -> [Study; 5] {
    [
        Study {
            name: "two_hole_plate",
            certifies_at: 7.811e2,
            refuses_at: None,
            refused_by: None,
            // The circles store their authored carrier, the radius as
            // `|r|`, so the rim at a vertex folds onto it in the tier's
            // own algebra: every rim decision is a THEOREM, none reaches
            // the door, and the plate refuses nowhere the bisection
            // reaches.
            without_the_registry: None,
            symbolic: [1149, 1149, 1149],
            at: Box::new(move |s: f64| crate::m10_7_plate::plate(5.0e-5 * s, 1.0e-5 * s, tol).0),
        },
        Study {
            name: "r1_annulus",
            certifies_at: 7.805e2,
            refuses_at: None,
            refused_by: None,
            // As the plate: the circles' stored carrier makes every rim
            // decision a theorem.
            without_the_registry: None,
            symbolic: [654, 654, 654],
            at: Box::new(move |s: f64| crate::m10_8_r1_probes_interval::annulus(s, tol).0),
        },
        Study {
            name: "r2_link",
            // The tangent arc stores Δθ = 4·atan(X), X the select
            // between `across / (|d| + along)` and `(|d| − along) /
            // across` (`profile`'s `sugar::split_half_tan`); at a
            // half-turn whose chord is the parameter `half_w`, `along` is
            // zero and X reads `2w / |2w|`, a sign the tier does not
            // hold. The rim identity the arc registers sits inside a
            // frozen compound, the mapped-source residual is read
            // numerically, and it leaves the band first at 3.030e2·ε.
            certifies_at: 3.029e2,
            refuses_at: Some(3.030e2),
            refused_by: None,
            // Door shut, the extrude's rim (half-edge 24v1) refuses at
            // check 4. The refusal decides the terms in order up to the
            // first one over the band alone, `Radius` (the class
            // circle's radius and orientation read as one pair), so it
            // decides 3 of the envelope's 9 terms on each of its two
            // reads.
            //
            // reads. Under INTENT stage 3 D's placement it decides two
            // fewer numeric comparisons on the way (712 -> 710).
            // Pinned at what main measures (unbisected between PR 5728
            // and PR 5811, `work/offset/main-red-three-m10-interval-pins-moved-at-5728`): shut, the door now refuses earlier,
            // at limb 2's stall against the cylinder rather than at
            // `pcurve_envelope`, after (709, 585), where it read (722, 702).
            without_the_registry: Some(((709, 585), Some("Stalled { operand: Cylinder"))),
            symbolic: [962, 962, 962],
            at: Box::new(move |s: f64| crate::m10_9_r2_probes_interval::link(s, tol).0),
        },
        Study {
            name: "r2_filleted_bracket",
            certifies_at: 3.870e2,
            // Bounded by `arc_span`, in the profile's validation.
            refuses_at: Some(7.624e2),
            refused_by: None,
            // The fillet's tangency is the registration
            // `centre ≡ t1 + σ·r·n̂₁`, not an identity of the offset
            // centre's algebra, so eight `dihedral_wedge` decisions are
            // registered rather than proved. The tangent certificate's
            // gap rows on the fillet's contact edges added ten (47 → 57).
            // Pinned at what main measures: (1566, 1384) -> (1474, 1228),
            // unbisected between PR 5728 and PR 5811 (`work/offset/main-red-three-m10-interval-pins-moved-at-5728`).
            // Then (1474, 1228) -> (1474, 1218) when the simplicity pass
            // stopped reading a gap on a line pair that shares a vertex
            // (PR 5952).
            without_the_registry: Some(((1474, 1218), None)),
            // One is the fillet run out read against its arrival carrier
            // (`path_run_out_carrier`), a margin the tier proves zero.
            // How many the tier reaches depends on the order the
            // variables' symbols sort in
            // (`work/rules/sym-tier-reach-depends-on-symbol-order`).
            symbolic: [1537, 1537, 1537],
            at: Box::new(move |s: f64| crate::m10_7_r2_probes_interval::bracket(s, tol).0),
        },
        Study {
            name: "r2_rounded_pad",
            // Its straight legs' chord directors are their components
            // (`profile`'s `path::chord_director`), so `line_span` no
            // longer bounds it (it did at 2.7783e3).
            certifies_at: 3.151e3,
            // Bounded by `arc_span`, in the profile's validation.
            refuses_at: Some(3.152e3),
            refused_by: None,
            // Rule F's differential on this document is
            // `m10_9_the_pad_at_both_rule_f_dials`'s. Door shut, the
            // pad reads the shipped receipt: with its legs' directors
            // exact components, the tier proves what the door used to
            // discharge (64 decisions, `(1147, 1392)` shut against
            // `(1211, 1328)` shipped, when they were angles).
            // Pinned at what main measures: (1546, 1260) -> (1592, 1250),
            // the same receipt as shipped, unbisected between PR 5728 and
            // PR 5811 (`work/offset/main-red-three-m10-interval-pins-moved-at-5728`).
            without_the_registry: Some(((1592, 1250), None)),
            // Three are the fillet run outs read against their arrival
            // carriers (`path_run_out_carrier`), margins the tier proves
            // zero.
            symbolic: [1592, 1592, 1592],
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

    use editor_core::{CancelToken, EvalOptions, NodeResult, evaluate};

    let opts = EvalOptions {
        param_box: Some(Arc::new(box_.clone())),
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
            Some(NodeResult::Failed(e)) => {
                Some(format!("node {} — {} — {:?}", id.0, e.kind, e.kind))
            }
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
/// identities).
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
    assert!(
        open.symbolic > shut.symbolic,
        "the rim registrant discharges on the plate: {open:?} vs {shut:?}"
    );
    // NOT "and `numeric` shrank": a REFUSING replay stops at its first
    // refusal, so the two runs do not decide the same population — the
    // door lets this one get further, and the decisions past the old
    // refusal are decisions the shut run never reached. The
    // out-of-`numeric`-and-nothing-else claim is pinned where every
    // site decides: at the scalar
    // (`geom_core::sym`'s `a_registered_identity_decides_zero`).
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
/// shipped set, at the certifying end of M10-9's bracket for each
/// (`measured_studies`): the plate, the annulus and the link certify
/// whole there, and the bracket and the pad refuse at
/// `pcurve_envelope`, asserted (`Study::refused_by`), so their counts
/// are over the decisions taken before it. **The width the row reads at is the ANALYZED
/// BOX** — `analyzed_box(doc, AnalysisPolicy::default())`, the same box
/// the driver replays over — and that width is what the exact witness
/// tests against, so the claim below is about the door's answers over
/// that box and not over a point.
///
/// **What each assertion catches, because no one of them catches
/// everything** (R2 MAJOR-1 / R1 MINOR-4 measured exactly this):
///
/// - **`symbolic`, pinned PER DOCUMENT**, and what the registry buys
///   of it ([`m10_9_what_the_registry_buys`]). A registration the exact
///   witness ADMITS but should not is invisible to a refusal count: at
///   `Sym<Interval>` two certified enclosures that still MEET are
///   `Witnessed`, which is the door's contract and not a defect it can
///   see. R2 planted `‖q − c‖ ≡ r · (1 + 2ε)` in `register_rim_identity`
///   and the enclosures still met — but the registry stops discharging,
///   so what it buys collapses and `numeric` rises. **These counts are
///   what a small lie moves**, so they are pinned, and they are
///   ε-independent: the same five numbers at `1e-6`, `1e-9` and
///   `1e-12`.
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
    let row = [1.0e-6, 1.0e-9, 1.0e-12]
        .iter()
        .position(|&e| (eps / e - 1.0).abs() < 1.0e-3)
        .unwrap_or_else(|| panic!("no measured row at eps = {eps:e}: measure one and add it"));
    // A moved count is a measurement, so every study's is gathered and
    // the row names them all at once; a refusal, a refused registration
    // or a disputed theorem stops the row where it is found.
    let mut moved: Vec<String> = Vec::new();
    for study in measured_studies(tol) {
        let name = study.name;
        let doc = (study.at)(study.certifies_at * eps);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let (refusal, counts) =
            replay_counts(&doc, &ParamBox::of(&analyzed), SymRules::shipped(), tol);
        println!("   {name} at eps={eps:e}: {counts:?} -> {refusal:?}");
        match study.refused_by {
            None => assert!(
                refusal.is_none(),
                "{name} at eps={eps:e}: certifies whole at M10-9's scale: {refusal:?}"
            ),
            Some(predicate) => assert!(
                refusal.as_deref().is_some_and(|r| r.contains(predicate)),
                "{name} at eps={eps:e}: refuses at `{predicate}` at M10-9's scale \
                 (`work/pcert/fillet-meridian-radius-term-is-registered-only`): {refusal:?}"
            ),
        }
        if counts.symbolic != study.symbolic[row] {
            moved.push(format!(
                "{name}: {} symbolic, measured {}: {counts:?}",
                counts.symbolic, study.symbolic[row]
            ));
        }
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
    assert!(
        moved.is_empty(),
        "at eps={eps:e} the tier proves a DIFFERENT number of decisions than the \
         measurement — a changed fold rule, a registrant that stopped discharging, or a \
         re-cut document. Decide which before re-baselining:\n{}",
        moved.join("\n")
    );
}

/// **WHAT THE REGISTRY BUYS**, as a differential in decisions and
/// verdicts: each measured document the door discharges on, replayed
/// whole at its `certifies_at` with the door shut (`registered: false`,
/// every other dial as shipped), against `Study::without_the_registry`.
/// Where the door-shut replay certifies it decides what the shipped one
/// does, and the door's decisions move from `symbolic` to `numeric`; on
/// the link the door is load-bearing, and shut it refuses at
/// `pcurve_envelope`. A registrant that started stating something
/// slightly false stops discharging without ever being refused, and
/// this is where that shows.
///
/// **What runs it: the slow set** (`.config/nextest.toml`): two
/// whole-box replays of the link, the bracket and the pad.
#[test]
fn m10_9_what_the_registry_buys() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let mut moved: Vec<String> = Vec::new();
    for study in measured_studies(tol) {
        let Some((want, want_refusal)) = study.without_the_registry else {
            continue;
        };
        let name = study.name;
        let doc = (study.at)(study.certifies_at * eps);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let box_ = ParamBox::of(&analyzed);
        let (on_refusal, on) = replay_counts(&doc, &box_, SymRules::shipped(), tol);
        let (off_refusal, off) =
            replay_counts(&doc, &box_, SymRules::shipped_without_the_door(), tol);
        println!("   {name} at eps={eps:e}: shipped {on:?} -> {on_refusal:?}");
        println!("   {name} at eps={eps:e}: door shut {off:?} -> {off_refusal:?}");
        assert!(
            on_refusal.is_none(),
            "{name}: certifies shipped: {on_refusal:?}"
        );
        let refused_at = |r: &Option<String>| {
            r.as_deref().map(|r| {
                want_refusal
                    .filter(|p| r.contains(p))
                    .unwrap_or("another predicate")
            })
        };
        if (off.symbolic, off.numeric) != want || refused_at(&off_refusal) != want_refusal {
            moved.push(format!(
                "{name}: door shut {:?} refusing at {:?}, stored {want:?} refusing at \
                 {want_refusal:?}: {off_refusal:?}",
                (off.symbolic, off.numeric),
                refused_at(&off_refusal)
            ));
        }
        if off_refusal.is_none() && off.decisions() != on.decisions() {
            moved.push(format!(
                "{name}: both certify, so the door moves decisions between columns and \
                 decides no others: {on:?} vs {off:?}"
            ));
        }
    }
    assert!(
        moved.is_empty(),
        "at eps={eps:e}, what the registry buys moved: {moved:?}"
    );
}

/// **THE PAD AT BOTH RULE-F DIALS** (adopted from SYM-8's review R2,
/// `sym8_r2_the_pads_four_re_taken`). The row above pins the shipped
/// side; this one is the DIFFERENTIAL for rule F alone
/// (`SymRules::without_rule_f`: both arms shut, every other rule as
/// shipped). On Duff's basis rule F moved four of the pad's theorems
/// into the door. The axis-order basis mints no `copysign` for rule F
/// to fold, and at the scale the pad certifies whole at, over its
/// analyzed box, the two dials read the same receipt at every ε row
/// (`work/rules/the-negative-arm-lost-its-document-consumer`).
///
/// Re-taken on the tree that composes three changes: PCERT's
/// chart-angle unit and the extrude's cap decide
/// (`cap_plane_orientation`) set the decision columns, and an intrinsic
/// edge description's surfaces becoming a set
/// (`geom_brep::SurfacePair`) moves only the frozen count, which is
/// 3232 composed (3171 without the set).
/// The cause is the transversality wedge
/// (`geom_brep::dihedral::wedge_decided`'s `n1.cross(n2)`), which the
/// certificate reads in key order and the extrude's dihedral in builder
/// order: symmetric in value up to sign, not in form, so mixed
/// orientations build distinct hash-consed forms. No decision moved.
/// Check 4 reading a `cos t`/`sin t` pair at its exact sup instead of
/// its coefficient sum builds a longer form per envelope term: frozen
/// 3280 → 3294, every decision column held.
/// Check 4 reading one vector at its norm where it summed its parts,
/// each site measured by reverting it alone: the turning cylinder's
/// pair against its class circle +103 frozen and the cylinder class
/// selector's radial circle at its exact sup (`radial_amplitude`) +56,
/// 3294 → 3453; the fidelity channels' norm and the retired `Tilt` and
/// `Orientation` slots move nothing here, and every decision column
/// holds. The pcurve envelope's orthonormal chart squaring its frame by
/// cross products (`geom::square_frame`) builds its frame terms in
/// another form: frozen 3453 → 3426, every decision column held.
/// An edge's wedge levered at its faces' extent (`topo::wedge_reach`)
/// where it was levered at the edge's own chord reads each certified
/// edge's two faces' boundary vertices: frozen 3426 → 3972, every
/// decision column held, and both dials still read one receipt.
/// INTENT stage 3 D places the pad's construction by a plane mate and
/// its values: symbolic and numeric +3 each (the placement's chart),
/// and frozen 3972 → 774, with both dials still reading one receipt.
/// A placement whose bundle reads a variable running its map at the
/// nominal (C-5 narrowed) re-certifies the placed copy over the box:
/// symbolic 1314 → 1500, numeric 1287 → 1446, frozen 774 → 1406, both
/// dials still one receipt.
///
/// Placing each row in the chart's principal window (PCERT B1): the
/// frame reads on rims centred on the axis gone and two numeric marks
/// per placed azimuth, (1372, 1284, 3426) → (1348, 1384, 3444), at both
/// dials alike: `symbolic` −24 (the frame reads), `numeric` +100 (fifty
/// placements' two marks), and `frozen` +18, the marks' contradiction
/// checks whose forms froze; the frame reads were proved, so froze
/// nothing. With the wedge levered at its faces' extent as well, the two
/// moves add: (1348, 1384, 3990), the wedge's +546 frozen on top. The
/// placement's azimuth levered at the carrier's farthest reach from the
/// axis at the span's ends and midpoint (PR 5225's one lever rule), not
/// at one direction's length, builds its marks' forms from three points:
/// frozen 3990 → 4098, every decision column held. Main's final
/// non-finite check asking boundedness takes 61 theorems out of
/// `symbolic`: (1287, 1384, 4098). The frame decided across the
/// window's cut sends the four theorems that a direction sat exactly on
/// the old frame's cut (`x ≡ 0`) to the numeric channel, and their
/// marks' forms freeze: (1283, 1388, 4112).
///
/// A harmonic row's windings stored as structure (PCERT B2a): the
/// decisions that read them gone, (1283, 1388, 4112) → (1211, 1328,
/// 4112), at both dials alike. The joint pass reading each joint's
/// turn, which the tier proves where it read a clearance numerically:
/// (1211, 1328, 4112) → (1223, 1316, 4112).
///
/// A chord's director spelled as its components over its length
/// (`profile`'s `path::chord_director`) where it was the `sin_cos` of an
/// `atan2`: 162 decisions move from `numeric` to `symbolic`, and the
/// forms the walks freeze rise, (1211, 1328, 4112) → (1373, 1166, 4298),
/// at both dials alike. No decision is gained or lost. The same
/// receipt holds at the pad's new `certifies_at` (its ceiling rose, see
/// [`measured_studies`]). The junction's turn levered by the shorter
/// leg's arm, and read again at the door once the leaving leg is
/// stored: sixteen decisions move from `numeric` to `symbolic`, the
/// total unchanged, and one more form freezes, (1373, 1166, 4298) →
/// (1389, 1150, 4299). Check 4 deciding no whole period (PCERT B2b),
/// fidelity's 48 numeric branch decisions gone, measured on that tree:
/// (1389, 1150, 4299) → (1389, 1102, 4299), at both dials alike. The
/// line×line fillet's half-angle tangent read by the home's select
/// (`profile`'s `sugar::split_half_tan`), one atom to the plain walk
/// where the corner's `v₁·v₂` is not a constant: two more forms
/// freeze, (1389, 1102, 4299) → (1389, 1102, 4301), decisions held.
///
/// **What runs it: the slow set** (`.config/nextest.toml`), at the
/// default ε on every PR whose diff seeds `editor-core`, and every night
/// at every ε row. It is two whole-box pad replays on top of the gating
/// row's, which keeps it off the per-PR fast set.
///
/// Re-taken on D under FORK-REST: (1546, 1324, 3024) at both dials,
/// which D measured before FORK-REST too (the pad's one placement reads
/// a declared variable and already ran its map). It moved from (1500,
/// 1446, 1406) with main's merge at `0a1d95321e`: `symbolic` 1500 ->
/// 1546, `numeric` 1446 -> 1324 and `frozen` 1406 -> 3024.
///
/// Main's merge at `557e581a64` moved it to (1546, 1260, 3026): check 4
/// decides no whole period (PCERT B2b), `numeric` 1324 -> 1260, and the
/// line×line fillet's half-angle tangent is chosen by `select_le_zero`,
/// `frozen` 3024 -> 3026. Measured identical with a written fraction of
/// a turn read as its rounded radian.
#[test]
fn m10_9_the_pad_at_both_rule_f_dials() {
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
        ("rule F shut", SymRules::without_rule_f()),
        ("shipped    ", SymRules::shipped()),
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
        got.push((c.symbolic, c.numeric, c.frozen));
    }
    // Pinned at what main measures, (1546, 1260, 3026) -> (1592, 1250,
    // 2465) at every ε row, unbisected between PR 5728 and PR 5811
    // (`work/offset/main-red-three-m10-interval-pins-moved-at-5728`).
    assert_eq!(got[0], (1592, 1250, 2465), "rule F shut: the pad's receipt");
    assert_eq!(
        got[1], got[0],
        "rule F moves nothing on the pad: the basis mints no copysign for it to fold"
    );
}
