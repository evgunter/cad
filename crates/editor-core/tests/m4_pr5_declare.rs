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

use crate::fixture;

use editor_core::{
    BooleanOp, BooleanValue, CapEnd, EntityKind, Node, NodeErrorKind, NodeResult, ProfileDoc,
    RecipeNodeId, RoleSeg, SitedRef, StableName, ValuePayload,
};
use fixture::{declare_x_offset_flush, fname, insert, len, on_frame, vname, wall};
use geom_core::Tol;
use topo::validate_pseudomanifold;

/// Evaluates, and holds every table the run produced to the N3
/// flatness rule on the way out. A tripwire over this suite's merged
/// rows, not the guard: the mint refuses a nested constituent before
/// a table is published, and the rows that carry the rule are
/// `docm8_flat_merged`'s (the corpus walk and the mint-site rows).
fn run(doc: &ProfileDoc) -> editor_core::Evaluation<f64> {
    let ev = editor_core::evaluate(
        doc,
        None,
        &editor_core::CancelToken::new(),
        &editor_core::EvalOptions::default(),
        Tol::witness(),
    );
    fixture::assert_no_nested_merged(&ev);
    ev
}

/// An axis-aligned block on the xy plane at height z0, extruded dz.
fn block(
    doc: ProfileDoc,
    x: (f64, f64),
    y: (f64, f64),
    z0: f64,
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, z0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(dz),
        },
    )
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
    doc: &editor_core::ProfileDoc,
    a: RecipeNodeId,
    b: RecipeNodeId,
    u: RecipeNodeId,
) -> (StableName, StableName) {
    // a's (1,1) top-cap vertex: profile (0,0)(1,0)(1,1)(0,1) → vertex 2.
    let va = vname(
        a,
        RoleSeg::CapVertex(CapEnd::End, crate::fixture::vpiece(doc, a, 0, 2)),
    );
    // b's (1,1) bottom-cap vertex: profile (1,1)(2,1)(2,2)(1,2) → vertex 0.
    let vb = vname(
        b,
        RoleSeg::CapVertex(CapEnd::Start, crate::fixture::vpiece(doc, b, 0, 0)),
    );
    (
        vname(u, RoleSeg::FromA(va.into())),
        vname(u, RoleSeg::FromB(vb.into())),
    )
}

/// D6.1: the closure-corpus 3′-refused row becomes a certified pass
/// exactly when the recipe declares the surviving intent — and stays
/// a loud refusal otherwise.
#[test]
fn reused_kiss_certifies_with_declared_intent_and_refuses_without() {
    // WITHOUT a Declare: the new result re-discovers the surviving
    // operand-internal kiss as UNDECLARED (the documented M3 gap).
    let (doc, a, b, base) = kiss_base(ProfileDoc::empty_derived("m4_pr5_declare", Tol::witness()));
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
    let errors = validate_pseudomanifold(body, contacts, Tol::witness()).unwrap_err();
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
    let (va, vb) = kiss_vertex_names(&doc_declared, a, b, base);
    let (doc_declared, decl) = insert(
        doc_declared,
        Node::declare_rest(vec![(SitedRef::new(base, va), SitedRef::new(base, vb))]),
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
        validate_pseudomanifold(body, contacts, Tol::witness()),
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
    let doc = ProfileDoc::empty_derived("m4_pr5_declare", Tol::witness());
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
        // Since R3 (LIB-PYG5) the undeclared-coincidence refusal
        // surfaces as the typed refusal-menu variant, finding attached.
        Some(NodeResult::Failed(e)) => assert!(
            matches!(e.kind, NodeErrorKind::UndeclaredContact { .. }),
            "expected the UndeclaredContact menu, got {:?}",
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
    assert_eq!(
        topo::mass_properties(body, Tol::witness()).unwrap().volume,
        1.5
    );
    assert_eq!(
        topo::validate::validate_geometric(body, Tol::witness()),
        Ok(())
    );
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
    let doc = ProfileDoc::empty_derived("m4_pr5_declare", Tol::witness());
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
    assert_eq!(
        topo::mass_properties(body, Tol::witness()).unwrap().volume,
        1.4375
    );
}

/// D6.4 / PR 3 R13: the #90 crossing-slots double subtract as a
/// recipe document — the first boolean-of-boolean corpus entry. The
/// slot floors are flush with each other (both z = 0.5 INSIDE the
/// slab), the tools' caps are flush with nothing (they straddle),
/// but each second-op slot floor meets the FIRST slot's floor in one
/// plane — declared through the FromA-wrapped name.
#[test]
fn crossing_slots_recipe_document_evaluates_and_resolves() {
    let doc = ProfileDoc::empty_derived("m4_pr5_declare", Tol::witness());
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
    // Slot floor of s1 = FromB(b1's Cap(Start)); the second tool's
    // start cap lies in the SAME plane (z = 0.5) — declared.
    let floor1 = fname(
        s1,
        RoleSeg::FromB(fname(b1, RoleSeg::Cap(CapEnd::Start)).into()),
    );
    let (doc, b2) = block(doc, (-1.0, 4.0), (1.0, 2.0), 0.5, 1.0);
    let (doc, decl) = insert(
        doc,
        Node::declare_rest(vec![(
            SitedRef::new(s1, floor1.clone()),
            SitedRef::new(b2, fname(b2, RoleSeg::Cap(CapEnd::Start))),
        )]),
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
        topo::mass_properties(body, Tol::witness()).unwrap().volume,
        9.0 - (3.0 * 0.5 + 3.0 * 0.5 - 0.5) * 1.0
    );
    assert_eq!(
        topo::validate_pseudomanifold(body, contacts, Tol::witness()),
        Ok(())
    );
    // Resolution corpus seed (boolean-of-boolean DAG): the two slot
    // floors GLUED (declared) — the wrapped floor lineage retired
    // into the Merged row, fails typed, and OFFERS the merge; the
    // Merged row itself resolves on the final body (N3, live).
    let ctx = editor_core::resolve::RunCtx {
        doc: &doc,
        eval: &ev,
    };
    let wrapped_floor1 = fname(s2, RoleSeg::FromA(floor1.into()));
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

/// N5 doors: a Declare naming a VANISHED or out-of-vocabulary
/// reference refuses with the typed error — no silent drop, no
/// best-effort gluing. (The FOREIGN arm — a name whose node id was
/// never minted — is defense-in-depth only: `apply` validates
/// Declare-named node EXISTENCE at edit time, so no apply-built
/// document can carry one; NodeGone-by-DELETE is the reachable case,
/// covered below.)
#[test]
fn declare_resolution_failures_are_typed_n5_errors() {
    let base = ProfileDoc::empty_derived("m4_pr5_declare", Tol::witness());
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
    let ghost = fname(a, wall(&base, a, 1)); // exists…
    let mut ghost = ghost;
    ghost.path = vec![RoleSeg::Cap(CapEnd::End), RoleSeg::Cap(CapEnd::End)]; // …not any more
    let (doc, decl) = insert(
        base.clone(),
        Node::declare_rest(vec![(
            SitedRef::new(a, ghost.clone()),
            SitedRef::new(b, fname(b, RoleSeg::Cap(CapEnd::End))),
        )]),
    );
    let (doc, u) = boolean_with(doc, decl);
    let ev = run(&doc);
    match ev.nodes.get(&u) {
        Some(NodeResult::Failed(e)) => match &e.kind {
            NodeErrorKind::DeclareResolve { error } => match error.as_ref() {
                editor_core::resolve::ResolveError::Vanished {
                    name,
                    diagnosis,
                    last_good,
                } => {
                    assert_eq!(name, &ghost);
                    // The mid-evaluation payload, pinned: nothing is
                    // banked to compare against, so the diagnosis
                    // names the MINTING NODE as the disagreement site
                    // rather than claiming an edit happened.
                    assert!(last_good.is_none(), "no prior run is consultable mid-eval");
                    assert!(
                        matches!(
                            diagnosis,
                            editor_core::resolve::Diagnosis::RecipeEdit {
                                edit: editor_core::resolve::RecipeEditRef::NodeChanged { node },
                            } if *node == ghost.node
                        ),
                        "{diagnosis:?}"
                    );
                }
                other => panic!("expected Vanished, got {other:?}"),
            },
            other => panic!("expected DeclareResolve, got {other:?}"),
        },
        other => panic!("expected Failed, got {other:?}"),
    }

    // Out-of-vocabulary: a cross-operand vertex pair (the pipeline
    // discovers cross contacts itself; v1 refuses the declaration).
    let va = vname(
        a,
        RoleSeg::CapVertex(CapEnd::End, crate::fixture::vpiece(&doc, a, 0, 0)),
    );
    let vb = vname(
        b,
        RoleSeg::CapVertex(CapEnd::End, crate::fixture::vpiece(&doc, b, 0, 0)),
    );
    let (doc, decl) = insert(
        base.clone(),
        Node::declare_rest(vec![(SitedRef::new(a, va), SitedRef::new(b, vb))]),
    );
    let (doc, u) = boolean_with(doc, decl);
    let k = failed_kind(&run(&doc), u);
    assert!(k.contains("DeclareUnsupportedPair"), "{k}");
}

/// Review F1, recipe door: flush caps DECLARED on an ordinary partial
/// overlap (walls offset) — the cap groups license, land outside the
/// merge's never-elide inventory, and are SKIPPED; the result must
/// still be tier-3 honest (no stale in-plane descriptions) with the
/// exact volume.
#[test]
fn skipped_declared_merge_recipe_door_is_tier3_green() {
    let doc = ProfileDoc::empty_derived("m4_pr5_declare", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.25, 1.25), 0.0, 1.0);
    let (doc, decl) = insert(
        doc,
        Node::declare_rest(vec![
            (
                SitedRef::new(a, fname(a, RoleSeg::Cap(CapEnd::End))),
                SitedRef::new(b, fname(b, RoleSeg::Cap(CapEnd::End))),
            ),
            (
                SitedRef::new(a, fname(a, RoleSeg::Cap(CapEnd::Start))),
                SitedRef::new(b, fname(b, RoleSeg::Cap(CapEnd::Start))),
            ),
        ]),
    );
    let (doc, u) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b,
            declare: Some(decl),
        },
    );
    let ev = run(&doc);
    let BooleanValue::Body { body, contacts, .. } = boolean_value(&ev, u) else {
        panic!("declared flush-caps union is a body");
    };
    assert_eq!(
        topo::mass_properties(body, Tol::witness()).unwrap().volume,
        1.625
    );
    assert_eq!(
        topo::validate::validate_geometric(body, Tol::witness()),
        Ok(()),
        "tier 3"
    );
    assert_eq!(
        topo::validate_pseudomanifold(body, contacts, Tol::witness()),
        Ok(())
    );
    // Review F6: this shape is the corpus's PURE-seam-vertex pin —
    // the skip lane's re-described in-plane chain leaves vertices
    // whose every incident edge is Seam-named from ONE line (single
    // Seam-headed path, no junction composition). Assert they exist.
    let pure_seam_vertices = ev
        .value(u)
        .unwrap()
        .name_table
        .iter()
        .filter(|(n, _)| {
            n.kind == EntityKind::Vertex
                && matches!(n.path.first(), Some(RoleSeg::Seam { .. }))
                && n.path
                    .iter()
                    .filter(|seg| matches!(seg, RoleSeg::Seam { .. }))
                    .count()
                    == 1
        })
        .count();
    assert!(
        pure_seam_vertices >= 2,
        "expected the pure-seam-vertex naming arm to fire (single-line \
         seam vertices), got {pure_seam_vertices}"
    );
}

/// Review F4: the remaining Declare eval doors, each typed.
///
/// `DeclareBothOperands` used to be the third: a name carried by BOTH
/// operands — two placements of one prototype, whose tables are
/// identical because a transform adds no segment (N1) — left the door
/// with no side to pick. A declared entity now names the operand it is
/// read at, so that state is a declaration the door READS rather than
/// one it refuses, and the arm is gone.
/// `docm7_union_declare`'s
/// `the_pair_boolean_declares_between_two_placements_of_one_prototype`
/// is the row in its place.
#[test]
fn declare_doors_node_gone_and_ambiguous() {
    use editor_core::DocEdit;
    let failed_kind = |ev: &editor_core::Evaluation<f64>, node| match ev.nodes.get(&node) {
        Some(NodeResult::Failed(e)) => format!("{:?}", e.kind),
        other => panic!("expected Failed, got {other:?}"),
    };

    // --- NodeGone by DELETE (the reachable N5 dangling case): the
    // Declare names a third body's face; deleting that node AFTER the
    // Declare strands the name; resolution refuses NodeGone with the
    // derived NodeDeleted edit.
    let doc = ProfileDoc::empty_derived("m4_pr5_declare", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, (5.0, 6.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, decl) = insert(
        doc,
        // Sited at the operands, as every declaration is; the NAME
        // is the third body's, and rung 1 outranks the site's own
        // table having no such row.
        Node::declare_rest(vec![(
            SitedRef::new(a, fname(c, RoleSeg::Cap(CapEnd::End))),
            SitedRef::new(b, fname(b, RoleSeg::Cap(CapEnd::End))),
        )]),
    );
    let (doc, u) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b,
            declare: Some(decl),
        },
    );
    let doc = doc
        .apply(
            &DocEdit::DeleteNode { id: c },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .unwrap()
        .doc;
    let ev = run(&doc);
    let k = failed_kind(&ev, u);
    assert!(
        k.contains("DeclareResolve") && k.contains("NodeGone") && k.contains("NodeDeleted"),
        "{k}"
    );

    // --- Ambiguous: the Declare names a TIED row (the symmetric U
    // cutter's N2 tie) — refused with the tie carried honestly:
    // candidates name the tied row itself, width = the recorded tie.
    let doc = ProfileDoc::empty_derived("m4_pr5_declare", Tol::witness());
    let (doc, ua) = block(doc, (0.0, 4.0), (0.0, 4.0), 0.0, 4.0);
    let (doc, up) = on_frame(
        doc,
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (2.0, 1.0),
            (6.0, 1.0),
            (6.0, 3.0),
            (2.0, 3.0),
            (2.0, 2.5),
            (5.0, 2.5),
            (5.0, 1.5),
            (2.0, 1.5),
        ]],
    );
    let (doc, ub) = insert(
        doc,
        Node::Extrude {
            profile: up,
            distance: len(2.0),
        },
    );
    let (doc, us) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a: ua,
            b: ub,
            declare: None,
        },
    );
    let ev1 = run(&doc);
    let tied: StableName = ev1
        .value(us)
        .expect("U subtract evaluates")
        .name_table
        .iter()
        .find_map(|(n, e)| matches!(e, editor_core::Entry::Tied(_)).then(|| n.clone()))
        .expect("the U fixture ties");
    let (doc, mate) = block(doc, (0.0, 4.0), (0.0, 4.0), 6.0, 1.0);
    let (doc, decl) = insert(
        doc,
        Node::declare_rest(vec![(
            SitedRef::new(us, tied.clone()),
            SitedRef::new(mate, fname(mate, RoleSeg::Cap(CapEnd::End))),
        )]),
    );
    let (doc, u2) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: us,
            b: mate,
            declare: Some(decl),
        },
    );
    let ev = run(&doc);
    match ev.nodes.get(&u2) {
        Some(NodeResult::Failed(e)) => match &e.kind {
            NodeErrorKind::DeclareResolve { error } => match error.as_ref() {
                editor_core::resolve::ResolveError::Ambiguous {
                    name,
                    candidates,
                    tie,
                } => {
                    assert_eq!(name, &tied);
                    // One-name tie payload, stated honestly: the
                    // candidate list is the tied row itself; the
                    // WIDTH carries the recorded multiplicity.
                    assert_eq!(candidates, &vec![tied.clone()]);
                    assert!(tie.width >= 2, "recorded tie width, got {}", tie.width);
                    assert_eq!(tie.at, tied);
                    // Mid-evaluation the witness site IS the minting
                    // node: there is no evaluation-wide index to name
                    // a carrying node from.
                    assert_eq!(tie.node, tied.node);
                }
                other => panic!("expected Ambiguous, got {other:?}"),
            },
            other => panic!("expected DeclareResolve, got {other:?}"),
        },
        other => panic!("expected Failed, got {other:?}"),
    }
}

/// Review F6 (second seam-JUNCTION instance): the crossing slots with
/// the SECOND slot subtracted first — same junction class (a vertex
/// where three seam lines meet on the shared floor plane), different
/// evaluation order; the multi-Seam-segment junction names must
/// appear and the document must certify.
#[test]
fn crossing_slots_swapped_order_hits_the_junction_arm() {
    let doc = ProfileDoc::empty_derived("m4_pr5_declare", Tol::witness());
    let (doc, slab) = block(doc, (0.0, 3.0), (0.0, 3.0), 0.0, 1.0);
    let (doc, b2) = block(doc, (-1.0, 4.0), (1.0, 2.0), 0.5, 1.0);
    let (doc, s1) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a: slab,
            b: b2,
            declare: None,
        },
    );
    let floor2 = fname(
        s1,
        RoleSeg::FromB(fname(b2, RoleSeg::Cap(CapEnd::Start)).into()),
    );
    let (doc, b1) = block(doc, (1.0, 2.0), (-1.0, 4.0), 0.5, 1.0);
    let (doc, decl) = insert(
        doc,
        Node::declare_rest(vec![(
            SitedRef::new(s1, floor2),
            SitedRef::new(b1, fname(b1, RoleSeg::Cap(CapEnd::Start))),
        )]),
    );
    let (doc, s2) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a: s1,
            b: b1,
            declare: Some(decl),
        },
    );
    let ev = run(&doc);
    let BooleanValue::Body { body, contacts, .. } = boolean_value(&ev, s2) else {
        panic!("swapped crossing slots is a body");
    };
    assert_eq!(
        topo::mass_properties(body, Tol::witness()).unwrap().volume,
        9.0 - (3.0 * 0.5 + 3.0 * 0.5 - 0.5) * 1.0
    );
    assert_eq!(
        topo::validate_pseudomanifold(body, contacts, Tol::witness()),
        Ok(())
    );
    // The junction arm's product: a vertex named by ≥ 2 Seam
    // segments (the sorted line set).
    let junctions = ev
        .value(s2)
        .unwrap()
        .name_table
        .iter()
        .filter(|(n, _)| {
            n.kind == EntityKind::Vertex
                && n.path
                    .iter()
                    .filter(|seg| matches!(seg, RoleSeg::Seam { .. }))
                    .count()
                    >= 2
        })
        .count();
    assert!(junctions >= 1, "expected junction-named vertices");
}

/// **The declare door asks WHAT the pair denotes before it asks how
/// many entities answer to either name** — PORT-DOORS-1's rule
/// (`assembly::resolve_face`) at a second door.
///
/// A pair the v1 vocabulary has no step for is unsupported however
/// many entities answer to either name, so the refusal that names
/// both kinds must not be preempted by the tie. Two same-operand
/// faces are such a pair (v1 threads face pairs across operands
/// only); the rows below declare one with a TIED name in it and one
/// with two unique names, and pin that the two documents get the same
/// refusal. Before the order changed, the tied one was told to narrow
/// a reference that would still have had no step however narrow it
/// was made.
#[test]
fn an_unsupported_declared_pair_answers_its_kinds_with_a_tied_name_in_it() {
    use editor_core::Entry;

    // The symmetric U cutter's N2 tie.
    let (doc, us) = fixture::u_cutter_tie(ProfileDoc::empty_derived(
        "m4_pr5_declare_tie_before_kind",
        Tol::witness(),
    ));
    let ev1 = run(&doc);
    let table = ev1
        .value(us)
        .expect("the U subtract evaluates")
        .name_table
        .clone();
    let tied: StableName = table
        .iter()
        .find_map(|(n, e)| {
            (n.kind == EntityKind::Face && matches!(e, Entry::Tied(_))).then(|| n.clone())
        })
        .expect("the U fixture ties a face");
    let uniques: Vec<StableName> = table
        .iter()
        .filter(|(n, e)| n.kind == EntityKind::Face && matches!(e, Entry::Unique(_)))
        .map(|(n, _)| n.clone())
        .take(2)
        .collect();
    let [u1, u2] = <[StableName; 2]>::try_from(uniques).expect("two unique face names");
    // Without this the rows below would pass for the wrong reason:
    // the point is that a name SEVERAL entities answer to reaches the
    // pair's own refusal.
    assert!(
        matches!(table.lookup(&tied), Some(Entry::Tied(c)) if c.len() >= 2),
        "the declared name must really be tied for this row to test anything"
    );

    let (doc, mate) = block(doc, (0.0, 4.0), (0.0, 4.0), 6.0, 1.0);
    let mut doc = doc;
    let union_of = |doc: ProfileDoc, pair: (SitedRef, SitedRef)| {
        let (doc, decl) = insert(doc, Node::declare_rest(vec![pair]));
        insert(
            doc,
            Node::Boolean {
                op: BooleanOp::Union,
                a: us,
                b: mate,
                declare: Some(decl),
            },
        )
    };
    let with_tie;
    let all_unique;
    // Both names are rows of the SUBTRACT's table, which is operand
    // A — which is what makes each pair a same-operand face pair.
    (doc, with_tie) = union_of(
        doc,
        (
            SitedRef::new(us, tied.clone()),
            SitedRef::new(us, u1.clone()),
        ),
    );
    (doc, all_unique) = union_of(
        doc,
        (SitedRef::new(us, u1.clone()), SitedRef::new(us, u2.clone())),
    );
    let ev = run(&doc);

    let refusal = |node: RecipeNodeId| -> String {
        match ev.nodes.get(&node) {
            Some(NodeResult::Failed(e)) => match &e.kind {
                NodeErrorKind::DeclareUnsupportedPair {
                    kinds,
                    cross_operand,
                } => {
                    assert_eq!(
                        *kinds,
                        (EntityKind::Face, EntityKind::Face),
                        "the refusal names the pair's two kinds"
                    );
                    assert!(
                        !*cross_operand,
                        "both names resolve in the SAME operand — that is what makes the \
                         face pair unsupported"
                    );
                    e.kind.to_string()
                }
                other => {
                    panic!("an unsupported pair must refuse DeclareUnsupportedPair, got {other:?}")
                }
            },
            other => panic!("expected Failed, got {other:?}"),
        }
    };
    assert_eq!(
        refusal(with_tie),
        refusal(all_unique),
        "the tie makes no difference to a pair the vocabulary has no step for"
    );
}

/// **A tie on the first name waits behind every per-name fault on the
/// second** — the cross-name half of the declare door's order, and the
/// part the `ladder` does NOT decide (it ranks within one name's walk).
///
/// The rule is that the tie is the one per-name refusal the PAIR's
/// kind question outranks, and a pair question cannot be asked until
/// both names have landed; so `NodeGone` and `Vanished` on the SECOND
/// name are now raised where the first name's tie used to preempt
/// them. Both are visible in Python — `resolve_error_tag` moves from
/// `ambiguous` to `node_gone` and to `vanished` — so both are pinned
/// here rather than left to the PR body.
#[test]
fn a_tied_first_name_waits_behind_the_second_names_own_faults() {
    use editor_core::{DocEdit, Entry};

    let (doc, us) = fixture::u_cutter_tie(ProfileDoc::empty_derived(
        "m4_pr5_declare_cross_name_order",
        Tol::witness(),
    ));
    let ev1 = run(&doc);
    let table = ev1
        .value(us)
        .expect("the U subtract evaluates")
        .name_table
        .clone();
    let tied: StableName = table
        .iter()
        .find_map(|(n, e)| {
            (n.kind == EntityKind::Face && matches!(e, Entry::Tied(_))).then(|| n.clone())
        })
        .expect("the U fixture ties a face");
    assert!(
        matches!(table.lookup(&tied), Some(Entry::Tied(c)) if c.len() >= 2),
        "the first declared name must really be tied for this row to test anything"
    );

    // A third body, named by the pair and then DELETED: rung 1 on the
    // second name. Its cap is a FACE, so the pair is a supported
    // cross-operand face pair and the kind question does not preempt.
    let (doc, mate) = block(doc, (0.0, 4.0), (0.0, 4.0), 6.0, 1.0);
    let (doc, ghost) = block(doc, (0.0, 1.0), (0.0, 1.0), 20.0, 1.0);
    // A face name at a LIVE node that names no row there: rung 3.
    let absent = fname(us, RoleSeg::Lateral(fixture::no_piece()));
    assert!(
        table.lookup(&absent).is_none(),
        "the vanished probe must name no row, or it pins nothing"
    );

    let union_of = |doc: ProfileDoc, pair: (SitedRef, SitedRef)| {
        let (doc, decl) = insert(doc, Node::declare_rest(vec![pair]));
        insert(
            doc,
            Node::Boolean {
                op: BooleanOp::Union,
                a: us,
                b: mate,
                declare: Some(decl),
            },
        )
    };
    let mut doc = doc;
    let with_gone;
    let with_absent;
    (doc, with_gone) = union_of(
        doc,
        (
            SitedRef::new(us, tied.clone()),
            SitedRef::new(mate, fname(ghost, RoleSeg::Cap(CapEnd::End))),
        ),
    );
    (doc, with_absent) = union_of(
        doc,
        (SitedRef::new(us, tied.clone()), SitedRef::new(us, absent)),
    );
    let doc = doc
        .apply(
            &DocEdit::DeleteNode { id: ghost },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .expect("the ghost block is deletable")
        .doc;
    let ev = run(&doc);

    // The VARIANT, not a substring of `Debug` — and spelled with the
    // words `pncad`'s `resolve_error_tag` uses, because the tag is
    // what a Python caller branches on and is what moved.
    let rung = |node: RecipeNodeId| -> &'static str {
        use editor_core::resolve::ResolveError;
        match ev.nodes.get(&node) {
            Some(NodeResult::Failed(e)) => match &e.kind {
                NodeErrorKind::DeclareResolve { error } => match &**error {
                    ResolveError::NodeGone { .. } => "node_gone",
                    ResolveError::Vanished { .. } => "vanished",
                    ResolveError::Ambiguous { .. } => "ambiguous",
                },
                other => panic!(
                    "the second name's own fault must be raised, not the first name's tie: \
                     got {other:?}"
                ),
            },
            other => panic!("expected Failed, got {other:?}"),
        }
    };
    assert_eq!(
        rung(with_gone),
        "node_gone",
        "a deleted second name outranks the first name's tie"
    );
    assert_eq!(
        rung(with_absent),
        "vanished",
        "a second name that names nothing here outranks the first name's tie"
    );
}
