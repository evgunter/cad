//! **The oblique trihedron fillets at a scalar with no fitted door.**
//! Its corner octants' general circles are imaged only by the fitted
//! lane, which a dual does not hold (`AtRestPolicy::fitted_lane` answers
//! `None`, DL1): such a scalar certifies nothing fitted, so those faces
//! are not owed rows there, and the mint leaves them rowless instead of
//! refusing the fillet.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Dual, Dual64, Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::ExtrudeSide;
use sweep::blend::build::fillet_edges;
use sweep::test_support::finished;
use sweep::{Extrusion, extrude};
use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
use topo::query;
use topo::{Body, BooleanDeclarations};

/// The oblique clip `m5_pr12_fix_pass`'s f4 row fillets, at `Dual64`.
fn oblique_clip() -> Body<Dual64> {
    let lift = Dual::constant;
    let prism = |pts: &[(f64, f64)], h: f64| -> Body<Dual64> {
        let lp = bulge_loop(
            pts.iter()
                .map(|(x, y)| (Point2::new(*x, *y), 0.0))
                .collect(),
        );
        let profile = Profile::new(SketchPlane::<Dual64>::xy(), vec![lp.map_scalar(lift)])
            .validate(Tol::witness())
            .unwrap();
        extrude(
            &profile,
            Extrusion::Distance {
                depth: lift(h),
                side: ExtrudeSide::Along,
            },
            Tol::witness(),
        )
        .unwrap()
        .body
    };
    let v = |x: f64, y: f64, z: f64| Vec3::new(lift(x), lift(y), lift(z));
    let c1 = prism(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], 1.0);
    let c2 = prism(&[(0.0, 0.0), (3.0, 0.0), (3.0, 3.0), (0.0, 3.0)], 3.0);
    let c2 = topo::transform_rigid(
        &c2,
        &Affine3::translation(v(-1.55, -1.55, -2.45)),
        Tol::witness(),
    )
    .unwrap();
    let c2 = topo::transform_rigid(
        &c2,
        &Affine3::rotation_about_axis(
            Point3::new(lift(1.0), lift(1.0), lift(1.0)),
            v(1.0, -1.0, 0.0).normalize(),
            lift(0.4),
        ),
        Tol::witness(),
    )
    .unwrap();
    boolean_op_with(
        BooleanOp::Intersect,
        &finished("the unit cube", c1, Tol::witness()),
        &finished("the tilted cube", c2, Tol::witness()),
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        Tol::witness(),
    )
    .expect("the oblique clip")
    .body()
    .expect("a body")
    .body
    .clone()
    .into_body()
}

/// Red if a doorless scalar's mint refuses the faces only the fitted
/// lane can image instead of leaving them rowless.
#[test]
fn the_oblique_trihedron_fillets_at_the_dual_scalar() {
    let clipped = oblique_clip();
    let edges = query::all_edges(&clipped);
    let f = fillet_edges(&clipped, &edges, Dual::constant(0.08), Tol::witness())
        .expect("the oblique trihedron fillets at a dual");
    let rowless = f
        .corner_faces
        .iter()
        .filter(|&&corner| {
            let face = f.body.get_face(corner).unwrap();
            let topo::LoopBoundary::Cycle { first } = f.body.get_loop(face.outer).unwrap().boundary
            else {
                return false;
            };
            f.body
                .loop_cycle(first)
                .unwrap()
                .into_iter()
                .all(|he| f.body.pcurve(he).is_none())
        })
        .count();
    assert!(
        rowless > 0,
        "the oblique corners, imaged only by the fitted lane, are left rowless at a dual"
    );
}
