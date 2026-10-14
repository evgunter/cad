//! **A closed conic lying in the partner's face.** A one-segment circle
//! (one vertex, one edge) that lies in a face of the other operand is a
//! closed section loop with one site.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::TAU;

use geom_core::{Affine3, Arc2, Point2, Tol, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, Segment, SketchPlane};
use sweep::test_support::brick;
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::Body;

fn tol() -> Tol {
    Tol::witness()
}

/// A one-segment circle of radius `r` about `(cx, cy)`, its one vertex
/// at azimuth `az`.
fn circle(cx: f64, cy: f64, r: f64, az: f64) -> ProfileLoop<f64> {
    RawLoop::new([(
        Point2::new(cx + r * az.cos(), cy + r * az.sin()),
        Segment::Arc(Arc2 {
            centre: Point2::new(cx, cy),
            radius: r,
            sweep: TAU,
        }),
    )])
}

/// `loops` extruded from `z = z0` up by `h`.
fn extruded(loops: Vec<ProfileLoop<f64>>, z0: f64, h: f64) -> Body<f64> {
    let profile = Profile::new(SketchPlane::<f64>::xy(), loops)
        .validate(tol())
        .unwrap();
    let depth = Extrusion::Distance {
        depth: h,
        side: ExtrudeSide::Along,
    };
    let body = extrude(&profile, depth, tol()).unwrap().body;
    let up = Affine3::translation(Vec3::new(0.0, 0.0, z0));
    topo::transform_rigid(&body, &geom_core::test_support::rigid(&up, tol()), tol()).unwrap()
}

fn fin(what: &str, body: Body<f64>) -> topo::AtRestBody<f64> {
    topo::test_support::finished(what, body, tol())
}


fn show(name: &str, a: &topo::AtRestBody<f64>, b: &topo::AtRestBody<f64>, v: (f64, f64, f64)) {
    for (op, r, want) in crate::common::differential::every_op_both_orders(a, b, v, tol()) {
        let line = crate::common::differential::outcome(r, want, tol());
        eprintln!("PROBE {name}: {op}: {line}");
    }
}

#[test]
fn probe_all() {
    use core::f64::consts::{PI, SQRT_2};
    let only = std::env::var("PROBE").unwrap_or_default();
    if only.is_empty() || only == "boss" {
        let boss = fin("boss", extruded(vec![circle(0.0, 0.0, 0.3, 0.0)], 0.0, 1.0));
        let plate = fin("plate", brick((-1.0, 1.0), (-1.0, 1.0), (-0.5, 0.0), tol()));
        show("boss", &boss, &plate, (0.09 * PI, 2.0, 0.0));
    }
    if only.is_empty() || only == "coax" {
        let lo = fin("lo", extruded(vec![circle(0.0, 0.0, 1.0, 0.0)], 0.0, 1.0));
        let hi = fin("hi", extruded(vec![circle(0.0, 0.0, 1.0, 0.0)], 1.0, 1.0));
        show("coax", &lo, &hi, (PI, PI, 0.0));
    }
    if only.is_empty() || only == "coax2" {
        let two = |z0: f64| {
            let p = |x: f64, y: f64| Point2::new(x, y);
            let lp = RawLoop::new([
                (p(1.0, 0.0), Segment::Arc(Arc2 { centre: p(0.0, 0.0), radius: 1.0, sweep: PI })),
                (p(-1.0, 0.0), Segment::Arc(Arc2 { centre: p(0.0, 0.0), radius: 1.0, sweep: PI })),
            ]);
            fin("two", extruded(vec![lp], z0, 1.0))
        };
        show("coax2", &two(0.0), &two(1.0), (PI, PI, 0.0));
    }
    if only.is_empty() || only == "puck" {
        use geom_core::Point3;
        let (a, r) = (1.0_f64, 0.25_f64);
        let q = (core::f64::consts::FRAC_PI_2 / 4.0).tan();
        let ring = sweep::test_support::revolved_about_y(
            vec![
                (Point2::new(a, 0.0), 0.0),
                (Point2::new(a + r, 0.0), q),
                (Point2::new(a, r), 0.0),
            ],
            sweep::Revolution::Full,
            tol(),
        );
        let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_x(), PI / 2.0);
        let mut ring =
            topo::transform_rigid(&ring, &geom_core::test_support::rigid(&turn, tol()), tol()).unwrap();
        ring.merge_coplanar_faces(tol()).unwrap();
        let ring = fin("ring", ring);
        let puck = if std::env::var("TWO").is_ok() {
            let p = |x: f64, y: f64| Point2::new(x, y);
            let lp = RawLoop::new([
                (p(a, 0.0), Segment::Arc(Arc2 { centre: p(0.0, 0.0), radius: a, sweep: PI })),
                (p(-a, 0.0), Segment::Arc(Arc2 { centre: p(0.0, 0.0), radius: a, sweep: PI })),
            ]);
            fin("puck", extruded(vec![lp], 0.0, r))
        } else {
            fin("puck", extruded(vec![circle(0.0, 0.0, a, 0.0)], 0.0, r))
        };
        if std::env::var("PRISTINE").is_ok() {
            let b: &Body<f64> = &puck;
            for (k, e) in b.edges() {
                let c = b.get_curve_geom(e.curve).unwrap().certified().map(|c| (c.carrier().clone(), c.params()));
                eprintln!("PRISTINE edge {k:?} he+ {:?} start {:?} he- {:?} start {:?} curve {c:?}", e.he_plus, b.get_half_edge(e.he_plus).unwrap().start, e.he_minus, b.get_half_edge(e.he_minus).unwrap().start);
            }
            for (k, f) in b.faces() {
                eprintln!("PRISTINE face {k:?} sense {} outer {:?} surf {:?}", f.sense, f.outer, b.get_surface(f.surface));
            }
            for (k, h) in b.half_edges() {
                eprintln!("PRISTINE he {k:?} loop {:?} edge {:?}", h.parent_loop, h.edge);
            }
        }
        let vr = (PI * r * r / 4.0) * core::f64::consts::TAU * (a + 4.0 * r / (3.0 * PI));
        show("puck", &puck, &ring, (PI * a * a * r, vr, 0.0));
    }
    if only.is_empty() || only == "ball" {
        let a = 1.0f64;
        let z0 = (2.0 - a * a).sqrt();
        let tube = extruded(vec![circle(0.0, 0.0, a, 0.0)], z0, 2.0);
        let o = geom_core::Point3::origin();
        let place = Affine3::rotation_about_axis(o, Vec3::unit_x(), 1.2);
        let tube = topo::transform_rigid(&tube, &geom_core::test_support::rigid(&place, tol()), tol()).unwrap();
        let ball = fin("ball", sweep::test_support::ball_poled_z(SQRT_2, Vec3::zero(), tol()));
        let vb = 4.0 / 3.0 * PI * SQRT_2.powi(3);
        let h = SQRT_2 - z0;
        let cap = PI * h * h * (3.0 * SQRT_2 - h) / 3.0;
        show("ball", &fin("tube", tube), &ball, (PI * a * a * 2.0, vb, cap));
    }
}

#[test]
fn probe_boss_on_plate() {
    let boss = fin("boss", extruded(vec![circle(0.0, 0.0, 0.3, 0.0)], 0.0, 1.0));
    let plate = fin("plate", brick((-1.0, 1.0), (-1.0, 1.0), (-0.5, 0.0), tol()));
    let r = topo::union(&plate, &boss, tol());
    panic!("{:?}", r.map(|_| "a body"));
}
