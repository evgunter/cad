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

use super::{ball_off_face, clusters, edge_clear_of_ball, reach, speed_bound};
use crate::body::Body;
use crate::entity::EdgeKey;
use crate::euler::MevSite;
use crate::test_support_fixtures::{CylFrame, cyl_wall_sheet};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
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
        major,
        minor,
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
                let found = clusters(
                    &s,
                    &c,
                    speed_bound(&c).unwrap(),
                    reach(&s).unwrap(),
                    span,
                    band(),
                )
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
    let found = clusters(
        &s,
        &c,
        speed_bound(&c).unwrap(),
        reach(&s).unwrap(),
        (-3.0, 3.0),
        band(),
    )
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
            witness: carrier.eval(0.5 * (t0 + t1)),
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
        None,
        (0.0, FRAC_PI_2),
        (0.0, 1.0),
        tol,
    );
    let on = |u: f64, v: f64| Point3::new(u.cos(), u.sin(), v);
    let in_band = -0.5 * (band.zero() + band.escalate());
    for (what, foot, off) in [
        ("in the band of the corner vertex", on(in_band, 0.0), false),
        ("half a radian past the sheet", on(-0.5, 0.5), true),
        ("in the sheet's middle", on(0.7, 0.5), false),
    ] {
        let got = ball_off_face(&body, face, foot, 1e-3, 1.0, band);
        assert!(
            matches!(got, Ok(b) if b == off),
            "a ball about a foot {what}: {got:?}"
        );
    }
}
