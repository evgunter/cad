//! **A join on two edges' one description records nothing**
//! (`[coincidence.door]`): two D-shaped rods, a half-turn wall between
//! two flat sides tangent to it, stacked on one axis. Their union glues
//! the walls and the sides and joins each of the four rulings where the
//! two rods meet. Each joined pair is two edges whose descriptions name
//! one surface pair (the walls' tangent rulings a `TangentIntersection`,
//! the sides' corners an `Intersection`), so the joined edge is that
//! pair's locus read twice, and no row records it. What a class reading's
//! join records is `topo`'s row (`boolean::edge_join::class_zero_rows`,
//! `boolean::ops::join_row_rows`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::test_support::finished;
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{BooleanResult, union};

/// The rod over `z ∈ [z0, z1]`: the half-turn arc over `y ≥ 0` of the
/// unit circle, its two ends run on tangent down to `y = −1`.
fn d_rod(z0: f64, z1: f64) -> topo::Body<f64> {
    let tol = Tol::witness();
    let lp = bulge_loop(vec![
        (Point2::new(1.0, 0.0), 1.0),
        (Point2::new(-1.0, 0.0), 0.0),
        (Point2::new(-1.0, -1.0), 0.0),
        (Point2::new(1.0, -1.0), 0.0),
    ]);
    let profile = Profile::new(SketchPlane::xy_at(Point3::new(0.0, 0.0, z0)), vec![lp])
        .validate(tol)
        .unwrap();
    let depth = Extrusion::Distance {
        depth: z1 - z0,
        side: ExtrudeSide::Along,
    };
    extrude(&profile, depth, tol).unwrap().body.into_body()
}

#[test]
fn stacked_rods_join_their_rulings_on_one_description_and_record_no_join() {
    let tol = Tol::witness();
    let a = finished("the lower rod", d_rod(0.0, 1.0), tol);
    let b = finished("the upper rod", d_rod(1.0, 2.0), tol);
    let BooleanResult::Body(out) = union(&a, &b, tol).unwrap() else {
        panic!("the stacked rods' union is a body");
    };
    assert_eq!(
        out.naming.edge_joins.len(),
        4,
        "each of the four rulings is joined where the rods meet"
    );
    let joins: Vec<_> = out
        .coincidences
        .iter()
        .filter(|row| row.site == topo::DecisionSite::IsoJoin)
        .collect();
    assert!(
        joins.is_empty(),
        "a join on one description is no class reading: {joins:?}"
    );
}
