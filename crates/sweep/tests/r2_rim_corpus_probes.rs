//! **Reviewer probes for the rim door's Phase-1 corpus** (PR 1821
//! review lane r2) — the two producer classes the shipped rows do not
//! reach.
//!
//! PR 1821's Phase-1 table has ten lines across five producer classes,
//! and says its raw output "is reproduced by the row
//! `rim_of_rows::every_arc_of_every_rim_names_the_same_cycle_from_wherever_it_starts`".
//! That row sweeps four fixtures — `lantern`, `waisted`, `dome`,
//! `sphere_zone` — which are the REVOLVE and one-edge classes only. The
//! two classes whose carriers are minted by something OTHER than a
//! revolve, and which are therefore the ones the exact match was
//! measured for, have no committed row: rims made by the BOOLEAN and
//! extrude's HOLE rims.
//!
//! These are those two lines, as rows. They are the C2 claim for their
//! class: that one rim's arcs really do store bit-identical circles, so
//! the exact door names the rim whole instead of refusing a real rim.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Curve3;
use geom_core::{Affine3, Point2, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::test_support::cube;
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::boolean::{BooleanDeclarations, BooleanOp, SweepStrategy, boolean_op_with};
use topo::query::rim_of;
use topo::{Body, EdgeKey};

fn tol() -> Tol {
    Tol::witness()
}

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn v(x: f64, y: f64, bulge: f64) -> ProfileVertex<f64> {
    ProfileVertex::new(p2(x, y), bulge)
}

fn subtract(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    boolean_op_with(
        BooleanOp::Subtract,
        a,
        b,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        tol(),
    )
    .expect("the subtraction runs")
    .body()
    .expect("the subtraction leaves a body")
    .body
    .clone()
}

/// A sphere of radius 0.3 centred at `c`, revolved from a half-disc.
fn ball_at(c: Vec3<f64>) -> Body<f64> {
    let lp = ProfileLoop::new(vec![v(0.0, -0.3, 1.0), v(0.0, 0.3, 0.0)]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol())
        .expect("the ball profile validates");
    let axis = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    let b = revolve(&vp, axis, Revolution::Full, tol())
        .expect("the ball revolves")
        .body;
    topo::transform_rigid(&b, &Affine3::translation(c), tol()).expect("the ball moves to its pip")
}

/// Every circle carrier on the body, grouped by the rim the door names
/// for it — and the check the Phase-1 table's cells are: within one
/// rim, `center`, `radius` and `axis` are the SAME BITS, and the axis
/// is never the negation.
fn every_rims_carriers_are_bit_identical(body: &Body<f64>, name: &str) -> usize {
    let mut seen: Vec<EdgeKey> = Vec::new();
    let mut rims = 0;
    let circles: Vec<EdgeKey> = body
        .edges()
        .filter(|(_, e)| {
            body.get_curve_geom(e.curve)
                .and_then(|g| g.certified())
                .is_some_and(|c| matches!(c.carrier(), Curve3::Circle { .. }))
        })
        .map(|(k, _)| k)
        .collect();

    for k in circles {
        if seen.contains(&k) {
            continue;
        }
        let Ok(rim) = rim_of(body, k) else {
            continue;
        };
        rims += 1;
        seen.extend(rim.iter().copied());
        // Every arc of THIS rim, read raw.
        let read = |e: EdgeKey| {
            let edge = body.get_edge(e).unwrap();
            let c = body
                .get_curve_geom(edge.curve)
                .and_then(|g| g.certified())
                .unwrap();
            match *c.carrier() {
                Curve3::Circle {
                    center,
                    axis,
                    radius,
                    ..
                } => (
                    [center.x, center.y, center.z].map(f64::to_bits),
                    [axis.x, axis.y, axis.z].map(f64::to_bits),
                    radius.to_bits(),
                ),
                _ => panic!("{name}: a rim's arc carries a circle"),
            }
        };
        let first = read(rim[0]);
        for e in &rim[1..] {
            let other = read(*e);
            assert_eq!(first.0, other.0, "{name}: one rim, one stored centre");
            assert_eq!(first.2, other.2, "{name}: one rim, one stored radius");
            assert_eq!(
                first.1, other.1,
                "{name}: one rim, one stored axis — never the negation"
            );
        }
    }
    rims
}

/// **A boolean-made rim's arcs store one circle, bit for bit.** The
/// carriers here are minted by the SUBTRACTION, not by a revolve, and
/// the Phase-1 table's claim for this class is exactly that they come
/// out bit-identical — if they did not, the exact door would refuse a
/// real rim and the unit's stop clause would have fired.
#[test]
fn r2_a_boolean_made_rims_arcs_store_one_circle() {
    let block = cube(1.0, tol());
    let pip = ball_at(Vec3::new(0.5, 0.5, 1.0));
    let dimpled = subtract(&block, &pip);
    let rims = every_rims_carriers_are_bit_identical(&dimpled, "the dimpled block");
    assert!(rims > 0, "the pip left at least one rim to read");
}

/// **An extruded hole's rims store one circle, bit for bit.** The other
/// producer the Phase-1 table names and no row reaches: a plate with a
/// circular hole through it, whose two hole rims are extrude's.
#[test]
fn r2_an_extruded_hole_rims_arcs_store_one_circle() {
    let outer = ProfileLoop::new(vec![
        v(0.0, 0.0, 0.0),
        v(2.0, 0.0, 0.0),
        v(2.0, 2.0, 0.0),
        v(0.0, 2.0, 0.0),
    ]);
    // A full circle as two half-bulge vertices, wound opposite the
    // outer loop so it reads as a hole.
    let hole = ProfileLoop::new(vec![v(1.5, 1.0, 1.0), v(0.5, 1.0, 1.0)]);
    let plate = Profile::new(SketchPlane::xy(), vec![outer, hole])
        .validate(tol())
        .expect("the plate profile validates");
    let body = extrude(&plate, Extrusion::Distance(0.5), tol())
        .expect("the plate extrudes")
        .body;
    let rims = every_rims_carriers_are_bit_identical(&body, "the pierced plate");
    assert!(rims >= 2, "the hole leaves a rim at each face, got {rims}");
}
