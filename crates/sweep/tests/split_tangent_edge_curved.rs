//! A plane along an edge of a curved operand: an in-plane line edge
//! between a plane and a cylinder (convex on a D-shaped bar, reflex on a
//! D-shaped hole), and the concave graze of a round hole, whose
//! refusal is correct only so long as it is not replaced by an answer
//! that puts the hole on the wrong side.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::ExtrudeSide;
use sweep::{Extrusion, extrude};
use topo::splitting::{SplitPart, SplitPlane, split};
use topo::{Body, mass_properties, validate_closed};

type Loop = Vec<((f64, f64), f64)>;

fn extruded(loops: Vec<Loop>) -> Body<f64> {
    let lps = loops
        .into_iter()
        .map(|l| {
            bulge_loop(
                l.into_iter()
                    .map(|((x, y), b)| (Point2::new(x, y), b))
                    .collect(),
            )
        })
        .collect();
    let vp = Profile::new(SketchPlane::xy(), lps)
        .validate(Tol::witness())
        .unwrap();
    extrude(
        &vp,
        Extrusion::Distance {
            depth: 1.0,
            side: ExtrudeSide::Along,
        },
        Tol::witness(),
    )
    .unwrap()
    .body
}

fn plane(o: (f64, f64), n: (f64, f64)) -> SplitPlane<f64> {
    let l = n.0.hypot(n.1);
    topo::test_support::split_plane(
        Point3::new(o.0, o.1, 0.0),
        Vec3::new(n.0 / l, n.1 / l, 0.0),
        Tol::witness(),
    )
}

fn volume(label: &str, part: &SplitPart<f64>) -> Option<f64> {
    part.body().map(|b| {
        assert_eq!(validate_closed(b), Ok(()), "{label}");
        mass_properties(b, Tol::witness()).unwrap().volume
    })
}

fn close(got: Option<f64>, want: Option<f64>) -> bool {
    match (got, want) {
        (None, None) => true,
        (Some(g), Some(w)) => (g - w).abs() <= 1e-9 * w.max(1.0),
        _ => false,
    }
}

fn outer() -> Loop {
    vec![
        ((-2.0, -2.0), 0.0),
        ((2.0, -2.0), 0.0),
        ((2.0, 2.0), 0.0),
        ((-2.0, 2.0), 0.0),
    ]
}

/// The half-disc bar (arc from (0.5, 0) over (0, 0.5), then the
/// diameter back): its line edge at (0.5, 0) is convex, flat face to
/// cylinder wall. A plane along it with both faces on one side, tilted
/// or tangent to the wall, puts the whole bar on that side.
#[test]
fn a_plane_along_a_convex_flat_to_wall_edge_lands_the_bar_whole() {
    let d = extruded(vec![vec![((0.5, 0.0), 1.0), ((-0.5, 0.0), 0.0)]]);
    let v = std::f64::consts::PI / 8.0;
    for n in [(-1.0, 1.0), (1.0, -1.0), (-1.0, 0.0), (1.0, 0.0)] {
        let label = format!("n = {n:?}");
        let r = split(&d, &plane((0.5, 0.0), n), Tol::witness())
            .unwrap_or_else(|e| panic!("{label}: {e:?}"));
        let want = if n.0 < 0.0 {
            (Some(v), None)
        } else {
            (None, Some(v))
        };
        let got = (volume(&label, &r.above), volume(&label, &r.below));
        assert!(
            close(got.0, want.0) && close(got.1, want.1),
            "{label}: {got:?}, want {want:?}"
        );
    }
}

/// The same edge as a D-shaped HOLE in a 4 × 4 square: reflex from the
/// material, with the reversed-sense wall. Below −x + y = −0.5 lies
/// 3.5² / 2 of the square; the hole lies wholly above.
#[test]
fn a_plane_along_a_reflex_flat_to_wall_edge_cuts_through() {
    let above = 16.0 - 6.125 - std::f64::consts::PI / 8.0;
    for (label, hole) in [
        ("cw", vec![((0.5, 0.0), 0.0), ((-0.5, 0.0), -1.0)]),
        ("ccw", vec![((0.5, 0.0), 1.0), ((-0.5, 0.0), 0.0)]),
    ] {
        let b = extruded(vec![outer(), hole]);
        for (s, want) in [(1.0, (above, 6.125)), (-1.0, (6.125, above))] {
            let label = format!("{label}, s = {s}");
            let r = split(&b, &plane((0.5, 0.0), (-s, s)), Tol::witness())
                .unwrap_or_else(|e| panic!("{label}: {e:?}"));
            let got = (volume(&label, &r.above), volume(&label, &r.below));
            assert!(
                close(got.0, Some(want.0)) && close(got.1, Some(want.1)),
                "{label}: {got:?}, want {want:?}"
            );
        }
    }
}

/// A round hole grazed from inside, at a ruling (y = 0.5) and at its
/// seam (x = 0.5): material on both sides. These refuse today. A rule
/// (b) that sent the graze's bisector duplicate or smooth seam with its
/// neighbours instead returns CLOSED halves with the hole on the wrong
/// side under one orientation (10 and 5.21 for 9.21 and 6), so the row
/// holds any answer to the true volumes and lets a refusal pass.
#[test]
fn a_concave_graze_never_answers_with_the_hole_on_the_wrong_side() {
    let hole_area = std::f64::consts::PI / 4.0;
    for (label, hole) in [
        ("cw", vec![((-0.5, 0.0), -1.0), ((0.5, 0.0), -1.0)]),
        ("ccw", vec![((-0.5, 0.0), 1.0), ((0.5, 0.0), 1.0)]),
    ] {
        let b = extruded(vec![outer(), hole]);
        // (origin, +normal, (above, below) under +normal)
        let cases = [
            ((0.0, 0.5), (0.0, 1.0), (6.0, 10.0 - hole_area)),
            ((0.5, 0.0), (1.0, 0.0), (6.0, 10.0 - hole_area)),
        ];
        for (o, n, (a, bl)) in cases {
            for (s, want) in [(1.0, (a, bl)), (-1.0, (bl, a))] {
                let label = format!("{label}, at {o:?}, s = {s}");
                if let Ok(r) = split(&b, &plane(o, (s * n.0, s * n.1)), Tol::witness()) {
                    let got = (volume(&label, &r.above), volume(&label, &r.below));
                    assert!(
                        close(got.0, Some(want.0)) && close(got.1, Some(want.1)),
                        "{label}: {got:?}, want {want:?}"
                    );
                }
            }
        }
    }
}

/// A cylinder grazed from outside, along a ruling (y = 0.5, the cap
/// rims' straight-sector duplicates) and along its seam (x = 0.5, a
/// smooth edge): all its material is on one side, yet the graze refuses
/// at the join's zero-area net, because rule (b) sends both entries
/// across by default (`splitting/rules.rs`). This row and the concave
/// graze above hold that default from both sides: a flip answers here
/// and turns the concave row red. It flips with
/// `work/cleave/split-refuses-a-convex-graze-of-a-curved-wall.md`.
#[test]
fn a_convex_graze_of_a_cylinder_refuses_on_area() {
    let disc = extruded(vec![vec![((-0.5, 0.0), 1.0), ((0.5, 0.0), 1.0)]]);
    for (o, n) in [((0.0, 0.5), (0.0, 1.0)), ((0.5, 0.0), (1.0, 0.0))] {
        for s in [1.0, -1.0] {
            let label = format!("at {o:?}, s = {s}");
            let r = split(&disc, &plane(o, (s * n.0, s * n.1)), Tol::witness());
            assert!(
                matches!(
                    r,
                    Err(topo::SplitError::Join(
                        topo::SplitJoinError::DegenerateSection { .. }
                    ))
                ),
                "{label}: {:?}",
                r.map(|r| (r.above.body().is_some(), r.below.body().is_some()))
            );
        }
    }
}
