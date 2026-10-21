//! M4 PR 7 (appearance half), interval lane: f64/Interval agreement
//! on appearance RESOLUTION — same verdicts ⇒ identical tables (the
//! PR 3 invariant) ⇒ identical resolved rows AND identical typed
//! losses at both scalar types. Resolution is scalar-independent data
//! (names, entity refs, exact attribute values), so the comparison is
//! direct equality.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    Attr, AttrKind, CancelToken, CapEnd, DocEdit, EntityKind, EvalOptions, Evaluation, Node,
    ProfileDoc, Rgba8, RoleSeg, StableName, evaluate,
};
use fixture::{insert, step};
use geom_core::Interval;
use geom_core::Tol;

fn run<T: editor_core::EvalScalar>(doc: &ProfileDoc) -> Evaluation<T> {
    evaluate::<T>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

#[test]
fn f64_and_interval_lanes_resolve_appearance_identically() {
    // An overlapping union plus a free extrude — enough to exercise
    // resolved rows (operand-wrapped and pass-through names), a
    // Vanished loss, and multi-attribute entries at both lanes.
    let doc = ProfileDoc::empty_derived("m4_pr7_appearance_interval", Tol::witness());
    let (doc, a) = crate::blocks::block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, b) = crate::blocks::block_on(doc, a, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, uni) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
        },
    );
    // Resolving: the union body, a union-minted face, and operand A's
    // end cap (resolves in A's own table).
    let ev: Evaluation<f64> = run(&doc);
    let uni_face = ev
        .table_at(uni)
        .unwrap()
        .iter()
        .find_map(|(n, e)| {
            (n.kind == EntityKind::Face && matches!(e, editor_core::Entry::Unique(_)))
                .then(|| n.clone())
        })
        .expect("union mints face names");
    let mut doc = doc;
    for (name, attr) in [
        (
            StableName {
                kind: EntityKind::Body,
                node: uni,
                path: vec![RoleSeg::OutputBody],
            },
            Attr::Label(editor_core::Label::new("union").unwrap()),
        ),
        (uni_face.clone(), Attr::Color(Rgba8::opaque(10, 200, 10))),
        (uni_face, Attr::Visibility(true)),
        (
            StableName {
                kind: EntityKind::Face,
                node: a,
                path: vec![RoleSeg::Cap(CapEnd::End)],
            },
            Attr::Color(Rgba8::opaque(200, 10, 10)),
        ),
        // Vanishing: a legal-at-edit-time name the evaluation never
        // mints (node live, name-level resolution is evaluation
        // work) — a typed Vanished loss at BOTH lanes.
        (
            StableName {
                kind: EntityKind::Face,
                node: uni,
                path: vec![RoleSeg::Cap(CapEnd::Start)],
            },
            Attr::Visibility(false),
        ),
    ] {
        let read = crate::fixture::painted_in(&doc, &name);
        (doc, _) = step(
            doc,
            DocEdit::SetAppearance {
                body: read,
                name,
                attr,
            },
        );
    }

    let ef: Evaluation<f64> = run(&doc);
    let ei: Evaluation<Interval> = run(&doc);
    // Sanity: the fixture exercises both sides of the resolution.
    assert!(!ef.appearance.resolved.is_empty());
    assert_eq!(ef.appearance.losses.len(), 1);
    // The Q1 genericity boundary: identical resolution, rows and
    // losses, at both scalar types.
    assert_eq!(ef.appearance, ei.appearance, "lane disagreement");
    // Multi-attribute agreement, spelled out: the union face carries
    // Color AND Visibility in both lanes.
    let rows_f = ef.appearance.for_node(uni).unwrap();
    let rows_i = ei.appearance.for_node(uni).unwrap();
    assert_eq!(rows_f, rows_i);
    assert!(
        rows_f
            .values()
            .any(|attrs| attrs.contains_key(&AttrKind::Color)
                && attrs.contains_key(&AttrKind::Visibility))
    );
}
