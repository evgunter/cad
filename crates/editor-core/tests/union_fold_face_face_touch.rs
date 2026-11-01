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
//!   a tenth of ε rad toward the face: tangent within the band at the pass's
//!   extent, not at the merged face's, which the step must not re-read.
//! - a rod whose carrier's origin stands 300 or 3000 behind the touch,
//!   leaning a tenth of ε: the step must place the pass's own line, not
//!   one rebuilt from the carrier;
//! - two flush plates whose faces merge, the rod touching only the
//!   second: the merged face is named after the first, so the record
//!   cites the pass's row through lineage alone.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block, block_on, failure, run};
use crate::fixture::{self, against, axes_at, insert, len, slid, union_over};
use editor_core::{
    BooleanValue, LoopProgram, Node, ProfileDoc, ProfileProgram, RecipeNodeId, ValuePayload,
};
use geom::SurfaceKind;
use geom_core::Tol;

/// A rod of radius 0.5 about `(cx, 2.5)` on its own xy, `length` up its
/// own z: a construction.
fn upright_rod(doc: ProfileDoc, cx: f64, length: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::circle_split(cx, 2.5, 0.5, 2, 0.0).expect("a finite circle")],
            ids: Vec::new(),
            on: None,
        }),
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(length),
        },
    )
}

/// A rod of radius 0.5 about (0, 2.5) from z = 0 to 1, the plate's
/// height, its rim vertices at (±0.5, 2.5): tangent to the plate's
/// y = 2 face along x = 0, its cap circles touching the plate's top and
/// bottom edges at the line's ends. Its copy stands in `plate`'s space.
fn rod(doc: ProfileDoc, plate: RecipeNodeId) -> (ProfileDoc, RecipeNodeId) {
    leaning_rod(doc, plate, 0.0)
}

/// [`rod`], its copy turned by `lean` about x, so its axis leans toward
/// the plate's face.
fn leaning_rod(doc: ProfileDoc, plate: RecipeNodeId, lean: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, body) = upright_rod(doc, 0.0, 1.0);
    against(
        doc,
        plate,
        body,
        &axes_at(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, lean.cos(), lean.sin()],
        ),
    )
}

/// The slab 0.2 thick across the line: `corners` on its own xy,
/// extruded 0.2, its copy standing in `plate`'s space at `origin` with
/// its own y along (0, 0.8, 0.6).
fn slab(
    doc: ProfileDoc,
    plate: RecipeNodeId,
    origin: [f64; 3],
    corners: Vec<(f64, f64)>,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, prof) = insert(doc, Node::Profile(fixture::desc(vec![corners])));
    let (doc, body) = insert(
        doc,
        Node::Extrude {
            profile: prof.into(),
            distance: len(0.2),
        },
    );
    against(
        doc,
        plate,
        body,
        &axes_at(origin, [1.0, 0.0, 0.0], [0.0, 0.8, 0.6]),
    )
}

const PI: f64 = core::f64::consts::PI;

/// The plate (−2..2)² × (0..1), and its document.
fn plate() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("fold-ff", Tol::witness());
    block(doc, (-2.0, 2.0), (-2.0, 2.0), 1.0)
}

/// Asserts the union of `members` answers with `solids` solids of
/// `volume`, and carries `records` face–face records, each a face of
/// the plate and a piece of the rod's wall, each citing the pass's one
/// tangency row: a row of the node's own list, decided by the tangency
/// witness between a face of `plate`'s read and a face of `rod`'s.
#[allow(clippy::too_many_arguments)]
fn touches(
    doc: &ProfileDoc,
    what: &str,
    members: &[RecipeNodeId],
    (plate, rod): (RecipeNodeId, RecipeNodeId),
    solids: usize,
    volume: f64,
    records: usize,
) {
    let (doc, u) = union_over(doc.clone(), members);
    let ev = run(&doc);
    if let Some(e) = failure(&ev, u) {
        panic!("{what}: refused: {e:?}");
    }
    let Some(value) = ev.value(u) else {
        let failed: Vec<_> = members.iter().filter_map(|&m| failure(&ev, m)).collect();
        panic!("{what}: the union did not evaluate; a member refused: {failed:?}");
    };
    let ValuePayload::Boolean(BooleanValue::Body { body, .. }) = &value.payload else {
        panic!("{what}: not a boolean body");
    };
    let contacts = &value.carry.contacts;
    assert_eq!(body.body().solids().count(), solids, "{what}: solids");
    let v = topo::test_support::pinned_volume(
        "the measured body",
        &topo::test_support::finished(
            "the measured body",
            topo::Body::clone(body.body()),
            Tol::witness(),
        ),
        Tol::witness(),
    )
    .volume;
    assert!(
        (v - volume).abs() < 1e-9,
        "{what}: volume {v}, want {volume}"
    );
    let kind = |f: topo::FaceKey| {
        let face = body.body().get_face(f).unwrap();
        body.body().get_surface(face.surface).unwrap().kind()
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
        records,
        "{what}: the face–face records of the plate's face and the rod's wall: {ff:?}"
    );
    let reads = [doc.output(plate, 0).unwrap(), doc.output(rod, 0).unwrap()];
    for (x, y, cites) in &ff {
        let mut kinds = [kind(*x), kind(*y)];
        kinds.sort_by_key(|k| *k != SurfaceKind::Plane);
        assert_eq!(
            kinds,
            [SurfaceKind::Plane, SurfaceKind::Cylinder],
            "{what}: a face–face record's faces"
        );
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

/// The plate's volume and the rod's.
const PLATE_AND_ROD: f64 = 16.0 + PI / 4.0;

#[test]
fn a_fold_step_records_a_given_tangency_through_both_interiors_in_every_order() {
    let (doc, plate) = plate();
    let (doc, rod) = rod(doc, plate);
    let (doc, apart) = block_on(doc, plate, (5.0, 6.0), (5.0, 6.0), 0.0, 1.0);
    for (what, members) in [
        ("plate, rod, block", [plate, rod, apart]),
        ("block, plate, rod", [apart, plate, rod]),
        ("rod, block, plate", [rod, apart, plate]),
    ] {
        touches(
            &doc,
            what,
            &members,
            (plate, rod),
            3,
            PLATE_AND_ROD + 1.0,
            1,
        );
    }
}

/// A block flush on the plate's y = 2 face beside the line (x 1..1.8),
/// and one crossing it (x 1..1.5, through the plate's height): folded
/// in before the rod, the step reads the line against a piece of the
/// plate's face; after it, the record is carried onto one.
#[test]
fn a_piece_of_the_plates_face_still_holds_the_line() {
    let (doc, plate) = plate();
    let (doc, rod) = rod(doc, plate);
    let (doc, flush) = block_on(doc, plate, (1.0, 1.8), (2.0, 2.6), 0.0, 1.0);
    let (doc, across) = block_on(doc, plate, (1.0, 1.5), (1.5, 2.5), -0.5, 2.0);
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
        touches(&doc, what, &members, (plate, rod), 2, volume, 1);
    }
}

/// A slab 0.2 thick crossing the plate and the rod obliquely over the
/// line's upper part, folded in after the rod (the record carried onto
/// the pieces) or before it (the step places the line against them).
/// The slab cuts the rod's wall in two along the line and leaves the
/// plate's face one face around it: both pieces of the wall touch it,
/// two records in every order.
#[test]
fn a_slab_across_the_line_leaves_the_touch_on_its_pieces() {
    let (doc, plate) = plate();
    let (doc, rod) = rod(doc, plate);
    let (doc, slab) = slab(
        doc,
        plate,
        [-1.0, 2.0, 0.7],
        vec![(0.0, -1.0), (2.0, -1.0), (2.0, 1.5), (0.0, 1.5)],
    );
    // The closed form is not simple; the orders that answer agree on it,
    // within what the run's ε resolves.
    let volume = 17.319_065_538_33;
    let within = 1e-7_f64.max(100.0 * Tol::witness().eps());
    for (what, members, records) in [
        ("plate, rod, slab", [plate, rod, slab], 2),
        ("rod, slab, plate", [rod, slab, plate], 2),
        ("slab, rod, plate", [slab, rod, plate], 2),
    ] {
        let (d, u) = union_over(doc.clone(), &members);
        let ev = run(&d);
        if let Some(e) = failure(&ev, u) {
            panic!("{what}: refused: {e:?}");
        }
        let v = topo::test_support::pinned_volume(
            "the measured body",
            &topo::test_support::finished(
                "the measured body",
                topo::Body::clone(&(crate::corpus::body_of(&ev, u))),
                Tol::witness(),
            ),
            Tol::witness(),
        )
        .volume;
        assert!((v - volume).abs() < within, "{what}: volume {v}");
        touches(&doc, what, &members, (plate, rod), 1, v, records);
    }
}

/// A rod leaning a tenth of ε rad toward the plate's face, tangent within the
/// band over the pair's own extent, and a neighbour (2..300) × (−2..2)
/// × (0..1) whose y = 2 face merges with the plate's before the rod is
/// folded. Read again over the merged face's extent the carriers decide
/// otherwise; the step places the pass's line and decides nothing of
/// them, so the union answers as the pair does. Folded after the rod,
/// the neighbour merges the record's face, and the record lands on the
/// merged face by lineage.
#[test]
fn a_merged_face_far_longer_than_the_pair_holds_the_pass_s_line() {
    let (doc, plate) = plate();
    let (doc, rod) = leaning_rod(doc, plate, 0.1 * Tol::witness().eps());
    let (doc, long) = block_on(doc, plate, (2.0, 300.0), (-2.0, 2.0), 0.0, 1.0);
    for (what, members) in [
        ("plate, long, rod", [plate, long, rod]),
        ("long, plate, rod", [long, plate, rod]),
        ("rod, plate, long", [rod, plate, long]),
        ("plate, rod, long", [plate, rod, long]),
    ] {
        touches(
            &doc,
            what,
            &members,
            (plate, rod),
            2,
            1192.0 + PLATE_AND_ROD,
            1,
        );
    }
}

/// A rod of radius 0.5 about (0, 2.5), turned by `lean` about x,
/// occupying stations 0..1 along its axis, whose cylinder carrier's
/// origin stands `back` behind station 0: extruded from station −`back`
/// to 1 in its own coordinates, the part below station 0 cut away by a
/// block slid down its own z, and the result's copy standing in
/// `plate`'s space with station 0 on the plate's xy.
fn far_origin_rod(
    doc: ProfileDoc,
    plate: RecipeNodeId,
    lean: f64,
    back: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (s, c) = (lean.sin(), lean.cos());
    let (doc, long) = upright_rod(doc, 0.0, back + 1.0);
    let (doc, tool_profile) = insert(
        doc,
        Node::Profile(fixture::desc(vec![vec![
            (-10.0, -10.0),
            (10.0, -10.0),
            (10.0, 10.0),
            (-10.0, 10.0),
        ]])),
    );
    let (doc, tool) = insert(
        doc,
        Node::Extrude {
            profile: tool_profile.into(),
            distance: len(back + 5.0),
        },
    );
    let (doc, tool) = slid(doc, long, tool, [0.0, 0.0, -5.0]);
    let (doc, cut) = insert(
        doc,
        Node::Subtract {
            from: long.into(),
            tool: tool.into(),
        },
    );
    against(
        doc,
        plate,
        cut,
        &axes_at([0.0, back * s, -back * c], [1.0, 0.0, 0.0], [0.0, c, s]),
    )
}

/// The rod's carrier origin 300 and 3000 behind the touch, leaning a
/// tenth of ε: a line rebuilt from the carrier's origin would stand
/// tens of ε off the plate's face at the touch. Every order carries the
/// pair's one record.
#[test]
fn a_carrier_whose_origin_stands_far_off_hands_on_the_pass_s_line() {
    let eps = Tol::witness().eps();
    let lean = 0.1 * eps;
    // A rod 3000 long is no finished body at ε 1e-12 (its volume's
    // off-axis reading escalates), so the far row reads 1000 there.
    let far = if eps < 1e-10 { 1000.0 } else { 3000.0 };
    for back in [300.0, far] {
        let (doc, plate) = plate();
        let (doc, rod) = far_origin_rod(doc, plate, lean, back);
        let (doc, apart) = block_on(doc, plate, (5.0, 6.0), (5.0, 6.0), 0.0, 1.0);
        for (order, members) in [
            ("plate, rod, block", [plate, rod, apart]),
            ("block, plate, rod", [apart, plate, rod]),
            ("rod, block, plate", [rod, apart, plate]),
            ("plate, block, rod", [plate, apart, rod]),
        ] {
            touches(
                &doc,
                &format!("back {back}: {order}"),
                &members,
                (plate, rod),
                3,
                PLATE_AND_ROD + 1.0,
                1,
            );
        }
    }
}

/// Two flush plates, x −2..0 and 0..2, whose y = 2 faces merge, and the
/// rod about (0.25, 2.5), touching the second plate's face only.
#[test]
fn a_merged_face_cites_the_verdict_of_the_member_face_it_holds() {
    let doc = ProfileDoc::empty_derived("fold-ff-merged", Tol::witness());
    let (doc, first) = block(doc, (-2.0, 0.0), (-2.0, 2.0), 1.0);
    let (doc, second) = block_on(doc, first, (0.0, 2.0), (-2.0, 2.0), 0.0, 1.0);
    let (doc, rod) = upright_rod(doc, 0.25, 1.0);
    let (doc, rod) = slid(doc, first, rod, [0.0; 3]);
    for (order, members) in [
        ("first, second, rod", [first, second, rod]),
        ("second, first, rod", [second, first, rod]),
        ("first, rod, second", [first, rod, second]),
    ] {
        touches(&doc, order, &members, (second, rod), 2, PLATE_AND_ROD, 1);
    }
}

/// The slab of [`a_slab_across_the_line_leaves_the_touch_on_its_pieces`]
/// run across the plate's whole width (x −3..3): it cuts the plate's
/// face in two as well, so each piece of the face touches only the
/// piece of the wall on its side of the slab, and the cross pairs of
/// pieces, whose extents along the line are disjoint, are no touch.
/// The orders folding the slab into the plate before the rod refuse
/// `CurvedPierceUnsupported`, as on main.
#[test]
fn a_slab_across_the_whole_face_pairs_each_piece_with_its_own_side() {
    let (doc, plate) = plate();
    let (doc, rod) = rod(doc, plate);
    let (doc, slab) = slab(
        doc,
        plate,
        [-3.0, 2.0, 0.7],
        vec![(0.0, -1.0), (6.0, -1.0), (6.0, 1.5), (0.0, 1.5)],
    );
    let within = 1e-7_f64.max(100.0 * Tol::witness().eps());
    let mut volume = None;
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
        let v = topo::test_support::pinned_volume(
            "the measured body",
            &topo::test_support::finished(
                "the measured body",
                topo::Body::clone(&(crate::corpus::body_of(&ev, u))),
                Tol::witness(),
            ),
            Tol::witness(),
        )
        .volume;
        let want = *volume.get_or_insert(v);
        assert!(
            (v - want).abs() < within,
            "{what}: volume {v}, the first order's {want}"
        );
        touches(&doc, what, &members, (plate, rod), 1, v, 2);
    }
}
