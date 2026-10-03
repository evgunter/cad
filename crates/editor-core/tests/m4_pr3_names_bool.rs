//! M4 PR 3 naming tests, part 2 (spec D3/D5/D6): boolean roles
//! (FromA/FromB/Seam), N2 discriminators (`Borders` wall sets,
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
    BooleanOp, CancelToken, CapEnd, EntityKind, Entry, EvalOptions, Evaluation, Node, ProfileDoc,
    Qualifier, RecipeNodeId, RoleSeg, StableName, evaluate,
};
use fixture::{
    ang, declare_x_offset_flush, declare_x_offset_flush_at, insert, len, minted, on_frame, scl,
    table,
};
use geom_core::Tol;

/// Evaluates, and holds every table the run produced to the N3
/// flatness rule on the way out. A tripwire over this suite's merged
/// rows, not the guard: the mint refuses a nested constituent before
/// a table is published, and the rows that carry the rule are
/// `docm8_flat_merged`'s (the corpus walk and the mint-site rows).
fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    let ev = evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    fixture::assert_no_nested_merged(&ev);
    ev
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
fn union_names_operand_descent_seams_and_rim_pieces_by_their_ends() {
    let doc = ProfileDoc::empty_derived("m4_pr3_names_bool", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let decl = declare_x_offset_flush(&doc, a, b);
    let (doc, u) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b,
            declare: decl,
        },
    );
    let ev = run(&doc);
    let t = table(&ev, u);
    // Body row.
    assert!(
        t.lookup(&minted(EntityKind::Body, u, RoleSeg::OutputBody))
            .is_some()
    );
    // M4 PR 5 (N3/D5, the Merged lane LIVE): the declared flush caps
    // GLUE — the operands' end caps retire into one `Merged` row whose
    // constituents are exactly the two FromX-wrapped cap names, sorted.
    let mut cap_constituents = vec![
        minted(
            EntityKind::Face,
            u,
            RoleSeg::FromA(minted(EntityKind::Face, a, RoleSeg::Cap(CapEnd::End)).into()),
        ),
        minted(
            EntityKind::Face,
            u,
            RoleSeg::FromB(minted(EntityKind::Face, b, RoleSeg::Cap(CapEnd::End)).into()),
        ),
    ];
    cap_constituents.sort_unstable();
    assert!(
        matches!(
            t.lookup(&minted(
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
        let inner = minted(
            EntityKind::Face,
            node,
            RoleSeg::Lateral(crate::fixture::piece(&doc, node, 0, seg as usize).into()),
        );
        let seg = if wrap_a {
            RoleSeg::FromA(inner.into())
        } else {
            RoleSeg::FromB(inner.into())
        };
        assert!(
            matches!(
                t.lookup(&minted(EntityKind::Face, u, seg)),
                Some(Entry::Unique(_))
            ),
            "missing surviving x-wall of {node:?}"
        );
    }
    // Cut rims are told apart by their ends (never bare indices).
    let pieces = t
        .iter()
        .filter(|(n, _)| {
            matches!(
                n.path.last(),
                Some(RoleSeg::Fragment(Qualifier::Ends(ends))) if ends.len() == 2
            )
        })
        .count();
    assert_eq!(
        pieces, 8,
        "rim pieces named by their ends (per-operand rims cut in two)"
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

// ---- Borders wall sets on the through-slot subtract. ----

#[test]
fn slot_subtract_names_cap_fragments_by_the_walls_they_border() {
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
            declare: Vec::new(),
        },
    );
    let ev = run(&doc);
    let t = table(&ev, sub);
    let end = minted(EntityKind::Face, a, RoleSeg::Cap(CapEnd::End));
    // Exactly two pieces of A's end cap, `Borders`-qualified, with
    // DISTINCT wall sets (each Unique — no tie: each borders its own
    // slot wall).
    let frags: Vec<&StableName> = t
        .iter()
        .filter_map(|(n, e)| {
            let is_frag = n.kind == EntityKind::Face
                && matches!(
                    n.path.first(),
                    Some(RoleSeg::FromA(inner)) if **inner == end
                )
                && matches!(
                    n.path.get(1),
                    Some(RoleSeg::Fragment(Qualifier::Borders(_)))
                );
            (is_frag && matches!(e, Entry::Unique(_))).then_some(n)
        })
        .collect();
    assert_eq!(frags.len(), 2, "expected two Borders-qualified cap pieces");
    assert_ne!(frags[0], frags[1]);
    // The walls are B lateral names (recipe-covariant references),
    // one per piece: the slot wall on its side.
    for f in frags {
        let Some(RoleSeg::Fragment(Qualifier::Borders(walls))) = f.path.get(1) else {
            unreachable!()
        };
        assert_eq!(walls.len(), 1, "{f:?}");
        for wall in walls {
            assert_eq!(wall.node, b, "a wall is not a B-operand face");
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
    // wall x = 4 into TWO prong fragments that border the same one
    // wall — no covariant qualifier separates them: the N2 tie,
    // recorded, naming total.
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
            declare: Vec::new(),
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

/// `node`'s translation along x, value-edited to `to`: every id stays.
fn slide(doc: ProfileDoc, node: RecipeNodeId, to: f64) -> ProfileDoc {
    fixture::step(
        doc,
        editor_core::DocEdit::SetParam {
            node,
            slot: editor_core::SlotId::Translation(editor_core::Axis3::X),
            expr: len(to),
        },
    )
    .0
}

#[test]
fn no_flip_translation_edit_leaves_every_table_identical() {
    // B placed by a Transform whose translation is the edited knob.
    let build = |tx: f64| {
        let doc = ProfileDoc::empty_derived("m4_pr3_names_bool", Tol::witness());
        let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
        let (doc, b0) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
        let (doc, tb) = insert(
            doc,
            Node::transform(
                b0,
                editor_core::Step::Rigid {
                    translation: [len(tx), len(0.0), len(0.0)],
                    axis: [scl(0.0), scl(0.0), scl(1.0)],
                    angle: ang(0.0),
                },
            ),
        );
        // The B side is read at the TRANSFORM, the boolean's operand.
        let decl = declare_x_offset_flush_at(&doc, (a, a), (tb, b0));
        let (doc, u) = insert(
            doc,
            Node::Boolean {
                op: BooleanOp::Union,
                a,
                b: tb,
                declare: decl,
            },
        );
        (doc, u, tb)
    };
    // 0.5 → 0.25, a value edit: still overlapping, same verdict vector
    // ⇒ N4 demands IDENTICAL tables everywhere (names AND keys).
    let (doc1, _, tb) = build(0.5);
    let doc2 = slide(doc1.clone(), tb, 0.25);
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
        let (doc, tb) = insert(
            doc,
            Node::transform(
                b0,
                editor_core::Step::Rigid {
                    translation: [len(tx), len(0.0), len(0.0)],
                    axis: [scl(0.0), scl(0.0), scl(1.0)],
                    angle: ang(0.0),
                },
            ),
        );
        // The B side is read at the TRANSFORM, the boolean's operand.
        let decl = declare_x_offset_flush_at(&doc, (a, a), (tb, b0));
        let (doc, u) = insert(
            doc,
            Node::Boolean {
                op: BooleanOp::Union,
                a,
                b: tb,
                declare: decl,
            },
        );
        (doc, u, tb)
    };
    // 0.5 (overlapping, Seamed) → 2.5 (disjoint, Assembly): verdicts
    // flip AT THE BOOLEAN; every upstream derivation is untouched, so
    // exactly one node's table may change (counted, not vibes).
    let (doc1, u1, tb) = build(0.5);
    let (doc2, u2) = (slide(doc1.clone(), tb, 2.5), u1);
    let ev1 = run(&doc1);
    let ev2 = run(&doc2);
    let changed: Vec<RecipeNodeId> = ev1
        .order
        .iter()
        .copied()
        .filter(|id| table(&ev1, *id) != table(&ev2, *id))
        .collect();
    assert_eq!(changed, vec![u1], "flip cone wider than the boolean");
    // And the flipped table's names differ in SHAPE: the disjoint
    // union has no fragments at all.
    assert!(
        table(&ev2, u2)
            .iter()
            .all(|(n, _)| { !matches!(n.path.last(), Some(RoleSeg::Fragment(_))) })
    );
}
