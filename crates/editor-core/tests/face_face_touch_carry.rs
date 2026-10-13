//! **A body's curve-touch records ride the wire into the next op.** A
//! rod resting on a plate's side face along a ruling through both
//! faces' interiors: the union is two solids carrying the face–face
//! record of the plate's face and the rod's wall, and an edge–face
//! record for each cap circle touching the plate's face at one point
//! (`crates/topo/README.md` C7). The wire hands every record to the
//! next boolean on the body (`ContactRecords::carried`), and the one
//! carry lands a record whose cells the op kept by lineage, unread: so a
//! later op that touches neither cell answers with every record, none
//! re-read
//! (`work/contacthold/a-carried-edge-face-record-the-census-cannot-read-refuses-every-later-op.md`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block, failure, run};
use crate::fixture::{ang, frame, insert, len, len2};
use editor_core::{
    Bodies, BooleanValue, ExtrudeSide, LoopProgram, Node, ProfileDoc, ProfileProgram, ValuePayload,
};
use geom_core::Tol;
use topo::ContactKind;

#[test]
fn a_later_op_on_a_rod_resting_on_a_plate_carries_its_cap_records() {
    let doc = ProfileDoc::empty_derived("ff-touch-carry", Tol::witness());
    let (doc, plate) = block(doc, (-2.0, 2.0), (-2.0, 2.0), 0.0, 1.0);
    let (doc, plane) = insert(
        doc,
        frame([0.0, 0.0, 0.25], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
    );
    let (doc, disc) = insert(
        doc,
        Node::Profile(ProfileProgram {
            frame: plane.into(),
            loops: vec![LoopProgram::CircleSplit {
                centre: len2([0.0, 2.5]),
                radius: len(0.5),
                n: 2,
                phase: ang(0.0),
            }],
            ids: Vec::new(),
        }),
    );
    let (doc, rod) = insert(
        doc,
        Node::Extrude {
            profile: disc.into(),
            distance: len(0.5),
            side: ExtrudeSide::Along,
        },
    );
    let (doc, rested) = insert(
        doc,
        Node::Union {
            members: Bodies::Spelled(vec![plate.into(), rod.into()]),
        },
    );
    let (doc, far) = block(doc, (10.0, 11.0), (10.0, 11.0), 0.0, 1.0);
    let (doc, hole) = block(doc, (-1.0, -0.5), (-1.0, -0.5), -1.0, 3.0);
    let (doc, joined) = insert(
        doc,
        Node::Union {
            members: Bodies::Spelled(vec![rested.into(), far.into()]),
        },
    );
    let (doc, drilled) = insert(
        doc,
        Node::Subtract {
            from: rested.into(),
            tool: hole.into(),
        },
    );
    let ev = run(&doc);
    let ValuePayload::Boolean(BooleanValue::Body { contacts, .. }) =
        &ev.value(rested).expect("plate ∪ rod evaluates").payload
    else {
        panic!("plate ∪ rod: a boolean body")
    };
    let kinds = |contacts: &topo::ContactRecords| {
        let mut kinds: Vec<_> = contacts
            .kinds()
            .map(|k| match k {
                ContactKind::EdgeFace(..) => "edge–face",
                ContactKind::FaceFace(..) => "face–face",
                _ => "other",
            })
            .collect();
        kinds.sort_unstable();
        kinds
    };
    assert_eq!(
        kinds(contacts),
        ["edge–face", "edge–face", "face–face"],
        "plate ∪ rod: the wall's face–face record and the two cap circles' edge–face records"
    );
    for (what, node) in [
        ("∪ a far block", joined),
        ("∖ a hole in the plate", drilled),
    ] {
        let Some(ValuePayload::Boolean(BooleanValue::Body { contacts, .. })) =
            ev.value(node).map(|v| &v.payload)
        else {
            panic!("{what}: a boolean body: {:?}", failure(&ev, node))
        };
        assert_eq!(
            kinds(contacts),
            ["edge–face", "edge–face", "face–face"],
            "{what}: every record lands on the cells the op kept"
        );
    }
}
