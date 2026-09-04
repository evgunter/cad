//! **The rim door on the bodies consumers actually hold**
//! ([`topo::query::rim_of`]): a revolve's seam-split rims, the repaired
//! pole body, a one-edge rim, a partial revolve's open rim, and the
//! end-to-end row the unit was filed for — the door's answer fed
//! straight to `fillet_edges`, which carves.
//!
//! The door's own contracts (order, rotation, the co-surface and
//! not-an-arc refusals on a hand-assembled body) are rowed in
//! `topo/tests/rim_of.rs`. What is HERE is what only these producers
//! can state: the arcs of one rim minted one per chart, each with its
//! own seam, which is the case the exact match had to be measured
//! against before it could be written.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;

use geom_core::Tol;
use sweep::Revolution;
use sweep::blend::build::fillet_edges;
use sweep::test_support::{arcs_at, dome, lantern, sphere_zone, waisted};
use topo::query::rim_of;
use topo::{Body, EdgeKey, RimError, mass_properties, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

fn volume(body: &Body<f64>) -> f64 {
    mass_properties(body, tol())
        .expect("mass properties")
        .volume
}

/// Is `b` a rotation of `a`? (Same length, same cycle, same direction.)
fn is_rotation(a: &[EdgeKey], b: &[EdgeKey]) -> bool {
    a.len() == b.len() && (0..a.len()).any(|k| (0..a.len()).all(|i| a[(i + k) % a.len()] == b[i]))
}

/// **Every rim of every seam-split fixture, from every one of its own
/// arcs, is one rim and one rotation.**
///
/// The corpus half of the door's order claim (`rim_of(b)` is a rotation
/// of `rim_of(a)`), swept rather than sampled: for each rim of each
/// body, seed the door with each of the rim's arcs in turn and require
/// the same cycle every time, starting at the seed.
#[test]
fn every_arc_of_every_rim_names_the_same_cycle_from_wherever_it_starts() {
    let mut rims = 0;
    for (name, body) in [
        ("the lantern", lantern(tol())),
        ("the waisted revolve", waisted(tol())),
        ("the dome", dome(1.0, tol())),
        ("the #935 zone", sphere_zone(0.5, Revolution::Full, tol())),
    ] {
        for (r, y) in [
            (1.0, 0.0),
            (0.8, 0.6),
            (0.2, 1.2),
            (0.5, 0.5),
            (1.0, 1.0),
            (0.5, 0.0),
            (0.5, core::f64::consts::FRAC_1_SQRT_2),
            (3.0f64.sqrt(), 1.0),
        ] {
            let arcs = arcs_at(&body, r, y);
            if arcs.is_empty() {
                continue;
            }
            rims += 1;
            let first = rim_of(&body, arcs[0])
                .unwrap_or_else(|e| panic!("{name}: the rim at ({r}, {y}) is one rim, got {e}"));
            assert_eq!(
                first.iter().copied().collect::<BTreeSet<_>>(),
                arcs.iter().copied().collect::<BTreeSet<_>>(),
                "{name}: the rim at ({r}, {y}) is exactly the arcs at that radius and station"
            );
            for seed in &first {
                let from_seed = rim_of(&body, *seed).expect("every arc of a rim names it");
                assert_eq!(from_seed[0], *seed, "{name}: the seed comes first");
                assert!(
                    is_rotation(&first, &from_seed),
                    "{name}: the rim at ({r}, {y}) from {seed:?} is a rotation, \
                     got {from_seed:?} against {first:?}"
                );
            }
        }
    }
    assert_eq!(rims, 10, "the sweep is not vacuous: ten rims measured");
}

/// **The repaired pole body's plane-hosted rim is still two arcs.** The
/// door reads SURFACE keys, not face keys, so the repair that merges
/// each cap's two half-disks into one face — which every consumer who
/// booleans has to run first — leaves the rim exactly where it was.
/// A face-keyed door would answer one arc here and be wrong.
#[test]
fn the_repaired_pole_bodys_rim_is_two_arcs_on_one_plane_face() {
    let mut source = lantern(tol());
    source
        .merge_coplanar_faces(tol())
        .expect("the pole-split caps repair");
    let arcs = arcs_at(&source, 1.0, 0.0);
    assert_eq!(arcs.len(), 2, "the neck rim is still two arcs");
    let faces = |k: EdgeKey| {
        let e = source.get_edge(k).unwrap();
        let f = |he| {
            source
                .get_loop(source.get_half_edge(he).unwrap().parent_loop)
                .unwrap()
                .face
        };
        (f(e.he_plus), f(e.he_minus))
    };
    let (a0, b0) = faces(arcs[0]);
    let (a1, b1) = faces(arcs[1]);
    assert!(
        [a0, b0].contains(&a1) || [a0, b0].contains(&b1),
        "after the repair one plane FACE hosts both arcs"
    );
    assert_eq!(
        rim_of(&source, arcs[0]).expect("the repaired rim is one rim"),
        vec![arcs[0], arcs[1]],
        "and the door hands back both"
    );
}

/// **A one-edge rim is `[edge]`.** The annular revolve's rims are
/// self-closed, so the door's chain walk closes on the first step; the
/// answer is the seed and nothing else.
#[test]
fn a_one_edge_rim_is_the_seed_alone() {
    let body = dome(1.0, tol());
    let equator = arcs_at(&body, 1.0, 0.0);
    assert_eq!(equator.len(), 1, "the dome's equator is one closed edge");
    assert_eq!(rim_of(&body, equator[0]).unwrap(), equator);
}

/// **A partial revolve's rim refuses `NotOneRim`, and the gap is at the
/// wedge's end.** The honest open-rim instance: the arcs are real, they
/// are on one circle between one pair of surfaces, and they do not tile
/// it. The refusal names them and the parameter the walk stopped at —
/// a quarter turn, which is exactly where this wedge ends — instead of
/// handing back a partial set a fillet request would then stall on.
#[test]
fn a_partial_revolves_open_rim_refuses_naming_the_gap_at_the_wedge_end() {
    let quarter = sphere_zone(
        0.5,
        Revolution::Partial(core::f64::consts::FRAC_PI_2),
        tol(),
    );
    let arcs = arcs_at(&quarter, 0.5, -0.5);
    assert_eq!(arcs.len(), 1, "the wedge's bore rim is one open arc");
    match rim_of(&quarter, arcs[0]) {
        Err(RimError::NotOneRim { arcs: matched, gap }) => {
            assert_eq!(matched, arcs, "the refusal names every arc that matched");
            assert!(
                (gap.abs() - core::f64::consts::FRAC_PI_2).abs() < 1e-9,
                "the tiling fails a quarter turn from the seam, got {gap}"
            );
        }
        other => panic!("an open rim is not one rim, got {other:?}"),
    }
}

/// **The end-to-end row the unit was filed for.** A caller holding a
/// solid of revolution names ONE arc of the rim it wants — the only
/// thing it can name without knowing how the charts split — asks the
/// door, and hands the answer straight to `fillet_edges`, which carves.
/// No scan, no co-surface filter to remember, no partial set.
///
/// Both material sides, because the door is indifferent to them and the
/// recourse that names it promises both: the waisted revolve's waist is
/// concave and its base and top are convex.
#[test]
fn the_doors_answer_feeds_fillet_edges_and_carves_on_either_side() {
    let source = waisted(tol());
    let before = volume(&source);
    for (name, r, y, convex) in [
        ("the concave waist", 0.5, 0.5, false),
        ("the convex base", 1.0, 0.0, true),
        ("the convex top", 1.0, 1.0, true),
    ] {
        let seed = arcs_at(&source, r, y)[0];
        let rim = rim_of(&source, seed).unwrap_or_else(|e| panic!("{name}: one rim, got {e}"));
        assert_eq!(rim.len(), 2, "{name} is seam-split");
        let out = fillet_edges(&source, &rim, 0.05, tol())
            .unwrap_or_else(|e| panic!("{name}: the door's answer carves, got {e:?}"));
        assert_eq!(out.band_faces.len(), 1, "{name}: one annulus band");
        validate_geometric(&out.body, tol())
            .unwrap_or_else(|e| panic!("{name}: tier-3 valid, got {e:?}"));
        let after = volume(&out.body);
        if convex {
            assert!(after < before, "{name}: a convex band removes material");
        } else {
            assert!(after > before, "{name}: a concave band adds material");
        }
    }
}
