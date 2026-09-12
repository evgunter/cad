//! **The profile step vocabulary, across the crate boundary
//! (LIB-SWITCH §4; the S4 "one vocabulary, N hand-synced copies"
//! shape).**
//!
//! `profile`'s `transition_table!` declares each authoring verb once
//! and projects four artifacts from that declaration — but all four
//! are INSIDE `profile`. `editor-core` re-spells the same vocabulary
//! twice more, because `profile` has neither expressions nor serde and
//! by G1 layering must not gain them: `ProgramStep` (the Expr-valued
//! document form) and `persist::wire`'s `WireStep` (the persisted
//! form).
//!
//! Two of the three hops need no test, because the compiler already
//! refuses them:
//!
//! - `WireStep` is produced and consumed by matches that are
//!   exhaustive on `ProgramStep` and on `WireStep`, so neither can
//!   gain a variant the other lacks;
//! - `eval::feed_step`, `eval::feed_lane_step` and
//!   `LoopProgram::from_recorded` are exhaustive on `profile::Step`, so
//!   a verb the table gains breaks `editor-core` at compile —
//!   measured: one added table verb, and exactly those THREE sites.
//!   `feed_lane_step` (M10-P) is the lift's second key feed and joined
//!   the list when it landed; it is named here rather than left to be
//!   rediscovered, since the whole point of this list is that it is the
//!   set a reader can trust to be complete.
//!
//! The hop the compiler does NOT check is the one that CONSTRUCTS.
//! `res_step` matches `ProgramStep` and builds a `Step`, so both
//! compile errors above can be discharged without the document
//! vocabulary ever learning the verb — a refusal arm in
//! `from_recorded`, a tag in `feed_step` and one in `feed_lane_step`,
//! and the wire and the
//! expression-slot vocabularies are quietly short. This suite is that
//! hop's census, anchored on `profile::Verb::ALL`: the same anchor
//! `profile`'s own replay-coverage census uses, read from the same
//! declaration.
//!
//! # The arc modes, one level down
//!
//! The same three spellings carry a second vocabulary INSIDE the
//! steps — the §2c arc modes — and a verb-keyed census is blind to
//! it: every mode travels inside `ArcTo` and the three fused verbs,
//! so the verb census above is green whatever the modes do.
//!
//! The hops are the same ones, one level down. `program::spec_lit`
//! and both content-key hashers are exhaustive on `profile::ArcData`,
//! so a mode the kernel gains breaks this crate at compile — and, as
//! above, each break can be discharged where it stands while
//! `res_spec` keeps constructing and the document, wire and slot
//! vocabularies stay short. `profile` declares the mode set once and
//! projects `ArcMode::ALL` from that declaration; the mode census
//! below is keyed on it, and its witness is a MATCH on the tag, so a
//! mode with no document spelling does not fail an assertion here —
//! it fails to compile.
//!
//! # The target forms, beside the modes
//!
//! A third vocabulary rides the steps and the arc specs BOTH: where a
//! target-taking verb ends (`profile::Target`). It is keyed the same
//! way, on `TargetKind::ALL` projected from the same declaration as the
//! variants, and its census is below the mode one. A verb-keyed check
//! is blind to it for the verbs' reason and a mode-keyed check is blind
//! to it for a second: three of the six modes carry a target and three
//! carry none, so a form can go missing from every arc spelling with
//! every mode still present.
//!
//! The corpus below is deliberately NOT a legal lattice walk. Nothing
//! here replays: resolution, persistence and slot addressing are all
//! total over the data type, and legality is `profile`'s census to
//! keep.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{
    Dimension, Expr, LoopProgram, ParamEnv, ProfilePayload, ProfileProgram, ProgramArcData,
    ProgramStep, ProgramTarget, SlotId,
};
use profile::{ArcMode, TargetKind, Verb};

/// The plane the corpus programs name. These programs are resolved and
/// serialized on their own, never inserted into a document, so nothing
/// here reads the node it points at.
const SCAFFOLD_PLANE: editor_core::RecipeNodeId = editor_core::RecipeNodeId(0);

fn len(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).unwrap()
}
fn ang(v: f64) -> Expr {
    Expr::literal(v, Dimension::Angle).unwrap()
}
fn sca(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).unwrap()
}
fn pt(x: f64, y: f64) -> [Expr; 2] {
    [len(x), len(y)]
}
fn point(x: f64, y: f64) -> ProgramTarget {
    ProgramTarget::Point(pt(x, y))
}

/// One document spec per arc mode — the mode census's witness.
///
/// It is a MATCH on the mode tag, not a list, and that is the whole
/// point: a mode the kernel vocabulary gains has no arm here, so this
/// function stops compiling until the document vocabulary learns the
/// mode too. Every downstream spelling follows from that one addition
/// by exhaustiveness — the wire's two conversions, `spec_slots`'
/// roles, and the kernel construction in `res_spec`.
///
/// The witnesses spread the forms an arc spec can target across the
/// modes that take one, so the corpus reaches them without a second
/// walk.
fn mode_witness(mode: ArcMode) -> ProgramArcData {
    match mode {
        ArcMode::Radius => ProgramArcData::Radius {
            r: len(2.0),
            side: profile::ArcSide::Left,
        },
        ArcMode::Bulge => ProgramArcData::Bulge {
            target: point(2.0, 1.0),
            b: sca(0.3),
        },
        ArcMode::Via => ProgramArcData::Via {
            q: pt(4.5, 0.5),
            target: point(5.0, 1.0),
        },
        ArcMode::Center => ProgramArcData::Center {
            c: pt(6.0, 1.0),
            winding: profile::ArcSweep::Cw,
            target: ProgramTarget::Start,
        },
        ArcMode::Sweep => ProgramArcData::Sweep {
            r: len(1.5),
            side: profile::ArcSide::Left,
            angle: ang(0.6),
        },
        ArcMode::ArcLen => ProgramArcData::ArcLen {
            r: len(2.5),
            side: profile::ArcSide::Right,
            len: len(0.7),
        },
    }
}

/// One document target per kernel target form — the target census's
/// witness, and the corpus's form side.
///
/// It is a MATCH on the form tag, not a list, and that is the whole
/// point: a form the kernel vocabulary gains has no arm here, so this
/// function stops compiling until the document vocabulary learns the
/// form too. `mode_witness` above is the same construction one level
/// up, and `res_target`'s exhaustiveness on `ProgramTarget` is what
/// carries the addition into every downstream spelling.
fn target_witness(kind: TargetKind) -> ProgramTarget {
    match kind {
        TargetKind::Point => point(1.0, 0.0),
        TargetKind::Start => ProgramTarget::Start,
        TargetKind::StartArriving => ProgramTarget::StartArriving,
    }
}

/// Every chain verb and every arc-spec mode, once each, with the two
/// target forms and both spec positions represented — one entry per
/// `ProgramStep` chain variant, plus one `ArcTo` per arc mode.
///
/// **Nothing in this function forces the VERB side**: it is a `Vec`,
/// and a variant added to `ProgramStep` will not break it. What forces
/// the corpus to grow is `Verb::ALL` in the census below, which goes
/// red when a table verb is unreachable from here. The per-variant
/// spelling is for reading, not for enforcement.
///
/// The MODE side is forced, and differently: the `ArcTo` block and
/// both single-spec fused blocks are generated from `ArcMode::ALL`
/// through [`mode_witness`], so every mode rides the wire round-trip
/// and the slot bijection below in both spec positions — the twins a
/// mode addresses in the arrival position are not the ones it
/// addresses as a fused incoming — without anyone remembering to add
/// it.
///
/// `ArcFilletArc` is generated from `ArcMode::ALL` too, at the SAME
/// mode in both positions: that is the pair whose two specs compete
/// for one role, so it is the pair the bijection census has to walk.
/// One cross-mode pair is written out beside it, because a generated
/// same-mode sweep says nothing about a step whose two specs differ.
fn chain_steps() -> Vec<ProgramStep> {
    let mut steps = vec![
        ProgramStep::At(pt(0.0, 0.0)),
        ProgramStep::Angle(ang(0.25)),
        ProgramStep::Toward {
            dx: sca(1.0),
            dy: sca(0.5),
        },
        ProgramStep::Tangent,
        ProgramStep::Cusp,
        ProgramStep::Turn(ang(0.1)),
        ProgramStep::Line(len(1.0)),
        ProgramStep::ContinueTo(point(2.0, 0.0)),
    ];
    // Every target form, once each, on the verb that takes one. This
    // is the FORM side of the corpus and it is generated from
    // `TargetKind::ALL`, so a form the vocabulary gains rides the wire
    // round-trip and the slot bijection below without anyone
    // remembering to add it — exactly what the `ArcMode::ALL` blocks
    // do for the modes.
    steps.extend(
        TargetKind::ALL
            .iter()
            .map(|kind| ProgramStep::LineTo(target_witness(*kind))),
    );
    steps.extend(
        ArcMode::ALL
            .iter()
            .map(|mode| ProgramStep::ArcTo(mode_witness(*mode))),
    );
    steps.extend([
        ProgramStep::TangentArcTo(ProgramTarget::Start),
        ProgramStep::ArcContinue(pt(3.0, 1.0)),
        ProgramStep::Fillet(len(0.2)),
    ]);
    // Every mode in the ARRIVAL (spec₂) position, then every mode in
    // the fused INCOMING position: the role twins each mode addresses
    // differ between the two, so a mode walked in one is not walked in
    // the other.
    steps.extend(ArcMode::ALL.iter().map(|mode| ProgramStep::FilletArc {
        radius: len(0.3),
        spec: mode_witness(*mode),
    }));
    steps.extend(ArcMode::ALL.iter().map(|mode| ProgramStep::ArcFillet {
        spec: mode_witness(*mode),
        radius: len(0.4),
    }));
    // Both specs of a fused step, at the same mode: the position where
    // the incoming and the arrival roles are drawn from one spec
    // vocabulary and each has to address its own argument.
    steps.extend(ArcMode::ALL.iter().map(|mode| ProgramStep::ArcFilletArc {
        spec: mode_witness(*mode),
        radius: len(0.5),
        spec2: mode_witness(*mode),
    }));
    steps.extend([
        ProgramStep::ArcFilletArc {
            spec: ProgramArcData::Sweep {
                r: len(1.5),
                side: profile::ArcSide::Left,
                angle: ang(0.6),
            },
            radius: len(0.5),
            spec2: ProgramArcData::Radius {
                r: len(1.25),
                side: profile::ArcSide::Right,
            },
        },
        ProgramStep::FarEndTo(pt(7.0, 2.0)),
        ProgramStep::CloseTo,
    ]);
    steps
}

/// The corpus: the chain above plus the two complete-loop carrier
/// forms, which are `LoopProgram` variants rather than steps.
fn corpus() -> ProfileProgram {
    ProfileProgram {
        // The corpus is resolved and serialized directly, never
        // inserted, so the frame it names is scaffolding: no row here
        // reads what the plane denotes.
        plane: SCAFFOLD_PLANE,
        loops: vec![
            LoopProgram::Chain(chain_steps()),
            LoopProgram::circle(1.0, 1.0, 0.5).unwrap(),
            LoopProgram::circle_split(2.0, 2.0, 0.75, 5, 0.2).unwrap(),
        ],
    }
}

/// The leading identifier of a `Debug` rendering — the variant name.
/// `ProgramStep`'s chain variants and `Verb`'s are named identically
/// because the transition table names both, so comparing the two
/// strings compares the authored verb against the lifted one.
fn variant_name(debug: &str) -> String {
    debug
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect()
}

/// **The census.** Every verb the transition table declares is
/// reachable as a document program and resolves back to ITS OWN verb —
/// so a table verb that never reached `ProgramStep`, and a lifting arm
/// that launders one verb into another, both go red here.
///
/// Two clauses, because the set alone is not enough: a subset check
/// stays green when two arms SWAP their verbs, since the set of verbs
/// seen is still complete. The position-by-position clause is what
/// catches the swap, and it is the one that makes the laundering
/// promise above true.
#[test]
fn every_table_verb_is_a_document_program() {
    let authored = chain_steps();
    let resolved = corpus()
        .resolve(&ParamEnv::<f64>::default())
        .expect("the corpus resolves at f64");

    let chain: Vec<Verb> = resolved[0].iter().map(profile::Step::verb).collect();
    assert_eq!(
        chain.len(),
        authored.len(),
        "the chain loop lifted {} steps from {} authored ones",
        chain.len(),
        authored.len()
    );
    for (step, verb) in authored.iter().zip(chain.iter()) {
        let from = variant_name(&format!("{step:?}"));
        let to = variant_name(&format!("{verb:?}"));
        assert_eq!(to, from, "ProgramStep::{from} lifted to Verb::{to}");
    }

    let seen: Vec<Verb> = resolved
        .iter()
        .flat_map(|loop_| loop_.iter().map(profile::Step::verb))
        .collect();
    let missing: Vec<&Verb> = Verb::ALL.iter().filter(|v| !seen.contains(v)).collect();
    assert!(
        missing.is_empty(),
        "the document step vocabulary is short of the transition table: {missing:?} — \
         spell them in `ProgramStep`. There is no exception list: the one entry this \
         census ever carried (`ContinueTo`, waiting on a format change) landed with the \
         seam's declared arrival, and an empty escape hatch is a hatch that will be used"
    );
}

/// **The TARGET census**, on the mode census's model and for its
/// reasons verbatim one level down.
///
/// `res_target` matches the document target vocabulary and CONSTRUCTS
/// the kernel one, so an arm that builds a NEIGHBOUR's form is
/// well-typed, ships, and silently re-authors the seam — a `Start` that
/// resolved to `StartArriving` would close a loop declaring something
/// the author never wrote. Comparing the resolved form against the form
/// asked for is what catches that, and it is exactly what the mode
/// census does for `ArcData`.
///
/// The anchor is `profile::TargetKind::ALL`, projected from the same
/// declaration as `Target`'s variants — the KERNEL vocabulary, not the
/// document one. That is the direction that matters: a form `Target`
/// gains is exactly what this hop can drop, and a roster written
/// against `ProgramTarget` would still be complete, and green, while
/// the kernel grew past it. [`target_witness`] is a match on the tag,
/// so such a form stops this file COMPILING rather than quietly not
/// being censused.
///
/// The corpus clause is the mode census's second half for the same
/// reason: the wire round-trip and the slot bijection walk `corpus()`,
/// and neither says anything about a form the corpus omits.
///
/// **Blind spot, stated:** a form added to `ProgramTarget` and to no
/// other vocabulary is not witnessed here — `res_target` would have to
/// resolve it into one of the kernel forms, and this census would stay
/// green. It is the mode census's blind spot too, one vocabulary over
/// (`work/wire/document-only-vocabulary-blind-spot.md`), and closing it
/// needs a document-side tag, which is a third spelling of the form
/// set.
#[test]
fn every_target_form_is_a_document_program() {
    for kind in TargetKind::ALL {
        let program = ProfileProgram {
            plane: SCAFFOLD_PLANE,
            loops: vec![LoopProgram::Chain(vec![ProgramStep::LineTo(
                target_witness(*kind),
            )])],
        };
        let resolved = program
            .resolve(&ParamEnv::<f64>::default())
            .expect("a one-step target witness resolves at f64");
        let profile::Step::LineTo(got) = &resolved[0][0] else {
            panic!("the witness for {kind:?} lifted to something other than a straight leg");
        };
        assert_eq!(
            got.kind(),
            *kind,
            "the document target for {kind:?} resolved to {:?}",
            got.kind()
        );
    }

    // The corpus clause: which forms the generated corpus actually
    // carries, so a form present in the vocabulary and absent from the
    // corpus is visible rather than assumed. A target rides four verbs
    // directly and one more inside every endpoint-bearing arc mode, so
    // the fused arc verbs are walked here too — named rather than swept
    // into a trailing arm, because which verbs can carry a target is
    // what this clause assumes, and a verb that gains one is
    // adjudicated here.
    let seen: Vec<TargetKind> = corpus()
        .resolve(&ParamEnv::<f64>::default())
        .expect("the corpus resolves at f64")
        .iter()
        .flat_map(|loop_| loop_.iter())
        .flat_map(|step| match step {
            profile::Step::LineTo(t)
            | profile::Step::ContinueTo(t)
            | profile::Step::TangentArcTo(t) => vec![t.kind()],
            profile::Step::ArcTo(spec)
            | profile::Step::FilletArc { spec, .. }
            | profile::Step::ArcFillet { spec, .. } => spec
                .target()
                .map(profile::Target::kind)
                .into_iter()
                .collect(),
            profile::Step::ArcFilletArc { spec, spec2, .. } => [spec, spec2]
                .into_iter()
                .filter_map(|s| s.target())
                .map(profile::Target::kind)
                .collect(),
            profile::Step::At(_)
            | profile::Step::Angle(_)
            | profile::Step::Toward { .. }
            | profile::Step::Tangent
            | profile::Step::Cusp
            | profile::Step::Turn(_)
            | profile::Step::Line(_)
            | profile::Step::ArcContinue(_)
            | profile::Step::Fillet { .. }
            | profile::Step::FarEndTo(_)
            | profile::Step::CloseTo
            | profile::Step::Circle { .. }
            | profile::Step::CircleSplit { .. } => vec![],
        })
        .collect();
    let missing: Vec<&TargetKind> = TargetKind::ALL
        .iter()
        .filter(|k| !seen.contains(k))
        .collect();
    assert!(
        missing.is_empty(),
        "the shared corpus reaches no target in these forms, so the wire and slot \
         censuses say nothing about them: {missing:?}"
    );
}

/// **The mode census.** Every arc mode the kernel vocabulary declares
/// is spellable as a document spec and resolves back to ITS OWN mode.
///
/// The two failures it separates are the two the verb census
/// separates one level up. A mode that never reached `ProgramArcData`
/// cannot compile [`mode_witness`], so that half is settled before
/// this test runs; what runs here is the other half — `res_spec`
/// matches the document vocabulary and CONSTRUCTS the kernel one, so
/// an arm that builds a NEIGHBOUR's mode is well-typed, ships, and
/// silently re-authors the arc. Comparing the resolved mode against
/// the mode asked for is what catches that.
///
/// The second clause is why the corpus is generated: the wire
/// round-trip and the slot bijection below walk `corpus()`, and
/// neither says anything about a mode the corpus omits.
#[test]
fn every_arc_mode_is_a_document_program() {
    for mode in ArcMode::ALL {
        let program = ProfileProgram {
            plane: SCAFFOLD_PLANE,
            loops: vec![LoopProgram::Chain(vec![ProgramStep::ArcTo(mode_witness(
                *mode,
            ))])],
        };
        let resolved = program
            .resolve(&ParamEnv::<f64>::default())
            .expect("a one-step mode witness resolves at f64");
        let profile::Step::ArcTo(spec) = &resolved[0][0] else {
            panic!("the witness for {mode:?} lifted to something other than an arc leg");
        };
        assert_eq!(
            spec.mode(),
            *mode,
            "the document spec for {mode:?} resolved to a different mode"
        );
    }

    let corpus_modes: Vec<ArcMode> = corpus()
        .resolve(&ParamEnv::<f64>::default())
        .expect("the corpus resolves at f64")
        .iter()
        .flat_map(|loop_| loop_.iter())
        .flat_map(|step| match step {
            profile::Step::ArcTo(spec)
            | profile::Step::FilletArc { spec, .. }
            | profile::Step::ArcFillet { spec, .. } => vec![spec.mode()],
            profile::Step::ArcFilletArc { spec, spec2, .. } => vec![spec.mode(), spec2.mode()],
            // Named rather than swept into a trailing arm: which verbs
            // carry an arc spec is what this clause assumes, so a verb
            // that gains one is adjudicated here.
            profile::Step::At(_)
            | profile::Step::Angle(_)
            | profile::Step::Toward { .. }
            | profile::Step::Tangent
            | profile::Step::Cusp
            | profile::Step::Turn(_)
            | profile::Step::Line(_)
            | profile::Step::LineTo(_)
            | profile::Step::ContinueTo(_)
            | profile::Step::TangentArcTo(_)
            | profile::Step::ArcContinue(_)
            | profile::Step::Fillet { .. }
            | profile::Step::FarEndTo(_)
            | profile::Step::CloseTo
            | profile::Step::Circle { .. }
            | profile::Step::CircleSplit { .. } => vec![],
        })
        .collect();
    let missing: Vec<&ArcMode> = ArcMode::ALL
        .iter()
        .filter(|m| !corpus_modes.contains(m))
        .collect();
    assert!(
        missing.is_empty(),
        "the shared corpus reaches no arc leg in these modes, so the wire and slot \
         censuses say nothing about them: {missing:?}"
    );
}

/// The persisted vocabulary is the document vocabulary: every verb and
/// every arc-spec mode in the corpus survives serialization unchanged.
/// `ProfileProgram`'s `PartialEq` is the D7 bit comparator, so this is
/// bit-identity, not approximate agreement.
#[test]
fn every_document_verb_survives_the_wire() {
    let before = corpus();
    let text = serde_json::to_string(&before).expect("the program serializes");
    let after: ProfileProgram = serde_json::from_str(&text).expect("the program deserializes");
    assert_eq!(before, after);
}

/// The expressions of a program, counted from the wire rather than
/// from a number written here: every expression this suite builds is a
/// bare literal, so the `Literal` tags in its serialization ARE its
/// expressions. It holds for the one-step programs below for the same
/// reason it holds for the corpus: each is one of the corpus's own
/// chain steps, so the property is inherited rather than re-argued.
fn literal_count(program: &ProfileProgram) -> usize {
    serde_json::to_string(program)
        .expect("the program serializes")
        .matches("\"Literal\"")
        .count()
}

/// Every chain step that enumerates a number of slots other than its
/// own expression count, rendered with both counts.
///
/// The census's count clause compares WHOLE-PROGRAM totals, so a verb
/// or arc mode whose slot enumeration is short reads as a bare delta
/// naming neither the step nor the vocabulary member: an arc mode
/// whose `spec_slots` arm enumerates nothing fails that clause as two
/// numbers fifteen apart. Re-running the same comparison one step at a
/// time names the step. It runs only on the failure path.
///
/// **It closes one of those two and not the other, and says so rather
/// than letting the sentence above read as both.** The label is the
/// step's VERB, because `variant_name` takes the leading identifier —
/// and `ArcTo` appears once per `ArcMode::ALL` entry in this corpus, so
/// a short arc mode still leaves a reader counting `chain_steps()` to
/// learn WHICH mode's arm is short. Naming the vocabulary member is the
/// rest of the distance.
///
/// Two further limits, stated rather than discovered: this walks
/// `chain_steps()` while the clause it explains asserts over
/// `corpus()`, so the two agree only because `corpus()`'s first loop is
/// `Chain(chain_steps())` — a second chain loop would make the indices
/// name the wrong step — and it prints no `loop_` though a
/// `SlotId::Profile` carries one.
fn steps_whose_slot_count_disagrees() -> Vec<String> {
    chain_steps()
        .into_iter()
        .enumerate()
        .map(|(i, step)| {
            let one = ProfileProgram {
                plane: SCAFFOLD_PLANE,
                loops: vec![LoopProgram::Chain(vec![step.clone()])],
            };
            (i, step, one.slots().len(), literal_count(&one))
        })
        .filter(|(_, _, slots, exprs)| slots != exprs)
        .map(|(i, step, slots, exprs)| {
            // The step INDEX, because that is what a `SlotId::Profile`
            // carries and what the addressing clause below prints; the
            // verb name for reading, with the caveat in the header. The
            // payload is deliberately not rendered — an `Expr`'s
            // `Debug` is several lines and there are as many of them
            // here as the step has arguments.
            let name = variant_name(&format!("{step:?}"));
            format!("chain step {i} ({name}) has {exprs} expressions and enumerates {slots} slots")
        })
        .collect()
}

/// Slot addressing is a BIJECTION onto the program's expressions:
/// every slot addresses one, no two address the same one, and there
/// are exactly as many slots as expressions. Each clause catches a
/// different silence — `step_expr`'s table ends in a catch-all `None`
/// (a role that enumerates but does not address), the fused arms fall
/// back from one spec to the other (two roles collapsing onto one
/// argument), and `spec_slots` could simply stop enumerating a role
/// (an expression no slot reaches, which neither of the other two
/// clauses can see).
///
/// The count comes from the wire rather than from a number written
/// here: every expression in the corpus is a bare literal, so the
/// `Literal` tags in its serialization ARE its expressions.
///
/// Blind spot, stated: this walks the corpus, so it says nothing about
/// step shapes the corpus omits. The one it deliberately omits is a
/// fused step whose two specs are the same `Sweep`/`ArcLen`/`Bulge`
/// mode — unreachable from every recording surface, representable by
/// hand, and aliasing today (issue #829).
#[test]
fn every_enumerated_slot_addresses_a_distinct_expression() {
    let program = corpus();
    let slots = program.slots();
    let expressions = literal_count(&program);
    assert_eq!(
        slots.len(),
        expressions,
        "the program has {expressions} expressions and enumerates {} slots; \
         the chain steps whose own counts disagree are {mismatched:?} \
         (empty means the disagreement is in a complete-loop carrier rather \
         than a chain step)",
        slots.len(),
        mismatched = steps_whose_slot_count_disagrees(),
    );
    let mut addresses: Vec<*const Expr> = Vec::new();
    for slot in &slots {
        let Some(expr) = program.expr(*slot) else {
            panic!("{slot:?} is enumerated but addresses nothing");
        };
        let addr: *const Expr = expr;
        assert!(
            !addresses.contains(&addr),
            "{slot:?} addresses an expression another slot already addresses"
        );
        addresses.push(addr);
        let SlotId::Profile { arg, .. } = slot else {
            panic!("a profile payload enumerated a non-profile slot: {slot:?}");
        };
        assert_eq!(
            expr.dim(),
            arg.dimension(),
            "{slot:?} addresses an expression of the wrong dimension"
        );
    }
}
