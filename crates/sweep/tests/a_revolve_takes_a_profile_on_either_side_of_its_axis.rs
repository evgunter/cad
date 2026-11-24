//! **A revolve takes a profile on either side of its axis.** The axis
//! line and the signed angle fix the turn: a profile to the left of the
//! directed axis is the same revolve as its image across the axis line
//! turned half a turn, so it builds where that image builds, and lands
//! where the authored turn carries it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common;

use core::f64::consts::{FRAC_PI_2, PI, TAU};
use geom_core::{Arc2, Band, Point2, Point3, Tol, Vec2, Vec3};
use profile::{ProfileLoop, RawLoop, test_support::bulge_loop};
use revolve_common::*;
use sweep::{CavityOrigin, Revolution, RevolveAxis, RevolveError, revolve};

/// The full circle of radius `r` about `(cx, cy)`, from carrier angle
/// `phase`.
fn circle(cx: f64, cy: f64, r: f64, phase: f64) -> ProfileLoop<f64> {
    RawLoop::new([(
        Point2::new(cx + r * phase.cos(), cy + r * phase.sin()),
        profile::Segment::Arc(Arc2 {
            centre: Point2::new(cx, cy),
            radius: r,
            sweep: TAU,
        }),
    )])
}

/// The axis through `(x, 0)` heading `±y`.
fn axis(x: f64, up: bool) -> RevolveAxis<f64> {
    RevolveAxis {
        origin: Point2::new(x, 0.0),
        dir: Vec2::new(0.0, if up { 1.0 } else { -1.0 }),
    }
}

/// `(x, y)` of the sketch plane (world XY) turned by `phi` about the
/// axis through `(ax, 0)` heading `±y`, right-handed.
fn turned(x: f64, y: f64, ax: f64, up: bool, phi: f64) -> Point3<f64> {
    let h = if up { 1.0 } else { -1.0 };
    let w = x - ax;
    Point3::new(ax + w * phi.cos(), y, -h * w * phi.sin())
}

fn containment(body: &topo::Body<f64>, q: Point3<f64>) -> topo::SolidContainment {
    let tol = Tol::witness();
    topo::point_in_solid(body, q, Band::linear(tol).unwrap(), tol).unwrap()
}

fn volume(body: &topo::AtRestBody<f64>, row: &str) -> topo::MassProperties<f64> {
    let tol = Tol::witness();
    topo::mass_properties(body, tol, topo::test_support::pinned_resolution(tol))
        .unwrap_or_else(|e| panic!("{row}: {e:?}"))
        .enclosure
}

/// **The pipe's end loops revolved into an elbow**, the PROFILE probe's
/// geometry on a free profile: the ring `0.6 ≤ ρ ≤ 1` about the origin
/// turned a quarter about an axis 2 clear of it, through `(±3, 0)`, for
/// both headings and both signs of the angle. Every row builds, with
/// the ring's material a quarter-turn's half along the authored turn
/// and none the other way, and the bore empty.
#[test]
fn a_ring_beside_its_axis_revolves_for_either_heading() {
    for ax in [3.0, -3.0] {
        for up in [true, false] {
            for theta in [FRAC_PI_2, -FRAC_PI_2] {
                let row = format!(
                    "axis through ({ax}, 0) heading {}y, angle {theta}",
                    if up { "+" } else { "-" }
                );
                let vp = validated(vec![circle(0.0, 0.0, 1.0, 0.0), circle(0.0, 0.0, 0.6, 0.0)]);
                let built = revolve(
                    &vp,
                    axis(ax, up),
                    Revolution::Partial(theta),
                    Tol::witness(),
                )
                .unwrap_or_else(|e| panic!("{row}: {e}"));
                let body = &built.body;
                assert_all_tiers(body);
                for (x, phi, want, what) in [
                    (
                        0.8,
                        theta / 2.0,
                        topo::SolidContainment::In,
                        "the ring along the turn",
                    ),
                    (
                        0.8,
                        -theta / 2.0,
                        topo::SolidContainment::Out,
                        "the ring against the turn",
                    ),
                    (
                        -0.8,
                        theta / 2.0,
                        topo::SolidContainment::In,
                        "the ring's far side along the turn",
                    ),
                    (
                        0.0,
                        theta / 2.0,
                        topo::SolidContainment::Out,
                        "the bore along the turn",
                    ),
                ] {
                    assert_eq!(
                        containment(body, turned(x, 0.0, ax, up, phi)),
                        want,
                        "{row}: {what}"
                    );
                }
            }
        }
    }
}

/// The half turn about the sketch `y` axis: what carries a profile to
/// its mirror across that axis.
fn half_turn(p: Point3<f64>) -> Point3<f64> {
    Point3::new(-p.x, p.y, -p.z)
}

fn half_turn_vec(v: Vec3<f64>) -> Vec3<f64> {
    Vec3::new(-v.x, v.y, -v.z)
}

fn near(a: Point3<f64>, b: Point3<f64>) -> bool {
    (a - b).norm() <= 1e-9
}

fn near_vec(a: Vec3<f64>, b: Vec3<f64>) -> bool {
    (a - b).norm() <= 1e-9
}

/// What a face reads out: its surface kind, its chart frame's origin and
/// axis, and its outward normal's chart sign.
#[derive(Debug)]
struct FaceRead {
    kind: geom::SurfaceKind,
    origin: Point3<f64>,
    axis: Vec3<f64>,
    sense: bool,
    /// The surface's own size: a radius, a minor radius, a half angle.
    size: f64,
}

fn faces(body: &topo::Body<f64>) -> Vec<FaceRead> {
    body.faces()
        .map(|(key, face)| {
            let surface = body.face_surface(key).unwrap();
            let frame = surface.frame().unwrap();
            FaceRead {
                kind: surface.kind(),
                origin: frame.origin(),
                axis: frame.axis(),
                sense: face.sense,
                size: match *surface {
                    geom::Surface::Cylinder { radius, .. }
                    | geom::Surface::Sphere { radius, .. } => radius,
                    geom::Surface::Torus { minor_radius, .. }
                    | geom::Surface::Spindle { minor_radius, .. } => minor_radius,
                    geom::Surface::Cone { half_angle, .. } => half_angle,
                    _ => 0.0,
                },
            }
        })
        .collect()
}

/// **The left body is the right body turned half a turn**, face for
/// face: every face of `right`, turned, has its own face of `left` of the
/// same kind and size, at the turned origin where `anchored`, whose
/// outward normal is the turned one; a revolution wall's chart axis is
/// the authored heading on both, and its sense the same.
fn assert_turned_faces(
    row: &str,
    right: &topo::Body<f64>,
    left: &topo::Body<f64>,
    heading: Vec3<f64>,
    anchored: bool,
) {
    let (rf, lf) = (faces(right), faces(left));
    assert_eq!(rf.len(), lf.len(), "{row}: face counts");
    // Each left face answers one right face at most: a bijection.
    let mut taken = vec![false; lf.len()];
    for r in &rf {
        let (origin, outward) = (
            half_turn(r.origin),
            half_turn_vec(if r.sense { r.axis } else { -r.axis }),
        );
        let found = (0..lf.len()).find(|&i| {
            let l = &lf[i];
            !taken[i]
                && l.kind == r.kind
                && (l.size - r.size).abs() <= 1e-12
                && (!anchored || near(l.origin, origin))
                && near_vec(if l.sense { l.axis } else { -l.axis }, outward)
                && (l.kind == geom::SurfaceKind::Plane
                    || (near_vec(l.axis, r.axis) && l.sense == r.sense))
        });
        if let Some(i) = found {
            taken[i] = true;
        }
        assert!(
            found.is_some(),
            "{row}: no face of the left body is {r:?} turned half a turn; left: {lf:?}"
        );
        if r.kind != geom::SurfaceKind::Plane {
            assert!(
                near_vec(r.axis, heading),
                "{row}: a right wall's chart axis {:?}",
                r.axis
            );
        }
    }
    for l in &lf {
        if l.kind != geom::SurfaceKind::Plane {
            assert!(
                near_vec(l.axis, heading),
                "{row}: a left wall's chart axis {:?}",
                l.axis
            );
        }
    }
}

/// Each profile beside the sketch `y` axis, mirrored across it: the
/// mirror turns by the same signed angle about the same directed axis,
/// so it is the first body turned half a turn about that axis. Both
/// build or both refuse alike; where they build, face for face the left
/// body is the right one turned (kind, origin, outward normal, and each
/// wall's chart axis the authored heading with the same sense), a point
/// of the right body's material turned is in the left body's, and the
/// certified volumes agree. For both headings and both signs of a
/// partial angle, and a full turn.
///
/// - A washer, clear of the axis.
/// - A half-disc on the axis (its vertices on the axis, its side the
///   arc's alone).
/// - A circle touching the axis at its one vertex, which pinches there
///   however far it turns and refuses on both sides.
/// - An apple's long arc between its two apexes, closed along the axis.
#[test]
fn a_profile_and_its_mirror_across_the_axis_revolve_alike() {
    /// A profile drawn on the side `s = ±1` of the axis, and a point of
    /// its material (none where the containment door reads no spindle
    /// wall; the faces then carry the placement).
    type Sided = (fn(f64) -> ProfileLoop<f64>, Option<Point2<f64>>);
    let rows: [(&str, Sided); 4] = [
        (
            "a washer",
            (
                |s| {
                    ProfileLoop::polygon([
                        Point2::new(s, 0.0),
                        Point2::new(2.0 * s, 0.0),
                        Point2::new(2.0 * s, 1.0),
                        Point2::new(s, 1.0),
                    ])
                },
                Some(Point2::new(1.5, 0.5)),
            ),
        ),
        (
            "a half-disc on the axis",
            (
                |s| {
                    bulge_loop(vec![
                        (Point2::new(0.0, -s), 1.0),
                        (Point2::new(0.0, s), 0.0),
                    ])
                },
                Some(Point2::new(0.5, 0.2)),
            ),
        ),
        (
            "a circle touching the axis",
            (
                |s| {
                    RawLoop::new([(
                        Point2::new(0.0, 0.0),
                        profile::Segment::Arc(Arc2 {
                            centre: Point2::new(s, 0.0),
                            radius: 1.0,
                            sweep: TAU,
                        }),
                    )])
                },
                Some(Point2::new(1.0, 0.0)),
            ),
        ),
        (
            "an apple's long arc between its apexes",
            (
                |s| {
                    let h = 0.75_f64.sqrt();
                    bulge_loop(vec![
                        (Point2::new(0.0, -s * h), (PI / 3.0).tan()),
                        (Point2::new(0.0, s * h), 0.0),
                    ])
                },
                None,
            ),
        ),
    ];
    for (what, (lp, inside)) in rows {
        for up in [true, false] {
            let heading = Vec3::new(0.0, if up { 1.0 } else { -1.0 }, 0.0);
            for revolution in [
                Revolution::Partial(1.0),
                Revolution::Partial(-1.0),
                Revolution::Full,
            ] {
                let build = |s: f64| {
                    revolve(
                        &validated(vec![lp(s)]),
                        axis(0.0, up),
                        revolution,
                        Tol::witness(),
                    )
                };
                let (right, left) = (build(1.0), build(-1.0));
                let row = format!(
                    "{what}, heading {}y, {revolution:?}",
                    if up { "+" } else { "-" }
                );
                match (&right, &left) {
                    (Ok(r), Ok(l)) => {
                        assert_all_tiers(&l.body);
                        // A full revolve of a left profile builds its turn
                        // about the reversed axis, so where a wall's chart
                        // is anchored along the axis follows that build
                        // (filed: work/lathe/a-left-profiles-full-revolve-
                        // keeps-the-reversed-builds-anchors-and-rims.md).
                        let anchored = !matches!(revolution, Revolution::Full);
                        assert_turned_faces(&row, &r.body, &l.body, heading, anchored);
                        if let Some(inside) = inside {
                            let phi = match revolution {
                                Revolution::Partial(t) => t / 2.0,
                                Revolution::Full => 1.0,
                            };
                            let q = turned(inside.x, inside.y, 0.0, up, phi);
                            assert_eq!(
                                containment(&r.body, q),
                                topo::SolidContainment::In,
                                "{row}: right material"
                            );
                            assert_eq!(
                                containment(&l.body, half_turn(q)),
                                topo::SolidContainment::In,
                                "{row}: left material"
                            );
                            if let Revolution::Partial(t) = revolution {
                                let q = turned(inside.x, inside.y, 0.0, up, -t / 2.0);
                                assert_eq!(
                                    containment(&r.body, q),
                                    topo::SolidContainment::Out,
                                    "{row}: right void"
                                );
                                assert_eq!(
                                    containment(&l.body, half_turn(q)),
                                    topo::SolidContainment::Out,
                                    "{row}: left void"
                                );
                            }
                        }
                        let (vr, vl) = (volume(&r.body, &row), volume(&l.body, &row));
                        assert!(
                            (vr.volume - vl.volume).abs()
                                <= 1e-9 * vr.volume + vr.volume_pad + vl.volume_pad,
                            "{row}: volumes {vr:?} right, {vl:?} left"
                        );
                    }
                    (Err(r), Err(l)) => assert_eq!(
                        core::mem::discriminant::<RevolveError>(r),
                        core::mem::discriminant::<RevolveError>(l),
                        "{row}: {r} right, {l} left"
                    ),
                    _ => panic!(
                        "{row}: right {:?}, left {:?}",
                        right.as_ref().map(|_| "built"),
                        left.as_ref().map(|_| "built")
                    ),
                }
            }
        }
    }
}

/// **Gaps rank along the authored axis, whichever side the profile
/// is on.** A full revolve of a box beside the axis that meets it in
/// three runs, notched off it between them: the notch at `y ∈ [1, 2]`
/// (to `|x| = 0.5`) and the one at `y ∈ [3, 4]` (to `|x| = 1`). Gap 0 is
/// the lower notch heading `+y` and the upper one heading `−y`, on
/// either side, read off each cavity's walls.
#[test]
fn gaps_rank_along_the_authored_axis_on_either_side() {
    let notched = |s: f64| {
        ProfileLoop::polygon(
            [
                (0.0, 0.0),
                (2.0, 0.0),
                (2.0, 5.0),
                (0.0, 5.0),
                (0.0, 4.0),
                (1.0, 3.5),
                (0.0, 3.0),
                (0.0, 2.0),
                (0.5, 1.5),
                (0.0, 1.0),
            ]
            .map(|(x, y)| Point2::new(s * x, y)),
        )
    };
    for s in [1.0, -1.0] {
        for up in [true, false] {
            let row = format!("side {s}, heading {}y", if up { "+" } else { "-" });
            let built = revolve(
                &validated(vec![notched(s)]),
                axis(0.0, up),
                Revolution::Full,
                Tol::witness(),
            )
            .unwrap_or_else(|e| panic!("{row}: {e}"));
            let height = |gap: usize| {
                let cavity = built
                    .cavities
                    .iter()
                    .find(|c| c.origin == CavityOrigin::Pocket { gap })
                    .unwrap_or_else(|| panic!("{row}: no gap {gap}"));
                let shell = built.body.get_shell(cavity.shell).unwrap();
                let ys: Vec<f64> = shell
                    .faces
                    .iter()
                    .filter_map(|&f| built.body.face_surface(f).unwrap().frame())
                    .map(|frame| frame.origin().y)
                    .collect();
                ys.iter().sum::<f64>() / ys.len() as f64
            };
            let (g0, g1) = (height(0), height(1));
            let (lower, upper) = ((1.0..=2.0).contains(&g0), (3.0..=4.0).contains(&g1));
            let (upper0, lower1) = ((3.0..=4.0).contains(&g0), (1.0..=2.0).contains(&g1));
            assert!(
                if up { lower && upper } else { upper0 && lower1 },
                "{row}: gap 0 at y {g0}, gap 1 at y {g1}"
            );
        }
    }
}

/// **What still refuses crosses the axis**, and the side read is the
/// profile's whole reach: a box reaching both sides refuses for either
/// heading, says it crosses, and names a vertex on the side away from
/// its widest reach. Balanced (the side reads zero), or a sliver off it
/// (in band), the authored frame is kept and names a vertex to the
/// authored axis's left.
#[test]
fn a_profile_across_its_axis_refuses_for_either_heading() {
    let d = 3.0 * eps();
    // (row, the box's x span, whether the named vertex lies at x > 0
    // heading +y and heading −y)
    for (what, (lo, hi), named_right) in [
        ("widest to the left", (-3.0, 1.0), [true, true]),
        ("widest to the right", (-1.0, 3.0), [false, false]),
        ("balanced", (-1.0, 1.0), [false, true]),
        ("a sliver off balanced", (-1.0 - d, 1.0), [false, true]),
    ] {
        let pts = [
            Point2::new(lo, 0.0),
            Point2::new(hi, 0.0),
            Point2::new(hi, 1.0),
            Point2::new(lo, 1.0),
        ];
        for (up, right) in [(true, named_right[0]), (false, named_right[1])] {
            let row = format!("{what}, heading {}y", if up { "+" } else { "-" });
            let e = revolve(
                &validated(vec![ProfileLoop::polygon(pts)]),
                axis(0.0, up),
                Revolution::Partial(1.0),
                Tol::witness(),
            )
            .unwrap_err();
            let RevolveError::VertexCrossesAxis {
                loop_index: 0,
                vertex_index,
            } = e
            else {
                panic!("{row}: {e:?}")
            };
            assert!(e.to_string().contains("crosses the axis"), "{row}: {e}");
            let x = pts[vertex_index].x;
            assert_eq!(x > 0.0, right, "{row}: vertex {vertex_index} at x {x}");
        }
    }
}
