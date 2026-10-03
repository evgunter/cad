//! **A decided zero and a poisoned offset are two outcomes.** Two
//! bricks meet on `z = 1`; the rows ask [`pair_finding`] about their
//! two caps, once as built and once with the lower cap's plane datum
//! poisoned, and read what the Boolean's undeclared coincidence says
//! on each.
//!
//! The poisoned operand is built through the failure-injection door:
//! the public re-charting doors refuse a plane no edge of the face
//! certifies against, so no NaN datum reaches a stored face there.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point3, Tol, Vec3};

use super::{FlushRung, pair_finding};
use crate::body::Body;
use crate::boolean::{
    BooleanError, BooleanOp, CarrierDesc, CarrierRelation, ConsumedExtent, Operand, PlaneDesc,
    PlaneEqError, PlaneIdentity, boolean_reduce, face_carrier, oriented_plane_eq,
};
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

/// `a` with its cap on `z = 1` re-charted onto the same plane, its
/// origin's `x` replaced by NaN: the offset datum `n·origin` is
/// poisoned while the normal stays readable, so the ladder reaches the
/// offset rung. Returns the cap.
fn poison_cap(a: &mut Body<f64>) -> FaceKey {
    let top = cap(a, 1.0);
    let f = a.get_face(top).unwrap();
    let Some(geom::Surface::Plane {
        origin,
        normal,
        u_ref,
    }) = a.get_surface(f.surface).cloned()
    else {
        panic!("the cap is planar");
    };
    let surface = FaceSurface::New {
        surface: geom::Surface::Plane {
            origin: Point3::new(f64::NAN, origin.y, origin.z),
            normal,
            u_ref,
        },
        sense: f.sense,
    };
    // Lifts both refusals: a plane whose offset datum is NaN is the row's premise, and no edge certifies against it.
    a.set_face_surface_stranding_for_tests(top, surface)
        .unwrap();
    top
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
    assert_eq!(
        evidence.relation,
        CarrierRelation::SameOpposite,
        "the caps rest"
    );
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
    let top = poison_cap(&mut a);
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

/// **The Boolean's undeclared coincidence quotes what its measure
/// read, on a decided zero.** The undeclared union of the stack
/// refuses on a pair of side walls whose offset decides zero, and
/// quotes that decided margin.
#[test]
fn the_boolean_refusal_on_a_decided_zero_quotes_its_margin() {
    let (a, b) = stacked();
    let err = match boolean_reduce(BooleanOp::Union, &a, &b, Tol::witness()) {
        Err(err) => err,
        Ok(_) => panic!("an undeclared flush stack does not reduce"),
    };
    let BooleanError::UndeclaredCoincidence { diag, .. } = &err else {
        panic!("the stack meets the coincidence door: {err:?}");
    };
    assert_eq!(diag.predicate, Some("bool_plane_offset"), "{err:?}");
    let text = err.to_string();
    assert!(text.contains("lies within the zero band"), "{text}");
    assert!(!text.contains("exactly zero"), "{text}");
}

/// **The Boolean's undeclared coincidence quotes what its measure
/// read, on a poisoned offset.** The plane ladder's public door refuses
/// a NaN offset datum `Undeclared`, and the Boolean raises that refusal
/// as it raises every undeclared coincidence: the text says the margin
/// is invalid, and never that the measure is exactly zero. (No Boolean
/// reaches it with a poisoned operand: the stack above refuses on its
/// side walls first.)
#[test]
fn an_undeclared_coincidence_on_a_poisoned_offset_says_it_is_poisoned() {
    let plane = |x| PlaneDesc {
        origin: Point3::new(x, 0.0, 1.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
    };
    let extent = ConsumedExtent::unwitnessed(geom_brep::ExtentBall::new(Point3::origin(), 1.0));
    let err = oriented_plane_eq(
        &plane(0.0),
        &plane(f64::NAN),
        PlaneIdentity::NONE,
        &extent,
        band(),
    )
    .unwrap_err();
    let PlaneEqError::Undeclared {
        coincidence,
        relation,
    } = err
    else {
        panic!("a NaN offset reaches the offset rung: {err:?}");
    };
    let text = BooleanError::UndeclaredCoincidence {
        diag: coincidence.reported(),
        pair: [
            (Operand::A, FaceKey::default()),
            (Operand::B, FaceKey::default()),
        ],
        relation,
    }
    .to_string();
    assert!(text.contains("margin is invalid"), "{text}");
    assert!(!text.contains("exactly zero"), "{text}");
}
