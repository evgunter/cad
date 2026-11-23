//! **A record on a cell a sphere cut-in divides lands on the piece that
//! holds it.** The banded ball (the unit ball kept between `y = ±0.6`)
//! carries a rest: a small cube's corner touching one half-band, or the
//! rim arc that half-band meets the plane `y = 0.6` along, the cube
//! turned so none of its faces is tangent there. A slab whose plane lies
//! 0.985 from the centre toward latitude 10° at azimuth π/2 cuts a cap
//! off that half-band with no edge crossing, so the op cuts the
//! half-band in along its meridian through the circle, the plane
//! `x = 0`, before it re-enters; the cut's ends split both rim arcs
//! there. A cut keeps the divided cell's key on one piece and mints the
//! other; the rest lies at azimuth 60° (`x > 0`) or 120° (`x < 0`), so
//! one of the two poses has it on the minted piece. The carry reads the
//! face and the arc through the cut, so in either pose the record lands
//! on the piece on the rest's side.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{brick, finished, revolved_about_y};
use topo::{
    AtRestBody, Body, BooleanBody, BooleanInputs, BooleanOp, BooleanResult, EdgeKey, FaceKey,
};
fn tol() -> Tol {
    Tol::witness()
}

fn banded_ball() -> AtRestBody<f64> {
    let ball = revolved_about_y(
        vec![(Point2::new(0.0, -1.0), 1.0), (Point2::new(0.0, 1.0), 0.0)],
        Revolution::Full,
        tol(),
    );
    let ball = finished("the ball", ball, tol());
    let band = finished(
        "the band's box",
        brick::<f64>((-2.0, 2.0), (-0.6, 0.6), (-2.0, 2.0), tol()),
        tol(),
    );
    match topo::boolean::intersect(&ball, &band, tol()) {
        Ok(BooleanResult::Body(b)) => finished("the banded ball", b.body.into_body(), tol()),
        other => panic!("the banded ball builds: {:?}", other.map(|_| ())),
    }
}

/// A `w`-cube with one corner at `p` and its diagonal along `d`: where
/// every direction along the cube from `p` leaves the ball, the cube meets
/// it only at that corner, with no face tangent.
fn corner_at(p: Point3<f64>, d: Vec3<f64>, w: f64) -> AtRestBody<f64> {
    let (diagonal, d) = (Vec3::new(1.0, 1.0, 1.0) / 3f64.sqrt(), d / d.norm());
    let axis = diagonal.cross(d);
    let rot = Affine3::rotation_about_axis(
        Point3::origin(),
        axis / axis.norm(),
        axis.norm().atan2(diagonal.dot(d)),
    );
    let cube: Body<f64> = brick((0.0, w), (0.0, w), (0.0, w), tol());
    let cube = topo::transform_rigid(
        &cube,
        &geom_core::test_support::rigid(&(Affine3::translation(p - Point3::origin()) * rot), tol()),
    )
    .unwrap();
    finished("the corner cube", cube, tol())
}

/// `banded ∪ cube`, the union's records with it.
fn rested(what: &str, banded: &AtRestBody<f64>, cube: &AtRestBody<f64>) -> BooleanBody<f64> {
    match topo::boolean::union(banded, cube, tol()) {
        Ok(BooleanResult::Body(b)) => b,
        other => panic!("{what}: the rest builds: {:?}", other.map(|_| ())),
    }
}

/// `rested` `op` `slab`, carrying `rested`'s records, which cuts the
/// half-band in once.
fn cut_by(
    what: &str,
    op: BooleanOp,
    rested: &BooleanBody<f64>,
    slab: &AtRestBody<f64>,
) -> BooleanBody<f64> {
    let r = match topo::boolean_op_with(
        op,
        &rested.body,
        slab,
        &BooleanInputs {
            records_a: rested.contacts.carried(),
            ..BooleanInputs::default()
        },
        topo::SweepStrategy::Realized,
        tol(),
    ) {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("{what}: builds: {:?}", other.map(|_| ())),
    };
    assert!(
        r.naming.chart_cuts[0].len() == 1 && r.naming.chart_cuts[1].is_empty(),
        "{what}: the slab cuts the half-band in once: {:?}",
        r.naming.chart_cuts
    );
    r
}

/// The x of `p`'s vertex.
fn x_of(body: &Body<f64>, v: topo::VertexKey) -> f64 {
    body.vertex_points()
        .find(|(k, _)| *k == v)
        .map(|(_, p)| p.x)
        .unwrap()
}

/// A 6 × 1 × 6 slab whose near face lies 0.985 from the origin toward
/// latitude 10° at azimuth π/2 (the y axis turned 80° about x).
fn slab() -> AtRestBody<f64> {
    let rot = Affine3::rotation_about_axis(
        Point3::origin(),
        Vec3::new(1.0, 0.0, 0.0),
        80f64.to_radians(),
    );
    let n = rot.transform_vec(Vec3::new(0.0, 1.0, 0.0));
    let slab: Body<f64> = brick((-3.0, 3.0), (0.0, 1.0), (-3.0, 3.0), tol());
    let slab = topo::transform_rigid(
        &slab,
        &geom_core::test_support::rigid(&(Affine3::translation(n * 0.985) * rot), tol()),
    )
    .unwrap();
    finished("the slab", slab, tol())
}

/// The x of every vertex on `face`'s loops.
fn face_xs(body: &Body<f64>, face: FaceKey) -> Vec<f64> {
    body.half_edges()
        .filter(|(_, h)| body.get_loop(h.parent_loop).is_some_and(|l| l.face == face))
        .map(|(_, h)| x_of(body, h.start))
        .collect()
}

/// The x of both ends of `edge`.
fn edge_xs(body: &Body<f64>, edge: EdgeKey) -> Vec<f64> {
    body.half_edges()
        .filter(|(_, h)| h.edge == edge)
        .map(|(_, h)| x_of(body, h.start))
        .collect()
}

#[test]
fn a_rest_on_either_piece_of_a_cut_in_face_lands_on_that_piece() {
    let (banded, slab) = (banded_ball(), slab());
    for az in [60f64, 120.0] {
        let n = Vec3::new(az.to_radians().cos(), 0.0, az.to_radians().sin());
        let cube = corner_at(Point3::origin() + n, n, 0.03);
        let rested = rested(&format!("az {az}"), &banded, &cube);
        assert_eq!(
            rested.contacts.vertex_face().count(),
            1,
            "az {az}: the corner rests on the half-band: {:?}",
            rested.contacts
        );
        for op in [BooleanOp::Union, BooleanOp::Subtract] {
            let what = format!("az {az}, {op:?}");
            let r = cut_by(&what, op, &rested, &slab);
            // Red on the carry that read the face by its key alone: at
            // azimuth 60 the record was dropped.
            let rests: Vec<_> = r.contacts.vertex_face().collect();
            let (&[(_, face)], 1) = (&rests[..], r.contacts.len()) else {
                panic!("{what}: the rest lands once: {:?}", r.contacts)
            };
            let xs = face_xs(&r.body, face);
            assert!(
                xs.iter().all(|&x| x * n.x >= -1e-9),
                "{what}: the record's face lies on the rest's side of the cut, x {xs:?}"
            );
        }
    }
}

#[test]
fn a_rest_on_either_piece_of_a_cut_in_rim_arc_lands_on_that_piece() {
    let (banded, slab) = (banded_ball(), slab());
    let up = Vec3::new(0.0, 1.0, 0.0);
    for az in [60f64, 120.0] {
        let (c, s) = (az.to_radians().cos(), az.to_radians().sin());
        let p = Point3::new(0.8 * c, 0.6, 0.8 * s);
        let cube = corner_at(p, (p - Point3::origin()) + up, 0.03);
        let rested = rested(&format!("az {az}"), &banded, &cube);
        assert_eq!(
            rested.contacts.vertex_edge().count(),
            1,
            "az {az}: the corner rests on the rim arc: {:?}",
            rested.contacts
        );
        for op in [BooleanOp::Union, BooleanOp::Subtract] {
            let what = format!("az {az}, {op:?}");
            let r = cut_by(&what, op, &rested, &slab);
            // Red on the carry that read the arc by its key alone: at
            // azimuth 120 the record named the arc's piece at x > 0.
            let rests: Vec<_> = r.contacts.vertex_edge().collect();
            let (&[(_, edge)], 1) = (&rests[..], r.contacts.len()) else {
                panic!("{what}: the rest lands once: {:?}", r.contacts)
            };
            let xs = edge_xs(&r.body, edge);
            assert!(
                xs.iter().all(|&x| x * p.x >= -1e-9),
                "{what}: the record's arc lies on the rest's side of the cut, x {xs:?}"
            );
        }
    }
}
