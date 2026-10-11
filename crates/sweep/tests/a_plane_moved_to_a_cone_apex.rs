//! **A plane face moved to within the band of a cone's apex still cuts
//! the cone.** A frustum cut across by a plane leaning nearly along a
//! generator has an elliptical rim shared by its cone wall and the
//! plane face. Offsetting that face toward the apex moves the plane
//! parallel to itself, and parallel planes cut the cone in ellipses
//! homothetic about the apex: the moved plane, its apex gap Zero, still
//! cuts an ellipse whose farthest point is `|δ|/|D|` from the apex (`δ`
//! the gap, `D` the plane-through-apex discriminant), past the band
//! when `D` is small. That ellipse is the rim's section; the apex point
//! is not, and the door must not read it as a plane that no longer
//! meets its neighbour.
//!
//! A plane through the apex holding two generators, moved into the band
//! of it, cuts a hyperbola instead, whose branches start `|δ|/D` from
//! the apex. On a needle that is past the band, and the door must not
//! keep the generator through the apex as the section there.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::solid_truth::{self, Op, Solid, Want};
use crate::revolve_common::{axis_y, validated};
use core::f64::consts::{FRAC_PI_2, FRAC_PI_6, PI};
use geom::Surface;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{ProfileLoop, RawLoop};
use sweep::test_support::{brick, finished};
use sweep::{Revolution, revolve};
use topo::{AtRestBody, Body, BooleanError, FaceKey, SplitJoinError};

fn p(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

/// The cone's half-angle.
const ALPHA: f64 = core::f64::consts::FRAC_PI_3;
/// The cut plane's discriminant `sin α·‖a×n‖ − cos α·|a·n|`: negative
/// (the plane through the apex clears the cone) and small, so the
/// ellipse is long against the plane's gap.
const DISCR: f64 = -1.0 / 256.0;
/// The cut plane's gap from the apex.
const GAP: f64 = 1.0 / 512.0;

/// A frustum of the cone with apex at the origin and axis `+y`, from
/// `y = 1/2048` to `y = 3/8`, cut by the plane `GAP` above the apex
/// whose normal leans off the axis toward `+x` until its discriminant
/// is `DISCR`; the part away from the apex. Its rim on the cone wall is
/// the ellipse the plane cuts (reach `GAP/|DISCR|` along a generator,
/// inside the frustum's slant window).
fn cut_frustum() -> Body<f64> {
    let (y0, y1, t) = (1.0 / 2048.0, 3.0 / 8.0, ALPHA.tan());
    let meridian = ProfileLoop::polygon([
        Point2::new(0.0, y0),
        Point2::new(y0 * t, y0),
        Point2::new(y1 * t, y1),
        Point2::new(0.0, y1),
    ]);
    let frustum = revolve(
        &validated(vec![meridian]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .expect("the meridian revolves")
    .body;
    let frustum = topo::test_support::finished("the frustum", frustum, Tol::witness());
    let lean = (-DISCR).acos() - ALPHA;
    let n = Vec3::new(lean.sin(), lean.cos(), 0.0);
    let plane = topo::test_support::split_plane(Point3::origin() + n * GAP, n, Tol::witness());
    let split = topo::splitting::split(&frustum, &plane, Tol::witness()).expect("the cut splits");
    let topo::splitting::SplitPart::Body(above) = split.above else {
        panic!("the part away from the apex carries material");
    };
    above
}

fn surface_of(body: &Body<f64>, face: FaceKey) -> Surface<f64> {
    body.get_surface(body.get_face(face).unwrap().surface)
        .unwrap()
        .clone()
}

/// **Every rim arc of the moved face plans its section.** The cut face
/// is offset toward the apex by its gap less `0.9·ε`, so the moved
/// plane's apex gap is in the zero band while the ellipse it cuts
/// reaches `0.9·ε/|D|` (`≈ 230·ε`) from the apex. Each rim arc between
/// the face and the cone wall is planned (`Ok(None)`): a deferred
/// `NoBranch` refusal is the apex point read as the section.
#[test]
fn a_plane_moved_to_the_apex_band_plans_the_ellipse_it_cuts() {
    let tol = Tol::witness();
    let (eps, k) = (tol.eps(), tol.k());
    let body = cut_frustum();
    let face = body
        .faces()
        .map(|(f, _)| f)
        .find(|&f| {
            matches!(surface_of(&body, f), Surface::Plane { normal, .. } if normal.x.abs() > 0.1)
        })
        .expect("the cut face");
    let Surface::Plane { origin, normal, .. } = surface_of(&body, face) else {
        unreachable!("the cut face is a plane")
    };
    let Some(Surface::Cone {
        apex,
        axis,
        half_angle,
        ..
    }) = body
        .faces()
        .map(|(f, _)| surface_of(&body, f))
        .find(|s| matches!(s, Surface::Cone { .. }))
    else {
        panic!("the frustum's wall is a cone");
    };

    // The face's outward normal faces the apex, so a positive offset
    // closes the gap.
    let gap = (apex - origin).dot(normal);
    assert!(
        (gap - GAP).abs() < 1e-12,
        "the cut face stands {gap} off the apex"
    );
    let d = gap - 0.9 * eps;
    let moved_gap = (apex - (origin + normal * d)).dot(normal);
    let discr =
        half_angle.sin() * axis.cross(normal).norm() - half_angle.cos() * axis.dot(normal).abs();
    assert!(
        moved_gap.abs() <= eps,
        "the moved plane's apex gap {moved_gap:e} is in the zero band"
    );
    assert!(
        moved_gap.abs() / discr.abs() > k * eps,
        "the moved plane's ellipse reaches {:e}, past the band",
        moved_gap.abs() / discr.abs()
    );

    let plans = topo::offset_edge_plans_for_tests(&body, face, d, tol);
    assert!(!plans.is_empty(), "the cut face has rim edges");
    for (edge, plan) in plans {
        let carrier = body
            .get_curve_geom(body.get_edge(edge).unwrap().curve)
            .and_then(topo::CurveGeom::certified)
            .unwrap()
            .carrier()
            .clone();
        assert!(
            matches!(carrier, geom::Curve3::Ellipse { .. }),
            "{edge:?}: the rim is the cut's ellipse, got {carrier:?}"
        );
        assert!(
            matches!(plan, Ok(None)),
            "{edge:?}: the moved plane's ellipse is the rim's section, got {plan:?}"
        );
    }
}

/// A quarter wedge of the cone of half-angle `atan(t)` with its apex at
/// the origin and axis `+y`, from the apex to `y = 3/8`: its two caps are
/// planes containing the axis, each meeting the wall along a generator
/// from the apex vertex.
fn apex_wedge(t: f64) -> Body<f64> {
    let meridian = ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(0.375 * t, 0.375),
        Point2::new(0.0, 0.375),
    ]);
    revolve(
        &validated(vec![meridian]),
        axis_y(),
        Revolution::Partial(FRAC_PI_2),
        Tol::witness(),
    )
    .expect("the meridian revolves")
    .body
}

/// The caps of a wedge about `+y`: its plane faces whose normals lie
/// across the axis.
fn axial_caps(body: &Body<f64>) -> Vec<FaceKey> {
    body.faces()
        .map(|(f, _)| f)
        .filter(|&f| {
            matches!(surface_of(body, f), Surface::Plane { normal, .. } if normal.y.abs() < 0.5)
        })
        .collect()
}

/// The single cone surface of `body`.
fn cone_of(body: &Body<f64>) -> Surface<f64> {
    body.faces()
        .map(|(f, _)| surface_of(body, f))
        .find(|s| matches!(s, Surface::Cone { .. }))
        .expect("the body has a cone wall")
}

/// Where the section of the plane `(origin, normal)` starts on each
/// nappe of `cone`, read off the surfaces: the real plane meets the line
/// through a unit generator `g` at `−δ/(g·n)` from the apex (`δ` the
/// apex gap), and `g·n` runs over `[−D, M]` (mirrored where `a·n < 0`),
/// so one nappe's section starts `|δ|/M` from the apex and the other's
/// `|δ|/D`. Returns `(on the +axis nappe, on the −axis nappe, |δ|/D)`.
fn nappe_starts(cone: &Surface<f64>, origin: Point3<f64>, normal: Vec3<f64>) -> (f64, f64, f64) {
    let Surface::Cone {
        apex,
        axis,
        half_angle,
        ..
    } = *cone
    else {
        unreachable!("a cone")
    };
    let (sin_a, cos_a) = half_angle.sin_cos();
    let (c, s) = (axis.dot(normal), axis.cross(normal).norm());
    let (d, m) = (sin_a * s - cos_a * c.abs(), sin_a * s + cos_a * c.abs());
    let gap = (apex - origin).dot(normal);
    // On the +axis nappe `t > 0`, so `g·n` has the sign of `−δ`: the
    // steep end `M` lies on the side of `c`.
    let (plus, minus) = if -gap * c > 0.0 { (m, d) } else { (d, m) };
    (gap.abs() / plus, gap.abs() / minus, gap.abs() / d)
}

/// What the plane×cone arm serves for a pair whose far branch starts
/// `start` from the apex: the pair within the zero band, an escalation
/// in it, the hyperbola past it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Start {
    Served,
    InBand,
    Past,
}

fn start_class(start: f64) -> Start {
    let tol = Tol::witness();
    if start <= tol.eps() {
        Start::Served
    } else if start <= tol.k() * tol.eps() {
        Start::InBand
    } else {
        Start::Past
    }
}

/// **A cap moved off a needle's apex does not keep the generator its
/// plane no longer holds.** A cap of [`apex_wedge`]`(1/64)` moved by `±0.9·ε`
/// stands in the zero band of the apex, and its plane cuts the wall in
/// a hyperbola whose branches start `0.9·ε/D` (`D = sin α ≈ 1/64`, so
/// `≈ 58·ε`) from the apex, past the band. The cap's wall edge runs from
/// the apex vertex, so the line pair through the apex is not its
/// section there: the edge refuses typed, naming the hyperbola (R1),
/// where the door had planned the old generator. The cap's other edges,
/// against planes, plan.
#[test]
fn a_cap_moved_off_a_needle_apex_refuses_the_hyperbola() {
    let tol = Tol::witness();
    let (eps, k) = (tol.eps(), tol.k());
    let body = apex_wedge(1.0 / 64.0);
    let cone = cone_of(&body);
    let caps = axial_caps(&body);
    assert_eq!(caps.len(), 2, "the wedge has two caps");
    for face in caps {
        let Surface::Plane { origin, normal, .. } = surface_of(&body, face) else {
            unreachable!("a cap is a plane")
        };
        for d in [0.9 * eps, -0.9 * eps] {
            let moved = origin + normal * d;
            let Surface::Cone { apex, .. } = cone else {
                unreachable!("a cone")
            };
            let moved_gap = (apex - moved).dot(normal);
            let (.., far) = nappe_starts(&cone, moved, normal);
            assert!(
                moved_gap.abs() <= eps && far > k * eps,
                "{face:?} by {d:e}: the moved gap {moved_gap:e} is in the zero band and the \
                 hyperbola starts {far:e} off the apex, past it"
            );
            let mut walls = 0;
            for (edge, plan) in topo::offset_edge_plans_for_tests(&body, face, d, tol) {
                match plan {
                    Err(topo::ReplaceFaceError::NeighborPoseUnroutable {
                        other_kind: geom::SurfaceKind::Cone,
                        why,
                        ..
                    }) => {
                        assert!(
                            why.contains("HYPERBOLA"),
                            "{face:?} by {d:e}, {edge:?}: refuses naming the hyperbola, got {why}"
                        );
                        walls += 1;
                    }
                    Ok(None) => {}
                    other => panic!("{face:?} by {d:e}, {edge:?}: got {other:?}"),
                }
            }
            assert_eq!(
                walls, 1,
                "{face:?} by {d:e}: the cap's one wall edge refuses"
            );
        }
    }
}

/// **A cap moved into the band of a wedge's apex escalates, where the
/// door had returned a body that fails tier 3.** A cap of
/// [`apex_wedge`]`(1/4)` moved by `−0.3·ε` cuts the wall in a hyperbola
/// whose branches start `0.3·ε/sin α ≈ 1.24·ε` from the apex vertex,
/// inside the band: the offset escalates naming `pn_apex_pair_reach`
/// and leaves the body untouched. The door used to serve the generator
/// pair there and return a body whose volume tier 3 cannot compute.
#[test]
fn a_cap_moved_into_the_band_of_a_wedge_apex_escalates() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let body = apex_wedge(0.25);
    let cone = cone_of(&body);
    let caps = axial_caps(&body);
    assert_eq!(caps.len(), 2, "the wedge has two caps");
    for face in caps {
        let Surface::Plane { origin, normal, .. } = surface_of(&body, face) else {
            unreachable!("a cap is a plane")
        };
        let d = -0.3 * eps;
        let (.., far) = nappe_starts(&cone, origin + normal * d, normal);
        assert_eq!(
            start_class(far),
            Start::InBand,
            "{face:?}: the hyperbola starts {far:e} off the apex"
        );
        let mut moved = body.clone();
        match topo::replace_face_offset(&mut moved, face, d, tol) {
            Err(topo::ReplaceFaceError::Escalated { source }) => assert_eq!(
                source.predicate,
                Some("pn_apex_pair_reach"),
                "{face:?}: escalates on the pair's reach"
            ),
            other => panic!("{face:?}: wanted the pair's escalation, got {other:?}"),
        }
        assert_eq!(
            moved.faces().count(),
            body.faces().count(),
            "{face:?}: the body is untouched on the refusal"
        );
    }
}

/// A quarter wedge of the frustum of half-angle `atan(t)`, apex at the
/// origin and axis `+y`, from `y = y0` to `y = 1`: its caps are planes
/// containing the axis, and its wall stays `y0` clear of the apex.
fn frustum_wedge(y0: f64, t: f64) -> (AtRestBody<f64>, Solid) {
    let meridian = ProfileLoop::polygon([
        Point2::new(0.0, y0),
        Point2::new(y0 * t, y0),
        Point2::new(t, 1.0),
        Point2::new(0.0, 1.0),
    ]);
    let tol = Tol::witness();
    let body = revolve(
        &validated(vec![meridian]),
        axis_y(),
        Revolution::Partial(FRAC_PI_2),
        tol,
    )
    .expect("the meridian revolves")
    .body;
    (
        finished("the frustum wedge", body, tol),
        Solid::Revolved {
            y0,
            y1: 1.0,
            r0: y0 * t,
            k: t,
            window: Some(FRAC_PI_2),
        },
    )
}

/// **A half-space flush with a frustum wedge's cap.** The brick
/// `[−2, 2] × [−1, 2] × [−δ, 2]` overlaps the `z = 0` cap of
/// [`frustum_wedge`] by `δ ∈ {0.5·ε, 0.9·ε}`, for `y0 ∈ {1/4, 1/512}`
/// and `tan α ∈ {1/64, 1/2, 1}`. Its face holds no generator: it meets
/// the cone in a hyperbola whose branches start `δ/sin α` from the apex
/// on both nappes (the plane is parallel to the axis, so `D = M`).
///
/// **The union is right at every pose here.** The wedge's wall stays
/// `y0` clear of the apex. Over that span the hyperbola lies within
/// `δ + δ²/(2·y0·tan α)` of the generator the pair serves, and both
/// terms are within ε. Main builds the union in both orders.
///
/// The arm reads where the section starts at the apex, not over the
/// span the join consumes:
/// - within ε, it serves the pair and the union builds;
/// - in the band, the join escalates naming `pn_apex_pair_reach`;
/// - past it, the join refuses the hyperbola.
///
/// Each refusal becomes the union (`V_wedge + V_brick − overlap`, the
/// `Want` this row checks the served poses against) once the arm reads
/// the consumed span
/// (`work/germ/pn-apex-line-reach-is-read-at-the-apex-not-over-the-consumed-span.md`).
#[test]
fn a_half_space_flush_with_a_frustum_wedge_cap_builds_only_where_its_start_is_served() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let mut reached = std::collections::BTreeSet::new();
    for y0 in [0.25, 1.0 / 512.0] {
        for t in [1.0 / 64.0, 0.5, 1.0] {
            for off in [0.5, 0.9] {
                let d = off * eps;
                let (wedge, sw) = frustum_wedge(y0, t);
                let bx = [(-2.0, 2.0), (-1.0, 2.0), (-d, 2.0)];
                let brick = finished("the half-space", brick(bx[0], bx[1], bx[2], tol), tol);
                let sb = Solid::Brick(bx);
                let label = format!("y0 = {y0}, tan α = {t}, {off}·ε over the cap");
                let face = brick
                    .faces()
                    .map(|(f, _)| surface_of(&brick, f))
                    .find(|s| matches!(s, Surface::Plane { origin, normal, .. } if normal.z.abs() > 0.5 && origin.z < 1.0))
                    .expect("the brick's face over the cap");
                let Surface::Plane { origin, normal, .. } = face else {
                    unreachable!("a plane")
                };
                let (plus, minus, far) = nappe_starts(&cone_of(&wedge), origin, normal);
                assert!(
                    (plus - minus).abs() <= 1e-12 * far,
                    "{label}: the face parallel to the axis starts the hyperbola as far on both \
                     nappes, {plus:e} and {minus:e}"
                );
                let class = start_class(far);
                reached.insert(format!("{class:?}"));
                let cap_area = t * (1.0 - y0 * y0) / 2.0;
                let v =
                    PI * t * t * (1.0 - y0.powi(3)) / 12.0 + 4.0 * 3.0 * (2.0 + d) - d * cap_area;
                let points = solid_truth::grid(p(-0.1, 0.0, -1.1), p(1.1, 1.1, 0.1), 6);
                let named = [p(0.3 * t, 0.6, -0.3 * t), p(0.0, 0.5, 1.0)];
                for (order, got) in [
                    ("wedge ∪ half-space", topo::union(&wedge, &brick, tol)),
                    ("half-space ∪ wedge", topo::union(&brick, &wedge, tol)),
                ] {
                    let label = format!("{label}, {order} (start {:.3}·ε)", far / eps);
                    match class {
                        // The flush slab is decided within the band, so
                        // the volume is good to ε over the cap.
                        Start::Served => {
                            solid_truth::assert_is(
                                &label,
                                &got,
                                Want::Body(v, 1e-9 * v + 2.0 * eps * cap_area),
                                &|q| Op::Union.depth(&sw, &sb, q),
                                &named,
                                &points,
                            );
                        }
                        Start::InBand => {
                            let Err(BooleanError::Join(SplitJoinError::Escalated { diag, .. })) =
                                &got
                            else {
                                panic!("{label}: wanted the pair's escalation, got {got:?}");
                            };
                            assert_eq!(diag.predicate, Some("pn_apex_pair_reach"), "{label}");
                        }
                        Start::Past => {
                            let Err(BooleanError::Join(SplitJoinError::Section {
                                source: geom_brep::SectionError::RoutesToGeneralRung { why, .. },
                                ..
                            })) = &got
                            else {
                                panic!("{label}: wanted the hyperbola refusal, got {got:?}");
                            };
                            assert!(why.contains("HYPERBOLA"), "{label}: {why}");
                        }
                    }
                }
            }
        }
    }
    assert_eq!(
        reached.len(),
        3,
        "the poses reach a served pair, the band and past it: {reached:?}"
    );
}

/// The frustum of the cone of half-angle π/6 about `+y` with its apex at
/// `(0, apex_y, 0)`, from `y = 1/4` to `y = 1`, and the box
/// `[0, 2] × [−2, 2]²` turned about the apex until its `x = 0` face is a
/// plane through the apex `1e-6` rad past the tangent plane along a
/// generator (`D = sin(1e-6)`), the pair posed together by `pose`.
fn frustum_and_virtual_apex_box(apex_y: f64, pose: &Affine3<f64>) -> [(AtRestBody<f64>, Solid); 2] {
    let tol = Tol::witness();
    let t = FRAC_PI_6.tan();
    let (y0, y1) = (0.25, 1.0);
    let meridian = ProfileLoop::polygon([
        Point2::new(0.0, y0),
        Point2::new((y0 - apex_y) * t, y0),
        Point2::new((y1 - apex_y) * t, y1),
        Point2::new(0.0, y1),
    ]);
    let mut frustum = revolve(&validated(vec![meridian]), axis_y(), Revolution::Full, tol)
        .expect("the meridian revolves")
        .body;
    frustum.merge_coplanar_faces(tol).unwrap();
    let frustum = topo::transform_rigid(&frustum, pose, tol).unwrap();
    let apex = Point3::new(0.0, apex_y, 0.0);
    let turn = *pose
        * Affine3::rotation_about_axis(apex, Vec3::new(0.0, 0.0, 1.0), 1e-6 - FRAC_PI_6)
        * Affine3::translation(apex - Point3::origin());
    let bx = [(0.0, 2.0), (-2.0, 2.0), (-2.0, 2.0)];
    let boxed = topo::transform_rigid(&brick(bx[0], bx[1], bx[2], tol), &turn, tol).unwrap();
    [
        (
            finished("the frustum", frustum, tol),
            Solid::Revolved {
                y0,
                y1,
                r0: (y0 - apex_y) * t,
                k: t,
                window: None,
            }
            .posed(*pose),
        ),
        (
            finished("the box", boxed, tol),
            Solid::Brick(bx).posed(turn),
        ),
    ]
}

/// **A box face built through a frustum's virtual apex.** The face is a
/// plane through the apex of [`frustum_and_virtual_apex_box`] with
/// `D = sin(1e-6)`, posed off the axes, so it stands only a rounding gap
/// `|δ| ≈ 7e-17` off the apex. The far branch starts `|δ|/D ≈ 7e-11`
/// from it:
/// - within ε at the default ε, where every op builds against the
///   closed-form truth;
/// - past the band at 1e-12, where the boolean frame refuses naming the
///   hyperbola.
///
/// At 1e-6, `D` itself lies in the band, and the row skips loudly.
///
/// **What main built.** Main built the right body for every op in both
/// orders at 1e-12 too. The frustum stays `≈ 0.55/cos α` clear of the
/// apex, and over that span the section is within `|δ|·sin α/√(D·M)`
/// (`≈ 4e-14`) of the pair.
///
/// The two apex heights put the frustum on the near nappe (start
/// `|δ|/M ≈ 1e-16`) and on the far one. Both refuse, so the refusal is
/// not mended by telling the arm which nappe the consumer's face lies
/// on. Telling it the span is what mends it
/// (`work/germ/pn-apex-line-reach-is-read-at-the-apex-not-over-the-consumed-span.md`).
#[test]
fn a_box_face_through_a_virtual_apex_builds_only_where_its_start_is_served() {
    let tol = Tol::witness();
    let pose = Affine3::translation(Vec3::new(0.3, 0.1, 0.2))
        * Affine3::rotation_about_axis(Point3::origin(), Vec3::new(1.0, 2.0, 3.0).normalize(), 0.3);
    let mut own = Vec::new();
    for apex_y in [-0.3, -0.17] {
        let [(frustum, sf), (boxed, sb)] = frustum_and_virtual_apex_box(apex_y, &pose);
        let cone = cone_of(&frustum);
        let Surface::Cone { apex, axis, .. } = cone else {
            unreachable!("a cone")
        };
        let (origin, normal) = boxed
            .faces()
            .find_map(|(f, _)| match surface_of(&boxed, f) {
                Surface::Plane { origin, normal, .. }
                    if (apex - origin).dot(normal).abs() < 1e-9 =>
                {
                    Some((origin, normal))
                }
                _ => None,
            })
            .expect("the box face through the apex");
        let gap = (apex - origin).dot(normal);
        assert!(
            gap != 0.0,
            "apex at {apex_y}: the face stands a rounding gap off the apex"
        );
        let (plus, minus, far) = nappe_starts(&cone, origin, normal);
        let (eps, k) = (tol.eps(), tol.k());
        if gap.abs() / far <= k * eps {
            println!(
                "SKIPPED at this ε: D = {:e} leaves the box face within the band of tangent to \
                 the cone, which the boolean decides as a coincidence before the apex lane",
                gap.abs() / far
            );
            return;
        }
        let inside = pose.transform_point(p(0.0, 0.6, 0.0));
        let start = if (inside - apex).dot(axis) > 0.0 {
            plus
        } else {
            minus
        };
        own.push((start, far));
        let class = start_class(far);
        let label = format!("apex at {apex_y}, start {far:e} (its own nappe {start:e})");
        // The sliver of the frustum past the box face: a circular
        // segment of depth `(y − apex_y)·(tan α − tan φ)` at height `y`,
        // `φ` the face's turn, integrated by midpoints.
        let (t, tp) = (FRAC_PI_6.tan(), (FRAC_PI_6 - 1e-6).tan());
        let n = 2000;
        let vi = (0..n)
            .map(|i| {
                let y = 0.25 + 0.75 * (i as f64 + 0.5) / n as f64;
                let (r, h) = ((y - apex_y) * t, (y - apex_y) * (t - tp));
                r * r * ((r - h) / r).acos() - (r - h) * (2.0 * r * h - h * h).sqrt()
            })
            .sum::<f64>()
            * 0.75
            / n as f64;
        let vf = PI * t * t * ((1.0 - apex_y).powi(3) - (0.25 - apex_y).powi(3)) / 3.0;
        let vb = 32.0;
        let points: Vec<_> = solid_truth::grid(p(-0.7, 0.2, -0.7), p(0.7, 1.05, 0.7), 7)
            .into_iter()
            .map(|q| pose.transform_point(q))
            .collect();
        let ops = [
            (
                "frustum ∪ box",
                topo::union(&frustum, &boxed, tol),
                Op::Union,
                &sf,
                &sb,
                vf + vb - vi,
            ),
            (
                "box ∪ frustum",
                topo::union(&boxed, &frustum, tol),
                Op::Union,
                &sf,
                &sb,
                vf + vb - vi,
            ),
            (
                "frustum ∩ box",
                topo::intersect(&frustum, &boxed, tol),
                Op::Intersect,
                &sf,
                &sb,
                vi,
            ),
            (
                "box ∩ frustum",
                topo::intersect(&boxed, &frustum, tol),
                Op::Intersect,
                &sf,
                &sb,
                vi,
            ),
            (
                "frustum ∖ box",
                topo::subtract(&frustum, &boxed, tol),
                Op::Subtract,
                &sf,
                &sb,
                vf - vi,
            ),
            (
                "box ∖ frustum",
                topo::subtract(&boxed, &frustum, tol),
                Op::Subtract,
                &sb,
                &sf,
                vb - vi,
            ),
        ];
        for (op, got, truth, a, b, v) in ops {
            let label = format!("{label}, {op}");
            match class {
                Start::Served => {
                    solid_truth::assert_is(
                        &label,
                        &got,
                        Want::Body(v, 1e-9 * v.max(1.0)),
                        &|q| truth.depth(a, b, q),
                        &[],
                        &points,
                    );
                }
                Start::InBand => {
                    let Err(BooleanError::Escalated { diag, .. }) = &got else {
                        panic!("{label}: wanted the pair's escalation, got {got:?}");
                    };
                    assert_eq!(diag.predicate, Some("pn_apex_pair_reach"), "{label}");
                }
                Start::Past => assert!(
                    matches!(
                        &got,
                        Err(BooleanError::GermSectionOutsideInventory {
                            conic: geom_brep::OutsideConic::Hyperbola,
                            ..
                        })
                    ),
                    "{label}: wanted the hyperbola refusal, got {got:?}"
                ),
            }
        }
    }
    assert!(
        own.iter().any(|&(s, far)| s < 1e-3 * far) && own.iter().any(|&(s, far)| s == far),
        "one pose puts the frustum on the near nappe and one on the far: {own:?}"
    );
}
