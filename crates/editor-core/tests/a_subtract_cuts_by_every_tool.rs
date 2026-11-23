//! **A subtract's tool is a `Bodies` seat** (DESIGN `[repetition]`,
//! REFERENCES DM4): `subtract(a, holes)` cuts `a` by every member, as
//! the fold `a − t₁ − … − tₙ`, which is order-free in the tools and
//! fuses none of them with another.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block, block_on};
use crate::fixture::{self, insert};
use editor_core::{
    Bodies, BodyRead, BooleanValue, Evaluation, Formula, Node, Operand, ProfileDoc, RecipeNodeId,
    RoleSeg, TargetAxis, ValuePayload,
};
use geom_core::Tol;

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    fixture::run(doc, &Default::default())
}

/// The plate, 3 × 1 × 0.2.
const PLATE: f64 = 3.0 * 0.2;
/// What one hole takes out of the plate: 0.25 × 0.5 through its 0.2.
const HOLE: f64 = 0.25 * 0.5 * 0.2;

fn volume(b: &topo::Body<f64>) -> f64 {
    let measured = topo::test_support::finished("the measured body", b.clone(), Tol::witness());
    topo::test_support::pinned_volume("the measured body", &measured, Tol::witness()).volume
}

/// The one body `id` holds, or a panic naming what it holds instead.
fn body(ev: &Evaluation<f64>, id: RecipeNodeId) -> &topo::Body<f64> {
    match ev.value(id).map(|v| &v.payload) {
        Some(ValuePayload::Boolean(BooleanValue::Body { body, .. })) => body.body(),
        _ => panic!("{id:?} is not one body: {:?}", ev.result(id)),
    }
}

/// The plate, and a 0.25 × 0.5 block through it at `x` in the plate's
/// space.
fn plate_and_hole(x: f64) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("a-subtract-cuts-by-every-tool", Tol::witness());
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 1.0), 0.2);
    let (doc, hole) = block_on(doc, plate, (x, x + 0.25), (0.25, 0.75), -1.0, 3.0);
    (doc, plate, hole)
}

/// **A plate cut by a row of two holes read whole is one plate with two
/// holes**, however the row is spelled at the tool seat: the family
/// spelled as one read, the family read whole, or its two members
/// written out. Each is one solid of the plate's volume less both holes,
/// and each face is named by the one read it came through: the plate's,
/// or the row's. The row read whole and spelled as one read name alike.
/// Red if a whole family at the tool makes one cut per member.
#[test]
fn a_plate_cut_by_a_row_read_whole_is_one_plate_with_every_hole() {
    let (doc, plate, hole) = plate_and_hole(0.25);
    let (doc, _, row) = fixture::row(doc, hole, ("k", 2), TargetAxis::X, 1.0);
    let cut = |doc, tool| {
        insert(
            doc,
            Node::Subtract {
                from: BodyRead::plain(Operand::Node(plate)),
                tool,
            },
        )
    };
    let member = |i| BodyRead::indexed(row, vec![Formula::count(i)]);
    let (doc, spelled) = cut(doc, Bodies::one(BodyRead::plain(Operand::Node(row))));
    let (doc, whole) = cut(doc, Bodies::Family(BodyRead::plain(Operand::Node(row))));
    let (doc, written) = cut(doc, Bodies::Spelled(vec![member(0), member(1)]));
    let ev = run(&doc);
    let plate_read = doc.read_of_node(plate).unwrap();
    let row_read = doc.read_of_node(row).unwrap();
    for (what, id) in [
        ("spelled as one read", spelled),
        ("read whole", whole),
        ("written out", written),
    ] {
        let cut = body(&ev, id);
        assert_eq!(cut.solids().count(), 1, "the row {what}: one plate");
        let v = volume(cut);
        assert!(
            (v - (PLATE - 2.0 * HOLE)).abs() < 1e-9,
            "the row {what}: the plate less both holes, got {v}"
        );
        for (name, _) in ev.table_at(id).unwrap().iter() {
            if name.kind != editor_core::EntityKind::Face {
                continue;
            }
            assert!(
                matches!(
                    name.path.first(),
                    Some(RoleSeg::From { read, .. }) if *read == plate_read || *read == row_read
                ),
                "the row {what}: a face named by the read it came through, got {name:?}"
            );
        }
    }
    // Each name as spelled, the node's own id (which the names it
    // mints embed) said as `SELF`.
    let names = |id: RecipeNodeId| {
        let own = format!("{id:?}");
        let mut spelled: Vec<String> = ev
            .table_at(id)
            .unwrap()
            .iter()
            .map(|(n, _)| format!("{:?} {:?}", n.kind, n.path).replace(&own, "SELF"))
            .collect();
        spelled.sort();
        spelled
    };
    assert_eq!(
        names(spelled),
        names(whole),
        "the row read whole and spelled as one read name alike"
    );
}

/// **Two tools that overlap each other still cut**: the fold cuts each
/// where it stands, fusing neither with the other, so the plate loses
/// their union once, in either order. Red if the fold refuses or counts
/// the overlap twice.
#[test]
fn a_tool_list_whose_members_overlap_each_other_still_cuts() {
    let (doc, plate, first) = plate_and_hole(0.25);
    let (doc, wide) = block_on(doc, plate, (0.25, 1.0), (0.25, 0.75), -1.0, 3.0);
    let (doc, second) = block_on(doc, plate, (0.75, 1.5), (0.3, 0.7), -1.0, 3.0);
    let read = |id| BodyRead::plain(Operand::Node(id));
    let (doc, forward) = insert(
        doc,
        Node::Subtract {
            from: read(plate),
            tool: Bodies::Spelled(vec![read(wide), read(second)]),
        },
    );
    let (doc, backward) = insert(
        doc,
        Node::Subtract {
            from: read(plate),
            tool: Bodies::Spelled(vec![read(second), read(wide)]),
        },
    );
    let (doc, three) = insert(
        doc,
        Node::Subtract {
            from: read(plate),
            tool: Bodies::Spelled(vec![read(wide), read(second), read(first)]),
        },
    );
    let ev = run(&doc);
    // 0.75 × 0.5 and 0.75 × 0.4, sharing 0.25 × 0.4, through 0.2.
    let removed = (0.75 * 0.5 + 0.75 * 0.4 - 0.25 * 0.4) * 0.2;
    for (what, id) in [
        ("in list order", forward),
        ("reversed", backward),
        ("with a third inside the first", three),
    ] {
        let cut = body(&ev, id);
        assert_eq!(cut.solids().count(), 1, "{what}: one plate");
        let v = volume(cut);
        assert!(
            (v - (PLATE - removed)).abs() < 1e-9,
            "{what}: the plate less the tools' union, got {v}"
        );
    }
}
