//! **A keel wrapped round at both ends nests its ring inside an elliptic
//! section.** A right cylinder of radius 3 about the axis through
//! `(2, 2, 0)` tilted 15° about x, `1` either way along it, less the
//! V-trench over the `(y, z)` triangle `(2, 0)`, `(2.6, 2)`, `(1.4, 2)`
//! for `x ∈ [1.5, 2.5]`, which leaves through the top cap. Split by
//! `z = 0`, which holds the trench's keel, either way up: the section is
//! an ellipse, and the side above carries the keel as a zero-area ring
//! of two coincident edges inside it. The ring's place is read by
//! containment on the ellipse's own carrier, since no segment is decided
//! clear of an ellipse by the whole ellipse lying to one side of it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::bores::halves_at_rest;
use core::f64::consts::PI;
use geom_core::{Point2, Point3, Tol, Vec3};
use sweep::test_support::{finished, prism_on, sketch_from_axes};
use topo::{Body, BooleanResult, ContactRecords, LoopBoundary, mass_properties, subtract};

fn tol() -> Tol {
    Tol::witness()
}

/// The tilt about x.
const TILT: f64 = 15.0;

/// The cylinder less the trench.
fn keeled() -> Body<f64> {
    let (s, c) = TILT.to_radians().sin_cos();
    let axis = Vec3::new(0.0, -s, c);
    let centre = Point3::new(2.0, 2.0, 0.0);
    let base = sketch_from_axes(
        centre - axis,
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, c, s),
        tol(),
    );
    let disc = vec![(Point2::new(-3.0, 0.0), 1.0), (Point2::new(3.0, 0.0), 1.0)];
    let cylinder = prism_on(base, disc, 2.0, tol());
    let end = sketch_from_axes(
        Point3::new(1.5, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        tol(),
    );
    let v = vec![
        (Point2::new(2.0, 0.0), 0.0),
        (Point2::new(2.6, 2.0), 0.0),
        (Point2::new(1.4, 2.0), 0.0),
    ];
    let trench = prism_on(end, v, 1.0, tol());
    match subtract(
        &finished("the cylinder", cylinder, tol()),
        &finished("the trench", trench, tol()),
        tol(),
    ) {
        Ok(BooleanResult::Body(b)) => b.body.into_body(),
        other => panic!("the cylinder less the trench: {other:?}"),
    }
}

/// **The ring nests inside the ellipse**: each side keeps its closed-form
/// volume and passes tiers 1, 2, 3 and 3′, and the side above holds one
/// ring of two coincident distinct edges.
///
/// The plane holds the axis's midpoint, so it halves the cylinder:
/// `9π` a side. Above it the trench takes, per unit of its length,
/// `∫ max(0, (1 + s·u)/c − |u|/0.3) du` across it (`u = y − 2`, the cap
/// at `(1 + s·u)/c`, the walls at `|u| = 0.3·z`): two triangles of
/// height `1/c` over `1/(c/0.3 ∓ s)`.
#[test]
fn a_keel_ring_nests_inside_an_elliptic_section() {
    let (s, c) = TILT.to_radians().sin_cos();
    let trench = (1.0 / (c / 0.3 - s) + 1.0 / (c / 0.3 + s)) / (2.0 * c);
    let half = 9.0 * PI;
    let body = keeled();
    for up in [1.0, -1.0] {
        let what = format!("the plane's normal {up} z");
        let plane = topo::test_support::split_plane(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, up),
            tol(),
        );
        let [below, above] = halves_at_rest(&what, &body, &plane);
        let (walls, floor) = if up > 0.0 {
            (above, below)
        } else {
            (below, above)
        };
        for (side, b, volume) in [("walls", &walls, half - trench), ("floor", &floor, half)] {
            let what = format!("{what}, the {side} side");
            assert_eq!(topo::validate_closed(b), Ok(()), "{what}: tier 2");
            assert_eq!(
                topo::validate_pseudomanifold(b, &ContactRecords::default(), tol()),
                Ok(()),
                "{what}: tier 3′"
            );
            let got = mass_properties(b, tol()).unwrap().volume;
            assert!(
                (got - volume).abs() < 1e-9 * volume,
                "{what}: volume {got}, closed form {volume}"
            );
        }
        let rings: Vec<Vec<topo::EdgeKey>> = walls
            .faces()
            .flat_map(|(_, f)| f.rings.clone())
            .filter_map(|l| match walls.get_loop(l).unwrap().boundary {
                LoopBoundary::Cycle { first } => Some(
                    walls
                        .loop_cycle(first)
                        .unwrap()
                        .into_iter()
                        .map(|he| walls.get_half_edge(he).unwrap().edge)
                        .collect(),
                ),
                LoopBoundary::Empty { .. } => None,
            })
            .filter(|edges: &Vec<_>| edges.len() == 2)
            .collect();
        let ends = |e: topo::EdgeKey| {
            let he = walls.get_edge(e).unwrap().he_plus;
            let mut v = [
                walls.get_half_edge(he).unwrap().start,
                walls.half_edge_end(he).unwrap(),
            ];
            v.sort();
            v
        };
        let [ring] = rings.as_slice() else {
            panic!("{what}: two-edge rings {rings:?}, not one");
        };
        assert!(
            ring[0] != ring[1] && ends(ring[0]) == ends(ring[1]),
            "{what}: the ring {ring:?} is not two distinct edges between one pair of vertices"
        );
    }
}
