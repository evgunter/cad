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
        vec![vec![
            (x0, 0.0),
            (x0 + 1.0, 0.0),
            (x0 + 1.0, 1.0),
            (x0, 1.0),
        ]],
    );
    insert(doc, Node::Extrude {
        profile: p,
        distance: len(1.0),
    })
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
    let (doc, u) = insert(doc, Node::Union {
        members: order.map(|i| boxes[i]).to_vec(),
    });
    (doc, boxes, u)
}

fn body_of(ev: &Evaluation<f64>, id: RecipeNodeId) -> topo::Body<f64> {
    match &ev.value(id).expect("the node evaluated").payload {
        ValuePayload::Body(b) => (**b).clone(),
        ValuePayload::Boolean(BooleanValue::Body { body, .. }) => (**body).clone(),
        other => panic!("expected a body, got {other:?}"),
    }
}

/// The member a union-minted name descends from, or `None` for a name
/// that is not a `FromMember` row.
fn member_of(name: &StableName) -> Option<RecipeNodeId> {
    match name.path.first() {
        Some(RoleSeg::FromMember(inner)) => Some(inner.node),
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
        assert_eq!(
            forward.len(),
            6,
            "member {k} contributes its six box faces"
        );
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
            [RoleSeg::FromMember(inner)] => assert!(
                !matches!(
                    inner.path.first(),
                    Some(RoleSeg::FromA(_) | RoleSeg::FromB(_) | RoleSeg::FromMember(_))
                ),
                "the wrapped name is the member's own, not a fold row: {inner:?}"
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
    let (doc, ab) = insert(doc, Node::Boolean {
        op: BooleanOp::Union,
        a: boxes[0],
        b: boxes[1],
        declare: None,
    });
    let (doc, abc) = insert(doc, Node::Boolean {
        op: BooleanOp::Union,
        a: ab,
        b: boxes[2],
        declare: None,
    });
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

// ---------------------------------------------------------------------
// A5 — SetMembers' refusals, and its round trip.
// ---------------------------------------------------------------------

/// A `SetMembers` at a node that carries no list.
#[test]
fn set_members_refuses_a_node_with_no_list_input() {
    let (doc, boxes, _) = three_boxes([0, 1, 2]);
    let (doc, pair) = insert(doc, Node::Boolean {
        op: BooleanOp::Union,
        a: boxes[0],
        b: boxes[1],
        declare: None,
    });
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
    let (doc, downstream) = insert(doc, Node::Boolean {
        op: BooleanOp::Union,
        a: u,
        b: boxes[0],
        declare: None,
    });
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
// The measured limit: members that share a minting node.
// ---------------------------------------------------------------------

/// **Two members whose names are the same names refuse typed.**
///
/// A member is keyed by the NAME its own table gives an entity, and a
/// pass-through op mints no name of its own: a transform's table IS
/// its input's, verbatim (`eval::wire::wire_transform` — "the input's
/// table rows hold verbatim: same names, same keys", N1's rule that a
/// pass-through adds no segment and the `node` stays the original
/// minter). So two transforms of ONE body carry two identical tables,
/// and `FromMember(inner)` maps their corresponding faces onto one
/// name.
///
/// What the union does with that is the N1 guarantee it must: it
/// refuses, naming the collision, rather than aliasing two faces under
/// one name. This row pins the refusal AND the mechanism behind it —
/// the second assertion is the measurement, and it is what says the
/// member-keyed emitter cannot yet name a union of placed copies of
/// one prototype (the die's 21 pips are exactly that shape).
#[test]
fn members_that_share_a_minting_node_refuse_rather_than_alias() {
    let doc = ProfileDoc::empty_derived("docm3_union", Tol::witness());
    let (doc, base) = cube(doc, 0.0);
    let place = |doc, dx: f64| {
        insert(doc, Node::Transform {
            input: base,
            translation: [len(dx), len(0.0), len(0.0)],
            rotation_axis: [fixture::scl(0.0), fixture::scl(0.0), fixture::scl(1.0)],
            rotation_angle: fixture::ang(0.0),
        })
    };
    let (doc, left) = place(doc, 0.0);
    let (doc, right) = place(doc, 2.0);
    let (doc, u) = insert(doc, Node::Union {
        members: vec![left, right],
    });
    let ev = run(&doc);

    // The mechanism: the two members' tables are the same table.
    assert_eq!(
        all_faces(&ev, left),
        all_faces(&ev, right),
        "a transform passes its input's names through, so two placements of one \
         body carry identical name tables"
    );
    // The consequence: the union refuses, loudly and by name.
    match ev.nodes.get(&u) {
        Some(editor_core::NodeResult::Failed(e)) => assert!(
            format!("{:?}", e.kind).contains("Duplicate"),
            "expected the no-silent-aliasing refusal, got {:?}",
            e.kind
        ),
        other => panic!("expected a typed refusal, got {other:?}"),
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
    let (doc, downstream) = insert(doc, Node::Transform {
        input: u,
        translation: [len(0.0), len(0.0), len(0.0)],
        rotation_axis: [fixture::scl(0.0), fixture::scl(0.0), fixture::scl(1.0)],
        rotation_angle: fixture::ang(0.0),
    });
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
