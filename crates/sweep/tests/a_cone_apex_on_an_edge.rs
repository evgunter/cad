//! **A Boolean whose edge passes through a cone's apex reads the edge's
//! meeting with the apex as the apex vertex's**: the edge's one root
//! there, a double one, is placed on the cone face's apex vertex
//! (`boolean::reduce::line_cone_roots`), splitting the edge there, and
//! the vertex pair reads the halves (`boolean::fan`). The cone of height
//! 1 about `y`, apex at `(0, 1, 0)`, base radius `tan α` on `y = 0`, in
//! both forms: its apex reached by one wrap edge
//! (`common::wrap_tip::one_wrap_edge`) and revolved by `sweep::revolve`.
//! Against a box one of whose edges runs through the apex, turned about
//! the cone's axis by generic turns, every quarter turn and just short
//! of the whole turn (where a box side lies just off the tipped cone's
//! seam), each op either way round (`∩` and `∪` with each operand first,
//! and both differences) holds its closed form, tiers 3 and 3′ over the
//! contacts it records, and a lattice of points the analytic region
//! places:
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
//! `box ∖ cone`, build at every pose. The poses that pinch are named,
//! every aperture alike, so a pose that starts or stops pinching turns
//! its row red.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::membership::{self, Region};
use crate::common::wrap_tip::{NORTH, one_wrap_edge};
use core::f64::consts::{FRAC_PI_2, PI, TAU};
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

/// One operand of [`six_ops`]: the body, its region, its volume.
type Operand<'a> = (&'a AtRestBody<f64>, &'a dyn Fn() -> Region, f64);

/// What one op of a pose is held to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Expect {
    /// It builds, at its closed form and its region.
    Builds,
    /// It refuses `ResultPinchedAtConeApex` by name: the cone's face cut
    /// along two rulings through its apex is left pinched there, which
    /// props cannot yet measure (`a_cone_apex_on_a_plane_through_it`).
    Pinches,
}

/// Holds the six ops of `a` and `b` (`a` the cone), either way round:
/// `∩` and `∪` with each first, `a ∖ b` and `b ∖ a`, to the volumes
/// `share` (their intersection's) sets, and to their regions at the `7³`
/// lattice over `lattice`, with tiers 3 and 3′ over the contacts each
/// result records, or to the refusal `expect` names for the op.
fn six_ops(
    what: &str,
    expect: &dyn Fn(&str) -> Expect,
    (a, ra, va): Operand,
    (b, rb, vb): Operand,
    share: f64,
    lattice: (Point3<f64>, Point3<f64>),
) {
    let both = || -> Region {
        let (x, y) = (ra(), rb());
        Box::new(move |p| x(p).min(y(p)))
    };
    let either = || -> Region {
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
        ("a ∩ b", intersect(a, b, tol()), share, both()),
        ("b ∩ a", intersect(b, a, tol()), share, both()),
        ("a ∪ b", union(a, b, tol()), va + vb - share, either()),
        ("b ∪ a", union(b, a, tol()), va + vb - share, either()),
        ("a ∖ b", subtract(a, b, tol()), va - share, a_less),
        ("b ∖ a", subtract(b, a, tol()), vb - share, b_less),
    ] {
        let what = format!("{what}, {op}");
        match expect(op) {
            Expect::Pinches => {
                assert!(
                    matches!(
                        result,
                        Err(topo::BooleanError::ResultPinchedAtConeApex { .. })
                    ),
                    "{what}: expected to pinch at the apex, got {result:?}"
                );
                continue;
            }
            Expect::Builds => {}
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

/// The box `x × y × z` turned `turn` about the cone's axis, and its
/// region.
fn turned_box(
    (x, y, z): ((f64, f64), (f64, f64), (f64, f64)),
    turn: f64,
) -> (AtRestBody<f64>, Affine3<f64>) {
    let m = Affine3::rotation_about_axis(NORTH, Vec3::new(0.0, 1.0, 0.0), turn);
    (moved(&topo::test_support::brick(x, y, z, tol()), m), m)
}

/// What an op of a pose is held to where the pose pinches (`pose`):
/// the ops keeping the cone's remainder, `∪` either way round and
/// `cone ∖ box`, refuse pinched at the apex; every other op builds.
fn pinched(pose: bool, op: &str) -> Expect {
    if pose && matches!(op, "a ∪ b" | "b ∪ a" | "a ∖ b") {
        Expect::Pinches
    } else {
        Expect::Builds
    }
}

/// The turns about the axis every box is read at: generic ones, the
/// quarter turns, and a ten-thousandth of a radian short of the whole
/// turn, where the box's side lies just off the tipped cone's seam: past
/// the band at every ε row, so every row reads the one pose (a turn
/// inside the band reads the seam's own pose, within the band of it).
const TURNS: [f64; 7] = [0.0, 0.4, FRAC_PI_2, 2.0, PI, 3.0 * FRAC_PI_2, TAU - 1e-4];

/// Holds every op of each cone form against the box `bx` (volume `vb`)
/// turned about the axis by each of [`TURNS`], to `share(V)` of the cone
/// `V`, or to the refusal `expect(α, turn, form, op)` names ([`six_ops`]).
fn against_a_box(
    what: &str,
    bx: ((f64, f64), (f64, f64), (f64, f64)),
    share: fn(f64) -> f64,
    expect: fn(f64, f64, &str, &str) -> Expect,
) {
    against_a_box_at(what, &[30.0, 45.0, 60.0], bx, share, expect);
}

/// [`against_a_box`] at the apertures `alphas` (degrees), the lattice
/// widened to the box.
fn against_a_box_at(
    what: &str,
    alphas: &[f64],
    bx: ((f64, f64), (f64, f64), (f64, f64)),
    share: fn(f64) -> f64,
    expect: fn(f64, f64, &str, &str) -> Expect,
) {
    let ((x0, x1), (y0, y1), (z0, z1)) = bx;
    let vb = (x1 - x0) * (y1 - y0) * (z1 - z0);
    let reach = x1.abs().max(z0.abs()).max(z1.abs()) + 0.5;
    for &alpha in alphas {
        let r = alpha.to_radians().tan();
        for turn in TURNS {
            let (b, m) = turned_box(bx, turn);
            for (form, body) in forms(alpha) {
                let c = finished("c", body, tol());
                let (ra, rb) = (
                    move || the_cone(r),
                    move || moved_region(membership::cuboid(bx.0, bx.1, bx.2), m),
                );
                six_ops(
                    &format!("{what}, {alpha}°, turned {turn}, {form}"),
                    &|op| expect(alpha, turn, form, op),
                    (&c, &ra, volume(alpha)),
                    (&b, &rb, vb),
                    share(volume(alpha)),
                    (
                        Point3::new(-reach, -0.25, -reach),
                        Point3::new(reach, 1.85, reach),
                    ),
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
        // Turned 2 or π, the tipped cone's seam lies outside the box, in
        // the remainder `∪` and `cone ∖ box` keep: the remainder's face
        // runs from the box's one ruling round through the seam to its
        // other, two lobes meeting only at the apex.
        |_, turn, form, op| pinched(form == "tipped" && (turn == 2.0 || turn == PI), op),
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
        // A seam outside the box, in the kept remainder, between the
        // box's two rulings (the remainder's face two lobes meeting only
        // at the apex): the tipped cone's turned 2, π, 3π/2 or just short
        // of the whole turn, the revolved cone's turned 2 or just short.
        |_, turn, form, op| {
            let tipped = [2.0, PI, 3.0 * FRAC_PI_2, TURNS[6]].contains(&turn);
            let revolved = [2.0, TURNS[6]].contains(&turn);
            pinched(if form == "tipped" { tipped } else { revolved }, op)
        },
    );
}

/// **An edge ending a fraction of the band off the apex refuses box
/// first, and builds cone first** (pinned as it stands; filed as
/// `work/splitdoor/a-box-corner-just-off-a-cone-apex-refuses-box-first-edge-undescribed.md`,
/// with the corner poses of the same root). The box
/// `[0, 2] × [0.55, 1] × [z₀, 2.3]`, `z₀ = ±0.3` zero bands, turned `0.4`
/// about the 45° cone's axis: its edge along `x` at `z₀` passes three
/// tenths of a band from the apex. Cone first, the apex vertex is placed
/// against the box's faces, and `∩` holds a quarter of the cone above
/// `y = 0.55` less the slab `z₀` thick, `V·0.45³/4 − z₀·tan α·0.45²/2`,
/// and `cone ∖ box` the rest, each to the band times the cut's area. Box first, `∩` and `box ∖ cone` refuse
/// `EdgeUndescribed`: the section edge beside the apex certifies to no
/// analytic bound (`AnalyticRung3`'s `KernelLimit`).
#[test]
fn an_edge_ending_just_off_the_apex_refuses_box_first_and_builds_cone_first() {
    let zero = geom_core::Band::linear(tol()).unwrap().zero();
    let r = 45f64.to_radians().tan();
    for z0 in [0.3 * zero, -0.3 * zero] {
        let (b, _) = turned_box(((0.0, 2.0), (0.55, 1.0), (z0, 2.3)), 0.4);
        let share = volume(45.0) * 0.45f64.powi(3) / 4.0 - z0 * r * 0.45f64.powi(2) / 2.0;
        for (form, body) in forms(45.0) {
            let what = format!("z₀ {z0:e}, {form}");
            let c = finished("c", body, tol());
            for (op, got, want) in [
                ("cone first ∩", intersect(&c, &b, tol()), share),
                ("cone ∖ box", subtract(&c, &b, tol()), volume(45.0) - share),
            ] {
                let Ok(BooleanResult::Body(body)) = got else {
                    panic!("{what}, {op}: builds, got {got:?}");
                };
                let v = topo::test_support::pinned_volume(op, &body.body, tol());
                // The apex lies within the band of the box's face, which
                // may read it on the face: the result is held to the band
                // times the cut's area (under a square metre).
                assert!(
                    (v.volume - want).abs() <= v.volume_pad + zero,
                    "{what}, {op}: {} against {want}",
                    v.volume
                );
            }
            for (op, got) in [
                ("box first ∩", intersect(&b, &c, tol())),
                ("box ∖ cone", subtract(&b, &c, tol())),
            ] {
                let named = matches!(&got, Err(topo::BooleanError::EdgeUndescribed { .. }))
                    && format!("{got:?}").contains("AnalyticRung3(KernelLimit");
                assert!(
                    named,
                    "{what}, {op}: EdgeUndescribed at the analytic rung, got {got:?}"
                );
            }
        }
    }
}

/// **A box corner a fraction of the band off the apex refuses
/// `CrossingAtConeApex`**, every op either way round, both forms (the
/// refusal's end-to-end row). The box `[0, 2] × [−2, 0] × [0, 2]` turned
/// `1.1` about `(0.3, 0.8, −0.5)`, its corner moved off the apex by a
/// fraction of the zero band: `0.9` of it down or up the axis at 30°,
/// `0.3` of it in toward the axis at 45°. One box edge near the corner
/// passes within the band of the apex, but its two roots there lie
/// `1.3` to `3` zero bands from its end along it, past the band
/// (`boolean::reduce::line_cone_roots`' spread rung): neither the end's
/// incidence with the apex vertex nor a crossing the band places, so the
/// edge refuses at the apex, naming the box as the edge's operand.
#[test]
fn a_box_corner_a_fraction_of_the_band_off_the_apex_refuses_at_the_apex() {
    let zero = tol().eps();
    let rot = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.3, 0.8, -0.5), 1.1);
    for (alpha, off) in [
        (30.0f64, Vec3::new(0.0, -0.9, 0.0)),
        (30.0, Vec3::new(0.0, 0.9, 0.0)),
        (45.0, Vec3::new(1.0, 0.0, 1.0).normalize() * 0.3),
    ] {
        let block = topo::test_support::brick((0.0, 2.0), (-2.0, 0.0), (0.0, 2.0), tol());
        let m =
            Affine3::translation(Vec3::new(0.0, 1.0, 0.0) + rot.transform_vec(off * zero)) * rot;
        let b = moved(&block, m);
        for (form, body) in forms(alpha) {
            let c = finished("c", body, tol());
            for (op, box_is, got) in [
                ("cone ∩ box", topo::Operand::B, intersect(&c, &b, tol())),
                ("box ∩ cone", topo::Operand::A, intersect(&b, &c, tol())),
                ("cone ∪ box", topo::Operand::B, union(&c, &b, tol())),
                ("box ∪ cone", topo::Operand::A, union(&b, &c, tol())),
                ("cone ∖ box", topo::Operand::B, subtract(&c, &b, tol())),
                ("box ∖ cone", topo::Operand::A, subtract(&b, &c, tol())),
            ] {
                assert!(
                    matches!(
                        got,
                        Err(topo::BooleanError::CrossingAtConeApex { operand, .. }) if operand == box_is
                    ),
                    "{alpha}°, {off:?}, {form}, {op}: CrossingAtConeApex on the box's edge, got {got:?}"
                );
            }
        }
    }
}

/// **The 80° cone's top against a box clear of its rim builds** (the
/// direct witness the `x90` row's open-conic refusal leaves wanting):
/// the box `[0, 3] × [0.55, 1] × [−3, 3.3]` across the apex, its top
/// holding the apex and its side the axis, wider than the 80° cone's
/// top above `y = 0.55` (radius `0.45·tan 80° ≈ 2.55`), so its far sides
/// clear it, and no wider: a box edge's reach enters the line lane's
/// root slack squared, which at ε 1e-12 a 6 m box's floor edge, crossing
/// the 80° cone's shallow rulings, does not fit. Every op either way round holds half the cone above
/// `y = 0.55`, `V·0.45³/2`, but the named pinches of the across-the-apex
/// row.
#[test]
fn an_80_degree_cones_top_against_a_box_clear_of_its_rim_builds() {
    against_a_box_at(
        "the 80° cone across the apex",
        &[80.0],
        ((0.0, 3.0), (0.55, 1.0), (-3.0, 3.3)),
        |v| v * 0.45f64.powi(3) / 2.0,
        |_, turn, form, op| pinched(form == "tipped" && (turn == 2.0 || turn == PI), op),
    );
}

