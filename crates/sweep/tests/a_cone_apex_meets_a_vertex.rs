//! **A Boolean whose operands meet at a cone's apex vertex reads the
//! apex as its rulings, in the vertex–vertex lane.** The cone of height
//! 1 about `y`, apex at `(0, 1, 0)`, base radius `tan α` on `y = 0`, in
//! both forms a user meets: its apex reached by one wrap edge
//! (`common::wrap_tip::one_wrap_edge`, the face's one corner there its
//! whole turn) and revolved by `sweep::revolve` (two faces meeting along
//! its seam ruling). Each op, either way round, holds its closed form,
//! tiers 3 and 3′ over the contacts it records, and a lattice of points
//! the analytic region places:
//!
//! - **the cone against its copy**: two fans of one cone, a coincidence
//!   the carrier ladder settles (`boolean::fan::fans_overlap`);
//! - **its copy turned over about the apex**: fans of one double cone
//!   on opposite nappes, which meet at the apex alone, read there as a
//!   touch of the two faces at a vertex both hold;
//! - **an hourglass**, two cones tip to tip on one axis with different
//!   apertures, or turned apart: their nappes meet at the apex alone, and
//!   each cone's rulings ending at the other's apex meet it there alone;
//! - **a box's corner at the apex**, its two faces holding the axis
//!   crossing the cone along two rulings each, turned about the axis so
//!   that the rulings fall inside the fan's quarters as well as on them:
//!   the box holds the cone's quadrant;
//! - **two cones through one apex crossing along two common rulings**,
//!   the copy turned about the apex, whose plane the join rides
//!   (`boolean::join`'s `KindArm::Apex`): the four ops agree with one
//!   another and with the region, and the intersection with its
//!   integral over the directions both cones hold.
//!
//! A result of two parts touching at the apex or along a ruling holds
//! tier 3′ short of the census's cross-solid backstop, which cannot yet
//! clear curved faces of two shells.
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
fn four_ops(what: &str, a: Operand, b: Operand, share: f64, lattice: (Point3<f64>, Point3<f64>)) {
    four_ops_pinching(what, a, b, share, lattice, &[]);
}

/// [`four_ops`], but each op named in `pinched` refuses
/// `ResultPinchedAtConeApex` by name, and every other op builds and is
/// held: a cone face cut along two rulings on both sides of its wrap seam
/// is two lobes meeting only at the apex, which props cannot yet measure
/// (`work/conearm/props-measures-a-cone-loop-pinched-at-its-apex.md`).
fn four_ops_pinching(
    what: &str,
    (a, ra, va): Operand,
    (b, rb, vb): Operand,
    share: f64,
    lattice: (Point3<f64>, Point3<f64>),
    pinched: &[&str],
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
        if pinched.contains(&op) {
            assert!(
                matches!(
                    result,
                    Err(topo::BooleanError::ResultPinchedAtConeApex { .. })
                ),
                "{what}: expected to pinch at the apex, got {result:?}"
            );
            continue;
        }
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
            Err(e) => panic!("{what}: {e:?}"),
        }
    }
}

/// The lattice about a cone of base radius `r` and its partner, out to
/// `top` along `y`.
fn about(r: f64, top: f64) -> (Point3<f64>, Point3<f64>) {
    (
        Point3::new(-r - 0.25, -0.25, -r - 0.25),
        Point3::new(r + 0.25, top + 0.25, r + 0.25),
    )
}

/// **A cone against its copy** builds every op: the two fans of one
/// cone overlap, a coincidence settled one carrier.
#[test]
fn a_cone_against_its_copy_builds_every_op() {
    for alpha in [30.0f64, 45.0, 60.0] {
        let r = alpha.to_radians().tan();
        for (form, body) in forms(alpha) {
            let (c, copy) = (
                finished("c", body.clone(), tol()),
                moved(&body, Affine3::identity()),
            );
            let region = || the_cone(r);
            four_ops(
                &format!("{alpha}°, {form}, copy"),
                (&c, &region, volume(alpha)),
                (&copy, &region, volume(alpha)),
                volume(alpha),
                about(r, 1.0),
            );
        }
    }
}

/// The half-turn about `x` through the apex: the cone turned over, its
/// nappe up.
fn turned_over() -> Affine3<f64> {
    Affine3::translation(Vec3::new(0.0, 2.0, 0.0))
        * Affine3::rotation_about_axis(Point3::origin(), Vec3::new(1.0, 0.0, 0.0), PI)
}

/// **A cone against its copy turned over about the apex**, either on its
/// own axis or turned apart, and against a cone of another aperture
/// turned over (an hourglass): the nappes meet at the apex alone, so the
/// union holds both, touching there, and the intersection is empty.
#[test]
fn an_hourglass_touches_at_the_apex_alone() {
    for (alpha, other, tilt) in [
        (30.0f64, 30.0f64, 0.0),
        (45.0, 45.0, 0.0),
        (60.0, 60.0, 0.0),
        (45.0, 30.0, 0.0),
        (30.0, 45.0, 0.0),
        (45.0, 45.0, 0.5),
        (30.0, 45.0, 0.3),
    ] {
        let (r, ro) = (alpha.to_radians().tan(), other.to_radians().tan());
        let m = Affine3::rotation_about_axis(NORTH, Vec3::new(0.0, 0.0, 1.0), tilt) * turned_over();
        for ((form, body), (_, top)) in forms(alpha).into_iter().zip(forms(other)) {
            let c = finished("c", body, tol());
            let up = moved(&top, m);
            let (ra, rb) = (move || the_cone(r), move || moved_region(the_cone(ro), m));
            four_ops(
                &format!("{alpha}° against {other}° turned over, tilted {tilt}, {form}"),
                (&c, &ra, volume(alpha)),
                (&up, &rb, volume(other)),
                0.0,
                about(r.max(ro) + 0.5, 2.0),
            );
        }
    }
}

/// **A box's corner at the apex**, its faces `x = 0` and `z = 0` holding
/// the axis and its top `y = 1` through the apex alone, its far faces
/// clear of the cone: it holds the cone's quadrant, a quarter of the
/// cone. Turned about the axis, its faces' rulings fall inside the fan's
/// quarters rather than on their bounds.
#[test]
fn a_box_corner_at_the_apex_holds_the_cones_quadrant() {
    let block = topo::test_support::brick((0.0, 2.0), (-1.0, 1.0), (0.0, 2.0), tol());
    for alpha in [30.0f64, 45.0, 60.0] {
        let r = alpha.to_radians().tan();
        for turn in [0.0, 0.4, 2.0] {
            let m = Affine3::rotation_about_axis(NORTH, Vec3::new(0.0, 1.0, 0.0), turn);
            let corner = moved(&block, m);
            for (form, body) in forms(alpha) {
                let c = finished("c", body, tol());
                let (ra, rb) = (
                    move || the_cone(r),
                    move || {
                        moved_region(membership::cuboid((0.0, 2.0), (-1.0, 1.0), (0.0, 2.0)), m)
                    },
                );
                four_ops(
                    &format!("{alpha}°, turned {turn}, {form}"),
                    (&c, &ra, volume(alpha)),
                    (&corner, &rb, 8.0),
                    volume(alpha) / 4.0,
                    (Point3::new(-2.5, -1.25, -2.5), Point3::new(2.5, 1.25, 2.5)),
                );
            }
        }
    }
}

/// The volume both cones of half-angle `alpha` (radians) hold, each of
/// height 1 from their common apex along its unit axis `a1`, `a2`
/// (toward its nappe): the integral over the directions `d` within
/// `alpha` of both axes of `⅓ min(1/(d·a1), 1/(d·a2))³`, by the
/// midpoint rule over the first cone's cap.
fn shared_volume(alpha: f64, a1: Vec3<f64>, a2: Vec3<f64>) -> f64 {
    let n = 2000;
    let (dt, dp) = (alpha / f64::from(n), 2.0 * PI / f64::from(n));
    let u = Vec3::new(1.0, 0.0, 0.0);
    let u = (u - a1 * u.dot(a1)).normalize();
    let v = a1.cross(u);
    let mut sum = 0.0;
    for i in 0..n {
        let t = (f64::from(i) + 0.5) * dt;
        for j in 0..n {
            let p = (f64::from(j) + 0.5) * dp;
            let d = a1 * t.cos() + (u * p.cos() + v * p.sin()) * t.sin();
            if d.dot(a2) <= alpha.cos() {
                continue;
            }
            let reach = (1.0 / d.dot(a1)).min(1.0 / d.dot(a2));
            sum += reach.powi(3) / 3.0 * t.sin() * dt * dp;
        }
    }
    sum
}

/// **Two cones through one apex cross along two common rulings**: the
/// 45° cone and its copy turned `0.6` about `z` through the apex. The
/// four ops agree with one another and with the regions at the lattice,
/// and the intersection with [`shared_volume`].
#[test]
fn two_cones_through_one_apex_cross_along_their_common_rulings() {
    let alpha = 45.0f64;
    let r = alpha.to_radians().tan();
    let m = Affine3::rotation_about_axis(NORTH, Vec3::new(0.0, 0.0, 1.0), 0.6);
    let down = Vec3::new(0.0, -1.0, 0.0);
    let want = shared_volume(alpha.to_radians(), down, m.transform_vec(down));
    for (form, body) in forms(alpha) {
        let c = finished("c", body.clone(), tol());
        let turned = moved(&body, m);
        let share = match intersect(&c, &turned, tol()) {
            Ok(BooleanResult::Body(b)) => {
                topo::test_support::pinned_volume("∩", &b.body, tol()).volume
            }
            other => panic!("{form}: the intersection {other:?}"),
        };
        assert!(
            (share - want).abs() < 2e-4,
            "{form}: ∩ {share} against the integral {want}"
        );
        let (ra, rb) = (move || the_cone(r), move || moved_region(the_cone(r), m));
        four_ops(
            &format!("{form}, turned 0.6"),
            (&c, &ra, volume(alpha)),
            (&turned, &rb, volume(alpha)),
            share,
            (
                Point3::new(-1.75, -0.5, -1.75),
                Point3::new(1.75, 1.25, 1.75),
            ),
        );
    }
}

/// **A small box cornered at the apex, its side holding the wrap seam**:
/// `[0, 0.3] × [0.7, 1] × [−0.3, 0]`, its top holding the apex and its
/// side `z = 0` the tipped cone's seam ruling at azimuth `0`. The kept
/// cone face's loop passes the apex once, between the seam vertex's two
/// copies, which props reads from the edge leaving the apex
/// (`geom_brep::props`' `cone_loop_winding`, "Through the apex"). Every op
/// builds at its closed form, the box holding a quarter of the cone
/// above `y = 0.7`: `V·0.3³/4`.
#[test]
fn a_small_box_whose_side_holds_the_wrap_seam_builds_every_op() {
    let (x, y, z) = ((0.0, 0.3), (0.7, 1.0), (-0.3, 0.0));
    let block = moved(
        &topo::test_support::brick(x, y, z, tol()),
        Affine3::identity(),
    );
    for alpha in [10.0f64, 30.0] {
        let r = alpha.to_radians().tan();
        for (form, body) in forms(alpha) {
            let c = finished("c", body, tol());
            let (ra, rb) = (move || the_cone(r), move || membership::cuboid(x, y, z));
            four_ops(
                &format!("{alpha}°, {form}, a small box at the seam"),
                (&c, &ra, volume(alpha)),
                (&block, &rb, 0.027),
                volume(alpha) * 0.3f64.powi(3) / 4.0,
                about(r.max(0.3), 1.0),
            );
        }
    }
}

/// **A box whose side holds the 80° cone's wrap seam refuses typed**, in
/// every op either way round (the R2 review's `x90` pose): the box
/// `[0, 5] × [−5, 0] × [0, 5]` turned a quarter about `x` and cornered
/// at the apex, its side holding the axis and the wrap edge's ruling. The
/// vertex pair records the wrap edge's bound `On` the box's side, which
/// the edge rules settle only for a planar sector: `VertexPairAtConeApex`
/// on the cone's face, never the kernel's own fault. The other ops reach
/// the 80° cone's hyperbolic section at the box's far faces first
/// (`GermSectionOpenConic`, the join's open-conic limit).
#[test]
fn a_box_whose_side_holds_the_80_degree_wrap_seam_refuses_typed() {
    let block = topo::test_support::brick((0.0, 5.0), (-5.0, 0.0), (0.0, 5.0), tol());
    let m = Affine3::translation(Vec3::new(0.0, 1.0, 0.0))
        * Affine3::rotation_about_axis(Point3::origin(), Vec3::new(1.0, 0.0, 0.0), PI / 2.0);
    let b = moved(&block, m);
    let c = finished("c", one_wrap_edge(cone(80.0), true).0, tol());
    // Each op names its refusal: where the box's part is kept against the
    // cone's face (`box ∖ cone`, and every op read box first but
    // `box ∖ cone`), the fan bound on the box's side; the others reach
    // the 80° cone's hyperbolic section at the box's far faces first.
    let pair = |r: &Result<BooleanResult<f64>, topo::BooleanError>| match r {
        Err(topo::BooleanError::VertexPairAtConeApex { .. }) => "vertex pair",
        Err(topo::BooleanError::GermSectionOpenConic { .. }) => "open conic",
        _ => "other",
    };
    for (order, x, y, want) in [
        (
            "cone first",
            &c,
            &b,
            ["open conic", "open conic", "open conic", "vertex pair"],
        ),
        (
            "box first",
            &b,
            &c,
            ["vertex pair", "vertex pair", "vertex pair", "open conic"],
        ),
    ] {
        let got = [
            union(x, y, tol()),
            intersect(x, y, tol()),
            subtract(x, y, tol()),
            subtract(y, x, tol()),
        ];
        let got = got.each_ref().map(pair);
        assert_eq!(got, want, "{order}: ∪, ∩, ∖, ∖ back");
    }
}

/// The volume of the cone of half-angle `alpha` (degrees, apex at
/// `NORTH`, height 1 down `y`) inside the trihedral wedge at its apex
/// spanned by the orthonormal `edges`: the integral over the cone's
/// directions `d` with `d·eᵢ ≥ 0` of `⅓ (1/(d·â))³`, by the midpoint rule
/// over its cap.
fn wedge_volume(alpha: f64, edges: [Vec3<f64>; 3]) -> f64 {
    let a = Vec3::new(0.0, -1.0, 0.0);
    let (u, v) = (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
    let n = 2000;
    let (dt, dp) = (alpha.to_radians() / f64::from(n), 2.0 * PI / f64::from(n));
    let mut sum = 0.0;
    for i in 0..n {
        let t = (f64::from(i) + 0.5) * dt;
        for j in 0..n {
            let p = (f64::from(j) + 0.5) * dp;
            let d = a * t.cos() + (u * p.cos() + v * p.sin()) * t.sin();
            if edges.iter().all(|e| d.dot(*e) >= 0.0) {
                sum += (1.0 / d.dot(a)).powi(3) / 3.0 * t.sin() * dt * dp;
            }
        }
    }
    sum
}

/// **A box cornered at the apex, turned off the axis**: the box
/// `[0, 5] × [−5, 0] × [0, 5]` cornered at the apex and turned `0.7`
/// about `(1, 0.3, −0.5)`, so that none of its faces through the apex
/// holds the axis and each face's side of the fan turns inside the
/// fan's quarters. Each flat sector is cut at the fan's extreme rulings
/// along it, and each fan sector at the planes' (`boolean::sectors`'
/// `partner_cuts`). Its far faces clear the cone, so the box holds the
/// cone inside its trihedral wedge, [`wedge_volume`].
#[test]
fn a_box_cornered_at_the_apex_off_the_axis_holds_the_cone_in_its_wedge() {
    let block = topo::test_support::brick((0.0, 5.0), (-5.0, 0.0), (0.0, 5.0), tol());
    let m = Affine3::rotation_about_axis(NORTH, Vec3::new(1.0, 0.3, -0.5).normalize(), 0.7)
        * Affine3::translation(Vec3::new(0.0, 1.0, 0.0));
    let b = moved(&block, m);
    let edges = [
        m.transform_vec(Vec3::new(1.0, 0.0, 0.0)),
        m.transform_vec(Vec3::new(0.0, -1.0, 0.0)),
        m.transform_vec(Vec3::new(0.0, 0.0, 1.0)),
    ];
    for alpha in [30.0f64, 45.0] {
        let r = alpha.to_radians().tan();
        let want = wedge_volume(alpha, edges);
        for (form, body) in forms(alpha) {
            let c = finished("c", body, tol());
            let share = match intersect(&c, &b, tol()) {
                Ok(BooleanResult::Body(x)) => {
                    topo::test_support::pinned_volume("∩", &x.body, tol()).volume
                }
                other => panic!("{alpha}°, {form}: the intersection {other:?}"),
            };
            assert!(
                (share - want).abs() < 2e-4,
                "{alpha}°, {form}: ∩ {share} against the integral {want}"
            );
            let (ra, rb) = (
                move || the_cone(r),
                move || moved_region(membership::cuboid((0.0, 5.0), (-5.0, 0.0), (0.0, 5.0)), m),
            );
            four_ops(
                &format!("{alpha}°, {form}, a box off the axis"),
                (&c, &ra, volume(alpha)),
                (&b, &rb, 125.0),
                share,
                (Point3::new(-2.0, -0.25, -2.0), Point3::new(2.0, 1.75, 2.0)),
            );
        }
    }
}

/// **An edge through the apex, not ending there, refuses at the apex**
/// where the sweep reads it before the vertex pair can: the box
/// `[0, 2] × [0.55, 1] × [−2, 2.3]`, its top holding the apex and its edge
/// along `z` there passing through it. Read box first, that edge's root
/// at the apex is no end's, and `box ∖ cone` refuses
/// `CrossingAtConeApex` (`boolean::reduce::line_cone_roots`) rather than
/// read a crossing there. Cone first, the apex vertex splits the edge
/// before it is swept, and the op builds.
#[test]
fn an_edge_through_the_apex_refuses_at_the_apex_box_first() {
    let b = moved(
        &topo::test_support::brick((0.0, 2.0), (0.55, 1.0), (-2.0, 2.3), tol()),
        Affine3::identity(),
    );
    for (form, body) in forms(30.0) {
        let c = finished("c", body, tol());
        let r = subtract(&b, &c, tol());
        assert!(
            matches!(r, Err(topo::BooleanError::CrossingAtConeApex { .. })),
            "{form}: box ∖ cone {r:?}"
        );
    }
}

/// The volume both cones hold, of half-angles `alpha1`, `alpha2`
/// (degrees), each of height 1 from their common apex along its unit
/// axis `a1`, `a2` (toward its nappe): [`shared_volume`]'s integral over
/// the first cone's cap, the second read at its own aperture.
fn shared_volume_of(alpha1: f64, a1: Vec3<f64>, alpha2: f64, a2: Vec3<f64>) -> f64 {
    let (h1, c2) = (alpha1.to_radians(), alpha2.to_radians().cos());
    let n = 1500;
    let (dt, dp) = (h1 / f64::from(n), 2.0 * PI / f64::from(n));
    let u = Vec3::new(1.0, 0.0, 0.0);
    let u = (u - a1 * u.dot(a1)).normalize();
    let v = a1.cross(u);
    let mut sum = 0.0;
    for i in 0..n {
        let t = (f64::from(i) + 0.5) * dt;
        for j in 0..n {
            let p = (f64::from(j) + 0.5) * dp;
            let d = a1 * t.cos() + (u * p.cos() + v * p.sin()) * t.sin();
            if d.dot(a2) <= c2 {
                continue;
            }
            let reach = (1.0 / d.dot(a1)).min(1.0 / d.dot(a2));
            sum += reach.powi(3) / 3.0 * t.sin() * dt * dp;
        }
    }
    sum
}

/// **Two cones through one apex, across the families of pose the R2
/// probe swept**: apertures `10/10`, `20/50`, `30/30` and `45/30`, the
/// second cone turned about the apex
///
/// - about `z` by half the sum of the half-angles (crossing mid-fan);
/// - about `z` by `0.02` (near-coaxial);
/// - about `z` by the difference of the half-angles plus a tenth of their
///   sum (just past the inner touch);
/// - about the skew axis `(1, 0.3, 1)` by half the sum (crossing off the
///   plane of the axes).
///
/// Both cones tipped, then both revolved: each op at the closed form its
/// intersection's direction integral sets ([`shared_volume_of`]), with
/// the regions at the lattice. The rim germs of the two common rulings
/// must each pair with the apex germ on their own ruling, not with each
/// other across the apex (`boolean::join`'s `two_rulings`).
#[test]
fn two_cones_through_one_apex_build_across_the_pose_families() {
    let down = Vec3::new(0.0, -1.0, 0.0);
    for (alpha1, alpha2) in [(10.0f64, 10.0f64), (20.0, 50.0), (30.0, 30.0), (45.0, 30.0)] {
        let sum = (alpha1 + alpha2).to_radians();
        let diff = (alpha1 - alpha2).abs().to_radians();
        let z = Vec3::new(0.0, 0.0, 1.0);
        for (pose, axis, turn) in [
            ("cross-mid", z, 0.5 * sum),
            ("near-coaxial", z, 0.02),
            ("near the inner touch", z, diff + 0.1 * sum),
            (
                "skew-cross",
                Vec3::new(1.0, 0.3, 1.0).normalize(),
                0.5 * sum,
            ),
        ] {
            let m = Affine3::rotation_about_axis(NORTH, axis, turn);
            let want = shared_volume_of(alpha1, down, alpha2, m.transform_vec(down));
            let (r1, r2) = (alpha1.to_radians().tan(), alpha2.to_radians().tan());
            for ((form, one), (_, two)) in forms(alpha1).into_iter().zip(forms(alpha2)) {
                let what = format!("{alpha1}/{alpha2}, {pose}, {form}");
                let a = finished("a", one, tol());
                let b = moved(&two, m);
                let share = match intersect(&a, &b, tol()) {
                    Ok(BooleanResult::Body(x)) => {
                        topo::test_support::pinned_volume("∩", &x.body, tol()).volume
                    }
                    Ok(BooleanResult::Empty) => 0.0,
                    Err(e) => panic!("{what}: the intersection {e:?}"),
                };
                assert!(
                    (share - want).abs() < 2e-4 * want.max(1.0),
                    "{what}: ∩ {share} against the integral {want}"
                );
                let (ra, rb) = (move || the_cone(r1), move || moved_region(the_cone(r2), m));
                let reach = r1.max(r2) + 1.0;
                four_ops_pinching(
                    &what,
                    (&a, &ra, volume(alpha1)),
                    (&b, &rb, volume(alpha2)),
                    share,
                    (
                        Point3::new(-reach, -0.5, -reach),
                        Point3::new(reach, 1.25 + reach, reach),
                    ),
                    pinches(alpha1, alpha2, pose, form),
                );
            }
        }
    }
}

/// The ops of [`two_cones_through_one_apex_build_across_the_pose_families`]
/// that keep a cone face cut along two rulings on both sides of its wrap
/// seam, pinched at the apex (refused `ResultPinchedAtConeApex`): the
/// 50° cone's remainder at 20/50 (∪ and b ∖ a) tipped at every crossing
/// pose and revolved skew, and the 45° cone's (∪ and a ∖ b) at 45/30
/// skew. Every other op builds.
fn pinches(alpha1: f64, alpha2: f64, pose: &str, form: &str) -> &'static [&'static str] {
    match ((alpha1 as u32, alpha2 as u32), pose, form) {
        ((20, 50), "cross-mid" | "near the inner touch", "tipped")
        | ((20, 50), "skew-cross", _) => &["a ∪ b", "b ∖ a"],
        ((45, 30), "skew-cross", _) => &["a ∪ b", "a ∖ b"],
        _ => &[],
    }
}
