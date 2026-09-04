//! **The n-ary union** (DOCM-3; DM4–DM6): names keyed by member and
//! not by position, the fold that equals the chain it replaces, the
//! list-input edit, and the pairwise-distinct-inputs rule at both
//! doors.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    BooleanOp, BooleanValue, CancelToken, DocEdit, EditError, EntityKind, EvalOptions, Evaluation,
    Node, ProfileDoc, RecipeNodeId, RoleSeg, StableName, ValuePayload, all_faces, evaluate,
};
use fixture::{insert, len, on_frame};
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
            profile: p,
            distance: len(1.0),
        },
    )
}

/// Three disjoint boxes, and a union of them in the given member
/// order. Returns the document, the three box nodes in construction
/// order, and the union node.
fn three_boxes(order: [usize; 3]) -> (ProfileDoc, [RecipeNodeId; 3], RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("docm3_union", Tol::witness());
    let (doc, a) = cube(doc, 0.0);
    let (doc, b) = cube(doc, 2.0);
    let (doc, c) = cube(doc, 4.0);
    let boxes = [a, b, c];
    let (doc, u) = insert(
        doc,
        Node::Union {
            members: order.map(|i| boxes[i]).to_vec(),
        },
    );
    (doc, boxes, u)
}

fn body_of(ev: &Evaluation<f64>, id: RecipeNodeId) -> topo::Body<f64> {
    match &ev.value(id).expect("the node evaluated").payload {
        ValuePayload::Body(b) => (**b).clone(),
        ValuePayload::Boolean(BooleanValue::Body { body, .. }) => (**body).clone(),
        other => panic!("expected a body, got {other:?}"),
    }
}

/// The member a union-minted name came from, or `None` for a name that
/// is not a `FromMember` row. Read off the member EDGE, which is the
/// only thing that answers: the inner name's minting node is the
/// PROTOTYPE's where a member is a placement of one.
fn member_of(name: &StableName) -> Option<RecipeNodeId> {
    match name.path.first() {
        Some(RoleSeg::FromMember { member, .. }) => Some(*member),
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
        let faces_under = |ev: &Evaluation<f64>, u: RecipeNodeId| -> Vec<StableName> {
            let mut v: Vec<StableName> = all_faces(ev, u)
                .into_iter()
                .filter(|n| member_of(n) == Some(*member))
                .collect();
            v.sort();
            v
        };
        let forward = faces_under(&ev, u);
        assert_eq!(forward.len(), 6, "member {k} contributes its six box faces");
        assert_eq!(
            forward,
            faces_under(&rev_ev, rev_u),
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
            [RoleSeg::FromMember { of, .. }] => assert!(
                !matches!(
                    of.path.first(),
                    Some(RoleSeg::FromA(_) | RoleSeg::FromB(_) | RoleSeg::FromMember { .. })
                ),
                "the wrapped name is the member's own, not a fold row: {of:?}"
            ),
            other => panic!("a disjoint union's faces are single `FromMember` rows: {other:?}"),
        }
    }
}

// ---------------------------------------------------------------------
// A3 — the fold equals the chain.
// ---------------------------------------------------------------------

/// **The fold is the chain's geometry.** One document carrying both a
/// three-member `Node::Union` and the pairwise `Boolean(Union)` chain
/// over the same three bodies: the two values agree face for face,
/// edge for edge, on the bits of every description. Only the NAMES
/// differ, which is the whole content of the change.
#[test]
fn the_fold_and_the_pairwise_chain_are_the_same_body() {
    let (doc, boxes, u) = three_boxes([0, 1, 2]);
    let (doc, ab) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: boxes[0],
            b: boxes[1],
            declare: None,
        },
    );
    let (doc, abc) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: ab,
            b: boxes[2],
            declare: None,
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
    let bits = |b: &topo::Body<f64>| {
        let mut v: Vec<String> = b.surfaces().map(|(_, s)| format!("{s:?}")).collect();
        v.sort();
        v
    };
    assert_eq!(
        bits(&folded),
        bits(&chained),
        "the fold's surfaces are not the chain's, description for description"
    );
    // And the names are what moved: the chain's are two descents deep
    // for the first member, the fold's are one wrapper for every member.
    let chain_names = all_faces(&ev, abc);
    assert!(
        chain_names
            .iter()
            .any(|n| matches!(n.path.first(), Some(RoleSeg::FromA(inner))
                if matches!(inner.path.first(), Some(RoleSeg::FromA(_))))),
        "the pairwise chain nests its first operand twice"
    );
}

// ---------------------------------------------------------------------
// A4 — DM5 refuses at both doors.
// ---------------------------------------------------------------------

/// **A node's inputs are pairwise distinct**, refused at the INSERT
/// door for every shape that can repeat one: the pair boolean, the
/// n-ary union's list, and the split.
#[test]
fn insert_refuses_a_node_that_takes_one_input_twice() {
    let (doc, boxes, _) = three_boxes([0, 1, 2]);
    let x = boxes[0];
    let shapes: Vec<Node<editor_core::ProfileProgram>> = vec![
        Node::Boolean {
            op: BooleanOp::Union,
            a: x,
            b: x,
            declare: None,
        },
        Node::Union {
            members: vec![x, x],
        },
        Node::Split { target: x, tool: x },
    ];
    for node in shapes {
        let err = doc
            .apply(&DocEdit::InsertNode { node: node.clone() }, Tol::witness())
            .expect_err("a repeated input must refuse");
        assert!(
            matches!(err, EditError::DuplicateInput { input, .. } if input == x),
            "{node:?} refused with {err:?}"
        );
    }
}

/// The same rule at the OTHER door: a `SetMembers` that would leave
/// one node in the list twice.
#[test]
fn set_members_refuses_a_duplicate_member() {
    let (doc, boxes, u) = three_boxes([0, 1, 2]);
    let err = doc
        .apply(
            &DocEdit::SetMembers {
                node: u,
                members: vec![boxes[0], boxes[1], boxes[0]],
            },
            Tol::witness(),
        )
        .expect_err("a duplicate member must refuse");
    assert!(
        matches!(err, EditError::DuplicateInput { node, input } if node == u && input == boxes[0]),
        "{err:?}"
    );
}

/// The THIRD door: a SNAPSHOT carrying a node the edit doors refuse.
///
/// A saved file is data, and its snapshot is the one way a node
/// reaches a document without passing `apply` — the edit log beside it
/// replays through the doors. So the load validator asks the same
/// function, and a hand-written file holding `Boolean { a: X, b: X }`
/// or a one-member union refuses rather than loading.
#[test]
fn a_snapshot_carrying_a_refused_node_does_not_load() {
    let tol = Tol::witness();
    let (doc, boxes, u) = three_boxes([0, 1, 2]);
    // The document is valid as saved; the corruption is introduced in
    // the SNAPSHOT afterwards, which is what a tampered file is.
    let text = editor_core::persist::save(&doc, &[], tol).expect("the document saves");
    // The member list is rewritten in place, whatever whitespace the
    // writer used around it, so the fixture is about the LIST and not
    // about the formatting.
    let corrupt = |members: String| {
        let (head, rest) = text
            .split_once("\"members\": [")
            .expect("the union's list is on the wire");
        let (_, tail) = rest.split_once(']').expect("the list closes");
        let tampered = format!("{head}\"members\": [{members}]{tail}");
        editor_core::persist::load(&tampered, tol)
    };
    // A repeated member in the union's list.
    let err = corrupt(format!("{},{},{}", boxes[0].0, boxes[1].0, boxes[0].0))
        .expect_err("a duplicate member must refuse");
    let said = format!("{err}");
    assert!(
        said.contains("pairwise distinct") && said.contains(&format!("{}", u.0)),
        "{said}"
    );
    // And a list left under two.
    let err = corrupt(format!("{}", boxes[0].0)).expect_err("a one-member union must refuse");
    let said = format!("{err}");
    assert!(said.contains("two or more"), "{said}");
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
        Node::Boolean {
            op: BooleanOp::Union,
            a: boxes[0],
            b: boxes[1],
            declare: None,
        },
    );
    let err = doc
        .apply(
            &DocEdit::SetMembers {
                node: pair,
                members: vec![boxes[0], boxes[2]],
            },
            Tol::witness(),
        )
        .expect_err("a boolean carries no list");
    assert!(
        matches!(err, EditError::SetMembersOnNonList { node } if node == pair),
        "{err:?}"
    );
}

/// A member that is not a live node.
#[test]
fn set_members_refuses_a_member_that_is_not_live() {
    let (doc, boxes, u) = three_boxes([0, 1, 2]);
    let ghost = RecipeNodeId(9999);
    let err = doc
        .apply(
            &DocEdit::SetMembers {
                node: u,
                members: vec![boxes[0], ghost],
            },
            Tol::witness(),
        )
        .expect_err("a dangling member must refuse");
    assert!(
        matches!(err, EditError::UnresolvedInput { input } if input == ghost),
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
        Node::Boolean {
            op: BooleanOp::Union,
            a: u,
            b: boxes[0],
            declare: None,
        },
    );
    let err = doc
        .apply(
            &DocEdit::SetMembers {
                node: u,
                members: vec![boxes[1], downstream],
            },
            Tol::witness(),
        )
        .expect_err("a member below the node closes a loop");
    assert!(matches!(err, EditError::WouldCycle { .. }), "{err:?}");
}

/// A list left with one entry.
#[test]
fn set_members_refuses_fewer_than_two() {
    let (doc, boxes, u) = three_boxes([0, 1, 2]);
    let err = doc
        .apply(
            &DocEdit::SetMembers {
                node: u,
                members: vec![boxes[0]],
            },
            Tol::witness(),
        )
        .expect_err("a union of one is its own input");
    assert!(
        matches!(err, EditError::TooFewMembers { node, found } if node == u && found == 1),
        "{err:?}"
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
        .order()
        .iter()
        .map(|id| DocEdit::InsertNode {
            node: doc.node(*id).expect("an ordered node").clone(),
        })
        .collect();
    edits.push(DocEdit::SetMembers {
        node: u,
        members: vec![boxes[2], boxes[0]],
    });
    edits.push(DocEdit::DeleteNode { id: boxes[1] });
    let mut replayed = empty.clone();
    for edit in &edits {
        replayed = replayed.apply(edit, tol).expect("the log replays").doc;
    }
    let text = editor_core::persist::save(&empty, &edits, tol).expect("the document saves");
    let loaded = editor_core::persist::load(&text, tol).expect("the document loads");
    assert!(
        loaded.doc.bit_eq(&replayed),
        "the loaded replay is not the document the log builds"
    );
    assert_eq!(loaded.edits.len(), edits.len());
    // And the surviving union is the two-member one the log states.
    let Some(Node::Union { members }) = loaded.doc.node(u) else {
        panic!("the union survived as something else")
    };
    assert_eq!(members, &vec![boxes[2], boxes[0]]);
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
        .filter(|n| member_of(n) != Some(boxes[0]))
        .collect();
    let doc = doc
        .apply(
            &DocEdit::SetMembers {
                node: u,
                members: vec![boxes[1], boxes[2]],
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
            Node::Transform {
                input: base,
                translation: [len(dx), len(0.0), len(0.0)],
                rotation_axis: [fixture::scl(0.0), fixture::scl(0.0), fixture::scl(1.0)],
                rotation_angle: fixture::ang(0.0),
            },
        )
    };
    let (doc, left) = place(doc, 0.0);
    let (doc, right) = place(doc, 2.0);
    let (doc, u) = insert(
        doc,
        Node::Union {
            members: vec![left, right],
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
                .filter(|n| member_of(n) == Some(member))
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
            Some(RoleSeg::FromMember { of, .. }) => Some((**of).clone()),
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
        .order()
        .iter()
        .copied()
        .find(|id| matches!(doc.node(*id), Some(Node::Union { .. })))
        .expect("the die fuses its pips with one union");
    let Some(Node::Union { members }) = doc.node(union) else {
        panic!("the union is a union")
    };
    let members = members.clone();
    assert_eq!(members.len(), 21, "the die has 21 pips");
    let blends: Vec<RecipeNodeId> = doc
        .order()
        .iter()
        .copied()
        .filter(|id| matches!(doc.node(*id), Some(Node::Fillet { .. })))
        .collect();
    assert_eq!(blends.len(), 2, "the box-edge blend and the rim blend");
    let before = run(&doc);
    let (rim_target, rim_radius, rims) = match doc.node(blends[1]) {
        Some(Node::Fillet {
            target,
            radius,
            selection,
        }) => (*target, radius.clone(), selection.clone()),
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
                    members: kept,
                },
                tol,
            )
            .unwrap_or_else(|e| panic!("dropping the {label} pip: {e}"))
            .doc;
        let edited = edited
            .apply(&DocEdit::DeleteNode { id: members[k] }, tol)
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
        // frozen names say so — every one of them names the dead
        // member and nothing else does. That is the whole claim: the
        // damage is exactly the removed member's, told apart by the
        // member edge in the name and by nothing positional.
        let doomed: Vec<StableName> = rims
            .iter()
            .filter(|n| editor_core::derivation_nodes(n).contains(&members[k]))
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
            Node::fillet(rim_target, rim_radius.clone(), kept_rims.clone()),
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
/// two runs agreeing. Face, edge and vertex counts agree and every
/// surface description agrees bit for bit; only the NAMES differ,
/// which is the whole content of the change.
#[test]
fn the_dies_union_is_the_chain_it_replaced() {
    let die = crate::corpus::die_composed_tour::document();
    let doc = die.doc;
    let union = doc
        .order()
        .iter()
        .copied()
        .find(|id| matches!(doc.node(*id), Some(Node::Union { .. })))
        .expect("the die fuses its pips with one union");
    let Some(Node::Union { members }) = doc.node(union) else {
        panic!("the union is a union")
    };
    let members = members.clone();
    // The chain this replaced, re-authored over the same members.
    let (doc, chain) = members.iter().skip(1).fold(
        (doc, members[0]),
        |(doc, acc): (ProfileDoc, RecipeNodeId), pip| {
            insert(
                doc,
                Node::Boolean {
                    op: BooleanOp::Union,
                    a: acc,
                    b: *pip,
                    declare: None,
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
    let bits = |b: &topo::Body<f64>| {
        let mut v: Vec<String> = b.surfaces().map(|(_, s)| format!("{s:?}")).collect();
        v.sort();
        v
    };
    assert_eq!(
        bits(&folded),
        bits(&chained),
        "the fold's surfaces are not the chain's, description for description"
    );
    // And the names are what moved. The chain's LAST member is one
    // descent deep and its FIRST is twenty; every member of the fold is
    // one wrapper, whatever its position.
    let depth = |n: &StableName| {
        let mut d = 0;
        let mut cur = n.path.first();
        while let Some(RoleSeg::FromA(inner) | RoleSeg::FromB(inner)) = cur {
            d += 1;
            cur = inner.path.first();
        }
        d
    };
    assert_eq!(
        all_faces(&ev, union).iter().map(depth).max(),
        Some(0),
        "a union's names carry no operand descent at all"
    );
    assert_eq!(
        all_faces(&ev, chain).iter().map(depth).max(),
        Some(20),
        "the chain's first member is twenty descents deep"
    );
}

/// How many names a blend node's selection carries.
fn selection_len(doc: &ProfileDoc, blend: RecipeNodeId) -> usize {
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
        Node::Transform {
            input: u,
            translation: [len(0.0), len(0.0), len(0.0)],
            rotation_axis: [fixture::scl(0.0), fixture::scl(0.0), fixture::scl(1.0)],
            rotation_angle: fixture::ang(0.0),
        },
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
