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
//! The corpus below is deliberately NOT a legal lattice walk. Nothing
//! here replays: resolution, persistence and slot addressing are all
//! total over the data type, and legality is `profile`'s census to
//! keep.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{
    Dimension, Expr, LoopProgram, ParamEnv, ProfilePayload, ProfileProgram, ProgramArcData,
    ProgramStep, ProgramTarget, SlotId,
};
use profile::{ArcMode, SketchPlane, Verb};

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
/// The witnesses spread the two target forms across the modes that
/// take one, so the corpus reaches both without a second walk.
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
/// The gap that remains, stated: `ArcFilletArc` is hand-written and
/// walked at ONE mode pair, because it is the step whose two specs can
/// address the same role twice (issue #829), so generating its pairs
/// would author the aliasing case rather than test around it.
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
        ProgramStep::LineTo(point(1.0, 0.0)),
        ProgramStep::ContinueTo(point(2.0, 0.0)),
        // The seam's two declared arrivals, so the corpus the wire
        // round-trip and the slot bijection walk carries them.
        ProgramStep::ContinueTo(ProgramTarget::StartArriving),
    ];
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
        plane: SketchPlane::xy(),
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
/// the author never wrote. Comparing the resolved form
/// against the form asked for is what catches that, and it is exactly
/// what the mode census does for `ArcData`.
///
/// The roster is a MATCH rather than a list, so a target form the
/// kernel gains stops this file compiling instead of quietly not being
/// censused. The corpus clause is the mode census's second half for the
/// same reason: the wire round-trip and the slot bijection walk
/// `corpus()`, and neither says anything about a form the corpus omits.
#[test]
fn every_target_form_is_a_document_program() {
    /// Every document target, with the kernel form it must resolve to.
    /// Exhaustive by construction: adding a `ProgramTarget` variant
    /// without a witness here fails to compile at the match below.
    fn witnesses() -> Vec<(ProgramTarget, &'static str)> {
        let all = [
            ProgramTarget::Point([
                Expr::literal(1.0, Dimension::Length).unwrap(),
                Expr::literal(2.0, Dimension::Length).unwrap(),
            ]),
            ProgramTarget::Start,
            ProgramTarget::StartArriving,
        ];
        all.into_iter()
            .map(|t| {
                let name = match &t {
                    ProgramTarget::Point(_) => "Point",
                    ProgramTarget::Start => "Start",
                    ProgramTarget::StartArriving => "Declared",
                };
                (t, name)
            })
            .collect()
    }

    for (target, name) in witnesses() {
        let program = ProfileProgram {
            plane: SketchPlane::xy(),
            loops: vec![LoopProgram::Chain(vec![ProgramStep::LineTo(target)])],
        };
        let resolved = program
            .resolve(&ParamEnv::<f64>::default())
            .expect("a one-step target witness resolves at f64");
        let profile::Step::LineTo(got) = &resolved[0][0] else {
            panic!("the witness for {name} lifted to something other than a straight leg");
        };
        let got_name = match got {
            profile::Target::Point(_) => "Point",
            profile::Target::Start => "Start",
            profile::Target::StartArriving => "Declared",
        };
        assert_eq!(
            got_name, name,
            "the document target {name} resolved to {got_name}"
        );
    }

    // The corpus clause: which forms the generated corpus actually
    // carries, so a form present in the vocabulary and absent from the
    // corpus is visible rather than assumed.
    let seen: Vec<&'static str> = corpus()
        .resolve(&ParamEnv::<f64>::default())
        .expect("the corpus resolves at f64")
        .iter()
        .flat_map(|loop_| loop_.iter())
        .filter_map(|step| match step {
            profile::Step::LineTo(t)
            | profile::Step::ContinueTo(t)
            | profile::Step::TangentArcTo(t) => Some(t),
            profile::Step::ArcTo(spec) => spec.target(),
            _ => None,
        })
        .map(|t| match t {
            profile::Target::Point(_) => "Point",
            profile::Target::Start => "Start",
            profile::Target::StartArriving => "Declared",
        })
        .collect();
    for form in ["Point", "Start", "Declared"] {
        assert!(
            seen.contains(&form),
            "the corpus carries no {form} target: {seen:?}"
        );
    }
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
            plane: SketchPlane::xy(),
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
    let expressions = serde_json::to_string(&program)
        .expect("the program serializes")
        .matches("\"Literal\"")
        .count();
    assert_eq!(
        slots.len(),
        expressions,
        "the program has {expressions} expressions and enumerates {} slots",
        slots.len()
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
