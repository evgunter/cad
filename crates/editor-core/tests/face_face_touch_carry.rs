//! **A body's curve-touch records ride the wire into the next op.** A
//! rod resting on a plate's side face along a ruling through both
//! faces' interiors: the union is two solids carrying the face–face
//! record of the plate's face and the rod's wall, and an edge–face
//! record for each cap circle touching the plate's face at one point
//! (`crates/topo/README.md` C7). The wire hands every record to the
//! next boolean on the body (`ContactRecords::carried`), and the one
//! carry reads each through the census's record confirm, which has no
//! lane for a circle touching a plane at a point
//! (`work/restread/an-edge-face-record-off-a-line-and-a-plane-has-no-curve-touch-lane.md`):
//! so a later op refuses `CarryRefused { Unread }` on that record even
//! where it touches neither of its cells
//! (`work/contacthold/a-carried-edge-face-record-the-census-cannot-read-refuses-every-later-op.md`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block, failure, run};
use crate::fixture::{ang, frame, insert, len, len2};
use editor_core::{
    Bodies, BooleanValue, ExtrudeSide, LoopProgram, Node, NodeErrorKind, ProfileDoc,
    ProfileProgram, ValuePayload,
};
use geom_core::Tol;
use topo::ContactKind;

#[test]
fn a_later_op_on_a_rod_resting_on_a_plate_refuses_on_its_unread_cap_record() {
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
            matches!(
                failure(&ev, node),
                Some(NodeErrorKind::Boolean(topo::BooleanError::CarryRefused {
                    refusal: topo::CarryRefusal::Unread {
                        cells: [topo::Cell::Edge(_), topo::Cell::Face(_)],
                        ..
                    },
                    ..
                }))
            ),
            "{what}: the carried cap circle's edge–face record is unread: {:?}",
            failure(&ev, node)
        );
    }
}
