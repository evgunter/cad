//! **A declaration licenses a coincidence, never a side.** Every
//! meeting of two solids is read by the one touch analysis, whether a
//! sweep found it undeclared, a record names it, or a declared face
//! pair backs it, and the pair clears only when each meeting reads
//! Rest. The rows here are the planar poses that cleared on their
//! records' word, their true-rest controls, and a true rest the
//! analysis cannot certify.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;
use geom_core::{Point3, Tol};
use std::collections::BTreeMap;
use topo::{
    Body, ContactRecords, EntityId, FaceKey, PatchContact, ValidationError, VertexKey, VfContact,
    VvContact, validate_pseudomanifold,
};

/// The I-profile, counterclockwise from `+z`: a head `y ∈ [1, 2]`, a
/// waist `x ∈ [0.5, 1.5], y ∈ [0, 1]` and a tail `y ∈ [-1, 0]`.
const I_PROFILE: [(f64, f64); 12] = [
    (0.0, -1.0),
    (2.0, -1.0),
    (2.0, 0.0),
    (1.5, 0.0),
    (1.5, 1.0),
    (2.0, 1.0),
    (2.0, 2.0),
    (0.0, 2.0),
    (0.0, 1.0),
    (0.5, 1.0),
    (0.5, 0.0),
    (0.0, 0.0),
];

fn assembly(parts: &[Body<f64>]) -> Body<f64> {
    let mut out = parts[0].clone();
    for part in &parts[1..] {
        topo::graft_disjoint(&mut out, part, Tol::witness()).unwrap();
    }
    out
}

/// The I-profile over `z ∈ [0, 2]`, shifted by `dy`.
fn i_profile(dy: f64) -> Body<f64> {
    let profile: Vec<(f64, f64)> = I_PROFILE.iter().map(|&(x, y)| (x, y + dy)).collect();
    common::prism_z::<f64>(&profile, 0.0, 2.0, Tol::witness()).body
}

/// The un-holed slab `[-5, 5] × [0, 1] × [-1, 3]`.
fn slab() -> Body<f64> {
    common::prism_z::<f64>(
        &[(-5.0, 0.0), (5.0, 0.0), (5.0, 1.0), (-5.0, 1.0)],
        -1.0,
        3.0,
        Tol::witness(),
    )
    .body
}

/// Every face's boundary vertices, with their points.
fn face_vertices(body: &Body<f64>) -> BTreeMap<FaceKey, Vec<(VertexKey, Point3<f64>)>> {
    let mut out: BTreeMap<FaceKey, Vec<(VertexKey, Point3<f64>)>> = BTreeMap::new();
    for (vk, v) in body.vertices() {
        let p = *body.get_point(v.point).unwrap();
        for f in body.faces_of_vertex(vk).unwrap() {
            out.entry(f).or_default().push((vk, p));
        }
    }
    out
}

/// The one face whose every boundary vertex satisfies `on`.
fn the_face(body: &Body<f64>, on: impl Fn(Point3<f64>) -> bool) -> FaceKey {
    let hits: Vec<FaceKey> = face_vertices(body)
        .into_iter()
        .filter(|(_, vs)| vs.iter().all(|&(_, p)| on(p)))
        .map(|(f, _)| f)
        .collect();
    assert_eq!(hits.len(), 1, "exactly one face: {hits:?}");
    hits[0]
}

/// Whether `a` and `b` are within a nanometre.
fn near(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

/// The I-profile through the slab: its head seated on the slab's
/// `y = 1` face and its tail on `y = 0`, its waist through the slab.
fn i_through_slab() -> Body<f64> {
    assembly(&[slab(), i_profile(0.0)])
}

/// The four seats of [`i_through_slab`]: `(the I's face, the slab's
/// face)` — the head's two undersides on the slab top, the tail's two
/// tops on the slab bottom.
fn i_seats(body: &Body<f64>) -> Vec<(FaceKey, FaceKey)> {
    let slab_top = the_face(body, |p| near(p.y, 1.0) && p.x.abs() > 4.0);
    let slab_bottom = the_face(body, |p| near(p.y, 0.0) && p.x.abs() > 4.0);
    let i_face = |y: f64, x: (f64, f64)| {
        the_face(body, |p| {
            near(p.y, y) && (near(p.x, x.0) || near(p.x, x.1)) && (0.0..=2.0).contains(&p.z)
        })
    };
    vec![
        (i_face(1.0, (0.0, 0.5)), slab_top),
        (i_face(1.0, (1.5, 2.0)), slab_top),
        (i_face(0.0, (0.0, 0.5)), slab_bottom),
        (i_face(0.0, (1.5, 2.0)), slab_bottom),
    ]
}

/// Every seat as a patch record.
fn patch_records(seats: &[(FaceKey, FaceKey)]) -> ContactRecords {
    let mut records = ContactRecords::default();
    for &(face_a, face_b) in seats {
        records.patches.push(PatchContact { face_a, face_b });
    }
    records
}

/// Every seat's corners as v-on-f records on the other face.
fn vertex_records(body: &Body<f64>, seats: &[(FaceKey, FaceKey)]) -> ContactRecords {
    let faces = face_vertices(body);
    let mut records = ContactRecords::default();
    for &(mine, theirs) in seats {
        for &(vertex, _) in &faces[&mine] {
            records.a_on_b.push(VfContact {
                vertex,
                face: theirs,
            });
        }
    }
    records
}

/// The placement refusals on a solid pair, as their reasons.
fn placement(errors: &[ValidationError]) -> Vec<&'static str> {
    errors
        .iter()
        .filter_map(|e| match e {
            ValidationError::CensusUndecidable {
                a: EntityId::Solid(_),
                b: EntityId::Solid(_),
                what,
            } => Some(*what),
            ValidationError::InstanceInterference { .. } => Some("interference"),
            _ => None,
        })
        .collect()
}

fn refusal(body: &Body<f64>, records: &ContactRecords) -> Vec<ValidationError> {
    validate_pseudomanifold(body, records, Tol::witness()).expect_err("the pose refuses")
}

/// **The I-profile's waist through a slab, declared as its four seats'
/// patches**, refuses on the pair's placement: every event the patches
/// back is read, and the waist's corners pass into the slab. It
/// cleared when the patches' events were taken on their word.
#[test]
fn an_i_profile_through_a_slab_declared_as_patches_refuses() {
    let body = i_through_slab();
    let records = patch_records(&i_seats(&body));
    let errors = refusal(&body, &records);
    assert!(
        errors
            .iter()
            .all(|e| !matches!(e, ValidationError::UndeclaredContact { .. })),
        "every meeting is backed: {errors:?}"
    );
    assert!(!placement(&errors).is_empty(), "{errors:?}");
}

/// **The same pose declared as sixteen v-on-f records** (every seat
/// corner) refuses the same way. The waist's corners are saddles, and
/// a record's touch that reads anything but Rest refuses.
#[test]
fn an_i_profile_through_a_slab_declared_as_vertex_records_refuses() {
    let body = i_through_slab();
    let records = vertex_records(&body, &i_seats(&body));
    assert_eq!(records.a_on_b.len(), 16, "four corners a seat");
    let errors = refusal(&body, &records);
    assert!(!placement(&errors).is_empty(), "{errors:?}");
}

/// **The control: the I-profile seated on the slab's top by its tail's
/// underside**, the waist clear of the slab, certifies declared as a
/// patch and as the seat's four corners.
#[test]
fn an_i_profile_seated_on_a_slab_certifies_declared_both_ways() {
    let body = assembly(&[slab(), i_profile(2.0)]);
    let seat = the_face(&body, |p| near(p.y, 1.0) && (0.0..=2.0).contains(&p.x));
    let slab_top = the_face(&body, |p| near(p.y, 1.0) && p.x.abs() > 4.0);
    let seats = [(seat, slab_top)];
    for records in [patch_records(&seats), vertex_records(&body, &seats)] {
        assert_eq!(
            validate_pseudomanifold(&body, &records, Tol::witness()),
            Ok(()),
            "{records:?}"
        );
    }
}

/// The L-bracket over `z ∈ [0, 1]`, counterclockwise from `+z`, reflex
/// at `(1, 1)`.
const L_PROFILE: [(f64, f64); 6] = [
    (0.0, 0.0),
    (3.0, 0.0),
    (3.0, 1.0),
    (1.0, 1.0),
    (1.0, 3.0),
    (0.0, 3.0),
];

/// **A saddle star at a declared site, no face of either on a common
/// plane.** A parallelepiped's corner meets the L-bracket's saddle
/// corner `(1, 1, 1)` from the notch, one of its edges dipping below
/// the bracket's top into the notch: a true rest, which no plane
/// separates and whose complement test fails, so the analysis reads it
/// unanalysed. Declared with a v-v record it refuses as that; it
/// cleared while a record's touch refused only on a decided crossing.
#[test]
fn a_saddle_corner_declared_vertex_to_vertex_refuses_unanalysed() {
    let (p, a, b, c) = (
        [1.0, 1.0, 1.0],
        [0.3, 0.3, -0.5],
        [0.0, 1.0, 0.5],
        [1.0, 0.0, 0.5],
    );
    let block = common::mapped_cube(
        move |u, v, w| {
            Point3::new(
                p[0] + u * a[0] + v * b[0] + w * c[0],
                p[1] + u * a[1] + v * b[1] + w * c[1],
                p[2] + u * a[2] + v * b[2] + w * c[2],
            )
        },
        Tol::witness(),
    );
    let bracket = common::prism_z::<f64>(&L_PROFILE, 0.0, 1.0, Tol::witness()).body;
    let body = assembly(&[bracket, block]);
    let at_corner: Vec<VertexKey> = body
        .vertices()
        .filter(|(_, v)| {
            let q = body.get_point(v.point).unwrap();
            near(q.x, 1.0) && near(q.y, 1.0) && near(q.z, 1.0)
        })
        .map(|(k, _)| k)
        .collect();
    assert_eq!(at_corner.len(), 2, "one corner of each");
    let undeclared = refusal(&body, &ContactRecords::default());
    assert!(
        undeclared.iter().any(|e| matches!(
            e,
            ValidationError::UndeclaredContact {
                contact: topo::CensusContact::VertexVertex { .. },
                ..
            }
        )),
        "{undeclared:?}"
    );
    let mut records = ContactRecords::default();
    records.vv.push(VvContact {
        a: at_corner[0],
        b: at_corner[1],
    });
    let errors = refusal(&body, &records);
    let reasons = placement(&errors);
    assert_eq!(reasons.len(), 1, "{errors:?}");
    assert!(
        reasons[0].contains("neither convex nor concave"),
        "{errors:?}"
    );
}
