//! **A decided zero and a poisoned offset are two outcomes.** Two
//! bricks meet on `z = 1`; the rows ask [`pair_finding`] about their
//! two caps, once as built and once with the lower cap's plane datum
//! poisoned.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point3, Tol};

use super::{FlushRung, pair_finding};
use crate::body::Body;
use crate::boolean::{CarrierDesc, CarrierRelation, face_carrier};
use crate::entity::FaceKey;
use crate::euler::FaceSurface;
use crate::query::all_faces;
use crate::test_support_fixtures::brick;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// The two stacked bricks, independently authored (no shared source).
fn stacked() -> (Body<f64>, Body<f64>) {
    (
        brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), Tol::witness()),
        brick((0.0, 1.0), (0.0, 1.0), (1.0, 2.0), Tol::witness()),
    )
}

/// The planar face of `body` on `z = 1` whose outward normal points
/// along `sign`·z.
fn cap(body: &Body<f64>, sign: f64) -> FaceKey {
    let hits: Vec<FaceKey> = all_faces(body)
        .into_iter()
        .filter(|&f| {
            matches!(face_carrier(body, f), Some(CarrierDesc::Plane { origin, normal })
                if (origin.z - 1.0).abs() < 1e-12 && normal.z * sign > 0.5)
        })
        .collect();
    let [f] = hits[..] else {
        panic!("expected one cap on z = 1, got {hits:?}");
    };
    f
}

/// `face`'s plane, its origin's `x` replaced by NaN: the offset datum
/// `n·origin` is poisoned while the normal stays readable, so the
/// ladder reaches the offset rung.
fn poisoned_plane(body: &Body<f64>, face: FaceKey) -> FaceSurface<f64> {
    let f = body.get_face(face).unwrap();
    let Some(geom::Surface::Plane {
        origin,
        normal,
        u_ref,
    }) = body.get_surface(f.surface).cloned()
    else {
        panic!("the cap is planar");
    };
    FaceSurface::New {
        surface: geom::Surface::Plane {
            origin: Point3::new(f64::NAN, origin.y, origin.z),
            normal,
            u_ref,
        },
        sense: f.sense,
    }
}

#[test]
fn probe_public_doors() {
    let (a, _) = stacked();
    let top = cap(&a, 1.0);
    let mut via_set = a.clone();
    eprintln!(
        "PROBE set_face_surface: {:?}",
        via_set.set_face_surface(top, poisoned_plane(&a, top))
    );
    let mut via_describing = a.clone();
    let FaceSurface::New { surface, sense } = poisoned_plane(&a, top) else {
        unreachable!()
    };
    eprintln!(
        "PROBE set_face_surfaces_describing: {:?}",
        via_describing.set_face_surfaces_describing(
            vec![crate::attach::Rechart::new(surface, top, sense)],
            &[],
            Tol::witness()
        )
    );
}

#[test]
fn probe_boolean() {
    let (mut a, b) = stacked();
    let top = cap(&a, 1.0);
    a.set_face_surface_stranding_for_tests(top, poisoned_plane(&a, top))
        .unwrap();
    match crate::boolean::boolean_reduce(crate::boolean::BooleanOp::Union, &a, &b, Tol::witness())
    {
        Ok(_) => eprintln!("PROBE boolean: Ok"),
        Err(e) => eprintln!("PROBE boolean: {e:?}\nDISPLAY: {e}"),
    }
}

/// **The no-regression half.** The caps decide coincident at zero
/// offset: a finding, on the geometric rung, resting.
#[test]
fn a_decided_zero_offset_is_a_decided_coincident_finding() {
    let (a, b) = stacked();
    let found = pair_finding(&a, cap(&a, 1.0), &b, cap(&b, -1.0), band());
    let Ok(Some(evidence)) = found else {
        panic!("two caps on one plane are a definite finding: {found:?}");
    };
    assert_eq!(evidence.relation, CarrierRelation::SameOpposite, "the caps rest");
    assert_eq!(
        evidence.rung,
        FlushRung::DecidedCoincident,
        "no shared source: the geometric rung decided"
    );
}

/// **The witness.** The lower cap's offset datum is NaN: nothing
/// decided the pair coincident, so it is no finding, and the detector
/// refuses with the poisoned margin.
#[test]
fn a_poisoned_offset_is_no_finding() {
    let (mut a, b) = stacked();
    let top = cap(&a, 1.0);
    // Lifts both refusals: a plane whose offset datum is NaN is the row's premise, and no edge certifies against it.
    a.set_face_surface_stranding_for_tests(top, poisoned_plane(&a, top))
        .unwrap();
    let found = pair_finding(&a, top, &b, cap(&b, -1.0), band());
    let Err(diag) = found else {
        panic!("a poisoned offset decides nothing, so it is no finding: {found:?}");
    };
    assert!(
        diag.margin.is_invalid(),
        "the refusal carries the poisoned margin: {diag:?}"
    );
    assert_eq!(
        diag.predicate,
        Some("bool_plane_offset"),
        "the offset rung refused: {diag:?}"
    );
}
