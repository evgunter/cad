//! **Review rows for a subtract's `Bodies` tool seat** (PR 7426,
//! DESIGN `[repetition]`, REFERENCES DM4): the cut is the fold
//! `a − t₁ − … − tₙ`, order-free in the tools; the door refuses an
//! indexed family argument; `SetMembers` keeps the names of the tools
//! that stay.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block, block_on};
use crate::fixture::{self, insert, insert_refused, step};
use editor_core::{
    Bodies, BodyRead, BooleanValue, DocEdit, EditError, Evaluation, Formula, Node, Operand,
    ProfileDoc, RecipeNodeId, TargetAxis, ValuePayload,
};
use geom_core::Tol;

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    fixture::run(doc, &Default::default())
}

fn read(id: RecipeNodeId) -> BodyRead<Formula> {
    BodyRead::plain(Operand::Node(id))
}

fn volume(ev: &Evaluation<f64>, id: RecipeNodeId) -> f64 {
    let b = match ev.value(id).map(|v| &v.payload) {
        Some(ValuePayload::Boolean(BooleanValue::Body { body, .. })) => body.body().clone(),
        Some(ValuePayload::Body(body)) => body.body().clone(),
        _ => panic!("{id:?} is not one body: {:?}", ev.result(id)),
    };
    let measured = topo::test_support::finished("the measured body", b, Tol::witness());
    topo::test_support::pinned_volume("the measured body", &measured, Tol::witness()).volume
}

/// Each name as spelled, the node's own id said as `SELF`, sorted.
fn names(ev: &Evaluation<f64>, id: RecipeNodeId) -> Vec<String> {
    let own = format!("{id:?}");
    let mut spelled: Vec<String> = ev
        .table_at(id)
        .unwrap_or_else(|| panic!("{id:?} has no table: {:?}", ev.result(id)))
        .iter()
        .map(|(n, _)| format!("{:?} {:?}", n.kind, n.path).replace(&own, "SELF"))
        .collect();
    spelled.sort();
    spelled
}

/// Each coincidence row, origin left out, cells sorted, the node's own
/// id said as `SELF`, sorted.
fn rows(ev: &Evaluation<f64>, id: RecipeNodeId) -> Vec<String> {
    let own = format!("{id:?}");
    let mut out: Vec<String> = ev
        .value(id)
        .unwrap()
        .coincidences
        .iter()
        .map(|r| {
            let mut cells: Vec<String> = r.cells.iter().map(|c| format!("{c:?}")).collect();
            cells.sort();
            format!("{:?} {cells:?}", r.relation).replace(&own, "SELF")
        })
        .collect();
    out.sort();
    out
}

/// A plate 3 × 1 × 0.2 and two blind pockets that cross: a long
/// shallow slot along x and a short deeper one along y, so the second
/// cut in either order splits faces the first one made.
fn plate_and_crossing_pockets() -> (ProfileDoc, RecipeNodeId, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("review-subtract-bodies", Tol::witness());
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 1.0), 0.2);
    let (doc, long) = block_on(doc, plate, (0.5, 2.5), (0.4, 0.6), 0.1, 1.0);
    let (doc, short) = block_on(doc, plate, (1.4, 1.6), (0.2, 0.8), 0.05, 1.0);
    (doc, plate, long, short)
}

/// **The fold is order-free in its names and its rows** (DM4: "order-free
/// in the tools"). Two crossing pockets, listed both ways, cut the same
/// solid; red if the names or the coincidence rows remember which tool
/// was folded first (the PR's own claim 1: the union's end passes do
/// not run here).
#[test]
fn crossing_tools_name_alike_in_either_order() {
    let (doc, plate, long, short) = plate_and_crossing_pockets();
    let (doc, forward) = insert(
        doc,
        Node::Subtract {
            from: read(plate),
            tool: Bodies::spelled(vec![read(long), read(short)]),
        },
    );
    let (doc, backward) = insert(
        doc,
        Node::Subtract {
            from: read(plate),
            tool: Bodies::spelled(vec![read(short), read(long)]),
        },
    );
    let ev = run(&doc);
    let removed = (2.0 * 0.2 * 0.1) + (0.2 * 0.6 * 0.15) - (0.2 * 0.2 * 0.1);
    for id in [forward, backward] {
        let v = volume(&ev, id);
        assert!((v - (0.6 - removed)).abs() < 1e-9, "{id:?}: {v}");
    }
    let (f, b) = (names(&ev, forward), names(&ev, backward));
    let only_f: Vec<&String> = f.iter().filter(|n| !b.contains(n)).collect();
    let only_b: Vec<&String> = b.iter().filter(|n| !f.contains(n)).collect();
    assert!(
        only_f.is_empty() && only_b.is_empty() && f.len() == b.len(),
        "the fold's names do not remember the tool order: {} vs {} names;\n\
         only forward: {only_f:#?}\nonly backward: {only_b:#?}",
        f.len(),
        b.len()
    );
}

/// **The fold's coincidence rows are order-free too**, as statements
/// (cells sorted, origin left out).
#[test]
fn crossing_tools_decide_alike_in_either_order() {
    let (doc, plate, long, short) = plate_and_crossing_pockets();
    let (doc, forward) = insert(
        doc,
        Node::Subtract {
            from: read(plate),
            tool: Bodies::spelled(vec![read(long), read(short)]),
        },
    );
    let (doc, backward) = insert(
        doc,
        Node::Subtract {
            from: read(plate),
            tool: Bodies::spelled(vec![read(short), read(long)]),
        },
    );
    let ev = run(&doc);
    assert_eq!(rows(&ev, forward), rows(&ev, backward));
}

/// **The union of the same two pockets is order-free in its names**:
/// the control for [`crossing_tools_name_alike_in_either_order`], so
/// the subtract fold's order-dependence is its own, not the class's
/// (the union runs `names::name_union`'s end passes; `fold_cuts` does
/// not).
#[test]
fn the_union_of_the_same_crossing_tools_names_alike_in_either_order() {
    let (doc, _, long, short) = plate_and_crossing_pockets();
    let (doc, forward) = insert(
        doc,
        Node::Union {
            members: Bodies::spelled(vec![read(long), read(short)]),
        },
    );
    let (doc, backward) = insert(
        doc,
        Node::Union {
            members: Bodies::spelled(vec![read(short), read(long)]),
        },
    );
    let ev = run(&doc);
    let (f, b) = (names(&ev, forward), names(&ev, backward));
    assert_eq!(f, b, "the union of the same two tools is order-free");
}

/// **`SetMembers` on a subtract's tools keeps the names of the tools
/// that stay**: three holes, the middle one dropped. Every name the
/// two-hole cut publishes is one the three-hole cut published. Red if
/// dropping a tool re-names the faces of the others (a chain's depth).
#[test]
fn set_members_on_a_subtract_keeps_the_names_of_the_tools_that_stay() {
    let doc = ProfileDoc::empty_derived("review-subtract-set-members", Tol::witness());
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 1.0), 0.2);
    let (doc, h0) = block_on(doc, plate, (0.25, 0.5), (0.25, 0.75), -1.0, 3.0);
    let (doc, h1) = block_on(doc, plate, (1.25, 1.5), (0.25, 0.75), -1.0, 3.0);
    let (doc, h2) = block_on(doc, plate, (2.25, 2.5), (0.25, 0.75), -1.0, 3.0);
    let (doc, cut) = insert(
        doc,
        Node::Subtract {
            from: read(plate),
            tool: Bodies::spelled(vec![read(h0), read(h1), read(h2)]),
        },
    );
    let before = names(&run(&doc), cut);
    let (after_doc, _) = step(
        doc,
        DocEdit::SetMembers {
            node: cut,
            members: Bodies::spelled(vec![read(h0), read(h2)]),
        },
    );
    let ev = run(&after_doc);
    let v = volume(&ev, cut);
    let hole = 0.25 * 0.5 * 0.2;
    assert!((v - (0.6 - 2.0 * hole)).abs() < 1e-9, "two holes left: {v}");
    let after = names(&ev, cut);
    let lost: Vec<&String> = after.iter().filter(|n| !before.contains(n)).collect();
    assert!(
        lost.is_empty(),
        "names minted afresh by the drop: {lost:#?}"
    );
}

/// **An indexed family argument at the tools refuses
/// `InputFault::IndexedFamily`**, at the insert door and at
/// `SetMembers`, as a union's does.
#[test]
fn an_indexed_family_argument_at_the_tools_refuses_at_both_doors() {
    let doc = ProfileDoc::empty_derived("review-subtract-indexed-family", Tol::witness());
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 1.0), 0.2);
    let (doc, hole) = block_on(doc, plate, (0.25, 0.5), (0.25, 0.75), -1.0, 3.0);
    let (doc, _, row) = fixture::row(doc, hole, ("k", 2), TargetAxis::X, 1.0);
    let member = BodyRead::indexed(row, vec![Formula::count(0)]);
    let refused = insert_refused(
        &doc,
        Node::Subtract {
            from: read(plate),
            tool: Bodies::Family(member.clone()),
        },
    );
    assert!(
        matches!(
            refused,
            EditError::IndexedRead {
                fault: editor_core::InputFault::IndexedFamily,
                ..
            }
        ),
        "insert: {refused:?}"
    );
    let (doc, cut) = insert(
        doc,
        Node::Subtract {
            from: read(plate),
            tool: Bodies::Family(read(row)),
        },
    );
    let refused = doc
        .apply(
            &DocEdit::SetMembers {
                node: cut,
                members: Bodies::Family(member),
            },
            Tol::witness(),
        )
        .expect_err("an indexed family argument refuses at SetMembers");
    assert!(
        matches!(
            refused,
            EditError::IndexedRead {
                fault: editor_core::InputFault::IndexedFamily,
                ..
            }
        ),
        "set_members: {refused:?}"
    );
}

/// **One tool, however spelled, is the same cut**: `Spelled([row])`,
/// `Family(row)` and the two members written out evaluate to one
/// plate of one volume; and a family of ONE member at the tool seat
/// (the one-tool pair path) names as its member read `row[0]` does,
/// up to the member segment. Red if `Family` of one member and
/// `Spelled([row])` part ways in the result.
#[test]
fn a_family_of_one_at_the_tools_cuts_as_its_member() {
    let doc = ProfileDoc::empty_derived("review-subtract-family-of-one", Tol::witness());
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 1.0), 0.2);
    let (doc, hole) = block_on(doc, plate, (0.25, 0.5), (0.25, 0.75), -1.0, 3.0);
    let (doc, _, row) = fixture::row(doc, hole, ("k", 1), TargetAxis::X, 1.0);
    let cut = |doc, tool| {
        insert(
            doc,
            Node::Subtract {
                from: read(plate),
                tool,
            },
        )
    };
    let (doc, spelled) = cut(doc, Bodies::one(read(row)));
    let (doc, whole) = cut(doc, Bodies::Family(read(row)));
    let (doc, member) = cut(
        doc,
        Bodies::one(BodyRead::indexed(row, vec![Formula::count(0)])),
    );
    let ev = run(&doc);
    let hole = 0.25 * 0.5 * 0.2;
    for id in [spelled, whole, member] {
        let v = volume(&ev, id);
        assert!((v - (0.6 - hole)).abs() < 1e-9, "{id:?}: {v}");
    }
    assert_eq!(names(&ev, spelled), names(&ev, whole));
    assert_eq!(
        names(&ev, spelled).len(),
        names(&ev, member).len(),
        "a family of one and its member name as many entities"
    );
}

/// **A tool list emptied by `SetMembers` is `from` as it stands** (the
/// `[]` arm of `wire_subtract`). Red if it refuses or builds empty.
#[test]
fn a_subtract_whose_tools_are_set_to_none_is_its_from() {
    let doc = ProfileDoc::empty_derived("review-subtract-no-tools", Tol::witness());
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 1.0), 0.2);
    let (doc, hole) = block_on(doc, plate, (0.25, 0.5), (0.25, 0.75), -1.0, 3.0);
    let (doc, cut) = insert(
        doc,
        Node::Subtract {
            from: read(plate),
            tool: Bodies::one(read(hole)),
        },
    );
    let (doc, _) = step(
        doc,
        DocEdit::SetMembers {
            node: cut,
            members: Bodies::Spelled(Vec::new()),
        },
    );
    let ev = run(&doc);
    let v = volume(&ev, cut);
    assert!((v - 0.6).abs() < 1e-9, "the plate whole: {v}");
}

/// **A plain body at `SetMembers`' family form refuses by kind at
/// `Cuts`** — what Python's `DocEdit.set_members(cut, b)` writes for a
/// bare operand (`MembersArg::Family` → `Bodies::Family`), while
/// `Node.subtract(a, b)` writes `Bodies::one(b)` for the same spelling
/// and is accepted.
#[test]
fn set_members_with_a_bare_body_at_a_subtract_refuses_by_kind() {
    let doc = ProfileDoc::empty_derived("review-subtract-bare-set-members", Tol::witness());
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 1.0), 0.2);
    let (doc, hole) = block_on(doc, plate, (0.25, 0.5), (0.25, 0.75), -1.0, 3.0);
    let (doc, other) = block_on(doc, plate, (1.25, 1.5), (0.25, 0.75), -1.0, 3.0);
    let (doc, cut) = insert(
        doc,
        Node::Subtract {
            from: read(plate),
            tool: Bodies::one(read(hole)),
        },
    );
    let got = doc.apply(
        &DocEdit::SetMembers {
            node: cut,
            members: Bodies::Family(read(other)),
        },
        Tol::witness(),
    );
    assert!(
        matches!(
            got,
            Err(EditError::SlotVarKind {
                slot: editor_core::SlotId::Operand(editor_core::OperandSlot::Cuts),
                ..
            })
        ),
        "{:?}",
        got.err()
    );
}
