//! SCRATCH (oneround/conic-suspects-probe, never merges). Task 1: the
//! suspect readers of an arc-held ellipse's derived centre, run through
//! public doors (`curve_pair::meet`, the plane door, `Curve3::on_carrier`).
//! Dropped into `crates/geom-brep/examples/` of a clean checkout of main:
//! `CAD_TOLERANCE_EPS=1e-12 cargo run -p geom-brep --example suspects_t1`.
//! One JSON line per run; every f64 in shortest round-trip form, so the
//! oracle (`oracle_t1.py`) reads the stored numbers exactly.
#![allow(
    clippy::all,
    clippy::pedantic,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic
)]
use geom::{Conic, Curve3, CurveDatum, DatumValue, Ellipse, Frame};
use geom_brep::curve_face::plane_crossing::plane_crossing_lane;
use geom_brep::curve_pair::{Span, meet};
use geom_core::{Affine3, Band, Point3, Tol, Vec3};

fn p3(p: Point3<f64>) -> String {
    format!("[{:?},{:?},{:?}]", p.x, p.y, p.z)
}
fn v3(p: Vec3<f64>) -> String {
    format!("[{:?},{:?},{:?}]", p.x, p.y, p.z)
}
fn trim(e: &impl core::fmt::Debug, n: usize) -> String {
    format!("{e:?}")
        .chars()
        .filter(|c| !c.is_whitespace())
        .take(n)
        .collect()
}

/// A curve as its stored numbers.
fn stored(c: &Curve3<f64>) -> String {
    match c {
        Curve3::Line(l) => format!(
            "{{\"k\":\"line\",\"o\":{},\"d\":{}}}",
            p3(l.origin),
            v3(l.dir.get())
        ),
        Curve3::Circle(k) => {
            let f = k.frame();
            format!(
                "{{\"k\":\"conic\",\"held\":\"centre\",\"S\":{},\"axis\":{},\"u\":{},\"a\":{:?},\"b\":{:?},\"phase\":[0.0,0.0]}}",
                p3(f.origin()),
                v3(f.axis()),
                v3(f.u_ref()),
                k.radius(),
                k.radius()
            )
        }
        Curve3::Ellipse(e) => {
            let pt = |d| match c.data().named(d) {
                Some(DatumValue::Point(p)) => Some(p),
                _ => None,
            };
            let (held, s, phase) = match (pt(CurveDatum::Anchor), pt(CurveDatum::Center)) {
                (Some(p), _) => (
                    "arc",
                    p,
                    match c.data().named(CurveDatum::Phase) {
                        Some(DatumValue::Direction(d)) => (d.x, d.y),
                        _ => panic!("phase"),
                    },
                ),
                (_, Some(p)) => ("centre", p, (0.0, 0.0)),
                _ => panic!("datum"),
            };
            let f = e.frame();
            let (a, b) = e.semi_axes();
            format!(
                "{{\"k\":\"conic\",\"held\":\"{held}\",\"S\":{},\"axis\":{},\"u\":{},\"a\":{a:?},\"b\":{b:?},\"phase\":[{:?},{:?}]}}",
                p3(s),
                v3(f.axis()),
                v3(f.u_ref()),
                phase.0,
                phase.1
            )
        }
        _ => panic!("kind"),
    }
}

struct Ctx {
    band: Band,
    eps: f64,
    a: f64,
    shape: &'static str,
    s: f64,
}

fn head(c: &Ctx, fam: &str, var: &str) -> String {
    format!(
        "\"eps\":{:?},\"a\":{:?},\"shape\":\"{}\",\"s\":{:?},\"fam\":\"{fam}\",\"var\":\"{var}\"",
        c.eps, c.a, c.shape, c.s
    )
}

fn run_meet(
    c: &Ctx,
    fam: &str,
    var: &str,
    ca: &Curve3<f64>,
    sa: (f64, f64),
    cb: &Curve3<f64>,
    sb: (f64, f64),
    hint: (f64, f64),
) {
    let (span_a, span_b) = (Span::new(ca, sa), Span::new(cb, sb));
    // the kernel's own decision log (a public bracket): every decided row and its sign, in order
    let bracket = geom_core::k_stats::Bracket::open();
    let got = meet(&span_a, &span_b, c.band);
    let rec = bracket.finish();
    let mut log: Vec<(String, usize)> = Vec::new();
    for v in &rec.verdicts {
        let k = format!(
            "{}:{}",
            v.predicate,
            match v.sign {
                geom_core::Sign::Positive => "+",
                geom_core::Sign::Negative => "-",
                geom_core::Sign::Zero => "0",
            }
        );
        match log.last_mut() {
            Some((last, n)) if *last == k => *n += 1,
            _ => log.push((k, 1)),
        }
    }
    let skip = log.len().saturating_sub(24);
    let log = log
        .iter()
        .skip(skip)
        .map(|(k, n)| format!("\"{k}x{n}\""))
        .collect::<Vec<_>>()
        .join(",");
    let esc = rec
        .escalations
        .iter()
        .map(|e| format!("{:?}", e.predicate()))
        .collect::<Vec<_>>()
        .join(",");
    let pts = |v: &[geom_brep::curve_pair::Meeting<f64>]| {
        v.iter()
            .map(|m| {
                format!(
                    "{{\"p\":{},\"ta\":{:?},\"tb\":{:?}}}",
                    p3(m.point),
                    span_a.at(m.on_a),
                    span_b.at(m.on_b)
                )
            })
            .collect::<Vec<_>>()
            .join(",")
    };
    let res = match &got {
        Ok(m) => format!(
            "\"kind\":\"ok\",\"shared\":{},\"points\":[{}]",
            m.shared.is_some(),
            pts(&m.points)
        ),
        Err(r) => format!(
            "\"kind\":\"refused\",\"why\":{:?},\"shared\":{},\"points\":[{}]",
            trim(&r.fault, 220),
            r.shared.is_some(),
            pts(&r.found)
        ),
    };
    println!(
        "{{\"door\":\"meet\",{},\"A\":{},\"sa\":[{:?},{:?}],\"B\":{},\"sb\":[{:?},{:?}],\"hint\":[{:?},{:?}],{res},\"log\":[{log}],\"esc\":[{esc}]}}",
        head(c, fam, var),
        stored(ca),
        sa.0,
        sa.1,
        stored(cb),
        sb.0,
        sb.1,
        hint.0,
        hint.1
    );
}

fn main() {
    let band = Band::linear(Tol::witness()).unwrap();
    let eps = band.zero();
    let anchor = Point3::new(1.3, -0.7, 0.4);
    let axis = Vec3::new(0.5522240050794545, 0.3110026418424273, 0.7735153553621528);
    let toward = Vec3::new(0.6816514125372342, -0.702621773621506, -0.20414209516648582);
    for a in [1e2_f64, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8, 1e9] {
        // fat: the earlier lane's shape. thin: the shape a body's far section has (b ~ 1.04 sqrt(a), task 3).
        for (shape, b) in [("fat", 0.3 * a), ("thin", 1.04 * a.sqrt())] {
            let e = Ellipse::anchored(anchor, axis, toward, a, b);
            let ce = Curve3::Ellipse(e);
            let n = e.frame().axis();
            let u = e.frame().u_ref();
            for s in [0.0, 0.3, -3.0] {
                let c = Ctx {
                    band,
                    eps,
                    a,
                    shape,
                    s,
                };
                let theta = s / b;
                let p = e.point_at(theta);
                let tan = e.derivative_at(theta).normalize();
                let d = tan.cross(n).normalize(); // in the plane, square to the arc, pointing OUT of the ellipse (checked by the oracle)
                let half = 10.0 / b;
                let se = (theta - half, theta + half);
                // ---- door O: on_carrier
                for (dir_name, dir) in [("n", n), ("d", d)] {
                    for k in [0.0, 0.5, 5.0, 30.0] {
                        let q = p + dir * (k * eps);
                        let got = ce.on_carrier(q, theta, band);
                        let kind = match got {
                            Ok(geom::OnCarrier::On { .. }) => "on".to_owned(),
                            Ok(geom::OnCarrier::Off) => "off".to_owned(),
                            Err(r) => format!("refused:{}", trim(&r, 120)),
                        };
                        println!(
                            "{{\"door\":\"on_carrier\",{},\"E\":{},\"q\":{},\"near\":{theta:?},\"kind\":{kind:?}}}",
                            head(&c, "O", &format!("{dir_name}{k}")),
                            stored(&ce),
                            p3(q)
                        );
                    }
                }
                // ---- door P: the plane door, planes parallel to the ellipse's own
                for k in [0.0, 5.0, 30.0] {
                    let o = e.point_at(0.0) + n * (k * eps);
                    let got = plane_crossing_lane(&ce, se.0, se.1, o, n, band);
                    println!(
                        "{{\"door\":\"plane\",{},\"E\":{},\"span\":[{:?},{:?}],\"o\":{},\"n\":{},\"kind\":{:?}}}",
                        head(&c, "P", &format!("own+{k}")),
                        stored(&ce),
                        se.0,
                        se.1,
                        p3(o),
                        v3(n),
                        trim(&got, 60)
                    );
                }
                // ---- S: `OnSurface::under` (row 13): the edge's plane against a line of a plane chart
                {
                    use geom_brep::curve_pair::on_surface::{
                        Charted, CoSurface, OnSurface, SurfaceCurve,
                    };
                    let key = geom_brep::SurfaceKey::default();
                    let edge = OnSurface::edge(key, &ce, se);
                    let g = 30.0_f64.to_radians();
                    let through = (d * g.cos() + n * g.sin()).normalize();
                    let cases: [(&str, Point3<f64>, Vec3<f64>, Vec3<f64>); 4] = [
                        ("in", p, n, d),                              // a line of the ellipse's own plane
                        ("lift5", p + n * (5.0 * eps), n, d), // of a parallel plane, in the gap
                        ("lift30", p + n * (30.0 * eps), n, d), // of a parallel plane 30 eps up
                        ("cross30", p - through * 2.0, tan, through), // a line piercing the plane at p, 30 degrees
                    ];
                    for (name, origin, normal, along) in cases {
                        let chart = Charted::held(
                            key,
                            geom::Surface::Plane {
                                frame: Frame::new(origin, normal, along),
                            },
                        );
                        let curve =
                            SurfaceCurve::u_line(&chart, geom_core::Point2::new(0.0, 0.0)).unwrap();
                        let carrier = curve.carrier();
                        let got = edge.under(&curve, (0.0, 4.0), false, band);
                        let kind = match &got {
                            Ok(CoSurface::InPlane) => "\"kind\":\"inplane\",\"pts\":[]".to_owned(),
                            Ok(CoSurface::Touch) => "\"kind\":\"touch\",\"pts\":[]".to_owned(),
                            Ok(CoSurface::Roots(m)) => format!(
                                "\"kind\":\"roots\",\"pts\":[{}]",
                                m.iter().map(|m| p3(m.point)).collect::<Vec<_>>().join(",")
                            ),
                            Err(r) => format!(
                                "\"kind\":\"refused\",\"why\":{:?},\"pts\":[]",
                                trim(r, 160)
                            ),
                        };
                        println!(
                            "{{\"door\":\"under\",{},\"E\":{},\"L\":{},\"span\":[0.0,4.0],{kind}}}",
                            head(&c, "S", name),
                            stored(&ce),
                            stored(&carrier)
                        );
                    }
                }
                // ---- A: a line in the plane, square to the arc (row 1 ii; the measured miss, kept as the baseline)
                let line = Curve3::line(p - d * 2.0, geom_core::UnitVec3::checked(d));
                run_meet(&c, "A", "through", &line, (0.0, 4.0), &ce, se, (2.0, theta));
                // ending short of the arc from inside / starting past it outside by k eps
                for k in [30.0, 300.0] {
                    run_meet(
                        &c,
                        "A",
                        &format!("short{k}"),
                        &line,
                        (0.0, 2.0 - k * eps),
                        &ce,
                        se,
                        (2.0 - k * eps, theta),
                    );
                }
                // ---- C: a line piercing the plane through the arc (row 1 iii, the crossing arm)
                for gamma in [90.0_f64, 30.0, 5.0, 0.5] {
                    let g = gamma.to_radians();
                    let dir = (d * g.cos() + n * g.sin()).normalize();
                    let through = Curve3::line(p - dir * 2.0, geom_core::UnitVec3::checked(dir));
                    run_meet(
                        &c,
                        "C",
                        &format!("g{gamma}"),
                        &through,
                        (0.0, 4.0),
                        &ce,
                        se,
                        (2.0, theta),
                    );
                    // moved off the arc, along d, so that the line passes 30 eps from it
                    let off = Curve3::line(p + d * (30.0 * eps / g.sin()) - dir * 2.0, geom_core::UnitVec3::checked(dir));
                    run_meet(
                        &c,
                        "C",
                        &format!("g{gamma}off30"),
                        &off,
                        (0.0, 4.0),
                        &ce,
                        se,
                        (2.0, theta),
                    );
                }
                // ---- D: a unit circle in the ellipse's plane (row 3)
                let circ = |centre: Point3<f64>, r: f64| Curve3::circle(centre, n, r, -tan);
                run_meet(
                    &c,
                    "D",
                    "cross",
                    &circ(p + tan * 0.6, 1.0),
                    (-1.0, 1.0),
                    &ce,
                    se,
                    (0.0, theta - 0.4 / b),
                );
                run_meet(
                    &c,
                    "D",
                    "cross-rev",
                    &ce,
                    se,
                    &circ(p + tan * 0.6, 1.0),
                    (-1.0, 1.0),
                    (theta - 0.4 / b, 0.0),
                );
                let touch = |k: f64| Curve3::circle(p + d * (1.0 + k * eps), n, 1.0, -d);
                for k in [-30.0, 30.0] {
                    run_meet(
                        &c,
                        "D",
                        &format!("clear{k}"),
                        &touch(k),
                        (-1.0, 1.0),
                        &ce,
                        se,
                        (0.0, theta),
                    );
                }
                for k in [30.0] {
                    run_meet(
                        &c,
                        "D",
                        &format!("lift{k}"),
                        &Curve3::circle(p + tan * 0.6 + n * (k * eps), n, 1.0, -tan),
                        (-1.0, 1.0),
                        &ce,
                        se,
                        (0.0, theta - 0.4 / b),
                    );
                }
                // ---- E: a unit circle across the plane, through the arc (rows 4, 5, 7; the measured miss, both orders)
                for gamma in [90.0_f64, 30.0, 5.0] {
                    let g = gamma.to_radians();
                    let w = (d * 0.7_f64.cos() + n * 0.7_f64.sin()).normalize();
                    let normal = (tan * g.sin() + w * g.cos()).normalize();
                    let e_dir = normal.cross(w.cross(tan)).normalize();
                    for (name, shift) in [("through", 0.0), ("off30", 30.0 * eps)] {
                        // shifted along the in-plane normal d (out of the ellipse) the circle's own plane still crosses, the circle misses
                        let centre = p - e_dir + d * shift * 1.5;
                        let k = Curve3::circle(centre, normal, 1.0, e_dir);
                        run_meet(
                            &c,
                            "E",
                            &format!("g{gamma}{name}"),
                            &ce,
                            se,
                            &k,
                            (-1.0, 1.0),
                            (theta, 0.0),
                        );
                        run_meet(
                            &c,
                            "E",
                            &format!("g{gamma}{name}-rev"),
                            &k,
                            (-1.0, 1.0),
                            &ce,
                            se,
                            (0.0, theta),
                        );
                    }
                }
                // ---- F: a small centre-held ellipse in the plane (row 6, `coplanar_ellipses`)
                let small = |centre: Point3<f64>, along: Vec3<f64>| {
                    Curve3::Ellipse(Ellipse::new(Frame::new(centre, n, along), 2.0, 1.0))
                };
                run_meet(
                    &c,
                    "F",
                    "cross",
                    &small(p + tan * 1.2, -tan),
                    (-1.0, 1.0),
                    &ce,
                    se,
                    (0.0, theta - 0.8 / b),
                );
                run_meet(
                    &c,
                    "F",
                    "cross-rev",
                    &ce,
                    se,
                    &small(p + tan * 1.2, -tan),
                    (-1.0, 1.0),
                    (theta - 0.8 / b, 0.0),
                );
                for k in [-30.0, 30.0] {
                    // its major vertex k eps outside (+) or inside (-) the arc
                    run_meet(
                        &c,
                        "F",
                        &format!("clear{k}"),
                        &small(p + d * (2.0 + k * eps), -d),
                        (-1.0, 1.0),
                        &ce,
                        se,
                        (0.0, theta),
                    );
                    run_meet(
                        &c,
                        "F",
                        &format!("clear{k}-rev"),
                        &ce,
                        se,
                        &small(p + d * (2.0 + k * eps), -d),
                        (-1.0, 1.0),
                        (theta, 0.0),
                    );
                }
                // ---- G: two arc-held ellipses of one size through one point
                let turned = |about: Vec3<f64>| {
                    let m = Affine3::rotation_about_axis(p, about, 0.7);
                    Curve3::Ellipse(Ellipse::anchored(
                        m.transform_point(anchor),
                        m.transform_vec(axis),
                        m.transform_vec(toward),
                        a,
                        b,
                    ))
                };
                run_meet(&c, "G", "coplanar", &ce, se, &turned(n), se, (theta, theta));
                run_meet(&c, "G", "across", &ce, se, &turned(d), se, (theta, theta));
            }
            // ---- B: a line along the major axis at the anchor vertex (row 1 i, the GAP row)
            let c = Ctx {
                band,
                eps,
                a,
                shape,
                s: 0.0,
            };
            let half = 10.0 / b;
            for k in [-300.0, -30.0, -2.0, 0.0, 30.0, 300.0] {
                // the line starts k eps outside (+) / inside (-) the vertex and runs outward
                let line = Curve3::line(anchor + u * (k * eps), geom_core::UnitVec3::checked(u));
                run_meet(
                    &c,
                    "B",
                    &format!("out{k}"),
                    &line,
                    (0.0, 2.0),
                    &ce,
                    (-half, half),
                    (0.0, 0.0),
                );
            }
        }
    }
}
