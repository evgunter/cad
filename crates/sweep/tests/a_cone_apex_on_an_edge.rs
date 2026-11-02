//! **A Boolean whose edge passes through a cone's apex reads the edge's
//! meeting with the apex as the apex vertex's**: the edge's one root
//! there, a double one, is placed on the cone face's apex vertex
//! (`boolean::reduce::line_cone_roots`), splitting the edge there, and
//! the vertex pair reads the halves (`boolean::fan`). The cone of height
//! 1 about `y`, apex at `(0, 1, 0)`, base radius `tan α` on `y = 0`, in
//! both forms: its apex reached by one wrap edge
//! (`common::wrap_tip::one_wrap_edge`) and revolved by `sweep::revolve`.
//! Against a box one of whose edges runs through the apex, turned about
//! the cone's axis so that the edge meets the cone's seam at no special
//! angle, each op, either way round, holds its closed form, tiers 3 and
//! 3′ over the contacts it records, and a lattice of points the analytic
//! region places:
//!
//! - **an edge across the apex**, the box's top holding the apex and its
//!   side `x = 0` the axis: the box holds half the cone above its floor;
//! - **an edge along the axis**, the box's two sides holding it: a
//!   quarter of the cone above its floor, the edge inside the nappe.
//!
//! Where the box cuts one cone face along two rulings and the result
//! keeps the cone's remainder (`∪`, `cone ∖ box`) on both lobes of that
//! face, the face is pinched at the apex, which props cannot yet measure:
//! that op refuses `ResultPinchedAtConeApex` by name
//! (`a_cone_apex_on_a_plane_through_it`). Which face is cut that way is
//! the turn's: the wrap-tipped cone has one face, the revolved cone two,
//! parted at its seam. The ops that keep the box's part, `∩` and
//! `box ∖ cone`, build at every pose.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::membership::{self, Region};
use crate::common::wrap_tip::{NORTH, one_wrap_edge};
use core::f64::consts::PI;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{finished, revolved_about_y};
use topo::{AtRestBody, BooleanResult, intersect, subtract, union};

fn tol() -> Tol {
    Tol::witness()
}

/// The cone of half-angle `alpha` (degrees), height 1, apex at `NORTH`.
fn cone(alpha: f64) -> topo::Body<f64> {
    let r = alpha.to_radians().tan();
    revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(r, 0.0), 0.0),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
        tol(),
    )
}

/// The cone in both forms: one wrap edge to its apex, and as
/// `sweep::revolve` builds it.
fn forms(alpha: f64) -> [(&'static str, topo::Body<f64>); 2] {
    [
        ("tipped", one_wrap_edge(cone(alpha), true).0),
        (
            "revolved",
            crate::revolve_common::needle(alpha, None, tol()).into_body(),
        ),
    ]
}

/// `body` moved rigidly by `m`, finished.
fn moved(body: &topo::Body<f64>, m: Affine3<f64>) -> AtRestBody<f64> {
    finished(
        "moved",
        topo::transform_rigid(body, &geom_core::test_support::rigid(&m, tol()), tol()).unwrap(),
        tol(),
    )
}

/// The cone's region, its base radius `r`.
fn the_cone(r: f64) -> Region {
    Box::new(move |p: Point3<f64>| (r * (1.0 - p.y) - p.x.hypot(p.z)).min(p.y).min(1.0 - p.y))
}

/// `region` moved by `m`: a point lies in it where `m⁻¹` takes it into
/// `region`.
fn moved_region(region: Region, m: Affine3<f64>) -> Region {
    let back = m.inverse();
    Box::new(move |p| region(back.transform_point(p)))
}

/// The cone of half-angle `alpha`'s volume, `π tan²α / 3`.
fn volume(alpha: f64) -> f64 {
    PI * alpha.to_radians().tan().powi(2) / 3.0
}

/// One operand of [`four_ops`]: the body, its region, its volume.
type Operand<'a> = (&'a AtRestBody<f64>, &'a dyn Fn() -> Region, f64);

/// Holds the four ops of `a` and `b` to the volumes `share` (their
/// intersection's) sets, and to their regions at the `7³` lattice over
/// `lattice`, with tiers 3 and 3′ over the contacts each result records.
fn four_ops(
    what: &str,
    pinch: bool,
    (a, ra, va): Operand,
    (b, rb, vb): Operand,
    share: f64,
    lattice: (Point3<f64>, Point3<f64>),
) {
    let both: Region = {
        let (x, y) = (ra(), rb());
        Box::new(move |p| x(p).min(y(p)))
    };
    let either: Region = {
        let (x, y) = (ra(), rb());
        Box::new(move |p| x(p).max(y(p)))
    };
    let a_less: Region = {
        let (x, y) = (ra(), rb());
        Box::new(move |p| x(p).min(-y(p)))
    };
    let b_less: Region = {
        let (x, y) = (ra(), rb());
        Box::new(move |p| y(p).min(-x(p)))
    };
    for (op, result, want, region) in [
        ("a ∩ b", intersect(a, b, tol()), share, both),
        ("a ∪ b", union(a, b, tol()), va + vb - share, either),
        ("a ∖ b", subtract(a, b, tol()), va - share, a_less),
        ("b ∖ a", subtract(b, a, tol()), vb - share, b_less),
    ] {
        let what = format!("{what}, {op}");
        match result {
            Ok(BooleanResult::Body(body)) => {
                membership::holds_short_of_tier_3_prime(
                    &what, &body.body, want, &region, lattice, 7,
                );
                // Parts touching at the apex or along a ruling are curved
                // faces of two shells the census cannot yet clear
                // (`work/restread/census-cross-solid-curved-pairs-undecidable-on-shell-results.md`).
                let census = topo::validate_pseudomanifold(&body.body, &body.contacts, tol());
                if body.body.shells().count() == 1 {
                    assert_eq!(census, Ok(()), "{what}: tier 3′ over the result's contacts");
                } else {
                    assert!(
                        census
                            .as_ref()
                            .is_err_and(|errors| errors.iter().all(|e| matches!(
                                e,
                                topo::ValidationError::CensusUndecidable {
                                    reason: topo::Undecidable::MayOverlap,
                                    ..
                                }
                            ))),
                        "{what}: tier 3′ {census:?}"
                    );
                }
            }
            Ok(BooleanResult::Empty) => assert!(want.abs() < 1e-12, "{what}: empty, not {want}"),
            // The wrap-tipped face cut along two rulings through its
            // apex is left pinched there, which props cannot yet measure
            // (`a_cone_apex_on_a_plane_through_it`): typed, by name.
            Err(topo::BooleanError::ResultPinchedAtConeApex { .. })
                if pinch && matches!(op, "a ∪ b" | "a ∖ b") => {}
            // The same pinch past the named reading, refused by the
            // at-rest gate's cone loop instead
            // (`work/splitdoor/a-pinched-cone-loop-reaches-the-gate-unnamed.md`).
            Err(topo::BooleanError::ResultInvalid { errors })
                if pinch
                    && matches!(op, "a ∪ b" | "a ∖ b")
                    && format!("{errors:?}").contains("props_cone_loop_vertex") => {}
            Err(e) => panic!("{what}: {e:?}"),
        }
    }
}

/// The box `x × y × z` turned `turn` about the cone's axis, and its
/// region.
fn turned_box(
    (x, y, z): ((f64, f64), (f64, f64), (f64, f64)),
    turn: f64,
) -> (AtRestBody<f64>, Affine3<f64>) {
    let m = Affine3::rotation_about_axis(NORTH, Vec3::new(0.0, 1.0, 0.0), turn);
    (moved(&topo::test_support::brick(x, y, z, tol()), m), m)
}

/// Holds every op of each cone form against the box `bx` (volume `vb`)
/// turned about the axis by each turn, to `share(V)` of the cone `V`.
fn against_a_box(what: &str, bx: ((f64, f64), (f64, f64), (f64, f64)), share: fn(f64) -> f64) {
    let ((x0, x1), (y0, y1), (z0, z1)) = bx;
    let vb = (x1 - x0) * (y1 - y0) * (z1 - z0);
    for alpha in [30.0f64, 45.0, 60.0] {
        let r = alpha.to_radians().tan();
        for turn in [0.0, 0.4, 2.0] {
            let (b, m) = turned_box(bx, turn);
            for (form, body) in forms(alpha) {
                let c = finished("c", body, tol());
                let (ra, rb) = (
                    move || the_cone(r),
                    move || moved_region(membership::cuboid(bx.0, bx.1, bx.2), m),
                );
                four_ops(
                    &format!("{what}, {alpha}°, turned {turn}, {form}"),
                    true,
                    (&c, &ra, volume(alpha)),
                    (&b, &rb, vb),
                    share(volume(alpha)),
                    (Point3::new(-2.5, -0.25, -2.5), Point3::new(2.5, 1.85, 2.5)),
                );
            }
        }
    }
}

/// **An edge across the apex**: the box `[0, 2] × [0.55, 1] × [−2, 2.3]`,
/// its top `y = 1` holding the apex and its edge along `z` there passing
/// through it, its side `x = 0` holding the axis, its far sides clear of
/// the cone. It holds half the cone above `y = 0.55`, `V·0.45³/2`.
#[test]
fn an_edge_across_the_apex_holds_half_the_cones_top() {
    against_a_box(
        "an edge across the apex",
        ((0.0, 2.0), (0.55, 1.0), (-2.0, 2.3)),
        |v| v * 0.45f64.powi(3) / 2.0,
    );
}

/// **An edge along the axis**: the box `[0, 2] × [0.55, 1.6] × [0, 2.1]`,
/// its edge along `y` the cone's axis, through the apex and inside the
/// nappe below it. It holds a quarter of the cone above `y = 0.55`,
/// `V·0.45³/4`.
#[test]
fn an_edge_along_the_axis_holds_a_quarter_of_the_cones_top() {
    against_a_box(
        "an edge along the axis",
        ((0.0, 2.0), (0.55, 1.6), (0.0, 2.1)),
        |v| v * 0.45f64.powi(3) / 4.0,
    );
}
