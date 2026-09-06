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
    ProfileEdgeRef, ProfileVertexRef, Qualifier, RecipeNodeId, RoleSeg, StableName, ValuePayload,
    evaluate,
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

/// One ENTITY of one member, as the UNION's own name space spells it —
/// the row `member_view` puts into that member's operand table.
fn member_entity(
    union: RecipeNodeId,
    member: RecipeNodeId,
    of: StableName,
    kind: EntityKind,
) -> StableName {
    StableName {
        kind,
        node: union,
        path: vec![RoleSeg::FromMember {
            member,
            of: Box::new(of),
        }],
    }
}

/// The same, for the face case every row but the carried-contact one
/// wants.
fn member_face(union: RecipeNodeId, member: RecipeNodeId, of: StableName) -> StableName {
    member_entity(union, member, of, EntityKind::Face)
}

/// A node's contact records, read out of its boolean value.
fn contacts_of(ev: &Evaluation<f64>, id: RecipeNodeId) -> topo::ContactRecords {
    match &ev.value(id).expect("the node evaluated").payload {
        ValuePayload::Boolean(BooleanValue::Body { contacts, .. }) => (**contacts).clone(),
        other => panic!("expected a boolean body, got {other:?}"),
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
    // One rebind per DISTINCT name. A name can appear in more than one
    // pair — a chain of contacts declares the middle member's faces
    // twice — and the second `Rebind` of one name would refuse
    // `RebindNoReferences`, the first having already moved every
    // reference to it.
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
        panic!(
            "expected the undeclared contact, got {:?}",
            failure(&ev, plain)
        );
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
    let (doc, union, _) =
        declared_union(doc, &[m1, m2], |u| flush_pairs(u, (m1, proto), (m2, proto)));
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
        Node::declare_rest(vec![(fname(proto, wall(0)), fname(proto, wall(0)))]),
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
///
/// **Description-level equality is the CEILING here**, and the
/// comparison says so: it sorts the two bodies' descriptions and
/// compares those, rather than asking for bit identity. Two bodies
/// minted by two different nodes cannot be bit-identical — every
/// minted description carries its own `GeomSource.node` (D1), which is
/// the union's in one and the boolean's in the other — so a `bit_eq`
/// between them would be measuring the node ids and failing on them.
/// What is comparable is what the two verbs computed, and that is what
/// is compared.
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
    let (doc, union, _) =
        declared_union(doc, &[m1, m2], |u| flush_pairs(u, (m1, proto), (m2, proto)));
    let ev = run(&doc);
    let t = table(&ev, union);
    // Four merged faces: the two y-walls and the two caps.
    let merged: Vec<&StableName> = t
        .iter()
        .map(|(n, _)| n)
        .filter(|n| matches!(n.path.first(), Some(RoleSeg::Merged(_))))
        .collect();
    assert_eq!(
        merged.len(),
        4,
        "the four declared contacts merged: {merged:?}"
    );
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
        let (doc, union, _) = declared_union(doc, &members, |u| flush_pairs(u, (a, a), (b, b)));
        (doc, union, a, b)
    };
    // Members (a, far, b): the declared pair belongs to step 3.
    let (doc, union, a, b) = build([0, 1, 2]);
    let ev = run(&doc);
    assert!(
        failure(&ev, union).is_none(),
        "the routed declaration refused: {:?}",
        failure(&ev, union)
    );
    let straight = body_of(&ev, union);
    // Members (b, a, far): the same two member ids, now the FIRST two,
    // so the same pair is fed at step 1 instead — derived, not stored.
    let (doc2, union2, a2, b2) = build([2, 0, 1]);
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
    // And the asymmetry, ASSERTED rather than explained: reordering
    // swaps which of the two touching members the pair verb sees as
    // operand A, and the pair emitter is not symmetric in that role.
    // The rim edges the merge splits are the A-side member's, so the
    // `Fragment(OrderAlong)` rows sit on `a`'s rims under (a, far, b)
    // and on `b`'s under (b, a, far). Volume, counts and the four
    // merges are the same; which member's names move is the PAIR
    // verb's, and this node inherits it
    // (`work/docm/the-pair-verbs-declared-merge-is-asymmetric-in-its-operands.md`).
    let fragmented_members = |ev: &Evaluation<f64>, id: RecipeNodeId| {
        let mut out: Vec<RecipeNodeId> = table(ev, id)
            .iter()
            .filter(|(n, _)| n.path.iter().any(|s| matches!(s, RoleSeg::Fragment(_))))
            .filter_map(|(n, _)| match n.path.first() {
                Some(RoleSeg::FromMember { member, .. }) => Some(*member),
                _ => None,
            })
            .collect();
        out.sort();
        out.dedup();
        out
    };
    assert_eq!(fragmented_members(&ev, union), vec![a]);
    assert_eq!(fragmented_members(&ev2, union2), vec![b2]);
    // The two documents are built separately, so the ids are the same
    // ones in the same seats: `a` is the first block of both.
    assert_eq!((a, b), (a2, b2));
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
        matches!(
            failure(&ev, union),
            Some(NodeErrorKind::DeclareResolve { .. })
        ),
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
        matches!(
            failure(&ev, union),
            Some(NodeErrorKind::DeclareResolve { .. })
        ),
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
        matches!(
            failure(&ev, union),
            Some(NodeErrorKind::UnionDeclareStep { .. })
        ),
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

/// **A union whose member list ends in a `Declare` refuses, typed.**
///
/// It is the twin of the declared union above: `[a, b]` with a
/// declaration `d` and `[a, b, d]` with none present the same three
/// upstream keys in the same order, which is why the content key feeds
/// the member count (`eval::content_key`).
///
/// What this row pins is the MIS-WIRE's own refusal, not that feed. The
/// feed cannot be pinned by any row: a memo is looked up by node id
/// before its key is compared, a foreign prior is dropped, and no edit
/// turns one of these two nodes into the other under one id — so there
/// is no document in which the collision is reachable. The feed is D8
/// key hygiene, argued at its own site and stated there as unguardable;
/// this row measures the half that IS reachable.
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
    // The mis-wire has no value of its own: it refuses the undeclared
    // contact at step 1, which it reaches before the declaration at a
    // body seat.
    assert!(
        ev.value(miswired).is_none(),
        "the mis-wired union produced a body"
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
    assert!(
        declare.is_some(),
        "the declare edge did not survive the wire"
    );
}

// ---------------------------------------------------------------------
// The order bound, measured (work/docm/member-space-declarations-are-
// order-shaped-across-a-chain.md).
// ---------------------------------------------------------------------

/// **A CHAIN of declared contacts fuses or refuses by member order.**
///
/// Three blocks, `a` meeting `c` and `c` meeting `d`, both contacts
/// declared in member space and nothing else. The orders that fold `c`
/// in LAST fuse; the orders that fold it in second refuse, because the
/// first step's declared merge consumed `c`'s faces and published
/// `Merged` rows in their place, so the second contact's names are no
/// longer operand rows when their step runs.
///
/// This asserts what the tree does TODAY, both halves, and it is the
/// measurement `work/docm/member-space-declarations-are-order-shaped-across-a-chain.md`
/// asks Ev to rule on — constituent look-through, flattened `Merged`
/// rows, or a narrowed contract. It flips when the ruling lands.
#[test]
fn member_space_declarations_across_a_chain_are_order_shaped() {
    let base = |label: &str| {
        let doc = ProfileDoc::empty_derived(label, Tol::witness());
        let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
        let (doc, c) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
        let (doc, d) = block(doc, (1.2, 2.2), (0.0, 1.0), 0.0, 1.0);
        (doc, a, c, d)
    };
    let build = |label: &str, order: [usize; 3]| {
        let (doc, a, c, d) = base(label);
        let all = [a, c, d];
        let members: Vec<RecipeNodeId> = order.iter().map(|i| all[*i]).collect();
        let (doc, union, _) = declared_union(doc, &members, |u| {
            let mut v = flush_pairs(u, (a, a), (c, c));
            v.extend(flush_pairs(u, (c, c), (d, d)));
            v
        });
        (doc, union)
    };
    // `c` folded in LAST: both contacts are still member faces at the
    // step that needs them, and the chain fuses into one body.
    for (label, order) in [
        ("docm7_chain_adc", [0, 2, 1]),
        ("docm7_chain_dac", [2, 0, 1]),
    ] {
        let (doc, union) = build(label, order);
        let ev = run(&doc);
        assert!(
            failure(&ev, union).is_none(),
            "{label}: the chain refused: {:?}",
            failure(&ev, union)
        );
        let body = body_of(&ev, union);
        let volume = topo::mass_properties(&body, Tol::witness())
            .expect("mass")
            .volume;
        assert!(
            (volume - 2.2).abs() < 1e-9,
            "{label}: one fused body of the chain's volume, got {volume}"
        );
    }
    // `c` folded in SECOND: its faces were merged away at step 1, so
    // the second contact's names no longer resolve. The refusal is the
    // vanished rung — the face existed and was consumed, which is what
    // the diagnosis says — and NOT `UnionDeclareStep`: the pair was
    // routed to the right step, and the step is not what is wrong.
    for (label, order) in [
        ("docm7_chain_acd", [0, 1, 2]),
        ("docm7_chain_cda", [1, 2, 0]),
    ] {
        let (doc, union) = build(label, order);
        let ev = run(&doc);
        assert!(
            matches!(
                failure(&ev, union),
                Some(NodeErrorKind::DeclareResolve { .. })
            ),
            "{label}: expected the vanished refusal, got {:?}",
            failure(&ev, union)
        );
    }
}

// ---------------------------------------------------------------------
// The diagnosis class: what this node denotes but no step has.
// ---------------------------------------------------------------------

/// **The union's own body row is no step's operand**, and says so.
///
/// `OutputBody` is a row this node PUBLISHES — the undeclared union of
/// the same members carries it — so "no table derives this name any
/// more" would be false. What is true is that a step's output body is
/// not one of that step's two inputs, which is `UnionDeclareStep`.
#[test]
fn the_unions_own_body_row_is_refused_as_unroutable_not_as_vanished() {
    let doc = ProfileDoc::empty_derived("docm7_output_body", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, union, _) = declared_union(doc, &[a, b], |u| {
        vec![(
            StableName {
                kind: EntityKind::Body,
                node: u,
                path: vec![RoleSeg::OutputBody],
            },
            member_face(u, b, fname(b, wall(0))),
        )]
    });
    let ev = run(&doc);
    assert!(
        matches!(
            failure(&ev, union),
            Some(NodeErrorKind::UnionDeclareStep { .. })
        ),
        "expected the unroutable refusal, got {:?}",
        failure(&ev, union)
    );
    // The name it refused IS one this node publishes: the same two
    // members with no declaration at all carry an `OutputBody` row.
    let far = ProfileDoc::empty_derived("docm7_output_body_bare", Tol::witness());
    let (far, p) = block(far, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (far, q) = block(far, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let (far, bare) = insert(
        far,
        Node::Union {
            members: vec![p, q],
            declare: None,
        },
    );
    let ev2 = run(&far);
    assert!(
        table(&ev2, bare)
            .iter()
            .any(|(n, _)| n.path.as_slice() == [RoleSeg::OutputBody]),
        "a union publishes an OutputBody row"
    );
}

/// **A fold row routed to a step that does not hold it yet is
/// unroutable, not vanished.**
///
/// The bucket a fold row is sent to is bounded by the LATEST member it
/// mentions, and that bound is not tight: a row whose path mentions
/// only member 0 can be minted at any later step. Paired with a member
/// that joins at the earlier bucket, it routes there — and the
/// accumulation at that step has no such row. The name is this node's
/// and the fold does mint rows of that shape, so the honest answer is
/// that no step has both names as operands.
#[test]
fn a_fold_row_routed_before_the_step_that_mints_it_has_no_step() {
    let doc = ProfileDoc::empty_derived("docm7_early_bucket", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, far) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, (8.0, 9.0), (0.0, 1.0), 0.0, 1.0);
    // `[FromMember{a}, Fragment(OrderAlong)]`: the shape the fold mints
    // for a fragment of member 0's own entity. It mentions member 0 and
    // nothing else, so it is bounded to bucket 1 — where the
    // accumulation, three disjoint blocks, carries no fragment at all.
    let fragment_of_a = |u: RecipeNodeId| StableName {
        kind: EntityKind::Face,
        node: u,
        path: vec![
            RoleSeg::FromMember {
                member: a,
                of: Box::new(fname(a, wall(0))),
            },
            RoleSeg::Fragment(Qualifier::OrderAlong { rank: 0, of: 2 }),
        ],
    };
    let (doc, union, _) = declared_union(doc, &[a, far, c], |u| {
        vec![(fragment_of_a(u), member_face(u, c, fname(c, wall(0))))]
    });
    let ev = run(&doc);
    assert!(
        matches!(
            failure(&ev, union),
            Some(NodeErrorKind::UnionDeclareStep { .. })
        ),
        "expected the unroutable refusal, got {:?}",
        failure(&ev, union)
    );
}

// ---------------------------------------------------------------------
// A2's two owed rows: the carried record, and two fold rows.
// ---------------------------------------------------------------------

/// **Two names in ONE member are that member's carried contact, fed at
/// that member's own step** — the pair chain's rule, on a member.
///
/// A Vertex–Face pair inside one member is a carried 3′ contact. As
/// operand B of the LAST step it reaches the union's value exactly as
/// the pair boolean's does; as member 0 of a two-step fold it is fed at
/// step 1, where member 0 is operand A, and does not reach the value —
/// a contact fed at a step before the last is consumed there, which is
/// what the pair chain does with it too.
#[test]
fn a_same_member_declared_pair_is_a_carried_record_at_its_step() {
    let doc = ProfileDoc::empty_derived("docm7_carried", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, far) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, far2) = block(doc, (8.0, 9.0), (0.0, 1.0), 0.0, 1.0);
    let vertex = StableName {
        kind: EntityKind::Vertex,
        node: a,
        path: vec![RoleSeg::CapVertex(
            CapEnd::End,
            ProfileVertexRef {
                loop_index: 0,
                vertex: 0,
            },
        )],
    };
    let face = fname(a, RoleSeg::Cap(CapEnd::Start));
    let carried = |u: RecipeNodeId| {
        vec![(
            member_entity(u, a, vertex.clone(), EntityKind::Vertex),
            member_face(u, a, face.clone()),
        )]
    };
    // The pair boolean's reading of the same claim, for reference.
    let (doc, pdecl) = insert(
        doc,
        Node::declare_rest(vec![(vertex.clone(), face.clone())]),
    );
    let (doc, pair) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: far,
            b: a,
            declare: Some(pdecl),
        },
    );
    // Member `a` as operand B of the LAST step.
    let (doc, last, _) = declared_union(doc, &[far, a], carried);
    // Member `a` as member 0 of a two-step fold: fed at step 1.
    let (doc, first, _) = declared_union(doc, &[a, far, far2], carried);
    let ev = run(&doc);
    for id in [pair, last, first] {
        assert!(failure(&ev, id).is_none(), "{id:?}: {:?}", failure(&ev, id));
    }
    assert_eq!(
        contacts_of(&ev, pair).b_on_a.len(),
        1,
        "the pair boolean carries the claim"
    );
    assert_eq!(
        contacts_of(&ev, last).b_on_a.len(),
        1,
        "the union fed it as operand B's carried record at the last step"
    );
    assert_eq!(
        contacts_of(&ev, first).b_on_a.len(),
        0,
        "fed at step 1, consumed there, and the value is the last step's"
    );
}

/// **Two fold rows are the accumulation's own carried contact, at the
/// first step that has both** — deviation 8's arm, reached.
///
/// Two flush placements of one prototype, declared, mint `Merged` rows
/// at step 1. A pair naming two of THOSE rows routes to the first step
/// after them: with a third member there is such a step, both rows
/// resolve in the accumulation, and the v1 vocabulary refuses a
/// same-operand Face–Face pair — which is the proof the step held both.
/// With no third member there is no step left, and it is unroutable.
#[test]
fn two_fold_rows_are_carried_at_the_first_step_that_has_both() {
    let doc = ProfileDoc::empty_derived("docm7_two_rows", Tol::witness());
    let (doc, proto) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, m1) = placed(doc, proto, 0.0);
    let (doc, m2) = placed(doc, proto, 0.5);
    let (doc, far) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    // The `Merged` row the declared contact mints, spelled as
    // `collapse` spells it: the constituent SET, sorted.
    let merged_row = |u: RecipeNodeId, seg: RoleSeg| {
        let mut set = vec![
            member_face(u, m1, fname(proto, seg.clone())),
            member_face(u, m2, fname(proto, seg)),
        ];
        set.sort();
        StableName {
            kind: EntityKind::Face,
            node: u,
            path: vec![RoleSeg::Merged(set)],
        }
    };
    let pairs = |u: RecipeNodeId| {
        let mut v = flush_pairs(u, (m1, proto), (m2, proto));
        v.push((merged_row(u, wall(0)), merged_row(u, wall(2))));
        v
    };
    let (three, union3, _) = declared_union(doc.clone(), &[m1, m2, far], pairs);
    let ev = run(&three);
    match failure(&ev, union3) {
        Some(NodeErrorKind::DeclareUnsupportedPair {
            kinds,
            cross_operand,
        }) => {
            assert_eq!(*kinds, (EntityKind::Face, EntityKind::Face));
            assert!(!cross_operand, "both rows were found in ONE operand");
        }
        other => panic!("expected the same-operand vocabulary refusal, got {other:?}"),
    }
    let (two, union2, _) = declared_union(doc, &[m1, m2], pairs);
    let ev = run(&two);
    assert!(
        matches!(
            failure(&ev, union2),
            Some(NodeErrorKind::UnionDeclareStep { .. })
        ),
        "with no step after the merge the pair is unroutable, got {:?}",
        failure(&ev, union2)
    );
}

// ---------------------------------------------------------------------
// The asymmetry the two-pass construction relies on.
// ---------------------------------------------------------------------

/// **A declared union's document LOADS, and cannot be rebuilt by
/// re-inserting its nodes in order.**
///
/// The `Declare` carrying member-space names is written BEFORE the
/// union it names (it has to be: the union's edge points at it), so the
/// saved document holds a payload name pointing FORWARD in `order()`.
/// The load door checks the mint counter and not the order, so the file
/// round-trips; the edit door refuses a payload name whose node is not
/// yet live, so replaying the same nodes in document order does not
/// rebuild it. This node is the first to rely on that asymmetry, so it
/// is stated as a row rather than left as an observation — the
/// authoring consequence is filed as
/// `work/chrome/a-declared-union-has-no-one-pass-authoring-path.md`.
#[test]
fn a_declared_unions_document_loads_but_does_not_replay_in_order() {
    let doc = ProfileDoc::empty_derived("docm7_forward_ref", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, union, decl) = declared_union(doc, &[a, b], |u| flush_pairs(u, (a, a), (b, b)));
    // The `Declare` precedes the union it names, and its payload names
    // that union.
    let positions = |id: RecipeNodeId| doc.order().iter().position(|n| *n == id);
    assert!(
        positions(decl) < positions(union),
        "the Declare comes first"
    );
    // Saved and read back: the forward reference is fine on file.
    let text = editor_core::persist::save(&doc, &[], Tol::witness()).expect("the document saves");
    let loaded = editor_core::persist::load(&text, Tol::witness())
        .expect("and loads")
        .doc;
    assert_eq!(loaded.order(), doc.order());
    let ev = run(&loaded);
    assert!(failure(&ev, union).is_none(), "{:?}", failure(&ev, union));
    // Rebuilding by re-inserting the nodes in document order refuses at
    // the `Declare`, whose payload names a union that does not exist
    // yet in the new document.
    let fresh = ProfileDoc::empty_derived("docm7_forward_ref_replay", Tol::witness());
    let mut replay = fresh;
    let mut refused = None;
    for id in doc.order() {
        let node = doc.node(*id).expect("a live node").clone();
        match replay.apply(&DocEdit::InsertNode { node }, Tol::witness()) {
            Ok(applied) => replay = applied.doc,
            Err(e) => {
                refused = Some(e);
                break;
            }
        }
    }
    assert!(
        matches!(refused, Some(EditError::DeclareNamesMissingNode { .. })),
        "expected the edit door's forward-name refusal, got {refused:?}"
    );
}
