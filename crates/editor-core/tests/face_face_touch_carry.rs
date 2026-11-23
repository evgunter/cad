//! **A body's curve-touch records ride the wire into the next op.** A
//! rod resting on a plate's side face along a ruling through both
//! faces' interiors: the union is two solids carrying the face–face
//! record of the plate's face and the rod's wall, and an edge–face
//! record for each cap circle touching the plate's face at one point
//! (`crates/topo/README.md` C7). The wire hands every record to the
//! next boolean on the body (`ContactRecords::carried`), and the one
//! carry lands a record whose cells the op kept on them by lineage,
//! unread: a later op that touches neither cell carries all three.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block, block_on, failure, run};
use crate::fixture::{insert, len, len2, slid};
use editor_core::{
    Bodies, BooleanValue, Dimension, Formula, LoopProgram, Node, ProfileDoc, ProfileProgram,
    ValuePayload,
};
use geom_core::Tol;
use topo::ContactKind;

#[test]
fn a_later_op_on_a_rod_resting_on_a_plate_carries_its_cap_records() {
    let doc = ProfileDoc::empty_derived("ff-touch-carry", Tol::witness());
    let (doc, plate) = block(doc, (-2.0, 2.0), (-2.0, 2.0), 1.0);
    let (doc, disc) = insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::CircleSplit {
                centre: len2([0.0, 2.5]),
                radius: len(0.5),
                n: 2,
                phase: Formula::zero(Dimension::Angle),
            }],
            ids: Vec::new(),
            on: None,
        }),
    );
    let (doc, rod) = insert(
        doc,
        Node::Extrude {
            profile: disc.into(),
            distance: len(0.5),
        },
    );
    let (doc, rod) = slid(doc, plate, rod, [0.0, 0.0, 0.25]);
    let (doc, rested) = insert(
        doc,
        Node::Union {
            members: Bodies::Spelled(vec![plate.into(), rod.into()]),
        },
    );
    let (doc, far) = block_on(doc, plate, (10.0, 11.0), (10.0, 11.0), 0.0, 1.0);
    let (doc, hole) = block_on(doc, plate, (-1.0, -0.5), (-1.0, -0.5), -1.0, 3.0);
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
    let rested_value = ev.value(rested).expect("plate ∪ rod evaluates");
    let ValuePayload::Boolean(BooleanValue::Body { .. }) = &rested_value.payload else {
        panic!("plate ∪ rod: a boolean body")
    };
    let contacts = &rested_value.carry.contacts;
    let mut kinds: Vec<_> = contacts
        .kinds()
        .map(|k| match k {
            ContactKind::EdgeFace(..) => "edge–face",
            ContactKind::FaceFace(..) => "face–face",
            _ => "other",
        })
        .collect();
    kinds.sort_unstable();
    assert_eq!(
        kinds,
        ["edge–face", "edge–face", "face–face"],
        "plate ∪ rod: the wall's face–face record and the two cap circles' edge–face records"
    );
    for (what, node) in [
        ("∪ a far block", joined),
        ("∖ a hole in the plate", drilled),
    ] {
        assert!(
            failure(&ev, node).is_none(),
            "{what}: answers: {:?}",
            failure(&ev, node)
        );
        let value = ev.value(node).expect("the op evaluates");
        let ValuePayload::Boolean(BooleanValue::Body { .. }) = &value.payload else {
            panic!("{what}: a boolean body")
        };
        let carried = &value.carry.contacts;
        assert_eq!(
            (carried.edge_face().count(), carried.face_face().count()),
            (2, 1),
            "{what}: the two cap records and the wall's record ride through"
        );
    }
}
