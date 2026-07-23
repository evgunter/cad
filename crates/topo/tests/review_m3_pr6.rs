//! M3 PR 6a ADVERSARIAL REVIEW (PR #75) — falsification suite.
//! Assignments R1–R5, R7 (R6 e2e lives in
//! `crates/stl/tests/review_m3_pr6_e2e.rs`). Each test names the claim
//! it attacks; comments record the verdict the run demonstrated.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{cube_into, mapped_cube, prism, prism_z};
use geom_core::{Bounds, Decide, Point3, Vec3};
use topo::{
    Body, BooleanError, BooleanResult, ContactRecords, SplitError, SplitJoinError, SplitPart,
    SplitPlane, ValidationError, intersect, mass_properties, split, subtract, union,
    validate_pseudomanifold,
};

fn brick<T: Decide>(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<T> {
    prism_z::<T>(&[(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)], z.0, z.1).body
}

fn plane_y<T: Decide>(c: f64, ny: f64) -> SplitPlane<T> {
    SplitPlane {
        origin: Point3::new(T::from_f64(0.0), T::from_f64(c), T::from_f64(0.0)),
        normal: Vec3::new(T::from_f64(0.0), T::from_f64(ny), T::from_f64(0.0)),
    }
}

fn body_of<T: geom_core::Real>(part: &SplitPart<T>) -> &Body<T> {
    part.body().expect("side has material")
}

// =================================================================
// R1 — the D7 mirror-identity pinch lane.
// =================================================================

/// The PR-3 pinch profiles, verbatim (below-pinch under +n / above-
/// pinch under +n respectively).
const MIRRORED: &[(f64, f64)] = &[
    (0.0, 0.0),
    (3.0, 0.0),
    (4.0, 1.0),
    (5.0, 0.0),
    (6.0, 1.0),
    (7.0, 1.0),
    (8.0, 0.0),
    (8.0, 2.0),
    (0.0, 2.0),
];
const NOTCHED: &[(f64, f64)] = &[
    (0.0, 0.0),
    (8.0, 0.0),
    (8.0, 2.0),
    (7.0, 1.0),
    (6.0, 1.0),
    (5.0, 2.0),
    (4.0, 1.0),
    (3.0, 2.0),
    (0.0, 2.0),
];

/// A profile pinching on BOTH sides of y = 1: a bottom notch with apex
/// (4,1) pinches the BELOW pieces (MIRRORED's class) and a top notch
/// with apex (9,1) pinches the ABOVE pieces (NOTCHED's class). A valid
/// decomposition exists (2 shells above pinched at (9,1) + 2 shells
/// below pinched at (4,1) — each side is exactly what the orientation
/// table produces one-sided), but the mirror lane refuses: the direct
/// run dies at the below pinch, the mirrored rerun dies at the
/// (now-below) above pinch.
const BOTH_SIDED: &[(f64, f64)] = &[
    (0.0, 0.0),
    (3.0, 0.0),
    (4.0, 1.0),
    (5.0, 0.0),
    (12.0, 0.0),
    (12.0, 2.0),
    (10.0, 2.0),
    (9.0, 1.0),
    (8.0, 2.0),
    (0.0, 2.0),
];
/// Single-sided controls carved out of BOTH_SIDED (same plane, same
/// tips) — these must succeed, proving the both-sided decomposition is
/// made of two independently-deliverable halves.
const BUMP_ONLY: &[(f64, f64)] = &[
    (0.0, 0.0),
    (3.0, 0.0),
    (4.0, 1.0),
    (5.0, 0.0),
    (12.0, 0.0),
    (12.0, 2.0),
    (0.0, 2.0),
];
const NOTCH_ONLY: &[(f64, f64)] = &[
    (0.0, 0.0),
    (12.0, 0.0),
    (12.0, 2.0),
    (10.0, 2.0),
    (9.0, 1.0),
    (8.0, 2.0),
    (0.0, 2.0),
];

/// R1(a)+(c): the both-sided pinch class. VERDICT recorded in the
/// review report: the lane REFUSES (typed `DegenerateSection`, no
/// panic, no wrong body) a body whose valid pinch decomposition exists
/// — the mirror rerun only relocates the refusal, falsifying the
/// documented "double refusal = genuine both-sided zero-area residue"
/// reading. Controls: each single-sided half succeeds under the SAME
/// plane with exact volume conservation.
fn both_sided_pinch_scenario<T: Decide + Bounds>() {
    for (profile, must_succeed) in [
        (BUMP_ONLY, true),
        (NOTCH_ONLY, true),
        (BOTH_SIDED, false),
    ] {
        let fx = prism::<T>(profile, 1.0);
        let v0 = mass_properties(&fx.body).unwrap().volume;
        match split(&fx.body, &plane_y::<T>(1.0, 1.0)) {
            Ok(r) => {
                assert!(must_succeed, "BOTH_SIDED unexpectedly split — re-examine");
                let (va, vb) = (
                    mass_properties(body_of(&r.above)).unwrap().volume,
                    mass_properties(body_of(&r.below)).unwrap().volume,
                );
                let d = (va + vb - v0).abs();
                assert!(d.hi() <= 1e-9, "volume conservation: {d:?}");
            }
            Err(e) => {
                assert!(!must_succeed, "single-sided control refused: {e:?}");
                // The double-refusal path must be TYPED (the direct
                // run's DegenerateSection), never a panic/wrong body.
                assert!(
                    matches!(
                        e,
                        SplitError::Join(SplitJoinError::DegenerateSection { .. })
                    ),
                    "double refusal must surface DegenerateSection, got {e:?}"
                );
            }
        }
    }
}

#[test]
fn r1_both_sided_pinch_f64() {
    both_sided_pinch_scenario::<f64>();
}

/// R1(b): structural comparison of the two paths of the identity —
/// `split(S, +n)` vs swap(`split(S, −n)`) — on both pinch profiles
/// (each orientation exercises the mirror lane for one of them).
/// Compares volumes, shell/face/edge/vertex counts per assigned side,
/// and the section-face normal convention (above section m = −n,
/// below m = +n) that the doc claims survives the swap.
fn mirror_identity_scenario<T: Decide + Bounds>() {
    for profile in [MIRRORED, NOTCHED] {
        let fx = prism::<T>(profile, 1.0);
        let rp = split(&fx.body, &plane_y::<T>(1.0, 1.0)).unwrap();
        let rn = split(&fx.body, &plane_y::<T>(1.0, -1.0)).unwrap();
        // swap(split(S,−n)): its BELOW is our ABOVE.
        let pairs = [
            (body_of(&rp.above), body_of(&rn.below), "above"),
            (body_of(&rp.below), body_of(&rn.above), "below"),
        ];
        for (x, y, side) in pairs {
            assert_eq!(
                x.shells().count(),
                y.shells().count(),
                "{profile:?} {side} shells"
            );
            assert_eq!(
                x.faces().count(),
                y.faces().count(),
                "{profile:?} {side} faces"
            );
            assert_eq!(
                x.edges().count(),
                y.edges().count(),
                "{profile:?} {side} edges"
            );
            assert_eq!(
                x.vertices().count(),
                y.vertices().count(),
                "{profile:?} {side} vertices"
            );
            let d = (mass_properties(x).unwrap().volume - mass_properties(y).unwrap().volume)
                .abs();
            assert!(d.hi() <= 1e-9, "{side}: volume mismatch {d:?}");
        }
        // Section-normal convention: outward normals point away from
        // material, so every y=1 section face on y>1 material carries
        // −y and on y<1 material +y — under BOTH plane orientations
        // (doc: "above face m = −n, below face m = +n" per call).
        // rp.above / rn.below are the y>1 material; rp.below /
        // rn.above the y<1 material.
        for (part, want_down) in [
            (body_of(&rp.above), true),
            (body_of(&rp.below), false),
            (body_of(&rn.below), true),
            (body_of(&rn.above), false),
        ] {
            let mut found = 0;
            for (_, f) in part.faces() {
                let Some(topo::Surface::Plane { origin, normal, .. }) =
                    part.get_surface(f.surface)
                else {
                    continue;
                };
                let on_plane = (origin.y - T::from_f64(1.0)).abs().hi() < 1e-9;
                if on_plane && normal.y.lo().abs().max(normal.y.hi().abs()) > 0.99 {
                    found += 1;
                    if want_down {
                        assert!(normal.y.hi() < 0.0, "{profile:?} above-side normal {:?}", normal.y);
                    } else {
                        assert!(normal.y.lo() > 0.0, "{profile:?} below-side normal {:?}", normal.y);
                    }
                }
            }
            assert!(found > 0, "{profile:?}: no section face found on y=1");
        }
    }
}

#[test]
fn r1_mirror_identity_structural_f64() {
    mirror_identity_scenario::<f64>();
}

#[cfg(feature = "interval")]
mod interval_r1 {
    use super::*;

    #[test]
    fn r1_interval_lane() {
        both_sided_pinch_scenario::<geom_core::Interval>();
        mirror_identity_scenario::<geom_core::Interval>();
    }
}
