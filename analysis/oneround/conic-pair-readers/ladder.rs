//! **The arc-held ladder**: `meet` for a line and for a unit circle
//! through a point of an ellipse `plane_cone_section` mints on its arc,
//! semi-axes `1e4` to `1e9`, near the anchor and far from it, each pose
//! also planted apart by `10ε` and `1000ε`, at `f64` and at `Interval`.
//!
//! A measurement, kept beside its oracle and never built in the tree: it
//! writes one JSON line per reading to `$CONIC_LADDER_OUT/ladder-eps<ε>.jsonl`,
//! every stored number in its shortest round-trip form, and the 60-digit
//! oracle (`oracle.py`, beside it) judges each line against the curve the
//! stored numbers state. Run by dropping it into `crates/geom-brep/examples/`:
//! `CONIC_LADDER_OUT=<dir> CAD_TOLERANCE_EPS=<ε> cargo run -p geom-brep
//! --features test-support --example ladder`, then `oracle.py <dir>/ladder-eps<ε>.jsonl`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::disallowed_methods)]

use std::fmt::Write as _;
use std::io::Write as _;

use geom::{Conic, Curve3, CurveData, DatumValue, Ellipse, Frame, Surface};
use geom_core::{Band, Interval, Point3, Tol, Vec3};

use geom_brep::curve_pair::{MeetRefusal, Meetings, Span, meet};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn v3(v: Vec3<f64>) -> String {
    format!("[{:?},{:?},{:?}]", v.x, v.y, v.z)
}

fn p3(p: Point3<f64>) -> String {
    v3(p - Point3::origin())
}

/// The section of the cone of half-angle `alpha` about a generic axis,
/// apex `apex`, by the plane tilted `tau` from the axis, `gap` up it.
fn mint(alpha: f64, tau: f64, gap: f64, apex: Point3<f64>) -> Option<Ellipse<f64>> {
    let ez = Vec3::new(0.36, 0.48, 0.8);
    let ex = Vec3::new(0.8, -0.6, 0.0);
    let ey = ez.cross(ex);
    let cone = Surface::Cone {
        frame: Frame::new(apex, ez, ex),
        half_angle: alpha,
    };
    let n = ex * tau.sin() + ez * tau.cos();
    let plane = Surface::Plane {
        frame: Frame::new(apex + ez * gap, n, ey),
    };
    let reach = geom_brep::Reach::fixture(apex, 10.0);
    match geom_brep::intersect::plane_cone_section(&plane, &cone, &reach, band()) {
        Ok(geom_brep::intersect::PlaneConeSection::TiltedEllipse(Curve3::Ellipse(e))) => Some(e),
        _ => None,
    }
}

/// A minted section with major semi-axis about `target`: `fat` holds the
/// semi-axes' ratio near 0.3, else the apex gap is 1 m. The anchor
/// stands near `(1.3, −0.7, 0.4)`.
fn minted(target: f64, fat: bool) -> Option<Ellipse<f64>> {
    const ALPHA: f64 = 1.3;
    let o = Point3::new(0.0, 0.0, 0.0);
    let major = |e: &Ellipse<f64>| e.semi_axes().0.max(e.semi_axes().1);
    let ratio = |e: &Ellipse<f64>| e.semi_axes().0.min(e.semi_axes().1) / major(e);
    let (mut lo, mut hi) = (1e-3, core::f64::consts::FRAC_PI_2 - ALPHA - 1e-12);
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        let grow = match mint(ALPHA, mid, 1.0, o) {
            Some(e) if fat => ratio(&e) > 0.3,
            Some(e) => major(&e) < target,
            None => false,
        };
        if grow {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let unit = mint(ALPHA, lo, 1.0, o)?;
    let gap = if fat { target / major(&unit) } else { 1.0 };
    let first = mint(ALPHA, lo, gap, o)?;
    let anchor = stored_point(&first);
    mint(ALPHA, lo, gap, o + (Point3::new(1.3, -0.7, 0.4) - anchor))
}

fn stored_point(e: &Ellipse<f64>) -> Point3<f64> {
    let CurveData::Analytic(data) = Curve3::Ellipse(*e).data() else {
        unreachable!("an ellipse is analytic")
    };
    data.into_iter()
        .find_map(|(d, v)| match (d, v) {
            (geom::CurveDatum::Anchor | geom::CurveDatum::Center, DatumValue::Point(p)) => Some(p),
            _ => None,
        })
        .expect("a stored point")
}

/// **The session box's own predicate** (`geom_core::tolerance::
/// SESSION_BOX_EXTENT`: no coordinate of a model larger in magnitude),
/// on a span's points: its ends and 4095 parameters between.
fn span_in_box(c: &Curve3<f64>, (lo, hi): (f64, f64)) -> bool {
    let e = geom_core::tolerance::SESSION_BOX_EXTENT;
    (0..=4096).all(|i| {
        let p = c.eval(lo + (hi - lo) * f64::from(i) / 4096.0);
        p.x.abs() <= e && p.y.abs() <= e && p.z.abs() <= e
    })
}

/// The stored numbers, as JSON fields.
fn stored(e: &Ellipse<f64>) -> String {
    let CurveData::Analytic(data) = Curve3::Ellipse(*e).data() else {
        unreachable!("an ellipse is analytic")
    };
    let mut phase = Vec3::new(1.0, 0.0, 0.0);
    let mut centre_held = false;
    for (d, v) in data {
        match (d, v) {
            (geom::CurveDatum::Phase, DatumValue::Direction(p)) => phase = p,
            (geom::CurveDatum::Center, _) => centre_held = true,
            _ => {}
        }
    }
    assert!(!centre_held, "the ladder's ellipses are held on their arcs");
    let (a, b) = e.semi_axes();
    format!(
        "\"S\":{},\"axis\":{},\"u\":{},\"a\":{a:?},\"b\":{b:?},\"phase\":[{:?},{:?}]",
        p3(stored_point(e)),
        v3(e.axis()),
        v3(e.u_ref()),
        phase.x,
        phase.y
    )
}

/// A reading as JSON: its kind and its points' parameters.
fn reading<T: geom_core::Decide>(
    got: &Result<Meetings<T>, MeetRefusal<T>>,
    (a, b): (&Span<'_, T>, &Span<'_, T>),
    mid: impl Fn(T) -> f64,
) -> String {
    match got {
        Ok(m) => {
            let mut pts = String::new();
            for (i, p) in m.points.iter().enumerate() {
                let sep = if i == 0 { "" } else { "," };
                write!(
                    pts,
                    "{sep}{{\"ta\":{:?},\"tb\":{:?}}}",
                    mid(a.at(p.on_a)),
                    mid(b.at(p.on_b))
                )
                .unwrap();
            }
            format!(
                "\"kind\":\"ok\",\"shared\":{},\"points\":[{pts}]",
                m.shared.is_some()
            )
        }
        Err(r) => format!(
            "\"kind\":\"refused\",\"fault\":{:?},\"points\":[]",
            format!("{:?}", r.fault)
        ),
    }
}

fn main() {
    let dir = std::env::var("CONIC_LADDER_OUT").expect("set CONIC_LADDER_OUT to a directory");
    let band = band();
    let eps = band.zero();
    let esc = band.escalate();
    let mut f = std::fs::File::create(format!("{dir}/ladder-eps{eps:e}.jsonl")).unwrap();
    let lift = |c: &Curve3<f64>| c.map_scalar(<Interval as geom_core::Real>::from_f64);
    let iv = <Interval as geom_core::Real>::from_f64;
    let imid = |x: Interval| 0.5 * (geom_core::Bounds::lo(x) + geom_core::Bounds::hi(x));
    for fat in [true, false] {
        for target in [1e4, 1e5, 1e6, 1e7, 1e8, 1e9] {
            let Some(e) = minted(target, fat) else {
                writeln!(
                    f,
                    "{{\"door\":\"mint\",\"fat\":{fat},\"target\":{target:?}}}"
                )
                .unwrap();
                continue;
            };
            let b = e.semi_axes().0.min(e.semi_axes().1);
            let mut cases = Vec::new();
            for gamma in [90.0_f64, 30.0, 5.0] {
                for (s, place) in [(0.3, "near"), (-3.0, "near"), (1000.0, "near")] {
                    let theta: f64 = s / b;
                    if theta.abs() < 0.3 {
                        let half = (10.0 / b).max(2.0 * theta.abs()).min(0.2);
                        cases.push((theta, half, gamma.to_radians(), place));
                    }
                }
                for theta in [0.5, 2.0] {
                    cases.push((theta, 0.1, gamma.to_radians(), "far"));
                }
            }
            let carrier = Curve3::Ellipse(e);
            let carrier_i = lift(&carrier);
            let only = std::env::var("CONIC_LADDER_ONLY").ok();
            for (case, &(theta, half, gamma, place)) in cases.iter().enumerate() {
                if only
                    .as_ref()
                    .is_some_and(|o| *o != format!("{fat},{target:?},{case}"))
                {
                    continue;
                }
                let t = e.point_at(theta);
                let tan = e.derivative_at(theta).normalize();
                let n = e.axis();
                let across = n.cross(tan).normalize();
                let span_e = (theta - half, theta + half);
                for shift in [0.0, 20.0 * eps, 1000.0 * eps] {
                    let head = format!(
                        "\"zero\":{eps:?},\"esc\":{esc:?},\"fat\":{fat},\"target\":{target:?},\"case\":{case},\"place\":\"{place}\",\
                         \"theta\":{theta:?},\"gamma\":{gamma:?},\"shift\":{shift:?},\
                         \"span\":[{:?},{:?}],{}",
                        span_e.0,
                        span_e.1,
                        stored(&e)
                    );
                    // The line door: in the ellipse's plane through `t`,
                    // at `gamma` to the arc, lifted off the plane by `shift`.
                    let d = (tan * gamma.cos() + across * gamma.sin()).normalize();
                    let lo = t - d * 2.0 + n * shift;
                    let line = Curve3::line(lo, d);
                    let (sa, sb) = (Span::new(&line, (0.0, 4.0)), Span::new(&carrier, span_e));
                    let got = meet(&sa, &sb, band);
                    let in_box = span_in_box(&line, (0.0, 4.0)) && span_in_box(&carrier, span_e);
                    let line_i = lift(&line);
                    let (ia, ib) = (
                        Span::new(&line_i, (iv(0.0), iv(4.0))),
                        Span::new(&carrier_i, (iv(span_e.0), iv(span_e.1))),
                    );
                    let got_i = meet(&ia, &ib, band);
                    for (lane, r) in [
                        ("f64", reading(&got, (&sa, &sb), |x| x)),
                        ("interval", reading(&got_i, (&ia, &ib), imid)),
                    ] {
                        writeln!(
                            f,
                            "{{\"door\":\"line\",\"lane\":\"{lane}\",{head},\"in_box\":{in_box},\"lo\":{},\"ld\":{},{r}}}",
                            p3(lo),
                            v3(d)
                        )
                        .unwrap();
                    }
                    // The circle door: a unit circle through `t` in a
                    // plane crossing the arc at `gamma`, moved `shift`
                    // off the ellipse.
                    let w = (across * 0.7_f64.cos() + n * 0.7_f64.sin()).normalize();
                    let normal = (tan * gamma.sin() + w * gamma.cos()).normalize();
                    let in_plane = normal.cross(w.cross(tan)).normalize();
                    // Apart along the pair's common normal at `t`.
                    let common = tan.cross(normal.cross(in_plane)).normalize();
                    let centre = t - in_plane + common * shift;
                    let circle = Curve3::circle(centre, normal, 1.0, in_plane);
                    let (sa, sb) = (Span::new(&carrier, span_e), Span::new(&circle, (-1.0, 1.0)));
                    let got = meet(&sa, &sb, band);
                    let in_box = span_in_box(&circle, (-1.0, 1.0)) && span_in_box(&carrier, span_e);
                    let circle_i = lift(&circle);
                    let (ia, ib) = (
                        Span::new(&carrier_i, (iv(span_e.0), iv(span_e.1))),
                        Span::new(&circle_i, (iv(-1.0), iv(1.0))),
                    );
                    let got_i = meet(&ia, &ib, band);
                    for (lane, r) in [
                        ("f64", reading(&got, (&sa, &sb), |x| x)),
                        ("interval", reading(&got_i, (&ia, &ib), imid)),
                    ] {
                        writeln!(
                            f,
                            "{{\"door\":\"circle\",\"lane\":\"{lane}\",{head},\"in_box\":{in_box},\"c\":{},\"m\":{},\"cu\":{},{r}}}",
                            p3(centre),
                            v3(normal),
                            v3(in_plane)
                        )
                        .unwrap();
                    }
                }
            }
        }
    }
}
