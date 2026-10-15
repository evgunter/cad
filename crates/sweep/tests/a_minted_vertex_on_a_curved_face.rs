//! **A vertex a split mints on an edge lying on a curved face is read
//! against that face** (`reduce::LyingIn::Curved`).
//!
//! The line body is the cylinder `r = 1`, `z ∈ [0, 2]`, unioned
//! (cylinder first) with a piece: a wedge prism whose apex edge rests on
//! the wall's ruling at `(1, 0)`, `z ∈ [0.5, 1.5]`, opening outward over
//! `−80°..−40°`. The tool is a wedge prism with its apex edge on the same
//! ruling, `z ∈ [0.2, 1.8]`. The piece's apex edge lies on the wall, read
//! by its ends; the tool's faces split it there, and the vertex the split
//! mints has to be read against the wall too, or the two operand slots
//! disagree about it.
//!
//! - **60°..120°**: the tool straddles the wall.
//! - **40°..80°**: the tool rests outside it, beside the piece.
//!
//! Every op builds in either slot, at its closed form, tier 3 clean.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Point2, Tol};
use sweep::test_support::{finished, prism_at};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{AtRestBody, BooleanError, BooleanResult};

fn t() -> Tol {
    Tol::witness()
}

/// The wedges' side length from the apex.
const LEN: f64 = 0.5;

/// The cylinder `r = 1` about the z axis, `z ∈ [0, 2]`.
fn rod() -> AtRestBody<f64> {
    let lp = profile::circle(Point2::new(0.0, 0.0), 1.0, t()).unwrap();
    let p = profile::Profile::new(profile::SketchPlane::xy(), vec![lp.into()])
        .validate(t())
        .unwrap();
    let rod = extrude(
        &p,
        Extrusion::Distance {
            depth: 2.0,
            side: ExtrudeSide::Along,
        },
        t(),
    )
    .unwrap()
    .body;
    finished("the rod", rod, t())
}

/// The wedge prism with its apex edge on the ruling `(1, 0)`, its sides
/// [`LEN`] long at `a°` and `b°` from `+x`, over `z ∈ [z0, z0 + h]`.
fn wedge(a: f64, b: f64, z0: f64, h: f64) -> AtRestBody<f64> {
    let (a, b) = (a.to_radians(), b.to_radians());
    let v = |x: f64, y: f64| (Point2::new(x, y), 0.0);
    let p = prism_at(
        vec![
            v(1.0, 0.0),
            v(1.0 + LEN * a.cos(), LEN * a.sin()),
            v(1.0 + LEN * b.cos(), LEN * b.sin()),
        ],
        z0,
        h,
        t(),
    );
    finished("the wedge", p, t())
}

/// The area of the triangle `(1, 0)`, `(1, 0) + LEN·(cos a, sin a)`,
/// `(1, 0) + LEN·(cos b, sin b)` (`0 < a < b < π`) inside the unit disc,
/// by the midpoint rule across `y`: the integrand is continuous, so
/// 2·10⁵ panels hold it well inside the `1e-7` the rows assert.
fn area_in_disc(a: f64, b: f64) -> f64 {
    let (a, b) = (a.to_radians(), b.to_radians());
    let top = LEN * a.sin().min(b.sin());
    let n = 200_000;
    let h = top / n as f64;
    // Each side as `x(y)` from the apex.
    let (xa, xb) = (|y: f64| 1.0 + y / a.tan(), |y: f64| 1.0 + y / b.tan());
    (0..n)
        .map(|i| {
            let y = (i as f64 + 0.5) * h;
            let (lo, hi) = (xa(y).min(xb(y)), xa(y).max(xb(y)));
            (hi.min((1.0 - y * y).sqrt()) - lo).max(0.0)
        })
        .sum::<f64>()
        * h
}

type Op = fn(&AtRestBody<f64>, &AtRestBody<f64>, Tol) -> Result<BooleanResult<f64>, BooleanError>;

fn builds_at(what: &str, got: Result<BooleanResult<f64>, BooleanError>, volume: f64) {
    match got {
        Ok(BooleanResult::Body(r)) => {
            topo::validate_geometric(&r.body, t())
                .unwrap_or_else(|e| panic!("{what}: tier 3: {e:?}"));
            let v = topo::mass_properties(&r.body, t()).unwrap().volume;
            assert!(
                (v - volume).abs() < 1e-7,
                "{what}: volume {v}, closed form {volume}"
            );
        }
        Ok(BooleanResult::Empty) if volume == 0.0 => {}
        other => panic!("{what}: {other:?}, where it builds at {volume}"),
    }
}

/// **Every op answers alike in both slots** (module docs), at its
/// closed form: the cylinder `2π`, the piece's triangle over its height,
/// the tool's over its own, less the tool's part inside the cylinder.
#[test]
fn a_wedge_on_the_ruling_answers_every_op_in_both_slots() {
    let triangle = |a: f64, b: f64| 0.5 * LEN * LEN * (b - a).to_radians().sin();
    let (pa, pb) = (-80.0, -40.0);
    let line = match topo::union(&rod(), &wedge(pa, pb, 0.5, 1.0), t()).unwrap() {
        BooleanResult::Body(r) => r.body,
        other => panic!("the line body: {other:?}"),
    };
    let piece = triangle(pa, pb);
    for (ta, tb) in [(60.0, 120.0), (40.0, 80.0)] {
        let tool = wedge(ta, tb, 0.2, 1.6);
        let (whole, inside) = (1.6 * triangle(ta, tb), 1.6 * area_in_disc(ta, tb));
        let rows: [(&str, Op, f64, f64); 3] = [
            ("intersection", topo::intersect, inside, inside),
            (
                "subtraction",
                topo::subtract,
                2.0 * PI + piece - inside,
                whole - inside,
            ),
            (
                "union",
                topo::union,
                2.0 * PI + piece + whole - inside,
                2.0 * PI + piece + whole - inside,
            ),
        ];
        for (name, op, line_first, tool_first) in rows {
            let what = format!("{ta}°..{tb}° {name}");
            builds_at(
                &format!("{what}, line body first"),
                op(&line, &tool, t()),
                line_first,
            );
            builds_at(
                &format!("{what}, tool first"),
                op(&tool, &line, t()),
                tool_first,
            );
        }
    }
}
