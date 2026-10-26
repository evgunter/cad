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

use geom_core::Tol;
use geom_core::{Point2, Point3, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::ExtrudeSide;
use sweep::{Extrusion, extrude};
use topo::Body;
use topo::splitting::split;

fn disc_cylinder() -> Body<f64> {
    let lp = bulge_loop(vec![
        (Point2::new(-0.5, 0.0), 1.0),
        (Point2::new(0.5, 0.0), 1.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(
        &profile,
        Extrusion::Distance {
            depth: 1.0,
            side: ExtrudeSide::Along,
        },
        Tol::witness(),
    )
    .unwrap()
    .body
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
    // below part's wall would be TWO same-key fragments meeting across
    // one original meridian strut (the C12.5 through-cut shape). The
    // split ends with the cosurface merge, which glues them.
    let body = disc_cylinder();
    let body = sweep::test_support::finished("the body", body, Tol::witness());
    let plane = topo::test_support::split_plane(
        Point3::new(0.2, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        geom_core::Tol::witness(),
    );
    let parts = split(&body, &plane, Tol::witness()).expect("the tilted-cut lane splits it");
    let mut part = topo::Body::clone(parts.below.body().expect("a below part exists"));
    let groups: Vec<_> = parts
        .naming
        .merge_groups
        .iter()
        .filter(|(side, _)| *side == topo::splitting::PlaneSide::Below)
        .map(|(_, g)| g)
        .collect();
    assert_eq!(groups.len(), 1, "one cosurface run: {groups:?}");
    assert_eq!(groups[0].killed_edges.len(), 1, "the shared strut dies");
    assert!(groups[0].rings_made.is_empty());
    assert_eq!(wall_count(&part), 1, "one maximal sub-period wall face");

    // Volume exact: the halves hold the operand's (the merge is pure
    // structure), and a second merge finds nothing.
    let vol = |b: &Body<f64>| {
        topo::test_support::pinned_volume(
            "the measured body",
            &topo::test_support::finished(
                "the measured body",
                topo::Body::clone(b),
                Tol::witness(),
            ),
            Tol::witness(),
        )
        .volume
    };
    let halves = vol(&part) + vol(parts.above.body().expect("an above part exists"));
    assert!(
        (halves - vol(&body)).abs() < 1e-12,
        "merge must not move volume: {halves} vs {}",
        vol(&body)
    );
    let again = part
        .merge_coplanar_faces(Tol::witness())
        .expect("the cosurface merge");
    assert!(
        again.groups.is_empty(),
        "the split left a maximal wall: {:?}",
        again.groups
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
    let vol_before = topo::test_support::pinned_volume(
        "the measured body",
        &topo::test_support::finished(
            "the measured body",
            topo::Body::clone(&body),
            Tol::witness(),
        ),
        Tol::witness(),
    )
    .volume;
    let out = body
        .merge_coplanar_faces(Tol::witness())
        .expect("the merge call itself");
    assert!(out.groups.is_empty(), "no group commits: {:?}", out.groups);
    assert_eq!(out.skipped.len(), 1, "the closure is a loud skip");
    assert!(
        matches!(
            out.skipped[0].reason,
            topo::MergeCoplanarError::PeriodClosure { .. }
        ),
        "the skip carries the period-closure refusal itself: {:?}",
        out.skipped[0].reason
    );
    let vol_after = topo::test_support::pinned_volume(
        "the measured body",
        &topo::test_support::finished(
            "the measured body",
            topo::Body::clone(&body),
            Tol::witness(),
        ),
        Tol::witness(),
    )
    .volume;
    assert!((vol_after - vol_before).abs() == 0.0, "body untouched");
    topo::validate_closed(&body).unwrap();
}

#[test]
fn distinct_key_curved_neighbors_stay_unmerged() {
    // The never-numeric rule, curved edition: two distinct-key,
    // value-different cylinder walls (a lens) never merge — no rung
    // licenses, the op is a no-op.
    let lp = bulge_loop(vec![
        (Point2::new(-0.5, 0.0), 0.3),
        (Point2::new(0.5, 0.0), 0.3),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let mut body = extrude(
        &profile,
        Extrusion::Distance {
            depth: 1.0,
            side: ExtrudeSide::Along,
        },
        Tol::witness(),
    )
    .unwrap()
    .body;
    let out = body
        .merge_coplanar_faces(Tol::witness())
        .expect("no-op merge");
    assert!(out.groups.is_empty(), "no rung licenses: {:?}", out.groups);
    assert!(out.skipped.is_empty());
}

/// **A split across a two-piece wall records its skip on each side.**
/// The disc cylinder cut by `z = 0.5`: each side keeps both half-walls
/// whole, which together close the chart's full period, so the merge the
/// split ends with leaves them unglued and records a `PeriodClosure`
/// skip per side; their boundaries are re-described and each side
/// stays tier-3 valid.
#[test]
fn a_split_across_a_two_piece_wall_records_its_skip_per_side() {
    let body = disc_cylinder();
    let body = sweep::test_support::finished("the body", body, Tol::witness());
    let plane = topo::test_support::split_plane(
        Point3::new(0.0, 0.0, 0.5),
        Vec3::new(0.0, 0.0, 1.0),
        geom_core::Tol::witness(),
    );
    let parts = split(&body, &plane, Tol::witness()).expect("the cross cut splits it");
    for (side, part) in [
        (topo::splitting::PlaneSide::Above, parts.above.body()),
        (topo::splitting::PlaneSide::Below, parts.below.body()),
    ] {
        let part = part.expect("both sides hold material");
        let skips: Vec<_> = parts
            .naming
            .merge_skipped
            .iter()
            .filter(|(s, _)| *s == side)
            .map(|(_, k)| k)
            .collect();
        assert!(
            matches!(skips.as_slice(), [k] if k.faces.len() == 2
                && matches!(k.reason, topo::MergeCoplanarError::PeriodClosure { .. })),
            "{side:?}: one period-closure skip over the two half-walls: {skips:?}"
        );
        assert_eq!(
            wall_count(part),
            2,
            "{side:?}: the half-walls stay two faces"
        );
        if let Err(errs) = topo::validate_geometric(part, Tol::witness()) {
            panic!("{side:?}: the side stays tier-3 valid: {errs:?}");
        }
    }
}
