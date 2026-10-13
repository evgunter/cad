//! **A union's fold step records the face–face touch of a given
//! tangency** (`crates/topo/README.md` C7, D1): a rod resting on a
//! plate's side face along a ruling through both faces' interiors, its
//! vertices off the ruling. The union judges the rod and the plate
//! pairwise and hands the fold step that joins them that tangency with
//! the line it was decided along; the step places the line against its
//! own pieces' faces and records the face–face touch, citing the pass's
//! tangency row.
//!
//! The rows, each in several member orders:
//! - a block apart from both;
//! - a block resting flush on the plate's face beside the line, and one
//!   crossing that face, folded in before the rod, so the step reads
//!   the line against a piece of the plate's face;
//! - a slab crossing plate and rod obliquely, before or after the rod;
//! - a neighbour flush with the plate along x, 296 long, whose face
//!   merges with the plate's before the rod is folded, the rod leaning
//!   1e-10 rad toward the face: tangent within the band at the pass's
//!   extent, not at the merged face's, which the step must not re-read.
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
    leaning_rod(doc, 0.0)
}

/// [`rod`], its sketch plane turned by `lean` about x, so its axis
/// leans toward the plate's face.
fn leaning_rod(doc: ProfileDoc, lean: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, plane) = insert(
        doc,
        fixture::frame(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, lean.cos(), lean.sin()],
        ),
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

const PI: f64 = core::f64::consts::PI;

/// The plate (−2..2)² × (0..1), and its document.
fn plate() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("fold-ff", Tol::witness());
    block(doc, (-2.0, 2.0), (-2.0, 2.0), 0.0, 1.0)
}

/// Asserts the union of `members` answers with `solids` solids of
/// `volume`, and carries one face–face record, the plate's face and the
/// rod's wall, citing the pass's one tangency row: a row of the node's
/// own list, decided by the tangency witness between a face of
/// `plate`'s read and a face of `rod`'s.
#[allow(clippy::too_many_arguments)]
fn touches(
    doc: &ProfileDoc,
    what: &str,
    members: &[RecipeNodeId],
    (plate, rod): (RecipeNodeId, RecipeNodeId),
    solids: usize,
    volume: f64,
) {
    let (doc, u) = union_over(doc.clone(), members);
    let ev = run(&doc);
    if let Some(e) = failure(&ev, u) {
        panic!("{what}: refused: {e:?}");
    }
    let value = ev.value(u).expect("the union evaluated");
    let ValuePayload::Boolean(BooleanValue::Body { body, contacts, .. }) = &value.payload else {
        panic!("{what}: not a boolean body");
    };
    assert_eq!(body.solids().count(), solids, "{what}: solids");
    let v = topo::mass_properties(body, Tol::witness()).unwrap().volume;
    assert!(
        (v - volume).abs() < 1e-9,
        "{what}: volume {v}, want {volume}"
    );
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

/// The plate's volume and the rod's.
const PLATE_AND_ROD: f64 = 16.0 + PI / 4.0;

#[test]
fn a_fold_step_records_a_given_tangency_through_both_interiors_in_every_order() {
    let (doc, plate) = plate();
    let (doc, rod) = rod(doc);
    let (doc, apart) = block(doc, (5.0, 6.0), (5.0, 6.0), 0.0, 1.0);
    for (what, members) in [
        ("plate, rod, block", [plate, rod, apart]),
        ("block, plate, rod", [apart, plate, rod]),
        ("rod, block, plate", [rod, apart, plate]),
    ] {
        touches(&doc, what, &members, (plate, rod), 3, PLATE_AND_ROD + 1.0);
    }
}

/// A block flush on the plate's y = 2 face beside the line (x 1..1.8),
/// and one crossing it (x 1..1.5, through the plate's height): folded
/// in before the rod, the step reads the line against a piece of the
/// plate's face; after it, the record is carried onto one.
#[test]
fn a_piece_of_the_plates_face_still_holds_the_line() {
    let (doc, plate) = plate();
    let (doc, rod) = rod(doc);
    let (doc, flush) = block(doc, (1.0, 1.8), (2.0, 2.6), 0.0, 1.0);
    let (doc, across) = block(doc, (1.0, 1.5), (1.5, 2.5), -0.5, 2.0);
    let flush_v = PLATE_AND_ROD + 0.8 * 0.6;
    let across_v = PLATE_AND_ROD + 0.5 * 2.0 - 0.5 * 0.5;
    for (what, members, volume) in [
        ("plate, flush, rod", vec![plate, flush, rod], flush_v),
        ("flush, plate, rod", vec![flush, plate, rod], flush_v),
        ("rod, flush, plate", vec![rod, flush, plate], flush_v),
        ("plate, across, rod", vec![plate, across, rod], across_v),
        ("across, plate, rod", vec![across, plate, rod], across_v),
        ("across, rod, plate", vec![across, rod, plate], across_v),
        ("rod, plate, across", vec![rod, plate, across], across_v),
    ] {
        touches(&doc, what, &members, (plate, rod), 2, volume);
    }
}

/// A slab 0.2 thick crossing the plate and the rod obliquely over the
/// line's upper part, folded in after the rod (the record carried onto
/// the pieces, the pair of pieces meeting only at the cut read apart)
/// or before it (the step reads the line against the pieces).
#[test]
fn a_slab_across_the_line_leaves_the_touch_on_its_pieces() {
    let (doc, plate) = plate();
    let (doc, rod) = rod(doc);
    let (doc, prof) = crate::fixture::on_frame(
        doc,
        [-1.0, 2.0, 0.7],
        [1.0, 0.0, 0.0],
        [0.0, 0.8, 0.6],
        vec![vec![(0.0, -1.0), (2.0, -1.0), (2.0, 1.5), (0.0, 1.5)]],
    );
    let (doc, slab) = insert(
        doc,
        Node::Extrude {
            profile: prof.into(),
            distance: len(0.2),
            side: ExtrudeSide::Along,
        },
    );
    // The closed form is not simple; the orders that answer agree on it.
    let volume = 17.319_065_538_33;
    for (what, members) in [
        ("plate, rod, slab", [plate, rod, slab]),
        ("rod, slab, plate", [rod, slab, plate]),
        ("slab, rod, plate", [slab, rod, plate]),
    ] {
        let (d, u) = union_over(doc.clone(), &members);
        let ev = run(&d);
        if let Some(e) = failure(&ev, u) {
            panic!("{what}: refused: {e:?}");
        }
        let v = topo::mass_properties(crate::corpus::body_of(&ev, u), Tol::witness())
            .unwrap()
            .volume;
        assert!((v - volume).abs() < 1e-8, "{what}: volume {v}");
        touches(&doc, what, &members, (plate, rod), 1, v);
    }
}

/// A rod leaning 1e-10 rad toward the plate's face, tangent within the
/// band over the pair's own extent, and a neighbour (2..300) × (−2..2)
/// × (0..1) whose y = 2 face merges with the plate's before the rod is
/// folded. Read again over the merged face's extent the carriers decide
/// otherwise; the step places the pass's line and decides nothing of
/// them, so the union answers as the pair does.
#[test]
fn a_merged_face_far_longer_than_the_pair_holds_the_pass_s_line() {
    let (doc, plate) = plate();
    let (doc, rod) = leaning_rod(doc, 1e-10);
    let (doc, long) = block(doc, (2.0, 300.0), (-2.0, 2.0), 0.0, 1.0);
    for (what, members) in [
        ("plate, long, rod", [plate, long, rod]),
        ("long, plate, rod", [long, plate, rod]),
    ] {
        touches(
            &doc,
            what,
            &members,
            (plate, rod),
            2,
            1192.0 + PLATE_AND_ROD,
        );
    }
}
