//! **Two radius-1 rods on crossing axes, one mitred by their bisector
//! plane.**
//!
//! - `A`: a rod about `z` over `z ∈ [−3, 3]` less the half-space under
//!   `z = x`, so its bottom rim is the ellipse `z = x` on its wall;
//! - `B`: a rod about `x` over `x ∈ [x0, 3]`. Its wall holds `A`'s rim,
//!   since `z = x` is a bisector plane of the two walls; its end cap at
//!   `x0 > 0` keeps the second ellipse `z = −x` and the walls' tangent
//!   points `(0, ±1, 0)` out of both solids.
//!
//! Each rod is spun about its own axis, which moves its seam rulings.
//! The closed forms: `vol A = 3π`, `vol B = (3 − x0)π`, and over
//! `x ∈ [x0, 1]` the two share the part of `B`'s disc above `z = x`,
//! `acos x − x√(1 − x²)`, whose integral is
//! `[x·acos x − √(1 − x²) + (1 − x²)^{3/2}/3]` from `x0` to `1`
//! ([`shared`]).
//!
//! The germ runs along `A`'s ellipse edge inside `B`'s wall, so the join
//! reads the walls' axes, finds them apart, and hands the germ to the
//! edge-plane lane rather than the parallel pair's radical plane. That
//! lane copies its edge on the edge's own side, the ellipse's own curve.
//! Past the copy, at spins `(0, 0)` every op builds a sound body at its
//! closed form, but above ε = 1e-12 its certified enclosure is wider
//! than `differential`'s budget (to `1.1e-5` m³ at ε = 1e-9, `5.7e-5`
//! at 1e-6), so it is judged against [`PAD_CEILING`]; at every other
//! pose every op refuses
//! `UnpairedLooseEnds`
//! (`honejoin/the-edge-plane-lane-copies-no-ellipse-edge`). The pins
//! name those endings exactly, so a fix to either moves them.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};

use crate::common::differential::{VOLUME_PAD_BUDGET, Verdict, judged_within, outcome};
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::test_support::{brick, finished};
use topo::{AtRestBody, BooleanError, BooleanResult, SplitJoinError};

/// A rod of radius 1 about `z` over `z ∈ [z0, z1]`, spun `spin` about
/// `z`.
fn rod(spin: f64, (z0, z1): (f64, f64)) -> AtRestBody<f64> {
    let tol = Tol::witness();
    let rod = sweep::test_support::prism_at(
        vec![(Point2::new(-1.0, 0.0), 1.0), (Point2::new(1.0, 0.0), 1.0)],
        z0,
        z1 - z0,
        tol,
    );
    let rod = finished("the rod", rod, tol);
    let m = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_z(), spin);
    finished(
        "the spun rod",
        topo::transform_rigid(&rod, &geom_core::test_support::rigid(&m, tol), tol).unwrap(),
        tol,
    )
}

/// `A`: [`rod`] over `z ∈ [−3, 3]` less the half-space under `z = x`.
fn mitred(spin: f64) -> AtRestBody<f64> {
    let tol = Tol::witness();
    let tilt = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_y(), -PI / 4.0);
    let slab = brick((-50.0, 50.0), (-50.0, 50.0), (-100.0, 0.0), tol);
    let under = finished(
        "the half-space under z = x",
        topo::transform_rigid(&slab, &geom_core::test_support::rigid(&tilt, tol), tol).unwrap(),
        tol,
    );
    match topo::subtract(&rod(spin, (-3.0, 3.0)), &under, tol) {
        Ok(BooleanResult::Body(b)) => b.body,
        other => panic!("the mitred rod builds: {other:?}"),
    }
}

/// `B`: [`rod`] over `[x0, 3]` turned onto `x`.
fn crossing(spin: f64, x0: f64) -> AtRestBody<f64> {
    let tol = Tol::witness();
    let m = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_y(), FRAC_PI_2);
    finished(
        "the crossing rod",
        topo::transform_rigid(
            &rod(spin, (x0, 3.0)),
            &geom_core::test_support::rigid(&m, tol),
            tol,
        )
        .unwrap(),
        tol,
    )
}

/// The volume `A` and `B` share for `B` from `x0`.
fn shared(x0: f64) -> f64 {
    let f = |x: f64| x * x.acos() - (1.0 - x * x).sqrt() + (1.0 - x * x).powf(1.5) / 3.0;
    f(1.0) - f(x0)
}

/// How one op ends past the edge-plane lane's copy of `A`'s ellipse
/// edge.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Ending {
    /// A body `differential` judges `SOUND` against an enclosure no
    /// wider than [`PAD_CEILING`].
    Builds,
    /// The join's loose ends left unpaired.
    LooseEnds,
}

/// The lines of the six ops at `(spin A, spin B, x0)` that do not end
/// as `want`, each with what it did against its closed form.
fn not_ending((sa, sb, x0): (f64, f64, f64), want: Ending) -> Vec<String> {
    let tol = Tol::witness();
    let (a, b) = (mitred(sa), crossing(sb, x0));
    let (va, vb, s) = (3.0 * PI, (3.0 - x0) * PI, shared(x0));
    let mut lines = Vec::new();
    for (op, r, volume) in [
        ("A ∪ B", topo::union(&a, &b, tol), va + vb - s),
        ("B ∪ A", topo::union(&b, &a, tol), va + vb - s),
        ("A ∩ B", topo::intersect(&a, &b, tol), s),
        ("B ∩ A", topo::intersect(&b, &a, tol), s),
        ("A ∖ B", topo::subtract(&a, &b, tol), va - s),
        ("B ∖ A", topo::subtract(&b, &a, tol), vb - s),
    ] {
        let loose = matches!(
            &r,
            Err(BooleanError::Join(j)) if matches!(j.error(), SplitJoinError::UnpairedLooseEnds { .. })
        );
        let (ending, line) = if loose {
            (Some(Ending::LooseEnds), outcome(r, volume, tol))
        } else {
            let j = judged_within(r, Some(volume), tol, PAD_CEILING);
            (
                (j.verdict == Verdict::Sound).then_some(Ending::Builds),
                j.line,
            )
        };
        if ending != Some(want) {
            lines.push(format!(
                "spins ({sa}, {sb}), B from x = {x0} | {op} | {line}"
            ));
        }
    }
    lines
}

/// **The crossing pair is not read as parallel**: at the row's pose
/// every op in both orders gets past the edge-plane lane's copy of the
/// ellipse edge and refuses `UnpairedLooseEnds`, a refusal on valid
/// input that `honejoin/the-edge-plane-lane-copies-no-ellipse-edge`
/// owns, and none at the parallel pair's radical plane (`JoinDesync`).
/// An op that builds, or refuses otherwise, moves this pin with that
/// row.
#[test]
fn crossing_rods_end_past_the_ellipse_edge_copy() {
    let moved = not_ending((0.4, 0.3, 0.2), Ending::LooseEnds);
    assert!(
        moved.is_empty(),
        "the-edge-plane-lane-copies-no-ellipse-edge: every op leaves loose ends\n{}",
        moved.join("\n")
    );
}

/// [`crossing_rods_end_past_the_ellipse_edge_copy`] at two more spin
/// pairs, both seam rulings at `±x` among them, and with `B` from
/// `x = 0.5`. With both seams at `±x`, every op builds.
#[test]
fn crossing_rods_end_past_the_ellipse_edge_copy_at_every_spin() {
    let moved: Vec<String> = [
        ((1.0, -0.7, 0.2), Ending::LooseEnds),
        ((0.0, 0.0, 0.2), Ending::Builds),
        ((0.4, 0.3, 0.5), Ending::LooseEnds),
    ]
    .into_iter()
    .flat_map(|(pose, want)| not_ending(pose, want))
    .collect();
    assert!(
        moved.is_empty(),
        "the-edge-plane-lane-copies-no-ellipse-edge: each pose ends as pinned\n{}",
        moved.join("\n")
    );
}

/// **The shared volume's closed form agrees with a slicing**: Simpson
/// over `x` of the area of `B`'s unit disc above `z = x`, itself
/// integrated by Simpson over `z`.
#[test]
fn the_shared_closed_form_agrees_with_a_slicing() {
    let simpson = |f: &dyn Fn(f64) -> f64, lo: f64, hi: f64| {
        let n = 2000;
        let h = (hi - lo) / f64::from(n);
        let sum: f64 = (0..=n)
            .map(|i| {
                let w = if i == 0 || i == n {
                    1.0
                } else if i % 2 == 1 {
                    4.0
                } else {
                    2.0
                };
                w * f(lo + h * f64::from(i))
            })
            .sum();
        sum * h / 3.0
    };
    for x0 in [0.2, 0.5] {
        let area = |x: f64| simpson(&|z: f64| 2.0 * (1.0 - z * z).max(0.0).sqrt(), x, 1.0);
        let sliced = simpson(&area, x0, 1.0);
        assert!(
            (sliced - shared(x0)).abs() < 1e-5,
            "x0 {x0}: sliced {sliced}, closed form {}",
            shared(x0)
        );
    }
}

/// The widest certified volume enclosure (`volume_pad`, m³) these
/// bodies are judged against at any ε the suite runs, in place of
/// `differential::VOLUME_PAD_BUDGET`, which they do not meet above
/// ε = 1e-12: measured at review of PR 5940 (2026-10-28) at most
/// `5.7e-5` for the rods and `4.3e-4` for the halves (ε = 1e-6),
/// `1.1e-5` and `6.6e-6` at 1e-9.
const PAD_CEILING: f64 = 1e-3;

/// How far the enclosure's midpoint may sit from the closed form, m³:
/// measured at review at most `1.4e-6` (the rods' intersection, ε =
/// 1e-6) and under `1e-7` everywhere else.
const MIDPOINT: f64 = 1e-5;

/// `r` judged `SOUND` against [`PAD_CEILING`], its enclosure's midpoint
/// within [`MIDPOINT`] of `want`, and at ε ≤ 1e-12 its enclosure within
/// `differential`'s own budget. The failing line otherwise.
fn held(
    what: &str,
    r: Result<BooleanResult<f64>, BooleanError>,
    want: f64,
    tol: Tol,
) -> Option<String> {
    let j = judged_within(r, Some(want), tol, PAD_CEILING);
    let midpoint = j.volume.is_some_and(|v| (v - want).abs() <= MIDPOINT);
    let tight = tol.eps() > 1e-12 || j.pad.is_some_and(|pad| pad <= VOLUME_PAD_BUDGET);
    (!(j.verdict == Verdict::Sound && midpoint && tight)).then(|| format!("{what} | {}", j.line))
}

/// **At spins `(0, 0)` the six ops hold their closed forms to a
/// bounded enclosure** (review of PR 5940): what
/// [`crossing_rods_end_past_the_ellipse_edge_copy_at_every_spin`] reads
/// as `Builds`, held here to [`MIDPOINT`] as well.
#[test]
fn crossing_rods_at_rest_spins_hold_a_bounded_enclosure() {
    let tol = Tol::witness();
    let x0 = 0.2;
    let (a, b) = (mitred(0.0), crossing(0.0, x0));
    let (va, vb, s) = (3.0 * PI, (3.0 - x0) * PI, shared(x0));
    let bad: Vec<String> = [
        ("A ∪ B", topo::union(&a, &b, tol), va + vb - s),
        ("B ∪ A", topo::union(&b, &a, tol), va + vb - s),
        ("A ∩ B", topo::intersect(&a, &b, tol), s),
        ("B ∩ A", topo::intersect(&b, &a, tol), s),
        ("A ∖ B", topo::subtract(&a, &b, tol), va - s),
        ("B ∖ A", topo::subtract(&b, &a, tol), vb - s),
    ]
    .into_iter()
    .filter_map(|(op, r, want)| held(op, r, want, tol))
    .collect();
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// **The rod cut by `z = x` and glued back is the rod** (review of PR
/// 5940): the two halves' rims are one plane section of one cylinder,
/// so the germ join reads them as one segment
/// (`sectors::same_construction`), at two spins and in both orders
/// (six unions at three spins took about 240 s in debug at review).
/// The halves' intersection and differences are not this row's: they
/// refuse `FallbackExtentUnsupported` upstream of the join.
#[test]
fn a_rod_cut_obliquely_and_glued_back_is_the_rod() {
    let tol = Tol::witness();
    let tilt = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_y(), -PI / 4.0);
    let slab = brick((-50.0, 50.0), (-50.0, 50.0), (-100.0, 0.0), tol);
    let under = finished(
        "the half-space under z = x",
        topo::transform_rigid(&slab, &geom_core::test_support::rigid(&tilt, tol), tol).unwrap(),
        tol,
    );
    let mut bad = Vec::new();
    for spin in [0.0, 1.0] {
        let upper = mitred(spin);
        let lower = match topo::intersect(&rod(spin, (-3.0, 3.0)), &under, tol) {
            Ok(BooleanResult::Body(b)) => b.body,
            other => panic!("the lower half builds: {other:?}"),
        };
        for (op, r) in [
            ("U ∪ L", topo::union(&upper, &lower, tol)),
            ("L ∪ U", topo::union(&lower, &upper, tol)),
        ] {
            bad.extend(held(&format!("spin {spin}, {op}"), r, 6.0 * PI, tol));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// **Offsetting the mitred wall refuses typed**: `A ∪ B` at spins
/// `(0, 0)` holds the ellipse edge between the two walls; moving `A`'s
/// wall out by 0.05 asks the section of a radius-1.05 and a radius-1
/// cylinder on meeting axes, which the C5 pose question serves (every
/// cylinder pose is on the section value). Past that gate the offset
/// door refuses at the moved vertex
/// (`shelf/an-offset-wall-against-a-crossing-cylinder-refuses-as-a-vertex-disagreement`);
/// the row moves when that is built.
#[test]
fn offsetting_the_mitred_wall_refuses_typed() {
    let tol = Tol::witness();
    let (a, b) = (mitred(0.0), crossing(0.0, 0.5));
    let Ok(BooleanResult::Body(u)) = topo::union(&a, &b, tol) else {
        panic!("A ∪ B builds at spins (0, 0)");
    };
    let mut body = u.body.into_body();
    // `A`'s wall, every face that wears its surface.
    let walls: Vec<_> = body
        .faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Cylinder { axis, .. }) if axis.z.abs() > 0.9
            )
        })
        .map(|(k, _)| k)
        .collect();
    assert!(!walls.is_empty(), "A's wall");
    let got = topo::replace_faces_offset(&mut body, &walls, 0.05, tol);
    assert!(
        matches!(got, Err(topo::ReplaceFaceError::VertexDisagreement { .. })),
        "{got:?}"
    );
}
