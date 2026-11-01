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
use test_utils::cone_cut::ConeCut;
use topo::{AtRestBody, Body, FaceKey};

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
    above.into_body()
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

/// The oracle's cut of `cone` by the plane `(origin, normal)`, in
/// double-double arithmetic off the stored `f64`s
/// ([`test_utils::cone_cut`]); it never reads the arm's bound.
fn cut(cone: &Surface<f64>, origin: Point3<f64>, normal: Vec3<f64>) -> ConeCut {
    let Surface::Cone {
        apex,
        axis,
        half_angle,
        ..
    } = *cone
    else {
        unreachable!("a cone")
    };
    let p3 = |q: Point3<f64>| [q.x, q.y, q.z];
    let w3 = |v: Vec3<f64>| [v.x, v.y, v.z];
    ConeCut::new(p3(apex), w3(axis), half_angle, (p3(origin), w3(normal)))
}

/// Where the section of the plane `(origin, normal)` starts on `cone`'s
/// `+axis` and `−axis` nappes, and the farther of the two, by the oracle.
fn nappe_starts(cone: &Surface<f64>, origin: Point3<f64>, normal: Vec3<f64>) -> (f64, f64, f64) {
    let cut = cut(cone, origin, normal);
    let (plus, minus) = (cut.start(1.0), cut.start(-1.0));
    (plus, minus, plus.max(minus))
}

/// How far the line pair through the apex parallel to the plane
/// `(origin, normal)` stands from its section with `cone` over the
/// points `(lo, hi)` along the axis from the apex, by the oracle: both
/// ways round, each section point from the nearer line and each line
/// point from the section.
fn pair_miss(
    cone: &Surface<f64>,
    (origin, normal): (Point3<f64>, Vec3<f64>),
    (lo, hi): (f64, f64),
) -> f64 {
    let Surface::Cone { half_angle, .. } = *cone else {
        unreachable!("a cone")
    };
    let cos = half_angle.cos();
    cut(cone, origin, normal).pair_miss((lo / cos, hi / cos))
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
/// section there: the edge refuses typed, naming the hyperbola,
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
                    Err(topo::ReplaceFaceError::EdgeSection {
                        other_kind: geom::SurfaceKind::Cone,
                        verdict:
                            topo::SectionVerdict::OpenConic {
                                conic: geom_brep::OpenConic::Hyperbola,
                            },
                        ..
                    }) => walls += 1,
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

/// **A cap moved off a frustum wedge's apex plans its wall edge.** A cap
/// of the quarter wedge of the `tan α = 1/64` frustum from `y = 1/256`
/// to `y = 3/8` moved by `±0.9·ε` cuts the wall in a hyperbola whose
/// branches start `≈ 58·ε` from the apex, past the band. The wall edge
/// the cap shares stays `1/256` clear of the apex, and over that span
/// the pair stands within ε of the hyperbola by the oracle
/// ([`pair_miss`]), so the
/// offset keeps the generator: every edge plans, and the offset door
/// moves the cap.
///
/// The truth is read off the circles the wall is made of: at height
/// `y` the moved plane, `d` off the axis, meets the wall's circle of
/// radius `r = y·tan α` at the angle `asin(d/r)` past the cap's
/// generator, so a point of the planned wall edge at that height
/// stands `|x − x'|` off the section point `x'` there, which this row
/// reads over the edge's span.
///
/// The moved body is right: its volume is `V + d·A` (`A` the cap's
/// area) within `ε·A/4`, it passes tier 3, and `point_in_solid` agrees
/// with the wedge's membership at every point of a grid over its upper
/// span: the wall is read from its boundary edges, whatever azimuths its
/// two rims span.
#[test]
fn a_cap_moved_off_a_frustum_wedge_apex_plans_its_wall_edge() {
    let tol = Tol::witness();
    let (eps, k) = (tol.eps(), tol.k());
    let (y0, y1, t) = (1.0 / 256.0, 0.375, 1.0 / 64.0);
    let body = frustum_wedge_body(y0, y1, t);
    let cone = cone_of(&body);
    let caps = axial_caps(&body);
    let v = PI * t * t * (y1.powi(3) - y0.powi(3)) / 12.0;
    let cap_area = t * (y1 * y1 - y0 * y0) / 2.0;
    // The moved cap stands `0.9·ε` off the wedge's, well inside the
    // truth's margin, so the wedge's own membership is the truth.
    let truth = Solid::Revolved {
        y0,
        y1,
        r0: y0 * t,
        k: t,
        window: Some(FRAC_PI_2),
    };
    let named = [p(0.0, 0.2, 0.5), p(0.5, 0.2, 0.0)];
    let points = solid_truth::grid(p(-0.002, 0.17, -0.008), p(0.008, 0.41, 0.002), 6);
    assert_eq!(caps.len(), 2, "the wedge has two caps");
    for face in caps {
        let Surface::Plane { origin, normal, .. } = surface_of(&body, face) else {
            unreachable!("a cap is a plane")
        };
        for d in [0.9 * eps, -0.9 * eps] {
            let moved = origin + normal * d;
            let label = format!("{face:?} by {d:e}");
            let (.., far) = nappe_starts(&cone, moved, normal);
            assert!(
                far > k * eps,
                "{label}: the hyperbola starts {far:e} off the apex"
            );
            let bound = pair_miss(&cone, (moved, normal), (y0, y1));
            assert_eq!(
                start_class(bound),
                Start::Served,
                "{label}: the pair stands {bound:e} off the section over the wall edge"
            );
            for (edge, plan) in topo::offset_edge_plans_for_tests(&body, face, d, tol) {
                assert!(matches!(plan, Ok(None)), "{label}, {edge:?}: got {plan:?}");
            }
            let mut walls = 0;
            for (edge, spec) in topo::offset_edge_specs_for_tests(&body, face, d, tol) {
                let spec = spec.unwrap_or_else(|e| panic!("{label}, {edge:?}: refused {e:?}"));
                // The cap's other edges are against planes: its wall
                // edge is a generator, a line at the half-angle off the
                // axis, read here at every height of the wall's span.
                let geom::Curve3::Line { origin: o, dir } = spec.carrier else {
                    continue;
                };
                if (dir.x.hypot(dir.z) - t * dir.y.abs()).abs() > 1e-6 * dir.norm() {
                    continue;
                }
                let at = |s: f64| {
                    let y = y0 + (y1 - y0) * s;
                    o + dir * ((y - o.y) / dir.y)
                };
                walls += 1;
                let miss = (0..=64)
                    .map(|i| at(f64::from(i) / 64.0))
                    .fold(0.0_f64, |miss, x| {
                        let off = Vec3::new(x.x, 0.0, x.z);
                        let r = x.y * t;
                        let along = off - normal * off.dot(normal);
                        let across = (d / r).clamp(-1.0, 1.0);
                        let truth = p(0.0, x.y, 0.0)
                            + along.normalize() * (r * (1.0 - across * across).sqrt())
                            + normal * (r * across);
                        miss.max((x - truth).norm())
                    });
                assert!(
                    miss <= eps,
                    "{label}, {edge:?}: the planned wall edge stands {:.3}·ε off the section",
                    miss / eps
                );
            }
            assert_eq!(walls, 1, "{label}: the cap has one wall edge");
            let mut offset = body.clone();
            topo::replace_face_offset(&mut offset, face, d, tol)
                .unwrap_or_else(|e| panic!("{label}: the offset refused {e:?}"));
            solid_truth::assert_body_is(
                &label,
                &offset,
                (v + d * cap_area, 1e-12 * v + 0.25 * eps * cap_area),
                &|q| truth.depth(q),
                &named,
                &points,
            );
        }
    }
}

/// A quarter wedge of the frustum of half-angle `atan(t)`, apex at the
/// origin and axis `+y`, from `y = y0` to `y = y1`: its caps are planes
/// containing the axis, and its wall stays `y0` clear of the apex.
fn frustum_wedge_body(y0: f64, y1: f64, t: f64) -> Body<f64> {
    let meridian = ProfileLoop::polygon([
        Point2::new(0.0, y0),
        Point2::new(y0 * t, y0),
        Point2::new(y1 * t, y1),
        Point2::new(0.0, y1),
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

/// [`frustum_wedge_body`] to `y = 1`, finished, with its truth.
fn frustum_wedge(y0: f64, t: f64) -> (AtRestBody<f64>, Solid) {
    let tol = Tol::witness();
    let body = frustum_wedge_body(y0, 1.0, t);
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

/// **A half-space flush with a frustum wedge's cap builds the union.**
/// The brick `[−2, 2] × [−1, 2] × [−δ, 2]` overlaps the `z = 0` cap of
/// [`frustum_wedge`] by `δ ∈ {0.5·ε, 0.9·ε}`, for `y0 ∈ {1/4, 1/512}`
/// and `tan α ∈ {1/64, 1/2, 1}`, in both orders. Its face holds no
/// generator: it meets the cone in a hyperbola whose branches start
/// `δ/sin α` from the apex on both nappes (the plane is parallel to the
/// axis, so `D = M`), past the band at `tan α = 1/64` and in it at
/// `1/2`.
///
/// The wedge's wall, and the section the join reads on it, stay `y0`
/// clear of the apex, on one nappe. Over that span the pair stands
/// about `δ·(1 + δ/(y0·sin α))` off the hyperbola, within ε at every
/// pose by the oracle ([`pair_miss`]), so the arm serves the pair
/// and the union builds: `V_wedge + V_brick − overlap`, against the
/// closed-form volume, tier 3 and `point_in_solid` on a grid. Read at
/// the apex alone, 20 of the 24 refuse.
#[test]
fn a_half_space_flush_with_a_frustum_wedge_cap_builds_the_union() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let mut past_the_apex_reading = 0;
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
                let cone = cone_of(&wedge);
                let (plus, minus, far) = nappe_starts(&cone, origin, normal);
                assert!(
                    (plus - minus).abs() <= 1e-12 * far,
                    "{label}: the face parallel to the axis starts the hyperbola as far on both \
                     nappes, {plus:e} and {minus:e}"
                );
                if start_class(far) != Start::Served {
                    past_the_apex_reading += 1;
                }
                let bound = pair_miss(&cone, (origin, normal), (y0, 1.0));
                assert_eq!(
                    start_class(bound),
                    Start::Served,
                    "{label}: the pair stands {:.3}·ε off the section over the wall",
                    bound / eps
                );
                let cap_area = t * (1.0 - y0 * y0) / 2.0;
                let v =
                    PI * t * t * (1.0 - y0.powi(3)) / 12.0 + 4.0 * 3.0 * (2.0 + d) - d * cap_area;
                let points = solid_truth::grid(p(-0.1, 0.0, -1.1), p(1.1, 1.1, 0.1), 6);
                let named = [p(0.3 * t, 0.6, -0.3 * t), p(0.0, 0.5, 1.0)];
                for (order, got) in [
                    ("wedge ∪ half-space", topo::union(&wedge, &brick, tol)),
                    ("half-space ∪ wedge", topo::union(&brick, &wedge, tol)),
                ] {
                    // The flush slab is decided within the band, so the
                    // volume is good to ε over the cap.
                    solid_truth::assert_is(
                        &format!("{label}, {order} (start {:.3}·ε)", far / eps),
                        &got,
                        Want::Body(v, 1e-9 * v + 2.0 * eps * cap_area),
                        &|q| Op::Union.depth(&sw, &sb, q),
                        &named,
                        &points,
                    );
                }
            }
        }
    }
    assert_eq!(
        past_the_apex_reading, 10,
        "the poses the far start alone refuses: 20 of the 24 unions"
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
    let frustum =
        topo::transform_rigid(&frustum, &geom_core::test_support::rigid(pose, tol), tol).unwrap();
    let apex = Point3::new(0.0, apex_y, 0.0);
    let turn = *pose
        * Affine3::rotation_about_axis(apex, Vec3::new(0.0, 0.0, 1.0), 1e-6 - FRAC_PI_6)
        * Affine3::translation(apex - Point3::origin());
    let bx = [(0.0, 2.0), (-2.0, 2.0), (-2.0, 2.0)];
    let boxed = topo::transform_rigid(
        &brick(bx[0], bx[1], bx[2], tol),
        &geom_core::test_support::rigid(&turn, tol),
        tol,
    )
    .unwrap();
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

/// **A box face built through a frustum's virtual apex builds every
/// op.** The face is a plane through the apex of
/// [`frustum_and_virtual_apex_box`] with `D = sin(1e-6)`, posed off the
/// axes, so it stands only a rounding gap `|δ| ≈ 7e-17` off the apex.
/// The far branch starts `|δ|/D ≈ 7e-11` from it: within ε at the
/// default ε, past the band at 1e-12.
///
/// The frustum stays `≈ 0.55/cos α` clear of the apex, and over that
/// span the pair stands within ε of the section by the oracle
/// ([`pair_miss`]), so every op builds, in both orders, at both ε, against the
/// closed-form truth. The two apex heights put the frustum on the near
/// nappe (start `|δ|/M ≈ 1e-16`) and on the far one.
///
/// At 1e-6, `D` itself lies in the band, and the row skips loudly.
#[test]
fn a_box_face_through_a_virtual_apex_builds_every_op() {
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
        let rims = [0.25, 1.0].map(|y| (pose.transform_point(p(0.0, y, 0.0)) - apex).dot(axis));
        let Surface::Cone { half_angle, .. } = cone else {
            unreachable!("a cone")
        };
        let d = half_angle.sin() * axis.cross(normal).norm()
            - half_angle.cos() * axis.dot(normal).abs();
        if d <= k * eps {
            test_utils::vacuity::stood_down(
                "a box face through a virtual apex",
                &format!(
                    "at ε = {eps:e}, D = {d:e} leaves the box face within the band of tangent \
                     to the cone, which the boolean decides as a coincidence before the apex \
                     lane: no op is built or checked"
                ),
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
        let label = format!("apex at {apex_y}, start {far:e} (its own nappe {start:e})");
        let bound = pair_miss(
            &cone,
            (origin, normal),
            (rims[0].min(rims[1]), rims[0].max(rims[1])),
        );
        assert_eq!(
            start_class(bound),
            Start::Served,
            "{label}: the pair stands {bound:e} off the section over the frustum"
        );
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
            solid_truth::assert_is(
                &format!("{label}, {op}"),
                &got,
                Want::Body(v, 1e-9 * v.max(1.0)),
                &|q| truth.depth(a, b, q),
                &[],
                &points,
            );
        }
    }
    assert!(
        own.iter().any(|&(s, far)| s < 1e-3 * far) && own.iter().any(|&(s, far)| s == far),
        "one pose puts the frustum on the near nappe and one on the far: {own:?}"
    );
}
