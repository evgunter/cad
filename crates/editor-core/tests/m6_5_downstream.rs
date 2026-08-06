//! **The dead end, demonstrably closed** (M6-5 PR-2).
//!
//! For a milestone the whole-body fillet door emitted the EMPTY name
//! table: its rebuild mints every face fresh, and it kept no birth
//! records, so there was nothing to emit honestly. That was loud
//! rather than silent — every downstream reference into such a body
//! failed to resolve, and an appearance record targeting one was a
//! typed loss — but it was still a dead end: no op could be written
//! that consumed a filleted body BY NAME.
//!
//! This file is the proof it is gone. A boolean over a whole-body-
//! filleted die resolves names THROUGH the fillet's table: its
//! `Declare` pairs name faces the fillet minted, the coincidence is
//! threaded into the kernel, and the boolean's own names carry the
//! fillet's names inside them (`FromA(FromTarget(Cap(Top)))` and
//! friends). Nothing here is a fillet test — it is a test that a
//! fillet result is now ORDINARY: nameable, referenceable, and
//! composable like any other body.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

use std::collections::BTreeSet;

use editor_core::{
    BooleanOp, CancelToken, CapEnd, Dimension, DocEdit, EntityKind, EvalOptions, Expr, Node,
    NodeResult, ProfileDesc, ProfileDoc, RecipeNodeId, RoleSeg, StableName, ValuePayload, apply,
    evaluate,
};
use fixture::prism_edges;

fn len(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).expect("a length literal")
}

fn eval(doc: &ProfileDoc) -> editor_core::Evaluation<f64> {
    evaluate::<f64>(doc, None, &CancelToken::new(), &EvalOptions::default())
}

fn insert(doc: &ProfileDoc, node: Node<ProfileDesc>) -> (ProfileDoc, RecipeNodeId) {
    let a = apply(doc, &DocEdit::InsertNode { node }).expect("the fixture builds");
    let id = a.record.minted.expect("a minted id");
    (a.doc, id)
}

/// An axis-aligned box `[x0,x1] × [y0,y1] × [z0, z0+dz]`.
fn block(
    doc: &ProfileDoc,
    x: (f64, f64),
    y: (f64, f64),
    z0: f64,
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let loops = vec![vec![(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)]];
    let (doc, p) = insert(
        doc,
        Node::Profile(fixture::desc(
            [0.0, 0.0, z0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            loops,
        )),
    );
    insert(
        &doc,
        Node::Extrude {
            profile: p,
            distance: len(dz),
        },
    )
}

fn table_of(
    ev: &editor_core::Evaluation<f64>,
    node: RecipeNodeId,
) -> std::sync::Arc<editor_core::NameTable> {
    match ev.nodes.get(&node) {
        Some(NodeResult::Ok(v)) => std::sync::Arc::clone(&v.name_table),
        other => panic!("node {node:?} did not evaluate: {other:?}"),
    }
}

/// The die blank: a unit cube with EVERY edge blended — the whole-body
/// door, the one that used to name nothing. Returns the document, the
/// extrude and the fillet.
fn filleted_blank() -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty();
    let (doc, cube) = block(&doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, blank) = insert(&doc, Node::fillet(cube, len(0.125), prism_edges(cube, 4)));
    (doc, cube, blank)
}

/// **The whole-body door names its result.** Twenty-six faces,
/// forty-eight edges, twenty-four vertices, plus the body row — all
/// named, none by matching.
#[test]
fn a_whole_body_fillet_emits_a_full_name_table() {
    let (doc, _, blank) = filleted_blank();
    let ev = eval(&doc);
    let table = table_of(&ev, blank);
    let NodeResult::Ok(v) = ev.nodes.get(&blank).expect("the fillet") else {
        panic!("the fillet evaluates")
    };
    let ValuePayload::Body(body) = &v.payload else {
        panic!("a body")
    };
    assert_eq!(body.faces().count(), 26);
    assert_eq!(body.edges().count(), 48);
    assert_eq!(body.vertices().count(), 24);
    assert_eq!(
        table.len(),
        26 + 48 + 24 + 1,
        "every entity named, plus the body row"
    );
    assert!(!table.is_empty(), "the empty-table dead end is gone");

    // The roles are the fillet vocabulary, and a shrunk support reads
    // as `FromTarget` — the SAME name the surgery door gives it,
    // where that face is a survivor rather than a fresh mint.
    let mut supports = 0usize;
    for (n, _) in table.iter() {
        match n.path.first().expect("a role path") {
            RoleSeg::OutputBody
            | RoleSeg::BlendFace(_)
            | RoleSeg::CornerFace(_)
            | RoleSeg::TrimEdge { .. }
            | RoleSeg::FootVertex { .. }
            | RoleSeg::CornerArc { .. } => {}
            RoleSeg::FromTarget(_) => supports += 1,
            other => panic!("a non-fillet role in a fillet table: {other:?}"),
        }
    }
    assert_eq!(supports, 6, "one shrunk support per source face");
}

/// **The appearance store resolves a fillet-minted face.**
///
/// This is the exact loss PR-1's own docs named: "an appearance record
/// targeting one is a typed loss". Here a colour is attached to a
/// BLEND face — a face that exists only because the fillet minted it —
/// and the evaluation resolves it losslessly onto that entity.
///
/// The appearance store is a production consumer of name resolution,
/// not a test harness: this is the dead end closed where it was
/// stated.
#[test]
fn an_appearance_record_on_a_fillet_minted_face_resolves() {
    let (doc, cube, blank) = filleted_blank();
    // The blend over the extrude's first cap-wall rim.
    let blend = StableName {
        kind: EntityKind::Face,
        node: blank,
        path: vec![RoleSeg::BlendFace(Box::new(StableName {
            kind: EntityKind::Edge,
            node: cube,
            path: vec![RoleSeg::RimEdge(
                CapEnd::Top,
                editor_core::ProfileEdgeRef {
                    loop_index: 0,
                    segment: 0,
                },
            )],
        }))],
    };
    assert!(
        table_of(&eval(&doc), blank).lookup(&blend).is_some(),
        "the blend is named by the fillet"
    );
    let doc = apply(
        &doc,
        &DocEdit::SetAppearance {
            name: blend.clone(),
            attr: editor_core::Attr::Color(editor_core::Rgba8::opaque(9, 8, 7)),
        },
    )
    .expect("the appearance edit applies")
    .doc;

    let ev = eval(&doc);
    assert!(
        ev.appearance.losses.is_empty(),
        "a fillet-minted face must not be a typed appearance loss: {:?}",
        ev.appearance.losses
    );
    let rows = ev
        .appearance
        .for_node(blank)
        .expect("the fillet node carries resolved appearance");
    assert_eq!(rows.len(), 1, "exactly the one attributed face");
    let ent = *rows.keys().next().expect("a row");
    assert_eq!(
        table_of(&ev, blank).name_of(&ent),
        Some(&blend),
        "the colour landed on the blend it names"
    );
}

/// **The resolve ladder resolves fillet-minted names.** The hit-test /
/// reference door (M4 PR 4) answers `Resolved` for every role the
/// whole-body fillet mints — blend, octant, trimline, corner arc,
/// foot, and the shrunk support — so a reference INTO a filleted body
/// is now an ordinary reference.
#[test]
fn every_fillet_minted_role_resolves_through_the_ladder() {
    use editor_core::resolve::{Resolution, RunCtx, resolve};
    let (doc, _, blank) = filleted_blank();
    let ev = eval(&doc);
    let table = table_of(&ev, blank);
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for (name, _) in table.iter() {
        let role = match name.path.first().expect("a role path") {
            RoleSeg::OutputBody => "body",
            RoleSeg::FromTarget(_) => "support",
            RoleSeg::BlendFace(_) => "blend",
            RoleSeg::CornerFace(_) => "octant",
            RoleSeg::TrimEdge { .. } => "trim",
            RoleSeg::CornerArc { .. } => "arc",
            RoleSeg::FootVertex { .. } => "foot",
            other => panic!("a non-fillet role in a fillet table: {other:?}"),
        };
        seen.insert(role);
        let ctx = RunCtx {
            doc: &doc,
            eval: &ev,
        };
        match resolve(ctx, name) {
            Resolution::Resolved { .. } => {}
            other => panic!("{role} name {name:?} did not resolve: {other:?}"),
        }
    }
    assert_eq!(
        seen.len(),
        7,
        "every whole-body fillet role is exercised, got {seen:?}"
    );
}

/// **The boolean consumer, ATTEMPTED and pinned** (M6-5 PR-2
/// deviation 1).
///
/// The spec's §5 row is a boolean over a whole-body-filleted body.
/// The NAMING half is ready — the two rows above show the fillet's
/// names resolving and composing — but the kernel's boolean cannot
/// take a body carrying sphere OCTANTS at all: it refuses
/// `FallbackExtentUnsupported` on the first octant it meets, needing
/// the closed-group extent certificate (PR 9c) that no per-face
/// chart-trim extent supplies. The refusal fires even for a DISJOINT
/// second operand, so it is not about the cut.
///
/// That frontier predates M6-5 and is untouched by it. This row pins
/// it executed — the `m6_composed_node.rs` idiom, which is exactly how
/// M6-5's own blocker was kept honest for a milestone — so the day the
/// extent lane reaches sphere groups, this test fails and the real
/// boolean row replaces it.
#[test]
fn a_boolean_over_a_filleted_body_still_meets_the_extent_frontier() {
    let (doc, _, blank) = filleted_blank();
    // Disjoint, far away: nothing to intersect, nothing to declare.
    let (doc, far) = block(&doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, union) = insert(
        &doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: blank,
            b: far,
            declare: None,
        },
    );
    let ev = eval(&doc);
    let Some(NodeResult::Failed(e)) = ev.nodes.get(&union) else {
        panic!(
            "the extent frontier moved — replace this pin with the real \
             downstream-of-fillet boolean row (M6-5 PR-2 deviation 1)"
        );
    };
    match &e.kind {
        editor_core::NodeErrorKind::Boolean(topo::BooleanError::FallbackExtentUnsupported {
            ..
        }) => {}
        other => panic!("expected the extent frontier, got {other:?}"),
    }
    // The naming side is nonetheless ready: the operand's table is
    // full, so the moment the kernel can take the body, the boolean
    // emitter has everything it needs.
    assert_eq!(table_of(&ev, blank).len(), 26 + 48 + 24 + 1);
}

/// **The reference survives a rebuild.** A parameter edit upstream of
/// the fillet recomputes its whole cone; the downstream `Declare`
/// still resolves, because the fillet's names are a function of the
/// extrude's names and those did not move.
#[test]
fn the_downstream_reference_survives_an_upstream_bump() {
    let (doc, cube, blank) = filleted_blank();
    let blank_top = StableName {
        kind: EntityKind::Face,
        node: blank,
        path: vec![RoleSeg::FromTarget(Box::new(StableName {
            kind: EntityKind::Face,
            node: cube,
            path: vec![RoleSeg::Cap(CapEnd::Top)],
        }))],
    };
    let before = table_of(&eval(&doc), blank);
    let bumped = apply(
        &doc,
        &DocEdit::SetParam {
            node: cube,
            slot: editor_core::SlotId::Distance,
            expr: len(1.25),
        },
    )
    .expect("the bump applies")
    .doc;
    let after = table_of(&eval(&bumped), blank);
    assert!(
        after.lookup(&blank_top).is_some(),
        "the downstream reference still resolves after the bump"
    );
    let names =
        |t: &editor_core::NameTable| t.iter().map(|(n, _)| n.clone()).collect::<BTreeSet<_>>();
    assert_eq!(
        names(&before),
        names(&after),
        "a stretch mints no entity, so the fillet's names are unchanged"
    );
}

// ------------------------------------------------------------------
// `all_edges`, the whole-body materializer (review F-1).
// ------------------------------------------------------------------

/// **`all_edges` is what a caller uses to select everything.** The
/// §0 ruling removed the live "All" variant on purpose: a live "all"
/// would silently GROW when an upstream edit adds an edge, which is
/// exactly the staleness a frozen selection exists to prevent. This
/// helper closes the gap the honest way — evaluate, take the set as it
/// stands, STORE it — and from that moment it behaves like any other
/// authored selection.
///
/// The row pins the equivalence the corpus depends on: what
/// `all_edges` hands back for an extruded square IS the set
/// `die_fillet` authors by hand through `prism_edges`.
#[test]
fn all_edges_materializes_exactly_the_authored_whole_body_set() {
    let doc = ProfileDoc::empty();
    let (doc, cube) = block(&doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let ev = eval(&doc);

    let materialized = editor_core::all_edges(&ev, cube);
    let mut authored = prism_edges(cube, 4);
    authored.sort();
    assert_eq!(materialized.len(), 12, "a box has twelve edges");
    assert_eq!(
        materialized, authored,
        "`all_edges` and the corpus's authored `prism_edges` agree"
    );
    assert!(
        materialized.windows(2).all(|w| w[0] < w[1]),
        "canonical, ready for `Node::fillet`"
    );

    // And it is a MATERIALIZER, not a query: the stored set does not
    // follow the document. Feeding it to a fillet and then bumping the
    // extrude leaves the selection exactly as stored.
    let (doc, blank) = insert(&doc, Node::fillet(cube, len(0.125), materialized.clone()));
    let bumped = apply(
        &doc,
        &DocEdit::SetParam {
            node: cube,
            slot: editor_core::SlotId::Distance,
            expr: len(1.25),
        },
    )
    .expect("the bump applies")
    .doc;
    let stored = match bumped.node(blank) {
        Some(Node::Fillet { selection, .. }) => selection.clone(),
        other => panic!("expected a fillet, got {other:?}"),
    };
    assert_eq!(stored, materialized, "the materialized set froze");
    match eval(&bumped).nodes.get(&blank) {
        Some(NodeResult::Ok(_)) => {}
        other => panic!("the frozen whole-body selection must still build, got {other:?}"),
    }
}

/// The empty case is handed back as empty rather than guessed at: a
/// node with no value, no table, or no edges yields no names, and the
/// FILLET is where that refuses (`FilletSelectionEmpty`) — one door
/// for the refusal, not two.
#[test]
fn all_edges_of_a_nameless_node_is_empty() {
    let doc = ProfileDoc::empty();
    let (doc, p) = insert(
        &doc,
        Node::Profile(fixture::desc(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
        )),
    );
    let ev = eval(&doc);
    // A profile node has no output body and so an empty table.
    assert!(editor_core::all_edges(&ev, p).is_empty());
    // A node that is not in the evaluation at all: also empty.
    assert!(editor_core::all_edges(&ev, RecipeNodeId(999)).is_empty());
}
