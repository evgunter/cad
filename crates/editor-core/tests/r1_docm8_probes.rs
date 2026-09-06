//! **R1's review probes for DOCM-8** (review lane, branch
//! `docm/8-review-r1`): chains the implementer did not choose, the
//! order-shapedness that survives, and the shapes the PR's deviations
//! rest on.
//!
//! Deliberately SELF-CONTAINED — every helper is re-derived here
//! rather than imported from the unit's own suite, so the file
//! compiles unchanged at the merge base and a bug in the unit's
//! helpers cannot hide in mine.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use crate::fixture::{ang, fname, insert, len, on_frame, scl, step, wall};

use editor_core::{
    CancelToken, CapEnd, DocEdit, EntityKind, Entry, EvalOptions, Evaluation, NameTable, Node,
    NodeErrorKind, NodeResult, ProfileDoc, RecipeNodeId, RoleSeg, StableName, ValuePayload,
    evaluate,
};
use geom_core::Tol;

// ---------------------------------------------------------------------
// helpers, re-derived
// ---------------------------------------------------------------------

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
    &ev.value(id).expect("the node evaluated").name_table
}

fn failure(ev: &Evaluation<f64>, id: RecipeNodeId) -> Option<&NodeErrorKind> {
    match ev.nodes.get(&id) {
        Some(NodeResult::Failed(e)) => Some(&e.kind),
        _ => None,
    }
}

fn body_of(ev: &Evaluation<f64>, id: RecipeNodeId) -> topo::Body<f64> {
    match &ev.value(id).expect("the node evaluated").payload {
        ValuePayload::Body(b) => (**b).clone(),
        ValuePayload::Boolean(editor_core::BooleanValue::Body { body, .. }) => (**body).clone(),
        other => panic!("expected a body, got {other:?}"),
    }
}

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

/// A rigid placement of `input`, translated along +x.
fn placed(doc: ProfileDoc, input: RecipeNodeId, dx: f64) -> (ProfileDoc, RecipeNodeId) {
    insert(
        doc,
        Node::Transform {
            input,
            translation: [len(dx), len(0.0), len(0.0)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: ang(0.0),
        },
    )
}

fn member_face(union: RecipeNodeId, member: RecipeNodeId, of: StableName) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: union,
        path: vec![RoleSeg::FromMember {
            member,
            of: Box::new(of),
        }],
    }
}

/// The four flush families two x-offset blocks share.
fn x_flush() -> [RoleSeg; 4] {
    [
        wall(0),
        wall(2),
        RoleSeg::Cap(CapEnd::Start),
        RoleSeg::Cap(CapEnd::End),
    ]
}

fn flush_pairs(
    union: RecipeNodeId,
    (m1, e1): (RecipeNodeId, RecipeNodeId),
    (m2, e2): (RecipeNodeId, RecipeNodeId),
) -> Vec<(StableName, StableName)> {
    x_flush()
        .into_iter()
        .map(|seg| {
            (
                member_face(union, m1, fname(e1, seg.clone())),
                member_face(union, m2, fname(e2, seg)),
            )
        })
        .collect()
}

fn declared_union(
    doc: ProfileDoc,
    members: &[RecipeNodeId],
    pairs: impl Fn(RecipeNodeId) -> Vec<(StableName, StableName)>,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, first) = insert(
        doc,
        Node::Union {
            members: members.to_vec(),
            declare: None,
        },
    );
    let (doc, decl) = insert(doc, Node::declare_rest(pairs(first)));
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: members.to_vec(),
            declare: Some(decl),
        },
    );
    let mut doc = doc;
    let mut moved: Vec<StableName> = Vec::new();
    for ((fa, fb), (ta, tb)) in pairs(first).into_iter().zip(pairs(union)) {
        for (from, to) in [(fa, ta), (fb, tb)] {
            if moved.contains(&from) {
                continue;
            }
            moved.push(from.clone());
            doc = step(doc, DocEdit::Rebind { from, to }).0;
        }
    }
    let (doc, _) = step(doc, DocEdit::DeleteNode { id: first });
    (doc, union)
}

fn perms(items: &[RecipeNodeId]) -> Vec<Vec<RecipeNodeId>> {
    if items.len() <= 1 {
        return vec![items.to_vec()];
    }
    let mut out = Vec::new();
    for (i, &head) in items.iter().enumerate() {
        let mut rest = items.to_vec();
        rest.remove(i);
        for mut tail in perms(&rest) {
            tail.insert(0, head);
            out.push(tail);
        }
    }
    out
}

fn volume(b: &topo::Body<f64>) -> f64 {
    topo::mass_properties(b, Tol::witness())
        .expect("mass")
        .volume
}

/// Every published merged row, as a sorted list of sorted constituent
/// spellings — the SET comparison the brief asks for, not the volume.
fn merged_sets(t: &NameTable) -> Vec<Vec<String>> {
    let mut rows: Vec<Vec<String>> = t
        .iter()
        .filter_map(|(n, _)| match n.path.as_slice() {
            [RoleSeg::Merged(cs)] => {
                let mut v: Vec<String> = cs.iter().map(|c| c.to_string()).collect();
                v.sort();
                Some(v)
            }
            _ => None,
        })
        .collect();
    rows.sort();
    rows
}

/// Chain contacts: every consecutive pair meets flush along x.
fn chain_pairs(union: RecipeNodeId, chain: &[RecipeNodeId]) -> Vec<(StableName, StableName)> {
    chain
        .windows(2)
        .flat_map(|w| flush_pairs(union, (w[0], w[0]), (w[1], w[1])))
        .collect()
}

/// Runs EVERY order of `members`; asserts each fuses to `want` and
/// that the merged-row constituent sets are identical across orders.
fn every_order_agrees(
    doc: &ProfileDoc,
    members: &[RecipeNodeId],
    pairs: impl Fn(RecipeNodeId) -> Vec<(StableName, StableName)> + Copy,
    want: f64,
    label: &str,
) {
    let mut witness: Option<(Vec<Vec<String>>, Vec<RecipeNodeId>)> = None;
    for order in perms(members) {
        let tag = format!("{label}: order {order:?}");
        let (docx, union) = declared_union(doc.clone(), &order, pairs);
        let ev = run(&docx);
        assert!(
            failure(&ev, union).is_none(),
            "{tag}: refused: {:?}",
            failure(&ev, union)
        );
        let v = volume(&body_of(&ev, union));
        assert!((v - want).abs() < 1e-9, "{tag}: want {want}, got {v}");
        let sets = merged_sets(table(&ev, union));
        match &witness {
            None => witness = Some((sets, order.clone())),
            Some((first, first_order)) => assert_eq!(
                &sets, first,
                "{tag}: merged constituent sets differ from order {first_order:?}"
            ),
        }
    }
}

// ---------------------------------------------------------------------
// C1 — chains the implementer did not choose
// ---------------------------------------------------------------------

/// **A FIVE-member chain fuses in all 120 orders** with identical flat
/// constituent sets. GREEN on the unit's head.
#[test]
fn r1_c1_a_five_member_chain_fuses_in_every_order() {
    let doc = ProfileDoc::empty_derived("r1_chain5", Tol::witness());
    let (doc, m0) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, m1) = block(doc, (0.8, 1.8), (0.0, 1.0), 0.0, 1.0);
    let (doc, m2) = block(doc, (1.6, 2.6), (0.0, 1.0), 0.0, 1.0);
    let (doc, m3) = block(doc, (2.4, 3.4), (0.0, 1.0), 0.0, 1.0);
    let (doc, m4) = block(doc, (3.2, 4.2), (0.0, 1.0), 0.0, 1.0);
    let chain = [m0, m1, m2, m3, m4];
    every_order_agrees(&doc, &chain, |u| chain_pairs(u, &chain), 4.2, "chain5");
}

/// **A chain of three PLACEMENTS of one prototype fuses in every
/// order** with identical flat sets — the case the pair boolean cannot
/// spell at all. GREEN on the unit's head.
#[test]
fn r1_c1_a_chain_of_placements_of_one_prototype_fuses_in_every_order() {
    let doc = ProfileDoc::empty_derived("r1_placements", Tol::witness());
    let (doc, proto) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, p0) = placed(doc, proto, 0.0);
    let (doc, p1) = placed(doc, proto, 0.8);
    let (doc, p2) = placed(doc, proto, 1.6);
    let members = [p0, p1, p2];
    every_order_agrees(
        &doc,
        &members,
        |u| {
            let mut v = flush_pairs(u, (p0, proto), (p1, proto));
            v.extend(flush_pairs(u, (p1, proto), (p2, proto)));
            v
        },
        2.6,
        "placements",
    );
}

/// **RED on the unit's head — the order-shapedness survives when a
/// declared member face is SPLIT rather than merged away.**
///
/// Three members. `a` and `c` meet flush along x (four families).
/// `s` sits on top of `a`, sharing only `a`'s two y-walls, and its
/// footprint is strictly inside `a`'s top cap, so folding `s` in
/// FRAGMENTS that cap. The same declaration set therefore fuses in the
/// orders that fold `s` last and refuses `DeclareResolve { Vanished }`
/// in the orders that fold it earlier — `a`'s cap is then neither a
/// row nor a member of any `Merged` row's constituent set, which is
/// the one thing [`look_through_merges`] looks for.
///
/// This is the shape the three rewritten prose sites now deny
/// ("reordering the members changes nothing about whether a
/// declaration resolves") and DOCM-7's deleted sentence covered.
#[test]
fn r1_c1_a_declared_face_split_by_a_later_member_is_still_order_shaped() {
    let doc = ProfileDoc::empty_derived("r1_split_order", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, s) = block(doc, (0.2, 0.4), (0.0, 1.0), 0.5, 1.0);
    let members = [a, c, s];
    let pairs = move |u: RecipeNodeId| {
        let mut v = flush_pairs(u, (a, a), (c, c));
        for seg in [wall(0), wall(2)] {
            v.push((
                member_face(u, a, fname(a, seg.clone())),
                member_face(u, s, fname(s, seg)),
            ));
        }
        v
    };
    let mut refused: Vec<String> = Vec::new();
    for order in perms(&members) {
        let name = |m: RecipeNodeId| {
            if m == a {
                "a"
            } else if m == c {
                "c"
            } else {
                "s"
            }
        };
        let tag: Vec<&str> = order.iter().map(|m| name(*m)).collect();
        let (docx, union) = declared_union(doc.clone(), &order, pairs);
        let ev = run(&docx);
        match failure(&ev, union) {
            None => {}
            Some(f) => refused.push(format!("{tag:?} -> {f:?}")),
        }
    }
    assert!(
        refused.is_empty(),
        "{} of 6 orders refused:\n{}",
        refused.len(),
        refused.join("\n")
    );
}

/// **RED on the unit's head — a middle member with THREE neighbours.**
///
/// `c` meets `w` and `e` along x and `t` (stacked in z, narrower in x)
/// along its two y-walls; `w`, `e`, `t` are pairwise disjoint. Reported
/// as a class with the row above: 8 of the 24 orders refuse
/// `DeclareResolve { Vanished }` on `c`'s end cap (the split), and 10
/// refuse a pre-existing, untouched `Naming(Emission { "seam vertex
/// parentage underdetermined from incident edges" })`
/// (`emit_topo.rs:1247`, outside this unit's diff).
#[test]
fn r1_c1_a_middle_member_with_three_neighbours_fuses_in_every_order() {
    let doc = ProfileDoc::empty_derived("r1_star", Tol::witness());
    let (doc, c) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, w) = block(doc, (-0.8, 0.2), (0.0, 1.0), 0.0, 1.0);
    let (doc, e) = block(doc, (0.8, 1.8), (0.0, 1.0), 0.0, 1.0);
    let (doc, t) = block(doc, (0.3, 0.7), (0.0, 1.0), 0.8, 1.0);
    let members = [c, w, e, t];
    let pairs = move |u: RecipeNodeId| {
        let mut v = flush_pairs(u, (c, c), (w, w));
        v.extend(flush_pairs(u, (c, c), (e, e)));
        for seg in [wall(0), wall(2)] {
            v.push((
                member_face(u, c, fname(c, seg.clone())),
                member_face(u, t, fname(t, seg)),
            ));
        }
        v
    };
    every_order_agrees(&doc, &members, pairs, 2.92, "star");
}

// ---------------------------------------------------------------------
// C2 — a merged face carried through TWO untouched steps
// ---------------------------------------------------------------------

/// **A merged face carried through two untouched fold steps before its
/// next merge is still minted flat.** GREEN on the unit's head; the
/// A2 walker runs over it too.
#[test]
fn r1_c2_a_merged_face_carried_through_two_untouched_steps_stays_flat() {
    let doc = ProfileDoc::empty_derived("r1_two_steps", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, f1) = block(doc, (6.0, 7.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, f2) = block(doc, (9.0, 10.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, d) = block(doc, (1.2, 2.2), (0.0, 1.0), 0.0, 1.0);
    let chain = [a, c, d];
    let (doc, union) = declared_union(doc, &[a, c, f1, f2, d], |u| chain_pairs(u, &chain));
    let ev = run(&doc);
    assert!(
        failure(&ev, union).is_none(),
        "refused: {:?}",
        failure(&ev, union)
    );
    let v = volume(&body_of(&ev, union));
    assert!((v - 4.2).abs() < 1e-9, "got volume {v}");
    let t = table(&ev, union);
    let mut set: Vec<StableName> = chain
        .iter()
        .map(|&m| member_face(union, m, fname(m, RoleSeg::Cap(CapEnd::Start))))
        .collect();
    set.sort();
    let row = StableName {
        kind: EntityKind::Face,
        node: union,
        path: vec![RoleSeg::Merged(set)],
    };
    assert!(
        matches!(t.lookup(&row), Some(Entry::Unique(_))),
        "the flat cap row over two untouched steps is not published: {row:?}"
    );
    fixture::assert_no_nested_merged(&ev);
}

// ---------------------------------------------------------------------
// C2 — the pass-through peel's `FromB` arm
// ---------------------------------------------------------------------

fn from_a(node: RecipeNodeId, inner: StableName) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::FromA(Box::new(inner))],
    }
}

fn from_b(node: RecipeNodeId, inner: StableName) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::FromB(Box::new(inner))],
    }
}

fn merged_of(node: RecipeNodeId, mut set: Vec<StableName>) -> StableName {
    set.sort();
    StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::Merged(set)],
    }
}

/// **A merged face carried through a step as operand B, then merged
/// again, is minted flat** — the `FromB` arm of `merged_foot`.
///
/// Every existing fixture carries a pass-through merged face on the A
/// side (a union's accumulation is always operand A, and
/// `corner_table` chains `Boolean { a: previous, b: new }`), so the
/// `FromB` arm is exercised nowhere: deleting it from `emit_topo.rs`
/// leaves the whole `editor-core` suite green.
#[test]
fn r1_c2_a_merged_face_passed_through_as_operand_b_is_still_flat() {
    let doc = ProfileDoc::empty_derived("r1_fromb", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, decl_ab) = insert(
        doc,
        Node::declare_rest(
            x_flush()
                .into_iter()
                .map(|seg| (fname(a, seg.clone()), fname(b, seg)))
                .collect(),
        ),
    );
    let (doc, inner) = insert(
        doc,
        Node::Boolean {
            op: editor_core::BooleanOp::Union,
            a,
            b,
            declare: Some(decl_ab),
        },
    );
    // A far block, unioned with the merge as operand B, so the merged
    // rows ride through as `mid:FromB(inner:Merged(..))`.
    let (doc, far) = block(doc, (8.0, 9.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, mid) = insert(
        doc,
        Node::Boolean {
            op: editor_core::BooleanOp::Union,
            a: far,
            b: inner,
            declare: None,
        },
    );
    let (doc, c) = block(doc, (1.2, 2.2), (0.0, 1.0), 0.0, 1.0);
    let carried = |seg: RoleSeg| {
        from_b(
            mid,
            merged_of(
                inner,
                vec![
                    from_a(inner, fname(a, seg.clone())),
                    from_b(inner, fname(b, seg)),
                ],
            ),
        )
    };
    let (doc, decl_mc) = insert(
        doc,
        Node::declare_rest(
            x_flush()
                .into_iter()
                .map(|seg| (carried(seg.clone()), fname(c, seg)))
                .collect(),
        ),
    );
    let (doc, outer) = insert(
        doc,
        Node::Boolean {
            op: editor_core::BooleanOp::Union,
            a: mid,
            b: c,
            declare: Some(decl_mc),
        },
    );
    let ev = run(&doc);
    assert!(
        failure(&ev, outer).is_none(),
        "the outer boolean refused: {:?}",
        failure(&ev, outer)
    );
    let t = table(&ev, outer);
    for seg in x_flush() {
        let want = merged_of(
            outer,
            vec![
                from_a(outer, from_b(mid, from_a(inner, fname(a, seg.clone())))),
                from_a(outer, from_b(mid, from_b(inner, fname(b, seg.clone())))),
                from_b(outer, fname(c, seg.clone())),
            ],
        );
        assert!(
            matches!(t.lookup(&want), Some(Entry::Unique(_))),
            "the FromB pass-through did not mint flat for {seg:?}: {want:?}"
        );
    }
    fixture::assert_no_nested_merged(&ev);
}

/// The fusing half of DOCM-7's DELETED measurement row, re-pinned
/// here: the two orders it asserted fuse still fuse to 2.2.
#[test]
fn r1_c1_the_deleted_measurement_rows_fusing_half_still_holds() {
    let doc = ProfileDoc::empty_derived("r1_deleted_half", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, d) = block(doc, (1.2, 2.2), (0.0, 1.0), 0.0, 1.0);
    for order in [[a, d, c], [d, a, c]] {
        let (docx, union) = declared_union(doc.clone(), &order, |u| chain_pairs(u, &[a, c, d]));
        let ev = run(&docx);
        assert!(failure(&ev, union).is_none(), "{order:?} refused");
        let v = volume(&body_of(&ev, union));
        assert!((v - 2.2).abs() < 1e-9, "{order:?}: got {v}");
    }
}
