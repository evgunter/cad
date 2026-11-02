//! **The n-ary union** (DOCM-3; DM4–DM6): names keyed by member and
//! not by position, the fold that equals the chain it replaces, the
//! list-input edit, and the repeated-read glue at every door.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use crate::fixture::member_of;

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
        },
    )
}

/// Three disjoint boxes, and a union of them in the given member
/// order (one union node for every order, [`crate::fixture::union_over`]). Returns the document, the three box operands in construction
/// order — each a copy placed against the first box, so any member can
/// be dropped and deleted alone — and the union node.
fn three_boxes(order: [usize; 3]) -> (ProfileDoc, [RecipeNodeId; 3], RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("docm3_union", Tol::witness());
    let (doc, root) = cube(doc, 0.0);
    let (doc, a) = fixture::slid(doc, root, root, [0.0; 3]);
    let (doc, b) = cube(doc, 2.0);
    let (doc, b) = fixture::slid(doc, root, b, [0.0; 3]);
    let (doc, c) = cube(doc, 4.0);
    let (doc, c) = fixture::slid(doc, root, c, [0.0; 3]);
    let boxes = [a, b, c];
    let (doc, u) = crate::fixture::union_over(doc, &order.map(|i| boxes[i]));
    (doc, boxes, u)
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
        },
    );
    let (doc, abc) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![ab.into(), boxes[2].into()]),
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
        surfaces(&folded),
        surfaces(&chained),
        "the fold's surfaces are not the chain's, description for description"
    );
    assert_eq!(
        curves(&folded),
        curves(&chained),
        "the fold's curves are not the chain's, description for description"
    );
    assert_eq!(
        points(&folded),
        points(&chained),
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
        },
    );
    let (doc, meet) = insert(
        doc,
        Node::Intersect {
            members: editor_core::Bodies::Spelled(vec![x.into(), x.into()]),
        },
    );
    let (doc, cut) = insert(
        doc,
        Node::Subtract {
            from: x.into(),
            tool: x.into(),
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
            .map(|m| {
                loaded
                    .operation_of(m.read)
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
        },
    );
    let err = doc
        .apply(
            &DocEdit::SetMembers {
                node: pair,
                members: editor_core::Bodies::Spelled(vec![boxes[0].into(), boxes[2].into()]),
            },
            Tol::witness(),
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
        },
    );
    let err = doc
        .apply(
            &DocEdit::SetMembers {
                node: u,
                members: editor_core::Bodies::Spelled(vec![boxes[1].into(), downstream.into()]),
            },
            Tol::witness(),
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
                members: editor_core::Bodies::Spelled(
                    members.into_iter().map(Into::into).collect(),
                ),
            },
            Tol::witness(),
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
    // Three disjoint boxes, the later two placed against the first, and
    // their union, as the log that authored them.
    let mut r = crate::fixture::Recorder::new();
    let empty = r.doc.clone();
    let mut boxes = Vec::new();
    for x0 in [0.0, 2.0, 4.0] {
        let p = r.profile(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(x0, 0.0), (x0 + 1.0, 0.0), (x0 + 1.0, 1.0), (x0, 1.0)]],
        );
        let b = r.insert(Node::Extrude {
            profile: p.into(),
            distance: len(1.0),
        });
        boxes.push(match boxes.first() {
            None => b,
            Some(&a) => r.against(a, b, &editor_core::Frame::IDENTITY),
        });
    }
    let u = r.insert(Node::Union {
        members: editor_core::Bodies::Spelled(boxes.iter().map(|&b| b.into()).collect()),
    });
    let mut edits = r.edits;
    edits.push(DocEdit::SetMembers {
        node: u,
        members: editor_core::Bodies::Spelled(vec![boxes[2].into(), boxes[0].into()]),
    });
    edits.push(DocEdit::DeleteNode { id: boxes[1] });
    let mut replayed = empty.clone();
    for edit in &edits {
        replayed = replayed.apply(edit, tol).expect("the log replays").doc;
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
        .map(|m| {
            loaded
                .doc
                .operation_of(m.read)
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
        )
        .expect("the drop applies")
        .doc;
    let doc = doc
        .apply(&DocEdit::DeleteNode { id: boxes[0] }, Tol::witness())
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
/// A copy carries its source's names as they are, so two placements of
/// one body carry one table, and a wrapper keyed on the prototype's name
/// alone would map their corresponding faces onto one name and refuse.
///
/// The segment keys on the member's read instead, so the two members
/// are told apart by the DAG edge that makes them two. The first
/// assertion is the mechanism — the two tables are one — and the second
/// is what the union does with it. The die's 21 pips are exactly this
/// shape, at scale.
#[test]
fn two_placements_of_one_prototype_are_two_members() {
    let doc = ProfileDoc::empty_derived("docm3_union", Tol::witness());
    let (doc, base) = cube(doc, 0.0);
    let place = |doc, dx: f64| fixture::slid(doc, base, base, [dx, 0.0, 0.0]);
    let (doc, left) = place(doc, 0.0);
    let (doc, right) = place(doc, 2.0);
    let (doc, u) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![left.into(), right.into()]),
        },
    );
    let ev = run(&doc);

    // The mechanism: the two members carry one table, the body's.
    assert_eq!(
        all_faces(&ev, left),
        all_faces(&ev, right),
        "two placements of one body carry the body's names as they are"
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
    // And the prototype's names alone would NOT have told them apart:
    // strip the member edge and the twelve collapse to six.
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
/// `SetMembers`, delete the orphaned placement, and re-evaluate with
/// the previous evaluation as `prior`. Both fillets must still
/// evaluate `Ok`, with two fewer rim arcs selected (a pip contributes
/// two rim edges) and no `SelectResolve` anywhere.
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
    let member_reads: Vec<editor_core::VarId> = members.reads().map(|m| m.read).collect();
    let members: Vec<RecipeNodeId> = member_reads
        .iter()
        .map(|&m| doc.operation_of(m).expect("a member reads a live output"))
        .collect();
    assert_eq!(members.len(), 21, "the die has 21 pips");
    // The die is what the world places: the rim blend, over the
    // box-edge blend. Every other placement places a pip on the cube.
    let in_world: Vec<RecipeNodeId> = doc
        .placements()
        .into_iter()
        .filter(|&p| crate::fixture::on_world(&doc, p))
        .collect();
    let rim_blend = match in_world.as_slice() {
        [placement] => match doc.node(*placement) {
            Some(Node::Place { source, .. }) => doc
                .operation_of(source.read)
                .expect("the placement reads the die"),
            other => panic!("a placement, got {other:?}"),
        },
        other => panic!("the tour places the die alone, got {other:?}"),
    };
    let box_blend = match doc.node(rim_blend) {
        Some(Node::Fillet { selection, .. }) => {
            doc.read_operation(*selection).expect("a live target")
        }
        other => panic!("the die is its rim blend, got {other:?}"),
    };
    assert!(
        matches!(doc.node(box_blend), Some(Node::Fillet { .. })),
        "the rim blend's target is the box-edge blend"
    );
    let blends = [box_blend, rim_blend];
    let before = run(&doc);
    // The radius as written: deleting the blend retires the anonymous
    // variable its radius reads, so the re-authored blend writes it
    // again.
    let (rim_target, rim_radius, rims) = match doc.node(blends[1]).map(|n| n.written(&doc)) {
        Some(Node::Fillet {
            radius,
            selection: editor_core::Operand::Select { body, names },
        }) => (*body, radius, names),
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
            )
            .unwrap_or_else(|e| panic!("dropping the {label} pip: {e}"))
            .doc;
        let edited = edited
            .apply(&DocEdit::DeleteNode { id: members[k] }, tol)
            .unwrap_or_else(|e| panic!("deleting the orphaned {label} placement: {e}"))
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
            .apply(&DocEdit::DeleteNode { id: blends[1] }, tol)
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
        // has them. That is what `SelectResolve` would say, and
        // the assertions above are that it does not.
    }
}

/// **A3 at the die's size**: the union of the 21 pips is the pairwise
/// chain it replaced, body for body.
///
/// Both are built in ONE document over the SAME 21 placements and
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
        .map(|m| {
            doc.operation_of(m.read)
                .expect("a member reads a live output")
        })
        .collect();
    // The chain this replaced, re-authored over the same members.
    let (doc, chain) = members.iter().skip(1).fold(
        (doc, members[0]),
        |(doc, acc): (ProfileDoc, RecipeNodeId), pip| {
            insert(
                doc,
                Node::Union {
                    members: editor_core::Bodies::Spelled(vec![acc.into(), (*pip).into()]),
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
        surfaces(&folded),
        surfaces(&chained),
        "the fold's surfaces are not the chain's, description for description"
    );
    assert_eq!(
        curves(&folded),
        curves(&chained),
        "the fold's curves are not the chain's, description for description"
    );
    assert_eq!(
        points(&folded),
        points(&chained),
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
        Some(Node::Fillet { selection, .. }) => crate::fixture::selected(doc, *selection).len(),
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
    let (doc, downstream) = fixture::place(doc, u);
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
// A later fold step against a flush member: what it glues and what
// the union publishes.
// ---------------------------------------------------------------------

/// A box on its own xy, footprint `[x0,x1]×[y0,y1]`, `h` tall.
fn boxed(doc: ProfileDoc, x: (f64, f64), y: (f64, f64), h: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(h),
        },
    )
}

fn failure(ev: &Evaluation<f64>, id: RecipeNodeId) -> Option<String> {
    match ev.nodes.get(&id) {
        Some(editor_core::NodeResult::Failed(e)) => Some(format!("{:?}", e.kind)),
        _ => None,
    }
}

/// **A union whose LATER fold step meets a flush member builds, as the
/// pair boolean of the two members that touch does, and publishes only
/// member-space names.**
///
/// The third member shares member 0's `x = 1` plane over a patch member
/// 1 does not cover, so step 2 folds a coincident pair against the
/// ACCUMULATED operand, whose table is the pair emitter's,
/// `FromA`/`FromB`-headed. The margins decide that pair one carrier, so
/// the step glues it exactly as the pair spelling `a ∪ d` does, and the
/// union's published table carries no fold row.
#[test]
fn a_later_fold_step_glues_a_flush_member_as_the_pair_does() {
    let doc = ProfileDoc::empty_derived("docm3_union_menu", Tol::witness());
    let (doc, a) = boxed(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, b) = boxed(doc, (0.5, 1.5), (0.2, 0.8), 0.5);
    let (doc, b) = fixture::slid(doc, a, b, [0.0, 0.0, 0.2]);
    let (doc, d) = boxed(doc, (1.0, 2.0), (0.0, 0.15), 1.0);
    let (doc, d) = fixture::slid(doc, a, d, [0.0; 3]);
    let (bare, u) = insert(
        doc.clone(),
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![a.into(), b.into(), d.into()]),
        },
    );
    let (bare, pair) = insert(
        bare,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![a.into(), d.into()]),
        },
    );
    let ev = run(&bare);
    assert_eq!(failure(&ev, pair), None, "the pair spelling builds");
    assert_eq!(failure(&ev, u), None, "the fold builds through step 2");
    let table = format!("{:?}", ev.table_at(u).expect("the union built"));
    assert!(
        !table.contains("FromA(") && !table.contains("FromB("),
        "the union publishes an uncollapsed fold row: {table}"
    );
}

// ---------------------------------------------------------------------
// The list floor reaches `Loft`, at both doors. Measured by both
// review branches (`r2_a_one_section_loft_at_the_insert_door`); the
// widening is disclosed in the PR body's deviation list.
// ---------------------------------------------------------------------

/// Four sections and a three-section loft over the first three, each
/// later one a unit further up the first's own z.
fn loft_doc() -> (ProfileDoc, RecipeNodeId, Vec<RecipeNodeId>) {
    let mut doc = ProfileDoc::empty_derived("docm3_union_loft", Tol::witness());
    let mut profiles = Vec::new();
    for s in [1.0, 1.6, 1.0, 1.2] {
        let (d, id) = on_frame(
            doc,
            [0.0; 3],
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
            first: profiles[0].into(),
            then: (1..3)
                .map(|i| editor_core::LoftSection {
                    profile: profiles[i].into(),
                    station: vec![editor_core::StationMove::Slide {
                        along: editor_core::Axis3::Z,
                        by: len(i as f64),
                    }],
                })
                .collect(),
            v_degree: editor_core::Formula::count(2),
        },
    );
    (doc, loft, profiles)
}

/// **The load door does not ask a loft's shape**: a snapshot whose loft
/// stands in a shape the edit doors refuse loads, and evaluation
/// refuses the loft. The shape here is a degree the sections cannot
/// carry, reached through a variable moved after the insert (the edit
/// doors refuse a degree past the sections as written, and a section
/// count below two in any form). The load validator asks
/// `input_fault`, which carries no list floor, so a document saved
/// under an older door loads as it was written.
#[test]
fn a_snapshot_carrying_a_loft_the_doors_refuse_loads() {
    let tol = Tol::witness();
    let (doc, _, profiles) = loft_doc();
    let skin = editor_core::VarName::from_static("skin");
    let (doc, _) = crate::fixture::step(
        doc,
        DocEdit::DeclareVar {
            name: skin.clone(),
            def: editor_core::VarDecl::Free(editor_core::FreeVar::Count { value: 1 }),
        },
    );
    let (doc, loft) = insert(
        doc,
        Node::Loft {
            first: profiles[0].into(),
            then: vec![editor_core::LoftSection {
                profile: profiles[1].into(),
                station: vec![editor_core::StationMove::Slide {
                    along: editor_core::Axis3::Z,
                    by: len(1.0),
                }],
            }],
            v_degree: editor_core::Formula::named(skin, editor_core::Dimension::Count),
        },
    );
    let var = doc.var_named("skin").expect("declared");
    let (doc, _) = crate::fixture::step(
        doc,
        DocEdit::SetVarValue {
            var: var.into(),
            value: editor_core::FreeValue::Count(2),
        },
    );
    let text = editor_core::persist::save(&doc, &[], tol).expect("the document saves");
    let loaded = editor_core::persist::load(&text, tol).expect("the loft loads");
    let ev = run(&loaded.doc);
    assert!(
        ev.value(loft).is_none(),
        "degree 2 through two sections refuses at evaluation: {:?}",
        ev.nodes.get(&loft)
    );
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

/// The list a node carries, as the reads it holds — a union's, an
/// intersect's or a `⊔`'s members, or a loft's sections — or `None` for a node
/// with no list. The test-side twin of the retired `Node::list_input`.
fn list_input(
    node: &editor_core::Node<editor_core::ProfileProgram>,
) -> Option<editor_core::Bodies<editor_core::BodyRead<editor_core::Formula>>> {
    match node {
        Node::Union { members, .. }
        | Node::Intersect { members, .. }
        | Node::Assemble { members } => Some(
            members
                .try_map(|_, m| {
                    Ok::<_, std::convert::Infallible>(editor_core::BodyRead::plain(m.read))
                })
                .unwrap_or_else(|never| match never {}),
        ),
        Node::Loft { first, then, .. } => Some(editor_core::Bodies::Spelled(
            std::iter::once(*first)
                .chain(then.iter().map(|s| s.profile))
                .map(editor_core::BodyRead::plain)
                .collect(),
        )),
        _ => None,
    }
}
