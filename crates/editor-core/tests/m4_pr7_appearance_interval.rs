//! M4 PR 7 (appearance half), interval lane: f64/Interval agreement
//! on appearance RESOLUTION — same verdicts ⇒ identical tables (the
//! PR 3 invariant) ⇒ identical resolved rows AND identical typed
//! losses at both scalar types. Resolution is scalar-independent data
//! (names, entity refs, exact attribute values), so the comparison is
//! direct equality.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{
    Attr, AttrKind, BooleanOp, CancelToken, CapEnd, DocEdit, EntityKind, EvalOptions, Evaluation,
    Node, ProfileDoc, RecipeNodeId, Rgba8, RoleSeg, StableName, evaluate,
};
use fixture::{declare_x_offset_flush, insert, len, on_frame, step};
use geom_core::Interval;
use geom_core::Tol;

/// A rectangular block: profile on the plane z = `z0`, extruded `dz`.
fn block(
    doc: ProfileDoc,
    (x0, x1): (f64, f64),
    (y0, y1): (f64, f64),
    z0: f64,
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, z0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(dz),
        },
    )
}

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
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, decl) = declare_x_offset_flush(doc, a, b);
    let (doc, uni) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b,
            declare: Some(decl),
        },
    );
    // Resolving: the union body, a union-minted face, and operand A's
    // top cap (resolves in A's own table).
    let ev: Evaluation<f64> = run(&doc);
    let uni_face = ev
        .value(uni)
        .unwrap()
        .name_table
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
            Attr::Label("union".into()),
        ),
        (uni_face.clone(), Attr::Color(Rgba8::opaque(10, 200, 10))),
        (uni_face, Attr::Visibility(true)),
        (
            StableName {
                kind: EntityKind::Face,
                node: a,
                path: vec![RoleSeg::Cap(CapEnd::Top)],
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
                path: vec![RoleSeg::Cap(CapEnd::Bottom)],
            },
            Attr::Visibility(false),
        ),
    ] {
        (doc, _) = step(doc, DocEdit::SetAppearance { name, attr });
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
