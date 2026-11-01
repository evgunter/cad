//! **A near-tangent ∩'s sliver lump reads its role off a ray aimed at
//! it.**
//!
//! A prism's corner on a cube's face tilted `d` rad off one of the
//! corner's edges (`crates/sweep/examples/near_tangent_census_probe.rs`,
//! pose `nt e0 a<k> d<d>`): the ∩ keeps a wedge 2 m long and `2·d` m
//! thick at its far end as its own shell. At ε = 1e-12 the wedge is tens
//! of bands thick at `d ≥ 1e-10`. At `d = 1e-11` its `V/A` is in band,
//! and its far end, twenty bands thick, still reads its role.
//!
//! Wider ε stands these rows down: the wedge is a band or less thick, and
//! the in-band twin is the pose measured at ε = 1e-12.

#![allow(clippy::expect_used, clippy::panic)]

use geom_core::{Point3, Tol};
use topo::{
    AtRestBody, BooleanDecision, BooleanError, Coincide, ContainDecision, LoopDecision, ShellRole,
    ValidationError, classify_shells, intersect,
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
    intersect(&prism, &cube, tol)
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
            shells.iter().any(|s| s.enclosure.volume.abs() < 1e-12),
            "d = {d:e}: one lump is the wedge: {shells:?}"
        );
    }
}

/// **The in-band twin builds**: at `d = 1e-11` (pose `a3`) the wedge's
/// `V/A` is less than a band, but it is `2d` thick at its far end, twenty
/// bands at ε = 1e-12, and a ray aimed at one of its faces reads its
/// role there (D10, Booleans: thinness is judged by pairs). The ∩ builds,
/// the wedge a solid of its own beside the bulk.
#[test]
fn a_near_tangent_wedge_whose_volume_is_in_band_reads_its_role_off_a_ray() {
    let tol = Tol::witness();
    if stood_down_above(1e-12, tol) {
        return;
    }
    let meet = notch307_meet(3, 1e-11, tol).unwrap_or_else(|e| panic!("the ∩ builds, got {e:?}"));
    let meet = meet.body().expect("the ∩ is not empty");
    assert_eq!(
        shells_per_solid(meet),
        [1, 1],
        "the bulk and the wedge are two solids"
    );
}

/// **A sliver lump six bands thick refuses at the strut side, in both
/// orders**: at `vee300 nt e0 a1 d6e-9`, ε = 1e-9, the ∩'s arrival edge
/// at the wedge's corner ends 6.8e-9 off the plane of the face the germ
/// runs along, a pair in band (D10: thinness is judged by pairs). The
/// strut order reads that offset, the germ levered by its lean
/// (`bool_strut_side`), and refuses typed as the sectors' coincidence in
/// both orders: the two orders read the one corner. The lump's
/// certified volume, `[1.69973e-9, 1.69973e-9]`, is never reached here.
#[test]
fn a_sliver_lump_six_bands_thick_refuses_at_the_strut_side_in_both_orders() {
    let tol = Tol::witness();
    if (tol.eps() - 1e-9).abs() > 1e-21 {
        test_utils::vacuity::stood_down(
            "eps other than 1e-9",
            "the wedge is six bands thick at ε = 1e-9 only",
        );
        return;
    }
    let profile = [(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (2.0, 0.5), (0.0, 4.0)];
    let (prism, cube) = corner_pose(&profile, [2.0, 0.5, 1.0], [2.0, 3.5, 0.0], 1, 6e-9, tol);
    for (order, result) in [
        ("prism first", intersect(&prism, &cube, tol)),
        ("cube first", intersect(&cube, &prism, tol)),
    ] {
        match result {
            Err(BooleanError::Escalated {
                decision: BooleanDecision::Coincidence(Coincide::Sectors, _),
                diag,
            }) => assert_eq!(
                diag.predicate,
                Some("bool_strut_side"),
                "{order}: refused at the strut side"
            ),
            other => panic!("{order}: the ∩ refuses in band at the strut side, got {other:?}"),
        }
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

/// The census over a built result's own contacts.
fn census(
    what: &str,
    result: Result<topo::BooleanResult<f64>, BooleanError>,
    tol: Tol,
) -> Result<(), Vec<ValidationError>> {
    let result = result.unwrap_or_else(|e| panic!("{what}: builds, got {e:?}"));
    let body = result
        .body()
        .unwrap_or_else(|| panic!("{what}: is not empty"));
    body.body.validate_pseudomanifold(&body.contacts, tol)
}

/// **A needle reads every pair of its cells definite**: the `notch307`
/// piece beyond its notch, ∩ the census witness cube (`nt e0 a3`),
/// leaves one lump, a wedge whose certified `V/A` lies in band (3.29e-9
/// at d = 1e-8, 9.87e-9 at d = 3e-8), alone in its solid. Its `V/A`
/// judges nothing (D10, Booleans), so every order builds, and the
/// census passes it: the two edges leaving the corner's point key part
/// by 8.4e-8 to 2.5e-7 at their far ends, a definite number however
/// thin. Read through the closed form from the edges' starts, their
/// crossing landed 1.4e-9 to 2.8e-9 off the shared point, and the
/// census escalated `pm_census_ee_span` on three of these runs.
/// At d = −1e-8 the cube misses the piece and the ∩ is empty.
#[test]
fn a_needle_reads_every_pair_of_its_cells_definite() {
    let tol = Tol::witness();
    if stood_down_off_1e9(tol) {
        return;
    }
    let corner = [2.0, 1.0, 1.0];
    let edge = [2.0, 1.0, 0.0];
    for (d, cube_first) in [
        (1e-8, false),
        (1e-8, true),
        (3e-8, false),
        (3e-8, true),
        (3.05e-8, false),
    ] {
        let (piece, cube) = corner_pose(&NOTCH307_PIECE, corner, edge, 3, d, tol);
        let (what, meet) = if cube_first {
            (
                format!("d = {d:e}, cube ∩ piece"),
                intersect(&cube, &piece, tol),
            )
        } else {
            (
                format!("d = {d:e}, piece ∩ cube"),
                intersect(&piece, &cube, tol),
            )
        };
        assert_eq!(
            census(&what, meet, tol),
            Ok(()),
            "{what}: every pair definite"
        );
    }
    let (piece, cube) = corner_pose(&NOTCH307_PIECE, corner, edge, 3, -1e-8, tol);
    let meet = intersect(&piece, &cube, tol)
        .unwrap_or_else(|e| panic!("d = -1e-8: the ∩ builds, got {e:?}"));
    assert!(meet.body().is_none(), "d = -1e-8: the ∩ is empty");
}

/// **A wedge beside a lump is a solid of its own**: the piece of the row
/// above and a block 0.4 m wide standing apart from it inside the cube,
/// two solids, ∩ the cube at d = 1e-8. The block comes through whole,
/// and the pieces sort reads the wedge's role off a ray aimed at one of
/// its faces, its `V/A` in band notwithstanding, and moves it out of the
/// block's solid.
#[test]
fn a_wedge_beside_a_lump_is_a_solid_of_its_own() {
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
    let meet = topo::intersect(&both.body, &cube, tol)
        .unwrap_or_else(|e| panic!("piece and block ∩ cube builds, got {e:?}"));
    let meet = meet.body().expect("the ∩ is not empty");
    assert_eq!(
        shells_per_solid(meet),
        [1, 1],
        "the block and the wedge are two solids"
    );
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
    intersect(&a, &b, tol)
}

/// A unit brick less a brick inset `d` on every side: a hollow whose
/// wall is `d` thick, one solid of two shells.
fn hollow(d: f64, tol: Tol) -> Result<topo::BooleanResult<f64>, BooleanError> {
    let a = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
    let core = brick((d, 1.0 - d), (d, 1.0 - d), (d, 1.0 - d), tol);
    topo::subtract(&a, &core, tol)
}

/// A unit brick with a fin `t` thick standing half a metre out of its
/// top: the brick ∪ a block 0.2 m thick, less a cutter that leaves `t`
/// of the block above the top. A fin is cut rather than placed, so it
/// reaches below the thinnest brick the prism door builds.
fn fin(t: f64, tol: Tol) -> Result<topo::BooleanResult<f64>, BooleanError> {
    let a = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
    let block = brick((0.5, 0.7), (0.25, 0.75), (0.5, 1.5), tol);
    let both = topo::union(&a, &block, tol).expect("the brick and the block unite");
    let both = &both.body().expect("the union is not empty").body;
    let cutter = brick((0.5 + t, 1.0), (0.0, 1.0), (1.0, 2.0), tol);
    topo::subtract(both, &cutter, tol)
}

/// A wall `t` thick, built by a boolean.
type Wall = fn(f64, Tol) -> Result<topo::BooleanResult<f64>, BooleanError>;

/// The thicknesses in ε below the band's top `Kε` (= 10ε) that each wall
/// refuses at, and the ones in `(Kε, 2Kε)` that it builds at, where its
/// `V/A` is still in band.
const BELOW_K: [f64; 3] = [3.0, 6.0, 9.9];
const ABOVE_K: [f64; 6] = [10.5, 11.0, 11.5, 12.0, 15.0, 19.0];

/// `refusal` is the near-coincidence of a pair of cells `t` apart: a
/// vertex and a face, or a vertex and a loop's boundary, whichever
/// question the op asks of the wall first.
fn assert_refuses_on_a_pair(what: &str, t: f64, refusal: Result<(), BooleanError>) -> f64 {
    let Err(BooleanError::Escalated { decision, diag }) = &refusal else {
        panic!("{what}: refuses Escalated, got {refusal:?}");
    };
    assert!(
        matches!(
            decision,
            BooleanDecision::Coincidence(Coincide::VertexOnFace, _)
                | BooleanDecision::Containment {
                    decision: Some(ContainDecision::Loop(LoopDecision::Boundary)),
                    ..
                }
        ),
        "{what}: the refusal's subject is a pair of cells: {decision:?}"
    );
    let gap = diag.margin.diagnostic_f64_for_error_text().value();
    assert!(
        gap.is_some_and(|g| (g.abs() - t).abs() < 1e-3 * t),
        "{what}: the pair is the wall's two sides, {t:e} apart: {diag:?}"
    );
    gap.map_or(t, f64::abs)
}

/// A wall built `t` thick: `shells` shells in one solid, and every pair
/// the census reads definite.
fn assert_builds(
    what: &str,
    shells: usize,
    built: Result<topo::BooleanResult<f64>, BooleanError>,
    tol: Tol,
) {
    assert_eq!(
        built_census(what, shells, built, tol),
        Ok(()),
        "{what}: every pair is definite"
    );
}

/// The census of a wall that builds with `shells` shells per solid.
fn built_census(
    what: &str,
    shells: usize,
    built: Result<topo::BooleanResult<f64>, BooleanError>,
    tol: Tol,
) -> Result<(), Vec<ValidationError>> {
    let meet = built.unwrap_or_else(|e| panic!("{what}: builds, got {e:?}"));
    assert_eq!(
        shells_per_solid(meet.body().expect("the wall is not empty")),
        [shells],
        "{what}: shells per solid"
    );
    census(what, Ok(meet), tol)
}

/// **Every thin wall refuses on a pair below `Kε` and builds above it**
/// (D10, Booleans): a slab alone in its solid, the wall of a hollow (a
/// solid of two shells), and a fin on a brick. Below `Kε` the op refuses
/// the pair of cells across the wall; in `(Kε, 2Kε)` each builds, its
/// `V/A` in band and every pair the census reads definite. At `Kε`
/// itself the pair's margin rounds either way (it builds at ε = 1e-9,
/// and refuses at 1e-6 on a margin a hair under `Kε`), and the census
/// of a wall built there escalates the edge pairs whose reach or gap
/// is `Kε` to within its rounding, or none. The poses scale
/// with ε, and run at 1e-9 and 1e-6.
#[test]
fn a_thin_wall_refuses_on_a_pair_below_k_eps_and_builds_above_it() {
    let tol = Tol::witness();
    if ![1e-9, 1e-6]
        .iter()
        .any(|e| (tol.eps() - e).abs() < 1e-3 * e)
    {
        test_utils::vacuity::stood_down(
            "eps other than 1e-9 or 1e-6",
            "the wall rows were measured at 1e-9 and 1e-6",
        );
        return;
    }
    let k_eps = geom_core::Band::linear(tol)
        .expect("the band forms")
        .escalate();
    let walls: [(&str, Wall, usize); 3] =
        [("slab", slab, 1), ("hollow", hollow, 2), ("fin", fin, 1)];
    for (wall, build, shells) in walls {
        for m in BELOW_K {
            let t = m * tol.eps();
            assert_refuses_on_a_pair(&format!("{wall} {m}ε"), t, build(t, tol).map(|_| ()));
        }
        let what = format!("{wall} at Kε");
        match build(k_eps, tol) {
            Err(refusal) => {
                let gap = assert_refuses_on_a_pair(&what, k_eps, Err(refusal));
                assert!(
                    gap < k_eps,
                    "{what}: refuses only on a margin under Kε: {gap:e}"
                );
            }
            built => {
                let errors = built_census(&what, shells, built, tol)
                    .err()
                    .unwrap_or_default();
                let on_k_eps = |e: &ValidationError| {
                    matches!(e, ValidationError::CensusEscalated { cause }
                        if cause.predicate.is_some_and(|p| p.starts_with("pm_census_ee_"))
                            && cause.margin.diagnostic_f64_for_error_text().value()
                                .is_some_and(|m| (m.abs() - k_eps).abs() < 1e-9 * k_eps))
                };
                assert!(
                    errors.iter().all(on_k_eps),
                    "{what}: the census escalates only a pair whose reading rounds onto Kε: \
                     {errors:?}"
                );
            }
        }
        for m in ABOVE_K {
            let t = m * tol.eps();
            assert_builds(&format!("{wall} {m}ε"), shells, build(t, tol), tol);
        }
    }
}

/// **A body mostly thin but nowhere in doubt builds**: a 1 mm cube
/// standing flush on a 1 m² plate 1.5e-8 thick, at ε = 1e-9. The
/// union's `V/A` is 8e-9, in band, while the plate's two faces are a
/// definite pair; it builds in both orders.
#[test]
fn a_millimetre_cube_on_a_thin_plate_builds() {
    let tol = Tol::witness();
    if stood_down_off_1e9(tol) {
        return;
    }
    let cube = brick((0.0, 1e-3), (0.0, 1e-3), (0.0, 1e-3), tol);
    let plate = brick((-0.5, 0.5), (-0.5, 0.5), (-1.5e-8, 0.0), tol);
    for (order, a, b) in [
        ("cube ∪ plate", &cube, &plate),
        ("plate ∪ cube", &plate, &cube),
    ] {
        let both = topo::union(a, b, tol).unwrap_or_else(|e| panic!("{order}: builds, got {e:?}"));
        let body = both.body().expect("the union is not empty");
        let m = topo::mass_properties(&body.body, tol, topo::test_support::pinned_resolution(tol))
            .expect("the union measures");
        let va = m.enclosure.volume / m.enclosure.surface_area;
        let band = geom_core::Band::linear(tol).expect("the band forms");
        assert!(
            va > band.zero() && va < band.escalate(),
            "{order}: V/A = {va:e} lies in the band"
        );
        assert_eq!(shells_per_solid(body), [1], "{order}: one lump");
        assert_eq!(
            census(order, Ok(both), tol),
            Ok(()),
            "{order}: every pair is definite"
        );
    }
}

/// Review probe (PR 6238): the sliver's refusal, its margin and site, in
/// both orders.
#[test]
fn rv_sliver_refusal_margins() {
    let tol = Tol::witness();
    let profile = [(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (2.0, 0.5), (0.0, 4.0)];
    let (prism, cube) = corner_pose(&profile, [2.0, 0.5, 1.0], [2.0, 3.5, 0.0], 1, 6e-9, tol);
    for (order, result) in [
        ("prism first", intersect(&prism, &cube, tol)),
        ("cube first", intersect(&cube, &prism, tol)),
    ] {
        eprintln!("RVSLIVER eps {:e} {order}: {:?}", tol.eps(), result.as_ref().err());
    }
}

/// Review probe (PR 6238): near-tangent corner poses through every op,
/// both orders, one line each, to diff head against main.
#[test]
fn rv_pose_hunt() {
    let tol = Tol::witness();
    let vee = [(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (2.0, 0.5), (0.0, 4.0)];
    let notch = [(0.0, 0.0), (4.0, 0.0), (4.0, 2.0), (2.0, 1.0), (0.0, 2.0)];
    let poses: [(&str, &[(f64, f64)], V3, V3); 5] = [
        ("vee-e0", &vee, [2.0, 0.5, 1.0], [2.0, 3.5, 0.0]),
        ("vee-e1", &vee, [2.0, 0.5, 1.0], [-2.0, 3.5, 0.0]),
        ("vee-up", &vee, [2.0, 0.5, 1.0], [0.0, 0.0, -1.0]),
        ("notch-e0", &notch, [2.0, 1.0, 1.0], [2.0, 1.0, 0.0]),
        ("notch-e1", &notch, [2.0, 1.0, 1.0], [-2.0, 1.0, 0.0]),
    ];
    let ds = [1e-3, 1e-4, 1e-5, 1e-6, 3e-7, 1e-7, 3e-8, 1e-8, 6e-9, 3e-9, 1e-9, 1e-10, 1e-11];
    let short = |r: &Result<topo::BooleanResult<f64>, BooleanError>| -> String {
        match r {
            Ok(b) => match b.body() {
                Some(b) => format!("ok({} faces)", b.body.faces().count()),
                None => "empty".into(),
            },
            Err(BooleanError::Escalated { decision, diag }) => {
                format!("ESC {decision:?} {:?}", diag.predicate)
            }
            Err(e) => {
                let s = format!("{e:?}");
                format!("ERR {}", &s[..s.len().min(70)])
            }
        }
    };
    for (name, profile, v, e) in poses {
        for a in 0..8u32 {
            for d in ds {
                let (prism, cube) = corner_pose(profile, v, e, a, d, tol);
                let rows = [
                    ("int pc", intersect(&prism, &cube, tol)),
                    ("int cp", intersect(&cube, &prism, tol)),
                    ("uni pc", topo::union(&prism, &cube, tol)),
                    ("uni cp", topo::union(&cube, &prism, tol)),
                    ("sub pc", topo::subtract(&prism, &cube, tol)),
                    ("sub cp", topo::subtract(&cube, &prism, tol)),
                ];
                for (op, r) in rows {
                    eprintln!("RVHUNT {name} a{a} d{d:e} {op}: {}", short(&r));
                }
            }
        }
    }
}
