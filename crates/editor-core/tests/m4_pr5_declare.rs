//! M4 PR 5 acceptance (spec D4–D6): Declare threading end-to-end.
//!
//! - **D6.1, the M3 closure gap CLOSED by recipe intent**: a 3′ body
//!   (corner-kiss union) reused as an operand certifies at the 3′
//!   gate exactly when the recipe DECLARES the surviving v-v intent
//!   by name — and stays a loud `UndeclaredContact` refusal without
//!   it (the envelope entry's fix direction, landed).
//! - **D6.3, the #91 flush-plane narrative**: a coincident-plane pair
//!   glues when authored WITH a Declare (N3 `Merged` row minted);
//!   undeclared it refuses typed; the decoupled variant is untouched.
//! - **D6.4 / PR 3 R13**: the #90 crossing-slots double subtract
//!   promoted to a recipe document — the first boolean-of-boolean
//!   naming/resolution corpus entry.
//! - **N5 doors**: Declare resolution failures are the typed
//!   ResolveError verbatim; out-of-vocabulary pairs refuse typed.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{
    BooleanOp, BooleanValue, CapEnd, EntityKind, Node, NodeErrorKind, NodeResult, ProfileDoc,
    ProfileVertexRef, RecipeNodeId, RoleSeg, StableName, ValuePayload,
};
use fixture::{declare_x_offset_flush, desc, fname, insert, len, wall};
use topo::validate_pseudomanifold;

fn run(doc: &ProfileDoc) -> editor_core::Evaluation<f64> {
    editor_core::evaluate(
        doc,
        None,
        &editor_core::CancelToken::new(),
        &editor_core::EvalOptions::default(),
    )
}

/// An axis-aligned block on the xy plane at height z0, extruded dz.
fn block(
    doc: ProfileDoc,
    x: (f64, f64),
    y: (f64, f64),
    z0: f64,
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = insert(
        doc,
        Node::Profile(desc(
            [0.0, 0.0, z0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)]],
        )),
    );
    insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(dz),
        },
    )
}

fn vname(node: RecipeNodeId, seg: RoleSeg) -> StableName {
    StableName {
        kind: EntityKind::Vertex,
        node,
        path: vec![seg],
    }
}

fn boolean_value(ev: &editor_core::Evaluation<f64>, node: RecipeNodeId) -> &BooleanValue<f64> {
    match &ev.value(node).expect("boolean evaluated").payload {
        ValuePayload::Boolean(b) => b,
        other => panic!("expected boolean, got {}", other.kind_name()),
    }
}

/// The corner-kiss base document: a = [0,1]³, b = [1,2]³, unioned
/// (the v-v kiss at (1,1,1) is DISCOVERED by the op and recorded in
/// the result's contacts). Returns (doc, a, b, union).
fn kiss_base(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId, RecipeNodeId, RecipeNodeId) {
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (1.0, 2.0), (1.0, 2.0), 1.0, 1.0);
    let (doc, u) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b,
            declare: None,
        },
    );
    (doc, a, b, u)
}

/// The base union's two kiss-vertex names, as the base's table
/// carries them (FromX-wrapped operand cap vertices at (1,1,1)).
fn kiss_vertex_names(
    a: RecipeNodeId,
    b: RecipeNodeId,
    u: RecipeNodeId,
) -> (StableName, StableName) {
    // a's (1,1) top-cap vertex: profile (0,0)(1,0)(1,1)(0,1) → vertex 2.
    let va = vname(
        a,
        RoleSeg::CapVertex(
            CapEnd::Top,
            ProfileVertexRef {
                loop_index: 0,
                vertex: 2,
            },
        ),
    );
    // b's (1,1) bottom-cap vertex: profile (1,1)(2,1)(2,2)(1,2) → vertex 0.
    let vb = vname(
        b,
        RoleSeg::CapVertex(
            CapEnd::Bottom,
            ProfileVertexRef {
                loop_index: 0,
                vertex: 0,
            },
        ),
    );
    (
        vname(u, RoleSeg::FromA(Box::new(va))),
        vname(u, RoleSeg::FromB(Box::new(vb))),
    )
}

/// D6.1: the closure-corpus 3′-refused row becomes a certified pass
/// exactly when the recipe declares the surviving intent — and stays
/// a loud refusal otherwise.
#[test]
fn reused_kiss_certifies_with_declared_intent_and_refuses_without() {
    // WITHOUT a Declare: the new result re-discovers the surviving
    // operand-internal kiss as UNDECLARED (the documented M3 gap).
    let (doc, a, b, base) = kiss_base(ProfileDoc::empty());
    let (doc_undeclared, mover) = block(doc.clone(), (1.5, 2.5), (1.5, 2.5), 1.5, 1.0);
    let (doc_undeclared, u2) = insert(
        doc_undeclared,
        Node::Boolean {
            op: BooleanOp::Union,
            a: base,
            b: mover,
            declare: None,
        },
    );
    let ev = run(&doc_undeclared);
    let BooleanValue::Body { body, contacts, .. } = boolean_value(&ev, u2) else {
        panic!("union is a body");
    };
    let errors = validate_pseudomanifold(body, contacts).unwrap_err();
    assert!(
        errors
            .iter()
            .all(|e| matches!(e, topo::ValidationError::UndeclaredContact { .. })),
        "undeclared reuse must 3'-refuse UndeclaredContact only: {errors:?}"
    );

    // WITH the Declare naming the surviving v-v intent BY NAME (the
    // reused body's declaration re-enters by name, never arena key):
    // certified 3' pass.
    let (doc_declared, mover) = block(doc, (1.5, 2.5), (1.5, 2.5), 1.5, 1.0);
    let (va, vb) = kiss_vertex_names(a, b, base);
    let (doc_declared, decl) = insert(
        doc_declared,
        Node::Declare {
            pairs: vec![(va, vb)],
        },
    );
    let (doc_declared, u2) = insert(
        doc_declared,
        Node::Boolean {
            op: BooleanOp::Union,
            a: base,
            b: mover,
            declare: Some(decl),
        },
    );
    let ev = run(&doc_declared);
    let BooleanValue::Body { body, contacts, .. } = boolean_value(&ev, u2) else {
        panic!("union is a body");
    };
    assert_eq!(
        validate_pseudomanifold(body, contacts),
        Ok(()),
        "declared reuse must certify at the 3' gate"
    );
    // Exactly the carried kiss survives as a v-v record (the mover
    // overlap is seam-crossing, not kissing — it declares nothing).
    assert_eq!(contacts.vv.len(), 1, "{contacts:?}");
}

/// D6.3, the #91 flush-plane narrative: the coincident-plane pair
/// GLUES when authored WITH a Declare (N3 `Merged` row), refuses
/// typed without one, and the decoupled variant is untouched.
#[test]
fn flush_plane_pair_glues_with_declare_refuses_without() {
    // Coincident-plane pair, UNDECLARED: typed refusal at the
    // coincidence door (rung (b): value equality never classifies).
    let doc = ProfileDoc::empty();
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc_undeclared, u) = insert(
        doc.clone(),
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b,
            declare: None,
        },
    );
    let ev = run(&doc_undeclared);
    match ev.nodes.get(&u) {
        Some(NodeResult::Failed(e)) => assert!(
            matches!(
                e.kind,
                NodeErrorKind::Boolean(topo::BooleanError::UndeclaredCoincidence { .. })
            ),
            "expected UndeclaredCoincidence, got {:?}",
            e.kind
        ),
        other => panic!("undeclared flush union must fail, got {other:?}"),
    }

    // WITH the Declare: glues — Merged rows minted, tiers green.
    let (doc_declared, decl) = declare_x_offset_flush(doc, a, b);
    let (doc_declared, u) = insert(
        doc_declared,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b,
            declare: Some(decl),
        },
    );
    let ev = run(&doc_declared);
    let BooleanValue::Body { body, .. } = boolean_value(&ev, u) else {
        panic!("declared union is a body");
    };
    assert_eq!(topo::mass_properties(body).unwrap().volume, 1.5);
    assert_eq!(topo::validate::validate_geometric(body), Ok(()));
    let merged_rows = ev
        .value(u)
        .unwrap()
        .name_table
        .iter()
        .filter(|(n, _)| matches!(n.path.first(), Some(RoleSeg::Merged(_))))
        .count();
    assert_eq!(merged_rows, 4, "caps + flush y-walls glue");

    // Decoupled variant (no coincident planes): untouched — no
    // Declare needed, transversal union works as before.
    let doc = ProfileDoc::empty();
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.25, 0.75), 0.25, 1.25);
    let (doc, u) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b,
            declare: None,
        },
    );
    let ev = run(&doc);
    let BooleanValue::Body { body, .. } = boolean_value(&ev, u) else {
        panic!("decoupled union is a body");
    };
    // 1 + (b = 1·0.5·1.25) − (overlap = 0.5·0.5·0.75) = 1.4375.
    assert_eq!(topo::mass_properties(body).unwrap().volume, 1.4375);
}

/// D6.4 / PR 3 R13: the #90 crossing-slots double subtract as a
/// recipe document — the first boolean-of-boolean corpus entry. The
/// slot floors are flush with each other (both z = 0.5 INSIDE the
/// slab), the tools' caps are flush with nothing (they straddle),
/// but each second-op slot floor meets the FIRST slot's floor in one
/// plane — declared through the FromA-wrapped name.
#[test]
fn crossing_slots_recipe_document_evaluates_and_resolves() {
    let doc = ProfileDoc::empty();
    let (doc, slab) = block(doc, (0.0, 3.0), (0.0, 3.0), 0.0, 1.0);
    let (doc, b1) = block(doc, (1.0, 2.0), (-1.0, 4.0), 0.5, 1.0);
    let (doc, s1) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a: slab,
            b: b1,
            declare: None,
        },
    );
    // Slot floor of s1 = FromB(b1's Cap(Bottom)); the second tool's
    // bottom cap lies in the SAME plane (z = 0.5) — declared.
    let floor1 = fname(
        s1,
        RoleSeg::FromB(Box::new(fname(b1, RoleSeg::Cap(CapEnd::Bottom)))),
    );
    let (doc, b2) = block(doc, (-1.0, 4.0), (1.0, 2.0), 0.5, 1.0);
    let (doc, decl) = insert(
        doc,
        Node::Declare {
            pairs: vec![(floor1.clone(), fname(b2, RoleSeg::Cap(CapEnd::Bottom)))],
        },
    );
    let (doc, s2) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a: s1,
            b: b2,
            declare: Some(decl),
        },
    );
    let ev = run(&doc);
    let BooleanValue::Body { body, contacts, .. } = boolean_value(&ev, s2) else {
        panic!("crossing slots is a body");
    };
    // #90's oracle: 9 − two 0.5-deep slots + shared middle.
    assert_eq!(
        topo::mass_properties(body).unwrap().volume,
        9.0 - (3.0 * 0.5 + 3.0 * 0.5 - 0.5) * 1.0
    );
    assert_eq!(topo::validate_pseudomanifold(body, contacts), Ok(()));
    // Resolution corpus seed (boolean-of-boolean DAG): the two slot
    // floors GLUED (declared) — the wrapped floor lineage retired
    // into the Merged row, fails typed, and OFFERS the merge; the
    // Merged row itself resolves on the final body (N3, live).
    let ctx = editor_core::resolve::RunCtx {
        doc: &doc,
        eval: &ev,
    };
    let wrapped_floor1 = fname(s2, RoleSeg::FromA(Box::new(floor1)));
    let merged_row: StableName = ev
        .value(s2)
        .unwrap()
        .name_table
        .iter()
        .find_map(|(n, _)| {
            matches!(n.path.first(), Some(RoleSeg::Merged(cs)) if cs.contains(&wrapped_floor1))
                .then(|| n.clone())
        })
        .expect("the glued floors mint a Merged row embedding the slot-floor lineage");
    match editor_core::resolve::resolve(ctx, &merged_row) {
        editor_core::resolve::Resolution::Resolved(r) => assert_eq!(r.node, s2),
        other => panic!("the Merged floor must resolve on the final body: {other:?}"),
    }
    match editor_core::resolve::resolve(ctx, &wrapped_floor1) {
        editor_core::resolve::Resolution::Failed(f) => assert!(
            f.offers.contains(&merged_row),
            "the retired floor lineage must offer its merge: {f:?}"
        ),
        other => panic!("expected the retired lineage to fail typed, got {other:?}"),
    }
}

/// N5 doors: a Declare naming a VANISHED, FOREIGN, or out-of-
/// vocabulary reference refuses with the typed error — no silent
/// drop, no best-effort gluing.
#[test]
fn declare_resolution_failures_are_typed_n5_errors() {
    let base = ProfileDoc::empty();
    let (base, a) = block(base, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (base, b) = block(base, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let boolean_with = |doc: ProfileDoc, decl| {
        insert(
            doc,
            Node::Boolean {
                op: BooleanOp::Union,
                a,
                b,
                declare: Some(decl),
            },
        )
    };
    let failed_kind = |ev: &editor_core::Evaluation<f64>, node| match ev.nodes.get(&node) {
        Some(NodeResult::Failed(e)) => format!("{:?}", e.kind),
        other => panic!("expected Failed, got {other:?}"),
    };

    // Vanished: a name the operands' tables never carried.
    let ghost = fname(a, wall(1)); // exists…
    let mut ghost = ghost;
    ghost.path = vec![RoleSeg::Cap(CapEnd::Top), RoleSeg::Cap(CapEnd::Top)]; // …not any more
    let (doc, decl) = insert(
        base.clone(),
        Node::Declare {
            pairs: vec![(ghost, fname(b, RoleSeg::Cap(CapEnd::Top)))],
        },
    );
    let (doc, u) = boolean_with(doc, decl);
    let k = failed_kind(&run(&doc), u);
    assert!(
        k.contains("DeclareResolve") && k.contains("Vanished"),
        "{k}"
    );

    // Out-of-vocabulary: a cross-operand vertex pair (the pipeline
    // discovers cross contacts itself; v1 refuses the declaration).
    let va = vname(
        a,
        RoleSeg::CapVertex(
            CapEnd::Top,
            ProfileVertexRef {
                loop_index: 0,
                vertex: 0,
            },
        ),
    );
    let vb = vname(
        b,
        RoleSeg::CapVertex(
            CapEnd::Top,
            ProfileVertexRef {
                loop_index: 0,
                vertex: 0,
            },
        ),
    );
    let (doc, decl) = insert(
        base.clone(),
        Node::Declare {
            pairs: vec![(va, vb)],
        },
    );
    let (doc, u) = boolean_with(doc, decl);
    let k = failed_kind(&run(&doc), u);
    assert!(k.contains("DeclareUnsupportedPair"), "{k}");
}
