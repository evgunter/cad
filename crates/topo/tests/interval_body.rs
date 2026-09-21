//! `Body<Interval>` instantiation (the M0 carry, discharged at M1 PR 5):
//! the declined cube built at `T = Interval` through the public Euler
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

use std::sync::Arc;

use geom::{NetState, NurbsSurface, Surface};
use geom_core::Tol;
use geom_core::spline::KnotVector;
use geom_core::{Bounds, Interval, Point3, Real};
use topo::{FaceSurface, ValidationError, validate, validate_closed, validate_geometric};

use crate::common;

#[test]
fn interval_cube_builds_and_validates_at_both_tiers() {
    // The §9.4.2-minimal cube (1 mvfs + 7 mev + 5 mef) with its face
    // geometry declined, at `T = Interval`: [`common::declined_cube`],
    // the same generic door the geometric rows below take. The pure-
    // replay claim this file is about is that ONE operator sequence
    // instantiates at both scalars, so taking the shared builder here
    // states it instead of transcribing it.
    let cube = common::declined_cube::<Interval>(Tol::witness());
    let body = cube.body;
    // The strut up from B, whose far end is the B' corner read below.
    let e_bb = cube.mevs[4];

    // Minimal counts, both validation tiers.
    assert_eq!(body.vertices().count(), 8);
    assert_eq!(body.edges().count(), 12);
    assert_eq!(body.faces().count(), 6);
    assert_eq!(body.half_edges().count(), 24);
    assert_eq!(validate(&body), Ok(()));
    assert_eq!(validate_closed(&body), Ok(()));

    // The geometry arenas really carry intervals, and this corner's
    // enclosure is a single POINT rather than a bracket: `Real::from_f64`
    // is an exact embedding and the shared door's coordinates are the
    // unit square's dyadic 0.0/1.0, so nothing here is widened. That is
    // this file's premise, not an incidental convenience — a door built
    // on coordinates no `f64` represents exactly would make these
    // equalities the wrong assertion rather than a failing one.
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
    let t = common::geometric_cube::<Interval>(geom_core::Tol::witness());
    assert_eq!(validate(&t.body), Ok(()));
    assert_eq!(validate_closed(&t.body), Ok(()));
    let mut body = t.body;
    common::describe_as_intersections(&mut body, geom_core::Tol::witness());
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
    let t = common::geometric_cube::<Interval>(geom_core::Tol::witness());
    let mut body = t.body;
    common::describe_as_intersections(&mut body, geom_core::Tol::witness());
    assert_eq!(validate_geometric(&body, Tol::witness()), Ok(()));
}

/// Check 1's described-poison arm at the interval scalar. Poison is the
/// scalar's own (`Real::from_f64(NaN)` is NaI here, as it is NaN at
/// `f64`), so the state a net is in is preserved by the lift and the
/// same face draws the same verdict.
#[test]
fn interval_described_net_carrying_poison_is_named_by_the_surface_check() {
    let t = common::geometric_cube::<Interval>(geom_core::Tol::witness());
    let mut body = t.body;
    common::describe_as_intersections(&mut body, geom_core::Tol::witness());
    assert_eq!(validate_geometric(&body, Tol::witness()), Ok(()));

    let face = t.mefs[0].face;
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control: Vec<Point3<Interval>> = (0..4)
        .map(|i| {
            Point3::new(
                Interval::from_f64(f64::NAN),
                Interval::from_f64(f64::from(i)),
                Interval::from_f64(2.0),
            )
        })
        .collect();
    let net = NurbsSurface::new(kv.clone(), kv, control, vec![1.0; 4]).unwrap();
    assert_eq!(
        net.net_state(),
        NetState::Poisoned,
        "the fixture is corrupt DESCRIBED geometry at this scalar, not the placeholder"
    );
    body.set_face_surface(face, FaceSurface::New(Surface::Nurbs(Arc::new(net))))
        .unwrap();

    let errs = validate_geometric(&body, Tol::witness())
        .expect_err("corrupt described geometry is refused at the interval scalar too");
    assert!(
        errs.contains(&ValidationError::PoisonedSurfaceDescription { face }),
        "check 1 must name the corrupt described surface: {errs:?}",
    );
    assert!(
        !errs
            .iter()
            .any(|e| matches!(e, ValidationError::UncertifiableSurface { .. })),
        "the placeholder's verdict is a different state's answer: {errs:?}",
    );
}
