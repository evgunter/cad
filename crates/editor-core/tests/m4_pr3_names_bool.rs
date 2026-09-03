//! M4 PR 3 naming tests, part 2 (spec D3/D5/D6): boolean roles
//! (FromA/FromB/Seam), N2 discriminators (SideOf sign vectors,
//! OrderAlong sub-edge ranks), the genuine-tie fixture, and
//! discriminator-flip localization (counted).
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unreachable
)]

use crate::fixture;

use editor_core::{
    BooleanOp, CancelToken, CapEnd, EntityKind, Entry, EvalOptions, Evaluation, NameTable, Node,
    ProfileDoc, Qualifier, RecipeNodeId, RoleSeg, StableName, evaluate,
};
use fixture::{ang, declare_x_offset_flush, insert, len, on_frame, scl};
use geom_core::Tol;

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

fn table(ev: &Evaluation<f64>, id: RecipeNodeId) -> &NameTable {
    &ev.value(id)
        .unwrap_or_else(|| panic!("node {id:?} has no value: {:?}", ev.nodes.get(&id)))
        .name_table
}

fn name1(kind: EntityKind, node: RecipeNodeId, seg: RoleSeg) -> StableName {
    StableName {
        kind,
        node,
        path: vec![seg],
    }
}

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

// ---- FromA/FromB + Seam + OrderAlong on the overlapping union. ----

#[test]
fn union_names_operand_descent_seams_and_ordered_rim_fragments() {
    let doc = ProfileDoc::empty_derived("m4_pr3_names_bool", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, decl) = declare_x_offset_flush(doc, a, b);
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
    let t = table(&ev, u);
    // Body row.
    assert!(
        t.lookup(&name1(EntityKind::Body, u, RoleSeg::OutputBody))
            .is_some()
    );
    // M4 PR 5 (N3/D5, the Merged lane LIVE): the declared flush caps
    // GLUE — the operands' top caps retire into one `Merged` row whose
    // constituents are exactly the two FromX-wrapped cap names, sorted.
    let mut cap_constituents = vec![
        name1(
            EntityKind::Face,
            u,
            RoleSeg::FromA(Box::new(name1(
                EntityKind::Face,
                a,
                RoleSeg::Cap(CapEnd::Top),
            ))),
        ),
        name1(
            EntityKind::Face,
            u,
            RoleSeg::FromB(Box::new(name1(
                EntityKind::Face,
                b,
                RoleSeg::Cap(CapEnd::Top),
            ))),
        ),
    ];
    cap_constituents.sort_unstable();
    assert!(
        matches!(
            t.lookup(&name1(
                EntityKind::Face,
                u,
                RoleSeg::Merged(cap_constituents)
            )),
            Some(Entry::Unique(_))
        ),
        "missing Merged top-cap row"
    );
    // Four Merged faces total (both caps + both flush y-walls); the
    // x-extreme walls survive under their FromX wraps unmerged.
    let merged_rows = t
        .iter()
        .filter(|(n, _)| matches!(n.path.first(), Some(RoleSeg::Merged(_))))
        .count();
    assert_eq!(merged_rows, 4);
    for (node, seg, wrap_a) in [(a, 3u32, true), (b, 1u32, false)] {
        let inner = name1(
            EntityKind::Face,
            node,
            RoleSeg::Lateral(editor_core::ProfileEdgeRef {
                loop_index: 0,
                segment: seg,
            }),
        );
        let seg = if wrap_a {
            RoleSeg::FromA(Box::new(inner))
        } else {
            RoleSeg::FromB(Box::new(inner))
        };
        assert!(
            matches!(
                t.lookup(&name1(EntityKind::Face, u, seg)),
                Some(Entry::Unique(_))
            ),
            "missing surviving x-wall of {node:?}"
        );
    }
    // Cut rims carry OrderAlong ranks 0 and 1 (never bare indices —
    // ordinal positions under the named order-along predicate).
    let ranked = t
        .iter()
        .filter(|(n, _)| {
            matches!(
                n.path.last(),
                Some(RoleSeg::Fragment(Qualifier::OrderAlong { of: 2, .. }))
            )
        })
        .count();
    assert_eq!(
        ranked, 8,
        "ranked rim fragments (per-operand rims cut in two)"
    );
    // Seam vertices exist, with operand-name arguments.
    let seams = t
        .iter()
        .filter(|(n, _)| {
            n.kind == EntityKind::Vertex && matches!(n.path.first(), Some(RoleSeg::Seam { .. }))
        })
        .count();
    assert!(seams >= 4, "expected seam vertices, got {seams}");
    // No ties anywhere in this asymmetric union.
    assert!(t.iter().all(|(_, e)| matches!(e, Entry::Unique(_))));
}

// ---- SideOf sign vectors on the through-slot subtract. ----

#[test]
fn slot_subtract_discriminates_cap_fragments_by_side_of_vectors() {
    let doc = ProfileDoc::empty_derived("m4_pr3_names_bool", Tol::witness());
    // A: 3×3×1 block; B: a slot crossing the top cap fully in y.
    let (doc, a) = block(doc, (0.0, 3.0), (0.0, 3.0), 0.0, 1.0);
    let (doc, b) = block(doc, (1.0, 2.0), (-1.0, 4.0), 0.5, 1.0);
    let (doc, sub) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a,
            b,
            declare: None,
        },
    );
    let ev = run(&doc);
    let t = table(&ev, sub);
    let top = name1(EntityKind::Face, a, RoleSeg::Cap(CapEnd::Top));
    // Exactly two fragments of A's top cap, SideOf-qualified, with
    // DISTINCT vectors (each Unique — no tie: the slot walls
    // discriminate).
    let frags: Vec<&StableName> = t
        .iter()
        .filter_map(|(n, e)| {
            let is_frag = n.kind == EntityKind::Face
                && matches!(
                    n.path.first(),
                    Some(RoleSeg::FromA(inner)) if **inner == top
                )
                && matches!(n.path.get(1), Some(RoleSeg::Fragment(Qualifier::SideOf(_))));
            (is_frag && matches!(e, Entry::Unique(_))).then_some(n)
        })
        .collect();
    assert_eq!(
        frags.len(),
        2,
        "expected two SideOf-qualified cap fragments"
    );
    assert_ne!(frags[0], frags[1]);
    // The vectors' partners are B lateral names (recipe-covariant
    // references), and every verdict is a definite sign.
    for f in frags {
        let Some(RoleSeg::Fragment(Qualifier::SideOf(vec))) = f.path.get(1) else {
            unreachable!()
        };
        assert!(!vec.is_empty());
        for (partner, _verdict) in vec {
            assert_eq!(partner.node, b, "partner is not a B-operand carrier");
        }
    }
}

// ---- The genuine tie (D6): a U cutter through one wall. ----

#[test]
fn symmetric_u_cutter_fragments_tie_and_naming_stays_total() {
    let doc = ProfileDoc::empty_derived("m4_pr3_names_bool", Tol::witness());
    // A: 4×4×4 block.
    let (doc, a) = block(doc, (0.0, 4.0), (0.0, 4.0), 0.0, 4.0);
    // B: U-shaped prongs along −x, base OUTSIDE A (x ∈ [5,6]), prong
    // tips crossing A's wall x = 4; z ∈ [1,3] (inside A). B's caps
    // (z = 1 and z = 3, U-shaped, horizontal) are each cut by A's
    // wall x = 4 into TWO prong fragments on the SAME side of the
    // only cutting carrier — no covariant qualifier separates them:
    // the N2 tie, recorded, naming total.
    let (doc, p) = on_frame(
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
    let (doc, b) = insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(2.0),
        },
    );
    let (doc, sub) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a,
            b,
            declare: None,
        },
    );
    let ev = run(&doc);
    let t = table(&ev, sub);
    // The tie marks: at least one tied name, each with exactly two
    // candidates, and both caps' prong fragments are the tied ones.
    let ties: Vec<(&StableName, &Entry)> = t
        .iter()
        .filter(|(_, e)| matches!(e, Entry::Tied(_)))
        .collect();
    assert!(!ties.is_empty(), "expected N2 ties, found none");
    for (n, e) in &ties {
        let Entry::Tied(cands) = e else {
            unreachable!()
        };
        assert_eq!(cands.len(), 2, "tie should have 2 candidates: {n:?}");
        assert!(
            matches!(n.path.first(), Some(RoleSeg::FromB(_))),
            "tie should be on B-cap fragments: {n:?}"
        );
    }
}

// ---- Flip localization (D5), node-granular and counted. ----

#[test]
fn no_flip_translation_edit_leaves_every_table_identical() {
    // B placed by a Transform whose translation is the edited knob.
    let build = |tx: f64| {
        let doc = ProfileDoc::empty_derived("m4_pr3_names_bool", Tol::witness());
        let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
        let (doc, b0) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
        let (doc, decl) = declare_x_offset_flush(doc, a, b0);
        let (doc, tb) = insert(
            doc,
            Node::Transform {
                input: b0,
                translation: [len(tx), len(0.0), len(0.0)],
                rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
                rotation_angle: ang(0.0),
            },
        );
        let (doc, u) = insert(
            doc,
            Node::Boolean {
                op: BooleanOp::Union,
                a,
                b: tb,
                declare: Some(decl),
            },
        );
        (doc, u)
    };
    // 0.5 → 0.25: still overlapping, same verdict vector ⇒ N4 demands
    // IDENTICAL tables everywhere (names AND keys).
    let (doc1, u1) = build(0.5);
    let (doc2, u2) = build(0.25);
    assert_eq!(u1, u2);
    let ev1 = run(&doc1);
    let ev2 = run(&doc2);
    let mut changed = 0usize;
    for id in &ev1.order {
        if table(&ev1, *id) != table(&ev2, *id) {
            changed += 1;
        }
    }
    assert_eq!(changed, 0, "no-flip motion changed {changed} tables");
}

#[test]
fn flip_changes_exactly_the_boolean_nodes_table() {
    let build = |tx: f64| {
        let doc = ProfileDoc::empty_derived("m4_pr3_names_bool", Tol::witness());
        let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
        let (doc, b0) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
        let (doc, decl) = declare_x_offset_flush(doc, a, b0);
        let (doc, tb) = insert(
            doc,
            Node::Transform {
                input: b0,
                translation: [len(tx), len(0.0), len(0.0)],
                rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
                rotation_angle: ang(0.0),
            },
        );
        let (doc, u) = insert(
            doc,
            Node::Boolean {
                op: BooleanOp::Union,
                a,
                b: tb,
                declare: Some(decl),
            },
        );
        (doc, u)
    };
    // 0.5 (overlapping, Seamed) → 2.5 (disjoint, Assembly): verdicts
    // flip AT THE BOOLEAN; every upstream derivation is untouched, so
    // exactly one node's table may change (counted, not vibes).
    let (doc1, u1) = build(0.5);
    let (doc2, u2) = build(2.5);
    let ev1 = run(&doc1);
    let ev2 = run(&doc2);
    let changed: Vec<RecipeNodeId> = ev1
        .order
        .iter()
        .copied()
        .filter(|id| table(&ev1, *id) != table(&ev2, *id))
        .collect();
    assert_eq!(changed, vec![u1], "flip cone wider than the boolean");
    assert_eq!(u1, u2);
    // And the flipped table's names differ in SHAPE: the disjoint
    // union has no fragments at all.
    assert!(
        table(&ev2, u2)
            .iter()
            .all(|(n, _)| { !matches!(n.path.last(), Some(RoleSeg::Fragment(_))) })
    );
}
