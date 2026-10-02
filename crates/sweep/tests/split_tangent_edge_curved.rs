//! A plane along an edge of a curved operand: an in-plane line edge
//! between a plane and a cylinder (convex on a D-shaped bar, reflex on a
//! D-shaped hole); the convex graze of a cylinder and of a cone, which
//! lands the body whole on its material's side; and the concave graze
//! of a round hole and of a conical socket, whose refusal is correct
//! only so long as it is not replaced by an answer that puts the hole
//! on the wrong side.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
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
    extrude(&vp, Extrusion::Distance(1.0), Tol::witness())
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
/// seam (x = 0.5): material on both sides, and the hole's piece meets
/// the cut face along a knife edge the split cannot declare, so these
/// refuse (`wedge_end_doors` pins that refusal). The row holds any
/// answer to the true volumes and lets a refusal pass.
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
/// smooth edge): its wall bends into its material, so all of it lies on
/// the material side and the whole cylinder lands there, under either
/// orientation of the plane. The section the plane makes is empty.
#[test]
fn a_convex_graze_of_a_cylinder_lands_it_whole() {
    let disc = extruded(vec![vec![((-0.5, 0.0), 1.0), ((0.5, 0.0), 1.0)]]);
    let v = std::f64::consts::PI / 4.0;
    for (o, n) in [((0.0, 0.5), (0.0, 1.0)), ((0.5, 0.0), (1.0, 0.0))] {
        for s in [1.0, -1.0] {
            let label = format!("at {o:?}, s = {s}");
            let p = plane(o, (s * n.0, s * n.1));
            let section = topo::splitting::plane_section(&disc, &p, Tol::witness())
                .unwrap_or_else(|e| panic!("{label}: section: {e:?}"));
            assert!(section.regions.is_empty(), "{label}: a section");
            let r = split(&disc, &p, Tol::witness()).unwrap_or_else(|e| panic!("{label}: {e:?}"));
            let want = if s > 0.0 {
                (None, Some(v))
            } else {
                (Some(v), None)
            };
            let got = (volume(&label, &r.above), volume(&label, &r.below));
            assert!(
                close(got.0, want.0) && close(got.1, want.1),
                "{label}: {got:?}, want {want:?}"
            );
        }
    }
}

/// A convex graze beside a real cut: a U whose left arm ends in an arc
/// of bulge 1/2 over its unit top (apex at y = 2.25) and whose right
/// arm stands to y = 3, split by y = 2.25. The plane touches the arc's
/// apex and cuts the right arm; the contact adds nothing to the
/// section, so the right arm's top, 1 × 0.75, is the one piece across
/// it, and the circular segment stays below.
#[test]
fn a_convex_graze_beside_a_real_cut_adds_nothing_to_the_section() {
    let b = 0.5f64;
    let u = extruded(vec![vec![
        ((0.0, 0.0), 0.0),
        ((3.0, 0.0), 0.0),
        ((3.0, 3.0), 0.0),
        ((2.0, 3.0), 0.0),
        ((2.0, 1.0), 0.0),
        ((1.0, 1.0), 0.0),
        ((1.0, 2.0), b),
        ((0.0, 2.0), 0.0),
    ]]);
    // The segment over a unit chord: angle 4·atan(b), radius
    // (1 + b²)/(4b).
    let (angle, radius) = (4.0 * b.atan(), (1.0 + b * b) / (4.0 * b));
    let segment = radius * radius * (angle - angle.sin()) / 2.0;
    let (top, rest) = (0.75, 6.0 + segment - 0.75);
    for s in [1.0, -1.0] {
        let label = format!("s = {s}");
        let r = split(&u, &plane((0.5, 2.0 + b / 2.0), (0.0, s)), Tol::witness())
            .unwrap_or_else(|e| panic!("{label}: {e:?}"));
        let want = if s > 0.0 { (top, rest) } else { (rest, top) };
        let got = (volume(&label, &r.above), volume(&label, &r.below));
        assert!(
            close(got.0, Some(want.0)) && close(got.1, Some(want.1)),
            "{label}: {got:?}, want {want:?}"
        );
    }
}

/// A body of revolution about y: `pts` is the x–y profile, revolved a
/// full turn.
fn revolved(pts: &[(f64, f64)]) -> Body<f64> {
    use crate::revolve_common::{axis_y, validated};
    use profile::RawLoop;
    let lp = profile::ProfileLoop::polygon(pts.iter().map(|&(x, y)| Point2::new(x, y)));
    sweep::revolve(
        &validated(vec![lp]),
        axis_y(),
        sweep::Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// The plane tangent to the cone `ρ = r0 + slope·y` along its ruling at
/// azimuth `u` (a unit x–z direction), with its normal `s ·` the cone's
/// outward gradient `u − slope·ŷ`.
fn cone_tangent(r0: f64, slope: f64, u: (f64, f64), s: f64) -> SplitPlane<f64> {
    let n = Vec3::new(s * u.0, -s * slope, s * u.1);
    topo::test_support::split_plane(
        Point3::new(r0 * u.0, 0.0, r0 * u.1),
        n / n.norm(),
        Tol::witness(),
    )
}

/// The four azimuths a cone row is grazed at: the revolve's seam (+x),
/// its far side, and the two between, where no edge runs.
const AZIMUTHS: [(f64, f64); 4] = [(1.0, 0.0), (0.0, 1.0), (-1.0, 0.0), (0.0, -1.0)];

/// A cone frustum grazed from outside along a ruling, narrowing and
/// widening upward (the two nappes of the stored cone), at its seam and
/// away from it: the wall bends into its material, so the whole frustum,
/// `7π/12`, lands on the material side.
#[test]
fn a_convex_graze_of_a_cone_lands_it_whole() {
    let v = 7.0 * std::f64::consts::PI / 12.0;
    for (name, body, r0, slope) in [
        (
            "narrowing",
            revolved(&[(0.0, 0.0), (1.0, 0.0), (0.5, 1.0), (0.0, 1.0)]),
            1.0,
            -0.5,
        ),
        (
            "widening",
            revolved(&[(0.0, 0.0), (0.5, 0.0), (1.0, 1.0), (0.0, 1.0)]),
            0.5,
            0.5,
        ),
    ] {
        for u in AZIMUTHS {
            for s in [1.0, -1.0] {
                let label = format!("{name}, u = {u:?}, s = {s}");
                let r = split(&body, &cone_tangent(r0, slope, u, s), Tol::witness())
                    .unwrap_or_else(|e| panic!("{label}: {e:?}"));
                let want = if s > 0.0 {
                    (None, Some(v))
                } else {
                    (Some(v), None)
                };
                let got = (volume(&label, &r.above), volume(&label, &r.below));
                assert!(
                    close(got.0, want.0) && close(got.1, want.1),
                    "{label}: {got:?}, want {want:?}"
                );
            }
        }
    }
}

/// A conical socket grazed from inside: a cylinder of radius 3 with the
/// narrowing frustum's cone as a through hole, and the plane tangent to
/// the hole's wall along a ruling. Material lies on both sides. Every
/// slice at height y is the disc of radius 3 cut by a chord at the
/// hole's radius `c = 1 − y/2`, tangent to the hole, so the side the
/// hole is not on holds `∫ 9·acos(c/3) − c·√(9 − c²) dy` and the rest
/// is the other side's. As the round hole above, the row holds any
/// answer to those volumes and lets a refusal pass.
#[test]
fn a_concave_graze_of_a_cone_never_answers_with_the_hole_on_the_wrong_side() {
    use std::f64::consts::PI;
    let body = revolved(&[(1.0, 0.0), (3.0, 0.0), (3.0, 1.0), (0.5, 1.0)]);
    let segment = |y: f64| {
        let c = 1.0 - y / 2.0;
        9.0 * (c / 3.0).acos() - c * (9.0 - c * c).sqrt()
    };
    // Composite Simpson; the integrand is smooth on [0, 1].
    let m = 2000;
    let h = 1.0 / f64::from(m);
    let beyond = (0..=m)
        .map(|i| {
            let w = if i == 0 || i == m {
                1.0
            } else if i % 2 == 1 {
                4.0
            } else {
                2.0
            };
            w * segment(f64::from(i) * h)
        })
        .sum::<f64>()
        * h
        / 3.0;
    let rest = 9.0 * PI - 7.0 * PI / 12.0 - beyond;
    for u in AZIMUTHS {
        for s in [1.0, -1.0] {
            let label = format!("u = {u:?}, s = {s}");
            // The hole's gradient points into the material, so `s = 1`
            // puts the far segment Above.
            let want = if s > 0.0 {
                (beyond, rest)
            } else {
                (rest, beyond)
            };
            if let Ok(r) = split(&body, &cone_tangent(1.0, -0.5, u, s), Tol::witness()) {
                let got = (volume(&label, &r.above), volume(&label, &r.below));
                assert!(
                    close(got.0, Some(want.0)) && close(got.1, Some(want.1)),
                    "{label}: {got:?}, want {want:?}"
                );
            }
        }
    }
}
