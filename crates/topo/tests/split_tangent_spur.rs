//! **A plane tangent to a solid along an edge stays refused when it
//! also cuts the solid elsewhere.**
//!
//! The plane y + z = 2 touches a block y, z ∈ (0, 1) along its top/far
//! edge y = z = 1 and nowhere else; a slab through the block at
//! x = 1.2..1.3 is genuinely cut by it. With the normal (0, h, h) the
//! block lies on the run's below side, the contact's null edges close
//! a zero-area polygon of their own, and the area certificate refuses
//! it (`DegenerateSection`). `split` reads that refusal as a pinch and
//! reruns under (0, −h, −h), where the contact's null edges join the
//! slab's section as an out-and-back spur of positive net area. That
//! run used to SUCCEED, leaving a zero-width slit in both halves; the
//! join now refuses it (`SectionSpur`). Each row names which predicate
//! refused, so a substitution of one for the other goes red.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point3, Tol, Vec3};
use topo::test_support::brick;
use topo::{Body, SplitError, SplitJoinError, SplitPlane, plane_section, split, union};

/// The plane y + z = 2, normal (0, s·h, s·h).
fn tangent_plane(s: f64) -> SplitPlane<f64> {
    let h = std::f64::consts::FRAC_1_SQRT_2;
    SplitPlane {
        origin: Point3::new(0.0, 1.0, 1.0),
        normal: Vec3::new(0.0, s * h, s * h),
    }
}

fn unite(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    union(a, b, Tol::witness())
        .unwrap()
        .body()
        .expect("the two bricks overlap")
        .body
        .clone()
}

fn block() -> Body<f64> {
    brick::<f64>((0.0, 1.5), (0.0, 1.0), (0.0, 1.0), Tol::witness())
}

/// Which join refusal a result carries, by name.
fn refusal<T>(label: &str, r: Result<T, SplitError>) -> &'static str {
    match r {
        Ok(_) => panic!("{label}: the operation succeeded"),
        Err(SplitError::Join(SplitJoinError::DegenerateSection { .. })) => "area",
        Err(SplitError::Join(SplitJoinError::SectionSpur { .. })) => "spur",
        Err(e) => panic!("{label}: {e:?}"),
    }
}

/// The contact standing alone — the block, and the block with a slab
/// the contact does not reach — refuses on area. These pass on the
/// pre-guard join as well: they are the posture the rows below hold.
#[test]
fn a_tangent_contact_standing_alone_refuses_on_area() {
    let far = brick::<f64>((0.2, 0.3), (-1.0, 2.0), (0.5, 3.0), Tol::witness());
    for (label, body) in [
        ("block", block()),
        ("block ∪ far slab", unite(&block(), &far)),
    ] {
        let r = split(&body, &tangent_plane(1.0), Tol::witness());
        assert_eq!(refusal(label, r), "area", "{label}");
        let r = plane_section(&body, &tangent_plane(1.0), Tol::witness());
        assert_eq!(refusal(label, r), "area", "{label}");
    }
}

/// With the slab the contact reaches, in both operand orders:
/// - `split` with (0, h, h) refuses on AREA — the direct run's refusal,
///   surfaced after the mirrored rerun refuses the spur (it used to
///   succeed there);
/// - `split` with (0, −h, −h) runs the spur orientation directly and
///   refuses on SPUR (it used to succeed);
/// - `plane_section`, which has no rerun, refuses on AREA with
///   (0, h, h) — true before the guard too — and on SPUR with
///   (0, −h, −h), where it used to succeed.
#[test]
fn a_tangent_contact_meeting_a_real_section_stays_refused() {
    let slab = brick::<f64>((1.2, 1.3), (-1.0, 2.0), (0.5, 3.0), Tol::witness());
    for (label, body) in [
        ("block ∪ slab", unite(&block(), &slab)),
        ("slab ∪ block", unite(&slab, &block())),
    ] {
        let r = split(&body, &tangent_plane(1.0), Tol::witness());
        assert_eq!(refusal(label, r), "area", "{label}: split, +n");
        let r = split(&body, &tangent_plane(-1.0), Tol::witness());
        assert_eq!(refusal(label, r), "spur", "{label}: split, −n");
        let r = plane_section(&body, &tangent_plane(1.0), Tol::witness());
        assert_eq!(refusal(label, r), "area", "{label}: section, +n");
        let r = plane_section(&body, &tangent_plane(-1.0), Tol::witness());
        assert_eq!(refusal(label, r), "spur", "{label}: section, −n");
    }
}
