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
//! # The mirror direction, and where its anchor comes from
//!
//! Every census named so far is anchored on the KERNEL vocabulary, and
//! that is the right anchor for the hop they exist to guard: the thing
//! that CONSTRUCTS is `editor-core`'s, so a kernel form the document
//! vocabulary never learned is what goes missing. The mirror failure
//! is a variant added to `ProgramArcData` or `ProgramTarget` alone.
//! The compiler forces such a variant through every match that
//! consumes it, so it cannot ship unnoticed — but every one of those
//! arms may legally resolve it into an EXISTING kernel form, and when
//! one does, every kernel-anchored clause here stays green while the
//! document form authors something nobody wrote.
//!
//! The two document enums have no `ALL` to key on, because neither is
//! projected from a declaration the way `profile`'s three vocabularies
//! are — and minting them one is a third spelling of each set, a
//! design call about where the anchor belongs rather than a test
//! change. So the anchor is the DECLARATION ITSELF, read as text:
//! `declared_variants` is the document vocabularies' `ALL`, and the
//! two censuses over it are bijections against the witnesses this
//! suite builds, not floors.
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
use test_utils::source;

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

/// An arc spec's MODE, and the target form riding inside it where the
/// mode carries one.
///
/// `variant_name` over the `Debug` rendering is how a document spec's
/// mode is read, because [`ProgramArcData`] has no tag door of its own
/// — that absence is the blind spot the two document censuses below
/// close, and this is the second place it costs something.
fn spec_label(spec: &ProgramArcData) -> String {
    let mode = variant_name(&format!("{spec:?}"));
    match spec {
        ProgramArcData::Bulge { target, .. }
        | ProgramArcData::Via { target, .. }
        | ProgramArcData::Center { target, .. } => {
            format!("{mode}/{}", variant_name(&format!("{target:?}")))
        }
        ProgramArcData::Radius { .. }
        | ProgramArcData::Sweep { .. }
        | ProgramArcData::ArcLen { .. } => mode,
    }
}

/// **The one "name the offender" door.** A step's verb, plus every
/// vocabulary member it carries.
///
/// `variant_name` alone answers the VERB and stops there, and in this
/// corpus a verb is not an identity: `ArcTo` appears once per
/// `ArcMode::ALL` entry and `LineTo` once per `TargetKind::ALL` entry,
/// so *"chain step 16 (ArcTo)"* leaves a reader counting
/// [`chain_steps`] to learn which member's arm is the short one. Every
/// clause below that localises a failure to a step says it through
/// here, so the three censuses name an offender the same way.
///
/// The match is exhaustive on [`ProgramStep`] rather than swept into a
/// trailing arm, for this file's standing reason: which verbs carry a
/// member is what the label claims, so a verb that gains one is
/// adjudicated here.
fn step_label(step: &ProgramStep) -> String {
    let verb = variant_name(&format!("{step:?}"));
    let members: Vec<String> = match step {
        ProgramStep::ArcTo(spec)
        | ProgramStep::FilletArc { spec, .. }
        | ProgramStep::ArcFillet { spec, .. } => vec![spec_label(spec)],
        ProgramStep::ArcFilletArc { spec, spec2, .. } => {
            vec![spec_label(spec), spec_label(spec2)]
        }
        ProgramStep::LineTo(target)
        | ProgramStep::ContinueTo(target)
        | ProgramStep::TangentArcTo(target) => {
            vec![variant_name(&format!("{target:?}"))]
        }
        ProgramStep::At(_)
        | ProgramStep::Angle(_)
        | ProgramStep::Toward { .. }
        | ProgramStep::Tangent
        | ProgramStep::Cusp
        | ProgramStep::Turn(_)
        | ProgramStep::Line(_)
        | ProgramStep::ArcContinue(_)
        | ProgramStep::Fillet(_)
        | ProgramStep::FarEndTo(_)
        | ProgramStep::CloseTo => vec![],
    };
    if members.is_empty() {
        verb
    } else {
        format!("{verb}({})", members.join(", "))
    }
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
    for (i, (step, verb)) in authored.iter().zip(chain.iter()).enumerate() {
        let from = variant_name(&format!("{step:?}"));
        let to = variant_name(&format!("{verb:?}"));
        assert_eq!(
            to,
            from,
            "chain step {i} ({}) lifted to Verb::{to}",
            step_label(step)
        );
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
/// **The mirror direction is not this census's**, and is not
/// uncovered: a form added to `ProgramTarget` and to no other
/// vocabulary is invisible here — `res_target` resolves it into one of
/// the kernel forms and `TargetKind::ALL` stays fully witnessed — so
/// [`every_document_target_is_witnessed`] anchors on the document
/// declaration instead. The same pair holds for the modes.
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
///
/// The anchor is the KERNEL vocabulary and only it, which is the right
/// anchor for the failure above and says nothing about a mode the
/// DOCUMENT vocabulary gains alone.
/// [`every_document_arc_spec_is_witnessed`] is that direction.
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

/// This crate's declaration of the document vocabularies, as text.
const PROGRAM_RS: &str = include_str!("../src/program.rs");

/// The variant names an enum DECLARES, read out of [`PROGRAM_RS`].
///
/// **This is the document vocabularies' `ALL`.** `profile` projects
/// `ArcMode::ALL` and `TargetKind::ALL` from the same declaration as
/// the variants, so a census keyed on either grows with the vocabulary
/// rather than behind it; `ProgramArcData` and `ProgramTarget` are
/// plain enums with no such projection, and minting them one is a
/// THIRD spelling of each vocabulary in a crate that already spells
/// them twice by G1 layering — a design call about where the anchor
/// belongs, not a test change (`work/wire/document-only-vocabulary-blind-spot.md`).
///
/// Reading the declaration is the same anchor without the third
/// spelling: the set still has exactly one home, and it is still the
/// declaration itself rather than a roster kept in step by hand. What
/// it costs is the lexer's own limit — an enum whose variants are
/// produced by a macro is invisible to any textual walk, so if either
/// of these two ever gains one, this door is what has to change. Both
/// are written out today, and a variant added by hand is exactly the
/// arrival this census exists to detect.
fn declared_variants(enum_name: &str) -> Vec<String> {
    let code = source::blanked(source::code_only, "editor-core/src/program.rs", PROGRAM_RS);
    let head = format!("pub enum {enum_name} {{");
    assert_eq!(
        code.matches(&head).count(),
        1,
        "`{head}` appears {} times in editor-core/src/program.rs, not once — the declaration \
         moved, was renamed, or grew a generic parameter, and this census is reading nothing",
        code.matches(&head).count()
    );
    let at = code.find(&head).expect("the declaration was just counted");
    let source::ItemBody::Body(body) = source::item_body(&code, at) else {
        panic!("`{enum_name}` declares a body");
    };
    let inside = &code[body.start + 1..body.end - 1];
    source::top_level_split(inside, ',')
        .into_iter()
        .map(|r| variant_name(inside[r].trim_start()))
        .filter(|name| !name.is_empty())
        .collect()
}

/// Arc specs the DOCUMENT vocabulary declares and the kernel mode
/// vocabulary does not, each beside the kernel mode its resolution is
/// declared to produce.
///
/// **Empty today, and that is a statement rather than a gap**:
/// `ProgramArcData`'s variants are one per `ArcMode` entry, so
/// [`mode_witness`] already witnesses every one of them. A variant
/// added to `ProgramArcData` alone reds the census below until it is
/// given a line here, and the line has to name the kernel mode
/// `res_spec` launders it into — which is the whole finding: such a
/// variant resolving into an EXISTING mode is legal, ships, and
/// silently authors something nobody wrote, and this is where it stops
/// being silent.
fn document_only_arc_specs() -> Vec<(ProgramArcData, ArcMode)> {
    vec![]
}

/// Targets the DOCUMENT vocabulary declares and `profile::TargetKind`
/// does not, each beside the kernel form its resolution is declared to
/// produce. [`document_only_arc_specs`] one vocabulary over, for its
/// reasons verbatim.
fn document_only_targets() -> Vec<(ProgramTarget, TargetKind)> {
    vec![]
}

/// **The mode census's mirror.** Every arc spec the DOCUMENT
/// vocabulary declares is witnessed by this suite, and resolves to the
/// kernel mode it is declared to resolve to.
///
/// [`every_arc_mode_is_a_document_program`] is anchored on
/// `ArcMode::ALL` and that anchor is deliberate: a mode the KERNEL
/// gains is exactly what the construct hop can drop. It says nothing
/// in the other direction. A variant added to `ProgramArcData` is
/// forced through every match that consumes it — `res_spec`,
/// the wire conversions, `spec_lit`, `spec_slots`, the content-key
/// hashers — but **every one of those arms may legally resolve it into
/// an existing kernel mode**, and when it does the kernel-anchored
/// census stays green, the corpus clauses stay green, and the document
/// form authors an arc the author did not write. [`mode_witness`] is a
/// match on the KERNEL tag, so nothing forces such a variant to
/// acquire a witness at all.
///
/// It is a BIJECTION and not a floor: the set the declaration spells
/// and the set this suite witnesses are the same set, so neither an
/// unwitnessed variant nor a witness for a variant that no longer
/// exists can pass. A count would have allowed both.
#[test]
fn every_document_arc_spec_is_witnessed() {
    let declared = declared_variants("ProgramArcData");
    // A set equality holds vacuously between two empty sets, and both
    // sides here derive from one scan of one file, so a scan that read
    // the wrong file would report agreement. The declared set is
    // asserted non-empty on its own before it is compared.
    assert!(
        !declared.is_empty(),
        "`ProgramArcData` is declared with no variants at all — the read of \
         editor-core/src/program.rs found a body and nothing in it, so this census \
         is comparing the witnesses against an empty set"
    );

    let mut witnessed: Vec<String> = ArcMode::ALL
        .iter()
        .map(|mode| variant_name(&format!("{:?}", mode_witness(*mode))))
        .collect();
    for (spec, resolves_to) in document_only_arc_specs() {
        let program = ProfileProgram {
            plane: SCAFFOLD_PLANE,
            loops: vec![LoopProgram::Chain(vec![ProgramStep::ArcTo(spec.clone())])],
        };
        let resolved = program
            .resolve(&ParamEnv::<f64>::default())
            .expect("a one-step document-only arc witness resolves at f64");
        let profile::Step::ArcTo(got) = &resolved[0][0] else {
            panic!(
                "the document-only witness {} lifted to {}, not an arc leg",
                spec_label(&spec),
                variant_name(&format!("{:?}", resolved[0][0]))
            );
        };
        assert_eq!(
            got.mode(),
            resolves_to,
            "the document-only spec {} is declared to resolve to {resolves_to:?} and \
             resolved to {:?}",
            spec_label(&spec),
            got.mode()
        );
        witnessed.push(variant_name(&format!("{spec:?}")));
    }

    let unwitnessed: Vec<&String> = declared.iter().filter(|d| !witnessed.contains(d)).collect();
    let unknown: Vec<&String> = witnessed.iter().filter(|w| !declared.contains(w)).collect();
    assert!(
        unwitnessed.is_empty() && unknown.is_empty(),
        "the arc specs this suite witnesses are not the arc specs `ProgramArcData` \
         declares.\n  declared and unwitnessed (give each a `document_only_arc_specs` \
         line naming the kernel mode it resolves to): {unwitnessed:?}\n  \
         witnessed and no longer declared (delete the witness): {unknown:?}"
    );
}

/// **The target census's mirror**, on the census above's model and for
/// its reasons verbatim one vocabulary over.
///
/// `res_target` is the arm that can launder a document-only form into
/// an existing kernel one — a new `ProgramTarget` variant resolving to
/// `Start` would close a loop declaring something the author never
/// wrote, with `TargetKind::ALL` still fully witnessed and every clause
/// in this file green.
#[test]
fn every_document_target_is_witnessed() {
    let declared = declared_variants("ProgramTarget");
    assert!(
        !declared.is_empty(),
        "`ProgramTarget` is declared with no variants at all — the read of \
         editor-core/src/program.rs found a body and nothing in it, so this census \
         is comparing the witnesses against an empty set"
    );

    let mut witnessed: Vec<String> = TargetKind::ALL
        .iter()
        .map(|kind| variant_name(&format!("{:?}", target_witness(*kind))))
        .collect();
    for (target, resolves_to) in document_only_targets() {
        let program = ProfileProgram {
            plane: SCAFFOLD_PLANE,
            loops: vec![LoopProgram::Chain(vec![ProgramStep::LineTo(
                target.clone(),
            )])],
        };
        let resolved = program
            .resolve(&ParamEnv::<f64>::default())
            .expect("a one-step document-only target witness resolves at f64");
        let name = variant_name(&format!("{target:?}"));
        let profile::Step::LineTo(got) = &resolved[0][0] else {
            panic!(
                "the document-only witness {name} lifted to {}, not a straight leg",
                variant_name(&format!("{:?}", resolved[0][0]))
            );
        };
        assert_eq!(
            got.kind(),
            resolves_to,
            "the document-only target {name} is declared to resolve to \
             {resolves_to:?} and resolved to {:?}",
            got.kind()
        );
        witnessed.push(name);
    }

    let unwitnessed: Vec<&String> = declared.iter().filter(|d| !witnessed.contains(d)).collect();
    let unknown: Vec<&String> = witnessed.iter().filter(|w| !declared.contains(w)).collect();
    assert!(
        unwitnessed.is_empty() && unknown.is_empty(),
        "the targets this suite witnesses are not the targets `ProgramTarget` \
         declares.\n  declared and unwitnessed (give each a `document_only_targets` \
         line naming the kernel form it resolves to): {unwitnessed:?}\n  \
         witnessed and no longer declared (delete the witness): {unknown:?}"
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
    let differences = wire_differences(&before, &after);
    assert!(
        before == after,
        "the corpus did not survive serialization. {}",
        if differences.is_empty() {
            "The loop-by-loop walk names no difference, so the two programs differ \
             somewhere it does not reach — widen `wire_differences` rather than \
             reading past this."
                .to_string()
        } else {
            format!("Changed: {differences:#?}")
        }
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
/// whose `spec_slots` arm enumerates nothing fails that clause as two
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
                        format!("loop {l} chain step {i} ({})", step_label(step)),
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
