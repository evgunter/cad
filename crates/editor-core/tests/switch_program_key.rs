//! **The v4 program KEY-SPACE pins (LIB-SWITCH §4e)** — the successor
//! of the retired `ProfileDesc::tokens` key pins (#101 MINOR-1/NOTE-1;
//! their subject, the stored token stream, died with the stored
//! segments): the profile content key now feeds from the program's
//! (tag, payload) stream — plane bits, per-loop LoopStart tags, per
//! resolved step the verb tag + structural tags + resolved-f64 bits —
//! so structure can never alias float data, resolved values move the
//! key, and display units NEVER enter it (D7).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{
    CancelToken, ContentKey, Dimension, DocEdit, DocParam, EvalOptions, Expr, LoopProgram, Node,
    ParamName, ProfileDoc, ProfileProgram, ProgramStep, ProgramTarget, RecipeNodeId, evaluate,
    parse_expr,
};
use profile::SketchPlane;
use geom_core::Tol;

fn key_of(doc: &ProfileDoc) -> ContentKey {
    let ev = evaluate::<f64>(doc, None, &CancelToken::new(), &EvalOptions::default(), Tol::witness());
    ev.value(RecipeNodeId(0))
        .expect("profile evaluates")
        .content_key
}

fn doc_with(loops: Vec<LoopProgram>) -> ProfileDoc {
    let doc = ProfileDoc::empty_derived("switch_program_key", Tol::witness());
    doc.apply(&DocEdit::InsertNode {
        node: Node::Profile(ProfileProgram {
            plane: SketchPlane::xy(),
            loops,
        }),
    }, Tol::witness())
    .expect("valid program")
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

/// The resolved-value convention: a param edit that changes a resolved
/// program value MOVES the key; re-spelling the same value as a
/// literal keys IDENTICALLY (resolved bits, not spelling, feed it).
#[test]
fn resolved_values_feed_the_key() {
    let with_param = |value: f64| {
        let doc = ProfileDoc::empty_derived("switch_program_key", Tol::witness());
        let doc = doc
            .apply(&DocEdit::SetDocParam {
                name: ParamName::new("r"),
                value: DocParam::Continuous {
                    dim: Dimension::Length,
                    value,
                },
            }, Tol::witness())
            .unwrap()
            .doc;
        doc.apply(&DocEdit::InsertNode {
            node: Node::Profile(ProfileProgram {
                plane: SketchPlane::xy(),
                loops: vec![LoopProgram::Circle {
                    centre: [
                        Expr::literal(0.0, Dimension::Length).unwrap(),
                        Expr::literal(0.0, Dimension::Length).unwrap(),
                    ],
                    radius: Expr::param(ParamName::new("r"), Dimension::Length),
                }],
            }),
        }, Tol::witness())
        .unwrap()
        .doc
    };
    let k_half = key_of(&with_param(0.5));
    let k_quarter = key_of(&with_param(0.25));
    assert_ne!(k_half, k_quarter, "a resolved-value change moves the key");
    let literal = doc_with(vec![LoopProgram::circle(0.0, 0.0, 0.5).unwrap()]);
    assert_eq!(
        k_half,
        key_of(&literal),
        "same resolved bits, same key — spelling does not enter (V2's \
         resolved-value convention, inherited from node slots)"
    );
}

/// D7's key blindness, pinned at the DOCUMENT level (§4g acceptance):
/// two docs differing ONLY in display units are key-equal.
#[test]
fn display_units_never_enter_the_key() {
    let params = std::collections::BTreeMap::new();
    let mm = parse_expr("500 mm", &params).unwrap();
    let m = parse_expr("0.5 m", &params).unwrap();
    let canonical = Expr::literal(0.5, Dimension::Length).unwrap();
    let make = |r: Expr| {
        doc_with(vec![LoopProgram::Circle {
            centre: [
                Expr::literal(0.0, Dimension::Length).unwrap(),
                Expr::literal(0.0, Dimension::Length).unwrap(),
            ],
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
    let lpt = |x: f64, y: f64| {
        [
            Expr::literal(x, Dimension::Length).unwrap(),
            Expr::literal(y, Dimension::Length).unwrap(),
        ]
    };
    let tri = LoopProgram::Chain(vec![
        ProgramStep::At(lpt(0.0, 0.0)),
        ProgramStep::LineTo(ProgramTarget::Point(lpt(2.0, 0.0))),
        ProgramStep::LineTo(ProgramTarget::Point(lpt(1.0, 2.0))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    let quad = LoopProgram::Chain(vec![
        ProgramStep::At(lpt(0.0, 0.0)),
        ProgramStep::LineTo(ProgramTarget::Point(lpt(2.0, 0.0))),
        ProgramStep::LineTo(ProgramTarget::Point(lpt(1.0, 2.0))),
        ProgramStep::LineTo(ProgramTarget::Point(lpt(0.0, 1.0))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    assert_ne!(key_of(&doc_with(vec![tri])), key_of(&doc_with(vec![quad])));
}
