//! Issue #93 review pins — the doubly-nested island chain (adversarial
//! review 2026-07-25, constructions adopted verbatim from the review
//! probes). One general-position chain (no value-equal plane pair
//! anywhere), plus the deeper probes issue #106 added:
//!
//! - **#105 exactness pin**: on pre-#93 main the second union (pillar
//!   into the plate patch inside the tube's hole) SILENTLY returned
//!   22.5 (exact 22.4375) passing tiers 1/2/3′ — a fail-loud
//!   violation; the #93 winding + laringmv repairs make it exact.
//! - **#106 exactness pin** (was a typed-refusal pin until #106 closed
//!   it, retired here per its own baked instructions): intersecting
//!   the exact chain with a slab across the tube's midriff puts
//!   island(tube outer) ⊃ ring(tube inner) ⊃ island(pillar) on the
//!   slab faces. The depth-2 annulus defeats the consecutive-triple
//!   centroid generator (every candidate lands in the hole → none
//!   certifies), which used to exhaust the anchor ladder and refuse
//!   TYPED; the vertex-pair-chord tier added for #106 certifies an
//!   outer↔ring diagonal midpoint instead, and the chain builds at
//!   exactly 3.25.
//! - **#106 depth-3 probe**: one nesting level deeper (hollow pillar
//!   holding a post) — pinned at its live outcome, which is likewise
//!   build-exact (205/64) with tiers green.
//! - **#106 comb probe**: the #93 review's F4 note (a comb-shaped
//!   nonconvex island) — pinned at its live outcome, build-exact
//!   (405/128); an ablation against the pre-#106 join shows this one
//!   never needed the new tier, unlike depth-2/depth-3.
//!
//! Depth-1 control (no pillar): same chain intersects exactly.
//!
//! The anchor-exhaustion arm stays load-bearing regardless: candidate
//! generation is still not a complete triangulation, and any residue
//! refuses typed rather than guessing.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::prism_z;
use geom_core::Decide;
use topo::{
    Body, BooleanBody, BooleanResult, intersect, mass_properties, subtract, union, validate,
    validate_closed, validate_pseudomanifold,
};
use geom_core::Tol;

/// Tube: outer [1,3]², hole [1.5,2.5]², z ∈ [0.5, 3] (cutter strictly
/// taller so the subtract pierces cleanly).
fn tube<T: Decide + geom_core::Bounds + topo::PropsQuadLane>() -> Body<T> {
    let outer = prism_z::<T>(&[(1.0, 1.0), (3.0, 1.0), (3.0, 3.0), (1.0, 3.0)], 0.5, 3.0);
    let cutter = prism_z::<T>(
        &[(1.5, 1.5), (2.5, 1.5), (2.5, 2.5), (1.5, 2.5)],
        0.25,
        3.25,
    );
    let BooleanResult::Body(t) = subtract(&outer.body, &cutter.body, Tol::witness()).expect("tube") else {
        panic!("tube subtract emptied");
    };
    t.body
}

/// Plate [0,4]² × [0,1] ∪ tube: exact 22.0 (plate 16 + tube walls
/// above the plate, annulus 3 × 2).
fn plate_with_tube<T: Decide + geom_core::Bounds + topo::PropsQuadLane>() -> Body<T> {
    let plate = prism_z::<T>(&[(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)], 0.0, 1.0);
    let BooleanResult::Body(u1) = union(&plate.body, &tube::<T>(), Tol::witness()).expect("plate|tube") else {
        panic!("plate|tube emptied");
    };
    u1.body
}

/// The f64 lane's `plate_with_tube`, with the exact-volume guard the
/// interval lane cannot state (Interval has no `PartialEq` oracle by
/// design — the interval lane checks censuses instead).
fn plate_with_tube_f64() -> Body<f64> {
    let u1 = plate_with_tube::<f64>();
    let v = mass_properties(&u1, Tol::witness()).expect("u1 mass").volume;
    assert!((v - 22.0).abs() < 1e-12, "u1 volume {v} != 22.0 exact");
    u1
}

/// The depth-2 chain's final operand: plate ∪ tube ∪ solid pillar.
fn depth2_chain<T: Decide + geom_core::Bounds + topo::PropsQuadLane>() -> Body<T> {
    let pillar = prism_z::<T>(
        &[(1.75, 1.75), (2.25, 1.75), (2.25, 2.25), (1.75, 2.25)],
        0.75,
        2.75,
    );
    let BooleanResult::Body(u2) = union(&plate_with_tube::<T>(), &pillar.body, Tol::witness()).expect("|pillar")
    else {
        panic!("|pillar emptied");
    };
    u2.body
}

/// The depth-3 chain's final operand: plate ∪ tube ∪ hollow pillar ∪
/// post.
fn depth3_chain<T: Decide + geom_core::Bounds + topo::PropsQuadLane>() -> Body<T> {
    let BooleanResult::Body(u2) =
        union(&plate_with_tube::<T>(), &pillar_tube::<T>(), Tol::witness()).expect("|pillar tube")
    else {
        panic!("|pillar tube emptied");
    };
    let post = prism_z::<T>(
        &[
            (1.9375, 1.9375),
            (2.0625, 1.9375),
            (2.0625, 2.0625),
            (1.9375, 2.0625),
        ],
        0.8125,
        2.6875,
    );
    let BooleanResult::Body(u3) = union(&u2.body, &post.body, Tol::witness()).expect("|post") else {
        panic!("|post emptied");
    };
    u3.body
}

/// Slab across the tube's midriff, z ∈ [1.375, 2.375] — every plane
/// value distinct from every operand plane (general position, no
/// declarations involved).
fn slab<T: Decide + geom_core::Bounds + topo::PropsQuadLane>() -> Body<T> {
    prism_z::<T>(
        &[(-1.0, -1.0), (5.0, -1.0), (5.0, 5.0), (-1.0, 5.0)],
        1.375,
        2.375,
    )
    .body
}

/// Structural census of a boolean result — the ONLY exactness the
/// interval lane can state (Interval has no `PartialEq` value oracle
/// by design), and a useful extra pin for the f64 lane.
fn census<T: Decide + geom_core::Bounds + topo::PropsQuadLane>(
    bb: &BooleanBody<T>,
    shells: usize,
    faces: usize,
    label: &str,
) {
    assert_eq!(bb.body.shells().count(), shells, "{label}: shell count");
    assert_eq!(bb.body.faces().count(), faces, "{label}: face count");
}

fn tiers<T: Decide + geom_core::Bounds + topo::PropsQuadLane>(bb: &BooleanBody<T>, label: &str) {
    assert_eq!(validate(&bb.body), Ok(()), "{label}: tier 1");
    assert_eq!(validate_closed(&bb.body), Ok(()), "{label}: tier 2");
    assert_eq!(
        validate_pseudomanifold(&bb.body, &bb.contacts, Tol::witness()),
        Ok(()),
        "{label}: tier 3′"
    );
}

/// #105 (main-was-silently-wrong regression): the pillar union is
/// exact on this branch. On main @ 75166b8 this union returned 22.5
/// with all tiers green — the pillar∩plate overlap (0.5×0.5×0.25 =
/// 0.0625) double-counted, a SILENT wrong body from three plain
/// general-position unions. Exact: plate 16 + tube walls 6 + pillar
/// above plate 0.25×1.75 = 22.4375.
#[test]
fn issue105_doubly_nested_union_exact() {
    let pillar = prism_z::<f64>(
        &[(1.75, 1.75), (2.25, 1.75), (2.25, 2.25), (1.75, 2.25)],
        0.75,
        2.75,
    );
    let BooleanResult::Body(u2) = union(&plate_with_tube_f64(), &pillar.body, Tol::witness()).expect("|pillar")
    else {
        panic!("|pillar emptied");
    };
    tiers(&u2, "u2");
    let v = mass_properties(&u2.body, Tol::witness()).expect("u2 mass").volume;
    assert!(
        (v - 22.4375).abs() < 1e-12,
        "REGRESSION toward issue #105: pillar union volume {v} != 22.4375 \
         exact (main silently returned 22.5)"
    );
}

/// #106 CLOSURE (was the completeness-gap refusal pin; retired per its
/// own baked instructions once the chain started building). The
/// depth-2 nested intersect now builds EXACTLY: the vertex-pair-chord
/// anchor tier certifies an interior point of the annular region the
/// consecutive-triple centroid generator could not reach.
///
/// Exact volume, derived independently of the kernel: the slab is
/// z ∈ [1.375, 2.375], height exactly 1, and every operand's
/// cross-section is constant over that z-interval, so V = area × 1.
/// - plate `[0,4]²×[0,1]`: z-disjoint from the slab — contributes 0.
/// - tube: z ∈ [0.5, 3] ⊃ slab; annulus area 2² − 1² = 3.
/// - pillar `[1.75,2.25]²`: z ∈ [0.75, 2.75] ⊃ slab; area 0.5² = 0.25,
///   and `[1.75,2.25]² ⊂ (1.5,2.5)²` lies strictly inside the tube's
///   hole, so it is disjoint from the annulus — areas add.
///
/// V = (3 + 1/4) × 1 = **13/4 = 3.25**, exact in binary.
#[test]
fn issue106_depth2_nested_intersect_exact() {
    // The input chain's own exactness is pinned by
    // `issue105_doubly_nested_union_exact` (u2 = 22.4375) and
    // `plate_with_tube_f64` (u1 = 22.0), so a miss below is the
    // intersect's fault, never a corrupt operand.
    let u2 = depth2_chain::<f64>();
    let vu2 = mass_properties(&u2, Tol::witness()).expect("u2 mass").volume;
    assert!((vu2 - 22.4375).abs() < 1e-12, "u2 volume {vu2} != 22.4375");
    match intersect(&u2, &slab::<f64>(), Tol::witness()) {
        Ok(BooleanResult::Body(bb)) => {
            tiers(&bb, "depth-2 intersect");
            // Tube annulus prism (10 faces) + pillar box (6).
            census(&bb, 2, 16, "depth-2 intersect");
            let v = mass_properties(&bb.body, Tol::witness()).expect("mass").volume;
            assert!(
                (v - 3.25).abs() < 1e-12,
                "REGRESSION toward issue #106: depth-2 nested intersect \
                 volume {v} != 3.25 exact"
            );
        }
        other => panic!(
            "REGRESSION toward issue #106: the depth-2 nested intersect no \
             longer builds: {other:?}"
        ),
    }
}

/// Depth-1 control: without the pillar the same intersect builds
/// exactly (annulus 3 × slab height 1) — the #93 coverage boundary is
/// precisely between depth 1 and depth 2.
#[test]
fn depth1_nested_intersect_control_exact() {
    match intersect(&plate_with_tube_f64(), &slab::<f64>(), Tol::witness()) {
        Ok(BooleanResult::Body(bb)) => {
            tiers(&bb, "depth-1 control");
            let v = mass_properties(&bb.body, Tol::witness()).expect("mass").volume;
            assert!((v - 3.0).abs() < 1e-12, "control volume {v} != 3.0 exact");
        }
        other => panic!("depth-1 control did not build: {other:?}"),
    }
}

/// Pillar-tube: outer [1.75,2.25]², hole [1.875,2.125]², z ∈ [0.75,
/// 2.75] — the depth-3 probe's middle shell. All plane values dyadic
/// and distinct from every other plane in the chain (general
/// position: no coincidence declarations anywhere).
fn pillar_tube<T: Decide + geom_core::Bounds + topo::PropsQuadLane>() -> Body<T> {
    let outer = prism_z::<T>(
        &[(1.75, 1.75), (2.25, 1.75), (2.25, 2.25), (1.75, 2.25)],
        0.75,
        2.75,
    );
    let cutter = prism_z::<T>(
        &[
            (1.875, 1.875),
            (2.125, 1.875),
            (2.125, 2.125),
            (1.875, 2.125),
        ],
        0.625,
        2.875,
    );
    let BooleanResult::Body(t) = subtract(&outer.body, &cutter.body, Tol::witness()).expect("pillar tube") else {
        panic!("pillar-tube subtract emptied");
    };
    t.body
}

/// DEPTH-3 PROBE (issue #106, one level deeper than the closed gap):
/// the same plate/tube chain, but the pillar is itself hollow and
/// holds a post, so the slab section carries island(tube outer) ⊃
/// ring(tube hole) ⊃ island(pillar outer) ⊃ ring(pillar hole) ⊃
/// island(post). Outcome pinned LIVE, as observed: it BUILDS, exact,
/// tiers green — the vertex-pair-chord tier is depth-agnostic (each
/// annular region gets its own outer↔ring diagonal), so closing the
/// depth-2 gap closed this one too. Result: 3 shells (two annular
/// prisms, one box), which is the honest topology.
///
/// Exact intersect volume (independent derivation, the same
/// constant-cross-section argument as the depth-2 pin — slab height
/// exactly 1, every shell's z-range ⊃ [1.375, 2.375], plate
/// z-disjoint, all three cross-sections pairwise disjoint since each
/// sits strictly inside the next one's hole):
/// - tube annulus 2² − 1² = 3
/// - pillar annulus 0.5² − 0.25² = 1/4 − 1/16 = 3/16
/// - post 0.125² = 1/64
///
/// V = (3 + 3/16 + 1/64) × 1 = 205/64 = **3.203125**, exact in binary.
///
/// The u3 chain itself is pinned exact first, so a volume miss here
/// can never be blamed on a corrupt input: u3 = plate 16 + tube walls
/// 3×2 + pillar walls above the plate 3/16 × 1.75 + post above the
/// plate 1/64 × 1.6875 = 22.3544921875 (everything below z = 1 is
/// already plate material).
#[test]
fn issue106_depth3_nested_intersect_probe() {
    let u3 = depth3_chain::<f64>();
    let vu3 = mass_properties(&u3, Tol::witness()).expect("u3 mass").volume;
    assert!(
        (vu3 - 22.3544921875).abs() < 1e-12,
        "depth-3 input chain u3 volume {vu3} != 22.3544921875 exact"
    );
    match intersect(&u3, &slab::<f64>(), Tol::witness()) {
        Ok(BooleanResult::Body(bb)) => {
            tiers(&bb, "depth-3 intersect");
            // Tube annulus (10) + pillar annulus (10) + post box (6).
            census(&bb, 3, 26, "depth-3 intersect");
            let v = mass_properties(&bb.body, Tol::witness()).expect("mass").volume;
            assert!(
                (v - 3.203125).abs() < 1e-12,
                "depth-3 nested intersect volume {v} != 3.203125 exact"
            );
        }
        // Honest live pin: if a future change makes this refuse, the
        // refusal must be TYPED (never a silent body) — see the
        // anchor-exhaustion arm in `boolean::join`.
        other => panic!(
            "depth-3 nested intersect no longer builds (was exact 3.203125): \
             {other:?}"
        ),
    }
}

// =====================================================================
// Interval lane: the same depth-2/depth-3 chains at T = Interval, to
// prove the #106 vertex-pair-chord anchor tier is scalar-generic (its
// candidates go through the same reified `point_in_face` /
// `point_in_solid` funnel, so an Indeterminate escalates rather than
// guessing). Censuses only — Interval has no exact-value oracle.
// =====================================================================

#[cfg(feature = "interval")]
mod interval {
    use super::*;
    use geom_core::Interval;

    #[test]
    fn nested_intersects_interval() {
        let BooleanResult::Body(d2) =
            intersect(&depth2_chain::<Interval>(), &slab::<Interval>()).expect("depth-2 interval")
        else {
            panic!("depth-2 interval intersect returned Empty");
        };
        tiers(&d2, "depth-2 interval");
        census(&d2, 2, 16, "depth-2 interval");
        let BooleanResult::Body(d3) =
            intersect(&depth3_chain::<Interval>(), &slab::<Interval>()).expect("depth-3 interval")
        else {
            panic!("depth-3 interval intersect returned Empty");
        };
        tiers(&d3, "depth-3 interval");
        census(&d3, 3, 26, "depth-3 interval");
    }
}

// =====================================================================
// Residue probe: the #93 review's F4 note ("a comb-like nonconvex
// region can defeat all consecutive-triple centroids"). Same depth-2
// nesting, but the island is a 12-vertex three-tooth comb whose every
// consecutive-triple centroid is either outside the comb or in the
// tube's hole. Outcome pinned LIVE.
// =====================================================================

/// Comb island, all coordinates dyadic and strictly inside the tube's
/// hole `(1.5, 2.5)²`: spine `x ∈ [1.625, 1.8125]`, `y ∈ [1.625,
/// 1.9375]`, three teeth reaching to `x = 2.375`. Area
/// 0.1875×0.3125 + 3×(0.5625×0.0625) = 21/128 = 0.1640625.
fn comb<T: Decide + geom_core::Bounds + topo::PropsQuadLane>() -> Body<T> {
    prism_z::<T>(
        &[
            (1.625, 1.625),
            (2.375, 1.625),
            (2.375, 1.6875),
            (1.8125, 1.6875),
            (1.8125, 1.75),
            (2.375, 1.75),
            (2.375, 1.8125),
            (1.8125, 1.8125),
            (1.8125, 1.875),
            (2.375, 1.875),
            (2.375, 1.9375),
            (1.625, 1.9375),
        ],
        0.75,
        2.75,
    )
    .body
}

/// Depth-2 with a COMB island (the F4 note's shape). Live outcome as
/// observed: BUILDS, exact, tiers green — and an ABLATION (this file
/// run against the pre-#106 `boolean::join`) shows it built already,
/// i.e. this shape never needed the new tier; the earlier tiers reach
/// it. Recorded honestly: F4's comb worry is real about the CENTROID
/// GENERATOR in isolation, but the region here is classifiable from
/// its vertices/edge midpoints. The same ablation fails the depth-2
/// and depth-3 pins, which is where the new tier is load-bearing.
/// Exact intersect
/// volume, same constant-cross-section derivation: tube annulus 3 +
/// comb 21/128, slab height 1 → 405/128 = 3.1640625. Chain guard:
/// u2 = 16 + 6 + (21/128)×1.75 = 22.287109375.
#[test]
fn issue106_comb_island_intersect_probe() {
    let BooleanResult::Body(u2) = union(&plate_with_tube_f64(), &comb::<f64>(), Tol::witness()).expect("|comb")
    else {
        panic!("|comb emptied");
    };
    tiers(&u2, "comb u2");
    let vu2 = mass_properties(&u2.body, Tol::witness()).expect("u2 mass").volume;
    assert!(
        (vu2 - 22.287109375).abs() < 1e-12,
        "comb chain u2 volume {vu2} != 22.287109375 exact"
    );
    match intersect(&u2.body, &slab::<f64>(), Tol::witness()) {
        Ok(BooleanResult::Body(bb)) => {
            tiers(&bb, "comb intersect");
            // Tube annulus (10 faces) + comb prism (12 sides + 2 caps).
            census(&bb, 2, 24, "comb intersect");
            let v = mass_properties(&bb.body, Tol::witness()).expect("mass").volume;
            assert!(
                (v - 3.1640625).abs() < 1e-12,
                "comb intersect volume {v} != 3.1640625 exact"
            );
        }
        other => panic!("comb intersect no longer builds (was exact 3.1640625): {other:?}"),
    }
}
