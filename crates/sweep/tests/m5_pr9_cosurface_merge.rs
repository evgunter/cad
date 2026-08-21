//! M5 PR 9 (C12.5): `merge_coplanar_faces` generalizes to cosurface
//! (same-surface) merging for curved faces — the same never-numeric
//! ladder, kind-agnostic on its hard rungs.
//!
//! The live case is the SUB-PERIOD re-merge (the through-cut shape
//! the boolean zip manufactures): a cylinder split by a plane leaves
//! each part's wall as same-key fragments meeting across a strut —
//! the structural rung glues them, the strut dies, volumes are
//! untouched, tier 3 holds. The FULL-PERIOD closure (the plain disc's
//! two half-walls) is outside the inventory at M5 — the exact-B-rep
//! props cannot integrate a full-wrap wall yet — and is recorded as a
//! LOUD skip naming `PeriodClosure`, never a silent no-op and never a
//! wrong-volume body.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Vec3};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::Body;
use topo::splitting::{SplitPlane, split};
use geom_core::Tol;

fn disc_cylinder() -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(-0.5, 0.0), 1.0),
        ProfileVertex::new(Point2::new(0.5, 0.0), 1.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(1.0), Tol::witness()).unwrap().body
}

/// Cylinder wall faces of `body`.
fn wall_count(body: &Body<f64>) -> usize {
    body.faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Cylinder { .. })
            )
        })
        .count()
}

#[test]
fn sub_period_wall_pieces_remerge_structurally() {
    // Split the disc cylinder by the vertical plane x = 0.2: the
    // below part's wall is TWO same-key fragments meeting across one
    // original meridian strut (the C12.5 through-cut shape).
    let body = disc_cylinder();
    let plane = SplitPlane {
        origin: Point3::new(0.2, 0.0, 0.0),
        normal: Vec3::new(1.0, 0.0, 0.0),
    };
    let parts = split(&body, &plane).expect("the tilted-cut lane splits it");
    let mut part = parts.below.body().expect("a below part exists").clone();
    assert_eq!(wall_count(&part), 2, "two same-key wall fragments");
    let vol_before = topo::mass_properties(&part, Tol::witness()).unwrap().volume;

    let out = part.merge_coplanar_faces(Tol::witness()).expect("the cosurface merge");
    assert!(
        out.skipped.is_empty(),
        "sub-period runs commit: {:?}",
        out.skipped
    );
    assert_eq!(out.groups.len(), 1, "one cosurface run: {:?}", out.groups);
    assert_eq!(out.groups[0].killed_edges.len(), 1, "the shared strut dies");
    assert!(out.groups[0].rings_made.is_empty());
    assert_eq!(wall_count(&part), 1, "one maximal sub-period wall face");

    // Volume untouched exactly (the merge is pure structure).
    let vol_after = topo::mass_properties(&part, Tol::witness()).unwrap().volume;
    assert!(
        (vol_after - vol_before).abs() < 1e-12,
        "merge must not move volume: {vol_before} -> {vol_after}"
    );
    if let Err(errs) = topo::validate_geometric(&part, Tol::witness()) {
        panic!("the merged part must stay tier-3 valid: {errs:?}");
    }
}

#[test]
fn full_period_closure_skips_loudly() {
    // The plain disc's two half-walls span the whole period: merging
    // them would need a ring-on-curved-face or a full-wrap seam-form
    // loop, neither of which the exact-B-rep props can integrate yet
    // — recorded as a LOUD skip naming the period closure; the body
    // is untouched and its volume stays exact.
    let mut body = disc_cylinder();
    let vol_before = topo::mass_properties(&body, Tol::witness()).unwrap().volume;
    let out = body.merge_coplanar_faces(Tol::witness()).expect("the merge call itself");
    assert!(out.groups.is_empty(), "no group commits: {:?}", out.groups);
    assert_eq!(out.skipped.len(), 1, "the closure is a loud skip");
    assert!(
        out.skipped[0].reason.contains("full chart period"),
        "the skip names the period closure: {}",
        out.skipped[0].reason
    );
    let vol_after = topo::mass_properties(&body, Tol::witness()).unwrap().volume;
    assert!((vol_after - vol_before).abs() == 0.0, "body untouched");
    topo::validate_closed(&body).unwrap();
}

#[test]
fn distinct_key_curved_neighbors_stay_unmerged() {
    // The never-numeric rule, curved edition: two distinct-key,
    // value-different cylinder walls (a lens) never merge — no rung
    // licenses, the op is a no-op.
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(-0.5, 0.0), 0.3),
        ProfileVertex::new(Point2::new(0.5, 0.0), 0.3),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let mut body = extrude(&profile, Extrusion::Distance(1.0), Tol::witness()).unwrap().body;
    let out = body.merge_coplanar_faces(Tol::witness()).expect("no-op merge");
    assert!(out.groups.is_empty(), "no rung licenses: {:?}", out.groups);
    assert!(out.skipped.is_empty());
}
