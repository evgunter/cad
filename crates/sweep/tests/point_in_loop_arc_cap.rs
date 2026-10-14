//! **`topo::point_in_loop` reads an arc-bearing loop on its edges' own
//! carriers.** A bore's cap ring is two semicircles through `(±1, 0)`:
//! its corner polygon is the chord between them, so a walk over the
//! corners would read a point inside the circle `Out` and the centre
//! `OnBoundary`. Read at `f64` on the bored brick's top-cap ring and at
//! `Interval` on a disc prism's top cap — the same circle.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::bores::bored_brick;
use crate::common::interval::{iv, p2};
use geom_core::{Band, Bounds, Decide, Interval, Point3, Tol, Vec3};
use topo::readback::vertex_point;
use topo::{Body, LoopContainment, LoopKey, OffPlaneCause, PointInLoopError, point_in_loop};

/// Each loop of `body` with its vertices' points, in face order.
fn loops<T: Decide + Bounds>(body: &Body<T>) -> Vec<(LoopKey, Vec<Point3<f64>>)> {
    body.faces()
        .flat_map(|(_, f)| std::iter::once(f.outer).chain(f.rings.iter().copied()))
        .map(|l| {
            let topo::LoopBoundary::Cycle { first } = body.get_loop(l).unwrap().boundary else {
                return (l, Vec::new());
            };
            let ps = body
                .loop_cycle(first)
                .unwrap()
                .into_iter()
                .map(|he| {
                    vertex_point(body, body.get_half_edge(he).unwrap().start)
                        .unwrap()
                        .map(|c| c.lo())
                })
                .collect();
            (l, ps)
        })
        .collect()
}

/// The one loop of `body` whose vertices satisfy `which`.
fn the_loop<T: Decide + Bounds>(
    body: &Body<T>,
    what: &str,
    which: impl Fn(&[Point3<f64>]) -> bool,
) -> LoopKey {
    let hits: Vec<LoopKey> = loops(body)
        .into_iter()
        .filter(|(_, ps)| !ps.is_empty() && which(ps))
        .map(|(l, _)| l)
        .collect();
    assert_eq!(hits.len(), 1, "one loop is {what}");
    hits[0]
}

/// The top cap's circle: every vertex on `z = 2.5`, one at `(1, 0)`.
fn top_circle<T: Decide + Bounds>(body: &Body<T>) -> LoopKey {
    the_loop(body, "the top cap's circle", |ps| {
        ps.iter().all(|p| p.z == 2.5) && ps.iter().any(|p| (p.x, p.y) == (1.0, 0.0))
    })
}

/// Inside the circle off the chord reads `In`; the centre, on the chord
/// but inside the circle, reads `In`; a point past the circle reads
/// `Out`; a point on the arc reads `OnBoundary`.
fn misreads<T: Decide + Bounds>(what: &str, body: &Body<T>) -> Vec<String> {
    let band = Band::linear(Tol::witness()).unwrap();
    let l = top_circle(body);
    let n = Vec3::new(0.0, 0.0, 1.0).map(T::from_f64);
    let mut wrong = Vec::new();
    for (name, (x, y), want) in [
        ("inside, off the chord", (0.0, 0.5), LoopContainment::In),
        ("the centre, on the chord", (0.0, 0.0), LoopContainment::In),
        ("past the arc", (0.0, 1.5), LoopContainment::Out),
        ("on the arc", (0.0, 1.0), LoopContainment::OnBoundary),
    ] {
        let q = Point3::new(x, y, 2.5).map(T::from_f64);
        let got = point_in_loop(body, l, n, q, band);
        if !matches!(got, Ok(v) if v == want) {
            wrong.push(format!(
                "{what}, {name} ({x}, {y}): want {want:?}, got {got:?}"
            ));
        }
    }
    wrong
}

#[test]
fn a_bored_bricks_cap_ring_is_read_on_its_circle() {
    let mut wrong = misreads("f64 bored brick", &bored_brick(0.0, 0.0, 1.0));
    let disc = sweep::test_support::prism::<Interval>(
        vec![(p2(1.0, 0.0), iv(1.0)), (p2(-1.0, 0.0), iv(1.0))],
        iv(2.5),
        Tol::witness(),
    );
    wrong.extend(misreads("Interval disc prism", &disc));
    assert!(wrong.is_empty(), "misread:\n{}", wrong.join("\n"));
}

/// The disc prism of radius 1 and height 2.5 about the `z` axis, at `T`.
fn disc<T: Decide + topo::AtRestPolicy>() -> Body<T> {
    let pt = |x: f64| (geom_core::Point2::new(x, 0.0).map(T::from_f64), T::one());
    sweep::test_support::prism::<T>(vec![pt(1.0), pt(-1.0)], T::from_f64(2.5), Tol::witness())
}

/// Each broken precondition `body` refuses on, as `(name, refusal)` where
/// the refusal is not the one wanted.
fn wrong_refusals<T: Decide + Bounds>(what: &str, body: &Body<T>) -> Vec<String> {
    let band = Band::linear(Tol::witness()).unwrap();
    let v = |x: f64, y: f64, z: f64| Vec3::new(x, y, z).map(T::from_f64);
    let p = |x: f64, y: f64, z: f64| Point3::new(x, y, z).map(T::from_f64);
    let cap = top_circle(body);
    // Either of the two half-cylinder walls: two lines and two arcs.
    let (wall, _) = loops(body)
        .into_iter()
        .find(|(_, ps)| ps.iter().any(|p| p.z == 0.0) && ps.iter().any(|p| p.z == 2.5))
        .expect("a wall's loop");
    let mut wrong = Vec::new();
    for (name, l, n, q, want) in [
        // The two corners are 2 apart along x.
        (
            "a normal across the corners",
            cap,
            v(1.0, 0.0, 0.0),
            p(0.0, 0.5, 2.5),
            "loop",
        ),
        // The corners lie on y = 0; the arcs do not.
        (
            "a normal through the corners",
            cap,
            v(0.0, 1.0, 0.0),
            p(0.0, 0.0, 2.5),
            "loop",
        ),
        (
            "a wall, which is not planar",
            wall,
            v(0.0, 0.0, 1.0),
            p(0.0, 0.0, 0.0),
            "loop",
        ),
        (
            "a point 49 m off the plane",
            cap,
            v(0.0, 0.0, 1.0),
            p(0.0, 0.5, 51.5),
            "query",
        ),
        (
            "a normal of length 2",
            cap,
            v(0.0, 0.0, 2.0),
            p(0.0, 0.5, 2.5),
            "normal",
        ),
    ] {
        let got = point_in_loop(body, l, n, q, band);
        let cause = match &got {
            Err(PointInLoopError::OffPlane(o)) => match o.cause {
                OffPlaneCause::Loop { .. } => "loop",
                OffPlaneCause::Query => "query",
                OffPlaneCause::NormalNotUnit => "normal",
            },
            _ => "",
        };
        if cause != want {
            wrong.push(format!(
                "{what}, {name}: want an off-plane {want}, got {got:?}"
            ));
        }
    }
    wrong
}

#[test]
fn a_plane_the_loop_or_the_point_is_not_in_refuses() {
    let mut wrong = wrong_refusals("f64", &disc::<f64>());
    wrong.extend(wrong_refusals("Interval", &disc::<Interval>()));
    assert!(
        wrong.is_empty(),
        "answered off the plane:\n{}",
        wrong.join("\n")
    );
}
