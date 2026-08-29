//! `Body<Interval>` instantiation (the M0 carry, discharged at M1 PR 5):
//! the ops cube built at `T = Interval` through the public Euler
//! operators, validated at both tiers.
//!
//! This is Q1's genericity boundary exercised end to end: topology is
//! scalar-free and never consults a predicate, so the construction is
//! the *same operator sequence* as the `f64` cube with `from_f64`
//! coordinate enclosures — the pure-replay model's interval lane. No
//! tolerance machinery is touched (structural validation reads no
//! scalar), so this file is free of the one-test-per-process funnel
//! discipline that `geom-core`'s interval band tests observe.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use geom_core::{Bounds, Interval, Point3, Real};
use topo::{Body, MefSite, MevSite, validate, validate_closed, validate_geometric};

mod common;

/// A point enclosure from exact `f64` coordinates ([`Real::from_f64`] is
/// an exact embedding; these dyadic values are single points).
fn pt(x: f64, y: f64, z: f64) -> Point3<Interval> {
    Point3::new(
        Interval::from_f64(x),
        Interval::from_f64(y),
        Interval::from_f64(z),
    )
}

#[test]
fn interval_cube_builds_and_validates_at_both_tiers() {
    // The §9.4.2-minimal cube (1 mvfs + 7 mev + 5 mef), transcribed from
    // the f64 acceptance test with interval coordinates.
    let mut body = Body::<Interval>::new();
    let seed = body.mvfs(pt(0.0, 0.0, 0.0)).unwrap();
    let e_ab = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            pt(1.0, 0.0, 0.0),
            Tol::witness(),
        )
        .unwrap();
    let strut = |body: &mut Body<Interval>, at, x, y, z| {
        body.mev_line(
            MevSite::Fan { he1: at, he2: at },
            pt(x, y, z),
            Tol::witness(),
        )
        .unwrap()
    };
    let mef = |body: &mut Body<Interval>, he1, he2| {
        body.mef_chord(MefSite::Chords { he1, he2 }, Tol::witness())
            .unwrap()
    };
    let e_bc = strut(&mut body, e_ab.he_minus, 1.0, 1.0, 0.0);
    let e_cd = strut(&mut body, e_bc.he_minus, 0.0, 1.0, 0.0);
    let he_dc = body
        .find_half_edge(seed.face, e_cd.vertex, e_bc.vertex)
        .unwrap();
    let f_bottom = mef(&mut body, he_dc, e_ab.he_plus);
    let e_aa = strut(&mut body, e_ab.he_plus, 0.0, 0.0, 1.0);
    let e_bb = strut(&mut body, e_bc.he_plus, 1.0, 0.0, 1.0);
    let e_cc = strut(&mut body, e_cd.he_plus, 1.0, 1.0, 1.0);
    let e_dd = strut(&mut body, f_bottom.he_plus, 0.0, 1.0, 1.0);
    let f_front = mef(&mut body, e_aa.he_minus, e_bb.he_minus);
    mef(&mut body, e_bb.he_minus, e_cc.he_minus);
    mef(&mut body, e_cc.he_minus, e_dd.he_minus);
    mef(&mut body, e_dd.he_minus, f_front.he_plus);

    // Minimal counts, both validation tiers.
    assert_eq!(body.vertices().count(), 8);
    assert_eq!(body.edges().count(), 12);
    assert_eq!(body.faces().count(), 6);
    assert_eq!(body.half_edges().count(), 24);
    assert_eq!(validate(&body), Ok(()));
    assert_eq!(validate_closed(&body), Ok(()));

    // The geometry arenas really carry intervals: a corner's enclosure
    // is the exact point it was built from (from_f64 embeds exactly).
    let b_prime = body.get_vertex(e_bb.vertex).unwrap();
    let p = body.get_point(b_prime.point).unwrap();
    assert_eq!((p.x.lo(), p.x.hi()), (1.0, 1.0));
    assert_eq!((p.y.lo(), p.y.hi()), (0.0, 0.0));
    assert_eq!((p.z.lo(), p.z.hi()), (1.0, 1.0));
}

// ---------------------------------------------------------------------
// M2 PR 3: the GEOMETRIC cube at the interval scalar — containment
// through certification. The same generic builder as the f64 lane
// (pure-replay: identical operator sequence, enclosure coordinates);
// every attachment gate and the full tier-3 pass classify their
// residual enclosures definitely.
// ---------------------------------------------------------------------

#[test]
fn interval_geometric_cube_passes_tier3() {
    // Upgraded first (M2 PR 4 fix pass: prefer-intrinsic enforcement —
    // the cube's transverse chords must carry Intersection at rest).
    let t = common::geometric_cube::<Interval>();
    assert_eq!(validate(&t.body), Ok(()));
    assert_eq!(validate_closed(&t.body), Ok(()));
    let mut body = t.body;
    common::describe_as_intersections(&mut body);
    assert_eq!(validate_geometric(&body, Tol::witness()), Ok(()));
    // Certification records are genuine enclosures: max residual
    // brackets are finite, tiny, and contain no poison.
    for (_, curve) in body.curves() {
        let r = curve.certified().unwrap().certificate().max_residual;
        assert!(r.lo().is_finite() && r.hi().is_finite());
        assert!(r.hi() < 1e-12, "residual enclosure too wide: {r:?}");
    }
}

#[test]
fn interval_cube_upgrades_to_intersections() {
    // The prefer-intrinsic upgrade in the interval lane: transversality
    // margins classify Positive from genuine enclosures.
    let t = common::geometric_cube::<Interval>();
    let mut body = t.body;
    common::describe_as_intersections(&mut body);
    assert_eq!(validate_geometric(&body, Tol::witness()), Ok(()));
}

// PLANTED FAILURES — TEMPORARY, REVERTED IN THIS SAME PR.
//
// The demonstration for issue 1128: without `--no-fail-fast` a red shard
// reports its first failure and stops, so four planted failures in one
// binary come back as one row per shard. With the flag they all come back.
// The four are deliberately adjacent so nextest's partitioning splits them
// across both shards, which is what makes the per-shard count readable.
#[test]
fn qa2_planted_failure_1() {
    panic!("QA-2 planted failure 1 of 4 (issue 1128 demonstration; reverted in PR 1232)");
}

#[test]
fn qa2_planted_failure_2() {
    panic!("QA-2 planted failure 2 of 4 (issue 1128 demonstration; reverted in PR 1232)");
}

#[test]
fn qa2_planted_failure_3() {
    panic!("QA-2 planted failure 3 of 4 (issue 1128 demonstration; reverted in PR 1232)");
}

#[test]
fn qa2_planted_failure_4() {
    panic!("QA-2 planted failure 4 of 4 (issue 1128 demonstration; reverted in PR 1232)");
}
