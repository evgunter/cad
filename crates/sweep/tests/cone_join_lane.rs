//! **The plane × cone join lane**, run on whole poses below the operand
//! gate: the germ-pair dispatch's plane × cone arms and the planar side's
//! chord, behind the plane × cone germ frame.
//!
//! The boolean still refuses a cone operand at its pair gate
//! (`reach_cone_root_lane.rs` pins that), so these rows reach the join
//! through `topo::join_admitting_cones` (`sweep-testing`), which runs
//! the production pipeline with the cone on the gate's roster and stops
//! after the join. The section certificate's cone rows place every
//! plane × cone section of these poses across an edge of an operand, so
//! the interior-loop guard admits each pose in every op, and the rows
//! say so. The join's output is pinned by the chords it minted in both
//! operands: every new conic edge is the closed-form section, carried by
//! the cutting plane and by the cone (the operand's own face on its
//! side, an equal aux copy on the planar side).
//!
//! The poses are the cone sector spec's: B4, the slab `y ∈ [0.3, 0.6]`
//! across the widening frustum; C1, a half-space brick whose face is
//! tilted 10° off axis-normal; T1, TANG's box turned −50° against the
//! π/6 cone, whose two ellipses bound a ring on the cone face. C2, B1
//! and C3 cut a hyperbola or a parabola, which the frame refuses by
//! decision.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common::{axis_y, validated};
use geom::{Curve3, Surface};
use geom_brep::EdgeDescription;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{ProfileLoop, RawLoop};
use sweep::test_support::{brick, finished};
use sweep::{Revolution, revolve};
use topo::{AtRestBody, Body, BooleanError, BooleanOp, ConeJoin};

/// The widening frustum: radius `0.5 → 1` over `y ∈ [0, 1]`, so apex
/// `(0, −1, 0)`, axis `+y`, `tan α = 1/2`.
fn widening() -> AtRestBody<f64> {
    let lp = ProfileLoop::polygon(
        [(0.0, 0.0), (0.5, 0.0), (1.0, 1.0), (0.0, 1.0)].map(|(x, y)| Point2::new(x, y)),
    );
    let body = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body;
    finished("the frustum", body, Tol::witness())
}

fn posed(what: &str, raw: Body<f64>, pose: Affine3<f64>) -> AtRestBody<f64> {
    let tol = Tol::witness();
    finished(what, topo::transform_rigid(&raw, &pose, tol).unwrap(), tol)
}

/// C1's cutting plane: through `(0, 0.5, 0)`, its normal `+y` turned
/// 10° about `z`.
fn c1_plane() -> (Point3<f64>, Vec3<f64>) {
    let t = 10f64.to_radians();
    (
        Point3::new(0.0, 0.5, 0.0),
        Vec3::new(-t.sin(), t.cos(), 0.0),
    )
}

/// C1's brick: 6 m below [`c1_plane`], its top face on it.
fn c1_brick() -> AtRestBody<f64> {
    let tol = Tol::witness();
    let raw = brick((-3.0, 3.0), (-6.0, 0.0), (-3.0, 3.0), tol);
    let turn = Affine3::rotation_about_axis(
        Point3::origin(),
        Vec3::new(0.0, 0.0, 1.0),
        10f64.to_radians(),
    );
    posed(
        "C1's brick",
        raw,
        Affine3::translation(c1_plane().0 - Point3::origin()) * turn,
    )
}

/// Every op in both member orders: `(op, cone first)`.
const OPS: [(BooleanOp, bool); 6] = [
    (BooleanOp::Union, true),
    (BooleanOp::Union, false),
    (BooleanOp::Intersect, true),
    (BooleanOp::Intersect, false),
    (BooleanOp::Subtract, true),
    (BooleanOp::Subtract, false),
];

/// The pose joined under `op`: the cone's operand and the other's as the
/// join leaves them, after holding the interior-loop guard to admitting
/// the pose.
fn joined(
    label: &str,
    cone: &Body<f64>,
    other: &Body<f64>,
    (op, cone_first): (BooleanOp, bool),
) -> (Body<f64>, Body<f64>) {
    let (a, b) = if cone_first {
        (cone, other)
    } else {
        (other, cone)
    };
    let ConeJoin {
        a,
        b,
        interior_loops,
    } = topo::join_admitting_cones(op, a, b, Tol::witness())
        .unwrap_or_else(|e| panic!("{label}: the join refused {e:?}"))
        .unwrap_or_else(|| panic!("{label}: answered without a join"));
    assert!(
        interior_loops.is_ok(),
        "{label}: the section certificate's cone rows place every section across an \
         edge, so the interior-loop guard admits the pose, got {interior_loops:?}"
    );
    if cone_first { (a, b) } else { (b, a) }
}

/// The cone carrier of `body`'s one cone face.
fn cone_of(body: &Body<f64>) -> Surface<f64> {
    body.faces()
        .filter_map(|(_, f)| body.get_surface(f.surface))
        .find(|s| matches!(s, Surface::Cone { .. }))
        .expect("a cone face")
        .clone()
}

fn same_cone(s: &Surface<f64>, cone: &Surface<f64>) -> bool {
    match (s, cone) {
        (
            Surface::Cone {
                apex,
                axis,
                half_angle,
                ..
            },
            Surface::Cone {
                apex: a2,
                axis: x2,
                half_angle: h2,
                ..
            },
        ) => {
            (*apex - *a2).norm() < 1e-12
                && (*axis - *x2).norm() < 1e-12
                && (half_angle - h2).abs() < 1e-12
        }
        _ => false,
    }
}

/// A conic chord: its carrier and the two surfaces its description
/// names.
struct Chord {
    carrier: Curve3<f64>,
    surfaces: [Surface<f64>; 2],
}

/// Every circle or ellipse edge of `body` described as the intersection
/// of two of its surfaces.
fn conic_chords(body: &Body<f64>) -> Vec<Chord> {
    body.edges()
        .filter_map(|(_, e)| {
            let c = body.get_curve_geom(e.curve)?.certified()?;
            if !matches!(c.carrier(), Curve3::Circle { .. } | Curve3::Ellipse { .. }) {
                return None;
            }
            let EdgeDescription::Intersection { s1, s2, .. } = c.description() else {
                return None;
            };
            Some(Chord {
                carrier: c.carrier().clone(),
                surfaces: [*s1, *s2].map(|s| body.get_surface(s).expect("a surface").clone()),
            })
        })
        .collect()
}

/// The chord's cutting plane, where one of its surfaces is a plane and
/// the other is `cone`.
fn cut_by(chord: &Chord, cone: &Surface<f64>) -> Option<(Point3<f64>, Vec3<f64>)> {
    match &chord.surfaces {
        [Surface::Plane { origin, normal, .. }, other]
        | [other, Surface::Plane { origin, normal, .. }]
            if same_cone(other, cone) =>
        {
            Some((*origin, *normal))
        }
        _ => None,
    }
}

/// The closed-form section of `cone` by the plane `(q, n)` off its apex
/// (the Dandelin construction): centre, normal, semi-major, semi-minor.
fn section(
    cone: &Surface<f64>,
    q: Point3<f64>,
    n: Vec3<f64>,
) -> (Point3<f64>, Vec3<f64>, f64, f64) {
    let &Surface::Cone {
        apex,
        axis,
        half_angle,
        ..
    } = cone
    else {
        panic!("a cone")
    };
    let n = n.normalize();
    let (s, c_a) = half_angle.sin_cos();
    let c = axis.dot(n);
    let delta = (apex - q).dot(n);
    let k = c * c - s * s;
    let centre = apex - (axis * c - n * (s * s)) * (delta / k);
    (
        centre,
        n,
        delta.abs() * s * c_a / k,
        delta.abs() * s / k.sqrt(),
    )
}

/// `chord`'s carrier is the conic `(centre, normal, a, b)`.
fn is_conic(chord: &Chord, (centre, normal, a, b): (Point3<f64>, Vec3<f64>, f64, f64)) -> bool {
    let (c, axis, ra, rb) = match chord.carrier {
        Curve3::Circle {
            center,
            axis,
            radius,
            ..
        } => (center, axis, radius, radius),
        Curve3::Ellipse {
            center,
            axis,
            major,
            minor,
            ..
        } => (center, axis, major, minor),
        _ => return false,
    };
    (c - centre).norm() < 1e-12
        && axis.normalize().cross(normal).norm() < 1e-12
        && (ra - a).abs() < 1e-12
        && (rb - b).abs() < 1e-12
}

/// The chords on `side` cut from `cone` by the plane `(q, n)`: at least
/// one, and every one of them the closed-form section, carried by that
/// plane and the cone.
fn assert_section_chords(
    label: &str,
    side: &Body<f64>,
    cone: &Surface<f64>,
    q: Point3<f64>,
    n: Vec3<f64>,
) {
    let n = n.normalize();
    let want = section(cone, q, n);
    let chords = conic_chords(side);
    let on_plane: Vec<&Chord> = chords
        .iter()
        .filter(|ch| {
            cut_by(ch, cone).is_some_and(|(o, m)| {
                m.normalize().cross(n).norm() < 1e-12 && (o - q).dot(n).abs() < 1e-12
            })
        })
        .collect();
    assert!(
        !on_plane.is_empty(),
        "{label}: no chord of the cone cut by the plane through {q:?} normal {n:?}"
    );
    for ch in on_plane {
        assert!(
            is_conic(ch, want),
            "{label}: the chord {:?} is the closed-form section {want:?}",
            ch.carrier
        );
    }
}

/// **B4's slab: both sides' chords are the circles `y = 0.3` and
/// `y = 0.6`.** In every op and member order the join connects, and on
/// the frustum's operand and the slab's alike, every chord the cone and
/// a slab face carry is that face's circle about the axis, of radius
/// `0.5 + 0.5·y`. On the slab's side the cone is an aux copy equal to the
/// frustum's carrier.
#[test]
fn the_slabs_chords_are_its_faces_circles_on_both_sides() {
    let tol = Tol::witness();
    let frustum = widening();
    let slab = finished(
        "B4's slab",
        brick((-2.0, 2.0), (0.3, 0.6), (-2.0, 2.0), tol),
        tol,
    );
    let cone = cone_of(&frustum);
    for op in OPS {
        let label = format!("B4, {op:?}");
        let (cone_side, slab_side) = joined(&label, &frustum, &slab, op);
        for (side, body) in [
            ("the frustum's side", &cone_side),
            ("the slab's side", &slab_side),
        ] {
            for y in [0.3, 0.6] {
                assert_section_chords(
                    &format!("{label}, {side}, y = {y}"),
                    body,
                    &cone,
                    Point3::new(0.0, y, 0.0),
                    Vec3::new(0.0, 1.0, 0.0),
                );
            }
        }
    }
}

/// **C1's tilted plane: both sides' chords are the ellipse the matched
/// germs name.** In every op and both member orders, every chord the
/// cone and the brick's tilted face carry is the Dandelin ellipse of
/// that plane, on the frustum's operand and on the brick's.
#[test]
fn the_tilted_planes_chords_are_its_ellipse_in_both_member_orders() {
    let frustum = widening();
    let c1 = c1_brick();
    let cone = cone_of(&frustum);
    let (q, n) = c1_plane();
    for op in OPS {
        let label = format!("C1, {op:?}");
        let (cone_side, brick_side) = joined(&label, &frustum, &c1, op);
        assert_section_chords(
            &format!("{label}, the frustum's side"),
            &cone_side,
            &cone,
            q,
            n,
        );
        assert_section_chords(
            &format!("{label}, the brick's side"),
            &brick_side,
            &cone,
            q,
            n,
        );
    }
}

/// T1's two cutting planes, closed form: the box's faces at its edge
/// through `s·(0.5, 1.5)` from the apex (`s = 0.6`), the faces `x = 0`
/// and `y = 0` of the brick turned −50° about `z`.
fn t1_planes() -> [(Point3<f64>, Vec3<f64>); 2] {
    let h = 1.2 / std::f64::consts::FRAC_PI_6.tan();
    let edge = Point3::new(0.5 * 0.6, h - 1.5 * 0.6, 0.0);
    let t = 50f64.to_radians();
    [
        (edge, Vec3::new(t.cos(), -t.sin(), 0.0)),
        (edge, Vec3::new(t.sin(), t.cos(), 0.0)),
    ]
}

/// Whether `chord` is cut from `cone` by the plane `(q, n)`.
fn cut_by_plane(chord: &Chord, cone: &Surface<f64>, (q, n): (Point3<f64>, Vec3<f64>)) -> bool {
    cut_by(chord, cone)
        .is_some_and(|(o, m)| m.normalize().cross(n).norm() < 1e-12 && (o - q).dot(n).abs() < 1e-12)
}

/// The point of an ellipse carrier at its angle parameter `t`, and the
/// parameter of a point on it.
fn ellipse_at(e: &Curve3<f64>, t: f64) -> Point3<f64> {
    let Curve3::Ellipse {
        center,
        axis,
        major,
        minor,
        u_ref,
    } = e
    else {
        panic!("an ellipse")
    };
    *center + *u_ref * (major * t.cos()) + axis.cross(*u_ref) * (minor * t.sin())
}

fn ellipse_param(e: &Curve3<f64>, p: Point3<f64>) -> f64 {
    let Curve3::Ellipse {
        center,
        axis,
        major,
        minor,
        u_ref,
    } = e
    else {
        panic!("an ellipse")
    };
    let w = p - *center;
    (w.dot(axis.cross(*u_ref)) / minor).atan2(w.dot(*u_ref) / major)
}

/// `cone`'s outward normal at (the foot on it of) `p`, for a solid on
/// the axis side of the wall: off the axis and against it by the
/// half-angle.
fn cone_outward(cone: &Surface<f64>, p: Point3<f64>) -> Vec3<f64> {
    let &Surface::Cone {
        apex,
        axis,
        half_angle,
        ..
    } = cone
    else {
        panic!("a cone")
    };
    let d = p - apex;
    let radial = (d - axis * d.dot(axis)).normalize();
    radial * half_angle.cos() - axis * half_angle.sin()
}

/// Where a lune loop stands on its face ([`lune_loops`]).
#[derive(Debug, PartialEq)]
enum LuneRole {
    /// The face's only loop.
    Island,
    /// A ring of a face whose outer loop is not a lune.
    Ring,
    /// A loop of a face every loop of which is a lune: the join's
    /// scaffolding between two copies of the lune.
    Scaffolding,
}

/// Every loop of a face on `cone` in `body` whose edges are all ellipse
/// chords cut by one of `planes`: where it stands on its face, and how
/// it winds about the cone's outward normal (the signed area of its
/// vertices and arc midpoints, in loop order, along the normal at their
/// centroid: positive is counter-clockwise).
fn lune_loops(
    body: &Body<f64>,
    cone: &Surface<f64>,
    planes: &[(Point3<f64>, Vec3<f64>)],
) -> Vec<(LuneRole, f64)> {
    let points: std::collections::HashMap<_, _> = body.vertex_points().collect();
    let mut found = Vec::new();
    for (_, face) in body.faces() {
        if !body
            .get_surface(face.surface)
            .is_some_and(|s| same_cone(s, cone))
        {
            continue;
        }
        let mut on_face = Vec::new();
        for &lp in std::iter::once(&face.outer).chain(&face.rings) {
            let topo::LoopBoundary::Cycle { first } = body.get_loop(lp).unwrap().boundary else {
                on_face.push((lp, None));
                continue;
            };
            let mut polygon = Vec::new();
            let mut all_lune = true;
            for he in body.loop_cycle(first).unwrap() {
                let half = body.get_half_edge(he).unwrap();
                let edge = body.get_edge(half.edge).unwrap();
                let geom = body.get_curve_geom(edge.curve).and_then(|c| c.certified());
                let Some(c) = geom else {
                    all_lune = false;
                    break;
                };
                let carrier = c.carrier();
                let EdgeDescription::Intersection { s1, s2, .. } = c.description() else {
                    all_lune = false;
                    break;
                };
                let chord = Chord {
                    carrier: carrier.clone(),
                    surfaces: [*s1, *s2].map(|s| body.get_surface(s).unwrap().clone()),
                };
                if !matches!(carrier, Curve3::Ellipse { .. })
                    || !planes.iter().any(|&pl| cut_by_plane(&chord, cone, pl))
                {
                    all_lune = false;
                    break;
                }
                let start = |h: topo::HalfEdgeKey| points[&body.get_half_edge(h).unwrap().start];
                let (t0, t1) = (
                    ellipse_param(carrier, start(edge.he_plus)),
                    ellipse_param(carrier, start(edge.he_minus)),
                );
                let sweep = (t1 - t0).rem_euclid(std::f64::consts::TAU);
                polygon.push(points[&half.start]);
                polygon.push(ellipse_at(carrier, t0 + sweep / 2.0));
            }
            if !all_lune {
                on_face.push((lp, None));
                continue;
            }
            let n = polygon.len() as f64;
            let centroid = polygon
                .iter()
                .fold(Point3::origin(), |acc, p| acc + (*p - Point3::origin()) / n);
            let area = (0..polygon.len()).fold(Vec3::new(0.0, 0.0, 0.0), |acc, i| {
                let (a, b) = (polygon[i], polygon[(i + 1) % polygon.len()]);
                acc + (a - centroid).cross(b - centroid)
            });
            on_face.push((lp, Some(area.dot(cone_outward(cone, centroid)))));
        }
        let every = on_face.iter().all(|(_, w)| w.is_some());
        for (lp, winding) in on_face {
            let Some(winding) = winding else { continue };
            let role = if every && face.rings.is_empty() {
                LuneRole::Island
            } else if every {
                LuneRole::Scaffolding
            } else {
                assert_ne!(
                    lp, face.outer,
                    "a lune outer loop on a face with other loops"
                );
                LuneRole::Ring
            };
            found.push((role, winding));
        }
    }
    found
}

/// **T1's ring: the cone face's island winds by the segment's curve.**
/// TANG's box edge pierces the π/6 cone's wall twice, and the box's two
/// faces at it ([`t1_planes`], closed form) cut the cone in two ellipses
/// whose arcs bound a lune: a ring on the cone face, which the join's
/// ring lane closes by the section's own curve (`RingClosure::Wall` on a
/// cone). In every op and member order the join connects, every chord
/// the cone and either face carry is that face's Dandelin ellipse, on
/// both sides, and each face carries one. On the cone's side the lune is
/// two loops: the island face's only loop, counter-clockwise about the
/// cone's outward normal, and the ring the remainder holds, clockwise.
#[test]
fn the_rings_island_on_the_cone_face_winds_by_the_sections_curve() {
    let c = crate::a_ring_on_a_cone_face::cone(Affine3::identity());
    let x = crate::a_ring_on_a_cone_face::wedge(0.6, Affine3::identity());
    let cone = cone_of(&c);
    let planes = t1_planes();
    for op in OPS {
        let label = format!("T1, {op:?}");
        let (cone_side, box_side) = joined(&label, &c, &x, op);
        for (side, body) in [
            ("the cone's side", &cone_side),
            ("the box's side", &box_side),
        ] {
            for (i, &(q, n)) in planes.iter().enumerate() {
                let label = format!("{label}, {side}, face {i}");
                assert_section_chords(&label, body, &cone, q, n);
                for ch in conic_chords(body)
                    .iter()
                    .filter(|ch| cut_by_plane(ch, &cone, (q, n)))
                {
                    assert!(
                        matches!(ch.carrier, Curve3::Ellipse { .. }),
                        "{label}: the chord is an ellipse, got {:?}",
                        ch.carrier
                    );
                }
            }
        }
        let lune = lune_loops(&cone_side, &cone, &planes);
        let winding = |role| {
            let of: Vec<f64> = lune.iter().filter(|l| l.0 == role).map(|l| l.1).collect();
            assert_eq!(of.len(), 1, "{label}: one {role:?} lune loop, got {lune:?}");
            of[0]
        };
        assert!(
            winding(LuneRole::Island) > 0.0 && winding(LuneRole::Ring) < 0.0,
            "{label}: the island's only loop winds counter-clockwise about the cone's outward \
             normal and the ring its remainder holds clockwise, got {lune:?}"
        );
    }
}

/// **A parabola or a hyperbola refuses by decision, end to end.** C2's
/// half-space brick, whose face `x = 0.3` runs along the axis, and B1,
/// the brick `[0.6, 2] × [0.3, 0.8] × [−0.2, 0.2]` across the widening
/// wall, whose faces `x = 0.6` and `z = ±0.2` do too, each cut the
/// frustum in hyperbolas; C3's, whose face through `(0.5, 0.5, 0)` lies
/// parallel to a generator, in a parabola. In every op and member order
/// the pipeline refuses at the germ pair's frame with the conic named,
/// the plane×cone pair in the operands' order: neither the missing-arm
/// refusal nor a desync.
#[test]
fn a_parabola_or_a_hyperbola_refuses_by_decision_in_every_op() {
    let tol = Tol::witness();
    let frustum = widening();
    let alpha = 0.5_f64.atan();
    let c3_turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 0.0, 1.0), -alpha);
    let poses = [
        (
            "C2",
            finished(
                "C2's brick",
                brick((0.3, 6.3), (-3.0, 3.0), (-3.0, 3.0), tol),
                tol,
            ),
            geom_brep::OutsideConic::Hyperbola,
        ),
        (
            "B1",
            finished("B1", brick((0.6, 2.0), (0.3, 0.8), (-0.2, 0.2), tol), tol),
            geom_brep::OutsideConic::Hyperbola,
        ),
        (
            "C3",
            posed(
                "C3's brick",
                brick((0.0, 6.0), (-3.0, 3.0), (-3.0, 3.0), tol),
                Affine3::translation(Vec3::new(0.5, 0.5, 0.0)) * c3_turn,
            ),
            geom_brep::OutsideConic::Parabola,
        ),
    ];
    for (pose, other, conic) in &poses {
        for (op, cone_first) in OPS {
            let label = format!("{pose}, {op:?}, cone first: {cone_first}");
            let (a, b, kinds) = if cone_first {
                (
                    &frustum,
                    other,
                    [geom::SurfaceKind::Cone, geom::SurfaceKind::Plane],
                )
            } else {
                (
                    other,
                    &frustum,
                    [geom::SurfaceKind::Plane, geom::SurfaceKind::Cone],
                )
            };
            let got = topo::join_admitting_cones(op, a, b, tol);
            assert!(
                matches!(
                    &got,
                    Err(BooleanError::GermSectionOutsideInventory {
                        a_kind,
                        b_kind,
                        conic: named,
                        ..
                    }) if [*a_kind, *b_kind] == kinds && named == conic
                ),
                "{label}: the {conic:?} refuses by decision at the germ pair's frame, got {:?}",
                got.map(|j| j.is_some())
            );
        }
    }
}
