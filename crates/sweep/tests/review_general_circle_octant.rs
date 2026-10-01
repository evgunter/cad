//! Reviewer probe for PR 3733: the minted `Fitted` rows on the oblique
//! trihedron's corner octants (the producer the PR wires), measured
//! between their certification samples — dense 3-D map residual
//! `|S(P(t)) − C(t)|` against the run's ε and the stored envelope.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::blend::build::fillet_edges;
use sweep::{Extrusion, extrude};
use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
use topo::query;
use topo::{Body, BooleanDeclarations};

/// The oblique clip the f4 row fillets, at any scalar the producers run.
fn oblique_clip<T>(lift: impl Fn(f64) -> T + Copy) -> Body<T>
where
    T: geom_core::Decide + geom_core::Bounds + topo::AtRestPolicy,
{
    let prism = |pts: &[(f64, f64)], h: f64| -> Body<T> {
        let lp = bulge_loop(
            pts.iter()
                .map(|(x, y)| (Point2::new(*x, *y), 0.0))
                .collect(),
        );
        let profile = Profile::new(SketchPlane::<T>::xy(), vec![lp.map_scalar(lift)])
            .validate(Tol::witness())
            .unwrap();
        extrude(&profile, Extrusion::Distance(lift(h)), Tol::witness())
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
        &c1,
        &c2,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        Tol::witness(),
    )
    .expect("the oblique clip")
    .body()
    .expect("a body")
    .body
    .clone()
}

/// (d) The exemption's retirement at a scalar with no fitted door: the
/// same oblique trihedron filleted at `Dual64`.
#[test]
fn the_oblique_trihedron_fillets_at_the_dual_scalar() {
    use geom_core::{Dual, Dual64};
    let clipped = oblique_clip::<Dual64>(Dual::constant);
    let edges = query::all_edges(&clipped);
    match fillet_edges(&clipped, &edges, Dual::constant(0.08), Tol::witness()) {
        Ok(_) => println!("Dual64 fillet: builds"),
        Err(e) => panic!("Dual64 fillet refused: {e:?}"),
    }
}

#[test]
fn the_octant_fitted_rows_between_their_samples() {
    let clipped = oblique_clip::<f64>(|x| x);
    let edges = query::all_edges(&clipped);
    let f = fillet_edges(&clipped, &edges, 0.08, Tol::witness()).expect("builds");
    let eps = Tol::witness().get().eps;
    let mut worst = 0.0_f64;
    let mut rows = 0;
    for &corner in &f.corner_faces {
        let face = f.body.get_face(corner).unwrap();
        let surface = f.body.get_surface(face.surface).unwrap().clone();
        let topo::LoopBoundary::Cycle { first } = f.body.get_loop(face.outer).unwrap().boundary
        else {
            continue;
        };
        for he in f.body.loop_cycle(first).unwrap() {
            let Some(row) = f.body.pcurve(he) else {
                continue;
            };
            if !matches!(row.pcurve(), geom_brep::Pcurve::Fitted(_)) {
                continue;
            }
            let edge = f
                .body
                .get_edge(f.body.get_half_edge(he).unwrap().edge)
                .unwrap();
            let Some(topo::CurveGeom::Certified(curve)) = f.body.get_curve_geom(edge.curve) else {
                panic!("certified carrier");
            };
            let (t0, t1) = curve.params();
            let carrier = curve.carrier();
            let res = |t: f64| {
                let p = row.pcurve().eval(t);
                (surface.eval(p.x, p.y) - carrier.eval(t)).norm()
            };
            let n = 20000;
            let (mut dense, mut at) = (0.0_f64, t0);
            for k in 0..=n {
                let t = t0 + (t1 - t0) * k as f64 / n as f64;
                if res(t) > dense {
                    dense = res(t);
                    at = t;
                }
            }
            let sampled = (0..9)
                .map(|k| res(t0 + (t1 - t0) * k as f64 / 8.0))
                .fold(0.0, f64::max);
            let cert = row.certificate();
            println!(
                "corner {corner:?} he {he:?}: span {:.3}  sampled {sampled:.2e}  dense {dense:.2e} at s={:.3}  envelope {:.2e}  max_residual {:.2e}",
                t1 - t0,
                (at - t0) / (t1 - t0),
                cert.envelope,
                cert.max_residual
            );
            worst = worst.max(dense);
            rows += 1;
        }
    }
    println!("{rows} fitted rows; worst dense map residual {worst:.3e} m against eps {eps:e}");
}
