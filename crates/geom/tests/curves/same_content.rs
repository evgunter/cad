//! **`Curve3::same_content` reads every value that states a carrier**:
//! for each kind, the carrier is its own content, and a twin with any
//! one stated value one ulp off is not. Two kinds are never one carrier,
//! and a value that carries more than its constant (a derivative) is
//! no answer, so not the same.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom::Curve3;
use geom_core::spline::KnotVector;
use geom_core::{Point3, Vec3};

fn up(x: f64) -> f64 {
    f64::next_up(x)
}

/// A point's or vector's three coordinates with coordinate `i` one ulp
/// off.
fn nudged(p: [f64; 3], i: usize) -> [f64; 3] {
    let mut q = p;
    q[i] = up(q[i]);
    q
}

fn pt(p: [f64; 3]) -> Point3<f64> {
    Point3::new(p[0], p[1], p[2])
}

fn v3(v: [f64; 3]) -> Vec3<f64> {
    Vec3::new(v[0], v[1], v[2])
}

const O: [f64; 3] = [0.25, -0.5, 1.0];
const Z: [f64; 3] = [0.0, 0.0, 1.0];
const X: [f64; 3] = [1.0, 0.0, 0.0];

/// The frame's nine values, each in turn one ulp off: `(which, centre,
/// axis, u_ref)`.
/// A frame twin: which value is off, then its centre, axis and `u_ref`.
type FrameTwin = (String, [f64; 3], [f64; 3], [f64; 3]);

/// A kind: its name, its carrier, and each named twin of it.
type Kind = (&'static str, Curve3<f64>, Vec<(String, Curve3<f64>)>);

fn frame_twins() -> Vec<FrameTwin> {
    let mut out = Vec::new();
    for i in 0..3 {
        out.push((format!("centre[{i}]"), nudged(O, i), Z, X));
        out.push((format!("axis[{i}]"), O, nudged(Z, i), X));
        out.push((format!("u_ref[{i}]"), O, Z, nudged(X, i)));
    }
    out
}

fn frame(c: [f64; 3], a: [f64; 3], u: [f64; 3]) -> geom::Frame<f64> {
    geom::Frame::new(pt(c), v3(a), v3(u))
}

fn line(o: [f64; 3], d: [f64; 3]) -> Curve3<f64> {
    Curve3::Line(geom::Line {
        origin: pt(o),
        dir: v3(d),
    })
}

fn circle(f: geom::Frame<f64>, r: f64) -> Curve3<f64> {
    Curve3::Circle(geom::Circle {
        frame: f,
        radius: r,
    })
}

fn ellipse(f: geom::Frame<f64>, su: f64, sv: f64) -> Curve3<f64> {
    Curve3::Ellipse(geom::Ellipse {
        frame: f,
        semi_u: su,
        semi_v: sv,
    })
}

fn spiric(c: [f64; 3], a: [f64; 3], u: [f64; 3], big: f64, small: f64, off: f64) -> Curve3<f64> {
    Curve3::Spiric {
        center: pt(c),
        axis: v3(a),
        u_ref: v3(u),
        major_radius: big,
        minor_radius: small,
        offset: off,
    }
}

const KNOTS: [f64; 7] = [0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0];
const NET: [[f64; 3]; 4] = [
    [0.0, 0.0, 0.0],
    [1.0, 1.0, 0.0],
    [2.0, 0.0, 0.5],
    [3.0, 1.0, 0.0],
];
const WEIGHTS: [f64; 4] = [1.0, 0.8, 1.25, 1.0];

fn net(knots: &[f64], control: &[[f64; 3]], weights: &[f64]) -> Curve3<f64> {
    Curve3::Nurbs(Arc::new(
        geom::NurbsCurve3::new(
            KnotVector::clamped(knots.to_vec(), 2).unwrap(),
            control.iter().map(|&p| pt(p)).collect(),
            weights.to_vec(),
        )
        .unwrap(),
    ))
}

/// Each kind's carrier, and every twin of it with one stated value one
/// ulp off.
fn kinds() -> Vec<Kind> {
    let mut line_twins = Vec::new();
    for i in 0..3 {
        line_twins.push((format!("origin[{i}]"), line(nudged(O, i), X)));
        line_twins.push((format!("dir[{i}]"), line(O, nudged(X, i))));
    }
    let frames = frame_twins();
    let base = frame(O, Z, X);
    let circle_twins = frames
        .iter()
        .map(|(w, c, a, u)| (w.clone(), circle(frame(*c, *a, *u), 1.5)))
        .chain([("radius".to_owned(), circle(base, up(1.5)))])
        .collect();
    let ellipse_twins = frames
        .iter()
        .map(|(w, c, a, u)| (w.clone(), ellipse(frame(*c, *a, *u), 2.0, 1.0)))
        .chain([
            ("semi_u".to_owned(), ellipse(base, up(2.0), 1.0)),
            ("semi_v".to_owned(), ellipse(base, 2.0, up(1.0))),
        ])
        .collect();
    let spiric_twins = frames
        .iter()
        .map(|(w, c, a, u)| (w.clone(), spiric(*c, *a, *u, 2.0, 0.5, 0.75)))
        .chain([
            (
                "major_radius".to_owned(),
                spiric(O, Z, X, up(2.0), 0.5, 0.75),
            ),
            (
                "minor_radius".to_owned(),
                spiric(O, Z, X, 2.0, up(0.5), 0.75),
            ),
            ("offset".to_owned(), spiric(O, Z, X, 2.0, 0.5, up(0.75))),
        ])
        .collect();
    let mut net_twins = Vec::new();
    // Each knot run moved as a whole, so the vector stays clamped.
    for (run, idx) in [("start", 0..3), ("interior", 3..4), ("end", 4..7)] {
        let mut k = KNOTS;
        for i in idx {
            k[i] = up(k[i]);
        }
        net_twins.push((format!("knots {run}"), net(&k, &NET, &WEIGHTS)));
    }
    for j in 0..NET.len() {
        for i in 0..3 {
            let mut c = NET;
            c[j] = nudged(c[j], i);
            net_twins.push((format!("control[{j}][{i}]"), net(&KNOTS, &c, &WEIGHTS)));
        }
        let mut w = WEIGHTS;
        w[j] = up(w[j]);
        net_twins.push((format!("weight[{j}]"), net(&KNOTS, &NET, &w)));
    }
    vec![
        ("line", line(O, X), line_twins),
        ("circle", circle(base, 1.5), circle_twins),
        ("ellipse", ellipse(base, 2.0, 1.0), ellipse_twins),
        ("spiric", spiric(O, Z, X, 2.0, 0.5, 0.75), spiric_twins),
        ("nurbs", net(&KNOTS, &NET, &WEIGHTS), net_twins),
    ]
}

#[test]
fn a_twin_one_ulp_off_in_any_stated_value_is_another_carrier() {
    let kinds = kinds();
    for (kind, carrier, twins) in &kinds {
        assert!(carrier.same_content(&carrier.clone()), "{kind}: itself");
        for (which, twin) in twins {
            assert!(
                !carrier.same_content(twin) && !twin.same_content(carrier),
                "{kind}: a twin with {which} one ulp off"
            );
        }
    }
    assert_eq!(
        kinds.iter().map(|(_, _, t)| t.len()).collect::<Vec<_>>(),
        [6, 10, 11, 12, 19],
        "one twin per stated value (a net's knots by run)"
    );
    for (a, ca, _) in &kinds {
        for (b, cb, _) in &kinds {
            if a != b {
                assert!(!ca.same_content(cb), "{a} is not {b}");
            }
        }
    }
}

#[test]
fn a_value_carrying_a_derivative_is_no_answer() {
    use geom_core::Dual64;
    let constant = |x: f64| Dual64::constant(x);
    let at = |p: [f64; 3]| Point3::new(constant(p[0]), constant(p[1]), constant(p[2]));
    let line = |o: Point3<Dual64>| {
        Curve3::Line(geom::Line {
            origin: o,
            dir: Vec3::new(constant(1.0), constant(0.0), constant(0.0)),
        })
    };
    let still = line(at(O));
    assert!(
        still.same_content(&still.clone()),
        "constants are one value"
    );
    let mut moving = at(O);
    moving.x = Dual64::new(O[0], 1.0);
    let moving = line(moving);
    assert!(
        !moving.same_content(&moving.clone()),
        "a value with a derivative is no plain constant, so no answer"
    );
}
