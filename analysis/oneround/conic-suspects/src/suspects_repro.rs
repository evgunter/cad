//! Reproductions (public API only) of the wrong answers the conic-suspects
//! lane measured on main at dc263f855. Each test states the right answer and
//! FAILS on main. Run at `CAD_TOLERANCE_EPS` 1e-9 and 1e-12.
#![allow(clippy::unwrap_used)]
use geom::{Conic, Curve3, Ellipse, Frame, Surface};
use geom_brep::curve_pair::on_surface::{Charted, CoSurface, OnSurface, SurfaceCurve};
use geom_brep::curve_pair::{Span, meet};
use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};

/// The stored numbers `plane_cone_section` mints (the earlier lane's ladder):
/// an ellipse held on its arc at its major vertex, semi-axes `a`, `0.3 a`.
struct Fixture {
    band: Band,
    eps: f64,
    a: f64,
    e: Ellipse<f64>,
    carrier: Curve3<f64>,
    /// 0.3 m of arc from the anchor.
    theta: f64,
    p: Point3<f64>,
    /// The unit tangent at `p`, the plane's normal, and the in-plane normal of the arc at `p`.
    tan: Vec3<f64>,
    n: Vec3<f64>,
    d: Vec3<f64>,
    span: (f64, f64),
}

const ANCHOR: (f64, f64, f64) = (1.3, -0.7, 0.4);
const AXIS: (f64, f64, f64) = (0.5522240050794545, 0.3110026418424273, 0.7735153553621528);
const TOWARD: (f64, f64, f64) = (0.6816514125372342, -0.702621773621506, -0.20414209516648582);

fn fixture() -> Fixture {
    let band = Band::linear(Tol::witness()).unwrap();
    let eps = band.zero();
    // where each answer below is wrong: 1e9 at eps 1e-9, 1e6 at eps 1e-12 (the tables give every size)
    let a = if eps < 1e-10 { 1e6 } else { 1e9 };
    let v = |(x, y, z)| Vec3::new(x, y, z);
    let e = Ellipse::anchored(Point3::new(ANCHOR.0, ANCHOR.1, ANCHOR.2), v(AXIS), v(TOWARD), a, 0.3 * a);
    let theta = 0.3 / (0.3 * a);
    let (p, tan, n) = (e.point_at(theta), e.derivative_at(theta).normalize(), e.frame().axis());
    let half = 10.0 / (0.3 * a);
    Fixture { band, eps, a, e, carrier: Curve3::Ellipse(e), theta, p, tan, n, d: tan.cross(n).normalize(), span: (theta - half, theta + half) }
}

fn meetings(f: &Fixture, other: &Curve3<f64>, span: (f64, f64)) -> usize {
    meet(&Span::new(other, span), &Span::new(&f.carrier, f.span), f.band).map_or(usize::MAX, |m| m.points.len())
}

/// 1. A line piercing the ellipse's plane through a point of the arc, 30 degrees off the plane.
#[test]
fn a_line_piercing_the_plane_through_a_point_of_the_arc_meets_it() {
    let f = fixture();
    let dir = (f.d * 30.0_f64.to_radians().cos() + f.n * 30.0_f64.to_radians().sin()).normalize();
    let line = Curve3::line(f.p - dir * 2.0, dir);
    assert_ne!(meetings(&f, &line, (0.0, 4.0)), 0, "a = {:e}: Ok, no meeting", f.a);
}

/// 2. A line along the major axis that starts 30 eps inside the anchor vertex and runs out through it.
#[test]
fn a_line_out_through_the_anchor_vertex_meets_it() {
    let f = fixture();
    let u = f.e.frame().u_ref();
    let line = Curve3::line(f.e.point_at(0.0) - u * (30.0 * f.eps), u);
    let half = 10.0 / (0.3 * f.a);
    let got = meet(&Span::new(&line, (0.0, 2.0)), &Span::new(&f.carrier, (-half, half)), f.band);
    assert_ne!(got.map_or(usize::MAX, |m| m.points.len()), 0, "a = {:e}: Ok, no meeting", f.a);
}

/// 3. A unit circle in the ellipse's plane, crossing the arc 0.4 m from `p`.
#[test]
fn a_circle_in_the_plane_crossing_the_arc_meets_it() {
    let f = fixture();
    let circle = Curve3::circle(f.p + f.tan * 0.6, f.n, 1.0, -f.tan);
    assert_ne!(meetings(&f, &circle, (-1.0, 1.0)), 0, "a = {:e}: Ok, no meeting", f.a);
}

/// 4. A 2 x 1 ellipse, held at its centre, in the plane and crossing the arc.
#[test]
fn a_small_ellipse_in_the_plane_crossing_the_arc_meets_it() {
    let f = fixture();
    let small = Curve3::Ellipse(Ellipse::new(Frame::new(f.p + f.tan * 1.2, f.n, -f.tan), 2.0, 1.0));
    assert_ne!(meetings(&f, &small, (-1.0, 1.0)), 0, "a = {:e}: Ok, no meeting", f.a);
}

/// 5. The same arc-held ellipse turned 0.7 rad about a line through `p`: both pass through `p`.
#[test]
fn two_arc_held_ellipses_through_one_point_meet() {
    let f = fixture();
    let v = |(x, y, z)| Vec3::new(x, y, z);
    for (name, about) in [("in one plane", f.n), ("across planes", f.d)] {
        let m = Affine3::rotation_about_axis(f.p, about, 0.7);
        let turned = Curve3::Ellipse(Ellipse::anchored(m.transform_point(Point3::new(ANCHOR.0, ANCHOR.1, ANCHOR.2)), m.transform_vec(v(AXIS)), m.transform_vec(v(TOWARD)), f.a, 0.3 * f.a));
        assert_ne!(meetings(&f, &turned, f.span), 0, "a = {:e}, {name}: Ok, no meeting", f.a);
    }
}

/// 6. `OnSurface::under`: a line of the ellipse's own plane (the plane through its stored point with its stored axis).
#[test]
fn a_line_of_the_edges_own_plane_lies_under_it() {
    let f = fixture();
    let key = geom_brep::SurfaceKey::default();
    let chart = Charted::held(key, Surface::Plane { frame: Frame::new(f.p, f.n, f.d) });
    let line = SurfaceCurve::u_line(&chart, Point2::new(0.0, 0.0)).unwrap();
    let got = OnSurface::edge(key, &f.carrier, f.span).under(&line, (0.0, 4.0), false, f.band);
    assert!(matches!(got, Ok(CoSurface::InPlane)), "a = {:e}: {got:?}", f.a);
}

/// 7. `OnSurface::under`, the other direction (eps 1e-12, a = 1e7, at the anchor): a line 30 eps ABOVE the plane.
#[test]
fn a_line_thirty_eps_above_the_edges_plane_is_not_in_it() {
    let f = fixture();
    let a = 1e7;
    let v = |(x, y, z)| Vec3::new(x, y, z);
    let e = Ellipse::anchored(Point3::new(ANCHOR.0, ANCHOR.1, ANCHOR.2), v(AXIS), v(TOWARD), a, 0.3 * a);
    let (n, d) = (e.frame().axis(), e.derivative_at(0.0).normalize().cross(e.frame().axis()).normalize());
    let carrier = Curve3::Ellipse(e);
    let half = 10.0 / (0.3 * a);
    let key = geom_brep::SurfaceKey::default();
    let chart = Charted::held(key, Surface::Plane { frame: Frame::new(e.point_at(0.0) + n * (30.0 * f.eps), n, d) });
    let line = SurfaceCurve::u_line(&chart, Point2::new(0.0, 0.0)).unwrap();
    let got = OnSurface::edge(key, &carrier, (-half, half)).under(&line, (0.0, 4.0), false, f.band);
    assert!(!matches!(got, Ok(CoSurface::InPlane)), "a = {a:e}: InPlane for a line 30 eps off the plane");
}
