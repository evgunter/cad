//! **The sense beside the pose, and the carrier-kind read** — the two
//! read-back doors DOCM-REFERENCES-DESIGN DM1a and DM2 add.
//!
//! What is pinned here is the kernel half of the contract: the pose's
//! `sense` is the face record's stored flag and nothing else (both
//! senses, against the stored bit, so a door that silently kept the
//! chart normal would fail one row); `axis` is still the chart's
//! direction on either sense; and the carrier kind is the stored tag
//! copied out, refusing only the dangling arms — every carrier has a
//! kind, so a tag read never has a "no answer" lane.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::SurfaceKind;
use geom_core::{Point3, Vec3};
use topo::readback::{DanglingRef, ReadbackError, face_carrier_kind, face_pose};
use topo::{Body, EntityId, FaceKey, FaceSurface, Surface};

/// A seed face carrying `surface`.
fn seed_face(surface: Surface<f64>) -> (Body<f64>, FaceKey) {
    let mut body = Body::<f64>::new();
    let seed = body
        .mvfs(Point3::new(0.0, 0.0, 0.0))
        .expect("mvfs has no preconditions");
    body.set_face_surface(seed.face, FaceSurface::New(surface))
        .expect("a live face takes a surface");
    (body, seed.face)
}

fn plane() -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(0.0, 0.0, 1.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// **The sense is the stored flag, on both senses, and the axis stays
/// the chart's on both.** The flipped body is the discriminating
/// input `flipped_face_sense_for_tests` exists for: a door that folded
/// the sense into `axis` would move the axis here, and a door that
/// ignored the sense would report `true` on the flipped row.
#[test]
fn face_pose_reports_the_stored_sense_beside_an_uncorrected_axis() {
    let (body, face) = seed_face(plane());
    let flipped = body
        .flipped_face_sense_for_tests(face)
        .expect("the seed face is live");
    for (label, b, stored) in [("minted", &body, true), ("flipped", &flipped, false)] {
        let stored_flag = b.get_face(face).expect("live").sense;
        assert_eq!(stored_flag, stored, "{label}: the fixture's own sense");
        let pose = face_pose(b, face).expect("a planar carrier");
        assert_eq!(
            pose.sense, stored,
            "{label}: the pose copies the stored flag out"
        );
        assert_eq!(
            pose.axis.z, 1.0,
            "{label}: the axis is the chart's, uncorrected"
        );
        // The outward normal is the reader's to form: sense · axis.
        let sign = if pose.sense { 1.0 } else { -1.0 };
        assert_eq!(
            pose.axis.z * sign,
            if stored { 1.0 } else { -1.0 },
            "{label}"
        );
    }
}

/// **The carrier kind is the stored tag, for every analytic kind and
/// for the two that have no canonical frame.** `face_pose` refuses a
/// NURBS carrier (rule 3); the kind read does not, because the kind
/// is exactly what IS stored about such a face.
#[test]
fn face_carrier_kind_copies_the_tag_out_for_every_kind() {
    let rows: [(Surface<f64>, SurfaceKind); 3] = [
        (plane(), SurfaceKind::Plane),
        (
            Surface::Cylinder {
                origin: Point3::new(0.0, 0.0, 0.0),
                axis: Vec3::new(0.0, 0.0, 1.0),
                radius: 1.0,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
            SurfaceKind::Cylinder,
        ),
        (
            Surface::Sphere {
                center: Point3::new(0.0, 0.0, 0.0),
                radius: 1.0,
                axis: Vec3::new(0.0, 0.0, 1.0),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
            SurfaceKind::Sphere,
        ),
    ];
    for (surface, kind) in rows {
        let (body, face) = seed_face(surface);
        assert_eq!(face_carrier_kind(&body, face), Ok(kind));
        // The sense flip changes nothing about the kind: two facts.
        let flipped = body.flipped_face_sense_for_tests(face).expect("live");
        assert_eq!(face_carrier_kind(&flipped, face), Ok(kind));
    }
}

/// **The only refusal is a dangling key**, in the same vocabulary
/// `face_pose` uses, so a caller maps the two doors' refusals alike.
#[test]
fn face_carrier_kind_refuses_dangling_and_nothing_else() {
    let (body, face) = seed_face(plane());
    let empty = Body::<f64>::new();
    assert_eq!(
        face_carrier_kind(&empty, face),
        Err(ReadbackError::Dangling {
            what: DanglingRef::Entity(EntityId::Face(face)),
        })
    );
    assert_eq!(face_carrier_kind(&body, face), Ok(SurfaceKind::Plane));
    // The placeholder a seed face is minted with is a kind too (rule
    // 1: the tag is stored data), where `face_pose` has no frame to
    // report.
    let mut bare = Body::<f64>::new();
    let seed = bare
        .mvfs(Point3::new(0.0, 0.0, 0.0))
        .expect("mvfs has no preconditions");
    assert!(matches!(
        face_pose(&bare, seed.face),
        Err(ReadbackError::NoCanonicalFrame { .. })
    ));
    assert!(face_carrier_kind(&bare, seed.face).is_ok());
}
