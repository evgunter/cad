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
/// `certifies_at` and `refuses_at` are the two ends of the ceiling's
/// bracket in multiples of ε, measurements and not targets; `door` is
/// what the registered-identity door is to the document at
/// `certifies_at`, the same at `1e-6`, `1e-9` and `1e-12`.
pub(crate) struct Study {
    pub(crate) name: &'static str,
    /// A multiple of ε that certified whole on M10-9's tier: the
    /// certifying end of the bracket M10-9 measured. On the shipped
    /// tree all five certify whole there (`refused_by`).
    pub(crate) certifies_at: f64,
    /// The predicate the whole-box replay refuses at, at
    /// `certifies_at` on the shipped tree, or `None` where it certifies
    /// whole there.
    pub(crate) refused_by: Option<&'static str>,
    /// A multiple of ε that refuses — `None` where the bisection found
    /// no refusal up to its top, `1e6·ε`.
    pub(crate) refuses_at: Option<f64>,
    /// What the registered-identity door is to this document
    /// ([`m10_9_what_the_registry_buys`]).
    pub(crate) door: Door,
    pub(crate) at: Box<dyn Fn(f64) -> ProfileDoc>,
}

/// **What the door is to a document**, read at its `certifies_at` by
/// replaying it whole with the door shut
/// (`SymRules::shipped_without_the_door`) beside the shipped replay.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Door {
    /// The door discharges nothing here: every decision it could answer
    /// is a theorem of the tier's own algebra, so shut, the replay reads
    /// the shipped receipt.
    Idle,
    /// Shut, the replay refuses at this predicate: the document
    /// certifies only through the door.
    #[expect(
        dead_code,
        reason = "none of the measured documents has a load-bearing door since the link's \
                  arcs depart along a minted axis; the row still reads one"
    )]
    LoadBearing(&'static str),
    /// Shut, the replay still certifies and decides the same decisions,
    /// and this many of them move from `symbolic` to `numeric`: what the
    /// door discharges here. A registrant that started stating
    /// something slightly false stops discharging without ever being
    /// refused (R2 planted `‖q − c‖ ≡ r · (1 + 2ε)` in
    /// `register_rim_identity`, and the enclosures still met), and this
    /// number is what that moves; no decision the door does not answer
    /// moves it.
    Buys(u64),
}

/// The five, in the order every row here reports them.
///
/// The scales are M10-9's brackets, the link's lowered to its own
/// ceiling, and all five certify whole at them. `refused_by` stays for
/// the next document that refuses.
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
            door: Door::Idle,
            at: Box::new(move |s: f64| crate::m10_7_plate::plate(5.0e-5 * s, 1.0e-5 * s, tol).0),
        },
        Study {
            name: "r1_annulus",
            certifies_at: 7.805e2,
            refuses_at: None,
            refused_by: None,
            // As the plate: the circles' stored carrier makes every rim
            // decision a theorem.
            door: Door::Idle,
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
            // numerically (`carrier_matches_mapped_source`), and it leaves
            // the band first at 4.395e2·ε, door open or shut; the bracket's
            // certifying end measures 4.393e2.
            certifies_at: 3.029e2,
            refuses_at: Some(4.395e2),
            refused_by: None,
            // Door shut the link certifies whole: the four decisions the
            // registry discharges are numeric, inside the band.
            door: Door::Buys(4),
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
            // centre's algebra, so its `dihedral_wedge` decisions, and the
            // tangent certificate's gap rows on the fillet's contact
            // edges, are registered rather than proved. Fifty on this
            // tree: fewer of the bracket's decisions reach the door at
            // all (`work/sym/the-bracket-proves-fewer-and-freezes-more-with-the-radius-stored-as-decided.md`).
            door: Door::Buys(50),
            at: Box::new(move |s: f64| crate::m10_7_r2_probes_interval::bracket(s, tol).0),
        },
        Study {
            name: "r2_rounded_pad",
            // Its straight legs' chord directors are their components
            // (`profile`'s `path::chord_director`), so `line_span` no
            // longer bounds it.
            certifies_at: 3.151e3,
            // Bounded by `arc_span`, in the profile's validation.
            refuses_at: Some(3.152e3),
            refused_by: None,
            // With its legs' directors exact components, the tier proves
            // what the door used to discharge, so shut it reads the
            // shipped receipt.
            door: Door::Buys(0),
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
            &editor_core::Stop::default(),
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

/// **THE MECHANISM, in the receipt.** A replay of the plate at
/// `1.6e3 · ε` with the door SHUT registers nothing; the same replay
/// with the door OPEN discharges through the registry: the arc
/// carrier's endpoint pinnings (`carrier.eval(0) = q_from`,
/// `carrier.eval(param_end) = q_to`) and its radial facts. The rim
/// identity `‖q − c‖ = r` is not among them: nothing registers it, and
/// a vertex's gap from its rim is a theorem with the door shut.
///
/// Both replays certify whole, so both decide the same population and
/// the door's whole effect is decisions moved out of `numeric`.
#[test]
fn m10_9_the_plates_registrants_discharge_through_the_door() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let doc = crate::m10_7_plate::plate(5.0e-5 * 1.6e3 * eps, 1.0e-5 * 1.6e3 * eps, tol).0;
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let box_ = ParamBox::of(&analyzed);
    let (shut_refusal, shut) = replay_counts(&doc, &box_, closed(), tol);
    let (open_refusal, open) = replay_counts(&doc, &box_, opened(), tol);
    println!(
        "   door shut {shut:?} -> {shut_refusal:?}\n   door open {open:?} -> {open_refusal:?}"
    );
    assert!(
        shut_refusal.is_none() && open_refusal.is_none(),
        "the plate certifies whole at 1.6e3·ε with the door shut and open alike: \
         {shut_refusal:?}, {open_refusal:?}"
    );
    assert!(
        open.symbolic > shut.symbolic,
        "the registrants discharge on the plate: {open:?} vs {shut:?}"
    );
    assert_eq!(
        (open.decisions(), open.symbolic - shut.symbolic),
        (shut.decisions(), shut.numeric - open.numeric),
        "neither replay refuses, so both decide the same sites and each decision the door \
         discharges leaves `numeric`: {open:?} vs {shut:?}"
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
            &editor_core::Stop::default(),
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
/// door was missing, at fixture scale, and the assertions that make it
/// one.
///
/// The five measured documents are replayed at `Sym<Interval>` with the
/// shipped set, at the certifying end of M10-9's bracket for each
/// (`measured_studies`), where each certifies whole or refuses where
/// `Study::refused_by` says. **The width the row reads at is the
/// ANALYZED BOX** — `analyzed_box(doc, AnalysisPolicy::default())`, the
/// same box the driver replays over — and that width is what the exact
/// witness tests against, so the claim below is about the door's
/// answers over that box and not over a point.
///
/// **What each assertion catches, because no one of them catches
/// everything** (R2 MAJOR-1 / R1 MINOR-4 measured exactly this):
///
/// - **What the registry buys** ([`m10_9_what_the_registry_buys`]). A
///   registration the exact witness ADMITS but should not is invisible
///   to a refusal count: at `Sym<Interval>` two certified enclosures
///   that still MEET are `Witnessed`, which is the door's contract and
///   not a defect it can see. A small lie stops the registry
///   discharging, and that row is where it shows ([`Door`]).
/// - **The registrants' own `debug_assert!`** (`swept::handle_registration`).
///   A GEOMETRIC lie separates the enclosures, the exact witness answers
///   `Contradicted`, and the registrant aborts — first, and in every CI
///   profile, because this workspace ships `debug-assertions = true` in
///   release too. So a 0.1 % lie reds this row through a panic and never
///   reaches the count below.
/// - **`registrations_refused == 0`.** What is left once those two have
///   had their turn: `Cyclic`, and any FUTURE registrant that binds the
///   refusal without asserting. It is the backstop, not the loud
///   channel.
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
        match study.refused_by {
            None => assert!(
                refusal.is_none(),
                "{name} at eps={eps:e}: certifies whole at M10-9's scale: {refusal:?}"
            ),
            Some(predicate) => assert!(
                refusal.as_deref().is_some_and(|r| r.contains(predicate)),
                "{name} at eps={eps:e}: refuses at `{predicate}` at M10-9's scale: {refusal:?}"
            ),
        }
        assert!(
            counts.symbolic > 0,
            "{name} at eps={eps:e}: the tier proved nothing, so no registrant was read: {counts:?}"
        );
        assert_eq!(
            counts.registrations_refused, 0,
            "{name} at eps={eps:e}: a registration was refused on a real document — at \
             this lane that is `Cyclic`, or an exact-witness refusal from a registrant \
             that binds it instead of asserting: {counts:?}"
        );
    }
}

/// **WHAT THE REGISTRY BUYS**, as a differential in decisions and
/// verdicts: each measured document the door discharges on, replayed
/// whole at its `certifies_at` with the door shut (`registered: false`,
/// every other dial as shipped) beside the shipped replay, read against
/// its [`Door`]. Where the door-shut replay certifies it decides what
/// the shipped one does, and the door's decisions move from `symbolic`
/// to `numeric`; a load-bearing door's document refuses shut (none of
/// the five is one since the link's arcs depart along a minted axis). A registrant that started stating something slightly false
/// stops discharging without ever being refused, and this is where that
/// shows: what the door buys falls, and a load-bearing door's document
/// refuses.
///
/// **What runs it: the slow set** (`.config/nextest.toml`): two
/// whole-box replays of each of the five.
#[test]
fn m10_9_what_the_registry_buys() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let mut moved: Vec<String> = Vec::new();
    for study in measured_studies(tol) {
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
        let bought = match study.door {
            Door::Idle => Some(0),
            Door::Buys(n) => Some(n),
            Door::LoadBearing(_) => None,
        };
        match (study.door, bought, &off_refusal) {
            (Door::LoadBearing(predicate), _, Some(r)) => {
                if !r.contains(predicate) {
                    moved.push(format!(
                        "{name}: shut, the replay refuses, but not at `{predicate}`: {r}"
                    ));
                }
            }
            (Door::LoadBearing(_), _, None) => moved.push(format!(
                "{name}: the door is load-bearing, but shut the replay certifies: {off:?}"
            )),
            (_, _, Some(r)) => moved.push(format!(
                "{name}: shut, the replay refuses where it certified: {r}"
            )),
            (_, Some(bought), None) => {
                if off.decisions() != on.decisions() {
                    moved.push(format!(
                        "{name}: both certify, so the door moves decisions between columns \
                         and decides no others: {on:?} vs {off:?}"
                    ));
                }
                let got = on.symbolic.checked_sub(off.symbolic);
                if got != Some(bought) {
                    moved.push(format!(
                        "{name}: the door discharges {got:?} decisions, measured {bought}: \
                         {on:?} vs {off:?}"
                    ));
                }
            }
            (_, None, None) => unreachable!("only a load-bearing door buys no count"),
        }
    }
    assert!(
        moved.is_empty(),
        "at eps={eps:e}, what the registry buys moved: {moved:?}"
    );
}

/// **THE PAD AT BOTH RULE-F DIALS** (adopted from SYM-8's review R2,
/// `sym8_r2_the_pads_four_re_taken`): the DIFFERENTIAL for rule F alone
/// (`SymRules::without_rule_f`: both arms shut, every other rule as
/// shipped). The axis-order basis mints no `copysign` for rule F to
/// fold, and at the scale the pad certifies whole at, over its analyzed
/// box, the two dials read the same receipt — every decision column and
/// the forms frozen — at every ε row
/// (`work/sym/the-negative-arm-lost-its-document-consumer`).
///
/// **What runs it: the slow set** (`.config/nextest.toml`), at the
/// default ε on every PR whose diff seeds `editor-core`, and every night
/// at every ε row. It is two whole-box pad replays on top of the gating
/// row's, which keeps it off the per-PR fast set.
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
    assert!(
        got[0].0 > 0,
        "rule F shut: the tier proves something on the pad"
    );
    assert_eq!(
        got[1], got[0],
        "rule F moves nothing on the pad: the basis mints no copysign for it to fold"
    );
}
