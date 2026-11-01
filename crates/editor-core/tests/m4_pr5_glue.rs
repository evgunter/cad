//! M4 PR 5 acceptance (spec D4–D6): coincident contacts glued
//! end-to-end through the recipe.
//!
//! - **D6.1**: a 3′ body (corner-kiss union) reused as an operand
//!   carries its kiss's contact record into the next union, which
//!   passes 3′.
//! - **D6.3, the #91 flush-plane narrative**: a coincident-plane pair
//!   glues (N3 `Merged` row minted); the decoupled variant is
//!   untouched.
//! - **D6.4 / PR 3 R13**: the #90 crossing-slots double subtract
//!   promoted to a recipe document — the first boolean-of-boolean
//!   naming/resolution corpus entry.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    BooleanValue, CapEnd, EntityKind, Node, ProfileDoc, RecipeNodeId, RoleSeg, StableName,
    ValuePayload,
};
use fixture::{fname, insert};
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

/// An axis-aligned block on its own xy, extruded `dz`: a root.
fn block(doc: ProfileDoc, x: (f64, f64), y: (f64, f64), dz: f64) -> (ProfileDoc, RecipeNodeId) {
    crate::blocks::block(doc, x, y, dz)
}

/// [`block`] placed against `target`, `z0` up: the block and its
/// placement, whose output is the operand.
fn block_at(
    doc: ProfileDoc,
    target: RecipeNodeId,
    x: (f64, f64),
    y: (f64, f64),
    z0: f64,
    dz: f64,
) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, b) = block(doc, x, y, dz);
    let (doc, at) = fixture::slid(doc, target, b, [0.0, 0.0, z0]);
    (doc, b, at)
}

fn boolean_value(
    ev: &editor_core::Evaluation<f64>,
    node: RecipeNodeId,
) -> (&BooleanValue<f64>, &topo::ContactRecords) {
    let value = ev.value(node).expect("boolean evaluated");
    match &value.payload {
        ValuePayload::Boolean(b) => (b, &value.carry.contacts),
        other => panic!("expected boolean, got {}", other.kind_name()),
    }
}

/// The corner-kiss base document: a = [0,1]³, b = [1,2]³, unioned
/// (the v-v kiss at (1,1,1) is DISCOVERED by the op and recorded in
/// the result's contacts). Returns (doc, a, b, union).
fn kiss_base(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId, RecipeNodeId, RecipeNodeId) {
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, _, b) = block_at(doc, a, (1.0, 2.0), (1.0, 2.0), 1.0, 1.0);
    let (doc, u) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
        },
    );
    (doc, a, b, u)
}

/// D6.1: a corner-kiss union reused as an operand carries its
/// operand-internal kiss into the next union's records, and the next
/// union passes 3′.
#[test]
fn a_reused_kiss_carries_its_record_into_the_next_union() {
    let (doc, a, _, base) = kiss_base(ProfileDoc::empty_derived("m4_pr5_declare", Tol::witness()));
    let (doc, _, mover) = block_at(doc, a, (1.5, 2.5), (1.5, 2.5), 1.5, 1.0);
    let (doc, u2) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![base.into(), mover.into()]),
        },
    );
    let ev = run(&doc);
    let (BooleanValue::Body { body, .. }, contacts) = boolean_value(&ev, u2) else {
        panic!("union is a body");
    };
    assert!(
        contacts.vertex_vertex().next().is_some(),
        "the kiss's vertex-vertex record"
    );
    assert_eq!(
        validate_pseudomanifold(body.body(), contacts, Tol::witness()),
        Ok(()),
        "the union over the reused kiss: 3′"
    );
}

/// D6.3, the #91 flush-plane narrative: the coincident-plane pair
/// GLUES — the margins decide the planes one carrier — the union names
/// the glued faces with N3 `Merged` rows, and the decoupled variant is
/// untouched.
#[test]
fn flush_plane_pair_glues() {
    let doc = ProfileDoc::empty_derived("m4_pr5_declare", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, _, b) = block_at(doc, a, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, u) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
        },
    );
    let ev = run(&doc);
    let (BooleanValue::Body { body, .. }, _) = boolean_value(&ev, u) else {
        panic!("the union is a body");
    };
    assert_eq!(
        topo::test_support::pinned_volume(
            "the measured body",
            &topo::test_support::finished(
                "the measured body",
                topo::Body::clone(body.body()),
                Tol::witness()
            ),
            Tol::witness()
        )
        .volume,
        1.5
    );
    assert_eq!(
        topo::validate::validate_geometric(body.body(), Tol::witness()),
        Ok(())
    );
    let merged = |kind| {
        crate::fixture::table(&ev, u)
            .iter()
            .filter(|(n, _)| n.kind == kind && matches!(n.path.first(), Some(RoleSeg::Merged(_))))
            .count()
    };
    assert_eq!(merged(EntityKind::Face), 4, "caps + flush y-walls glue");
    assert_eq!(merged(EntityKind::Edge), 4, "the rims between them join");

    // Decoupled variant (no coincident planes): a transversal union.
    let doc = ProfileDoc::empty_derived("m4_pr5_declare", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, _, b) = block_at(doc, a, (0.5, 1.5), (0.25, 0.75), 0.25, 1.25);
    let (doc, u) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
        },
    );
    let ev = run(&doc);
    let (BooleanValue::Body { body, .. }, _) = boolean_value(&ev, u) else {
        panic!("decoupled union is a body");
    };
    // 1 + (b = 1·0.5·1.25) − (overlap = 0.5·0.5·0.75) = 1.4375.
    assert_eq!(
        topo::test_support::pinned_volume(
            "the measured body",
            &topo::test_support::finished(
                "the measured body",
                topo::Body::clone(body.body()),
                Tol::witness()
            ),
            Tol::witness()
        )
        .volume,
        1.4375
    );
}

/// D6.4 / PR 3 R13: the #90 crossing-slots double subtract as a
/// recipe document — the first boolean-of-boolean corpus entry. The
/// slot floors are flush with each other (both z = 0.5 INSIDE the
/// slab), the tools' caps are flush with nothing (they straddle),
/// but each second-op slot floor meets the FIRST slot's floor in one
/// plane, and the subtract glues them.
#[test]
fn crossing_slots_recipe_document_evaluates_and_resolves() {
    let doc = ProfileDoc::empty_derived("m4_pr5_declare", Tol::witness());
    let (doc, slab) = block(doc, (0.0, 3.0), (0.0, 3.0), 1.0);
    let (doc, b1_block, b1) = block_at(doc, slab, (1.0, 2.0), (-1.0, 4.0), 0.5, 1.0);
    let (doc, s1) = insert(
        doc,
        Node::Subtract {
            from: slab.into(),
            tool: b1.into(),
        },
    );
    // Slot floor of s1 = FromB(b1's Cap(Start)); the second tool's
    // start cap lies in the SAME plane (z = 0.5).
    let floor1 = fname(
        s1,
        RoleSeg::From {
            read: fixture::out(&doc, b1),
            of: fname(b1_block, RoleSeg::Cap(CapEnd::Start)).into(),
        },
    );
    let (doc, _, b2) = block_at(doc, slab, (-1.0, 4.0), (1.0, 2.0), 0.5, 1.0);
    let (doc, s2) = insert(
        doc,
        Node::Subtract {
            from: s1.into(),
            tool: b2.into(),
        },
    );
    let ev = run(&doc);
    let (BooleanValue::Body { body, .. }, contacts) = boolean_value(&ev, s2) else {
        panic!("crossing slots is a body");
    };
    // #90's oracle: 9 − two 0.5-deep slots + shared middle.
    assert_eq!(
        topo::test_support::pinned_volume(
            "the measured body",
            &topo::test_support::finished(
                "the measured body",
                topo::Body::clone(body.body()),
                Tol::witness()
            ),
            Tol::witness()
        )
        .volume,
        9.0 - (3.0 * 0.5 + 3.0 * 0.5 - 0.5) * 1.0
    );
    assert_eq!(
        topo::validate_pseudomanifold(body.body(), contacts, Tol::witness()),
        Ok(())
    );
    // Resolution corpus seed (boolean-of-boolean DAG): the two slot
    // floors GLUED — the wrapped floor lineage retired
    // into the Merged row, fails typed, and OFFERS the merge; the
    // Merged row itself resolves on the final body (N3, live).
    let ctx = editor_core::resolve::RunCtx {
        doc: &doc,
        eval: &ev,
    };
    let wrapped_floor1 = fname(
        s2,
        RoleSeg::From {
            read: fixture::out(&doc, s1),
            of: floor1.into(),
        },
    );
    let merged_row: StableName = crate::fixture::table(&ev, s2)
        .iter()
        .find_map(|(n, _)| {
            matches!(n.path.first(), Some(RoleSeg::Merged(cs)) if cs.contains(&wrapped_floor1))
                .then(|| n.clone())
        })
        .expect("the glued floors mint a Merged row embedding the slot-floor lineage");
    match editor_core::resolve::resolve(
        ctx,
        ctx.eval.at_var(s2).expect("the final body's read"),
        &merged_row,
    )
    .expect("a body read")
    {
        editor_core::resolve::Resolution::Resolved(r) => assert_eq!(r.node, s2),
        other => panic!("the Merged floor must resolve on the final body: {other:?}"),
    }
    match editor_core::resolve::resolve(
        ctx,
        ctx.eval.at_var(s2).expect("the final body's read"),
        &wrapped_floor1,
    )
    .expect("a body read")
    {
        editor_core::resolve::Resolution::Failed(f) => assert!(
            f.offers.contains(&merged_row),
            "the retired floor lineage must offer its merge: {f:?}"
        ),
        other => panic!("expected the retired lineage to fail typed, got {other:?}"),
    }
}

/// Review F1, recipe door: flush caps on an ordinary partial overlap
/// (walls offset). Each cap pair meets along a seam bent at
/// the overlap's corner; the merge joins the pair and deletes the seam
/// edge left dangling with the corner, so each cap MERGES into one
/// octagon. The result must be tier-3 honest (no stale in-plane
/// descriptions) with the exact volume.
#[test]
fn l_corner_caps_merge_at_the_recipe_door_tier3_green() {
    let doc = ProfileDoc::empty_derived("m4_pr5_declare", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, _, b) = block_at(doc, a, (0.5, 1.5), (0.25, 1.25), 0.0, 1.0);
    let (doc, u) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
        },
    );
    let ev = run(&doc);
    let (BooleanValue::Body { body, .. }, contacts) = boolean_value(&ev, u) else {
        panic!("the flush-caps union is a body");
    };
    assert_eq!(
        topo::test_support::pinned_volume(
            "the measured body",
            &topo::test_support::finished(
                "the measured body",
                topo::Body::clone(body.body()),
                Tol::witness()
            ),
            Tol::witness()
        )
        .volume,
        1.625
    );
    // Both caps merged: two octagonal caps and eight walls, where two
    // unglued cap pairs would leave twelve faces.
    assert_eq!(
        body.body().faces().count(),
        10,
        "each cap pair merged into one"
    );
    assert_eq!(
        topo::validate::validate_geometric(body.body(), Tol::witness()),
        Ok(()),
        "tier 3"
    );
    assert_eq!(
        topo::validate_pseudomanifold(body.body(), contacts, Tol::witness()),
        Ok(())
    );
    // Review F6: the merged caps keep, on their boundary, the vertices
    // where the two blocks' walls cross, each named as the one crossing
    // of two wall edges (no junction composition); the bent seam's
    // corner, the one vertex the merge deletes, is not among them.
    // Assert they exist.
    let wall_crossings = crate::fixture::table(&ev, u)
        .iter()
        .filter(|(n, _)| {
            n.kind == EntityKind::Vertex
                && matches!(n.path.as_slice(), [RoleSeg::EdgeCrossing { .. }])
        })
        .count();
    assert!(
        wall_crossings >= 2,
        "expected the wall edges' crossings to be named, got {wall_crossings}"
    );
}

/// The crossing slots with the SECOND slot subtracted first. Where the
/// first slot's floor edge (y = 1, z = 0.5) meets the second slot's
/// bottom rim (x = 1, z = 0.5), the vertex lies on both operand edges,
/// so it is the edge crossing of those two edges (N2), each with its
/// sense: the floor edge enters or leaves the second slot, and the
/// second slot's rim lies in the first cavity's closed body on both
/// sides, on its floor. Not a junction of seam lines, which is a vertex
/// no operand edge passes through. The second
/// slot's rim reaches the result in pieces split off around a stretch
/// the subtract killed, and is named a piece of that rim.
#[test]
fn crossing_slots_swapped_order_names_the_floor_corner_for_both_edges() {
    let doc = ProfileDoc::empty_derived("m4_pr5_declare", Tol::witness());
    let (doc, slab) = block(doc, (0.0, 3.0), (0.0, 3.0), 1.0);
    let (doc, _, b2) = block_at(doc, slab, (-1.0, 4.0), (1.0, 2.0), 0.5, 1.0);
    let (doc, s1) = insert(
        doc,
        Node::Subtract {
            from: slab.into(),
            tool: b2.into(),
        },
    );
    let (doc, _, b1) = block_at(doc, slab, (1.0, 2.0), (-1.0, 4.0), 0.5, 1.0);
    let (doc, s2) = insert(
        doc,
        Node::Subtract {
            from: s1.into(),
            tool: b1.into(),
        },
    );
    let ev = run(&doc);
    let (BooleanValue::Body { body, .. }, contacts) = boolean_value(&ev, s2) else {
        panic!("swapped crossing slots is a body");
    };
    assert_eq!(
        topo::test_support::pinned_volume(
            "the measured body",
            &topo::test_support::finished(
                "the measured body",
                topo::Body::clone(body.body()),
                Tol::witness()
            ),
            Tol::witness()
        )
        .volume,
        9.0 - (3.0 * 0.5 + 3.0 * 0.5 - 0.5) * 1.0
    );
    assert_eq!(
        topo::validate_pseudomanifold(body.body(), contacts, Tol::witness()),
        Ok(())
    );
    let corner = crate::fixture::table(&ev, s2)
        .iter()
        .filter_map(|(n, e)| match (n.kind, e) {
            (EntityKind::Vertex, editor_core::Entry::Unique(r)) => match r.key {
                editor_core::EntityKey::Vertex(v) => {
                    let p = body
                        .body()
                        .get_point(body.body().get_vertex(v).unwrap().point)
                        .unwrap();
                    ((p.x - 1.0).abs() < 1e-12
                        && (p.y - 1.0).abs() < 1e-12
                        && (p.z - 0.5).abs() < 1e-12)
                        .then(|| n.clone())
                }
                _ => None,
            },
            _ => None,
        })
        .collect::<Vec<_>>();
    let [corner] = corner.as_slice() else {
        panic!("one vertex at the floor corner (1, 1, 0.5), got {corner:?}");
    };
    assert!(
        matches!(
            corner.path.as_slice(),
            [RoleSeg::EdgeCrossing(run)] if matches!(
                run.as_slice(),
                [x, y] if x.line.kind == EntityKind::Edge
                    && y.line.kind == EntityKind::Edge
                    && matches!(
                        (&x.sense, &y.sense),
                        (s, editor_core::Sense::Inside)
                            | (editor_core::Sense::Inside, s) if s.crosses()
                    )
            )
        ),
        "the floor corner is the edge crossing of two operand edges, one \
         slot's floor edge crossing the other's body and the other's rim \
         inside the first's: {corner}"
    );
}
