//! M1: first-degree roots, three ways, through the plane door
//! (`plane_crossing_lane`) and `curve_pair::meet`.

use std::io::Write;

use geom::{Conic, Curve3, Ellipse};
use geom_core::{Point3, Vec3};

use super::{band, minted, out, p3, stored, v3};
use crate::curve_face::plane_crossing::{ConicPlaneMeet, PlaneCrossingLane, plane_crossing_lane};
use crate::curve_pair::{Span, meet};

struct Case {
    theta: f64,
    half: f64,
    gamma: f64,
    place: &'static str,
}

fn cases(e: &Ellipse<f64>) -> Vec<Case> {
    let (a, b) = e.semi_axes();
    let _ = a;
    let mut out = Vec::new();
    for gamma in [90.0_f64, 30.0, 5.0] {
        for (s, place) in [(0.3, "near"), (-3.0, "near"), (1000.0, "mid")] {
            let theta = s / b;
            if theta.abs() < 0.3 {
                out.push(Case { theta, half: (10.0 / b).max(2.0 * theta.abs()).min(0.2), gamma: gamma.to_radians(), place });
            }
        }
        for theta in [0.5, 2.0] {
            out.push(Case { theta, half: 0.1, gamma: gamma.to_radians(), place: "far" });
        }
    }
    out
}

/// The pose a case reads: the target point, the unit tangent there, a
/// plane normal crossing the arc at `gamma`, and a unit direction in
/// that plane.
fn pose(e: &Ellipse<f64>, c: &Case) -> (Point3<f64>, Vec3<f64>, Vec3<f64>, Vec3<f64>) {
    let t = e.point_at(c.theta);
    let tan = e.derivative_at(c.theta).normalize();
    let n = e.axis();
    let w = (n.cross(tan).normalize() * 0.7_f64.cos() + n * 0.7_f64.sin()).normalize();
    let normal = (tan * c.gamma.sin() + w * c.gamma.cos()).normalize();
    let in_plane = normal.cross(w.cross(tan)).normalize();
    (t, tan, normal, in_plane)
}

fn log_json() -> String {
    let lines: Vec<String> = geom::probe::take_log().into_iter().map(|l| format!("{l:?}")).collect();
    format!("[{}]", lines.join(","))
}

#[test]
#[ignore = "probe"]
fn m1_first_degree_roots() {
    let band = band();
    let mut f = out("m1");
    for fat in [true, false] {
        for target in [1e4, 1e5, 1e6, 1e7, 1e8, 1e9] {
            let Some(e) = minted(target, fat) else {
                writeln!(f, "{{\"door\":\"mint\",\"fat\":{fat},\"target\":{target:?},\"refused\":true}}").unwrap();
                continue;
            };
            let carrier = Curve3::Ellipse(e);
            for (i, c) in cases(&e).iter().enumerate() {
                let (t, tan, normal, in_plane) = pose(&e, c);
                let head = format!(
                    "\"fat\":{fat},\"target\":{target:?},\"case\":{i},\"place\":\"{}\",\"theta\":{:?},\"gamma\":{:?},\"span\":[{:?},{:?}],{}",
                    c.place, c.theta, c.gamma, c.theta - c.half, c.theta + c.half, stored(&e)
                );
                // Door A: the plane door.
                let origin = t + in_plane * 0.37;
                for variant in 0..3u8 {
                    let _ = geom::probe::take_log();
                    let got = geom::probe::with_variant(variant, || {
                        plane_crossing_lane(&carrier, c.theta - c.half, c.theta + c.half, origin, normal, band)
                    });
                    let (kind, roots) = match got {
                        PlaneCrossingLane::Conic(ConicPlaneMeet::Roots(Ok(r))) => ("roots".to_owned(), r),
                        PlaneCrossingLane::Conic(ConicPlaneMeet::Roots(Err(why))) => (format!("refused:{why:?}"), vec![]),
                        PlaneCrossingLane::Conic(ConicPlaneMeet::Miss) => ("miss".to_owned(), vec![]),
                        PlaneCrossingLane::Conic(ConicPlaneMeet::InPlane) => ("inplane".to_owned(), vec![]),
                        other => (format!("{other:?}"), vec![]),
                    };
                    writeln!(
                        f,
                        "{{\"door\":\"A\",\"variant\":{variant},{head},\"o\":{},\"n\":{},\"kind\":{kind:?},\"roots\":{roots:?}}}",
                        p3(origin), v3(normal)
                    )
                    .unwrap();
                }
                // Door B: curve_pair::meet, the ellipse against a unit
                // circle through the target in the same plane as door A's.
                let e_dir = in_plane;
                let centre = t - e_dir;
                let circle = Curve3::circle(centre, normal, 1.0, e_dir);
                for variant in 0..3u8 {
                    let _ = geom::probe::take_log();
                    let got = geom::probe::with_variant(variant, || {
                        meet(
                            &Span::new(&carrier, (c.theta - c.half, c.theta + c.half)),
                            &Span::new(&circle, (-1.0, 1.0)),
                            band,
                        )
                    });
                    let span_a = Span::new(&carrier, (c.theta - c.half, c.theta + c.half));
                    let span_b = Span::new(&circle, (-1.0, 1.0));
                    let (kind, pts) = match &got {
                        Ok(m) => (
                            format!("ok shared={:?}", m.shared),
                            m.points.iter().map(|m| format!("{{\"p\":{},\"ta\":{:?},\"tb\":{:?},\"on_a\":{:?},\"on_b\":{:?}}}", p3(m.point), span_a.at(m.on_a), span_b.at(m.on_b), format!("{:?}", m.on_a), format!("{:?}", m.on_b))).collect::<Vec<_>>(),
                        ),
                        Err(r) => (format!("refused:{:?}", r.fault), vec![]),
                    };
                    writeln!(
                        f,
                        "{{\"door\":\"B\",\"variant\":{variant},{head},\"c\":{},\"m\":{},\"cu\":{},\"kind\":{kind:?},\"points\":[{}],\"log\":{}}}",
                        p3(centre), v3(normal), v3(e_dir), pts.join(","), log_json()
                    )
                    .unwrap();
                }
                // Door C: curve_pair::meet, a line in the ellipse's plane.
                let d = (tan * c.gamma.cos() + e.axis().cross(tan).normalize() * c.gamma.sin()).normalize();
                let lo = t - d * 2.0;
                let line = Curve3::line(lo, d);
                for variant in 0..3u8 {
                    let _ = geom::probe::take_log();
                    let span_a = Span::new(&line, (0.0, 4.0));
                    let span_b = Span::new(&carrier, (c.theta - c.half, c.theta + c.half));
                    let got = geom::probe::with_variant(variant, || meet(&span_a, &span_b, band));
                    let (kind, pts) = match &got {
                        Ok(m) => (
                            format!("ok shared={:?}", m.shared),
                            m.points.iter().map(|m| format!("{{\"p\":{},\"ta\":{:?},\"tb\":{:?},\"on_a\":{:?},\"on_b\":{:?}}}", p3(m.point), span_a.at(m.on_a), span_b.at(m.on_b), format!("{:?}", m.on_a), format!("{:?}", m.on_b))).collect::<Vec<_>>(),
                        ),
                        Err(r) => (format!("refused:{:?}", r.fault), vec![]),
                    };
                    writeln!(
                        f,
                        "{{\"door\":\"C\",\"variant\":{variant},{head},\"lo\":{},\"ld\":{},\"kind\":{kind:?},\"points\":[{}],\"log\":{}}}",
                        p3(lo), v3(d), pts.join(","), log_json()
                    )
                    .unwrap();
                }
            }
        }
    }
}

/// Door D: `conic_quadric_roots`' first-harmonic lane, whose roots are
/// `harmonic_roots_as`'. D1: a circle of radius `a` (stored at its
/// centre, as every circle is) against a unit sphere beside its arc. D2:
/// the minted ellipse against the wall along which it projects to a
/// circle of radius `b`, the wall's axis 1 m beside the ellipse's own.
#[test]
#[ignore = "probe"]
fn m1_harmonic_roots_as() {
    use crate::curve_face::conic_quadric::conic_quadric_roots;
    use geom::{Frame, Surface};
    let band = band();
    let mut f = out("m1d");
    let show = |got: &Result<Result<crate::curve_face::circle_roots::CircleRoots<f64>, crate::roots::Unsettled>, crate::curve_face::conic_quadric::ConicQuadricRefusal>| match got {
        Ok(Ok(r)) => (format!("{}", match r { crate::roots::Roots::Miss => "miss", crate::roots::Roots::On(_) => "on", _ => "roots" }), match r { crate::roots::Roots::Certified(found) => found.roots().to_vec(), _ => vec![] }),
        Ok(Err(u)) => (format!("unsettled:{u:?}"), vec![]),
        Err(e) => (format!("refused:{e:?}"), vec![]),
    };
    let (ex, ey, ez) = super::triple();
    for target in [1e2, 1e3, 1e4, 1e5, 1e6, 1e7] {
        // D1: the circle through `p0`, its centre `target` off along ex.
        let p0 = Point3::new(1.3, -0.7, 0.4);
        let circle = Curve3::circle(p0 - ex * target, ez, target, ex);
        for (i, (off, depth)) in [(0.3, 0.2), (-3.0, 0.7), (0.3, 0.999)].into_iter().enumerate() {
            // A unit sphere whose centre stands `depth` off the arc, at
            // `off` metres of arc from p0.
            let theta = off / target;
            let Curve3::Circle(c) = circle else { unreachable!() };
            let at = c.point_at(theta);
            let radial = (at - c.frame().origin()).normalize();
            let centre = at + radial * depth * 0.6 + ez * depth * 0.8;
            let sphere = Surface::Sphere { frame: Frame::new(centre, ez, ex), radius: 1.0 };
            let half = 5.0 / target;
            for variant in [0u8] {
                let _ = geom::probe::take_log();
                let got = geom::probe::with_variant(variant, || conic_quadric_roots(&circle, theta - half, theta + half, &sphere, band));
                let (kind, roots) = show(&got);
                writeln!(f, "{{\"door\":\"D1\",\"variant\":{variant},\"target\":{target:?},\"case\":{i},\"S\":{},\"axis\":{},\"u\":{},\"a\":{target:?},\"b\":{target:?},\"phase\":[0.0,0.0],\"span\":[{:?},{:?}],\"o\":{},\"waxis\":null,\"r\":1.0,\"kind\":{kind:?},\"roots\":{roots:?},\"log\":{}}}",
                    p3(c.frame().origin()), v3(ez), v3(ex), theta - half, theta + half, p3(centre), log_json()).unwrap();
            }
        }
        // D2.
        if target < 1e4 { continue; }
        for fat in [true, false] {
            let Some(e) = minted(target, fat) else { continue };
            let (a, b) = e.semi_axes();
            let (n, u, v) = (e.axis(), e.u_ref(), e.v_ref());
            let cg = b / a;
            let w = (n * cg + u * (1.0 - cg * cg).sqrt()).normalize();
            let carrier = Curve3::Ellipse(e);
            for (i, shift) in [1.0, -6.0, 200.0].into_iter().enumerate() {
                let origin = e.frame().origin() + v * shift;
                let wall = Surface::Cylinder { frame: Frame::new(origin, w, v), radius: b };
                let theta = shift / (2.0 * b);
                let half = (10.0 / b).max(2.0 * theta.abs()).min(0.2);
                for variant in 0..3u8 {
                    let _ = geom::probe::take_log();
                    let got = geom::probe::with_variant(variant, || conic_quadric_roots(&carrier, theta - half, theta + half, &wall, band));
                    let (kind, roots) = show(&got);
                    writeln!(f, "{{\"door\":\"D2\",\"variant\":{variant},\"fat\":{fat},\"target\":{target:?},\"case\":{i},{},\"span\":[{:?},{:?}],\"o\":{},\"waxis\":{},\"r\":{b:?},\"kind\":{kind:?},\"roots\":{roots:?},\"log\":{}}}",
                        stored(&e), theta - half, theta + half, p3(origin), v3(w), log_json()).unwrap();
                }
            }
        }
    }
}
