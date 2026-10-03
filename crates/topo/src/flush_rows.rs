//! **A decided zero and a poisoned offset are two outcomes.** Two
//! bricks meet on `z = 1`; the rows ask [`pair_finding`] about their
//! two caps, once as built and once with the lower cap's plane datum
//! poisoned (NaN, or `+∞`), and read what the Boolean's undeclared
//! coincidence says on each.
//!
//! The poisoned operand is built through the failure-injection door:
//! the public re-charting doors refuse a plane no edge of the face
//! certifies against, so no NaN datum reaches a stored face there.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, KERNEL_OR_FILE_DEFECT_ENDING, Point3, Tol, Vec3};

use super::{FlushRefusal, FlushRung, PairUndecided, find_flush_candidates, pair_finding};
use crate::body::Body;
use crate::boolean::{
    BooleanError, BooleanOp, CarrierDesc, CarrierRelation, CoincidenceMeasure, ConsumedExtent,
    Operand, PlaneDesc, PlaneEqError, PlaneIdentity, PlaneRelation, boolean_reduce, face_carrier,
    oriented_plane_eq, undeclared_coincidence,
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

/// `a` with its cap on `z = 1` re-charted onto a plane of the same
/// normal through `origin` of its own origin: with a datum that is not
/// finite, the offset `n·origin` is poisoned while the normal stays
/// readable, so the ladder reaches the offset rung. Returns the cap.
fn poison_cap(a: &mut Body<f64>, origin: impl FnOnce(Point3<f64>) -> Point3<f64>) -> FaceKey {
    let top = cap(a, 1.0);
    let f = a.get_face(top).unwrap();
    let Some(geom::Surface::Plane {
        origin: at,
        normal,
        u_ref,
    }) = a.get_surface(f.surface).cloned()
    else {
        panic!("the cap is planar");
    };
    let surface = FaceSurface::New {
        surface: geom::Surface::Plane {
            origin: origin(at),
            normal,
            u_ref,
        },
        sense: f.sense,
    };
    // Lifts both refusals: a plane whose offset datum is not finite is
    // the row's premise, and no edge certifies against it.
    a.set_face_surface_stranding_for_tests(top, surface)
        .unwrap();
    top
}

/// The origin's `x` replaced by NaN.
fn nan_x(o: Point3<f64>) -> Point3<f64> {
    Point3::new(f64::NAN, o.y, o.z)
}

/// The origin moved to `z = +∞`.
fn infinite_z(o: Point3<f64>) -> Point3<f64> {
    Point3::new(o.x, o.y, f64::INFINITY)
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
    let top = poison_cap(&mut a, nan_x);
    let found = pair_finding(&a, top, &b, cap(&b, -1.0), band());
    let Err(PairUndecided::Unreadable(diag)) = found else {
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

/// **An infinite offset is poison too.** The lower cap re-charted to
/// `z = +∞` decides no sign: it is no finding, and the detector names
/// the pair as unreadable, in the poison's own words: a datum that is
/// not finite, ending as the operand's defect, with no lever offered.
#[test]
fn an_infinite_offset_is_no_finding_and_says_it_is_not_finite() {
    let (mut a, b) = stacked();
    let top = poison_cap(&mut a, infinite_z);
    let found = pair_finding(&a, top, &b, cap(&b, -1.0), band());
    let Err(PairUndecided::Unreadable(diag)) = found else {
        panic!("an infinite offset decides nothing, so it is no finding: {found:?}");
    };
    assert_eq!(diag.predicate, Some("bool_plane_offset"), "{diag:?}");
    let refusal = find_flush_candidates(&a, &b, Tol::witness());
    let Err(refusal @ FlushRefusal::PairUnreadable { .. }) = refusal else {
        panic!("the detector names the unreadable pair: {refusal:?}");
    };
    let text = refusal.to_string();
    assert!(
        text.contains("is compared on a surface datum that is not finite"),
        "{text}"
    );
    assert!(text.ends_with(KERNEL_OR_FILE_DEFECT_ENDING), "{text}");
    assert!(!text.contains("separate the geometry"), "{text}");
    assert!(!text.contains("Recourse:"), "{text}");
}

/// **The plane door reads an infinite offset as unreadable, and the
/// largest finite one as a decision.** An origin at `z = +∞` against
/// `z = 1` is no plane either side of the other; at `f64::MAX` it
/// stays finite and decides apart.
#[test]
fn the_plane_door_reads_an_infinite_offset_as_unreadable() {
    let plane = |z| PlaneDesc {
        origin: Point3::new(0.0, 0.0, z),
        normal: Vec3::new(0.0, 0.0, 1.0),
    };
    let extent = ConsumedExtent::unwitnessed(geom_brep::ExtentBall::new(Point3::origin(), 1.0));
    let read = |z| oriented_plane_eq(&plane(z), &plane(1.0), PlaneIdentity::NONE, &extent, band());
    let err = read(f64::INFINITY);
    assert!(
        matches!(
            err,
            Err(PlaneEqError::Undeclared {
                coincidence: CoincidenceMeasure::Unreadable(diag),
                ..
            }) if diag.margin.is_invalid() && diag.predicate == Some("bool_plane_offset")
        ),
        "an infinite offset is unreadable: {err:?}"
    );
    assert_eq!(
        read(f64::MAX).ok(),
        Some(PlaneRelation::Distinct),
        "the largest finite offset decides"
    );
}

/// **A poisoned offset is no coincidence the Boolean offers to
/// declare.** The plane ladder's public door refuses a NaN offset datum
/// as unreadable, and the Boolean raises it as every raise site of an
/// undeclared coincidence does: a datum that is not finite, read at
/// rest on an operand, ending as the kernel's or the file's defect,
/// with no declaration offered and never a measure that is exactly
/// zero. (No Boolean reaches it with a poisoned operand: the stack
/// above refuses on its side walls first.)
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
    let face = FaceKey::default();
    let err = undeclared_coincidence(
        coincidence,
        [(Operand::A, face), (Operand::B, face)],
        relation,
    );
    assert!(
        matches!(err, BooleanError::PoisonedCarrierDatum { diag, .. }
            if diag.predicate == Some("bool_plane_offset")),
        "{err:?}"
    );
    let text = err.to_string();
    let lead = "a surface datum two faces of the operands are compared on is not finite, so \
                the faces describe no shape to compare. ";
    assert_eq!(text, format!("{lead}{KERNEL_OR_FILE_DEFECT_ENDING}"));
    assert!(!text.contains("declare"), "{text}");
    assert!(!text.contains("exactly zero"), "{text}");
}
