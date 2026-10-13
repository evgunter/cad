//! **A union's fold step records the face–face touch of a given
//! tangency** (`crates/topo/README.md` C7, D1): a rod resting on a
//! plate's side face along a ruling through both faces' interiors, its
//! vertices off the ruling, and a block apart from both. The union
//! judges the rod and the plate pairwise and hands the fold step that
//! joins them that tangency; the step reads the line against the two
//! faces as a pair boolean does and records the face–face touch, citing
//! the pass's tangency row. Every member order: the touching pair joined
//! by the first step, by the second (the block folded in first), and the
//! rod first.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block, failure, run};
use crate::fixture::{self, insert, len, union_over};
use editor_core::{
    BooleanValue, ExtrudeSide, LoopProgram, Node, ProfileDoc, ProfileProgram, RecipeNodeId,
    ValuePayload,
};
use geom::SurfaceKind;
use geom_core::Tol;

/// A rod of radius 0.5 about (0, 2.5) from z = 0 to 1, the plate's
/// height, its rim vertices at (±0.5, 2.5): tangent to the plate's
/// y = 2 face along x = 0, its cap circles touching the plate's top and
/// bottom edges at the line's ends.
fn rod(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId) {
    let (doc, plane) = insert(
        doc,
        fixture::frame([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
    );
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            frame: plane.into(),
            loops: vec![LoopProgram::circle_split(0.0, 2.5, 0.5, 2, 0.0).expect("a finite circle")],
            ids: Vec::new(),
        }),
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    )
}

#[test]
fn a_fold_step_records_a_given_tangency_through_both_interiors_in_every_order() {
    let doc = ProfileDoc::empty_derived("fold-ff", Tol::witness());
    let (doc, plate) = block(doc, (-2.0, 2.0), (-2.0, 2.0), 0.0, 1.0);
    let (doc, rod) = rod(doc);
    let (doc, apart) = block(doc, (5.0, 6.0), (5.0, 6.0), 0.0, 1.0);
    for (what, members) in [
        ("plate, rod, block", [plate, rod, apart]),
        ("block, plate, rod", [apart, plate, rod]),
        ("rod, block, plate", [rod, apart, plate]),
    ] {
        let (doc, u) = union_over(doc.clone(), &members);
        let ev = run(&doc);
        if let Some(e) = failure(&ev, u) {
            panic!("{what}: refused: {e:?}");
        }
        let value = ev.value(u).expect("the union evaluated");
        let ValuePayload::Boolean(BooleanValue::Body { body, contacts, .. }) = &value.payload
        else {
            panic!("{what}: not a boolean body");
        };
        let kind = |f: topo::FaceKey| {
            let face = body.get_face(f).unwrap();
            body.get_surface(face.surface).unwrap().kind()
        };
        let ff: Vec<_> = contacts
            .rows()
            .iter()
            .filter_map(|c| match c.record.kind() {
                topo::ContactKind::FaceFace(x, y) => Some((x, y, c.cites.clone())),
                _ => None,
            })
            .collect();
        assert_eq!(
            ff.len(),
            1,
            "{what}: one face–face record, the plate's face and the rod's wall: {ff:?}"
        );
        let (x, y, cites) = &ff[0];
        let mut kinds = [kind(*x), kind(*y)];
        kinds.sort_by_key(|k| *k != SurfaceKind::Plane);
        assert_eq!(
            kinds,
            [SurfaceKind::Plane, SurfaceKind::Cylinder],
            "{what}: the face–face record's faces"
        );
        // The record cites the pass's tangency row: a row of the node's
        // own list, decided by the tangency witness between a face of
        // the plate's read and a face of the rod's.
        let reads = [doc.output(plate, 0).unwrap(), doc.output(rod, 0).unwrap()];
        let cited: Vec<_> = cites.iter().collect();
        assert_eq!(
            cited.len(),
            1,
            "{what}: one decision backs the touch: {cited:?}"
        );
        let topo::Backing::Decided(row) = cited[0] else {
            panic!("{what}: the touch cites a carried record: {cited:?}");
        };
        let row = &value.coincidences[row as usize];
        assert_eq!(
            (row.relation, row.site),
            (
                topo::Relation::SurfaceTangent,
                topo::DecisionSite::TangentWitness
            ),
            "{what}: the cited row is a tangency: {row:?}"
        );
        let mut inputs: Vec<_> = row
            .cells
            .iter()
            .map(|c| match c {
                editor_core::coincide::NamedCell::Entity { input, .. } => *input,
                other => panic!("{what}: a cited cell off the members: {other:?}"),
            })
            .collect();
        inputs.sort();
        let mut want = reads.to_vec();
        want.sort();
        assert_eq!(
            inputs, want,
            "{what}: the cited row is the pass's, in the members' reads"
        );
    }
}
