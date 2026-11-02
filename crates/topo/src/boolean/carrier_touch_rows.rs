//! **The localization keeps every meeting, and an edge is cleared only
//! when it is clear, whatever order an ellipse's semi-axes are stored
//! in.**
//!
//! `geom::Curve3::Ellipse` certifies its semi-axes positive and not
//! ordered: an ellipse stored minor first names the same curve through
//! a `u_ref` along its minor axis, and STEP import stores the semi-axes
//! as written. Each ellipse row here runs the same geometric ellipse in
//! both storages, so a bound read off one field (the larger is `major`)
//! reds one of the pair whichever field it reads. The torus-axis row
//! pins a piece whose middle lies on the axis, where the closed-form
//! foot divides by zero.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::FRAC_PI_2;

use geom::{Curve3, Surface};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, SurfacePair};
use geom_core::{Band, Point3, Tol, Vec3};

use super::{ball_off_face, boxes, clusters, edge_clear_of_ball, off_face, reach};
use crate::body::Body;
use crate::entity::EdgeKey;
use crate::euler::MevSite;
use crate::test_support_fixtures::{CylFrame, cyl_wall_sheet};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// The rates [`clusters`] reads: the speed bound and the curvature bound.
fn rates(c: &Curve3<f64>) -> (geom_core::SupSpeed<f64>, f64) {
    (
        geom::carrier_speed::speed_range(c).1,
        geom::carrier_speed::curvature_hi(c),
    )
}

/// The ellipse about `center` in the plane normal to the unit `axis`,
/// with semi-axis `a` along the unit `along` (square to `axis`) and `b`
/// along `axis × along`. It is stored with `a` in `major` (so `u_ref` is
/// `along`) or, `b_in_major`, with `b` there (so `u_ref` is
/// `axis × along`, and the parameter runs a quarter turn behind the
/// other storage's, [`param`]).
fn ellipse_in(
    center: Point3<f64>,
    axis: Vec3<f64>,
    along: Vec3<f64>,
    (a, b): (f64, f64),
    b_in_major: bool,
) -> Curve3<f64> {
    let (major, minor, u_ref) = if b_in_major {
        (b, a, axis.cross(along))
    } else {
        (a, b, along)
    };
    Curve3::Ellipse {
        center,
        axis,
        semi_u: major,
        semi_v: minor,
        u_ref,
    }
}

/// [`ellipse_in`] in the plane `z = 0` about the origin, `a` along x.
fn ellipse(a: f64, b: f64, b_in_major: bool) -> Curve3<f64> {
    ellipse_in(
        Point3::origin(),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
        (a, b),
        b_in_major,
    )
}

/// The parameter of [`ellipse_in`]'s storage at the point the
/// `a`-in-`major` storage reaches at `t`.
fn param(t: f64, b_in_major: bool) -> f64 {
    if b_in_major { t - FRAC_PI_2 } else { t }
}

/// **A small sphere crossed twice by an ellipse keeps both crossings,
/// in either storage.** The sphere (radius 0.02) is centred 0.01 off
/// the curve, so the ellipse dips 0.01 into it and crosses it
/// transversally twice, one crossing either side of the point nearest
/// its centre: a sound localization holds a cluster on each side. The
/// ellipses are long (axis ratios up to 40), so a speed bound read off
/// the smaller semi-axis under-states arc length by that ratio and
/// clears pieces that hold the crossings.
#[test]
fn an_ellipse_keeps_its_crossings_in_either_storage() {
    for (a, b) in [(1.0, 0.1), (2.0, 0.05), (1.0, 0.3)] {
        for k in 0..20 {
            let tc = 0.3 + 0.12 * f64::from(k);
            let p = ellipse(a, b, false).eval(tc);
            let s = Surface::Sphere {
                center: p + Vec3::new(0.0, 0.0, 0.01),
                radius: 0.02,
                axis: Vec3::new(0.0, 0.0, 1.0),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            };
            for b_in_major in [false, true] {
                let c = ellipse(a, b, b_in_major);
                let t = param(tc, b_in_major);
                assert!((c.eval(t) - p).norm() < 1e-12, "the storages agree");
                let span = (param(0.0, b_in_major), param(3.0, b_in_major));
                let found = clusters(&s, &c, rates(&c), reach(&s).unwrap(), span, band())
                    .expect("localizes within the budgets");
                assert!(
                    found.iter().any(|&(_, hi)| hi <= t) && found.iter().any(|&(lo, _)| lo >= t),
                    "a = {a}, b = {b}, b_in_major = {b_in_major}, tc = {tc}: a crossing either \
                     side of {t}, got {found:?}"
                );
            }
        }
    }
}

/// **A line over a torus's axis, tangent to its top twice, localizes
/// both touches.** The span `x ∈ [−3, 3]` at height `r` over the
/// `R = 2, r = 1/2` torus has its middle on the axis, where the foot
/// is undefined: that piece decides nothing and is cut, and the touches
/// at `x = ±2` each keep a cluster.
#[test]
fn a_span_whose_middle_is_on_the_torus_axis_localizes_both_touches() {
    let s = Surface::Torus {
        center: Point3::origin(),
        axis: Vec3::new(0.0, 0.0, 1.0),
        major_radius: 2.0,
        minor_radius: 0.5,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let c = Curve3::Line {
        origin: Point3::new(0.0, 0.0, 0.5),
        dir: Vec3::new(1.0, 0.0, 0.0),
    };
    let found = clusters(&s, &c, rates(&c), reach(&s).unwrap(), (-3.0, 3.0), band())
        .expect("localizes within the budgets");
    for touch in [-2.0, 2.0] {
        assert!(
            found.iter().any(|&(lo, hi)| lo <= touch && touch <= hi),
            "the touch at x = {touch} keeps a cluster: {found:?}"
        );
    }
}

/// The section of the unit cylinder about z by the plane through the
/// origin tilted π/3 from z towards x: the ellipse with semi-axis 2
/// along the plane's steepest descent and 1 along y.
fn section(b_in_major: bool) -> Curve3<f64> {
    let tilt = core::f64::consts::FRAC_PI_3;
    ellipse_in(
        Point3::origin(),
        Vec3::new(tilt.sin(), 0.0, tilt.cos()),
        Vec3::new(tilt.cos(), 0.0, -tilt.sin()),
        (2.0, 1.0),
        b_in_major,
    )
}

/// A body holding one elliptic edge: the arc `t ∈ [0.3, 1.2]` of the
/// `a`-in-`major` storage of [`section`], in the given storage,
/// described as the plane against the cylinder.
fn elliptic_edge(b_in_major: bool) -> (Body<f64>, EdgeKey) {
    let carrier = section(b_in_major);
    let (t0, t1) = (param(0.3, b_in_major), param(1.2, b_in_major));
    let (p0, p1) = (carrier.eval(t0), carrier.eval(t1));
    let Curve3::Ellipse { axis, u_ref, .. } = carrier else {
        unreachable!("a section is an ellipse")
    };
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(p0, true).unwrap();
    let plane = body.add_surface(Surface::Plane {
        origin: Point3::origin(),
        normal: axis,
        u_ref,
    });
    let cyl = body.add_surface(Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    });
    let arc = EdgeCurveSpec {
        description: EdgeDescriptionSpec::Intersection {
            pair: SurfacePair::new(plane, cyl),
            selector: geom_brep::CurveSelector::Witness(carrier.eval(0.5 * (t0 + t1))),
        },
        carrier,
        param_start: t0,
        param_end: t1,
    };
    let e = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p1,
            arc,
            Tol::witness(),
        )
        .unwrap();
    (body, e.edge)
}

/// **A ball about a point of an elliptic edge is not cleared of it, in
/// either storage; a ball off the annulus the ellipse lies in is.** The
/// ball centres on the arc's own points (inside the annulus between
/// the semi-axes, so a gap read with the semi-axes in the wrong order
/// is positive there); its radius, 1e-3, is far above the band and far
/// below the annulus. The control centres the same ball 0.05 off the
/// plane of the ellipse, where every storage must clear it.
#[test]
fn an_elliptic_edge_is_not_cleared_of_a_ball_on_it_in_either_storage() {
    for b_in_major in [false, true] {
        let (body, edge) = elliptic_edge(b_in_major);
        let reference = section(false);
        for k in 0..=8 {
            let at = reference.eval(0.3 + 0.9 * f64::from(k) / 8.0);
            assert!(
                !edge_clear_of_ball(&body, edge, at, 1e-3, band()).unwrap(),
                "b_in_major = {b_in_major}: the ball about {at:?} on the arc is cleared of it"
            );
            let Curve3::Ellipse { axis, .. } = reference else {
                unreachable!("a section is an ellipse")
            };
            let above = at + axis * 0.05;
            assert!(
                edge_clear_of_ball(&body, edge, above, 1e-3, band()).unwrap(),
                "b_in_major = {b_in_major}: the ball 0.05 above the plane is not cleared"
            );
        }
    }
}

/// **A ball holding a vertex of the face is not off it, wherever the
/// band puts the vertex against the foot.** The quarter sheet of the
/// unit cylinder about z over azimuths `[0, π/2]` and heights `[0, 1]`;
/// the foot sits on the carrier just past the sheet's corner `(1, 0, 0)`,
/// midway through the band from it, so the face's placement door cannot
/// tell the foot from that vertex. The ball (radius 1e-3) holds the
/// vertex, so it is not off the face, and the answer is `false`, not
/// the placement's escalation. Controls: the same ball half a radian
/// past the sheet is off it, and one in the sheet's middle is not.
#[test]
fn a_ball_holding_a_face_vertex_in_the_band_of_its_foot_is_not_off_the_face() {
    let tol = Tol::witness();
    let band = band();
    let mut body = Body::<f64>::new();
    let face = cyl_wall_sheet(
        &mut body,
        CylFrame::canonical(1.0),
        (0.0, FRAC_PI_2),
        (0.0, 1.0),
        tol,
    );
    crate::test_support_fixtures::set_sheet_sense(&mut body, face, true, tol);
    let on = |u: f64, v: f64| Point3::new(u.cos(), u.sin(), v);
    let in_band = -0.5 * (band.zero() + band.escalate());
    for (what, foot, off) in [
        ("in the band of the corner vertex", on(in_band, 0.0), false),
        ("half a radian past the sheet", on(-0.5, 0.5), true),
        ("in the sheet's middle", on(0.7, 0.5), false),
    ] {
        let got = ball_off_face(&body, face, foot, 1e-3, super::Reach::Bounded(1.0), band);
        assert!(
            matches!(got, Ok(b) if b == off),
            "a ball about a foot {what}: {got:?}"
        );
    }
}

/// The point of the unit cylinder about z at azimuth `u` and height
/// `1/2`, its outward normal and its tangent along the azimuth.
fn on_unit_cylinder(u: f64) -> (Point3<f64>, Vec3<f64>, Vec3<f64>) {
    let (sin, cos) = u.sin_cos();
    (
        Point3::new(cos, sin, 0.5),
        Vec3::new(cos, sin, 0.0),
        Vec3::new(-sin, cos, 0.0),
    )
}

/// The spiric oval of the torus `R = 1/2, r = 1/5` cut at offset `1/10`,
/// placed in the plane `z = 1/2` so that its outer extreme `v = 0` is
/// the cylinder's point at azimuth `u`, its tangent along the azimuth
/// there and the oval on the far side of that tangent from the axis:
/// it touches the unit cylinder at `v = 0` from outside, and nowhere
/// else.
fn spiric_touching(u: f64) -> Curve3<f64> {
    let (p, n, t) = on_unit_cylinder(u);
    let z = Vec3::new(0.0, 0.0, 1.0);
    // `P(0)` lies along `axis × u_ref` from the oval's plane centre, so
    // `axis = −t` puts the oval's centre along `+n` from `p`.
    let at = |center| Curve3::spiric(center, -t, z, 0.5, 0.2, 0.1, band()).unwrap();
    let shift = p - at(Point3::origin()).eval(0.0);
    let c = at(Point3::origin() + shift);
    assert!(
        (c.eval(0.0) - p).norm() < 1e-12,
        "the oval's extreme is the touch"
    );
    assert!(
        (Point3::origin() + shift + z * 0.1 - p).dot(n) > 0.0,
        "the oval lies outside the cylinder's tangent there"
    );
    c
}

/// The quadratic Bézier in the plane `z = 1/2` from `p + n/10 − 3t/10`
/// through `p` at `s = 1/2` to `p + n/10 + 3t/10`, `(p, n, t)` the
/// cylinder's point at azimuth `u`: its offset along `n` is
/// `(2s − 1)²/10`, so it touches the unit cylinder at `s = 1/2` from
/// outside, and nowhere else.
fn spline_touching(u: f64) -> Curve3<f64> {
    use geom::NurbsCurve3;
    use geom_core::spline::{KnotVector, Knots};
    let (p, n, t) = on_unit_cylinder(u);
    let control = vec![p + n * 0.1 - t * 0.3, p - n * 0.1, p + n * 0.1 + t * 0.3];
    let knots = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let c = Curve3::Nurbs(std::sync::Arc::new(
        NurbsCurve3::new(Knots::from(knots), control, vec![1.0; 3]).unwrap(),
    ));
    assert!((c.eval(0.5) - p).norm() < 1e-12, "the middle is the touch");
    c
}

/// **A spiric or spline span tangent to a wall is certified off the
/// face where the touch lies past it, and not where it lies in it.**
/// The face is [`a_ball_holding_a_face_vertex_in_the_band_of_its_foot_is_not_off_the_face`]'s
/// quarter sheet over azimuths `[0, π/2]`; each curve touches the
/// carrier once from outside, at azimuth `−1/2` (off the sheet) or
/// `7/10` (on it). The touch is a double root no root door certifies, so
/// whether the span crosses the face is the localization's to say: the
/// curve's speed and curvature bounds localize the touch into a ball
/// off the face.
#[test]
fn a_spiric_or_spline_touch_past_the_face_is_off_it() {
    let tol = Tol::witness();
    let mut body = Body::<f64>::new();
    let frame = CylFrame::canonical(1.0);
    let face = cyl_wall_sheet(&mut body, frame, (0.0, FRAC_PI_2), (0.0, 1.0), tol);
    crate::test_support_fixtures::set_sheet_sense(&mut body, face, true, tol);
    let surface = frame.surface::<f64>();
    for (u, off) in [(-0.5, true), (0.7, false)] {
        for (what, curve, span) in [
            ("spiric", spiric_touching(u), (-0.4, 0.4)),
            ("spline", spline_touching(u), (0.0, 1.0)),
        ] {
            let got = off_face(&body, face, &surface, &curve, span, band());
            assert!(
                matches!(got, Ok(b) if b == off),
                "a {what} touching the wall at azimuth {u}: {got:?}"
            );
        }
    }
}

/// A body holding one edge on `carrier` over `span`, described as the
/// meeting of `a` and `b`.
fn edge_on(
    carrier: Curve3<f64>,
    (t0, t1): (f64, f64),
    (a, b): (Surface<f64>, Surface<f64>),
) -> (Body<f64>, EdgeKey) {
    let (p0, p1) = (carrier.eval(t0), carrier.eval(t1));
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(p0, true).unwrap();
    let (a, b) = (body.add_surface(a), body.add_surface(b));
    let spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::Intersection {
            pair: SurfacePair::new(a, b),
            selector: geom_brep::CurveSelector::Witness(carrier.mid_point(t0, t1)),
        },
        carrier,
        param_start: t0,
        param_end: t1,
    };
    let e = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p1,
            spec,
            Tol::witness(),
        )
        .unwrap();
    (body, e.edge)
}

/// Whether the floor on `carrier`'s span `(0, 1)` clears a ball of radius
/// 1e-2 about `at`.
fn floor_clears(carrier: &Curve3<f64>, at: Point3<f64>) -> bool {
    crate::splitting::containment::arc_distance_floor(carrier, (0.0, 1.0), at)
        .unwrap()
        .clears(|gap| super::clear(gap, 1e-2, band()))
}

/// **A ball beside a spline or spiric edge, inside the ball that holds
/// the whole edge, is cleared of it piece by piece; a ball on it is
/// not.** Each ball (radius 1e-2) stands 5e-2 off the edge, inside the
/// ball holding the whole edge, so that ball does not clear it; the
/// edge's pieces do. The controls centre the same ball on the edge.
///
/// The spiric is the arc `v ∈ [−1, 1]` of the torus `R = 1.2,
/// r = 0.225` about z cut at `x = 1/20`, read through the door: its
/// ball is its speed bound times its half-width about `P(0)`, and the
/// ball beside it lies inside the edge's box. The spline is the
/// quadratic from the origin through `(1/2, 1/2, 0)` (its `s = 1/2`
/// point) to `(1, 0, 0)`, read on its floor: its control ball is the
/// one about its control points' box centre.
#[test]
fn a_ball_beside_a_spline_or_spiric_edge_is_cleared_of_it() {
    use geom::NurbsCurve3;
    use geom_core::spline::{KnotVector, Knots};
    let (x, y, z) = (
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
    );
    let o = Point3::origin();
    let spiric = Curve3::spiric(o, z, x, 1.2, 0.225, 0.05, band()).unwrap();
    let torus = Surface::Torus {
        center: o,
        axis: z,
        major_radius: 1.2,
        minor_radius: 0.225,
        u_ref: x,
    };
    let plane = Surface::Plane {
        origin: Point3::new(0.05, 0.0, 0.0),
        normal: x,
        u_ref: y,
    };
    let (body, edge) = edge_on(spiric.clone(), (-1.0, 1.0), (plane, torus));
    // Square to the tangent in the oval's plane, towards its far side.
    let inward = |v: f64| {
        let n = x.cross(spiric.deriv(v)).normalize();
        if n.dot(spiric.eval(core::f64::consts::PI) - spiric.eval(v)) > 0.0 {
            n
        } else {
            -n
        }
    };
    let beside = spiric.eval(0.5) + inward(0.5) * 0.05;
    assert!(
        boxes::edge_box(&body, edge, boxes::sweep_pad(band())).overlaps(&boxes::centred_box(
            beside,
            1e-2,
            boxes::sweep_pad(band())
        )),
        "the ball beside the spiric lies inside its box"
    );
    assert!(
        edge_clear_of_ball(&body, edge, beside, 1e-2, band()).unwrap(),
        "the ball beside the spiric edge is not cleared of it"
    );
    assert!(
        !edge_clear_of_ball(&body, edge, spiric.eval(0.3), 1e-2, band()).unwrap(),
        "the ball on the spiric edge is cleared of it"
    );
    let knots = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let spline = Curve3::Nurbs(std::sync::Arc::new(
        NurbsCurve3::new(
            Knots::from(knots),
            vec![o, Point3::new(0.5, 1.0, 0.0), Point3::new(1.0, 0.0, 0.0)],
            vec![1.0; 3],
        )
        .unwrap(),
    ));
    assert!(
        floor_clears(&spline, Point3::new(0.5, 0.45, 0.0)),
        "the ball beside the spline is not cleared of it"
    );
    assert!(
        !floor_clears(&spline, spline.eval(0.3)),
        "the ball on the spline is cleared of it"
    );
}

/// **A spline with a corner crosses the face past its corner, and is not
/// certified off it.** The degree-1 spline in the plane `z = 1/2` runs
/// along the tangent of the unit cylinder at azimuth `7/10` (on the
/// quarter sheet), `0.28` outside it, to a corner just past its middle
/// point, then turns straight in and ends `~0.008` inside the cylinder:
/// it crosses the sheet once, on the face. Each span is a segment, so a
/// bound read span by span is zero, though the curve's tangent turns a
/// right angle at the knot: the crease bounds no curvature.
#[test]
fn a_spline_with_a_corner_crossing_the_face_is_not_off_it() {
    use geom::NurbsCurve3;
    use geom_core::spline::{KnotVector, Knots};
    let tol = Tol::witness();
    let mut body = Body::<f64>::new();
    let frame = CylFrame::canonical(1.0);
    let face = cyl_wall_sheet(&mut body, frame, (0.0, FRAC_PI_2), (0.0, 1.0), tol);
    crate::test_support_fixtures::set_sheet_sense(&mut body, face, true, tol);
    let surface = frame.surface::<f64>();
    let (p, n, t) = on_unit_cylinder(0.7);
    let m = p + n * 0.28;
    let (a, k) = (m - t * 0.3, m + t * 0.012);
    let b = k - n * 0.288;
    let knots = KnotVector::clamped(vec![0.0, 0.0, 0.52, 1.0, 1.0], 1).unwrap();
    let c = Curve3::Nurbs(std::sync::Arc::new(
        NurbsCurve3::new(Knots::from(knots), vec![a, k, b], vec![1.0; 3]).unwrap(),
    ));
    assert!(
        (b - Point3::new(0.0, 0.0, 0.5)).norm() < 1.0,
        "the end lies inside the cylinder"
    );
    assert!(
        geom::carrier_speed::curvature_hi(&c).is_infinite(),
        "the corner is a crease, which bounds no curvature"
    );
    let got = off_face(&body, face, &surface, &c, (0.0, 1.0), band());
    assert!(
        matches!(got, Ok(false)),
        "the spline crosses the sheet at azimuth ~0.71, on the face: {got:?}"
    );
}

/// **A creased spline crossing the wall past the face is off it, at
/// `Interval`.** Two quadratic spans in the plane `z = 1/2`: the first
/// runs straight out through the unit cylinder at azimuth `−1/2`, past
/// the quarter sheet, crossing it once; the second turns a right angle
/// at the knot and runs along the azimuth, outside the cylinder. The
/// double knot is a crease, so the spline bounds no curvature: at
/// `Interval` its bound is poison, and a second-order bound read with it
/// would poison the first-order one it is maxed with. The crossing is
/// localized on the first bound alone, into a ball off the face.
#[test]
fn a_creased_spline_crossing_past_the_face_is_off_it_at_interval() {
    use geom::NurbsCurve3;
    use geom_core::Interval;
    use geom_core::spline::{KnotVector, Knots};
    let tol = Tol::witness();
    let mut body = Body::<Interval>::new();
    let frame = CylFrame::canonical(1.0);
    let face = cyl_wall_sheet(&mut body, frame, (0.0, FRAC_PI_2), (0.0, 1.0), tol);
    crate::test_support_fixtures::set_sheet_sense(&mut body, face, true, tol);
    let surface = frame.surface::<Interval>();
    let (p, n, t) = on_unit_cylinder(-0.5);
    let corner = p + n * 0.1;
    let knots = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 2.0, 2.0], 2).unwrap();
    let net = NurbsCurve3::new(
        Knots::from(knots),
        vec![p - n * 0.1, p, corner, corner + t * 0.1, corner + t * 0.2],
        vec![1.0; 5],
    )
    .unwrap();
    let c = Curve3::Nurbs(std::sync::Arc::new(
        net.map_scalar(<Interval as geom_core::Real>::from_f64),
    ));
    assert!(
        !geom_core::is_finite_length(geom::carrier_speed::curvature_hi(&c)),
        "the crease bounds no curvature"
    );
    let span = (
        Interval::from_bounds(0.0, 0.0),
        Interval::from_bounds(2.0, 2.0),
    );
    let got = off_face(&body, face, &surface, &c, span, band());
    assert!(
        matches!(got, Ok(true)),
        "the crossing lies past the sheet: {got:?}"
    );
}
