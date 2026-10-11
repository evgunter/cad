//! **A near-tangent ∩'s sliver lump reads its role off its own size.**
//!
//! A prism's corner on a cube's face tilted `d` rad off one of the
//! corner's edges (`crates/sweep/examples/near_tangent_census_probe.rs`,
//! pose `nt e0 a<k> d<d>`): the ∩ keeps a wedge 2 m long and `2·d` m
//! thick at its far end as its own shell. At ε = 1e-12 the wedge is tens
//! of bands thick at `d ≥ 1e-10`, and its certified role is read off the
//! polyhedron of its vertex points. At `d = 1e-11` it is in band, and the
//! enclosure sits inside the band.
//!
//! Wider ε stands these rows down: the wedge is a band or less thick, and
//! the in-band twin is the pose measured at ε = 1e-12.

#![allow(clippy::expect_used, clippy::panic)]

use geom_core::{Point3, Tol};
use topo::{
    AtRestBody, BooleanDecision, BooleanDeclarations, BooleanError, ShellRole, classify_shells,
    intersect_with,
};

use crate::common;

type V3 = [f64; 3];

fn dot(a: V3, b: V3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: V3, b: V3) -> V3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn add(a: V3, b: V3) -> V3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
fn scale(a: V3, s: f64) -> V3 {
    a.map(|c| c * s)
}
fn unit(a: V3) -> V3 {
    scale(a, 1.0 / dot(a, a).sqrt())
}
fn basis(m: V3) -> (V3, V3) {
    let m = unit(m);
    let seed = if m[2].abs() < 0.9 {
        [0.0, 0.0, 1.0]
    } else {
        [1.0, 0.0, 0.0]
    };
    let u = unit(cross(seed, m));
    (u, cross(m, u))
}

/// The probe's prism `profile` (corner `v`, its edge `e` there) and its
/// cube at pose `nt e<·> a<a> d<d>`, in the probe's own arithmetic.
fn corner_pose(
    profile: &[(f64, f64)],
    v: V3,
    e: V3,
    a: u32,
    d: f64,
    tol: Tol,
) -> (AtRestBody<f64>, AtRestBody<f64>) {
    let prism = common::prism::<f64>(profile, 1.0, tol).body;
    let prism = AtRestBody::validate(prism, tol).expect("the prism is at rest");
    let m = cube_axis(e, a, d);
    let (u, w) = basis(m);
    let cube = common::mapped_cube::<f64>(
        move |x, y, z| {
            let (a, b, c) = (-2.0 + 4.0 * x, -2.0 + 4.0 * y, 4.0 * z);
            let p = add(v, add(scale(u, a), add(scale(w, b), scale(m, c))));
            Point3::new(p[0], p[1], p[2])
        },
        tol,
    );
    let cube = AtRestBody::validate(cube, tol).expect("the cube is at rest");
    (prism, cube)
}

/// The axis of the probe's cube at pose `nt e<·> a<a> d<d>` about the
/// corner's edge `e`: the normal of its face through the corner.
fn cube_axis(e: V3, a: u32, d: f64) -> V3 {
    let e = unit(e);
    let (p1, p2) = basis(e);
    let al = std::f64::consts::TAU * (f64::from(a) + 0.25) / 16.0;
    let base = add(scale(p1, al.cos()), scale(p2, al.sin()));
    unit(unit(add(base, scale(e, d))))
}

/// The probe's `notch307` prism ∩ its cube at pose `nt e0 a<a> d<d>`.
fn notch307_meet(a: u32, d: f64, tol: Tol) -> Result<topo::BooleanResult<f64>, BooleanError> {
    let profile = [(0.0, 0.0), (4.0, 0.0), (4.0, 2.0), (2.0, 1.0), (0.0, 2.0)];
    let (prism, cube) = corner_pose(&profile, [2.0, 1.0, 1.0], [2.0, 1.0, 0.0], a, d, tol);
    intersect_with(&prism, &cube, &BooleanDeclarations::default(), tol)
}

/// Stands the row down where ε is wider than `widest`.
fn stood_down_above(widest: f64, tol: Tol) -> bool {
    if tol.eps() > widest {
        test_utils::vacuity::stood_down(
            "eps too wide for the pose",
            "the near-tangent wedge is a band or less thick at this ε, or past its pose's band",
        );
        return true;
    }
    false
}

/// **The witness**: at `d = 1e-9` and `1e-10` the wedge is certified a
/// lump of its own, and the ∩ builds.
#[test]
fn a_near_tangent_wedge_tens_of_bands_thick_reads_outer() {
    let tol = Tol::witness();
    if stood_down_above(1e-11, tol) {
        return;
    }
    for d in [1e-9, 1e-10] {
        let meet = notch307_meet(0, d, tol)
            .unwrap_or_else(|e| panic!("d = {d:e}: the ∩ builds, got {e:?}"));
        let body = &meet.body().expect("the ∩ is not empty").body;
        let shells = classify_shells(body, tol)
            .unwrap_or_else(|e| panic!("d = {d:e}: every shell classifies, got {e:?}"));
        let roles: Vec<_> = shells.iter().map(|s| s.role).collect();
        assert_eq!(
            roles,
            [ShellRole::Outer, ShellRole::Outer],
            "d = {d:e}: the bulk and the wedge are two lumps"
        );
        assert!(
            shells.iter().any(|s| s.volume.abs() < 1e-12),
            "d = {d:e}: one lump is the wedge: {shells:?}"
        );
    }
}

/// **The in-band twin**: at `d = 1e-11` (pose `a3`) the wedge is less
/// than a band thick, and the ∩ refuses its result as the operands'
/// ill-conditioning (D10, Booleans), on the wedge's certified enclosure
/// wholly inside the sliver band, not on one straddling zero.
#[test]
fn a_near_tangent_wedge_in_band_refuses_on_an_enclosure_inside_the_band() {
    let tol = Tol::witness();
    if stood_down_above(1e-12, tol) {
        return;
    }
    let refusal = notch307_meet(3, 1e-11, tol).map(|_| ());
    let Err(BooleanError::Escalated {
        decision: BooleanDecision::ShellRole { others: 0, .. },
        diag,
    }) = &refusal
    else {
        panic!("the ∩ refuses its in-band shell, got {refusal:?}");
    };
    assert_eq!(
        (diag.predicate, diag.terminal_sliver),
        (Some("positive_volume_exact"), true),
        "the certified reading is wholly inside the sliver band: {diag:?}"
    );
}

/// **A walk that reads zero does not hide a certified sliver**: at
/// `vee300 nt e0 a1 d6e-9`, ε = 1e-9, the ∩'s lump has its `V/A`
/// certified in [1.69973e-9, 1.69973e-9], wholly in band, while the f64
/// walk reads 8.7e-10 (prism first) and 3.3e-10 (cube first), in the
/// zero band. The door reads the certificate, in both orders.
#[test]
fn a_sliver_the_walk_reads_as_zero_refuses_in_band_in_both_orders() {
    let tol = Tol::witness();
    if (tol.eps() - 1e-9).abs() > 1e-21 {
        test_utils::vacuity::stood_down(
            "eps other than 1e-9",
            "the walk reads this pose's lump in the zero band at ε = 1e-9 only",
        );
        return;
    }
    let profile = [(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (2.0, 0.5), (0.0, 4.0)];
    let (prism, cube) = corner_pose(&profile, [2.0, 0.5, 1.0], [2.0, 3.5, 0.0], 1, 6e-9, tol);
    let decls = BooleanDeclarations::default();
    for (order, refusal) in [
        ("prism first", intersect_with(&prism, &cube, &decls, tol)),
        ("cube first", intersect_with(&cube, &prism, &decls, tol)),
    ] {
        let refusal = refusal.map(|_| ());
        let Err(BooleanError::Escalated {
            decision: BooleanDecision::ShellRole { others: 0, .. },
            diag,
        }) = &refusal
        else {
            panic!("{order}: the ∩ refuses its certified sliver in band, got {refusal:?}");
        };
        assert!(diag.terminal_sliver, "{order}: {diag:?}");
        assert!(
            diag.to_string().contains("enclosure [1.69972"),
            "{order}: the certified V/A is quoted: {diag}"
        );
    }
}

/// The convex piece of the `notch307` prism beyond its notch.
const NOTCH307_PIECE: [(f64, f64); 4] = [(2.0, 0.0), (4.0, 0.0), (4.0, 2.0), (2.0, 1.0)];

/// Stands the row down at every ε but 1e-9, where these poses' lumps
/// were measured in band.
fn stood_down_off_1e9(tol: Tol) -> bool {
    if (tol.eps() - 1e-9).abs() > 1e-21 {
        test_utils::vacuity::stood_down(
            "eps other than 1e-9",
            "the notch307 piece's lump is in band at ε = 1e-9",
        );
        return true;
    }
    false
}

/// The refusal a result whose in-band shell is alone in its solid
/// earns: `Escalated` on its shell role, the certified `V/A` wholly in
/// band.
fn assert_refuses_in_band(what: &str, refusal: Result<(), BooleanError>) {
    let Err(BooleanError::Escalated {
        decision: BooleanDecision::ShellRole { others: 0, .. },
        diag,
    }) = &refusal
    else {
        panic!("{what}: the ∩ refuses its in-band shell, got {refusal:?}");
    };
    assert!(
        diag.terminal_sliver,
        "{what}: the certified reading is wholly in band: {diag:?}"
    );
}

/// **A result that is only a sliver refuses**: the `notch307` piece
/// beyond its notch, ∩ the census witness cube (`nt e0 a3`), leaves one
/// lump, a wedge less than a band thick, as the only shell of its
/// solid. At d = 1e-8 and 3e-8 its certified `V/A` lies wholly in band,
/// and the ∩ refuses it as the operands' ill-conditioning (D10,
/// Booleans); at d = 3.05e-8 it is past the band and the ∩ builds;
/// at d = −1e-8 the cube misses the piece and the ∩ is empty.
#[test]
fn a_result_that_is_only_a_sliver_refuses_in_band() {
    let tol = Tol::witness();
    if stood_down_off_1e9(tol) {
        return;
    }
    let corner = [2.0, 1.0, 1.0];
    let edge = [2.0, 1.0, 0.0];
    for d in [1e-8, 3e-8] {
        let (piece, cube) = corner_pose(&NOTCH307_PIECE, corner, edge, 3, d, tol);
        let refusal = topo::intersect(&piece, &cube, tol).map(|_| ());
        assert_refuses_in_band(&format!("d = {d:e}"), refusal);
    }
    let (piece, cube) = corner_pose(&NOTCH307_PIECE, corner, edge, 3, 3.05e-8, tol);
    let meet = topo::intersect(&piece, &cube, tol)
        .unwrap_or_else(|e| panic!("d = 3.05e-8: the ∩ builds, got {e:?}"));
    assert_eq!(
        shells_per_solid(meet.body().expect("d = 3.05e-8: the ∩ is not empty")),
        [1],
        "d = 3.05e-8: one lump"
    );
    let (piece, cube) = corner_pose(&NOTCH307_PIECE, corner, edge, 3, -1e-8, tol);
    let meet = topo::intersect(&piece, &cube, tol)
        .unwrap_or_else(|e| panic!("d = -1e-8: the ∩ builds, got {e:?}"));
    assert!(meet.body().is_none(), "d = -1e-8: the ∩ is empty");
}

/// **A wedge that lands in a lump's solid refuses through check 10**:
/// the piece of the row above and a block 0.4 m wide standing apart
/// from it inside the cube, two solids, ∩ the cube at d = 1e-8. The
/// block comes through whole, and the pieces sort leaves the wedge,
/// whose role does not read, in the block's solid, where check 10
/// reads it (the path before check 7's was read at the door).
#[test]
fn a_wedge_sharing_a_lumps_solid_refuses_through_check_10() {
    let tol = Tol::witness();
    if stood_down_off_1e9(tol) {
        return;
    }
    let (corner, edge, d) = ([2.0, 1.0, 1.0], [2.0, 1.0, 0.0], 1e-8);
    let (piece, cube) = corner_pose(&NOTCH307_PIECE, corner, edge, 3, d, tol);
    // Two metres along the cube's axis from the corner: inside the cube,
    // and above the piece, whose top is z = 1.
    let at = add(corner, scale(cube_axis(edge, 3, d), 2.0));
    assert!(at[2] > 1.5, "the block stands above the piece: {at:?}");
    let block = common::mapped_cube::<f64>(
        move |x, y, z| {
            Point3::new(
                at[0] + 0.4 * (x - 0.5),
                at[1] + 0.4 * (y - 0.5),
                at[2] + 0.4 * (z - 0.5),
            )
        },
        tol,
    );
    let block = AtRestBody::validate(block, tol).expect("the block is at rest");
    let both = topo::union(&piece, &block, tol).expect("the piece and the block unite");
    let both = both.body().expect("the union is not empty");
    let solids: Vec<usize> = both.body.solids().map(|(_, s)| s.shells.len()).collect();
    assert_eq!(solids, [1, 1], "the piece and the block are two solids");
    let refusal = topo::intersect(&both.body, &cube, tol).map(|_| ());
    assert_refuses_in_band("piece and block", refusal);
}

/// Shells per solid of a boolean's result.
fn shells_per_solid(meet: &topo::BooleanBody<f64>) -> Vec<usize> {
    meet.body.solids().map(|(_, s)| s.shells.len()).collect()
}

/// A brick at rest.
fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64), tol: Tol) -> AtRestBody<f64> {
    AtRestBody::validate(common::brick::<f64>(x, y, z, tol), tol).expect("the brick is at rest")
}

/// A unit brick ∩ a brick standing on it from `z = 1 - t`: a slab `t`
/// thick, 1 m by 1 m, its `V/A` about `t/2`.
fn slab(t: f64, tol: Tol) -> Result<topo::BooleanResult<f64>, BooleanError> {
    let a = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
    let b = brick((-1.0, 2.0), (-1.0, 2.0), (1.0 - t, 2.0), tol);
    topo::intersect(&a, &b, tol)
}

/// **A slab is a sliver below twice the band**: its only shell's
/// certified `V/A` is about `t/2`, so at t = 1e-8 it lies in band
/// (1e-9 to 1e-8) and the ∩ refuses it as an in-band shell (D10,
/// Booleans), and at t = 2.1e-8 it is past the band and builds.
#[test]
fn a_slab_thinner_than_two_bands_refuses_and_a_thicker_one_builds() {
    let tol = Tol::witness();
    if stood_down_off_1e9(tol) {
        return;
    }
    assert_refuses_in_band("slab t = 1e-8", slab(1e-8, tol).map(|_| ()));
    let meet = slab(2.1e-8, tol).unwrap_or_else(|e| panic!("slab t = 2.1e-8 builds, got {e:?}"));
    assert_eq!(
        shells_per_solid(meet.body().expect("slab t = 2.1e-8 is not empty")),
        [1],
        "slab t = 2.1e-8: one lump"
    );
}

/// **A thin-walled hollow ships today**: a unit brick less a brick
/// inset δ = 1e-8 on every side is one solid of two shells, each role
/// definite, while its wall is as thin as the slab
/// [`a_slab_thinner_than_two_bands_refuses_and_a_thicker_one_builds`]
/// refuses. Pinned as it stands, the question open in
/// `work/flush/a-thin-walled-hollow-ships-while-a-slab-as-thin-refuses.md`.
#[test]
fn a_thin_walled_hollow_ships_while_a_slab_as_thin_refuses() {
    let tol = Tol::witness();
    if stood_down_off_1e9(tol) {
        return;
    }
    let d = 1e-8;
    let a = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
    let core = brick((d, 1.0 - d), (d, 1.0 - d), (d, 1.0 - d), tol);
    let hollow = topo::subtract(&a, &core, tol)
        .unwrap_or_else(|e| panic!("the hollow builds today, got {e:?}"));
    assert_eq!(
        shells_per_solid(hollow.body().expect("the hollow is not empty")),
        [2],
        "one solid, its outer shell and its cavity"
    );
}
