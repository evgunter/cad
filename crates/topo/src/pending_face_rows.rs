//! **A face with no surface yet is refused as pending, never panicked
//! on and never read as something else.** Rows over a closed prism
//! whose faces are all pending (the `mvfs` seed's slot, which every
//! `mef` of the declined prism inherits), and over a finished brick
//! with one face re-charted pending: each public door refuses typed,
//! and the refusals that name a cause name pending, apart from a net
//! that is not finite.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use std::panic::{AssertUnwindSafe, catch_unwind};

use geom_core::{Band, Point3, RigidMotion, Tol, Vec3};

/// Whether `f` panicked (the hook prints its message).
fn panicked<R>(f: impl FnOnce() -> R) -> bool {
    catch_unwind(AssertUnwindSafe(f)).is_err()
}

/// Every door here, run on `body`, the names of those that panicked.
fn doors_that_panic(body: &crate::Body<f64>, tol: Tol) -> Vec<String> {
    let band = Band::linear(tol).unwrap();
    let mut out = Vec::new();
    let mut door = |name: &str, p: bool| {
        if p {
            out.push(name.to_owned());
        }
    };
    door("validate", panicked(|| crate::validate::validate(body)));
    door(
        "validate_closed",
        panicked(|| crate::validate::validate_closed(body)),
    );
    door(
        "validate_geometric",
        panicked(|| crate::validate::validate_geometric(body, tol)),
    );
    door(
        "transform_rigid",
        panicked(|| {
            crate::transform::transform_rigid(
                body,
                &RigidMotion::translation(Vec3::new(1.0, 2.0, 3.0)).unwrap(),
            )
        }),
    );
    door(
        "mirror_body",
        panicked(|| {
            let r = crate::mirror::Reflection::across(
                Point3::new(0.1, -0.2, 0.3),
                Vec3::new(1.0, 2.0, 3.0),
                tol,
            )
            .unwrap();
            crate::mirror::mirror_body(body, &r, tol)
        }),
    );
    door("reverted", panicked(|| body.reverted()));
    door(
        "point_in_solid",
        panicked(|| {
            crate::solid_contain::point_in_solid(body, Point3::new(0.5, 0.5, 0.5), band, tol)
        }),
    );
    door(
        "merge_coplanar_faces",
        panicked(|| body.clone().merge_coplanar_faces(tol)),
    );
    door(
        "mass_properties_closed_form",
        panicked(|| crate::props::mass_properties_closed_form(body, band, tol)),
    );
    for f in body.faces().map(|(k, _)| k).collect::<Vec<_>>() {
        door(
            "face_carrier_kind",
            panicked(|| crate::readback::face_carrier_kind(body, f)),
        );
        door(
            "face_pose",
            panicked(|| crate::readback::face_pose(body, f)),
        );
        door(
            "face_cover",
            panicked(|| crate::cover::face_cover(body, f, band)),
        );
    }
    out
}

/// **No door panics on an all-pending prism.**
#[test]
fn no_door_panics_on_an_all_pending_prism() {
    let tol = Tol::witness();
    let body = crate::fixtures::raw_prism(4, tol).body;
    assert!(
        body.surface_slots().all(|(_, s)| s.is_pending()),
        "the fixture: every slot pending"
    );
    let panics = doors_that_panic(&body, tol);
    assert!(panics.is_empty(), "doors that panicked: {panics:#?}");
}

/// **A finished face cannot be re-charted pending**: its edges'
/// descriptions name its surface, so moving the face off it strands
/// them, and the door refuses typed, naming each edge, with the body
/// unchanged.
#[test]
fn a_described_face_is_not_recharted_pending() {
    use crate::construction::FaceSurface;
    let tol = Tol::witness();
    let brick = crate::test_support_fixtures::brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
    let face = brick.faces().map(|(k, _)| k).next().unwrap();
    let sense = brick.get_face(face).unwrap().sense;
    let mut b = brick.clone();
    let refused = b.set_face_surface(face, FaceSurface::Pending { sense });
    assert!(
        matches!(
            &refused,
            Err(crate::EulerOpError::RechartStrandsDescriptions { edges, .. }) if edges.len() == 4
        ),
        "the four rim edges are named: {refused:?}"
    );
    assert_eq!(
        format!("{b:?}"),
        format!("{brick:?}"),
        "the body is unchanged"
    );
}

/// **`face_cover` and `point_in_solid` tell a pending face from one on a
/// net that is not finite**: the pending face's refusal is the pending
/// arm, the corrupt net's is its own, and neither reads as the other.
#[test]
fn a_pending_face_is_refused_apart_from_a_corrupt_net() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let pending = crate::fixtures::raw_prism(4, tol).body;
    let pf = pending.faces().map(|(k, _)| k).next().unwrap();
    let mut poisoned =
        crate::test_support_fixtures::brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
    let qf = poisoned.faces().map(|(k, _)| k).next().unwrap();
    let key = poisoned.get_face(qf).unwrap().surface;
    *poisoned.surface_mut(key).unwrap() = crate::fixtures::poisoned_net(|_| true);

    let cover_pending = crate::cover::face_cover(&pending, pf, band).expect_err("no cover");
    let cover_poisoned = crate::cover::face_cover(&poisoned, qf, band).expect_err("no cover");
    assert!(
        cover_pending.is_pending() && cover_pending.carrier().is_none(),
        "a pending face's cover refusal is the pending arm: {cover_pending:?}"
    );
    assert!(
        !cover_poisoned.is_pending()
            && cover_poisoned.carrier().is_some_and(|c| {
                c.unknowns()
                    .iter()
                    .any(|&(_, what)| what == geom::cover::Unknown::PoisonedNet)
            }),
        "a corrupt net's cover refusal is its net's: {cover_poisoned:?}"
    );

    let inside = Point3::new(0.5, 0.5, 0.5);
    let pis_pending = crate::solid_contain::point_in_solid(&pending, inside, band, tol);
    let pis_poisoned = crate::solid_contain::point_in_solid(&poisoned, inside, band, tol);
    assert!(
        matches!(
            pis_pending,
            Err(crate::solid_contain::PointInSolidError::PendingFace { .. })
        ),
        "point_in_solid names a pending face: {pis_pending:?}"
    );
    assert!(
        !matches!(
            pis_poisoned,
            Err(crate::solid_contain::PointInSolidError::PendingFace { .. })
        ),
        "a corrupt net is not read as pending: {pis_poisoned:?}"
    );
}
