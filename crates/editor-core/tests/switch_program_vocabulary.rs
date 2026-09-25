//! **The profile step vocabulary, across the crate boundary
//! (LIB-SWITCH §4; the S4 "one vocabulary, N hand-synced copies"
//! shape).**
//!
//! `profile`'s `transition_table!` declares each authoring verb once
//! and projects four artifacts from that declaration — but all four
//! are INSIDE `profile`. `editor-core` re-spells the same vocabulary
//! ONCE more, because `profile` has neither expressions nor serde and
//! by G1 layering must not gain them: `ProgramStep`, the Expr-valued
//! document form, which is also the PERSISTED form — it derives serde
//! where it is declared, so there is no third spelling to keep in step
//! and no mapping between two of them to get wrong.
//!
//! The kernel→document hop needs no test, because the compiler already
//! refuses it: `eval::feed_step`, `eval::feed_lane_step` and
//! `LoopProgram::from_recorded` are exhaustive on `profile::Step`, so
//! a verb the table gains breaks `editor-core` at compile — measured:
//! one added table verb, and exactly those THREE sites.
//! `feed_lane_step` (M10-P) is the lift's second key feed and joined
//! the list when it landed; it is named here rather than left to be
//! rediscovered, since the whole point of this list is that it is the
//! set a reader can trust to be complete.
//!
//! The hop the compiler does NOT check is the one that CONSTRUCTS.
//! `res_step` matches `ProgramStep` and builds a `Step`, so the
//! compile errors above can be discharged without the document
//! vocabulary ever learning the verb — a refusal arm in
//! `from_recorded`, a tag in `feed_step` and one in `feed_lane_step`,
//! and the document and expression-slot vocabularies are quietly
//! short. This suite is that hop's census, anchored on
//! `profile::Verb::ALL`: the same anchor `profile`'s own
//! replay-coverage census uses, read from the same declaration.
//!
//! # The spelling, now that the document form is the format
//!
//! A verb added to `ProgramStep` still breaks the compile at three
//! sites in `program.rs` — `loop_roles`, `res_step` and `step_bit_eq`
//! (measured: one added document verb, exactly those three) — so it
//! cannot arrive unnoticed, and what carries it into the corpus, the
//! round trip and the pin below is the `ALL_NAMES` census rather than
//! any of them.
//!
//! What no match anywhere reports is a verb RENAMED. Every census here
//! compares one projection of a declaration against another projection
//! of the same declaration, so a rename moves both sides at once; and
//! since the document form is the serde type, that rename is a change
//! to the persisted format. `PERSISTED_SPELLING` at the foot of this
//! file is the literal pin that a rename cannot move with itself.
//!
//! # The arc modes, one level down
//!
//! The same two spellings carry a second vocabulary INSIDE the
//! steps — the §2c arc modes — and a verb-keyed census is blind to
//! it: every mode travels inside `ArcTo` and the three fused verbs,
//! so the verb census above is green whatever the modes do.
//!
//! The hops are the same ones, one level down. `program::spec_lit`
//! and both content-key hashers are exhaustive on `profile::ArcData`,
//! so a mode the kernel gains breaks this crate at compile — and, as
//! above, each break can be discharged where it stands while
//! `res_spec` keeps constructing and the document and slot
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
//! # The mirror direction, and where its anchor comes from
//!
//! Every census named so far is anchored on the KERNEL vocabulary, and
//! that is the right anchor for the hop they exist to guard: the thing
//! that CONSTRUCTS is `editor-core`'s, so a kernel form the document
//! vocabulary never learned is what goes missing.
//!
//! The mirror failure is a variant added to a DOCUMENT enum alone, and
//! **every document vocabulary has it — one per construct hop.**
//! `LoopProgram` at `LoopProgram::resolve`, `ProgramStep` at
//! `res_step`, `ProgramArcData` at `res_spec`, `ProgramTarget` at
//! `res_target`: each matches the document form and BUILDS the kernel
//! one. The compiler forces a new variant through every match that
//! consumes it, so it cannot ship unnoticed; but every one of those
//! arms may legally resolve it into an EXISTING kernel form, and when
//! one does, every kernel-anchored clause here stays green while the
//! document form authors something nobody wrote. The verb side is no
//! safer than the rest: `chain_steps()` is a `Vec` and forces no verb,
//! which its own doc says.
//!
//! The anchor is `ALL_NAMES`, projected from each enum's declaration by
//! `program.rs`'s `document_vocabulary!` exactly as `profile` projects
//! `Verb::ALL`, `ArcMode::ALL` and `TargetKind::ALL` from theirs — a
//! compile-time constant over the declaring tokens, so nothing in front
//! of a variant's name can hide it. The census over it is a set
//! equality against what the CORPUS carries: not a floor, and not
//! against a witness function, so a declared member is a member the
//! wire round-trip and the slot bijection actually walk. WHICH
//! vocabularies get censused is projected from the same invocation
//! (`program::DOCUMENT_VOCABULARIES`), so a new one arrives in the
//! census rather than waiting for a call site to be written.
//!
//! The corpus below is deliberately NOT a legal lattice walk. Nothing
//! here replays: resolution, persistence and slot addressing are all
//! total over the data type, and legality is `profile`'s census to
//! keep.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;

use editor_core::{
    Dimension, Expr, LoopProgram, ParamEnv, ParamName, ProfilePayload, ProfileProgram,
    ProgramArcData, ProgramStep, ProgramTarget, SlotId, StepArg,
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
/// by exhaustiveness — `spec_roles`' rows, `spec_bit_eq`, and the
/// kernel construction in `res_spec`. The wire needs no arm: the
/// document type is the serde type.
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
        // The travel sense the mode witnesses do not reach. Every
        // other structural tag on this wire rides a generated block,
        // but `mode_witness` has one `Center` and so one winding, and
        // a tag the corpus never carries is a tag the persisted-
        // spelling pin below says nothing about.
        ProgramStep::ArcTo(ProgramArcData::Center {
            c: pt(6.0, 3.0),
            winding: profile::ArcSweep::Ccw,
            target: point(7.0, 3.0),
        }),
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

/// The vocabulary members one step carries, kept in the shape the
/// message wants: the verb, each arc spec on the step paired with the
/// target form riding inside it, and each target the step carries
/// directly.
///
/// **One descent, three readers.** [`step_label`] formats it for a
/// failure message and the three document censuses flatten it into
/// their vocabularies' witness sets, so which member rides which verb
/// is decided in exactly one place. Re-deriving it per reader is the
/// second-corpus-kept-in-step-by-hand defect this file exists to catch.
struct StepMembers {
    /// The step's own [`ProgramStep`] variant name.
    verb: String,
    /// One entry per arc spec on the step: its [`ProgramArcData`]
    /// variant name, and the [`ProgramTarget`] variant name inside it
    /// where the mode carries a target.
    specs: Vec<(String, Option<String>)>,
    /// [`ProgramTarget`] variant names the step carries itself.
    targets: Vec<String>,
}

/// The [`ProgramTarget`] a spec carries, where its mode carries one.
///
/// Exhaustive rather than swept into a trailing arm, for this file's
/// standing reason: which modes carry a target is what the census's
/// target side assumes, so a mode that gains one is adjudicated here.
fn spec_target(spec: &ProgramArcData) -> Option<&ProgramTarget> {
    match spec {
        ProgramArcData::Bulge { target, .. }
        | ProgramArcData::Via { target, .. }
        | ProgramArcData::Center { target, .. } => Some(target),
        ProgramArcData::Radius { .. }
        | ProgramArcData::Sweep { .. }
        | ProgramArcData::ArcLen { .. } => None,
    }
}

/// The one descent over a step. Exhaustive on [`ProgramStep`] for
/// [`spec_target`]'s reason: which verbs carry a spec or a target is
/// what every reader below assumes.
///
/// A variant name comes from the `Debug` rendering because no document
/// enum carries a tag door of its own; what forces the SET to be
/// complete is `ALL_NAMES`, not this.
fn step_members(step: &ProgramStep) -> StepMembers {
    let spec_entry = |spec: &ProgramArcData| {
        (
            variant_name(&format!("{spec:?}")),
            spec_target(spec).map(|t| variant_name(&format!("{t:?}"))),
        )
    };
    let (specs, targets) = match step {
        ProgramStep::ArcTo(spec)
        | ProgramStep::FilletArc { spec, .. }
        | ProgramStep::ArcFillet { spec, .. } => (vec![spec_entry(spec)], vec![]),
        ProgramStep::ArcFilletArc { spec, spec2, .. } => {
            (vec![spec_entry(spec), spec_entry(spec2)], vec![])
        }
        ProgramStep::LineTo(target)
        | ProgramStep::ContinueTo(target)
        | ProgramStep::TangentArcTo(target) => (vec![], vec![variant_name(&format!("{target:?}"))]),
        ProgramStep::At(_)
        | ProgramStep::Angle(_)
        | ProgramStep::Toward { .. }
        | ProgramStep::Tangent
        | ProgramStep::Cusp
        | ProgramStep::Turn(_)
        | ProgramStep::Line(_)
        | ProgramStep::Fillet(_)
        | ProgramStep::FarEndTo(_)
        | ProgramStep::CloseTo => (vec![], vec![]),
    };
    StepMembers {
        verb: variant_name(&format!("{step:?}")),
        specs,
        targets,
    }
}

/// **The one "name the offender" door.** A step's verb, plus every
/// vocabulary member it carries.
///
/// `variant_name` alone answers the VERB and stops there, and in this
/// corpus a verb is not an identity: `ArcTo` appears once per
/// `ArcMode::ALL` entry and `LineTo` once per `TargetKind::ALL` entry,
/// so *"chain step 16 (ArcTo)"* leaves a reader counting
/// [`chain_steps`] to learn which member's arm is the short one.
fn step_label(step: &ProgramStep) -> String {
    let m = step_members(step);
    let members: Vec<String> = m
        .specs
        .iter()
        .map(|(mode, target)| match target {
            Some(t) => format!("{mode}/{t}"),
            None => mode.clone(),
        })
        .chain(m.targets.iter().cloned())
        .collect();
    if members.is_empty() {
        m.verb
    } else {
        format!("{}({})", m.verb, members.join(", "))
    }
}

/// **The one position label**, so the verb clause and the slot
/// localiser below name a chain step the same way — `loop_` and `step`
/// being exactly what a `SlotId::Profile` carries.
fn position_label(loop_: usize, step: usize, what: &ProgramStep) -> String {
    format!("loop {loop_} chain step {step} ({})", step_label(what))
}

/// A loop's own label: the chain's length, or the carrier's form.
fn loop_label(loop_: &LoopProgram) -> String {
    match loop_ {
        LoopProgram::Chain(steps) => format!("Chain of {} steps", steps.len()),
        LoopProgram::Circle { .. } | LoopProgram::CircleSplit { .. } => {
            variant_name(&format!("{loop_:?}"))
        }
    }
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
    // The authored side is read OUT of the program that is resolved,
    // never rebuilt beside it: `chain_steps()` called a second time is
    // a second corpus kept in step by hand, and it lines up with the
    // resolved loops only while `corpus()`'s first loop happens to be
    // the chain. Walking every chain loop drops that assumption too.
    let program = corpus();
    let resolved = program
        .resolve(&ParamEnv::<f64>::default())
        .expect("the corpus resolves at f64");

    let mut chains = 0usize;
    for (l, loop_) in program.loops.iter().enumerate() {
        let LoopProgram::Chain(authored) = loop_ else {
            continue;
        };
        chains += 1;
        let lifted: Vec<Verb> = resolved[l].iter().map(profile::Step::verb).collect();
        assert_eq!(
            lifted.len(),
            authored.len(),
            "loop {l} lifted {} steps from {} authored ones",
            lifted.len(),
            authored.len()
        );
        for (i, (step, verb)) in authored.iter().zip(lifted.iter()).enumerate() {
            let from = variant_name(&format!("{step:?}"));
            let to = variant_name(&format!("{verb:?}"));
            assert_eq!(
                to,
                from,
                "{} lifted to Verb::{to}",
                position_label(l, i, step)
            );
        }
    }
    assert!(
        chains > 0,
        "the corpus holds {} loop(s) and not one of them is a chain, so the \
         position-by-position clause above walked nothing",
        program.loops.len()
    );

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
/// **The mirror direction is not this census's**, and is not
/// uncovered: a form added to `ProgramTarget` and to no other
/// vocabulary is invisible here — `res_target` resolves it into one of
/// the kernel forms and `TargetKind::ALL` stays fully witnessed — so
/// [`every_document_vocabulary_member_is_witnessed`] anchors on
/// `ProgramTarget::ALL_NAMES` instead. Every document vocabulary has
/// that pair, one per construct hop.
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
            panic!(
                "the witness for {kind:?} lifted to {}, not a straight leg",
                variant_name(&format!("{:?}", resolved[0][0]))
            );
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
///
/// The anchor is the KERNEL vocabulary and only it, which is the right
/// anchor for the failure above and says nothing about a mode the
/// DOCUMENT vocabulary gains alone.
/// [`every_document_vocabulary_member_is_witnessed`] is that
/// direction, keyed on `ProgramArcData::ALL_NAMES`.
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
            panic!(
                "the witness for {mode:?} lifted to {}, not an arc leg",
                variant_name(&format!("{:?}", resolved[0][0]))
            );
        };
        assert_eq!(
            spec.mode(),
            *mode,
            "the document spec for {mode:?} resolved to {:?}",
            spec.mode()
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

/// Where a round trip changed the program, named one step at a time.
///
/// Zips the loops and, inside a chain, the steps, so a laundered
/// vocabulary member is reported as the step it rode rather than as
/// the corpus it sat in. It runs only on the failure path.
///
/// **It reads the two programs handed to it**, never [`corpus`]: a
/// localiser that rebuilds its own subject is a second corpus kept in
/// step by hand, which is the defect this whole file exists to catch.
///
/// Returns every difference it can name, and an empty vector where the
/// two programs differ somewhere this walk does not reach — the caller
/// says so rather than letting silence read as agreement.
fn wire_difference_report(before: &ProfileProgram, after: &ProfileProgram) -> String {
    let differences = wire_differences(before, after);
    if differences.is_empty() {
        "The loop-by-loop walk names no difference, so the two programs differ \
         somewhere it does not reach — widen `wire_differences` rather than reading \
         past this."
            .to_string()
    } else {
        format!("Changed: {differences:#?}")
    }
}

fn wire_differences(before: &ProfileProgram, after: &ProfileProgram) -> Vec<String> {
    let mut out = Vec::new();
    if before.plane != after.plane {
        out.push(format!(
            "the plane went over as {:?} and came back as {:?}",
            before.plane, after.plane
        ));
    }
    if before.loops.len() != after.loops.len() {
        out.push(format!(
            "the program went over with {} loops and came back with {}",
            before.loops.len(),
            after.loops.len()
        ));
    }
    for (l, (was, now)) in before.loops.iter().zip(after.loops.iter()).enumerate() {
        if was == now {
            continue;
        }
        match (was, now) {
            (LoopProgram::Chain(was_steps), LoopProgram::Chain(now_steps)) => {
                if was_steps.len() != now_steps.len() {
                    out.push(format!(
                        "loop {l} went over with {} chain steps and came back with {}",
                        was_steps.len(),
                        now_steps.len()
                    ));
                }
                for (i, (was_step, now_step)) in was_steps.iter().zip(now_steps.iter()).enumerate()
                {
                    if was_step != now_step {
                        out.push(format!(
                            "loop {l} chain step {i}: {} went over the wire and came back as {}",
                            step_label(was_step),
                            step_label(now_step)
                        ));
                    }
                }
            }
            // Not a chain on one side or the other: the carrier forms
            // have no step index to name, so the loop's own form is
            // the finest thing there is to say.
            _ => out.push(format!(
                "loop {l}: {} went over the wire and came back as {}",
                loop_label(was),
                loop_label(now)
            )),
        }
    }
    out
}

// ------------------------------------------------------------------
// The mirror direction: the DOCUMENT vocabularies' own anchor
// ------------------------------------------------------------------

/// Every document vocabulary member the CORPUS carries, by vocabulary:
/// the loop forms, the verbs, the arc-spec modes, the target forms.
///
/// The corpus is what the wire round-trip and the slot bijection below
/// walk, so a member witnessed here is a member those clauses cover —
/// which is why this, and not a witness function, is the set the
/// censuses compare against. A member reachable only from a witness the
/// corpus omits would be declared and uncovered, and the two clauses
/// that matter would still say nothing about it.
fn corpus_vocabulary() -> CorpusVocabulary {
    let mut seen = CorpusVocabulary::default();
    for loop_ in &corpus().loops {
        // The loop forms are a vocabulary in their own right, and
        // `LoopProgram::resolve` is their construct hop: it matches
        // this enum and builds `Step::Circle` / `Step::CircleSplit`, so
        // a carrier form added here alone can be resolved into an
        // existing kernel step exactly as a `ProgramStep` variant can.
        // Declining to witness them, which this walk used to do, left
        // the fourth hop uncovered while the census disclosed its shape
        // hypothetically.
        seen.loops.push(variant_name(&format!("{loop_:?}")));
        let LoopProgram::Chain(steps) = loop_ else {
            // A carrier is a one-step program with no `ProgramStep` in
            // it, so it contributes to no other vocabulary here.
            continue;
        };
        for step in steps {
            let m = step_members(step);
            seen.verbs.push(m.verb);
            seen.targets.extend(m.targets);
            for (mode, target) in m.specs {
                seen.specs.push(mode);
                seen.targets.extend(target);
            }
        }
    }
    seen
}

/// The witness sets, one per document vocabulary.
#[derive(Default)]
struct CorpusVocabulary {
    /// [`LoopProgram`] variant names.
    loops: Vec<String>,
    /// [`ProgramStep`] variant names.
    verbs: Vec<String>,
    /// [`ProgramArcData`] variant names.
    specs: Vec<String>,
    /// [`ProgramTarget`] variant names.
    targets: Vec<String>,
}

/// One document vocabulary's complaint, or `None` where it is whole.
///
/// The caller is the census; this only answers for one vocabulary, so
/// that the caller can walk every one of them and name all the short
/// ones rather than aborting on the first. That matters here more than
/// most places: a run naming one of four offenders is the defect this
/// whole file was opened to fix.
fn unwitnessed_report(vocabulary: &str, declared: &[&str], witnessed: &[String]) -> Option<String> {
    let witnessed: Vec<&str> = witnessed.iter().map(String::as_str).collect();
    test_utils::census::set_difference(
        declared,
        &witnessed,
        &format!("`{vocabulary}` declares members the corpus does not witness"),
        "witnessed and no longer declared — delete the witness",
        "declared and unwitnessed — give it a witness in `chain_steps` or `corpus`. A \
         variant with no kernel form of its own launders into one, which is the failure \
         this clause exists to catch, so it has to be exercised rather than excused",
    )
}

/// **The document vocabularies' census**, and the mirror of every
/// kernel-anchored clause above.
///
/// # Why this direction needs its own anchor
///
/// Every other census in this file is anchored on the KERNEL
/// vocabulary, which is the right anchor for the hop they guard: the
/// thing that CONSTRUCTS is `editor-core`'s, so a kernel form the
/// document vocabulary never learned is what goes missing. The mirror
/// failure is a variant added to a DOCUMENT enum alone. The compiler
/// forces it through every match that consumes it, but **every one of
/// those arms may legally resolve it into an existing kernel form**,
/// and when one does, every kernel-anchored clause here stays green
/// while the document form authors something nobody wrote.
///
/// There are four such construct hops and this covers all four:
/// `res_step`, `res_spec`, `res_target`, and `LoopProgram::resolve`.
///
/// # Two bijections, one per level
///
/// `ALL_NAMES` answers which VARIANTS a vocabulary declares;
/// `DOCUMENT_VOCABULARIES` answers which VOCABULARIES exist. Both are
/// projected from the one macro invocation that declares the enums, so
/// neither a variant nor a whole vocabulary can arrive without a
/// census. Naming the vocabularies at call sites here would have been a
/// hand-kept roster over a projected set — this file's own defect, one
/// level up from where it catches it.
///
/// The witness SETS are the part that cannot be projected: only this
/// suite knows which walk of `corpus()` answers for which vocabulary.
/// So they are bijected too, and a vocabulary with no witness set reds
/// with instructions rather than going uncensused.
///
/// # Why the equality is safe, and where there is no exception list
///
/// A set equality passes when both sides are empty; here that is closed
/// by construction, since `ALL_NAMES` is a compile-time constant over a
/// declaration that has variants, so a witness walk collapsing to
/// nothing reds. And a document-only variant gets no allow-list line
/// naming what it launders into: it reds until it has a WITNESS in the
/// corpus, which is also what makes the wire round-trip and the slot
/// bijection cover it. The verb census's own message settles the shape
/// — *"an empty escape hatch is a hatch that will be used"*.
#[test]
fn every_document_vocabulary_member_is_witnessed() {
    let seen = corpus_vocabulary();
    let witnesses: Vec<(&str, &Vec<String>)> = vec![
        ("LoopProgram", &seen.loops),
        ("ProgramStep", &seen.verbs),
        ("ProgramArcData", &seen.specs),
        ("ProgramTarget", &seen.targets),
    ];

    let declared: Vec<&str> = editor_core::program::DOCUMENT_VOCABULARIES
        .iter()
        .map(|(name, _)| *name)
        .collect();
    let supplied: Vec<&str> = witnesses.iter().map(|(name, _)| *name).collect();
    if let Some(report) = test_utils::census::set_difference(
        &declared,
        &supplied,
        "the vocabularies `document_vocabulary!` declares are not the vocabularies this \
         suite can witness",
        "supplied a witness set and no longer declared — delete the line",
        "declared and uncensused — add the walk of `corpus()` that answers for it to \
         `corpus_vocabulary` and its line to `witnesses` above; until then nothing in \
         this file says anything about it",
    ) {
        panic!("{report}");
    }

    let complaints: Vec<String> = editor_core::program::DOCUMENT_VOCABULARIES
        .iter()
        .filter_map(|(vocabulary, names)| {
            let (_, witnessed) = witnesses
                .iter()
                .find(|(name, _)| name == vocabulary)
                .expect("the set equality above admits only vocabularies with a witness set");
            unwitnessed_report(vocabulary, names, witnessed)
        })
        .collect();
    assert!(
        complaints.is_empty(),
        "{} of the {} document vocabularies are short:\n  {}",
        complaints.len(),
        editor_core::program::DOCUMENT_VOCABULARIES.len(),
        complaints.join("\n  ")
    );
}

/// The persisted vocabulary is the document vocabulary: every verb and
/// every arc-spec mode in the corpus survives serialization unchanged.
/// `ProfileProgram`'s `PartialEq` is the D7 bit comparator, so this is
/// bit-identity, not approximate agreement.
///
/// The comparison is `assert!` over `==` rather than `assert_eq!`, and
/// deliberately: `assert_eq!` renders BOTH operands, and this corpus's
/// `Debug` is three loops and forty-odd chain steps of `Expr` records
/// twice over, which is the reader diffing a page by eye for a
/// laundering that moved one step. [`wire_differences`] names the step.
#[test]
fn every_document_verb_survives_the_wire() {
    let before = corpus();
    // A round trip over an EMPTY program is bit-identical whatever the
    // wire does with the vocabulary, and so is a round trip over one
    // whose expressions all vanished on the way out. The subject is
    // asserted non-empty before the comparison that would otherwise
    // pass over nothing.
    let literals = literal_count(&before);
    assert!(
        !before.loops.is_empty() && literals > 0,
        "the corpus offered to the wire is {} loop(s) carrying {literals} literal \
         expression(s) — an empty subject round-trips identically for the wrong reason",
        before.loops.len()
    );
    let text = serde_json::to_string(&before).expect("the program serializes");
    let after: ProfileProgram = serde_json::from_str(&text).expect("the program deserializes");
    assert!(
        before == after,
        "the corpus did not survive serialization. {}",
        // Evaluated only here, on the failure path: an `assert!`'s
        // format arguments are untouched while the condition holds.
        wire_difference_report(&before, &after)
    );
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

/// Every position in a program that enumerates a number of slots other
/// than its own expression count, rendered with both counts.
///
/// The census's count clause compares WHOLE-PROGRAM totals, so a verb
/// or arc mode whose slot enumeration is short reads as a bare delta
/// naming neither the position nor the vocabulary member: an arc mode
/// whose `spec_roles` arm enumerates nothing fails that clause as two
/// numbers fifteen apart. Re-running the same comparison one position
/// at a time names the position, and [`step_label`] names the member
/// riding it — `ArcTo` alone would not, since this corpus carries one
/// per `ArcMode::ALL` entry.
///
/// **It walks the program it is given**, so the indices it prints are
/// the indices of the program the clause asserts over — a `loop_` and
/// a `step`, which is exactly what a `SlotId::Profile` carries.
/// Re-deriving the subject from [`chain_steps`] instead made the two
/// agree only because `corpus()`'s first loop happens to be
/// `Chain(chain_steps())`; a second chain loop, or a reorder, and the
/// localiser named the wrong step confidently.
///
/// The complete-loop carriers are walked too, each as its own one-loop
/// program, so a carrier whose enumeration is short is named here
/// rather than left as an empty list for the reader to interpret.
fn positions_whose_slot_count_disagrees(program: &ProfileProgram) -> Vec<String> {
    let mut out = Vec::new();
    for (l, loop_) in program.loops.iter().enumerate() {
        // One (label, one-loop program) per position: a chain's steps
        // individually, a carrier whole — a carrier is a one-step
        // program by construction, so it has no finer position.
        let positions: Vec<(String, LoopProgram)> = match loop_ {
            LoopProgram::Chain(steps) => steps
                .iter()
                .enumerate()
                .map(|(i, step)| {
                    (
                        position_label(l, i, step),
                        LoopProgram::Chain(vec![step.clone()]),
                    )
                })
                .collect(),
            LoopProgram::Circle { .. } | LoopProgram::CircleSplit { .. } => {
                vec![(format!("loop {l} ({})", loop_label(loop_)), loop_.clone())]
            }
        };
        for (label, one) in positions {
            // The payload is deliberately not rendered — an `Expr`'s
            // `Debug` is several lines and there are as many of them
            // here as the position has arguments.
            let alone = ProfileProgram {
                plane: program.plane,
                loops: vec![one],
            };
            let (slots, exprs) = (alone.slots().len(), literal_count(&alone));
            if slots != exprs {
                out.push(format!(
                    "{label} has {exprs} expressions and enumerates {slots} slots"
                ));
            }
        }
    }
    out
}

/// Slot addressing is a BIJECTION onto the program's expressions:
/// every slot addresses one, no two address the same one, and there
/// are exactly as many slots as expressions. Enumeration and
/// addressing read one role table (`loop_roles` in `program.rs`), so
/// what each clause catches is a fault in that table: a row whose role
/// repeats another's — a fused step's arrival spec written at the
/// incoming spec's roles, say — addresses one expression twice and the
/// other never, and a row the table omits is an expression no slot
/// reaches, which only the count clause can see.
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
    // A bijection between two empty sets is a bijection. The corpus is
    // asserted non-empty on BOTH sides before the equality, so a
    // program that enumerated nothing, or one whose expressions all
    // vanished, reds here rather than passing as a clean bijection.
    assert!(
        expressions > 0 && !slots.is_empty(),
        "the corpus carries {expressions} expressions and enumerates {} slots — \
         a program with neither satisfies every clause below vacuously",
        slots.len()
    );
    assert_eq!(
        slots.len(),
        expressions,
        "the program has {expressions} expressions and enumerates {} slots; \
         the positions whose own counts disagree are {mismatched:#?}",
        slots.len(),
        mismatched = positions_whose_slot_count_disagrees(&program),
    );
    let mut addresses: Vec<(*const Expr, SlotId)> = Vec::new();
    for slot in &slots {
        let Some(expr) = program.expr(*slot) else {
            panic!("{slot:?} is enumerated but addresses nothing");
        };
        let addr: *const Expr = expr;
        if let Some((_, first)) = addresses.iter().find(|(seen, _)| *seen == addr) {
            panic!("{slot:?} addresses the expression {first:?} already addresses");
        }
        addresses.push((addr, *slot));
        let SlotId::Profile { arg, .. } = slot else {
            panic!("a profile payload enumerated a non-profile slot: {slot:?}");
        };
        assert_eq!(
            expr.dim(),
            arg.dimension(),
            "{slot:?} addresses an expression of dimension {:?}, and the role wants {:?}",
            expr.dim(),
            arg.dimension()
        );
    }
}

/// **A refusal reports at the slot the census enumerates.** The
/// enumeration and the resolution (`res_step` / `res_spec` /
/// `res_target`) both read the role table (`loop_roles` in
/// `program.rs`): the resolvers name no role themselves and reach the
/// evaluator through one leaf that tags a refusal with the role the
/// table pairs with the refusing expression. The bijection census above
/// reads only the enumeration's side. So each enumerated slot's
/// expression is replaced, one at a time, by a reference to a parameter
/// nothing binds, and the program's resolution must refuse AT that
/// slot — a resolver arm that named a role of its own, or tagged the
/// wrong step, would point the user at the wrong field here, and
/// nowhere else; and a row the table omits fails the leaf's lookup on
/// every resolution of that step shape, which reds this test too.
///
/// Blind spot, stated: the corpus's, as for the census above.
#[test]
fn every_enumerated_slot_is_where_its_refusal_reports() {
    let program = corpus();
    let slots = program.slots();
    assert!(!slots.is_empty(), "the corpus enumerates no slot");
    let unbound = ParamName::new("nothing binds this");
    let mut misplaced = Vec::new();
    for slot in &slots {
        let mut broken = program.clone();
        let expr = broken
            .expr_mut(*slot)
            .unwrap_or_else(|| panic!("{} is enumerated but addresses nothing", slot.label()));
        *expr = Expr::param(unbound.clone(), expr.dim());
        match broken.resolve(&ParamEnv::<f64>::default()) {
            Err((reported, _)) if reported == *slot => {}
            Err((reported, _)) => {
                misplaced.push(format!("{} refuses at {}", slot.label(), reported.label()))
            }
            Ok(_) => misplaced.push(format!(
                "{} resolved over an unbound parameter",
                slot.label()
            )),
        }
    }
    assert!(
        misplaced.is_empty(),
        "the resolver addresses these slots at a role the enumeration does not: {misplaced:#?}"
    );
}

/// The value a resolved step carries in the field `arg` NAMES — this
/// suite's own reading of each role, written from the kernel type and
/// the role's label and deliberately NOT from the program's role table,
/// so the two are independent spellings a test may compare.
///
/// A role on a step shape that has no such field answers `None`.
fn field_named(step: &profile::Step<f64>, arg: StepArg) -> Option<f64> {
    use StepArg as A;
    use profile::{ArcData as D, Step as S, Target};
    let target = |t: &Target<f64>, x: bool| match t {
        Target::Point(p) => Some(if x { p.x } else { p.y }),
        Target::Start | Target::StartArriving => None,
    };
    // A spec's field for an incoming-spec role.
    let spec = |d: Option<&D<f64>>, role: StepArg| match (d?, role) {
        (D::Radius { r, .. } | D::Sweep { r, .. } | D::ArcLen { r, .. }, A::CarrierRadius) => {
            Some(*r)
        }
        (D::Bulge { b, .. }, A::Bulge) => Some(*b),
        (D::Via { q, .. }, A::ViaX) => Some(q.x),
        (D::Via { q, .. }, A::ViaY) => Some(q.y),
        (D::Center { c, .. }, A::CenterX) => Some(c.x),
        (D::Center { c, .. }, A::CenterY) => Some(c.y),
        (D::Sweep { angle, .. }, A::SweepVal) => Some(*angle),
        (D::ArcLen { len, .. }, A::ArcLenVal) => Some(*len),
        (
            D::Bulge { target: t, .. } | D::Via { target: t, .. } | D::Center { target: t, .. },
            A::TargetX | A::TargetY,
        ) => target(t, role == A::TargetX),
        _ => None,
    };
    let (incoming, arrival) = match step {
        S::ArcTo(d) | S::ArcFillet { spec: d, .. } => (Some(d), None),
        S::FilletArc { spec: d, .. } => (None, Some(d)),
        S::ArcFilletArc { spec, spec2, .. } => (Some(spec), Some(spec2)),
        _ => (None, None),
    };
    match (step, arg) {
        (S::At(p) | S::FarEndTo(p), A::PointX) => Some(p.x),
        (S::At(p) | S::FarEndTo(p), A::PointY) => Some(p.y),
        (S::Angle(v), A::AngleVal) | (S::Turn(v), A::TurnVal) | (S::Line(v), A::Length) => Some(*v),
        (S::Toward { dx, .. }, A::DirX) => Some(*dx),
        (S::Toward { dy, .. }, A::DirY) => Some(*dy),
        (S::LineTo(t) | S::ContinueTo(t) | S::TangentArcTo(t), A::TargetX | A::TargetY) => {
            target(t, arg == A::TargetX)
        }
        (
            S::Fillet { radius }
            | S::FilletArc { radius, .. }
            | S::ArcFillet { radius, .. }
            | S::ArcFilletArc { radius, .. }
            | S::Circle { radius, .. }
            | S::CircleSplit { radius, .. },
            A::Radius,
        ) => Some(*radius),
        (S::Circle { centre, .. } | S::CircleSplit { centre, .. }, A::CenterX) => Some(centre.x),
        (S::Circle { centre, .. } | S::CircleSplit { centre, .. }, A::CenterY) => Some(centre.y),
        (S::CircleSplit { phase, .. }, A::Phase) => Some(*phase),
        (_, A::CarrierRadius2) => spec(arrival, A::CarrierRadius),
        (_, A::Bulge2) => spec(arrival, A::Bulge),
        (_, A::Via2X) => spec(arrival, A::ViaX),
        (_, A::Via2Y) => spec(arrival, A::ViaY),
        (_, A::Center2X) => spec(arrival, A::CenterX),
        (_, A::Center2Y) => spec(arrival, A::CenterY),
        (_, A::SweepVal2) => spec(arrival, A::SweepVal),
        (_, A::ArcLenVal2) => spec(arrival, A::ArcLenVal),
        (_, A::Target2X) => spec(arrival, A::TargetX),
        (_, A::Target2Y) => spec(arrival, A::TargetY),
        (_, role) => spec(incoming, role),
    }
}

/// **Every enumerated slot resolves into the field its role NAMES.**
///
/// The two censuses above check that enumeration, addressing and
/// resolution AGREE. All three read one role table (`loop_roles` in
/// `program.rs`), so a table row that pairs a role with the wrong
/// field — `ViaX` with the via's y — is read the same wrong way by every
/// consumer, agrees with itself, and moves y when a user edits "via x".
/// So each slot's expression is replaced, one at a time, by a sentinel
/// literal, the program is resolved, and the resolved kernel step must
/// carry the sentinel in the field [`field_named`] reads for that role.
///
/// Blind spot, stated: the corpus's, as for the censuses above.
#[test]
fn every_enumerated_slot_resolves_into_the_field_its_role_names() {
    let program = corpus();
    let slots = program.slots();
    assert!(!slots.is_empty(), "the corpus enumerates no slot");
    // Finite and valid in every dimension; no corpus literal is this.
    let sentinel = 7.123_456_789;
    let mut misread = Vec::new();
    for slot in &slots {
        let SlotId::Profile { loop_, step, arg } = *slot else {
            panic!("a profile payload enumerated a non-profile slot: {slot:?}");
        };
        let mut probe = program.clone();
        let expr = probe
            .expr_mut(*slot)
            .unwrap_or_else(|| panic!("{} is enumerated but addresses nothing", slot.label()));
        *expr = Expr::literal(sentinel, expr.dim()).expect("a finite literal");
        let loops = probe
            .resolve(&ParamEnv::<f64>::default())
            .unwrap_or_else(|(at, e)| {
                panic!("the literal corpus refuses at {}: {e:?}", at.label())
            });
        let resolved = &loops[loop_ as usize][step as usize];
        if field_named(resolved, arg) != Some(sentinel) {
            misread.push(format!("{} resolves into {resolved:?}", slot.label()));
        }
    }
    assert!(
        misread.is_empty(),
        "these slots resolve into a field other than the one their role names: {misread:#?}"
    );
}

// ------------------------------------------------------------------
// The persisted spelling
// ------------------------------------------------------------------

/// Every JSON object KEY and every JSON string the corpus's persisted
/// form carries, read from the bytes and not from the declarations
/// that produced them.
///
/// Externally-tagged enums put a variant's name in one of exactly
/// those two places — an object key for a variant with a payload, a
/// bare string for one without — and a struct variant's field names
/// are object keys beside it. So this set IS the persisted vocabulary
/// of everything the corpus reaches, with no per-variant walk to keep
/// in step with the enums.
fn persisted_tokens(program: &ProfileProgram) -> BTreeSet<String> {
    fn walk(v: &serde_json::Value, out: &mut BTreeSet<String>) {
        match v {
            serde_json::Value::Object(map) => {
                for (key, value) in map {
                    out.insert(key.clone());
                    walk(value, out);
                }
            }
            serde_json::Value::Array(items) => {
                for item in items {
                    walk(item, out);
                }
            }
            serde_json::Value::String(s) => {
                out.insert(s.clone());
            }
            serde_json::Value::Null | serde_json::Value::Bool(_) | serde_json::Value::Number(_) => {
            }
        }
    }
    let mut out = BTreeSet::new();
    walk(
        &serde_json::to_value(program).expect("the program serializes"),
        &mut out,
    );
    out
}

/// **The persisted spelling of the profile payload, pinned as
/// literals.**
///
/// Every other census in this file compares one projection of a
/// declaration against another projection of the same declaration, so
/// renaming a variant moves both sides together and nothing reds. That
/// is the right shape for a census of WHICH members a vocabulary has.
/// It is the wrong shape for their SPELLING, because the document
/// enums ARE the serde types: a renamed variant and a renamed field
/// are changes to the FORMAT, and the format is not derivable from the
/// declaration that changed with it.
///
/// So this list is written out, and that is the point of it — it is
/// the one thing on this wire that a rename cannot move with itself.
/// A red here is not repaired by copying the new tokens over: it says
/// a document the previous build saved no longer reads the same. The
/// repair is to decide the new spelling is right, regenerate the
/// checked-in corpus (`PNCAD_BLESS=1`, `lib_dietool_crossing`'s
/// header), and re-pin.
///
/// It covers what the corpus reaches, which is every member of all
/// four document vocabularies (the censuses above are what make that
/// true) plus the `Expr` records they carry.
///
/// **It is a SET, and that is its blind spot.** A swapped `Ccw`/`Cw`, a
/// `spec`/`spec2` exchanged between two fused verbs, a reordered
/// `Literal` record: each leaves this set identical while changing
/// where every word goes. `tests/wire_rv_bytes.rs` pins the
/// ARRANGEMENT byte for byte and is what kills those; this row is the
/// one that localises a rename to the word. Neither subsumes the
/// other, and a reader chasing a red uses which of the two fired to
/// tell a rename from a rearrangement.
const PERSISTED_SPELLING: &[&str] = &[
    // The `Expr` record and its closed tables: the dimensionless
    // literal's display symbol is the empty string.
    "",
    "Length",
    "Literal",
    "Scalar",
    "dim",
    "m",
    "rad",
    "unit",
    "value",
    // `ProfileProgram` and `LoopProgram`.
    "Chain",
    "Circle",
    "CircleSplit",
    "centre",
    "loops",
    "n",
    "phase",
    "plane",
    "radius",
    // `ProgramStep`, and the field names of the four that name theirs.
    "Angle",
    "ArcFillet",
    "ArcFilletArc",
    "ArcTo",
    "At",
    "CloseTo",
    "ContinueTo",
    "Cusp",
    "FarEndTo",
    "Fillet",
    "FilletArc",
    "Line",
    "LineTo",
    "Tangent",
    "TangentArcTo",
    "Toward",
    "Turn",
    "dx",
    "dy",
    "spec",
    "spec2",
    // `ProgramArcData` and its fields, then the two kernel-foreign
    // tags its fields carry (`profile::ArcSide`, `profile::ArcSweep`).
    "ArcLen",
    "Bulge",
    "Center",
    "Radius",
    "Sweep",
    "Via",
    "angle",
    "b",
    "c",
    "len",
    "q",
    "r",
    "side",
    "target",
    "winding",
    "Ccw",
    "Cw",
    "Left",
    "Right",
    // `ProgramTarget`. (`Angle` above is a step verb and a dimension
    // both; `Radius`/`Sweep` are arc modes and `Line`/`Fillet` verbs —
    // this is a set of TOKENS, not a table keyed by vocabulary.)
    "Point",
    "Start",
    "StartArriving",
];

#[test]
fn the_persisted_spelling_of_the_program_is_pinned() {
    let found = persisted_tokens(&corpus());
    let pinned: BTreeSet<String> = PERSISTED_SPELLING
        .iter()
        .map(|s| (*s).to_string())
        .collect();
    let added: Vec<&String> = found.difference(&pinned).collect();
    let gone: Vec<&String> = pinned.difference(&found).collect();
    assert!(
        added.is_empty() && gone.is_empty(),
        "the persisted spelling of the profile program moved. New on the wire: \
         {added:?}. Gone from the wire: {gone:?}. A document the previous build saved \
         no longer reads the same — decide whether the new spelling is right, then \
         regenerate the checked-in corpus (`PNCAD_BLESS=1 cargo test -p editor-core \
         --test all lib_dietool_crossing`, the same for `wire_rv_bytes`, and \
         `corpus/die_composed_tour.rs`'s own line for the tour) and re-pin here."
    );
}
