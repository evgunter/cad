//! **The n-ary union's declaration channel, in member space** (DOCM-7;
//! DM4 as amended): the `declare` edge, the routing of each pair to
//! the fold step that joins the two things it names, the resolver the
//! union shares with the pair boolean, and the `Merged` rows the
//! channel makes reachable.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use crate::corpus::body_of;
use crate::wire::doctored;
use editor_core::{
    BooleanOp, BooleanValue, CancelToken, CapEnd, DocEdit, EditError, EntityKind, Entry,
    EvalOptions, Evaluation, Node, NodeErrorKind, NodeResult, ProfileDoc, ProfileEdgeRef,
    ProfileVertexRef, RecipeNodeId, ResolveError, RoleSeg, SitedRef, StableName, ValuePayload,
    evaluate,
};
use fixture::{ang, fname, insert, len, on_frame, scl, step, table, wall};
pub(crate) use fixture::{flush_pairs, member_face};
use geom_core::Tol;

/// Evaluates, and holds every table the run produced to the N3
/// flatness rule on the way out. A tripwire over this suite's merged
/// rows, not the guard: the mint refuses a nested constituent before
/// a table is published, and the rows that carry the rule are
/// `docm8_flat_merged`'s (the corpus walk and the mint-site rows).
pub(crate) fn run(doc: &ProfileDoc) -> Evaluation<f64> {
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

pub(crate) fn failure(ev: &Evaluation<f64>, id: RecipeNodeId) -> Option<&NodeErrorKind> {
    match ev.nodes.get(&id) {
        Some(NodeResult::Failed(e)) => Some(&e.kind),
        _ => None,
    }
}

/// An axis-aligned block on the xy plane at height `z0`, extruded `dz`.
pub(crate) fn block(
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

/// A node's contact records, read out of its boolean value.
fn contacts_of(ev: &Evaluation<f64>, id: RecipeNodeId) -> topo::ContactRecords {
    match &ev.value(id).expect("the node evaluated").payload {
        ValuePayload::Boolean(BooleanValue::Body { contacts, .. }) => (**contacts).clone(),
        other => panic!("expected a boolean body, got {other:?}"),
    }
}

/// **A union carrying a declaration, in the two edits it takes.**
///
/// A declared pair names SITED entities — the face IN the member, with
/// the member beside it — so it names only what exists BEFORE the
/// union. The `Declare` goes in first and the union carrying its edge
/// second; nothing is rebound and no intermediate union is built.
pub(crate) fn declared_union(
    doc: ProfileDoc,
    members: &[RecipeNodeId],
    pairs: Vec<(SitedRef, SitedRef)>,
) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, decl) = insert(doc, Node::declare_rest(pairs));
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: members.to_vec(),
            declare: Some(decl),
        },
    );
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
    let named: Vec<RecipeNodeId> = vec![finding.pair.0.at, finding.pair.1.at];
    assert!(
        named.contains(&m1) && named.contains(&m2),
        "the refusal names both members: {named:?}"
    );
    // Declared at the members: the same two placements fuse, in two
    // edits — the `Declare` and the union that consumes it.
    let (doc, union, _) = declared_union(doc, &[m1, m2], flush_pairs((m1, proto), (m2, proto)));
    let ev = run(&doc);
    assert!(
        failure(&ev, union).is_none(),
        "the declared union refused: {:?}",
        failure(&ev, union)
    );
    let volume = topo::mass_properties(body_of(&ev, union), Tol::witness())
        .expect("the fused body has mass")
        .volume;
    assert_eq!(volume, 1.5, "the fused volume is the two blocks' union");
}

/// **The pair boolean declares between two placements of one
/// prototype**, which the site is what makes possible.
///
/// A transform contributes no role segment (N1), so two placements of
/// one prototype carry IDENTICAL tables: every name an author could
/// write resolves in BOTH operands, and a bare name could not say
/// which. A SITED name does — it names the operand it is read at — so
/// the pair boolean fuses the two placements from one `Declare`, in
/// one pass, with no member keying anywhere.
#[test]
fn the_pair_boolean_declares_between_two_placements_of_one_prototype() {
    let doc = ProfileDoc::empty_derived("docm7_pair_sited", Tol::witness());
    let (doc, proto) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, m1) = placed(doc, proto, 0.0);
    let (doc, m2) = placed(doc, proto, 0.5);
    // The four flush planes, each named ONCE in the prototype's
    // vocabulary and sited at the two placements that carry it.
    let (doc, decl) = insert(
        doc,
        Node::declare_rest(flush_pairs((m1, proto), (m2, proto))),
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
        failure(&ev, pair).is_none(),
        "the sited declaration refused: {:?}",
        failure(&ev, pair)
    );
    let volume = topo::mass_properties(body_of(&ev, pair), Tol::witness())
        .expect("the fused body has mass")
        .volume;
    assert_eq!(volume, 1.5, "the fused volume is the two blocks' union");
}

/// **A site that is neither operand refuses typed.** The site IS the
/// side, so a site the consumer does not have is a declaration it
/// cannot read: there is no table to resolve the name in, and the
/// door says that rather than guessing a side.
#[test]
fn a_site_that_is_neither_operand_refuses() {
    let doc = ProfileDoc::empty_derived("docm7_pair_site", Tol::witness());
    let (doc, proto) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, m1) = placed(doc, proto, 0.0);
    let (doc, m2) = placed(doc, proto, 0.5);
    // Sited at the PROTOTYPE, whose table holds the name — but which
    // is neither operand of the boolean below.
    let (doc, decl) = insert(
        doc,
        Node::declare_rest(vec![(
            SitedRef::new(proto, fname(proto, wall(0))),
            SitedRef::new(m2, fname(proto, wall(0))),
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
            Some(NodeErrorKind::DeclareSiteNotAnOperand { at }) if *at == proto
        ),
        "expected the site refusal, got {:?}",
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
    let (doc, union, _) = declared_union(doc, &[a, b], flush_pairs((a, a), (b, b)));
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
    assert_eq!(surfaces(folded), surfaces(paired));
    assert_eq!(curves(folded), curves(paired));
    assert_eq!(points(folded), points(paired));
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
    let (doc, union, _) = declared_union(doc, &[m1, m2], flush_pairs((m1, proto), (m2, proto)));
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
    for (a, b) in flush_pairs((m1, proto), (m2, proto)) {
        // The row is the union's, so the constituents are the two
        // sited names in the union's own member space.
        let mut set = vec![
            member_face(union, a.at, a.name),
            member_face(union, b.at, b.name),
        ];
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
    // `b` is a TRANSFORM of its own extrude, so the pair's b-side has
    // `at != name.node`: the site is the transform, the name is the
    // extrude's (N1 — a pass-through op mints nothing). A routing that
    // read the name's minting node instead of the site would look for
    // `b0`, which is in no member list here, and refuse.
    let build = |order: [usize; 3]| {
        let doc = ProfileDoc::empty_derived("docm7_routing", Tol::witness());
        let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
        let (doc, far) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
        let (doc, b0) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
        let (doc, b) = placed(doc, b0, 0.5);
        let all = [a, far, b];
        let members: Vec<RecipeNodeId> = order.iter().map(|i| all[*i]).collect();
        let (doc, union, _) = declared_union(doc, &members, flush_pairs((a, a), (b, b0)));
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
        topo::mass_properties(straight, Tol::witness())
            .expect("mass")
            .volume,
        topo::mass_properties(reordered, Tol::witness())
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
    // Reordering swaps which of the two touching members the pair verb
    // sees as operand A, and the pair emitter is not symmetric in that
    // role: the flush stretch is named for the A-side member
    // (`work/docm/the-pair-verbs-declared-merge-is-asymmetric-in-its-operands.md`).
    // What no longer follows it is which member's rims carry a
    // `Fragment(OrderAlong)`: the union numbers a member edge's pieces by
    // the cells the finished body cuts it into, counting the cells the
    // other member holds (`emit_union::rank_member_edges`), so both
    // members' cut rims are ranked in both orders.
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
    let mut both = vec![a, b];
    both.sort();
    assert_eq!(fragmented_members(&ev, union), both);
    assert_eq!(fragmented_members(&ev2, union2), both);
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
    // A name the member's table does not carry — a wall the block
    // does not have: the site routes, the lookup finds nothing.
    let (doc, union, _) = declared_union(
        doc,
        &[a, b],
        vec![(
            SitedRef::new(a, fname(a, wall(0))),
            SitedRef::new(b, fname(b, wall(7))),
        )],
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

/// **A declared member the list no longer holds refuses**, which is
/// the state `SetMembers` creates by removing one.
#[test]
fn a_declared_member_removed_by_set_members_refuses() {
    let doc = ProfileDoc::empty_derived("docm7_dropped", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, far) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, union, _) = declared_union(doc, &[a, b, far], flush_pairs((a, a), (b, b)));
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

/// **A row the fold MINTS cannot be declared at all** — the class
/// `an_accumulation_entity_paired_with_an_earlier_member_refuses`,
/// `the_unions_own_body_row_is_refused_as_unroutable_not_as_vanished`,
/// `a_fold_row_routed_before_the_step_that_mints_it_has_no_step` and
/// `two_fold_rows_are_carried_at_the_first_step_that_has_both` used to
/// measure as four evaluation refusals, now unrepresentable by type.
///
/// A declared entity is SITED at a node the declaration can name, and
/// the union's own rows — a `Seam`, a `Merged`, a `Fragment`, the
/// output body — exist only in the union's own evaluation, after the
/// `Declare` that would name them. There is no node to site them at.
///
/// In Rust the type says so and nothing can be built; a FILE can still
/// spell anything, so this is the row: a `Declare` whose pair side is
/// a bare name — the old shape, which is what a fold row would have to
/// arrive as — does not load.
#[test]
fn a_declared_pair_side_that_is_a_bare_name_does_not_load() {
    let tol = Tol::witness();
    let doc = ProfileDoc::empty_derived("docm7_bare_side", tol);
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, _union, decl) = declared_union(doc, &[a, b], flush_pairs((a, a), (b, b)));
    let text = editor_core::persist::save(&doc, &[], tol).expect("the document saves");
    // Doctored BY PATH, through the wire's own structure, so a field
    // rename breaks the probe instead of silently moving it.
    let bare_sided = doctored(&text, |wire| {
        let side = &mut wire["snapshot"]["nodes"][decl.0.to_string()]["Declare"]["pairs"][0][0][0];
        let bare = side["name"].clone();
        assert!(
            !bare.is_null(),
            "a sited side carries a name beside its site"
        );
        *side = bare;
    });
    let refused = editor_core::persist::load(&bare_sided, tol)
        .expect_err("a declared side without its site loaded");
    // The `Unreadable` DETAIL is serde's own, and stays serde's: a
    // shape mismatch has no analogue of the mate head's constructor
    // sentence, which refuses a name of the wrong KIND. What the row
    // holds it to is that the message names the SITE SHAPE — the two
    // fields a side has — so a reader is told what was expected and
    // not merely that the file is unreadable.
    let said = refused.to_string();
    for word in ["`at`", "`name`", "unknown field"] {
        assert!(
            said.contains(word),
            "the refusal does not say {word}: {said}"
        );
    }
    // The other half of the same shape: an EXTRA field on a side is
    // refused too, which is what `deny_unknown_fields` buys.
    let surprised = doctored(&text, |wire| {
        let side = &mut wire["snapshot"]["nodes"][decl.0.to_string()]["Declare"]["pairs"][0][0][0];
        side["surprise"] = serde_json::Value::from(1);
    });
    let said2 = editor_core::persist::load(&surprised, tol)
        .expect_err("an unknown field on a site loaded")
        .to_string();
    assert!(said2.contains("`surprise`"), "{said2}");
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
        &editor_core::RefusingReach,
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

/// **The insert door refuses a `Declare` whose NAME's node or whose
/// SITE is not live** — the two halves of the payload, each checked by
/// the door that owns it (`Node::payload_names`,
/// `Node::payload_read_sites`).
///
/// A declaration names what exists BEFORE its consumer, so neither
/// half can point forward the way a member-space name used to: this
/// row measures the door that keeps it that way.
#[test]
fn the_insert_door_refuses_a_declare_whose_name_or_site_is_not_live() {
    let doc = ProfileDoc::empty_derived("docm7_forward", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    // An id no node has yet — one past the last live node.
    let future = RecipeNodeId(doc.order().last().expect("a node").0 + 1);
    let refused = doc.apply(
        &DocEdit::InsertNode {
            node: Node::declare_rest(vec![(
                SitedRef::new(a, fname(future, wall(0))),
                SitedRef::new(b, fname(b, wall(0))),
            )]),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    );
    assert!(
        matches!(refused, Err(EditError::DeclareNamesMissingNode { .. })),
        "expected the payload-name door's refusal, got {refused:?}"
    );
    // The read-site door, on EACH side in turn. A door that yielded
    // only the first site of a pair would pass the first of these and
    // admit the second.
    for sides in [
        (
            SitedRef::new(future, fname(a, wall(0))),
            SitedRef::new(b, fname(b, wall(0))),
        ),
        (
            SitedRef::new(a, fname(a, wall(0))),
            SitedRef::new(future, fname(b, wall(0))),
        ),
    ] {
        let refused = doc.apply(
            &DocEdit::InsertNode {
                node: Node::declare_rest(vec![sides]),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        );
        assert!(
            matches!(refused, Err(EditError::ReadSiteMissingNode { at }) if at == future),
            "expected the read-site door's refusal, got {refused:?}"
        );
    }
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
    let (doc, union, decl) = declared_union(doc, &[a, b], flush_pairs((a, a), (b, b)));
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
    let (doc, _union, _) = declared_union(
        base,
        &[a, b],
        vec![(
            SitedRef::new(a, fname(a, wall(0))),
            SitedRef::new(b, fname(b, wall(2))),
        )],
    );
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
    let (doc, union, _) = declared_union(doc, &[a, b], flush_pairs((a, a), (b, b)));
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
// The carried record.
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
    let vertex = fixture::cap_vertex(
        a,
        CapEnd::End,
        ProfileVertexRef {
            loop_index: 0,
            vertex: 0,
        },
    );
    let face = fname(a, RoleSeg::Cap(CapEnd::Start));
    // Two entities of ONE member, sited there: the same pair reads as
    // that member's carried contact wherever the member sits.
    let carried = vec![(
        SitedRef::new(a, vertex.clone()),
        SitedRef::new(a, face.clone()),
    )];
    // The pair boolean's reading of the same claim, for reference.
    let (doc, pdecl) = insert(doc, Node::declare_rest(carried.clone()));
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
    let (doc, last, _) = declared_union(doc, &[far, a], carried.clone());
    // Member `a` as member 0 of a two-step fold: fed at the first step,
    // where it is operand A.
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
    // Fed at the FIRST step and consumed there: the value's records
    // are the LAST step's, which is exactly what the pair chain this
    // fold replaces publishes. Asserted against that chain rather
    // than against zero, so the row says what the record IS.
    let (doc, chain0) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b: far,
            declare: Some(pdecl),
        },
    );
    let (doc, chain1) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: chain0,
            b: far2,
            declare: None,
        },
    );
    let ev = run(&doc);
    assert!(failure(&ev, chain1).is_none(), "{:?}", failure(&ev, chain1));
    let (fold, chain) = (contacts_of(&ev, first), contacts_of(&ev, chain1));
    assert_eq!(
        (fold.b_on_a.len(), fold.a_on_b.len()),
        (chain.b_on_a.len(), chain.a_on_b.len()),
        "the fold publishes the last step's records, as the chain does"
    );
}

// ---------------------------------------------------------------------
// One pass: the document replays as it is ordered.
// ---------------------------------------------------------------------

/// **A declared union's document replays in document order** — what
/// the sited payload bought, measured as a document.
///
/// `a_declared_unions_document_loads_but_does_not_replay_in_order`
/// pinned the opposite: a `Declare` carrying member-space names was
/// written BEFORE the union it named, so the saved document held a
/// payload name pointing FORWARD in `order()`; the file round-tripped
/// because the load door checks the mint counter rather than the
/// order, and re-inserting the same nodes in document order refused at
/// the `Declare`. A sited declaration names only what precedes it, so
/// the forward reference is gone and the asymmetry with it: the same
/// document saves, loads, AND rebuilds edit by edit.
#[test]
fn a_declared_unions_document_replays_in_document_order() {
    let doc = ProfileDoc::empty_derived("docm7_forward_ref", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, union, decl) = declared_union(doc, &[a, b], flush_pairs((a, a), (b, b)));
    // The `Declare` precedes the union that consumes it, and names
    // nothing that comes after itself.
    let positions = |id: RecipeNodeId| doc.order().iter().position(|n| *n == id);
    assert!(
        positions(decl) < positions(union),
        "the Declare comes first"
    );
    let Some(Node::Declare { pairs }) = doc.node(decl) else {
        panic!("the Declare survived as something else")
    };
    for r in pairs.iter().flat_map(|((x, y), _)| [x, y]) {
        assert!(positions(r.at) < positions(decl), "a site points forward");
        assert!(
            positions(r.name.node) < positions(decl),
            "a name points forward"
        );
    }
    // Saved and read back: the forward reference is fine on file.
    let text = editor_core::persist::save(&doc, &[], Tol::witness()).expect("the document saves");
    let loaded = editor_core::persist::load(&text, Tol::witness())
        .expect("and loads")
        .doc;
    assert_eq!(loaded.order(), doc.order());
    let ev = run(&loaded);
    assert!(failure(&ev, union).is_none(), "{:?}", failure(&ev, union));
    // Rebuilding by re-inserting the nodes in document order works:
    // every payload name and every site is live by the time its
    // carrier arrives.
    let mut replay = ProfileDoc::empty_derived("docm7_forward_ref_replay", Tol::witness());
    for id in doc.order() {
        let node = doc.node(*id).expect("a live node").clone();
        replay = replay
            .apply(
                &DocEdit::InsertNode { node },
                Tol::witness(),
                &editor_core::RefusingReach,
            )
            .unwrap_or_else(|e| panic!("re-inserting {id:?} refused: {e:?}"))
            .doc;
    }
    assert_eq!(replay.order().len(), doc.order().len());
    let ev = run(&replay);
    let rebuilt = replay.order()[doc
        .order()
        .iter()
        .position(|n| *n == union)
        .expect("the union")];
    assert!(
        failure(&ev, rebuilt).is_none(),
        "{:?}",
        failure(&ev, rebuilt)
    );
}

// ---------------------------------------------------------------------
// The refusal against a row the fold minted.
// ---------------------------------------------------------------------

/// **A union's undeclared contact against a MERGED row is refused
/// `UndeclaredContact` sited at a CONSTITUENT of that merge**, with
/// the whole flat set beside it.
///
/// Two placements of one prototype are declared flush, so their y=0
/// walls fuse into one `Merged` row; a third block rests flush under
/// that merged wall, undeclared. The merged row is the union's own,
/// so no `SitedRef` names it — but every constituent is a member's
/// face, and declaring the contact at any of them resolves back to
/// the merged row through the look-through. The refusal therefore
/// carries a pair the caller can declare verbatim, and names the
/// constituents it chose between.
#[test]
fn a_union_refusal_against_a_merged_wall_is_sited_at_a_constituent() {
    let doc = ProfileDoc::empty_derived("rv_r2_merged_refusal", Tol::witness());
    let (doc, proto) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, m1) = placed(doc, proto, 0.0);
    let (doc, m2) = placed(doc, proto, 0.5);
    // Flush under the merged y=0 wall (x 0..1.5 once m1 and m2 fuse).
    let (doc, m3) = block(doc, (0.0, 1.5), (-1.0, 0.0), 0.0, 1.0);
    let (doc, union, _) = declared_union(doc, &[m1, m2, m3], flush_pairs((m1, proto), (m2, proto)));
    let ev = run(&doc);
    let what = failure(&ev, union);
    let Some(NodeErrorKind::UndeclaredContact {
        finding, merged, ..
    }) = what
    else {
        panic!("the refusal a caller can act on, got {what:?}")
    };
    // The merged side is the ACCUMULATION's: the constituents are the
    // two placements' own walls, in member order, and the finding's
    // side is the first of them.
    assert_eq!(
        merged.0.iter().map(|r| r.at).collect::<Vec<_>>(),
        vec![m1, m2],
        "the constituents, in member order"
    );
    assert!(merged.1.is_empty(), "the joining member's side is its own");
    assert_eq!(finding.pair.0, merged.0[0], "the finding takes the first");
    assert_eq!(
        finding.pair.1.at, m3,
        "the other side is the joining member"
    );
    // Declared verbatim, the same document fuses: any constituent
    // names the merged row through the look-through.
    let mut pairs = flush_pairs((m1, proto), (m2, proto));
    pairs.push((finding.pair.0.clone(), finding.pair.1.clone()));
    let (doc, again, _) = declared_union(doc.clone(), &[m1, m2, m3], pairs);
    let ev = run(&doc);
    assert!(failure(&ev, again).is_none(), "{:?}", failure(&ev, again));
}

/// **The site is the OPERAND, not the minting node** — through a
/// pass-through `Transform` (N1), and the name resolves in the
/// transform's table.
#[test]
fn a_pass_through_operand_is_the_site_and_the_minting_node_is_not() {
    let build = |site_at_extrude: bool| {
        let doc = ProfileDoc::empty_derived("rv_r2_passthrough", Tol::witness());
        let (doc, proto) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
        let (doc, m1) = placed(doc, proto, 0.0);
        let (doc, m2) = placed(doc, proto, 0.5);
        let (a_at, b_at) = if site_at_extrude {
            (proto, proto)
        } else {
            (m1, m2)
        };
        let pairs: Vec<(SitedRef, SitedRef)> = [
            wall(0),
            wall(2),
            RoleSeg::Cap(CapEnd::Start),
            RoleSeg::Cap(CapEnd::End),
        ]
        .into_iter()
        .map(|seg| {
            (
                SitedRef::new(a_at, fname(proto, seg.clone())),
                SitedRef::new(b_at, fname(proto, seg)),
            )
        })
        .collect();
        let (doc, decl) = insert(doc, Node::declare_rest(pairs));
        let (doc, pair) = insert(
            doc,
            Node::Boolean {
                op: BooleanOp::Union,
                a: m1,
                b: m2,
                declare: Some(decl),
            },
        );
        (doc, pair, proto)
    };
    let (doc, pair, _) = build(false);
    let ev = run(&doc);
    assert!(failure(&ev, pair).is_none(), "{:?}", failure(&ev, pair));
    let (doc, pair, proto) = build(true);
    let ev = run(&doc);
    assert!(
        matches!(
            failure(&ev, pair),
            Some(NodeErrorKind::DeclareSiteNotAnOperand { at }) if *at == proto
        ),
        "expected the site refusal, got {:?}",
        failure(&ev, pair)
    );
}

/// **A name the sited operand does not carry refuses `Vanished`, and
/// a dead minting node outranks it** — rungs 3 and 1 of N5.
#[test]
fn a_name_the_site_does_not_carry_refuses_vanished_under_node_gone() {
    let doc = ProfileDoc::empty_derived("rv_r2_rungs", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, spare) = block(doc, (8.0, 9.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, decl) = insert(
        doc,
        Node::declare_rest(vec![(
            SitedRef::new(a, fname(a, wall(0))),
            SitedRef::new(b, fname(spare, wall(0))),
        )]),
    );
    let (doc, pair) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b,
            declare: Some(decl),
        },
    );
    let ev = run(&doc);
    assert!(
        matches!(
            failure(&ev, pair),
            Some(NodeErrorKind::DeclareResolve { error })
                if matches!(**error, ResolveError::Vanished { .. })
        ),
        "expected rung 3, got {:?}",
        failure(&ev, pair)
    );
    let (doc, _) = step(doc, DocEdit::DeleteNode { id: spare });
    let ev = run(&doc);
    assert!(
        matches!(
            failure(&ev, pair),
            Some(NodeErrorKind::DeclareResolve { error })
                if matches!(**error, ResolveError::NodeGone { .. })
        ),
        "expected rung 1, got {:?}",
        failure(&ev, pair)
    );
}

/// **A deleted SITE is refused at the next evaluation**, and with which
/// arm.
#[test]
fn a_deleted_site_refuses_at_the_next_evaluation() {
    let doc = ProfileDoc::empty_derived("rv_r2_deleted_site", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, far) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, union, _) = declared_union(doc, &[a, b, far], flush_pairs((a, a), (b, b)));
    let (doc, _) = step(
        doc,
        DocEdit::SetMembers {
            node: union,
            members: vec![a, far],
        },
    );
    let ev = run(&doc);
    assert!(
        failure(&ev, union).is_some(),
        "a site that left the member list must refuse"
    );
    let applied = doc
        .apply(
            &DocEdit::DeleteNode { id: b },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .expect("a delete never refuses over a dangling reference (N5)");
    let ev = run(&applied.doc);
    assert!(
        failure(&ev, union).is_some(),
        "a deleted site refuses at the next evaluation, got {:?}",
        failure(&ev, union)
    );
}

/// **`Rebind` a declared name onto a name minted elsewhere while the
/// site stays** — what refuses, and where.
#[test]
fn rebind_moves_the_name_and_leaves_the_site() {
    let doc = ProfileDoc::empty_derived("rv_r2_rebind", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, spare) = block(doc, (8.0, 9.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, union, decl) = declared_union(doc, &[a, b], flush_pairs((a, a), (b, b)));
    let ev = run(&doc);
    assert!(failure(&ev, union).is_none(), "{:?}", failure(&ev, union));
    let from = fname(a, wall(0));
    let to = fname(spare, wall(0));
    let applied = doc
        .apply(
            &DocEdit::Rebind {
                from: from.clone(),
                to: to.clone(),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .expect("the rebind applies");
    let Some(Node::Declare { pairs }) = applied.doc.node(decl) else {
        panic!("the Declare survived as something else")
    };
    let moved = pairs
        .iter()
        .flat_map(|((x, y), _)| [x, y])
        .find(|r| r.name == to)
        .expect("the name moved");
    assert_eq!(moved.at, a, "the site stayed where it was authored");
    let ev = run(&applied.doc);
    assert!(
        matches!(
            failure(&ev, union),
            Some(NodeErrorKind::DeclareResolve { error })
                if matches!(**error, ResolveError::Vanished { .. })
        ),
        "expected rung 3, got {:?}",
        failure(&ev, union)
    );
}

/// **The site is the side, in the one direction that can tell**: a name
/// the SITED operand does not carry, which the OTHER operand does,
/// refuses `Vanished` — it is not silently read in the other table.
///
/// Every existing row sites each name at the operand whose table holds
/// it, so a resolver that ignored the site and fell back to the other
/// table would pass them all.
#[test]
fn a_name_the_other_operand_carries_is_not_read_at_its_site() {
    let doc = ProfileDoc::empty_derived("rv_r2_wrong_side", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    // Both sides name entities of `b`; the first is SITED at `a`,
    // whose table does not carry it.
    let (doc, decl) = insert(
        doc,
        Node::declare_rest(vec![(
            SitedRef::new(a, fname(b, wall(0))),
            SitedRef::new(b, fname(b, wall(2))),
        )]),
    );
    let (doc, pair) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b,
            declare: Some(decl),
        },
    );
    let ev = run(&doc);
    assert!(
        matches!(
            failure(&ev, pair),
            Some(NodeErrorKind::DeclareResolve { error })
                if matches!(**error, ResolveError::Vanished { .. })
        ),
        "a name read in the operand its site does not name: {:?}",
        failure(&ev, pair)
    );
}

/// **A `Declare` has no inputs at all** — its two sides are
/// REFERENCES, not DAG edges (D3's name-reference carve-out), and the
/// site is a node id the node reads at rather than one it consumes.
///
/// The content key leans on exactly this when it feeds both sites
/// (`eval::content_key`'s `Declare` arm): a node id in a key is
/// otherwise a Merkle link to an input (D8), and the arm's reason for
/// feeding one anyway is that this node has none.
#[test]
fn a_declare_has_no_inputs() {
    let doc = ProfileDoc::empty_derived("docm7_no_inputs", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, decl) = insert(doc, Node::declare_rest(flush_pairs((a, a), (b, b))));
    let node = doc.node(decl).expect("the Declare is live");
    assert!(
        node.inputs().is_empty(),
        "a Declare's sides are references, not edges: {:?}",
        node.inputs()
    );
    // And the sites are what `payload_read_sites` answers with — the
    // reading edges, which is the other half of the same fact.
    let sites: Vec<RecipeNodeId> = node.payload_read_sites();
    assert_eq!(sites.len(), 8, "two sites per pair, four pairs");
    assert!(sites.iter().all(|at| *at == a || *at == b), "{sites:?}");
}
