//! **The plane × cone join lane**, run on whole poses below the operand
//! gate: the germ-pair dispatch's plane × cone arms and the planar side's
//! chord, behind the plane × cone germ frame.
//!
//! The boolean still refuses a cone operand at its pair gate
//! (`reach_cone_root_lane.rs` pins that), so these rows reach the join
//! through `topo::join_admitting_cones` (`sweep-testing`), which runs
//! the production pipeline with the cone on the gate's roster and stops
//! after the join. The interior-loop guard decides every cone pair
//! `Intractable` until its section certificate has cone rows, so each
//! pose's verdict there is the guard's refusal, and the rows say so. The
//! join's output is pinned by the chords it minted in both operands:
//! every new conic edge is the closed-form section, carried by the
//! cutting plane and by the cone (the operand's own face on its side,
//! an equal aux copy on the planar side).
//!
//! The poses are the cone sector spec's: B4, the slab `y ∈ [0.3, 0.6]`
//! across the widening frustum; C1, a half-space brick whose face is
//! tilted 10° off axis-normal; T1, TANG's box turned −50° against the
//! π/6 cone, whose two ellipses bound a ring on the cone face.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common::{axis_y, validated};
use geom::{Curve3, Surface};
use geom_brep::EdgeDescription;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{ProfileLoop, RawLoop};
use sweep::test_support::{brick, finished};
use sweep::{Revolution, revolve};
use topo::{AtRestBody, Body, BooleanError, BooleanOp, ConeJoin, PairRefusalSite};

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
/// join leaves them, after holding the interior-loop guard's verdict to
/// its cone refusal.
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
        matches!(
            interior_loops,
            Err(BooleanError::CurvedPairUnsupported {
                site: PairRefusalSite::InteriorLoopGuard,
                kind: geom::SurfaceKind::Cone,
                ..
            })
        ),
        "{label}: the interior-loop guard refuses the cone pair until its certificate \
         has cone rows, got {interior_loops:?}"
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

/// **T1's ring: the cone face's island winds by the segment's curve.**
/// TANG's box edge pierces the π/6 cone's wall twice, and the box's two
/// faces at it cut the cone in two ellipses whose arcs bound a lune: a
/// ring on the cone face, which the join's ring lane closes by the
/// section's own curve (`RingClosure::Wall` on a cone). In every op and
/// member order the join connects, and every chord the cone and a box
/// face carry is that face's ellipse, on both sides.
#[test]
fn the_rings_island_on_the_cone_face_winds_by_the_sections_curve() {
    let c = crate::a_ring_on_a_cone_face::cone(Affine3::identity());
    let x = crate::a_ring_on_a_cone_face::wedge(0.6, Affine3::identity());
    let cone = cone_of(&c);
    // The box faces at the edge, as the box carries them.
    let cutting: Vec<(Point3<f64>, Vec3<f64>)> =
        conic_chords(&joined("T1, probe", &c, &x, OPS[0]).1)
            .iter()
            .filter_map(|ch| cut_by(ch, &cone))
            .collect();
    assert!(
        !cutting.is_empty(),
        "T1: the box's chords name its cutting faces"
    );
    for op in OPS {
        let label = format!("T1, {op:?}");
        let (cone_side, box_side) = joined(&label, &c, &x, op);
        for &(q, n) in &cutting {
            assert_section_chords(
                &format!("{label}, the cone's side"),
                &cone_side,
                &cone,
                q,
                n,
            );
            assert_section_chords(&format!("{label}, the box's side"), &box_side, &cone, q, n);
        }
    }
}
