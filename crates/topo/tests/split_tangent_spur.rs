//! **A plane tangent to a solid along an edge refuses the same way
//! whether or not the contact meets a real section.**
//!
//! The plane y + z = 2 touches a block y, z ∈ (0, 1) along its top/far
//! edge y = z = 1 and nowhere else. On its own that contact joins into
//! a zero-area section polygon, refused `DegenerateSection`. With a
//! slab through the block that the plane really cuts, the contact's
//! null edges join into the slab's section polygon instead — an
//! out-and-back spur along the edge, which leaves the polygon's net
//! area positive. The join refuses that spur the same way.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point3, Tol, Vec3};
use topo::test_support::brick;
use topo::{Body, SplitError, SplitJoinError, SplitPlane, plane_section, split, union};

fn tangent_plane() -> SplitPlane<f64> {
    let h = std::f64::consts::FRAC_1_SQRT_2;
    SplitPlane {
        origin: Point3::new(0.0, 1.0, 1.0),
        normal: Vec3::new(0.0, h, h),
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

fn refuses_degenerate(label: &str, body: &Body<f64>) {
    let err = split(body, &tangent_plane(), Tol::witness())
        .err()
        .unwrap_or_else(|| panic!("{label}: the split succeeded"));
    assert!(
        matches!(
            err,
            SplitError::Join(SplitJoinError::DegenerateSection { .. })
        ),
        "{label}: {err:?}"
    );
}

/// The block alone, and the block with a slab away from the tangent
/// edge's reach, refuse `DegenerateSection` — the contact is its own
/// zero-area polygon.
#[test]
fn a_tangent_contact_standing_alone_refuses_degenerate() {
    let block = brick::<f64>((0.0, 1.5), (0.0, 1.0), (0.0, 1.0), Tol::witness());
    refuses_degenerate("block", &block);
    let far = brick::<f64>((0.2, 0.3), (-1.0, 2.0), (0.5, 3.0), Tol::witness());
    refuses_degenerate("block ∪ slab at x = 0.2", &unite(&block, &far));
}

/// With a slab at x = 1.2..1.3 the contact joins the slab's section as
/// a spur (on the pre-fix join the split SUCCEEDED, with two below
/// copies of every vertex along the edge). Both operand orders, and
/// the section-only door, now refuse like the standing-alone contact.
#[test]
fn a_tangent_contact_meeting_a_real_section_refuses_the_same_way() {
    let block = brick::<f64>((0.0, 1.5), (0.0, 1.0), (0.0, 1.0), Tol::witness());
    let slab = brick::<f64>((1.2, 1.3), (-1.0, 2.0), (0.5, 3.0), Tol::witness());
    for (label, body) in [
        ("block ∪ slab", unite(&block, &slab)),
        ("slab ∪ block", unite(&slab, &block)),
    ] {
        refuses_degenerate(label, &body);
        let err = plane_section(&body, &tangent_plane(), Tol::witness())
            .err()
            .unwrap_or_else(|| panic!("{label}: the section succeeded"));
        assert!(
            matches!(
                err,
                SplitError::Join(SplitJoinError::DegenerateSection { .. })
            ),
            "{label}: {err:?}"
        );
    }
}
