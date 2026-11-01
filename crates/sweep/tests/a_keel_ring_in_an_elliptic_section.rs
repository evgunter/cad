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
use geom_core::RigidMotion;
use geom_core::{Point2, Point3, Tol, Vec3};
use profile::SketchPlane;
use sweep::test_support::{finished, prism_at, prism_on, sketch_from_axes};
use topo::{Body, BooleanResult, ContactRecords, LoopBoundary, subtract, union};

fn tol() -> Tol {
    Tol::witness()
}

/// The tilt about x.
const TILT: f64 = 15.0;

/// The cylinder of radius 3 about the axis through `(2, 2, 0)` tilted
/// `tilt` degrees about x, `half` either way along it.
fn cylinder(tilt: f64, half: f64) -> Body<f64> {
    let (s, c) = tilt.to_radians().sin_cos();
    let axis = Vec3::new(0.0, -s, c);
    let base = sketch_from_axes(
        Point3::new(2.0, 2.0, 0.0) - axis * half,
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, c, s),
        tol(),
    );
    let disc = vec![(Point2::new(-3.0, 0.0), 1.0), (Point2::new(3.0, 0.0), 1.0)];
    prism_on(base, disc, 2.0 * half, tol())
}

/// The V-trench whose keel runs from `(1.5, 2, 0)` to `(2.5, 2, 0)`.
fn trench() -> Body<f64> {
    let end = SketchPlane::from_frame(RigidMotion::axes_yz(Point3::new(1.5, 0.0, 0.0)));
    let v = vec![
        (Point2::new(2.0, 0.0), 0.0),
        (Point2::new(2.6, 2.0), 0.0),
        (Point2::new(1.4, 2.0), 0.0),
    ];
    prism_on(end, v, 1.0, tol())
}

/// The square `[lo, hi]²` over `z ∈ [z0, z0 + h]`.
fn square(lo: f64, hi: f64, z0: f64, h: f64) -> Body<f64> {
    let corners = [(lo, lo), (hi, lo), (hi, hi), (lo, hi)];
    prism_at(
        corners
            .iter()
            .map(|&(x, y)| (Point2::new(x, y), 0.0))
            .collect(),
        z0,
        h,
        tol(),
    )
}

/// A Boolean door.
type Door = fn(
    &topo::AtRestBody<f64>,
    &topo::AtRestBody<f64>,
    Tol,
) -> Result<BooleanResult<f64>, topo::BooleanError>;

/// `a` op `b`, both finished.
fn op(what: &str, f: Door, a: Body<f64>, b: Body<f64>) -> Body<f64> {
    match f(&finished("a", a, tol()), &finished("b", b, tol()), tol()) {
        Ok(BooleanResult::Body(r)) => r.body.into_body(),
        other => panic!("{what}: {other:?}"),
    }
}

/// The cylinder less the trench.
fn keeled() -> Body<f64> {
    op(
        "the cylinder less the trench",
        subtract,
        cylinder(TILT, 1.0),
        trench(),
    )
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
            let got = topo::test_support::pinned_volume(
                "the measured body",
                &topo::test_support::finished("the measured body", topo::Body::clone(b), tol()),
                tol(),
            )
            .volume;
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

/// **A ring on an island inside a hole of an elliptic section nests on
/// the island.** The cylinder `1.5` either way along its axis, tilted 0°
/// and 15°, less the square pocket `[0.5, 3.5]² × [−0.5, 3]`, with the
/// post `[1, 3]² × [−0.8, 1.2]` put back inside it, less the trench,
/// whose keel lies on the post. The section at `z = 0` is the ellipse
/// with the pocket's square as a hole and the post's square as an
/// island in it. Tilted, the nesting rule leaves the square hole on a
/// face of its own (no segment is decided clear of an ellipse), and the
/// ring goes to the island, the innermost outline holding it that no
/// hole takes it out of. Each side passes tiers 1, 2 and 3 and holds its
/// closed-form volume; tier 3′ passes untilted, and tilted refuses, as it
/// refuses the same split without the trench, a filed finding.
///
/// Above the plane: the half cylinder, `13.5π`, less the pocket under
/// the cap, `3 · 3 · 1.5 / c`, plus the post, `2 · 2 · 1.2`, less the
/// trench in it, `0.3 · 1.2²`. Below: `13.5π`, less the pocket's
/// `9 · 0.5`, plus the post's `4 · 0.5`.
#[test]
fn a_keel_ring_on_an_island_in_a_hole_nests_on_the_island() {
    for tilt in [0.0, TILT] {
        let c = tilt.to_radians().cos();
        let half = 13.5 * PI;
        let (walls_volume, floor_volume) = (
            half - 13.5 / c + 4.8 - 0.3 * 1.2f64.powi(2),
            half - 4.5 + 2.0,
        );
        let pocketed = op(
            "the cylinder less the pocket",
            subtract,
            cylinder(tilt, 1.5),
            square(0.5, 3.5, -0.5, 3.5),
        );
        let posted = op(
            "the post put back",
            union,
            pocketed,
            square(1.0, 3.0, -0.8, 2.0),
        );
        let body = op("less the trench", subtract, posted.clone(), trench());
        for up in [1.0, -1.0] {
            let what = format!("tilted {tilt}°, the plane's normal {up} z");
            let plane = topo::test_support::split_plane(
                Point3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, up),
                tol(),
            );
            for (side, half) in halves_at_rest(&what, &posted, &plane).iter().enumerate() {
                let pseudo = topo::validate_pseudomanifold(half, &ContactRecords::default(), tol());
                assert_eq!(
                    pseudo.is_ok(),
                    tilt == 0.0,
                    "{what}, no trench, side {side}: tier 3′ {pseudo:?}"
                );
            }
            let [below, above] = halves_at_rest(&what, &body, &plane);
            let (walls, floor) = if up > 0.0 {
                (above, below)
            } else {
                (below, above)
            };
            for (side, b, volume) in [
                ("walls", &walls, walls_volume),
                ("floor", &floor, floor_volume),
            ] {
                let what = format!("{what}, the {side} side");
                assert_eq!(topo::validate_closed(b), Ok(()), "{what}: tier 2");
                // Tilted, the square hole keeps a face of its own over the
                // ellipse's, and tier 3′ refuses each side, trench or no
                // trench (`work/cleave/a-hole-in-an-elliptic-section-keeps-a-face-of-its-own-that-tier-3-prime-refuses.md`).
                let pseudo = topo::validate_pseudomanifold(b, &ContactRecords::default(), tol());
                assert_eq!(pseudo.is_ok(), tilt == 0.0, "{what}: tier 3′ {pseudo:?}");
                let got = topo::test_support::pinned_volume(
                    "the measured body",
                    &topo::test_support::finished("the measured body", topo::Body::clone(b), tol()),
                    tol(),
                )
                .volume;
                assert!(
                    (got - volume).abs() < 1e-9 * volume,
                    "{what}: volume {got}, closed form {volume}"
                );
            }
            // The ring's face is the island's: its outline is the post's square.
            let ring_face = walls.faces().find(|(_, f)| {
                f.rings
                    .iter()
                    .any(|&l| match walls.get_loop(l).unwrap().boundary {
                        LoopBoundary::Cycle { first } => {
                            walls.loop_cycle(first).unwrap().len() == 2
                        }
                        LoopBoundary::Empty { .. } => false,
                    })
            });
            let Some((_, face)) = ring_face else {
                panic!("{what}: no face holds a two-edge ring");
            };
            let LoopBoundary::Cycle { first } = walls.get_loop(face.outer).unwrap().boundary else {
                panic!("{what}: the ring's face has no outline");
            };
            let corners: Vec<_> = walls
                .loop_cycle(first)
                .unwrap()
                .into_iter()
                .map(|he| {
                    let v = walls.get_half_edge(he).unwrap().start;
                    *walls.get_point(walls.get_vertex(v).unwrap().point).unwrap()
                })
                .collect();
            assert!(
                corners.len() == 4
                    && corners.iter().all(|p| {
                        [p.x, p.y]
                            .iter()
                            .all(|&a| (a - 1.0).abs() < 1e-9 || (a - 3.0).abs() < 1e-9)
                    }),
                "{what}: the ring's face is bounded by {corners:?}, not the post's square"
            );
        }
    }
}
