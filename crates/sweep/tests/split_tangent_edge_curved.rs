//! A plane along an edge of a curved operand: an in-plane line edge
//! between a plane and a cylinder (convex on a D-shaped bar, reflex on a
//! D-shaped hole); the convex graze of a cylinder and of a cone, which
//! lands the body whole on its material's side; and the concave graze
//! of a round hole, a conical socket, a counterbore and a filleted
//! hole, which refuses the knife edge it would mint.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::SurfaceKind;
use geom_core::{Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::ExtrudeSide;
use sweep::{Extrusion, extrude};
use topo::splitting::{SplitPart, SplitPlane, split};
use topo::{
    Body, SplitError, SplitFinishError, SplitReduceError, mass_properties, validate_closed,
};

type Loop = Vec<((f64, f64), f64)>;

fn extruded(loops: Vec<Loop>) -> Body<f64> {
    extruded_loops(
        loops
            .into_iter()
            .map(|l| {
                bulge_loop(
                    l.into_iter()
                        .map(|((x, y), b)| (Point2::new(x, y), b))
                        .collect(),
                )
            })
            .collect(),
    )
}

fn extruded_loops(loops: Vec<profile::ProfileLoop<f64>>) -> Body<f64> {
    let vp = Profile::new(SketchPlane::xy(), loops)
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
    let d = sweep::test_support::finished("the d", d, Tol::witness());
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
        let b = sweep::test_support::finished("the b", b, Tol::witness());
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

/// Azimuths a graze is posed at: the quarter points (the profile's
/// seams and the rulings between them) and six off-axis angles.
const THETAS: [f64; 10] = [
    0.0,
    std::f64::consts::FRAC_PI_2,
    std::f64::consts::PI,
    3.0 * std::f64::consts::FRAC_PI_2,
    0.3,
    1.1,
    2.0,
    2.9,
    4.0,
    5.5,
];

fn unit(t: f64) -> (f64, f64) {
    (t.cos(), t.sin())
}

/// A concave graze has material on both sides of the plane, and the
/// hole's piece would meet the cut face tangentially along the contact:
/// a knife edge the split, having no declaration channel, cannot make.
/// Rule (b) reads the graze and refuses it there, naming the wall
/// (`ConcaveGraze`). An answer here is wrong whatever its volumes, and
/// it need not even carry an edge for the contact: sent with its
/// material, the graze answers the true volumes with the hole's wall
/// touching the cut face's interior along the ruling, which tier 3
/// passes.
fn concave_graze_refuses(label: &str, body: &Body<f64>, p: &SplitPlane<f64>, wall: SurfaceKind) {
    let operand = sweep::test_support::finished("the operand", body.clone(), Tol::witness());
    match split(&operand, p, Tol::witness()) {
        Err(SplitError::Reduce(SplitReduceError::ConcaveGraze { face, .. })) => {
            let named = body
                .get_face(face)
                .and_then(|f| body.get_surface(f.surface));
            assert_eq!(
                named.map(geom::Surface::kind),
                Some(wall),
                "{label}: the refusal names the grazed wall"
            );
        }
        Ok(r) => {
            let got = (volume(label, &r.above), volume(label, &r.below));
            panic!("{label}: answered {got:?}");
        }
        Err(e) => panic!("{label}: refused {e:?}, not the graze's knife edge"),
    }
}

/// A round hole (radius 0.5, both loop orientations) in a 4 × 4 plate,
/// grazed from inside at every azimuth of [`THETAS`] with either
/// normal: it refuses ([`concave_graze_refuses`]).
#[test]
fn a_concave_graze_of_a_round_hole_refuses() {
    for (label, hole) in [
        ("cw", vec![((-0.5, 0.0), -1.0), ((0.5, 0.0), -1.0)]),
        ("ccw", vec![((-0.5, 0.0), 1.0), ((0.5, 0.0), 1.0)]),
    ] {
        let b = extruded(vec![outer(), hole]);
        for t in THETAS {
            let u = unit(t);
            for s in [1.0, -1.0] {
                let label = format!("{label}, θ = {t}, s = {s}");
                concave_graze_refuses(
                    &label,
                    &b,
                    &plane((0.5 * u.0, 0.5 * u.1), (s * u.0, s * u.1)),
                    SurfaceKind::Cylinder,
                );
            }
        }
    }
}

/// A cylinder grazed from outside at every azimuth of [`THETAS`]: along
/// its seams (x = ±0.5, smooth edges), along the rulings between them
/// (the cap rims' straight-sector duplicates) and off-axis. Its wall
/// bends into its material, so all of it lies on the material side and
/// the whole cylinder lands there, under either orientation of the
/// plane. The section the plane makes is empty.
#[test]
fn a_convex_graze_of_a_cylinder_lands_it_whole() {
    let disc = extruded(vec![vec![((-0.5, 0.0), 1.0), ((0.5, 0.0), 1.0)]]);
    let disc = sweep::test_support::finished("the disc", disc, Tol::witness());
    let v = std::f64::consts::PI / 4.0;
    for t in THETAS {
        let u = unit(t);
        for s in [1.0, -1.0] {
            let label = format!("θ = {t}, s = {s}");
            let p = plane((0.5 * u.0, 0.5 * u.1), (s * u.0, s * u.1));
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

/// The convex ruling and seam grazes at `T = Interval`: the whole
/// cylinder on one side, its volume enclosing π/4.
#[test]
fn a_convex_graze_of_a_cylinder_lands_it_whole_at_interval() {
    use crate::common::interval::{iv, p2, p3, v3};
    use geom_core::{Bounds, Interval};
    let lp = bulge_loop(vec![(p2(-0.5, 0.0), iv(1.0)), (p2(0.5, 0.0), iv(1.0))]);
    let vp = Profile::new(SketchPlane::<Interval>::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let body = extrude(
        &vp,
        Extrusion::Distance {
            depth: iv(1.0),
            side: ExtrudeSide::Along,
        },
        Tol::witness(),
    )
    .unwrap()
    .body;
    let body = sweep::test_support::finished("the body", body, Tol::witness());
    let pi4 = std::f64::consts::PI / 4.0;
    for (o, n, above) in [
        (p3(0.0, 0.5, 0.0), v3(0.0, 1.0, 0.0), false),
        (p3(0.0, 0.5, 0.0), v3(0.0, -1.0, 0.0), true),
        (p3(0.5, 0.0, 0.0), v3(1.0, 0.0, 0.0), false),
    ] {
        let label = format!("{o:?}, {n:?}");
        let plane = topo::test_support::split_plane(o, n, Tol::witness());
        let r = split(&body, &plane, Tol::witness()).unwrap_or_else(|e| panic!("{label}: {e:?}"));
        let (whole, empty) = if above {
            (&r.above, &r.below)
        } else {
            (&r.below, &r.above)
        };
        assert!(empty.body().is_none(), "{label}: two-sided");
        let v = mass_properties(whole.body().expect("the whole cylinder"), Tol::witness())
            .unwrap()
            .volume;
        assert!(
            v.lo() - 1e-12 <= pi4 && pi4 <= v.hi() + 1e-12,
            "{label}: [{}, {}]",
            v.lo(),
            v.hi()
        );
    }
}

/// Planes just off the cylinder's top ruling, inside (a thin circular
/// segment above) and outside (the whole below), both normals. An
/// answer is the true one, or — inside, where the segment is below
/// rounding scale — the whole; the tightest inside offsets refuse.
#[test]
fn a_near_graze_of_a_cylinder_never_answers_wrongly() {
    let disc = extruded(vec![vec![((-0.5, 0.0), 1.0), ((0.5, 0.0), 1.0)]]);
    let disc = sweep::test_support::finished("the disc", disc, Tol::witness());
    let (r, v) = (0.5f64, std::f64::consts::PI / 4.0);
    for d in [1e-13, 1e-11, 1e-9, 1e-7, 1e-5, 1e-4] {
        for (inside, off) in [(true, r - d), (false, r + d)] {
            let seg = if inside {
                r * r * (off / r).acos() - off * (r * r - off * off).sqrt()
            } else {
                0.0
            };
            for s in [1.0, -1.0] {
                let label = format!("δ = {d:e}, inside = {inside}, s = {s}");
                let Ok(res) = split(&disc, &plane((0.0, off), (0.0, s)), Tol::witness()) else {
                    continue;
                };
                let opt = |x: f64| (x > 0.0).then_some(x);
                let want = if s > 0.0 {
                    (opt(seg), opt(v - seg))
                } else {
                    (opt(v - seg), opt(seg))
                };
                let got = (volume(&label, &res.above), volume(&label, &res.below));
                let whole = seg < 1e-12 && got.0.xor(got.1).is_some_and(|g| (g - v).abs() < 1e-9);
                assert!(
                    (close(got.0, want.0) && close(got.1, want.1)) || whole,
                    "{label}: {got:?}, want {want:?}"
                );
            }
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
    let u = sweep::test_support::finished("the u", u, Tol::witness());
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

/// The cone grazes that may refuse, by (frustum, azimuth): at the
/// default ε they stop before rule (b) reads the wall, and at a wider
/// band they answer
/// (`work/cleave/a-convex-graze-of-a-cone-refuses-at-some-azimuths.md`).
const CONE_GRAZES_REFUSED: [(&str, f64); 3] =
    [("narrowing", 0.3), ("widening", 1.1), ("widening", 2.9)];

/// A cone frustum grazed from outside along a ruling, narrowing and
/// widening upward (the two nappes of the stored cone), at every
/// azimuth of [`THETAS`] (the revolve's seam is at +x): the wall bends
/// into its material, so the whole frustum, `7π/12`, lands on the
/// material side — save the refusals of [`CONE_GRAZES_REFUSED`].
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
        for t in THETAS {
            let refused = CONE_GRAZES_REFUSED.contains(&(name, t));
            for s in [1.0, -1.0] {
                let label = format!("{name}, θ = {t}, s = {s}");
                match split(
                    &sweep::test_support::finished("the operand", body.clone(), Tol::witness()),
                    &cone_tangent(r0, slope, unit(t), s),
                    Tol::witness(),
                ) {
                    Ok(r) => {
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
                    Err(e) => assert!(refused, "{label}: {e:?}"),
                }
            }
        }
    }
}

/// A plane tangent to the r = 0.5 wall of a revolved step, the graze
/// sharing its rim vertex with the step's flat, which the plane crosses
/// (y = 0.5, normal radial at azimuth θ). Material lies on the boss
/// side, `beyond` past the plane.
fn step_tangent(t: f64, s: f64) -> SplitPlane<f64> {
    let u = unit(t);
    topo::test_support::split_plane(
        Point3::new(0.5 * u.0, 0.5, 0.5 * u.1),
        Vec3::new(s * u.0, 0.0, s * u.1),
        Tol::witness(),
    )
}

/// The circular segment of a disc of radius `r` beyond a chord at `d`.
fn segment(r: f64, d: f64) -> f64 {
    r * r * (d / r).acos() - d * (r * r - d * d).sqrt()
}

/// A boss (r = 0.5, y ∈ [1, 2]) on a disc (r = 2, y ∈ [0, 1]), the
/// plane tangent to the boss's wall at every azimuth of [`THETAS`]: the
/// boss lands whole on the disc's side, and the disc's far segment
/// beyond the plane is the one piece across it.
#[test]
fn a_convex_graze_of_a_boss_on_a_step_cuts_only_the_step() {
    use std::f64::consts::PI;
    let boss = revolved(&[
        (0.0, 0.0),
        (2.0, 0.0),
        (2.0, 1.0),
        (0.5, 1.0),
        (0.5, 2.0),
        (0.0, 2.0),
    ]);
    let boss = sweep::test_support::finished("the boss", boss, Tol::witness());
    let beyond = segment(2.0, 0.5);
    let rest = 4.0 * PI + PI / 4.0 - beyond;
    for t in THETAS {
        for s in [1.0, -1.0] {
            let label = format!("θ = {t}, s = {s}");
            let r = split(&boss, &step_tangent(t, s), Tol::witness())
                .unwrap_or_else(|e| panic!("{label}: {e:?}"));
            let want = if s > 0.0 {
                (beyond, rest)
            } else {
                (rest, beyond)
            };
            let got = (volume(&label, &r.above), volume(&label, &r.below));
            assert!(
                (got.0.unwrap() - want.0).abs() <= 1e-7 * want.0.max(1.0)
                    && (got.1.unwrap() - want.1).abs() <= 1e-7 * want.1.max(1.0),
                "{label}: {got:?}, want {want:?}"
            );
        }
    }
}

/// The 6 × 4 rectangle on `[0, 6] × [0, 4]` with its corners rounded
/// r = 0.5 through the fillet door (declared tangent joints: smooth
/// edges between each flat and its corner wall). Its NE corner wall is
/// centred at (5.5, 3.5).
fn rounded_outline() -> profile::ProfileLoop<f64> {
    use profile::{Open, Start};
    let (w, h, r, t) = (6.0, 4.0, 0.5, Tol::witness());
    Open.at(Point2::new(w / 2.0, 0.0))
        .toward(1.0, 0.0, t)
        .unwrap()
        .fillet(r, t)
        .unwrap()
        .at(Point2::new(w, h / 2.0), t)
        .unwrap()
        .toward(0.0, 1.0, t)
        .unwrap()
        .fillet(r, t)
        .unwrap()
        .at(Point2::new(w / 2.0, h), t)
        .unwrap()
        .toward(-1.0, 0.0, t)
        .unwrap()
        .fillet(r, t)
        .unwrap()
        .at(Point2::new(0.0, h / 2.0), t)
        .unwrap()
        .toward(0.0, -1.0, t)
        .unwrap()
        .fillet(r, t)
        .unwrap()
        .to(Start, t)
        .unwrap()
        .into()
}

/// A 6 × 4 slab whose corners are rounded r = 0.5 through the fillet
/// door (declared tangent joints, smooth edges between each flat and
/// its corner wall), grazed along its NE corner wall at angle φ and
/// coplanar with the flats the corner continues (φ = 0, π/2): the slab
/// lands whole on the material side. φ = 1.2 may refuse before rule (b)
/// (`split_conic_departure` at the default ε, filed with the cone's
/// refusals).
#[test]
fn a_convex_graze_of_a_filleted_corner_lands_the_slab_whole() {
    let (w, h, r, t) = (6.0, 4.0, 0.5, Tol::witness());
    let body = extruded_loops(vec![rounded_outline()]);
    let body = sweep::test_support::finished("the body", body, t);
    let v = w * h - (4.0 - std::f64::consts::PI) * r * r;
    let c = (w - r, h - r);
    for phi in [
        0.0f64,
        0.3,
        std::f64::consts::FRAC_PI_4,
        1.2,
        std::f64::consts::FRAC_PI_2,
    ] {
        let n = unit(phi);
        for s in [1.0, -1.0] {
            let label = format!("φ = {phi}, s = {s}");
            let p = plane((c.0 + r * n.0, c.1 + r * n.1), (s * n.0, s * n.1));
            match split(&body, &p, t) {
                Ok(res) => {
                    let want = if s > 0.0 {
                        (None, Some(v))
                    } else {
                        (Some(v), None)
                    };
                    let got = (volume(&label, &res.above), volume(&label, &res.below));
                    assert!(
                        close(got.0, want.0) && close(got.1, want.1),
                        "{label}: {got:?}, want {want:?}"
                    );
                }
                Err(e) => assert!(phi == 1.2, "{label}: {e:?}"),
            }
        }
    }
}

/// Revolved holes grazed from inside at every azimuth of [`THETAS`],
/// with either normal, refuse ([`concave_graze_refuses`]): a conical
/// socket (a cylinder of radius 3 with the narrowing frustum's cone as
/// a through hole, the plane tangent to the hole's wall along a
/// ruling), and a counterbore (a bore of r = 0.5 under one of r = 1 in
/// a disc of r = 2, the plane tangent to the narrow bore's wall,
/// crossing the counterbore's floor).
#[test]
fn a_concave_graze_of_a_revolved_hole_refuses() {
    let socket = revolved(&[(1.0, 0.0), (3.0, 0.0), (3.0, 1.0), (0.5, 1.0)]);
    let bore = revolved(&[
        (0.5, 0.0),
        (2.0, 0.0),
        (2.0, 2.0),
        (1.0, 2.0),
        (1.0, 1.0),
        (0.5, 1.0),
    ]);
    for t in THETAS {
        for s in [1.0, -1.0] {
            let label = format!("socket, θ = {t}, s = {s}");
            let p = cone_tangent(1.0, -0.5, unit(t), s);
            concave_graze_refuses(&label, &socket, &p, SurfaceKind::Cone);
            let label = format!("counterbore, θ = {t}, s = {s}");
            concave_graze_refuses(&label, &bore, &step_tangent(t, s), SurfaceKind::Cylinder);
        }
    }
}

/// [`rounded_outline`] as a hole in a 10 × 8 plate, grazed from inside
/// along its NE corner wall at the convex slab's angles. At the
/// interior angles rule (b) reads the graze and refuses it
/// ([`concave_graze_refuses`]). Coplanar with a flat the corner
/// continues (φ = 0, π/2), the flat's sector is rule (a)'s, and the
/// knife edge is found on the section's boundary instead, at the smooth
/// edge between the flat and the corner wall: `SectionCusp`, naming the
/// wall, under either normal. φ = 1.2 refuses before rule (b) reads the
/// wall, as the slab's φ = 1.2 does
/// (`work/cleave/a-convex-graze-of-a-cone-refuses-at-some-azimuths.md`).
#[test]
fn a_concave_graze_of_a_filleted_hole_refuses() {
    use profile::RawLoop;
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_4};
    let plate = profile::ProfileLoop::polygon([
        Point2::new(-2.0, -2.0),
        Point2::new(8.0, -2.0),
        Point2::new(8.0, 6.0),
        Point2::new(-2.0, 6.0),
    ]);
    let body = extruded_loops(vec![plate, rounded_outline()]);
    let body = sweep::test_support::finished("the plate", body, Tol::witness());
    let (r, c) = (0.5, (5.5, 3.5));
    for phi in [0.0f64, 0.3, FRAC_PI_4, 1.2, FRAC_PI_2] {
        let n = unit(phi);
        for s in [1.0, -1.0] {
            let label = format!("φ = {phi}, s = {s}");
            let p = plane((c.0 + r * n.0, c.1 + r * n.1), (s * n.0, s * n.1));
            if phi == 0.0 || phi == FRAC_PI_2 {
                match split(&body, &p, Tol::witness()) {
                    Err(SplitError::Finish(SplitFinishError::SectionCusp { face, .. })) => {
                        let named = body
                            .get_face(face)
                            .and_then(|f| body.get_surface(f.surface));
                        assert_eq!(
                            named.map(geom::Surface::kind),
                            Some(SurfaceKind::Cylinder),
                            "{label}: the refusal names the corner wall"
                        );
                    }
                    other => panic!("{label}: expected SectionCusp, got {other:?}"),
                }
            } else if phi == 1.2 {
                assert!(
                    split(&body, &p, Tol::witness()).is_err(),
                    "{label}: answered"
                );
            } else {
                concave_graze_refuses(&label, &body, &p, SurfaceKind::Cylinder);
            }
        }
    }
}
