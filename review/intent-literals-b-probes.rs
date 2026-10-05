//! Reviewer probes for INTENT-LITERALS PR B (not committed).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::docm7_union_declare::block;
use crate::fixture;
use editor_core::{
    Dimension, DocEdit, DocumentId, EditError, Formula, Frame, FreeVar, Node, PatternKind,
    ProfileDoc, ProfileProgram, VarName, apply,
};
use fixture::scl;
use geom_core::Tol;

fn p(name: &'static str) -> VarName {
    VarName::from_static(name)
}

fn edit(doc: &ProfileDoc, e: DocEdit<ProfileProgram>) -> Result<ProfileDoc, EditError> {
    apply(doc, &e, Tol::witness(), &editor_core::RefusingReach).map(|a| a.doc)
}

fn body() -> (ProfileDoc, editor_core::RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("probe_b", Tol::witness());
    block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0)
}

/// P1: an explicit-list pattern whose count is an unheld name.
#[test]
fn p1_listed_pattern_with_an_unheld_count_name_refuses_not_panics() {
    let (doc, b) = body();
    let r = edit(
        &doc,
        DocEdit::InsertNode {
            node: Box::new(Node::Pattern {
                input: b,
                count: Formula::named(p("nope"), Dimension::Count),
                kind: PatternKind::Explicit(vec![Frame::IDENTITY]),
            }),
        },
    );
    println!("P1 -> {r:?}");
    assert!(r.is_err());
}

/// P2: a placed union's list with an unheld count name.
#[test]
fn p2_listed_union_with_an_unheld_count_name_refuses_not_panics() {
    let (doc, b) = body();
    let r = edit(
        &doc,
        DocEdit::InsertNode {
            node: Box::new(Node::PlacedUnion {
                input: b,
                count: Some(Formula::named(p("nope"), Dimension::Count)),
                kind: PatternKind::Explicit(vec![Frame::IDENTITY]),
            }),
        },
    );
    println!("P2 -> {r:?}");
    assert!(r.is_err());
}

/// P3: DefineVar with a kind change AND an unheld name: which wins?
#[test]
fn p3_define_var_kind_and_unheld_name() {
    let doc = ProfileDoc::empty(DocumentId::derive("probe-b3"), Tol::witness());
    let doc = edit(
        &doc,
        DocEdit::DeclareVar {
            name: p("n"),
            def: editor_core::VarDecl::Free(FreeVar::Count { value: 4 }),
        },
    )
    .unwrap();
    let r = edit(
        &doc,
        DocEdit::DefineVar {
            var: p("n").into(),
            def: editor_core::VarDecl::Defined(Formula::named(p("nope"), Dimension::Length)),
        },
    )
    .expect_err("refuses");
    println!("P3 -> {r:?}");
    assert!(
        matches!(r, EditError::VarKindFixed { .. }),
        "base order: VarKindFixed before the name; got {r:?}"
    );
}

/// P4: an insert holding one held and one unheld name speaks the id of
/// the node it would mint. Under the base the held name was lowered
/// before the id was drawn, so `w` by name and `w` by id spoke ONE id.
#[test]
fn p4_refused_insert_speaks_one_id_by_name_or_by_id() {
    let (doc, b) = body();
    let applied = apply(
        &doc,
        &DocEdit::DeclareVar {
            name: p("w"),
            def: editor_core::VarDecl::Free(FreeVar::written_length(
                quantity::WrittenLength::in_unit(5.0, quantity::MM),
            )),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .unwrap();
    let w = applied.record.minted_var.unwrap();
    let doc = applied.doc;
    let node = |first: Formula| -> editor_core::AuthoredNode { Node::Pattern {
        input: b,
        count: Formula::count(2),
        kind: PatternKind::Linear {
            direction: [scl(1.0), scl(0.0), scl(0.0)],
            spacing: Formula::add(first, Formula::named(p("nope"), Dimension::Length)).unwrap(),
        },
    }};
    let by_name = edit(
        &doc,
        DocEdit::InsertNode {
            node: Box::new(node(Formula::named(p("w"), Dimension::Length))),
        },
    )
    .expect_err("nope is unheld");
    let by_id = edit(
        &doc,
        DocEdit::InsertNode {
            node: Box::new(node(Formula::var(w, Dimension::Length))),
        },
    )
    .expect_err("nope is unheld");
    println!("P4 by name -> {by_name}\nP4 by id   -> {by_id}");
    assert_eq!(by_name.to_string(), by_id.to_string());
}

/// P5: on every corpus node, the slots `try_map_slots` visits equal the
/// slots the door pre-checks (`slots()` plus the payload carriers').
#[test]
fn p5_try_map_slots_visits_exactly_the_prechecked_slots() {
    let mut bad = Vec::new();
    let mut seen = 0usize;
    for d in crate::corpus::documents() {
        for &id in d.doc.order() {
            let node = d.doc.node(id).unwrap();
            let mut visited = 0usize;
            let _ = node.try_map_slots(
                |p: &ProfileProgram, f| p.try_map_slots(&mut |e| f(e)),
                &mut |e: &editor_core::Expr| {
                    visited += 1;
                    Ok::<_, ()>(e.clone())
                },
            );
            if matches!(node, Node::Measure { .. } | Node::Assertion { .. }) {
                continue;
            }
            let checked = node.slots().len();
            seen += 1;
            if visited != checked {
                bad.push((d.name, id, visited, checked));
            }
        }
    }
    println!("P5 nodes {seen}, mismatches {bad:?}");
    assert!(bad.is_empty());
}
