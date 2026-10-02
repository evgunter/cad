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
use topo::{Body, LoopContainment, LoopKey, point_in_loop};

/// The loop of `body` whose every vertex lies on `z = 2.5` and one of
/// whose vertices is `(1, 0, 2.5)`: the top cap's circle.
fn top_circle<T: Decide + Bounds>(body: &Body<T>) -> LoopKey {
    let on_cap = |l: LoopKey| {
        let topo::LoopBoundary::Cycle { first } = body.get_loop(l).unwrap().boundary else {
            return false;
        };
        let ps: Vec<Point3<f64>> = body
            .loop_cycle(first)
            .unwrap()
            .into_iter()
            .map(|he| {
                vertex_point(body, body.get_half_edge(he).unwrap().start)
                    .unwrap()
                    .map(|c| c.lo())
            })
            .collect();
        ps.iter().all(|p| p.z == 2.5) && ps.iter().any(|p| (p.x, p.y) == (1.0, 0.0))
    };
    let mut hits = body
        .faces()
        .flat_map(|(_, f)| std::iter::once(f.outer).chain(f.rings.iter().copied()))
        .filter(|&l| on_cap(l));
    let l = hits.next().expect("the top cap's circle");
    assert!(hits.next().is_none(), "one loop is the top cap's circle");
    l
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
