//! **The n-ary union's declaration channel, in member space** (DOCM-7;
//! DM4 as amended): the `declare` edge, the routing of each pair to
//! the fold step that joins the two things it names, the resolver the
//! union shares with the pair boolean, and the `Merged` rows the
//! channel makes reachable.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    BooleanOp, BooleanValue, CancelToken, CapEnd, DocEdit, EditError, EntityKind, Entry,
    EvalOptions, Evaluation, NameTable, Node, NodeErrorKind, NodeResult, ProfileDoc,
    ProfileEdgeRef, RecipeNodeId, RoleSeg, StableName, ValuePayload, evaluate,
};
use fixture::{ang, fname, insert, len, on_frame, scl, step, wall};
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

fn failure(ev: &Evaluation<f64>, id: RecipeNodeId) -> Option<&NodeErrorKind> {
    match ev.nodes.get(&id) {
        Some(NodeResult::Failed(e)) => Some(&e.kind),
        _ => None,
    }
}

fn body_of(ev: &Evaluation<f64>, id: RecipeNodeId) -> topo::Body<f64> {
    match &ev.value(id).expect("the node evaluated").payload {
        ValuePayload::Body(b) => (**b).clone(),
        ValuePayload::Boolean(BooleanValue::Body { body, .. }) => (**body).clone(),
        other => panic!("expected a body, got {other:?}"),
    }
}

/// An axis-aligned block on the xy plane at height `z0`, extruded `dz`.
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

/// One face of one member, as the UNION's own name space spells it —
/// the row `member_view` puts into that member's operand table.
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

/// The four flush pairs of two blocks that share their y-range and
/// z-range and differ along x only, in one union's member space: the
/// y-walls and both caps — `declare_x_offset_flush`'s pairs, lifted.
fn flush_pairs(
    union: RecipeNodeId,
    (m1, e1): (RecipeNodeId, RecipeNodeId),
    (m2, e2): (RecipeNodeId, RecipeNodeId),
) -> Vec<(StableName, StableName)> {
    let mut out = Vec::new();
    for seg in [
        wall(0),
        wall(2),
        RoleSeg::Cap(CapEnd::Start),
        RoleSeg::Cap(CapEnd::End),
    ] {
        out.push((
            member_face(union, m1, fname(e1, seg.clone())),
            member_face(union, m2, fname(e2, seg)),
        ));
    }
    out
}

/// **A union carrying a declaration, built through the doors that
/// exist.**
///
/// A member-space name carries the UNION's own node id, so the
/// `Declare` that names one cannot be inserted before the union it
/// names: the insert door admits a payload name only when its node is
/// already live (`EditError::DeclareNamesMissingNode`, pinned by
/// [`a_declare_cannot_name_a_union_that_does_not_exist_yet`]). And no
/// edit rewires a live node's inputs (DM6). So the order that works is
/// a FIRST union, whose space the declaration is written in, the
/// second union carrying the edge, one `Rebind` per name onto it, and
/// the first deleted. `pairs` is asked for its names twice — once in
/// each union's space — because that is what the rebinds move.
fn declared_union(
    doc: ProfileDoc,
    members: &[RecipeNodeId],
    pairs: impl Fn(RecipeNodeId) -> Vec<(StableName, StableName)>,
) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
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
    for ((fa, fb), (ta, tb)) in pairs(first).into_iter().zip(pairs(union)) {
        for (from, to) in [(fa, ta), (fb, tb)] {
            doc = step(doc, DocEdit::Rebind { from, to }).0;
        }
    }
    let (doc, _) = step(doc, DocEdit::DeleteNode { id: first });
    (doc, union, decl)
}

// ---------------------------------------------------------------------
// A1 — the motivating case fuses.
// ---------------------------------------------------------------------

/// **Two flush placements of ONE prototype fuse when the contact is
/// declared, and refuse naming both members when it is not.**
///
/// This is DOCM-3's reviewer probe. Undeclared, the fold refuses the
/// contact exactly as a pair boolean's operands would; declared in
/// member space, the same document is one body of the fused volume.
#[test]
fn a_union_of_two_flush_placements_of_one_prototype_fuses_when_declared() {
    let doc = ProfileDoc::empty_derived("docm7_placements", Tol::witness());
    let (doc, proto) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, m1) = placed(doc, proto, 0.0);
    let (doc, m2) = placed(doc, proto, 0.5);
    // Undeclared: the refusal, naming both members.
    let (bare, plain) = insert(
        doc.clone(),
        Node::Union {
            members: vec![m1, m2],
            declare: None,
        },
    );
    let ev = run(&bare);
    let Some(NodeErrorKind::UndeclaredContact { finding, .. }) = failure(&ev, plain) else {
        panic!("expected the undeclared contact, got {:?}", failure(&ev, plain));
    };
    let member_of = |n: &StableName| match n.path.first() {
        Some(RoleSeg::FromMember { member, .. }) => Some(*member),
        _ => None,
    };
    let named: Vec<Option<RecipeNodeId>> =
        vec![member_of(&finding.pair.0), member_of(&finding.pair.1)];
    assert!(
        named.contains(&Some(m1)) && named.contains(&Some(m2)),
        "the refusal names both members: {named:?}"
    );
    // Declared in member space: the same two placements fuse.
    let (doc, union, _) = declared_union(doc, &[m1, m2], |u| {
        flush_pairs(u, (m1, proto), (m2, proto))
    });
    let ev = run(&doc);
    assert!(
        failure(&ev, union).is_none(),
        "the declared union refused: {:?}",
        failure(&ev, union)
    );
    let volume = topo::mass_properties(&body_of(&ev, union), Tol::witness())
        .expect("the fused body has mass")
        .volume;
    assert_eq!(volume, 1.5, "the fused volume is the two blocks' union");
}

/// **The pair boolean cannot spell this contact at all**, which is
/// what the member-keyed channel adds.
///
/// A transform contributes no role segment (N1), so two placements of
/// one prototype carry IDENTICAL tables: every name the author could
/// write resolves in BOTH operands, and the pair boolean's declaration
/// door declines to pick a side. The union's names carry the member
/// EDGE, so the same intent is unambiguous there.
#[test]
fn the_pair_boolean_cannot_declare_between_two_placements_of_one_prototype() {
    let doc = ProfileDoc::empty_derived("docm7_pair_gap", Tol::witness());
    let (doc, proto) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, m1) = placed(doc, proto, 0.0);
    let (doc, m2) = placed(doc, proto, 0.5);
    let (doc, decl) = insert(
        doc,
        Node::declare_rest(vec![(
            fname(proto, wall(0)),
            fname(proto, wall(0)),
        )]),
    );
    let (doc, pair) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: m1,
            b: m2,
            declare: Some(decl),
        },
    );
    let ev = run(&doc);
    assert!(
        matches!(
            failure(&ev, pair),
            Some(NodeErrorKind::DeclareBothOperands { .. })
        ),
        "expected the side-picking refusal, got {:?}",
        failure(&ev, pair)
    );
}

/// **A declared union is the pair boolean's body**, on two members
/// whose names the pair spelling can tell apart.
///
/// Same two blocks, same four declared contacts, spelled once in the
/// union's member space and once in the boolean's operand space: one
/// body, face for face and description for description. The geometry
/// is the pair verb's at every step, which is the whole claim.
#[test]
fn a_declared_union_is_the_pair_booleans_body() {
    let doc = ProfileDoc::empty_derived("docm7_pair_eq", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, decl) = fixture::declare_x_offset_flush(doc, a, b);
    let (doc, pair) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b,
            declare: Some(decl),
        },
    );
    let (doc, union, _) = declared_union(doc, &[a, b], |u| flush_pairs(u, (a, a), (b, b)));
    let ev = run(&doc);
    let (folded, paired) = (body_of(&ev, union), body_of(&ev, pair));
    assert_eq!(folded.faces().count(), paired.faces().count());
    assert_eq!(folded.edges().count(), paired.edges().count());
    assert_eq!(folded.vertices().count(), paired.vertices().count());
    let sorted = |mut v: Vec<String>| {
        v.sort();
        v
    };
    let surfaces =
        |b: &topo::Body<f64>| sorted(b.surfaces().map(|(_, s)| format!("{s:?}")).collect());
    let curves = |b: &topo::Body<f64>| sorted(b.curves().map(|(_, c)| format!("{c:?}")).collect());
    let points = |b: &topo::Body<f64>| sorted(b.points().map(|(_, p)| format!("{p:?}")).collect());
    assert_eq!(surfaces(&folded), surfaces(&paired));
    assert_eq!(curves(&folded), curves(&paired));
    assert_eq!(points(&folded), points(&paired));
}

// ---------------------------------------------------------------------
// A3 — the `Merged` arm, and what a declaration does NOT rename.
// ---------------------------------------------------------------------

/// **The declared coincidence mints `Merged` rows under the union**,
/// and renames nothing else.
///
/// The four declared face pairs retire into four merged faces whose
/// constituents are the member-keyed names the author declared — the
/// arm `collapse` carries for exactly this, unreachable before the
/// channel existed. Every other face keeps the one-wrapper
/// `FromMember` name it would have had, built here by hand rather than
/// read back from the table, so the row states the name instead of
/// echoing it.
#[test]
fn a_declaration_mints_merged_rows_and_renames_nothing_else() {
    let doc = ProfileDoc::empty_derived("docm7_merged", Tol::witness());
    let (doc, proto) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, m1) = placed(doc, proto, 0.0);
    let (doc, m2) = placed(doc, proto, 0.5);
    let (doc, union, _) = declared_union(doc, &[m1, m2], |u| {
        flush_pairs(u, (m1, proto), (m2, proto))
    });
    let ev = run(&doc);
    let t = table(&ev, union);
    // Four merged faces: the two y-walls and the two caps.
    let merged: Vec<&StableName> = t
        .iter()
        .map(|(n, _)| n)
        .filter(|n| matches!(n.path.first(), Some(RoleSeg::Merged(_))))
        .collect();
    assert_eq!(merged.len(), 4, "the four declared contacts merged: {merged:?}");
    // And each merged row's constituents are the declared names, as a
    // sorted set — so a selector spelled against the merged face is
    // exactly this name, and it resolves.
    for (a, b) in flush_pairs(union, (m1, proto), (m2, proto)) {
        let mut set = vec![a, b];
        set.sort();
        let name = StableName {
            kind: EntityKind::Face,
            node: union,
            path: vec![RoleSeg::Merged(set)],
        };
        assert!(
            matches!(t.lookup(&name), Some(Entry::Unique(_))),
            "the merged face {name} does not resolve"
        );
    }
    // The x-extreme walls are untouched: one `FromMember` wrapper over
    // the prototype's own name, exactly as an undeclared fold gives.
    for (member, seg) in [(m1, 3u32), (m2, 1u32)] {
        let name = member_face(
            union,
            member,
            fname(
                proto,
                RoleSeg::Lateral(ProfileEdgeRef {
                    loop_index: 0,
                    segment: seg,
                }),
            ),
        );
        assert!(
            matches!(t.lookup(&name), Some(Entry::Unique(_))),
            "the untouched wall {name} lost its member-keyed name"
        );
    }
}

// ---------------------------------------------------------------------
// A2 — routing, from member ids only.
// ---------------------------------------------------------------------

/// **A declared pair is fed at the LATER member's step, and the list's
/// order is not part of the declaration.**
///
/// Three members: the first and the third touch, the second stands
/// clear. The declaration reaches the third member's step, and the
/// SAME pair — the same two member ids, the same names — still routes
/// and still fuses when the list is reordered so those two members sit
/// somewhere else entirely. Nothing in the declaration had to change,
/// which is what "records no fold position" means.
#[test]
fn a_declared_pair_routes_by_member_id_and_survives_a_reorder() {
    let build = |order: [usize; 3]| {
        let doc = ProfileDoc::empty_derived("docm7_routing", Tol::witness());
        let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
        let (doc, far) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
        let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
        let all = [a, far, b];
        let members: Vec<RecipeNodeId> = order.iter().map(|i| all[*i]).collect();
        let (doc, union, _) =
            declared_union(doc, &members, |u| flush_pairs(u, (a, a), (b, b)));
        (doc, union)
    };
    // Members (a, far, b): the declared pair belongs to step 3.
    let (doc, union) = build([0, 1, 2]);
    let ev = run(&doc);
    assert!(
        failure(&ev, union).is_none(),
        "the routed declaration refused: {:?}",
        failure(&ev, union)
    );
    let straight = body_of(&ev, union);
    // Members (b, a, far): the same two member ids, now the FIRST two,
    // so the same pair is fed at step 1 instead — derived, not stored.
    let (doc2, union2) = build([2, 0, 1]);
    let ev2 = run(&doc2);
    assert!(
        failure(&ev2, union2).is_none(),
        "the reordered declaration refused: {:?}",
        failure(&ev2, union2)
    );
    let reordered = body_of(&ev2, union2);
    assert_eq!(straight.faces().count(), reordered.faces().count());
    assert_eq!(straight.edges().count(), reordered.edges().count());
    assert_eq!(straight.vertices().count(), reordered.vertices().count());
    assert_eq!(
        topo::mass_properties(&straight, Tol::witness())
            .expect("mass")
            .volume,
        topo::mass_properties(&reordered, Tol::witness())
            .expect("mass")
            .volume,
    );
    // Both spellings merge the same four contacts, so the declaration
    // did the same work at whichever step it landed on.
    let merged = |ev: &Evaluation<f64>, id: RecipeNodeId| {
        table(ev, id)
            .iter()
            .filter(|(n, _)| matches!(n.path.first(), Some(RoleSeg::Merged(_))))
            .count()
    };
    assert_eq!(merged(&ev, union), 4);
    assert_eq!(merged(&ev2, union2), 4);
    // What is NOT asserted here, because it is measurably false: that
    // the two orders give the same names and the same descriptions.
    // Reordering swaps which of the two touching members the pair verb
    // sees as operand A, and the pair emitter is not symmetric in that
    // role — the merged face keeps the A-side member's carrier plane
    // (a different origin on the same plane), and the rim edges that
    // the merge splits are the A-side member's (`Fragment(OrderAlong)`
    // rows on `a`'s rims under (a, far, b), on `b`'s under (b, a,
    // far)). The volume, the counts and the merge are the same; the
    // asymmetry is the PAIR verb's and this node inherits it.
}

// ---------------------------------------------------------------------
// A4 — the refusals, one row each.
// ---------------------------------------------------------------------

/// **A declared name in neither table refuses through the N5 ladder**,
/// never silently.
#[test]
fn a_declared_name_that_denotes_nothing_refuses() {
    let doc = ProfileDoc::empty_derived("docm7_vanished", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    // A member-space name whose INNER name is a wall the prototype
    // does not have: the member routes, the lookup finds nothing.
    let (doc, union, _) = declared_union(doc, &[a, b], |u| {
        vec![(
            member_face(u, a, fname(a, wall(0))),
            member_face(u, b, fname(b, wall(7))),
        )]
    });
    let ev = run(&doc);
    assert!(
        matches!(failure(&ev, union), Some(NodeErrorKind::DeclareResolve { .. })),
        "expected the N5 refusal, got {:?}",
        failure(&ev, union)
    );
}

/// **A declared member the list no longer holds refuses**, which is
/// the state `SetMembers` creates by removing one.
#[test]
fn a_declared_member_removed_by_set_members_refuses() {
    let doc = ProfileDoc::empty_derived("docm7_dropped", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, far) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, union, _) = declared_union(doc, &[a, b, far], |u| flush_pairs(u, (a, a), (b, b)));
    // The declaration survives the edit as written; it is the next
    // evaluation that refuses it.
    let (doc, _) = step(
        doc,
        DocEdit::SetMembers {
            node: union,
            members: vec![a, far],
        },
    );
    let ev = run(&doc);
    assert!(
        matches!(failure(&ev, union), Some(NodeErrorKind::DeclareResolve { .. })),
        "expected the N5 refusal, got {:?}",
        failure(&ev, union)
    );
}

/// **An accumulation entity paired with a member the fold had already
/// joined has no step**, and is refused rather than re-read as a
/// same-operand claim.
#[test]
fn an_accumulation_entity_paired_with_an_earlier_member_refuses() {
    let doc = ProfileDoc::empty_derived("docm7_unroutable", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, union, _) = declared_union(doc, &[a, b], |u| {
        // A merge of the two members' y-walls — a row the fold could
        // only mint at step 1 — paired with member `a`, which joined
        // at step 1 as operand A. No step has both as its two sides.
        let mut set = vec![
            member_face(u, a, fname(a, wall(0))),
            member_face(u, b, fname(b, wall(0))),
        ];
        set.sort();
        vec![(
            StableName {
                kind: EntityKind::Face,
                node: u,
                path: vec![RoleSeg::Merged(set)],
            },
            member_face(u, a, fname(a, wall(2))),
        )]
    });
    let ev = run(&doc);
    assert!(
        matches!(failure(&ev, union), Some(NodeErrorKind::UnionDeclareStep { .. })),
        "expected the unroutable-pair refusal, got {:?}",
        failure(&ev, union)
    );
}

/// **The edit door refuses a `declare` input that is not a `Declare`**
/// — for the union as for the boolean, since the rule is asked of one
/// answer (`Node::declare_input`).
#[test]
fn the_edit_door_refuses_a_union_declare_that_is_not_a_declare() {
    let doc = ProfileDoc::empty_derived("docm7_edit_door", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    // A third live node, because a declare edge naming one of the
    // members is refused by the list's own rule first (DM5).
    let (doc, far) = block(doc, (8.0, 9.0), (0.0, 1.0), 0.0, 1.0);
    let refused = doc.apply(
        &DocEdit::InsertNode {
            node: Node::Union {
                members: vec![a, b],
                declare: Some(far),
            },
        },
        Tol::witness(),
    );
    assert!(
        matches!(
            refused,
            Err(EditError::DeclareInputNotDeclare { input, .. }) if input == far
        ),
        "expected the declare edge's kind refusal, got {refused:?}"
    );
}

/// **The load door asks the same question of file data.**
///
/// A saved document is edited to point a union's `declare` at a body
/// node; the validator refuses it, as the edit door would have.
#[test]
fn a_snapshot_whose_union_declare_is_not_a_declare_does_not_load() {
    let tol = Tol::witness();
    let doc = ProfileDoc::empty_derived("docm7_load_door", tol);
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, far) = block(doc, (8.0, 9.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, _) = insert(
        doc,
        Node::Union {
            members: vec![a, b],
            declare: None,
        },
    );
    let text = editor_core::persist::save(&doc, &[], tol).expect("the document saves");
    let (head, rest) = text
        .split_once("\"members\": [")
        .expect("the union's list is on the wire");
    let (list, tail) = rest.split_once(']').expect("the list closes");
    // A node that is NOT one of the members, so the pairwise-distinct
    // rule does not answer first.
    let tampered = format!(
        "{head}\"members\": [{list}]{}",
        tail.replacen("\"declare\": null", &format!("\"declare\": {}", far.0), 1)
    );
    let err = editor_core::persist::load(&tampered, tol)
        .expect_err("a union whose declare is a body must refuse");
    let said = format!("{err}");
    assert!(said.contains("not a declaration"), "{said}");
}

/// **A `Declare` cannot name a union that does not exist yet.**
///
/// A member-space name carries the union's own node id, and the insert
/// door admits a payload name only when its node is live. This is the
/// door that makes [`declared_union`]'s two-pass shape the only way to
/// author the edge today; it is pinned so the constraint is a measured
/// fact rather than a claim in a comment.
#[test]
fn a_declare_cannot_name_a_union_that_does_not_exist_yet() {
    let doc = ProfileDoc::empty_derived("docm7_forward", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    // The id the union WOULD get, one past the last live node.
    let future = RecipeNodeId(doc.order().last().expect("a node").0 + 1);
    let refused = doc.apply(
        &DocEdit::InsertNode {
            node: Node::declare_rest(vec![(
                member_face(future, a, fname(a, wall(0))),
                member_face(future, b, fname(b, wall(0))),
            )]),
        },
        Tol::witness(),
    );
    assert!(
        matches!(refused, Err(EditError::DeclareNamesMissingNode { .. })),
        "expected the payload-name door's refusal, got {refused:?}"
    );
}

// ---------------------------------------------------------------------
// A5 — the key and the wire.
// ---------------------------------------------------------------------

/// **A union with a declaration is not the union whose member list
/// ends in that same `Declare` node.**
///
/// The two present the SAME upstream keys in the same order, so the
/// member count is what separates them. Without that separation the
/// second would take the first's memo entry and report a fused body
/// for a document that puts a declaration at a body seat.
#[test]
fn a_declare_on_the_edge_is_not_a_declare_in_the_member_list() {
    let doc = ProfileDoc::empty_derived("docm7_key", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, union, decl) = declared_union(doc, &[a, b], |u| flush_pairs(u, (a, a), (b, b)));
    let (doc, miswired) = insert(
        doc,
        Node::Union {
            members: vec![a, b, decl],
            declare: None,
        },
    );
    let ev = run(&doc);
    assert!(
        failure(&ev, union).is_none(),
        "the declared union refused: {:?}",
        failure(&ev, union)
    );
    // The mis-wire has no value of its own — and could only have one
    // by taking the declared union's, which is what the separation in
    // the key prevents. (Its own refusal is the undeclared contact at
    // step 1, which it reaches before the declaration at a body seat.)
    assert!(
        ev.value(miswired).is_none(),
        "the mis-wired union was served the declared union's body"
    );
    assert!(failure(&ev, miswired).is_some(), "and it refuses typed");
}

/// **The declare edge recomputes the union and nothing upstream.**
///
/// The same members, once without the edge and once with it: every
/// node the two documents share hits the memo, and the union — which
/// is the node whose inputs changed — does not.
#[test]
fn the_declare_edge_recomputes_the_union_alone() {
    let base = ProfileDoc::empty_derived("docm7_memo", Tol::witness());
    let (base, a) = block(base, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (base, b) = block(base, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let (bare, _) = insert(
        base.clone(),
        Node::Union {
            members: vec![a, b],
            declare: None,
        },
    );
    let prior = run(&bare);
    // Disjoint members, so the undeclared union evaluates; the second
    // document declares a contact that does not exist, which is what
    // makes the union recompute and refuse while its members do not.
    let (doc, _union, _) = declared_union(base, &[a, b], |u| {
        vec![(
            member_face(u, a, fname(a, wall(0))),
            member_face(u, b, fname(b, wall(2))),
        )]
    });
    let ev = evaluate::<f64>(
        &doc,
        Some(&prior),
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    // Everything the two documents share is reused: the members and
    // their whole upstream. What is not is the union and its Declare.
    assert_eq!(
        ev.reused,
        doc.order().len() - 2,
        "the declare edge recomputed more than the union and its declaration"
    );
    assert_eq!(
        ev.value(a).expect("the member evaluated").content_key,
        prior.value(a).expect("the member evaluated").content_key,
        "a member's identity moved when the union gained a declaration"
    );
}

/// **The wire replays a union carrying a declaration, bit for bit.**
#[test]
fn a_declared_union_replays_bit_identically() {
    let tol = Tol::witness();
    let doc = ProfileDoc::empty_derived("docm7_wire", tol);
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, union, _) = declared_union(doc, &[a, b], |u| flush_pairs(u, (a, a), (b, b)));
    let text = editor_core::persist::save(&doc, &[], tol).expect("the document saves");
    let loaded = editor_core::persist::load(&text, tol).expect("the document loads");
    assert!(
        loaded.doc.bit_eq(&doc),
        "the loaded document is not the one that was saved"
    );
    let Some(Node::Union { declare, .. }) = loaded.doc.node(union) else {
        panic!("the union survived as something else")
    };
    assert!(declare.is_some(), "the declare edge did not survive the wire");
}
