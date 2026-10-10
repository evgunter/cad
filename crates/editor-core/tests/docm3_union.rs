//! **The n-ary union** (DOCM-3; DM4–DM6): names keyed by member and
//! not by position, the fold that equals the chain it replaces, the
//! list-input edit, and the repeated-read glue at every door.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::ExtrudeSide;

use crate::corpus::body_of;
use editor_core::{
    CancelToken, DocEdit, EditError, EntityKind, EvalOptions, Evaluation, Node, ProfileDoc,
    RecipeNodeId, RoleSeg, StableName, all_faces, evaluate,
};
use fixture::{insert, len, on_frame};
use geom_core::Tol;

/// Evaluates, and holds every table the run produced to the N3
/// flatness rule on the way out. A tripwire over this suite's tables,
/// not the guard: the mint refuses a nested constituent before a
/// table is published, and the rows that carry the rule are
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

/// A box: profile on z = 0, extruded 1.
fn cube(doc: ProfileDoc, x0: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(x0, 0.0), (x0 + 1.0, 0.0), (x0 + 1.0, 1.0), (x0, 1.0)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    )
}

/// Three disjoint boxes, and a union of them in the given member
/// order (one union node for every order, [`crate::fixture::union_over`]). Returns the document, the three box nodes in construction
/// order, and the union node.
fn three_boxes(order: [usize; 3]) -> (ProfileDoc, [RecipeNodeId; 3], RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("docm3_union", Tol::witness());
    let (doc, a) = cube(doc, 0.0);
    let (doc, b) = cube(doc, 2.0);
    let (doc, c) = cube(doc, 4.0);
    let boxes = [a, b, c];
    let (doc, u) = crate::fixture::union_over(doc, &order.map(|i| boxes[i]), Vec::new());
    (doc, boxes, u)
}

/// The member a union-minted name came from, or `None` for a name that
/// is not a carried (`From`) row. Read off the member's READ, which is
/// the only thing that answers: the inner name's minting node is the
/// PROTOTYPE's where a member is a placement of one.
fn member_of(doc: &ProfileDoc, name: &StableName) -> Option<RecipeNodeId> {
    match name.path.first() {
        Some(RoleSeg::From { read, .. }) => doc.operation_of(*read),
        _ => None,
    }
}

// ---------------------------------------------------------------------
// A2 — names are position-free.
// ---------------------------------------------------------------------

/// **A member's names under a union do not depend on its position in
/// the list**, which is the property the whole node exists for.
///
/// Three distinct boxes, unioned in two different orders; for each
/// member `k`, the set of face names the union's table gives that
/// member's faces is IDENTICAL between the two evaluations. Under the
/// pairwise chain this replaces, every one of those names would carry
/// a `FromA`/`FromB` descent as deep as the member's position.
#[test]
fn a_members_face_names_are_the_same_first_or_last() {
    let (doc, boxes, u) = three_boxes([0, 1, 2]);
    let (rev_doc, rev_boxes, rev_u) = three_boxes([2, 1, 0]);
    assert_eq!(boxes, rev_boxes, "the two documents mint the same box ids");
    let (ev, rev_ev) = (run(&doc), run(&rev_doc));

    for (k, member) in boxes.iter().enumerate() {
        let faces_under = |doc: &ProfileDoc, ev: &Evaluation<f64>, u: RecipeNodeId| {
            let mut v: Vec<StableName> = all_faces(ev, u)
                .into_iter()
                .filter(|n| member_of(doc, n) == Some(*member))
                .collect();
            v.sort();
            v
        };
        let forward = faces_under(&doc, &ev, u);
        assert_eq!(forward.len(), 6, "member {k} contributes its six box faces");
        assert_eq!(
            forward,
            faces_under(&rev_doc, &rev_ev, rev_u),
            "member {k}'s face names moved with its list position"
        );
    }
}

/// The wrapper is ONE segment deep for every member, at every
/// position — the fold's `FromA`/`FromB` chain does not survive into
/// the node's names.
#[test]
fn every_union_name_wraps_its_member_exactly_once() {
    let (doc, _, u) = three_boxes([0, 1, 2]);
    let ev = run(&doc);
    for name in all_faces(&ev, u) {
        assert_eq!(name.node, u, "a union's names are minted by the union");
        match name.path.as_slice() {
            [RoleSeg::From { of, .. }] => assert!(
                !matches!(of.path.first(), Some(RoleSeg::From { .. })),
                "the wrapped name is the member's own, not a fold row: {of:?}"
            ),
            other => panic!("a disjoint union's faces are single `From` rows: {other:?}"),
        }
    }
}

// ---------------------------------------------------------------------
// A3 — the fold equals the chain.
// ---------------------------------------------------------------------

/// **The fold is the chain's geometry.** One document carrying both a
/// three-member `Node::Union` and the pairwise `Boolean(Union)` chain
/// over the same three bodies: the two values agree face for face and
/// edge for edge in COUNT, and description for description on every
/// surface, curve and point. Only the NAMES differ, which is the whole
/// content of the change.
#[test]
fn the_fold_and_the_pairwise_chain_are_the_same_body() {
    let (doc, boxes, u) = three_boxes([0, 1, 2]);
    let (doc, ab) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![boxes[0].into(), boxes[1].into()]),
            declare: Vec::new(),
        },
    );
    let (doc, abc) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![ab.into(), boxes[2].into()]),
            declare: Vec::new(),
        },
    );
    let ev = run(&doc);
    let (folded, chained) = (body_of(&ev, u), body_of(&ev, abc));
    assert_eq!(
        folded.faces().count(),
        chained.faces().count(),
        "the fold and the chain differ in face count"
    );
    assert_eq!(folded.edges().count(), chained.edges().count());
    assert_eq!(folded.vertices().count(), chained.vertices().count());
    // Surfaces, curves AND points: surfaces alone leave the fold free
    // to differ in every edge and vertex the seams are made of, which
    // is where a union's geometry actually lives. Curves and points
    // adopted from `docm/3-review-r2`'s
    // `r2_the_die_fold_and_chain_agree_on_curves_and_points`.
    let sorted = |mut v: Vec<String>| {
        v.sort();
        v
    };
    let surfaces =
        |b: &topo::Body<f64>| sorted(b.surfaces().map(|(_, s)| format!("{s:?}")).collect());
    let curves = |b: &topo::Body<f64>| sorted(b.curves().map(|(_, c)| format!("{c:?}")).collect());
    let points = |b: &topo::Body<f64>| sorted(b.points().map(|(_, p)| format!("{p:?}")).collect());
    assert_eq!(
        surfaces(folded),
        surfaces(chained),
        "the fold's surfaces are not the chain's, description for description"
    );
    assert_eq!(
        curves(folded),
        curves(chained),
        "the fold's curves are not the chain's, description for description"
    );
    assert_eq!(
        points(folded),
        points(chained),
        "the fold's points are not the chain's, description for description"
    );
    // And the names are what moved: the chain's are two descents deep
    // for the first member, the fold's are one wrapper for every member.
    let chain_names = all_faces(&ev, abc);
    let (ab_read, first_read) = (fixture::out(&doc, ab), fixture::out(&doc, boxes[0]));
    assert!(
        chain_names.iter().any(
            |n| matches!(n.path.first(), Some(RoleSeg::From { read, of: inner })
                if *read == ab_read
                    && matches!(inner.path.first(),
                        Some(RoleSeg::From { read: r, .. }) if *r == first_read))
        ),
        "the pairwise chain nests its first operand twice"
    );
}

// ---------------------------------------------------------------------
// A4 — a repeated read GLUES (DM4, ratified): no door refuses it.
// ---------------------------------------------------------------------

/// True iff `id` evaluated to the typed empty body.
fn is_empty_body(ev: &Evaluation<f64>, id: RecipeNodeId) -> bool {
    matches!(
        ev.value(id).map(|v| &v.payload),
        Some(editor_core::ValuePayload::Boolean(
            editor_core::BooleanValue::Empty
        ))
    )
}

/// **A node that takes one input twice is accepted at the INSERT door
/// and glues**: `Union[X, X]` and `Intersect[X, X]` are `X` (its six
/// faces), and `Subtract { X, X }` is the typed empty body.
#[test]
fn a_repeated_input_glues_at_the_insert_door() {
    let (doc, boxes, _) = three_boxes([0, 1, 2]);
    let x = boxes[0];
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![x.into(), x.into()]),
            declare: Vec::new(),
        },
    );
    let (doc, meet) = insert(
        doc,
        Node::Intersect {
            members: editor_core::Bodies::Spelled(vec![x.into(), x.into()]),
            declare: Vec::new(),
        },
    );
    let (doc, cut) = insert(
        doc,
        Node::Subtract {
            from: x.into(),
            tool: x.into(),
            declare: Vec::new(),
        },
    );
    let ev = run(&doc);
    for (what, id) in [("Union[X, X]", union), ("Intersect[X, X]", meet)] {
        assert_eq!(
            body_of(&ev, id).faces().count(),
            body_of(&ev, x).faces().count(),
            "{what} is X: {:?}",
            ev.nodes.get(&id)
        );
    }
    assert!(
        is_empty_body(&ev, cut),
        "Subtract {{ X, X }} is the typed empty body: {:?}",
        ev.nodes.get(&cut)
    );
}

/// The same rule at the OTHER door: a `SetMembers` that leaves one
/// node in the list twice is accepted, and the repeat glues — the
/// union is the two distinct boxes' twelve faces.
#[test]
fn set_members_accepts_a_repeated_member_and_glues_it() {
    let (doc, boxes, u) = three_boxes([0, 1, 2]);
    let doc = doc
        .apply(
            &DocEdit::SetMembers {
                node: u,
                members: editor_core::Bodies::Spelled(vec![
                    boxes[0].into(),
                    boxes[1].into(),
                    boxes[0].into(),
                ]),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .expect("a repeated member glues")
        .doc;
    let ev = run(&doc);
    assert_eq!(
        body_of(&ev, u).faces().count(),
        12,
        "two distinct boxes, one of them read twice, fuse to twelve faces"
    );
}

/// The THIRD door: a SNAPSHOT carrying a repeated member, or a
/// one-member union, LOADS — the load validator asks the same function
/// the edit doors do, and neither shape is a fault any more.
#[test]
fn a_snapshot_carrying_a_repeated_or_single_member_loads() {
    let tol = Tol::witness();
    let (doc, boxes, u) = three_boxes([0, 1, 2]);
    let text = editor_core::persist::save(&doc, &[], tol).expect("the document saves");
    // The spelled member list is rewritten in place, whatever
    // whitespace the writer used around it, so the fixture is about the
    // LIST and not about the formatting.
    let corrupt = |members: String| {
        let (head, rest) = text
            .split_once("\"Spelled\":")
            .expect("the union's spelled list is on the wire");
        let (gap, list) = rest.split_once('[').expect("the list opens");
        let (_, tail) = list.split_once(']').expect("the list closes");
        let tampered = format!("{head}\"Spelled\":{gap}[{members}]{tail}");
        editor_core::persist::load(&tampered, tol)
    };
    let read = |node| doc.output(node, 0).expect("a box defines its body").0;
    let members_of = |loaded: &editor_core::ProfileDoc| -> Vec<RecipeNodeId> {
        let Some(Node::Union { members, .. }) = loaded.node(u) else {
            panic!("the union loaded as something else")
        };
        members
            .reads()
            .map(|&m| {
                loaded
                    .operation_of(m)
                    .expect("a member reads a live output")
            })
            .collect()
    };
    // A repeated member in the union's list. Every box stays read, so
    // the snapshot's root set still covers the document.
    let loaded = corrupt(format!(
        "\"{}\",\"{}\",\"{}\",\"{}\"",
        read(boxes[0]),
        read(boxes[1]),
        read(boxes[2]),
        read(boxes[0])
    ))
    .expect("a repeated member loads");
    assert_eq!(
        members_of(&loaded.doc),
        vec![boxes[0], boxes[1], boxes[2], boxes[0]]
    );
    // And a list of one. Dropping two boxes from the list in the text
    // would leave them roots the snapshot's root set does not name, so
    // the one-member union is authored at the edit door, which keeps the
    // root set, and the load door reads what the writer saved.
    let one = doc
        .apply(
            &DocEdit::SetMembers {
                node: u,
                members: editor_core::Bodies::Spelled(vec![boxes[0].into()]),
            },
            tol,
            &editor_core::RefusingReach,
        )
        .expect("a one-member union is accepted")
        .doc;
    let text = editor_core::persist::save(&one, &[], tol).expect("the document saves");
    let loaded = editor_core::persist::load(&text, tol).expect("a one-member union loads");
    assert_eq!(members_of(&loaded.doc), vec![boxes[0]]);
}

// ---------------------------------------------------------------------
// A5 — SetMembers' refusals, and its round trip.
// ---------------------------------------------------------------------

/// A `SetMembers` at a node that carries no list.
#[test]
fn set_members_refuses_a_node_with_no_list_input() {
    let (doc, boxes, _) = three_boxes([0, 1, 2]);
    let (doc, pair) = insert(
        doc,
        Node::Subtract {
            from: boxes[0].into(),
            tool: boxes[1].into(),
            declare: Vec::new(),
        },
    );
    let err = doc
        .apply(
            &DocEdit::SetMembers {
                node: pair,
                members: editor_core::Bodies::Spelled(vec![boxes[0].into(), boxes[2].into()]),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .expect_err("a subtract carries no list");
    assert!(
        matches!(&err, EditError::SetMembersOnNonList { node } if node.id() == pair),
        "{err:?}"
    );
}

/// A member that is not a live node.
#[test]
fn set_members_refuses_a_member_that_is_not_live() {
    let (doc, boxes, u) = three_boxes([0, 1, 2]);
    let ghost = RecipeNodeId::new(0, 9999);
    let err = doc
        .apply(
            &DocEdit::SetMembers {
                node: u,
                members: editor_core::Bodies::Spelled(vec![boxes[0].into(), ghost.into()]),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .expect_err("a dangling member must refuse");
    assert!(
        matches!(&err, EditError::UnresolvedInput { input } if input.id() == ghost),
        "{err:?}"
    );
}

/// A member downstream of the node itself — the cycle the insert door
/// can never see, because an insert names only pre-existing nodes.
#[test]
fn set_members_refuses_a_cycle() {
    let (doc, boxes, u) = three_boxes([0, 1, 2]);
    let (doc, downstream) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![u.into(), boxes[0].into()]),
            declare: Vec::new(),
        },
    );
    let err = doc
        .apply(
            &DocEdit::SetMembers {
                node: u,
                members: editor_core::Bodies::Spelled(vec![boxes[1].into(), downstream.into()]),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .expect_err("a member below the node closes a loop");
    assert!(matches!(err, EditError::WouldCycle { .. }), "{err:?}");
}

/// A list left with ONE entry is accepted and builds that member's
/// body; a list left EMPTY is accepted and builds the typed empty body
/// (DM4, ratified: neither is a refusal any more).
#[test]
fn set_members_accepts_one_member_and_none() {
    let (doc, boxes, u) = three_boxes([0, 1, 2]);
    let set = |members: Vec<editor_core::Operand>| {
        doc.apply(
            &DocEdit::SetMembers {
                node: u,
                members: editor_core::Bodies::Spelled(members),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .expect("a short list is a legal edit")
        .doc
    };
    let one = set(vec![boxes[0].into()]);
    let ev = run(&one);
    assert_eq!(
        body_of(&ev, u).faces().count(),
        6,
        "a union of one is its member's body: {:?}",
        ev.nodes.get(&u)
    );
    let none = set(Vec::new());
    let ev = run(&none);
    assert!(
        is_empty_body(&ev, u),
        "a union of none is the typed empty body: {:?}",
        ev.nodes.get(&u)
    );
}

/// **The wire round trip**: a document carrying a `Union` and a
/// `SetMembers` in its edit log saves, loads and replays to the same
/// document, bit for bit.
#[test]
fn a_union_and_a_set_members_replay_bit_identically() {
    let tol = Tol::witness();
    let (doc, boxes, u) = three_boxes([0, 1, 2]);
    let empty = ProfileDoc::empty_derived("docm3_union", tol);
    let mut edits: Vec<DocEdit<editor_core::ProfileProgram>> = doc
        .ids()
        .iter()
        .map(|id| DocEdit::InsertNode {
            node: Box::new(crate::fixture::as_authored(
                &doc,
                doc.node(*id).expect("an ordered node"),
            )),
            fresh: Vec::new(),
        })
        .collect();
    edits.push(DocEdit::SetMembers {
        node: u,
        members: editor_core::Bodies::Spelled(vec![boxes[2].into(), boxes[0].into()]),
    });
    edits.push(DocEdit::DeleteNode { id: boxes[1] });
    let mut replayed = empty.clone();
    for edit in &edits {
        replayed = replayed
            .apply(edit, tol, &editor_core::RefusingReach)
            .expect("the log replays")
            .doc;
    }
    let text =
        editor_core::persist::save(&empty, &edits.to_vec(), tol).expect("the document saves");
    let loaded = editor_core::persist::load(&text, tol).expect("the document loads");
    assert!(
        loaded.doc.bit_eq(&replayed),
        "the loaded replay is not the document the log builds"
    );
    assert_eq!(loaded.edits.len(), edits.len());
    // And the surviving union is the two-member one the log states.
    let Some(Node::Union { members, .. }) = loaded.doc.node(u) else {
        panic!("the union survived as something else")
    };
    let members: Vec<RecipeNodeId> = members
        .reads()
        .map(|&m| {
            loaded
                .doc
                .operation_of(m)
                .expect("a member reads a live output")
        })
        .collect();
    assert_eq!(members, vec![boxes[2], boxes[0]]);
}

/// Dropping a member leaves every OTHER member's names untouched —
/// the property `SetMembers` exists to deliver, at the smallest size
/// that can show it.
#[test]
fn dropping_a_member_leaves_the_others_names_alone() {
    let (doc, boxes, u) = three_boxes([0, 1, 2]);
    let before = run(&doc);
    let kept: Vec<StableName> = all_faces(&before, u)
        .into_iter()
        .filter(|n| member_of(&doc, n) != Some(boxes[0]))
        .collect();
    let doc = doc
        .apply(
            &DocEdit::SetMembers {
                node: u,
                members: editor_core::Bodies::Spelled(vec![boxes[1].into(), boxes[2].into()]),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .expect("the drop applies")
        .doc;
    let doc = doc
        .apply(
            &DocEdit::DeleteNode { id: boxes[0] },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .expect("the orphan deletes")
        .doc;
    let after = run(&doc);
    assert_eq!(
        all_faces(&after, u),
        kept,
        "removing one member renamed the others"
    );
}

// ---------------------------------------------------------------------
// Members that share a minting node — the die's shape.
// ---------------------------------------------------------------------

/// **Two placements of ONE prototype are two members, and their names
/// are distinct.** This is the row the member EDGE exists for.
///
/// A pass-through op mints no name of its own: a transform's table IS
/// its input's, verbatim (`eval::wire::wire_transform` — "the input's
/// table rows hold verbatim: same names, same keys", N1's rule that a
/// pass-through adds no segment and the `node` stays the original
/// minter). So two transforms of one body carry two IDENTICAL tables,
/// and a wrapper keyed on the inner name alone would map their
/// corresponding faces onto one name and refuse.
///
/// The segment keys on the member's node id instead, so the two
/// members are told apart by the DAG edge that makes them two. The
/// first assertion is the mechanism — the tables really are equal —
/// and the second is what the union does with it. The die's 21 pips
/// are exactly this shape, at scale.
#[test]
fn two_placements_of_one_prototype_are_two_members() {
    let doc = ProfileDoc::empty_derived("docm3_union", Tol::witness());
    let (doc, base) = cube(doc, 0.0);
    let place = |doc, dx: f64| {
        insert(
            doc,
            Node::transform(
                base,
                editor_core::Step::Rigid {
                    translation: [len(dx), len(0.0), len(0.0)],
                    axis: [fixture::scl(0.0), fixture::scl(0.0), fixture::scl(1.0)],
                    angle: fixture::ang(0.0),
                },
            ),
        )
    };
    let (doc, left) = place(doc, 0.0);
    let (doc, right) = place(doc, 2.0);
    let (doc, u) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![left.into(), right.into()]),
            declare: Vec::new(),
        },
    );
    let ev = run(&doc);

    // The mechanism: the two members' tables are the same table.
    assert_eq!(
        all_faces(&ev, left),
        all_faces(&ev, right),
        "a transform passes its input's names through, so two placements of one \
         body carry identical name tables"
    );
    // The consequence: twelve distinct face names under the union, six
    // per member, told apart by the member edge and by nothing else.
    let faces = all_faces(&ev, u);
    assert_eq!(faces.len(), 12, "two boxes fuse to twelve faces: {faces:?}");
    for member in [left, right] {
        assert_eq!(
            faces
                .iter()
                .filter(|n| member_of(&doc, n) == Some(member))
                .count(),
            6,
            "member {member:?} contributes its six faces"
        );
    }
    // And the inner names alone would NOT have told them apart: strip
    // the member edge and the twelve collapse to six.
    let inner: std::collections::BTreeSet<StableName> = faces
        .iter()
        .filter_map(|n| match n.path.first() {
            Some(RoleSeg::From { of, .. }) => Some((**of).clone()),
            _ => None,
        })
        .collect();
    assert_eq!(
        inner.len(),
        6,
        "the inner names are the prototype's, six of them, shared by both members"
    );
}

// ---------------------------------------------------------------------
// A1 — remove one pip and both fillets still resolve.
// ---------------------------------------------------------------------

/// **The row the naming design exists for**, at the size a person
/// actually sees: the tour's die — 21 pips fused by one
/// `Node::Union`, cut from a cube, then twelve box edges and 42 rim
/// arcs blended over frozen selections.
///
/// For the FIRST, a MIDDLE and the LAST pip: drop it with
/// `SetMembers`, delete the orphaned transform, and re-evaluate with
/// the previous evaluation as `prior`. Both fillets must still
/// evaluate `Ok`, with two fewer rim arcs selected (a pip contributes
/// two rim edges) and no `BlendSelectionResolve` anywhere.
///
/// Under the pairwise chain this replaces the row goes red for every
/// pip but the last: a chain records join DEPTH in each name, so
/// removing one link renames every pip that joined before it and the
/// rim fillet's frozen selection fails typed for each of them.
#[test]
fn removing_any_pip_leaves_both_die_fillets_resolving() {
    let tol = Tol::witness();
    let die = crate::corpus::die_composed_tour::document();
    let doc = die.doc;
    let union = doc
        .ids()
        .iter()
        .copied()
        .find(|id| matches!(doc.node(*id), Some(Node::Union { .. })))
        .expect("the die fuses its pips with one union");
    let Some(Node::Union { members, .. }) = doc.node(union) else {
        panic!("the union is a union")
    };
    let member_reads: Vec<editor_core::VarId> = members.reads().copied().collect();
    let members: Vec<RecipeNodeId> = member_reads
        .iter()
        .map(|&m| doc.operation_of(m).expect("a member reads a live output"))
        .collect();
    assert_eq!(members.len(), 21, "the die has 21 pips");
    let blends: Vec<RecipeNodeId> = doc
        .ids()
        .iter()
        .copied()
        .filter(|id| matches!(doc.node(*id), Some(Node::Fillet { .. })))
        .collect();
    assert_eq!(blends.len(), 2, "the box-edge blend and the rim blend");
    let before = run(&doc);
    // The radius as written: deleting the blend retires the anonymous
    // variable its radius reads, so the re-authored blend writes it
    // again.
    let (rim_target, rim_radius, rims) = match doc.node(blends[1]).map(|n| n.written(&doc)) {
        Some(Node::Fillet {
            target,
            radius,
            selection,
        }) => (target, radius, selection),
        other => panic!("the die's last node is the rim blend, got {other:?}"),
    };
    assert_eq!(rims.len(), 42, "the die selects two rim arcs per pip");

    for (label, k) in [("first", 0usize), ("middle", 10), ("last", 20)] {
        let kept: Vec<RecipeNodeId> = members
            .iter()
            .copied()
            .filter(|m| *m != members[k])
            .collect();
        let edited = doc
            .apply(
                &DocEdit::SetMembers {
                    node: union,
                    members: editor_core::Bodies::Spelled(
                        kept.into_iter().map(Into::into).collect(),
                    ),
                },
                tol,
                &editor_core::RefusingReach,
            )
            .unwrap_or_else(|e| panic!("dropping the {label} pip: {e}"))
            .doc;
        let edited = edited
            .apply(
                &DocEdit::DeleteNode { id: members[k] },
                tol,
                &editor_core::RefusingReach,
            )
            .unwrap_or_else(|e| panic!("deleting the orphaned {label} transform: {e}"))
            .doc;
        let after = evaluate::<f64>(
            &edited,
            Some(&before),
            &CancelToken::new(),
            &EvalOptions::default(),
            tol,
        );
        // The removed pip's OWN rim arcs are gone with it, and their
        // frozen names say so — every one of them carries the dead
        // member's read and nothing else does. That is the whole claim: the
        // damage is exactly the removed member's, told apart by the
        // member edge in the name and by nothing positional.
        let doomed: Vec<StableName> = rims
            .iter()
            .filter(|n| editor_core::derivation_reads(n).contains(&member_reads[k]))
            .cloned()
            .collect();
        assert_eq!(
            doomed.len(),
            2,
            "the {label} pip contributes exactly its own two rim arcs"
        );
        // Deleting a pip is that edit plus dropping the names it took
        // with it — one committed action a user can spell because the
        // names say which member they came from. The blend is
        // re-authored because a selection is frozen payload and there
        // is no edit that prunes one.
        let kept_rims: Vec<StableName> = rims
            .iter()
            .filter(|n| !doomed.contains(n))
            .cloned()
            .collect();
        assert_eq!(kept_rims.len(), rims.len() - 2);
        let edited = edited
            .apply(
                &DocEdit::DeleteNode { id: blends[1] },
                tol,
                &editor_core::RefusingReach,
            )
            .expect("the rim blend is a sink")
            .doc;
        let (edited, rim) = insert(
            edited,
            Node::fillet(rim_target.clone(), rim_radius.clone(), kept_rims.clone()),
        );
        let after = evaluate::<f64>(
            &edited,
            Some(&after),
            &CancelToken::new(),
            &EvalOptions::default(),
            tol,
        );
        for (what, node) in [("box-edge", blends[0]), ("rim", rim)] {
            let failure = after.nodes.get(&node).and_then(|r| match r {
                editor_core::NodeResult::Failed(e) => Some(format!("{:?}", e.kind)),
                _ => None,
            });
            assert!(
                failure.is_none(),
                "dropping the {label} pip broke the {what} fillet: {failure:?}"
            );
            assert!(
                after.value(node).is_some(),
                "dropping the {label} pip left the {what} fillet without a value"
            );
        }
        assert_eq!(
            selection_len(&edited, rim),
            rims.len() - 2,
            "the rim blend selects every arc but the dead pip's two"
        );
        // The selections are FROZEN, so they still name every edge they
        // named — including the two rim arcs of the pip that is gone,
        // which the blend now resolves against a body that no longer
        // has them. That is what `BlendSelectionResolve` would say, and
        // the assertions above are that it does not.
    }
}

/// **A3 at the die's size**: the union of the 21 pips is the pairwise
/// chain it replaced, body for body.
///
/// Both are built in ONE document over the SAME 21 transforms and
/// evaluated at one scalar, so nothing about the comparison depends on
/// two runs agreeing. Face, edge and vertex COUNTS agree, and so does
/// every description the body carries — surfaces, curves AND points,
/// as multisets of their debug forms. Curves and points come from
/// `docm/3-review-r2`'s `r2_the_die_fold_and_chain_agree_on_curves_and_points`:
/// surfaces alone left the fold free to differ in every edge and vertex
/// the seams are actually made of, which is where a union's geometry
/// lives. Only the NAMES differ, which is the whole content of the
/// change.
#[test]
fn the_dies_union_is_the_chain_it_replaced() {
    let die = crate::corpus::die_composed_tour::document();
    let doc = die.doc;
    let union = doc
        .ids()
        .iter()
        .copied()
        .find(|id| matches!(doc.node(*id), Some(Node::Union { .. })))
        .expect("the die fuses its pips with one union");
    let Some(Node::Union { members, .. }) = doc.node(union) else {
        panic!("the union is a union")
    };
    let members: Vec<RecipeNodeId> = members
        .reads()
        .map(|&m| doc.operation_of(m).expect("a member reads a live output"))
        .collect();
    // The chain this replaced, re-authored over the same members.
    let (doc, chain) = members.iter().skip(1).fold(
        (doc, members[0]),
        |(doc, acc): (ProfileDoc, RecipeNodeId), pip| {
            insert(
                doc,
                Node::Union {
                    members: editor_core::Bodies::Spelled(vec![acc.into(), (*pip).into()]),
                    declare: Vec::new(),
                },
            )
        },
    );
    let ev = run(&doc);
    let (folded, chained) = (body_of(&ev, union), body_of(&ev, chain));
    assert_eq!(
        folded.faces().count(),
        chained.faces().count(),
        "the fold and the chain differ in face count"
    );
    assert_eq!(folded.edges().count(), chained.edges().count());
    assert_eq!(folded.vertices().count(), chained.vertices().count());
    // Surfaces, curves AND points: surfaces alone leave the fold free
    // to differ in every edge and vertex the seams are made of, which
    // is where a union's geometry actually lives. Curves and points
    // adopted from `docm/3-review-r2`'s
    // `r2_the_die_fold_and_chain_agree_on_curves_and_points`.
    let sorted = |mut v: Vec<String>| {
        v.sort();
        v
    };
    let surfaces =
        |b: &topo::Body<f64>| sorted(b.surfaces().map(|(_, s)| format!("{s:?}")).collect());
    let curves = |b: &topo::Body<f64>| sorted(b.curves().map(|(_, c)| format!("{c:?}")).collect());
    let points = |b: &topo::Body<f64>| sorted(b.points().map(|(_, p)| format!("{p:?}")).collect());
    assert_eq!(
        surfaces(folded),
        surfaces(chained),
        "the fold's surfaces are not the chain's, description for description"
    );
    assert_eq!(
        curves(folded),
        curves(chained),
        "the fold's curves are not the chain's, description for description"
    );
    assert_eq!(
        points(folded),
        points(chained),
        "the fold's points are not the chain's, description for description"
    );
    // And the names are what moved. The chain's LAST member is one
    // descent deep and its FIRST is twenty; every member of the fold is
    // one wrapper, whatever its position.
    let depth = |n: &StableName| {
        let mut d = 0;
        let mut cur = n.path.first();
        while let Some(RoleSeg::From { read, of: inner }) = cur {
            d += 1;
            // A read of a fold step (a union node) is one more descent;
            // the first read that is not one is the member's own.
            if !matches!(
                doc.operation_of(*read).and_then(|n| doc.node(n)),
                Some(Node::Union { .. })
            ) {
                break;
            }
            cur = inner.path.first();
        }
        d
    };
    assert_eq!(
        all_faces(&ev, union).iter().map(depth).max(),
        Some(1),
        "a union's names carry one member wrapper and no operand descent"
    );
    assert_eq!(
        all_faces(&ev, chain).iter().map(depth).max(),
        Some(20),
        "the chain's first member is twenty descents deep"
    );
}

/// How many names a blend node's selection carries.
fn selection_len(doc: &editor_core::ProfileDoc, blend: RecipeNodeId) -> usize {
    match doc.node(blend) {
        Some(Node::Fillet { selection, .. }) => selection.len(),
        other => panic!("expected a fillet, got {other:?}"),
    }
}

// ---------------------------------------------------------------------
// A6 — the seat tracks the door.
// ---------------------------------------------------------------------

/// A union DENOTES A BODY at a single-body operand seat: one body out,
/// exactly as the pair union it generalizes, so the seat's answer and
/// the evaluator's door agree.
#[test]
fn a_union_is_one_body_at_an_operand_seat() {
    let (doc, _, u) = three_boxes([0, 1, 2]);
    let (doc, downstream) = insert(
        doc,
        Node::transform(
            u,
            editor_core::Step::Rigid {
                translation: [len(0.0), len(0.0), len(0.0)],
                axis: [fixture::scl(0.0), fixture::scl(0.0), fixture::scl(1.0)],
                angle: fixture::ang(0.0),
            },
        ),
    );
    let ev = run(&doc);
    assert!(
        ev.value(downstream).is_some(),
        "the union fed a body seat: {:?}",
        ev.nodes.get(&downstream)
    );
    assert_eq!(
        all_faces(&ev, u)
            .iter()
            .filter(|n| n.kind == EntityKind::Face)
            .count(),
        18,
        "three disjoint boxes fuse to eighteen faces"
    );
}

// ---------------------------------------------------------------------
// The refusal's name space. Adopted from `docm/3-review-r1`'s
// `r1_refusal_from_a_later_fold_step_names_union_space_names`; R2's
// `r2_a_refusal_at_a_later_fold_step_names_a_fold_row` measured the
// same defect from the other side.
// ---------------------------------------------------------------------

/// A box on a frame at height `z0`, footprint `[x0,x1]×[y0,y1]`.
fn boxed(
    doc: ProfileDoc,
    x: (f64, f64),
    y: (f64, f64),
    z0: f64,
    h: f64,
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
            profile: p.into(),
            distance: len(h),
            side: ExtrudeSide::Along,
        },
    )
}

fn failure(ev: &Evaluation<f64>, id: RecipeNodeId) -> Option<String> {
    match ev.nodes.get(&id) {
        Some(editor_core::NodeResult::Failed(e)) => Some(format!("{:?}", e.kind)),
        _ => None,
    }
}

/// **A refusal raised at a LATER fold step names entities in the
/// union's published space**, not in the fold's internal one.
///
/// `refusal_menu` resolves the raise site's face keys through the two
/// operand tables. From step 2 on the `a` side is the ACCUMULATED
/// table — the pair emitter's, `FromA`/`FromB`-headed — so an
/// unfiltered refusal carries a name minted under the union's id that
/// NO published table holds: `resolve` cannot look it up and a selector
/// written against it matches nothing. Every name a union's refusal
/// carries goes through the same collapse `name_union` applies, so what
/// comes out is member-keyed.
///
/// The third member shares member 0's `x = 1` plane over a patch member
/// 1 does not cover, so step 2 raises `UndeclaredCoincidence` — and the
/// PAIR spelling of that same contact refuses identically, which is
/// what says the fold added no refusal, only a name space.
///
/// The recourse a caller whose members touch has is this node's own
/// `declare` list, whose pairs are exactly what this refusal hands
/// back: each side a `SitedRef` naming the MEMBER it was read at and
/// the entity's name in that member's own table, which is a
/// declaration the caller can write verbatim (`docm7_union_declare`).
#[test]
fn a_refusal_at_a_later_fold_step_names_member_space_entities() {
    let doc = ProfileDoc::empty_derived("docm3_union_menu", Tol::witness());
    let (doc, a) = boxed(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = boxed(doc, (0.5, 1.5), (0.2, 0.8), 0.2, 0.5);
    let (doc, d) = boxed(doc, (1.0, 2.0), (0.0, 0.15), 0.0, 1.0);
    let (doc, u) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![a.into(), b.into(), d.into()]),
            declare: Vec::new(),
        },
    );
    let (doc, pair) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![a.into(), d.into()]),
            declare: Vec::new(),
        },
    );
    let ev = run(&doc);
    let pf = failure(&ev, pair).expect("the pair spelling refuses the undeclared contact");
    let class = |id| ev.node_error(id).map(|e| e.kind.class());
    assert_eq!(
        class(pair),
        Some(editor_core::NodeErrorClass::UndeclaredCoincidence),
        "{pf}"
    );
    let uf = failure(&ev, u).expect("the fold refuses the undeclared contact at step 2");
    assert_eq!(
        class(u),
        class(pair),
        "the fold's refusal is the pair's, not a new class: {uf}"
    );
    // The whole point: no fold row survives into the refusal.
    assert!(
        !uf.contains("FromA(") && !uf.contains("FromB("),
        "the fold's refusal names an uncollapsed fold row: {uf}"
    );
    // And what it hands back is a declarable pair: each side sited at
    // the member it was read at, named in that member's own table.
    let Some(editor_core::NodeResult::Failed(e)) = ev.nodes.get(&u) else {
        panic!("the fold refuses")
    };
    let editor_core::NodeErrorKind::UndeclaredCoincidence { finding, .. } = &e.kind else {
        panic!("the fold's refusal is the undeclared contact: {uf}")
    };
    let sites = [finding.pair.0.at, finding.pair.1.at];
    assert!(
        sites.contains(&fixture::out(&doc, a)) && sites.contains(&fixture::out(&doc, d)),
        "the refusal sites its two faces at the members that touch: {sites:?}"
    );
    for r in [&finding.pair.0, &finding.pair.1] {
        assert_eq!(
            Some(r.name.node),
            doc.operation_of(r.at),
            "and names each in that member's own table, not in the union's"
        );
    }
}

// ---------------------------------------------------------------------
// The list floor reaches `Loft`, at both doors. Measured by both
// review branches (`r2_a_one_section_loft_at_the_insert_door`); the
// widening is disclosed in the PR body's deviation list.
// ---------------------------------------------------------------------

/// Four stacked sections and a three-section loft over the first three.
fn loft_doc() -> (ProfileDoc, RecipeNodeId, Vec<RecipeNodeId>) {
    let mut doc = ProfileDoc::empty_derived("docm3_union_loft", Tol::witness());
    let mut profiles = Vec::new();
    for (z, s) in [(0.0, 1.0), (1.0, 1.6), (2.0, 1.0), (3.0, 1.2)] {
        let (d, id) = on_frame(
            doc,
            [0.0, 0.0, z],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![
                (0.0, 0.0),
                (2.0 * s, 0.0),
                (2.0 * s, 1.0 * s),
                (0.0, 1.0 * s),
            ]],
        );
        doc = d;
        profiles.push(id);
    }
    let (doc, loft) = insert(
        doc,
        Node::Loft {
            profiles: profiles[..3].iter().copied().map(Into::into).collect(),
            v_degree: editor_core::Formula::count(2),
        },
    );
    (doc, loft, profiles)
}

/// **A ONE-SECTION loft is accepted at the insert door** (DM4: the
/// list floor is retired), and the loft itself refuses it when it
/// evaluates — a single section has nothing to loft between.
#[test]
fn a_one_section_loft_is_accepted_at_the_insert_door() {
    let (doc, _, profiles) = loft_doc();
    let (doc, loft) = insert(
        doc,
        Node::Loft {
            profiles: vec![profiles[0].into()],
            v_degree: editor_core::Formula::count(1),
        },
    );
    let ev = run(&doc);
    assert!(
        ev.value(loft).is_none(),
        "a one-section loft has no second section to loft to: {:?}",
        ev.nodes.get(&loft)
    );
}

/// The LOAD-door twin: the same one-section loft in a SNAPSHOT loads,
/// since the load validator asks `input_fault`, which carries no
/// floor. The snapshot is the saved document itself, so its root set
/// is the one the document holds.
#[test]
fn a_snapshot_carrying_a_one_section_loft_loads() {
    let tol = Tol::witness();
    let (doc, _, profiles) = loft_doc();
    let (doc, loft) = insert(
        doc,
        Node::Loft {
            profiles: vec![profiles[0].into()],
            v_degree: editor_core::Formula::count(1),
        },
    );
    let text = editor_core::persist::save(&doc, &[], tol).expect("the document saves");
    let loaded = editor_core::persist::load(&text, tol).expect("a one-section loft loads");
    let Some(Node::Loft { profiles: read, .. }) = loaded.doc.node(loft) else {
        panic!("the loft loaded as something else")
    };
    assert_eq!(read.len(), 1, "the loft keeps its one section");
}

// ---------------------------------------------------------------------
// `SetMembers`' remaining doors and the root-set contract.
// ---------------------------------------------------------------------

/// `SetMembers` naming a node the document does not hold refuses
/// `UnknownNode` — the first question the edit asks, before it has a
/// node to rewrite or a list to check.
#[test]
fn set_members_refuses_an_unknown_node() {
    let (doc, boxes, _) = three_boxes([0, 1, 2]);
    let err = doc
        .apply(
            &DocEdit::SetMembers {
                node: RecipeNodeId::new(0, 9999),
                members: editor_core::Bodies::Spelled(vec![boxes[0].into(), boxes[1].into()]),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .expect_err("a node the document does not hold cannot be re-membered");
    assert!(
        matches!(&err, EditError::UnknownNode { id } if id.id() == RecipeNodeId::new(0, 9999)),
        "{err:?}"
    );
}

/// **The read and the write are one answer.** `Node::list_input` says
/// which kinds have a list and `set_list_input` writes it; a kind that
/// one treats as list-free while the other writes is a list nothing can
/// read back. Both matches are exhaustive with no wildcard, and this
/// row holds them to the same answer over every node the CORPUS builds,
/// through the public door — so a new kind that grows one and not the
/// other is caught by a row and not only by review.
///
/// The door asks `list_input` first (`SetMembersOnNonList`), and asks
/// the write's own answer again after rewriting; if the two matches
/// ever disagreed, one of those two questions would answer differently
/// from the other and this row would see it. Re-stating a node's OWN
/// current list is the edit used, because it is the one list guaranteed
/// live, acyclic and long enough for every list node in the corpus —
/// so anything that refuses here refuses about the KIND.
#[test]
fn list_input_and_set_list_input_agree_on_every_node_kind() {
    let tol = Tol::witness();
    let mut seen: std::collections::BTreeSet<&'static str> = std::collections::BTreeSet::new();
    for corpus in crate::corpus::documents() {
        let doc = &corpus.doc;
        for id in doc.ids() {
            let Some(node) = doc.node(id) else { continue };
            seen.insert(crate::corpus::node_kind(node));
            let list = list_input(node);
            let has_list = list.is_some();
            let outcome = doc.apply(
                &DocEdit::SetMembers {
                    node: id,
                    members: list.unwrap_or_else(|| {
                        editor_core::Bodies::Spelled(
                            doc.ids()[..2].iter().map(Into::into).collect(),
                        )
                    }),
                },
                tol,
                &editor_core::RefusingReach,
            );
            let non_list = matches!(outcome, Err(EditError::SetMembersOnNonList { .. }));
            assert_eq!(
                has_list,
                !non_list,
                "node {} ({}) reads as list_input={has_list} but the door answers \
                 SetMembersOnNonList={non_list}",
                id.0,
                crate::corpus::node_kind(node)
            );
        }
    }
    // A row that walked no nodes would pass vacuously.
    assert!(
        seen.len() >= 15,
        "the corpus offered only {} node kinds to hold the two matches to",
        seen.len()
    );
}

/// The list a node carries, as the reads it holds — a union's or an
/// intersect's members, or a loft's sections — or `None` for a node
/// with no list. The test-side twin of the retired `Node::list_input`.
fn list_input(
    node: &editor_core::Node<editor_core::ProfileProgram>,
) -> Option<editor_core::Bodies<editor_core::Operand>> {
    match node {
        Node::Union { members, .. } | Node::Intersect { members, .. } => Some(
            members
                .try_map(|_, &m| Ok::<_, std::convert::Infallible>(editor_core::Operand::from(m)))
                .unwrap_or_else(|never| match never {}),
        ),
        Node::Loft { profiles, .. } => Some(editor_core::Bodies::Spelled(
            profiles
                .iter()
                .map(|&m| editor_core::Operand::from(m))
                .collect(),
        )),
        _ => None,
    }
}

/// **`on_set_members`' ordering contract**, which the doc comment
/// states and nothing measured: existing roots keep their ORDER, and
/// nodes the rewrite orphaned join at the END in document order.
///
/// The edit moves the sink set in both directions at once — a member
/// the new list dropped may have become a root, a member it added may
/// have stopped being one — so the set is recomputed rather than
/// spliced, and the recomputation has to be order-stable or a
/// document's root list would shuffle under an unrelated edit.
#[test]
fn set_members_keeps_root_order_and_appends_orphans_last() {
    let tol = Tol::witness();
    // Two unions over four boxes, so the document has two roots; the
    // second union is edited to drop a member, which orphans it.
    let doc = ProfileDoc::empty_derived("docm3_union_roots", tol);
    let (doc, a) = cube(doc, 0.0);
    let (doc, b) = cube(doc, 2.0);
    let (doc, c) = cube(doc, 4.0);
    let (doc, d) = cube(doc, 6.0);
    let (doc, first) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
            declare: Vec::new(),
        },
    );
    let (doc, second) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![c.into(), d.into()]),
            declare: Vec::new(),
        },
    );
    assert_eq!(
        doc.roots(),
        &[first, second],
        "the two unions are the document's roots"
    );
    // Drop `d` from the second union and add nothing: `d` is orphaned.
    let after = doc
        .apply(
            &DocEdit::SetMembers {
                node: second,
                members: editor_core::Bodies::Spelled(vec![c.into(), a.into()]),
            },
            tol,
            &editor_core::RefusingReach,
        )
        .expect("re-membering the second union is a legal edit")
        .doc;
    assert_eq!(
        after.roots(),
        &[first, second, d],
        "the existing roots keep their order and the orphan joins at the end"
    );
}
