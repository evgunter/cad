//! **The v4 program KEY-SPACE pins (LIB-SWITCH §4e)** — the successor
//! of the retired `ProfileDesc::tokens` key pins (#101 MINOR-1/NOTE-1;
//! their subject, the stored token stream, died with the stored
//! segments): the profile content key now feeds from the program's
//! (tag, payload) stream — per-loop LoopStart tags, per resolved step
//! the verb tag + structural tags + resolved-f64 bits — so structure
//! can never alias float data, resolved values move the key, and
//! display units NEVER enter it (D7).
//!
//! The plane is NOT in that stream. It is a document node the profile
//! names, so it folds into the key as an upstream input key, the same
//! way every other input does; these rows build every document with
//! its frame first and read the profile second.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{ang, len, len2, scl, xy_frame};
use editor_core::{
    CancelToken, ContentKey, Dimension, DocEdit, DocParam, EvalOptions, Expr, LoopProgram, Node,
    ParamName, ProfileDoc, ProfileProgram, ProgramArcData, ProgramStep, ProgramTarget,
    RecipeNodeId, SlotId, StepArg, evaluate, parse_expr,
};
use geom_core::Tol;

/// The frame every document below is built on: the id its insert
/// mints, first among the minting edits of every document here, so one
/// id in all of them (D9).
fn plane() -> RecipeNodeId {
    crate::fixture::newest(&with_frame(ProfileDoc::empty_derived(
        "switch_program_key",
        Tol::witness(),
    )))
}

/// The profile drawn on that frame: the document's second node.
fn profile(doc: &ProfileDoc) -> RecipeNodeId {
    doc.order()[1]
}

fn key_of(doc: &ProfileDoc) -> ContentKey {
    let ev = evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    ev.value(profile(doc))
        .expect("profile evaluates")
        .content_key
}

fn doc_with(loops: Vec<LoopProgram>) -> ProfileDoc {
    let doc = ProfileDoc::empty_derived("switch_program_key", Tol::witness());
    with_frame(doc)
        .apply(
            &DocEdit::InsertNode {
                node: Node::Profile(ProfileProgram {
                    plane: plane(),
                    loops,
                    ids: Vec::new(),
                }),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .expect("valid program")
        .doc
}

/// The world xy frame, inserted first. Every row here is about the PROGRAM's
/// key, so every document shares one plane: a key difference between
/// two of these documents can only have come from their programs.
fn with_frame(doc: ProfileDoc) -> ProfileDoc {
    doc.apply(
        &DocEdit::InsertNode { node: xy_frame() },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("the frame inserts")
    .doc
}

/// `doc` with the one expression of its profile at argument `arg`
/// re-spelled as `expr`, through the value door: the steps keep the ids
/// the insert minted, so two documents compared here differ in that one
/// spelling and nothing else. (Two documents authored apart with
/// different spellings mint their steps from different edits, so their
/// ids — and so their keys — differ; a display unit alone is not a
/// different spelling, D6.)
fn respelled(doc: &ProfileDoc, arg: StepArg, expr: Expr) -> ProfileDoc {
    let slots: Vec<SlotId> = doc
        .node(profile(doc))
        .expect("the profile is the second node")
        .slots()
        .into_iter()
        .filter(|s| matches!(*s, SlotId::Profile { arg: a, .. } if a == arg))
        .collect();
    let [slot] = slots.as_slice() else {
        panic!("one {arg:?} slot, got {slots:?}");
    };
    doc.apply(
        &DocEdit::SetParam {
            node: profile(doc),
            slot: *slot,
            expr,
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("the re-spelling applies")
    .doc
}

/// Authored program ORDER is structure: the same hole wound the other
/// way is a different program, never a key alias (the retired
/// LoopStart token's aliasing job, now carried by the program shape
/// riding the tag stream).
#[test]
fn loop_structure_never_aliases_data() {
    let outer = || LoopProgram::polygon([(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)]).unwrap();
    let hole = LoopProgram::polygon([(1.0, 1.0), (1.0, 2.0), (2.0, 2.0), (2.0, 1.0)]).unwrap();
    let hole_ccw = LoopProgram::polygon([(1.0, 1.0), (2.0, 1.0), (2.0, 2.0), (1.0, 2.0)]).unwrap();
    let a = doc_with(vec![outer(), hole]);
    let b = doc_with(vec![outer(), hole_ccw]);
    assert_ne!(key_of(&a), key_of(&b));
}

/// Verb identity is structure: a `circle` program and a `circle_split`
/// program of the SAME carrier never share a key.
///
/// This is one pair, end to end through `evaluate`. That NO two verbs
/// share a tag is the stronger property and is computed over
/// `profile::Verb::ALL` by `eval::verb_tag_tests::verb_tags_are_injective`.
#[test]
fn verb_tags_are_structure() {
    let a = doc_with(vec![LoopProgram::circle(1.0, 1.0, 0.5).unwrap()]);
    let b = doc_with(vec![
        LoopProgram::circle_split(1.0, 1.0, 0.5, 2, 0.0).unwrap(),
    ]);
    assert_ne!(key_of(&a), key_of(&b));
}

/// The resolved-value convention, and **the one place it no longer
/// holds** (SEAT-7, key format v5).
///
/// A param edit that changes a resolved program value still moves the
/// key. What changed is the second half: a CARRIER LOOP's radius
/// re-spelled at the same value now keys DIFFERENTLY, because that
/// expression stopped being only a number. The sweeps declare the
/// profile edge's radius into the walls they mint, so an extrude of
/// this circle carries the lowered identity of THIS expression in its
/// cylinder's field source — and two spellings of one value are two
/// different bodies downstream. Keying them identically would let the
/// memo serve a body whose token names an expression the document no
/// longer holds, which is the stale-token class the blend's own
/// flow-bearing slot was fixed for at v4.
///
/// The convention itself is unchanged everywhere it still applies:
/// every OTHER program expression — centres, chain steps, phases — is
/// resolved-bits-only, and the row below pins one of them so the
/// exception is bounded rather than assumed.
#[test]
fn resolved_values_feed_the_key() {
    let with_param = |value: f64| {
        let doc = ProfileDoc::empty_derived("switch_program_key", Tol::witness());
        let doc = doc
            .apply(
                &DocEdit::SetDocParam {
                    name: ParamName::from_static("r"),
                    value: DocParam::continuous(Dimension::Length, value),
                },
                Tol::witness(),
                &editor_core::RefusingReach,
            )
            .unwrap()
            .doc;
        with_frame(doc)
            .apply(
                &DocEdit::InsertNode {
                    node: Node::Profile(ProfileProgram {
                        plane: plane(),
                        loops: vec![LoopProgram::Circle {
                            centre: [len(0.0), len(0.0)],
                            radius: Expr::param(ParamName::from_static("r"), Dimension::Length),
                        }],
                        ids: Vec::new(),
                    }),
                },
                Tol::witness(),
                &editor_core::RefusingReach,
            )
            .unwrap()
            .doc
    };
    let k_half = key_of(&with_param(0.5));
    let k_quarter = key_of(&with_param(0.25));
    assert_ne!(k_half, k_quarter, "a resolved-value change moves the key");
    let literal = doc_with(vec![LoopProgram::circle(0.0, 0.0, 0.5).unwrap()]);
    assert_ne!(
        k_half,
        key_of(&literal),
        "a carrier radius is flow-bearing: its SPELLING reaches the walls a \
         sweep mints, so two spellings of one value must not share a memo entry"
    );
}

/// **The exception is exactly one expression wide**: a carrier loop's
/// CENTRE, re-spelled at the same value, keys identically.
///
/// Nothing downstream carries a centre's identity — the walls store a
/// radius, and a placement is not a stored scalar — so the
/// resolved-value convention is untouched for it. Without this row the
/// row above would read as "spelling entered the key", which is not
/// what happened.
#[test]
fn a_carrier_centre_respelled_keys_identically() {
    let doc = ProfileDoc::empty_derived("switch_program_key", Tol::witness());
    let doc = doc
        .apply(
            &DocEdit::SetDocParam {
                name: ParamName::from_static("cx"),
                value: DocParam::continuous(Dimension::Length, 1.0),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .unwrap()
        .doc;
    let parameterized = with_frame(doc)
        .apply(
            &DocEdit::InsertNode {
                node: Node::Profile(ProfileProgram {
                    plane: plane(),
                    loops: vec![LoopProgram::Circle {
                        centre: [
                            Expr::param(ParamName::from_static("cx"), Dimension::Length),
                            len(0.0),
                        ],
                        radius: len(0.5),
                    }],
                    ids: Vec::new(),
                }),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .unwrap()
        .doc;
    let literal = respelled(&parameterized, StepArg::CenterX, len(1.0));
    assert_eq!(
        key_of(&parameterized),
        key_of(&literal),
        "a centre's spelling does not reach any stored field, so it must not enter the key"
    );
}

/// A chain whose one arc is drawn at `radius`, closed back to its
/// start: a straight leg, a tangent quarter-turn arc, and the closing
/// leg.
fn one_arc_chain(radius: Expr) -> LoopProgram {
    LoopProgram::Chain(vec![
        ProgramStep::At([len(0.0), len(0.0)]),
        ProgramStep::Toward {
            dx: scl(1.0),
            dy: scl(0.0),
        },
        ProgramStep::Line(len(4.0)),
        ProgramStep::Tangent,
        ProgramStep::ArcTo(ProgramArcData::Sweep {
            r: radius,
            side: profile::ArcSide::Left,
            angle: ang(core::f64::consts::FRAC_PI_2),
        }),
        ProgramStep::LineTo(ProgramTarget::Start),
    ])
}

/// A document declaring `r` at `value`, carrying one profile built from
/// `loops` — the same two nodes every row here uses.
fn doc_with_r(value: f64, loops: Vec<LoopProgram>) -> ProfileDoc {
    let doc = ProfileDoc::empty_derived("switch_program_key", Tol::witness())
        .apply(
            &DocEdit::SetDocParam {
                name: ParamName::from_static("r"),
                value: DocParam::continuous(Dimension::Length, value),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .unwrap()
        .doc;
    with_frame(doc)
        .apply(
            &DocEdit::InsertNode {
                node: Node::Profile(ProfileProgram {
                    plane: plane(),
                    loops,
                    ids: Vec::new(),
                }),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .unwrap()
        .doc
}

/// **A CHAIN's arc radius is flow-bearing exactly as a carrier loop's
/// is** — the v5 exception is about the ROLE, not about the loop form.
///
/// [`resolved_values_feed_the_key`] pins it for the one radius a
/// carrier loop is drawn at. A chain has no such loop-wide radius: each
/// arc step authors its own, and each reaches the wall its own arc
/// sweeps, so each spelling is an input to a body downstream and two
/// spellings of one value are two different bodies. Keying them
/// identically would let the memo serve a body whose token names an
/// expression the document no longer holds — for exactly the loops the
/// carrier-radius pin cannot see.
#[test]
fn a_chain_arcs_radius_feeds_the_key() {
    let parameterized = doc_with_r(
        0.5,
        vec![one_arc_chain(Expr::param(
            ParamName::from_static("r"),
            Dimension::Length,
        ))],
    );
    let literal = doc_with_r(0.5, vec![one_arc_chain(len(0.5))]);
    assert_ne!(
        key_of(&parameterized),
        key_of(&literal),
        "a chain arc's radius is flow-bearing: its SPELLING reaches the wall that arc \
         sweeps, so two spellings of one value must not share a memo entry"
    );
}

/// **A chain with no arc has no radius to feed**: every expression it
/// holds is a coordinate or a length, and re-spelling one at the same
/// value keys identically.
///
/// The bound on the row above, and the same bound
/// [`a_carrier_centre_respelled_keys_identically`] puts on the carrier
/// form's: what entered the key is the radius ROLE and nothing else. A
/// feed that wrote every Length expression of a chain would red here.
#[test]
fn a_straight_chain_respelled_keys_identically() {
    let straight = |length: Expr| {
        LoopProgram::Chain(vec![
            ProgramStep::At([len(0.0), len(0.0)]),
            ProgramStep::Toward {
                dx: scl(1.0),
                dy: scl(0.0),
            },
            ProgramStep::Line(length),
            ProgramStep::LineTo(ProgramTarget::Point([len(2.0), len(3.0)])),
            ProgramStep::LineTo(ProgramTarget::Start),
        ])
    };
    let parameterized = doc_with_r(
        4.0,
        vec![straight(Expr::param(
            ParamName::from_static("r"),
            Dimension::Length,
        ))],
    );
    let literal = respelled(&parameterized, StepArg::Length, len(4.0));
    assert_eq!(
        key_of(&parameterized),
        key_of(&literal),
        "a leg's length reaches no stored field, so its spelling must not enter the key"
    );
}

/// D7's key blindness, pinned at the DOCUMENT level (§4g acceptance):
/// two docs differing ONLY in display units are key-equal.
#[test]
fn display_units_never_enter_the_key() {
    let params = std::collections::BTreeMap::new();
    let mm = parse_expr("500 mm", &params).unwrap();
    let m = parse_expr("0.5 m", &params).unwrap();
    let canonical = len(0.5);
    let make = |r: Expr| {
        doc_with(vec![LoopProgram::Circle {
            centre: [len(0.0), len(0.0)],
            radius: r,
        }])
    };
    let k_mm = key_of(&make(mm));
    assert_eq!(k_mm, key_of(&make(m)));
    assert_eq!(k_mm, key_of(&make(canonical)));
}

/// A structural program edit is a key change even when many floats
/// coincide: appending a step re-tags the stream.
#[test]
fn step_structure_moves_the_key() {
    let tri = LoopProgram::Chain(vec![
        ProgramStep::At(len2([0.0, 0.0])),
        ProgramStep::LineTo(ProgramTarget::Point(len2([2.0, 0.0]))),
        ProgramStep::LineTo(ProgramTarget::Point(len2([1.0, 2.0]))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    let quad = LoopProgram::Chain(vec![
        ProgramStep::At(len2([0.0, 0.0])),
        ProgramStep::LineTo(ProgramTarget::Point(len2([2.0, 0.0]))),
        ProgramStep::LineTo(ProgramTarget::Point(len2([1.0, 2.0]))),
        ProgramStep::LineTo(ProgramTarget::Point(len2([0.0, 1.0]))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    assert_ne!(key_of(&doc_with(vec![tri])), key_of(&doc_with(vec![quad])));
}
