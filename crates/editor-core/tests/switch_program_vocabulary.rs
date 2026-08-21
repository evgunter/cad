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
//! - `eval::feed_step` and `LoopProgram::from_recorded` are exhaustive
//!   on `profile::Step`, so a verb the table gains breaks `editor-core`
//!   at compile — measured: one added table verb, and exactly those two
//!   sites.
//!
//! The hop the compiler does NOT check is the one that CONSTRUCTS.
//! `res_step` matches `ProgramStep` and builds a `Step`, so both
//! compile errors above can be discharged without the document
//! vocabulary ever learning the verb — a refusal arm in
//! `from_recorded`, a tag in `feed_step`, and the wire and the
//! expression-slot vocabularies are quietly short. This suite is that
//! hop's census, anchored on `profile::Verb::ALL`: the same anchor
//! `profile`'s own replay-coverage census uses, read from the same
//! declaration.
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
use profile::{SketchPlane, Verb};

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

/// Every chain verb and every arc-spec mode, once each, with the two
/// target forms and both spec positions represented — one entry per
/// `ProgramStep` chain variant.
///
/// **Nothing in this function forces that**: it is a `Vec`, and a
/// variant added to `ProgramStep` will not break it. What forces the
/// corpus to grow is `Verb::ALL` in the census below, which goes red
/// when a table verb is unreachable from here. The per-variant
/// spelling is for reading, not for enforcement, and this comment says
/// so rather than claiming a construction guarantee the code does not
/// have.
fn chain_steps() -> Vec<ProgramStep> {
    vec![
        ProgramStep::At(pt(0.0, 0.0)),
        ProgramStep::Angle(ang(0.25)),
        ProgramStep::Toward {
            dx: sca(1.0),
            dy: sca(0.5),
        },
        ProgramStep::Tangent,
        ProgramStep::Turn(ang(0.1)),
        ProgramStep::Line(len(1.0)),
        ProgramStep::LineTo(point(1.0, 0.0)),
        ProgramStep::ArcTo(ProgramArcData::Radius {
            r: len(2.0),
            side: profile::ArcSide::Left,
        }),
        ProgramStep::ArcTo(ProgramArcData::Bulge {
            target: point(2.0, 1.0),
            b: sca(0.3),
        }),
        ProgramStep::ArcTo(ProgramArcData::ArcLen {
            r: len(2.5),
            side: profile::ArcSide::Right,
            len: len(0.7),
        }),
        ProgramStep::TangentArcTo(ProgramTarget::Start),
        ProgramStep::ArcContinue(pt(3.0, 1.0)),
        ProgramStep::Fillet(len(0.2)),
        ProgramStep::FilletArc {
            radius: len(0.3),
            spec: ProgramArcData::Via {
                q: pt(4.0, 1.0),
                target: point(5.0, 1.0),
            },
        },
        ProgramStep::ArcFillet {
            spec: ProgramArcData::Center {
                c: pt(6.0, 1.0),
                winding: profile::ArcSweep::Ccw,
                target: ProgramTarget::Start,
            },
            radius: len(0.4),
        },
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
    ]
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
        .resolve(&ParamEnv::default())
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
        "the document step vocabulary is short of the transition table: {missing:?}"
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
