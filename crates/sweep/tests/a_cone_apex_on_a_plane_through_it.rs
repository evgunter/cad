//! **A Boolean whose planar face passes through a cone's apex reads the
//! apex as the cone's rulings.** The cone of height 1 about `y`, apex at
//! `(0, 1, 0)`, base radius `tan α` on `y = 0`, its apex reached by one
//! wrap edge (`common::wrap_tip::one_wrap_edge`), so the face's one
//! corner there is its whole turn round the apex. Against the half-space
//! bounded by a plane through the apex, the apex pierces the plane face:
//! the pierce subdivides the turn where the plane's side along the
//! rulings turns (`boolean::sectors::build_sectors`), and a transition
//! sub-sector's germ is the section ruling inside it
//! (`chord_join::apex_section_ruling`). Each op, either way round, holds
//! the cone over its part of the base disc, passes tier 3 and tier 3′
//! over the contacts it records, and
//! places a lattice of points as the analytic region does:
//!
//! - planes holding the axis, one square to the wrap edge's plane and
//!   one holding the wrap edge, which is then a section ruling;
//! - tilted planes, among them the one whose base chord is shorter than
//!   the ruling to its end at 30°, where a rim crossing's nearest site
//!   facing it lies on the other ruling (`boolean::join`'s `facing`);
//! - the same planes against the cone `sweep::revolve` builds, whose seam
//!   meridian runs to the apex;
//! - planes meeting the cone at its apex alone, either side;
//! - wedges of the cone revolved a quarter turn and over a half turn,
//!   whose apex is a corner of the cone face and of the two planar faces
//!   the revolve leaves, against planes holding the axis and tilted off
//!   it.
//!
//! A box whose face holds the apex, cutting the tipped cone's one face
//! along two rulings, leaves a face pinched at the apex, which props
//! cannot yet measure: it refuses `ResultPinchedAtConeApex` by name, and
//! the revolved cone, whose seam parts the lobes, builds. A cylinder wall
//! resting on the apex refuses `CurvedBooleanUnsupported` on the
//! cylinder: the pierce at an apex reads a plane only.
//!
//! A plane tangent to the cone along a ruling is a tangency between
//! distinct carriers, and an undeclared one refuses typed at the
//! ruling's rim end (`CurvedBooleanUnsupported`), never building. The
//! apex against its copy's apex is a vertex pair, whose algebra over a
//! fan of rulings is unbuilt: it refuses `VertexPairAtConeApex` by name.
//! A vertex near a needle cone's axis, off its apex, refuses
//! `VertexNearConeAxis` by name.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::membership::{self, Region};
use crate::common::wrap_tip::{NORTH, one_wrap_edge};
use core::f64::consts::PI;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{finished, revolved_about_y};
use topo::{AtRestBody, BooleanError, BooleanResult, intersect, subtract, union};

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

/// [`cone`] with its apex reached by one wrap edge.
fn tipped(alpha: f64) -> AtRestBody<f64> {
    finished("the tipped cone", one_wrap_edge(cone(alpha), true).0, tol())
}

/// The cone's region, its base radius `r`.
fn the_cone(r: f64) -> Region {
    Box::new(move |p: Point3<f64>| (r * (1.0 - p.y) - p.x.hypot(p.z)).min(p.y).min(1.0 - p.y))
}

/// The half-space `n·(p − NORTH) > 0`, as the cube of side 8 with a face
/// on its plane, the apex at that face's centre.
fn half_space(n: Vec3<f64>) -> AtRestBody<f64> {
    let n = n.normalize();
    let b = topo::test_support::brick((0.0, 8.0), (-4.0, 4.0), (-4.0, 4.0), tol());
    let x = Vec3::new(1.0, 0.0, 0.0);
    let turn = if x.cross(n).norm() < 1e-15 {
        Affine3::rotation_about_axis(
            Point3::origin(),
            Vec3::new(0.0, 0.0, 1.0),
            PI * (1.0 - n.x) / 2.0,
        )
    } else {
        Affine3::rotation_about_axis(
            Point3::origin(),
            x.cross(n).normalize(),
            x.dot(n).clamp(-1.0, 1.0).acos(),
        )
    };
    let m = Affine3::translation(NORTH - Point3::origin()) * turn;
    let moved =
        topo::transform_rigid(&b, &geom_core::test_support::rigid(&m, tol()), tol()).unwrap();
    finished("the half-space", moved, tol())
}

/// The half-space's region, cut to its cube.
fn the_half_space(n: Vec3<f64>) -> Region {
    let n = n.normalize();
    Box::new(move |p: Point3<f64>| {
        let along = n.dot(p - NORTH);
        along
            .min(8.0 - along)
            .min(4.0 - (p - NORTH - n * along).norm())
    })
}

/// The volume of the cone of base radius `r` on the side
/// `n·(p − NORTH) > 0` of a plane through its apex: the cone over that
/// side's part of its base disc, a third of its area.
fn cone_beyond(r: f64, n: Vec3<f64>) -> f64 {
    // On the base, `n·(q − NORTH) = n_x x + n_z z − n_y`: the side is the
    // disc's part beyond the line at signed distance `n_y / |n_xz|`.
    let m = n.x.hypot(n.z);
    let disc = PI * r * r;
    if m < 1e-15 {
        return if n.y < 0.0 { disc / 3.0 } else { 0.0 };
    }
    let d = n.y / m;
    let segment = if d >= r {
        0.0
    } else if d <= -r {
        disc
    } else {
        r * r * (d / r).acos() - d * (r * r - d * d).sqrt()
    };
    segment / 3.0
}

/// Holds the six ops of `c` and `h` to their closed forms and regions,
/// `share` the volume of the cone in the half-space.
fn six(what: &str, (c, r): (&AtRestBody<f64>, f64), (h, n): (&AtRestBody<f64>, Vec3<f64>)) {
    let (vc, vh) = (PI * r * r / 3.0, 512.0);
    let share = cone_beyond(r, n);
    let lattice = (
        Point3::new(-r - 0.25, -0.25, -r - 0.25),
        Point3::new(r + 0.25, 1.25, r + 0.25),
    );
    let both = move || -> Region {
        let (c, h) = (the_cone(r), the_half_space(n));
        Box::new(move |p| c(p).min(h(p)))
    };
    let either = move || -> Region {
        let (c, h) = (the_cone(r), the_half_space(n));
        Box::new(move |p| c(p).max(h(p)))
    };
    let c_less = move || -> Region {
        let (c, h) = (the_cone(r), the_half_space(n));
        Box::new(move |p| c(p).min(-h(p)))
    };
    let h_less = move || -> Region {
        let (c, h) = (the_cone(r), the_half_space(n));
        Box::new(move |p| h(p).min(-c(p)))
    };
    for (op, result, want, region) in [
        ("c ∩ h", intersect(c, h, tol()), share, both()),
        ("h ∩ c", intersect(h, c, tol()), share, both()),
        ("c ∪ h", union(c, h, tol()), vc + vh - share, either()),
        ("h ∪ c", union(h, c, tol()), vc + vh - share, either()),
        ("c ∖ h", subtract(c, h, tol()), vc - share, c_less()),
        ("h ∖ c", subtract(h, c, tol()), vh - share, h_less()),
    ] {
        let what = format!("{what}, {op}");
        match result {
            Ok(BooleanResult::Body(b)) => {
                membership::holds_short_of_tier_3_prime(&what, &b.body, want, &region, lattice, 7);
                // A solid touching the other's face at the apex alone is a
                // contact the result records.
                assert_eq!(
                    topo::validate_pseudomanifold(&b.body, &b.contacts, tol()),
                    Ok(()),
                    "{what}: tier 3′ over the result's contacts"
                );
            }
            Ok(BooleanResult::Empty) => assert!(want.abs() < 1e-12, "{what}: empty, not {want}"),
            Err(e) => panic!("{what}: {e:?}"),
        }
    }
}

/// **Planes holding the axis**, square to the wrap edge's plane and
/// holding the wrap edge, each either way round: each side half the
/// cone.
#[test]
fn a_plane_holding_the_axis_cuts_each_tipped_cone_at_its_closed_form() {
    for alpha in [30.0, 45.0, 60.0] {
        let c = tipped(alpha);
        let r = alpha.to_radians().tan();
        for n in [
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, -1.0),
        ] {
            six(&format!("{alpha}°, n {n:?}"), (&c, r), (&half_space(n), n));
        }
    }
}

/// **Tilted planes through the apex**, among them `(1, 0.3, 0)` at 30°,
/// whose base chord is shorter than the rulings to its ends.
#[test]
fn a_tilted_plane_through_the_apex_cuts_each_tipped_cone_at_its_closed_form() {
    for alpha in [30.0, 45.0, 60.0] {
        let c = tipped(alpha);
        let r = alpha.to_radians().tan();
        for n in [
            Vec3::new(1.0, 0.3, 0.2),
            Vec3::new(-0.4, 0.25, 1.0),
            Vec3::new(1.0, 0.3, 0.0),
            Vec3::new(-1.0, -0.3, 0.1),
        ] {
            six(&format!("{alpha}°, n {n:?}"), (&c, r), (&half_space(n), n));
        }
    }
}

/// [`cone`] as `sweep::revolve` builds it, the form a user gets: its
/// seam meridian runs to the apex, which is a corner of the face on
/// either side of it (`revolve_common::needle`).
fn revolved(alpha: f64) -> AtRestBody<f64> {
    crate::revolve_common::needle(alpha, None, tol())
}

/// **Planes holding the axis cut the revolved cone** as they cut the
/// tipped one: [`a_plane_holding_the_axis_cuts_each_tipped_cone_at_its_closed_form`]
/// on [`revolved`].
#[test]
fn a_plane_holding_the_axis_cuts_each_revolved_cone_at_its_closed_form() {
    for alpha in [30.0, 45.0, 60.0] {
        let c = revolved(alpha);
        let r = alpha.to_radians().tan();
        for n in [
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, -1.0),
        ] {
            six(&format!("{alpha}°, n {n:?}"), (&c, r), (&half_space(n), n));
        }
    }
}

/// **Tilted planes through the apex cut the revolved cone** as they cut
/// the tipped one: [`a_tilted_plane_through_the_apex_cuts_each_tipped_cone_at_its_closed_form`]
/// on [`revolved`].
#[test]
fn a_tilted_plane_through_the_apex_cuts_each_revolved_cone_at_its_closed_form() {
    for alpha in [30.0, 45.0, 60.0] {
        let c = revolved(alpha);
        let r = alpha.to_radians().tan();
        for n in [
            Vec3::new(1.0, 0.3, 0.2),
            Vec3::new(-0.4, 0.25, 1.0),
            Vec3::new(1.0, 0.3, 0.0),
            Vec3::new(-1.0, -0.3, 0.1),
        ] {
            six(&format!("{alpha}°, n {n:?}"), (&c, r), (&half_space(n), n));
        }
    }
}

/// **A cut along two rulings through a wrap-tipped apex refuses the
/// pinch by name**: the box `[−0.3, 0] × [0.95, 1.2] × [−0.3, 0.3]`, its
/// face `x = 0` through the apex, cuts the tipped cone's one face along
/// the two rulings in that plane, leaving one face whose loop is two
/// lobes meeting only at the apex. The body is well formed, but props
/// cannot yet measure such a loop
/// (`work/conearm/props-measures-a-cone-loop-pinched-at-its-apex.md`),
/// so the Boolean refuses `ResultPinchedAtConeApex` before the gate. On
/// the revolved cone the seam parts the lobes into two faces, and every
/// op builds: the box holds half the cone's tip of height `0.05`.
#[test]
fn a_cut_along_two_rulings_through_a_wrap_tipped_apex_refuses_the_pinch_by_name() {
    let (lo, hi) = (Point3::new(-0.3, 0.95, -0.3), Point3::new(0.0, 1.2, 0.3));
    let b = finished(
        "the box",
        topo::test_support::brick((lo.x, hi.x), (lo.y, hi.y), (lo.z, hi.z), tol()),
        tol(),
    );
    let the_box = move |p: Point3<f64>| {
        (p.x - lo.x)
            .min(hi.x - p.x)
            .min(p.y - lo.y)
            .min(hi.y - p.y)
            .min(p.z - lo.z)
            .min(hi.z - p.z)
    };
    for alpha in [15.0f64, 30.0, 45.0] {
        let r = alpha.to_radians().tan();
        let t = tipped(alpha);
        for (op, result) in [
            ("t ∪ b", union(&t, &b, tol())),
            ("b ∪ t", union(&b, &t, tol())),
            ("t ∖ b", subtract(&t, &b, tol())),
        ] {
            assert!(
                matches!(result, Err(BooleanError::ResultPinchedAtConeApex { .. })),
                "{alpha}°, tipped, {op}: {result:?}"
            );
        }
        let c = revolved(alpha);
        let (vc, vb) = (PI * r * r / 3.0, 0.3 * 0.25 * 0.6);
        let tip = PI * (r * 0.05).powi(2) * 0.05 / 6.0;
        let lattice = (
            Point3::new(-r - 0.25, -0.25, -r - 0.25),
            Point3::new(r + 0.25, 1.25, r + 0.25),
        );
        let cone = move || the_cone(r);
        for (op, result, want, region) in [
            (
                "c ∪ b",
                union(&c, &b, tol()),
                vc + vb - tip,
                Box::new(move |p| cone()(p).max(the_box(p))) as Region,
            ),
            (
                "b ∪ c",
                union(&b, &c, tol()),
                vc + vb - tip,
                Box::new(move |p| cone()(p).max(the_box(p))),
            ),
            (
                "c ∖ b",
                subtract(&c, &b, tol()),
                vc - tip,
                Box::new(move |p| cone()(p).min(-the_box(p))),
            ),
        ] {
            let what = format!("{alpha}°, revolved, {op}");
            match result {
                Ok(BooleanResult::Body(body)) => {
                    membership::holds(&what, &body.body, want, &region, lattice, 7);
                }
                other => panic!("{what}: {other:?}"),
            }
        }
    }
}

/// **A cylinder wall resting on the apex refuses by name**: the rod of
/// radius `ρ` along `z`, its wall's lowest ruling through the apex, the
/// cone wholly below it, either cone and either rod. The pierce at the
/// apex reads the pierced face's plane, and a curved face's section of
/// the cone there curves off the ruling its tangent plane cuts, which
/// nothing reads: `CurvedBooleanUnsupported` on the cylinder, before the
/// extent fallback refuses the pose unnamed.
#[test]
fn a_cylinder_resting_on_a_cone_apex_refuses_its_curved_wall() {
    for alpha in [30.0, 45.0] {
        for (form, c) in [("tipped", tipped(alpha)), ("revolved", revolved(alpha))] {
            for rho in [0.5, 2.0] {
                let rod = finished(
                    "the rod",
                    crate::common::cavity::rod(Point2::new(0.0, 1.0 + rho), rho, -1.0, 1.0),
                    tol(),
                );
                for (op, r) in [
                    ("c ∪ r", union(&c, &rod, tol())),
                    ("c ∩ r", intersect(&c, &rod, tol())),
                    ("c ∖ r", subtract(&c, &rod, tol())),
                    ("r ∖ c", subtract(&rod, &c, tol())),
                ] {
                    assert!(
                        matches!(
                            r,
                            Err(BooleanError::CurvedBooleanUnsupported {
                                kind: geom::SurfaceKind::Cylinder,
                                ..
                            })
                        ),
                        "{alpha}°, {form}, ρ {rho}, {op}: {r:?}"
                    );
                }
            }
        }
    }
}

/// **A plane meeting the cone at its apex alone**, from above and below
/// and tilted: the cone lies wholly on one side.
#[test]
fn a_plane_meeting_a_tipped_cone_at_its_apex_alone_keeps_it_whole() {
    for alpha in [30.0, 60.0] {
        let c = tipped(alpha);
        let r = alpha.to_radians().tan();
        for n in [
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.2, 1.0, 0.1),
            Vec3::new(0.0, -1.0, 0.0),
        ] {
            six(&format!("{alpha}°, n {n:?}"), (&c, r), (&half_space(n), n));
        }
    }
}

/// **A plane tangent to the cone along a ruling refuses typed**, from
/// either side and at the wrap edge's ruling or another: an undeclared
/// tangency between distinct carriers.
#[test]
fn a_plane_grazing_a_tipped_cone_along_a_ruling_refuses_typed() {
    for alpha in [30.0f64, 60.0] {
        let a = alpha.to_radians();
        let c = tipped(alpha);
        for phi in [0.0f64, 1.0, PI] {
            let along = Vec3::new(a.cos() * phi.cos(), a.sin(), a.cos() * phi.sin());
            for n in [along, -along] {
                let h = half_space(n);
                for (op, r) in [
                    ("c ∩ h", intersect(&c, &h, tol())),
                    ("c ∪ h", union(&c, &h, tol())),
                    ("c ∖ h", subtract(&c, &h, tol())),
                    ("h ∖ c", subtract(&h, &c, tol())),
                ] {
                    assert!(
                        matches!(
                            r,
                            Err(BooleanError::CurvedBooleanUnsupported {
                                kind: geom::SurfaceKind::Cone,
                                ..
                            })
                        ),
                        "{alpha}°, φ {phi}, n {n:?}, {op}: {r:?}"
                    );
                }
            }
        }
    }
}

/// **The apex against its copy's apex refuses the vertex pair by name**:
/// the copy turned over about the apex, the two cones meeting there
/// alone.
#[test]
fn a_tipped_cones_apex_on_its_copys_refuses_the_vertex_pair_by_name() {
    let c = tipped(45.0);
    let over = Affine3::translation(Vec3::new(0.0, 2.0, 0.0))
        * Affine3::rotation_about_axis(Point3::origin(), Vec3::new(1.0, 0.0, 0.0), PI);
    let copy = topo::transform_rigid(
        &one_wrap_edge(cone(45.0), true).0,
        &geom_core::test_support::rigid(&over, tol()),
        tol(),
    )
    .unwrap();
    let copy = finished("the copy", copy, tol());
    for (op, r) in [
        ("c ∪ copy", union(&c, &copy, tol())),
        ("c ∩ copy", intersect(&c, &copy, tol())),
        ("copy ∖ c", subtract(&copy, &c, tol())),
    ] {
        assert!(
            matches!(r, Err(BooleanError::VertexPairAtConeApex { .. })),
            "{op}: {r:?}"
        );
    }
}

/// The 3° needle (`revolve_common::needle`, two meridians at its apex)
/// against the block below the plane through the point `k·ε` down its
/// seam ruling, the plane's normal tilted `tilt` off the axis toward
/// azimuth 0.
fn needle_and_block(
    k: f64,
    tilt: f64,
) -> (AtRestBody<f64>, AtRestBody<f64>, Point3<f64>, Vec3<f64>) {
    use geom_core::{Band, UnitVec3, linalg::RigidMotion};
    let a = 3f64.to_radians();
    let band = Band::linear(tol()).unwrap();
    let foot = NORTH + Vec3::new(a.sin(), -a.cos(), 0.0) * (k * tol().eps());
    let n = Vec3::new(tilt.sin(), tilt.cos(), 0.0);
    let block = sweep::test_support::brick((-2.0, 2.0), (-3.0, 0.0), (-2.0, 2.0), tol());
    let turn = RigidMotion::rotation_then_translation(
        UnitVec3::new(Vec3::new(0.0, 0.0, -n.x), "the block's turn", band).unwrap(),
        tilt,
        foot - Point3::origin(),
    )
    .unwrap();
    let block = finished(
        "the block",
        topo::transform_rigid(&block, &turn, tol()).unwrap(),
        tol(),
    );
    (
        crate::revolve_common::needle(3.0, None, tol()),
        block,
        foot,
        n,
    )
}

/// **A needle's vertex in the band of its axis, off its apex, refuses by
/// name**: the block's plane through the point `40ε` down the seam cuts
/// the needle's far side `≈ 6ε` from its axis, a vertex whose tangent
/// plane reads ill-conditioned.
#[test]
fn a_needle_vertex_near_its_axis_refuses_by_name() {
    let (needle, block, ..) = needle_and_block(40.0, 1.2);
    let r = intersect(&needle, &block, tol());
    assert!(
        matches!(r, Err(BooleanError::VertexNearConeAxis { .. })),
        "{r:?}"
    );
}

/// **A needle's apex in the band of the block's top builds**: the plane
/// through the point `2ε` down its seam, tilted off every ruling, holds
/// the apex to the band, and the needle lies wholly below it.
#[test]
fn a_needle_apex_in_the_band_of_a_plane_builds() {
    let (needle, block, foot, n) = needle_and_block(2.0, 1.2);
    let r = 3f64.to_radians().tan();
    let lattice = (
        Point3::new(-0.07, -0.05, -0.07),
        Point3::new(0.07, 1.05, 0.07),
    );
    let region = {
        let c = the_cone(r);
        move |p: Point3<f64>| c(p).min(-n.dot(p - foot))
    };
    match intersect(&needle, &block, tol()) {
        // An 8³ lattice: at ε 1e-6 the 9³ one holds a point whose ray
        // `point_in_solid` reads in band of the needle's wall, and refuses.
        Ok(BooleanResult::Body(b)) => {
            membership::holds(
                "needle ∩ block",
                &b.body,
                PI * r * r / 3.0,
                &region,
                lattice,
                8,
            );
        }
        other => panic!("needle ∩ block: {other:?}"),
    }
    let rest = subtract(&needle, &block, tol());
    assert!(
        matches!(rest, Ok(BooleanResult::Empty)),
        "needle ∖ block: {rest:?}"
    );
}

/// The overlap of the arcs `[a0, a1]` and `[b0, b1]` of the circle, each
/// under a full turn: its measure, and how many arcs it is.
fn arc_overlap((a0, a1): (f64, f64), (b0, b1): (f64, f64)) -> (f64, usize) {
    (-2..=2)
        .map(|k| {
            let s = f64::from(k) * 2.0 * PI;
            (a1.min(b1 + s) - a0.max(b0 + s)).max(0.0)
        })
        .filter(|&m| m > 0.0)
        .fold((0.0, 0), |(m, n), arc| (m + arc, n + 1))
}

/// The part of the unit disc's sector of azimuths `[0, θ]` beyond the
/// line `ρ cos(φ + β) = d`, `d ≥ 0` (azimuths in the plane
/// `(X, Y) = (x, −z)`): its area, and how many arcs of azimuth reach it.
/// Along azimuth `φ` the ray is beyond from `ρ = d / cos(φ + β)` to the
/// rim, where `cos(φ + β) > d`, so each arc `[φ₀, φ₁]` of those holds
/// `½(φ₁ − φ₀) − ½d²(tan(φ₁ + β) − tan(φ₀ + β))`.
fn sector_beyond(theta: f64, beta: f64, d: f64) -> (f64, usize) {
    let half = d.acos();
    (-2..=2)
        .map(|k| {
            let s = f64::from(k) * 2.0 * PI;
            (0.0f64.max(-beta - half + s), theta.min(-beta + half + s))
        })
        .filter(|(lo, hi)| hi > lo)
        .fold((0.0, 0), |(area, n), (lo, hi)| {
            let arc = 0.5 * (hi - lo) - 0.5 * d * d * ((hi + beta).tan() - (lo + beta).tan());
            (area + arc, n + 1)
        })
}

/// **A wedge of the cone cut through its apex holds its closed form**:
/// the cone of radius and height 1 revolved through `θ`, a quarter turn
/// and over a half turn, its apex a corner of the cone face and the two
/// planar faces the revolve leaves. Against the half-space beyond a
/// plane through the apex, holding the axis or tilted off it by a lift
/// `0.3` along it, the cut keeps the cone over the base's sector beyond
/// the plane's trace ([`sector_beyond`]): a third of its area, in one
/// lobe per arc of azimuth. Two lobes touch along the axis or at the
/// apex, and the census cannot yet clear their curved faces of two
/// shells (`CensusUndecidable`,
/// `work/restread/census-cross-solid-curved-pairs-undecidable-on-shell-results.md`).
#[test]
fn a_wedge_of_the_cone_cut_through_its_apex_holds_its_closed_form() {
    let band = geom_core::Band::linear(tol()).unwrap();
    for theta in [PI / 2.0, 4.5] {
        let wedge = finished(
            "the wedge",
            revolved_about_y(
                vec![
                    (Point2::new(0.0, 0.0), 0.0),
                    (Point2::new(1.0, 0.0), 0.0),
                    (Point2::new(0.0, 1.0), 0.0),
                ],
                Revolution::Partial(theta),
                tol(),
            ),
            tol(),
        );
        // The revolve sweeps from `+x` toward `−z`: azimuths in the
        // plane `(X, Y) = (x, −z)`.
        assert_eq!(
            topo::point_in_solid(&wedge, Point3::new(0.29, 0.2, -0.09), band, tol()),
            Ok(topo::SolidContainment::In),
            "θ {theta}: the wedge sweeps toward −z"
        );
        let in_wedge = move |p: Point3<f64>| {
            let (start, end) = (-p.z, theta.sin() * p.x + theta.cos() * p.z);
            let side = if theta <= PI {
                start.min(end)
            } else {
                start.max(end)
            };
            (1.0 - p.y - p.x.hypot(p.z)).min(p.y).min(side)
        };
        for (beta, lift) in [0.0f64, 1.0, 2.5, 4.0]
            .into_iter()
            .flat_map(|beta| [(beta, 0.0), (beta, 0.3)])
        {
            let n = Vec3::new(beta.cos(), lift, beta.sin());
            let h = half_space(n);
            // On the base `n·(q − NORTH) = ρ cos(φ + β) − lift`: beyond the
            // line at `d = lift`, and short of it the rest of the sector.
            let (area, lobes) = sector_beyond(theta, beta, lift);
            let lobes_left = if lift == 0.0 {
                arc_overlap((0.0, theta), (-beta + PI / 2.0, -beta + 1.5 * PI)).1
            } else {
                // A tilted trace leaves the sector short of it whole round
                // the apex: one lobe.
                1
            };
            let psi = 2.0 * area;
            let beyond = move |p: Point3<f64>| n.dot(p - NORTH);
            let lattice = (Point3::new(-1.2, -0.2, -1.2), Point3::new(1.2, 1.2, 1.2));
            for (op, r, want, pieces, region) in [
                (
                    "w ∩ h",
                    intersect(&wedge, &h, tol()),
                    psi / 6.0,
                    lobes,
                    Box::new(move |p| in_wedge(p).min(beyond(p))) as Region,
                ),
                (
                    "w ∖ h",
                    subtract(&wedge, &h, tol()),
                    (theta - psi) / 6.0,
                    lobes_left,
                    Box::new(move |p| in_wedge(p).min(-beyond(p))),
                ),
            ] {
                let what = format!("θ {theta}, β {beta}, lift {lift}, {op}");
                match r {
                    Ok(BooleanResult::Body(b)) => {
                        membership::holds_short_of_tier_3_prime(
                            &what, &b.body, want, &region, lattice, 7,
                        );
                        assert_eq!(b.body.shells().count(), pieces, "{what}: its lobes");
                        // Two lobes touch along the axis, curved faces of two
                        // parts the census cannot yet clear.
                        let census = topo::validate_pseudomanifold(&b.body, &b.contacts, tol());
                        match pieces {
                            1 => assert_eq!(census, Ok(()), "{what}: tier 3′"),
                            _ => assert!(
                                census.as_ref().is_err_and(|errors| errors.iter().all(
                                    |e| matches!(
                                        e,
                                        topo::ValidationError::CensusUndecidable {
                                            reason: topo::Undecidable::MayOverlap,
                                            ..
                                        }
                                    )
                                )),
                                "{what}: tier 3′ {census:?}"
                            ),
                        }
                    }
                    Ok(BooleanResult::Empty) => assert!(want < 1e-12, "{what}: empty, not {want}"),
                    Err(e) => panic!("{what}: {e:?}"),
                }
            }
        }
    }
}
