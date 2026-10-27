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
            let v = topo::test_support::pinned_volume("the measured body", &r.body, t()).volume;
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

/// The line body: the cylinder unioned (cylinder first) with the wedge
/// prism `wedge(a, b, z0, h)`.
fn line_body(a: f64, b: f64, z0: f64, h: f64) -> AtRestBody<f64> {
    match topo::union(&rod(), &wedge(a, b, z0, h), t()).unwrap() {
        BooleanResult::Body(r) => r.body,
        other => panic!("the line body: {other:?}"),
    }
}

/// **Two wedges sharing the wall's tangent-plane face refuse an untyped
/// join, in every op and order**, today
/// (`work/tang/two-wedges-sharing-a-tangent-plane-face-on-a-ruling-refuse-unpaired-loose-ends.md`).
/// The piece opens over −90°..−50° and the tool over −150°..−90°, both
/// over `z ∈ [0.2, 1.8]`, so their caps coincide; each has a side along
/// −90°, in the plane `x = 1`. Pins the refusal, so it flips when fixed.
#[test]
fn two_wedges_sharing_the_tangent_plane_face_refuse_unpaired_loose_ends() {
    let line = line_body(-90.0, -50.0, 0.2, 1.6);
    let tool = wedge(-150.0, -90.0, 0.2, 1.6);
    let ops: [(&str, Op); 3] = [
        ("intersection", topo::intersect),
        ("subtraction", topo::subtract),
        ("union", topo::union),
    ];
    for (name, op) in ops {
        for (order, got) in [
            ("line body first", op(&line, &tool, t())),
            ("tool first", op(&tool, &line, t())),
        ] {
            let Err(e) = got else {
                panic!("{name}, {order}: builds, where it refused")
            };
            assert_eq!(
                format!("{e:?}"),
                "Join(BooleanJoin(UnpairedLooseEnds { count: 4 }))",
                "{name}, {order}"
            );
        }
    }
}

/// **A difference leaving a zero-angle cusp along the ruling builds.**
/// Cell A: the piece `50°..90°` over `z ∈ [0.5, 1.5]` and the tool
/// `60°..120°` over `z ∈ [0.7, 1.2]`; cell B, its mirror in `y = 0`, both
/// over `z ∈ [0.2, 1.8]`. `tool ∖ line` keeps the tool's part outside the
/// cylinder and the piece: a sliver under the tool's far side between
/// the piece's face `x = 1` and the wall, meeting along the ruling at a
/// zero angle (a cusp edge, D1's wedge end, which tier 3 reads), and a
/// triangle beyond the piece's far side by the tool's `60°` side. It
/// builds at the closed form, tier 3 clean: no two of the result's
/// edges lie on the ruling, so the result gate reads no wedge there
/// (`work/tangtail/a-difference-leaving-a-cusp-along-a-wall-ruling-refuses-sector-straight.md`).
#[test]
fn a_difference_leaving_a_cusp_along_the_ruling_builds() {
    // The tool's far side stands `LEN · sin 60°` off the ruling's plane.
    let h = LEN * 60f64.to_radians().sin();
    // Between `x = 1` and the wall `x = √(1 − y²)`, up to `y = h`.
    let cusp = h - (h * (1.0 - h * h).sqrt() + h.asin()) / 2.0;
    // The triangle cut from the tool's far corner by the piece's far side.
    let at = |deg: f64| {
        let r = deg.to_radians();
        (1.0 + LEN * r.cos(), LEN * r.sin())
    };
    let meet = |(p, q): ((f64, f64), (f64, f64)), (r, u): ((f64, f64), (f64, f64))| {
        let d = (p.0 - q.0) * (r.1 - u.1) - (p.1 - q.1) * (r.0 - u.0);
        let k = ((p.0 - r.0) * (r.1 - u.1) - (p.1 - r.1) * (r.0 - u.0)) / d;
        (p.0 + k * (q.0 - p.0), p.1 + k * (q.1 - p.1))
    };
    let far_side = (at(50.0), at(90.0));
    let corner = at(60.0);
    let q1 = meet(far_side, ((0.0, h), (2.0, h)));
    let q2 = meet(far_side, ((1.0, 0.0), corner));
    let triangle =
        0.5 * ((q1.0 - corner.0) * (q2.1 - corner.1) - (q2.0 - corner.0) * (q1.1 - corner.1)).abs();
    let cells = [
        ("A", (50.0, 90.0, 0.5, 1.0), (60.0, 120.0, 0.7, 0.5)),
        ("B", (-90.0, -50.0, 0.2, 1.6), (-120.0, -60.0, 0.2, 1.6)),
    ];
    for (cell, (pa, pb, pz, ph), (ta, tb, tz, th)) in cells {
        let line = line_body(pa, pb, pz, ph);
        let tool = wedge(ta, tb, tz, th);
        let BooleanResult::Body(r) =
            topo::subtract(&tool, &line, t()).unwrap_or_else(|e| panic!("cell {cell}: {e:?}"))
        else {
            panic!("cell {cell}: empty");
        };
        let v = topo::test_support::pinned_volume("the measured body", &r.body, t()).volume;
        let want = (cusp + triangle) * th;
        assert!(
            (v - want).abs() < 1e-9,
            "cell {cell}: volume {v}, closed form {want}"
        );
        assert_eq!(
            topo::validate_geometric(&r.body, t()),
            Ok(()),
            "cell {cell}: tier 3"
        );
    }
}

/// **The same cusp against a box tangent to the wall builds.** The line
/// body is the rod with the box `x ∈ [1, 1.5]`, `y ∈ [0, 0.5]`,
/// `z ∈ [0.5, 1.5]`, whose face `x = 1` is the wall's tangent plane
/// along the ruling; the tool is `60°..120°` over `z ∈ [0.2, 1.8]`.
/// `tool ∖ line` keeps the tool outside the disc over its height, less
/// its part at `x ≥ 1` (all of it inside the box, and outside the disc)
/// over the box's: that part runs from the ruling to the `60°` side, up
/// to the tool's far side at `y = LEN·sin 60°`.
#[test]
fn a_difference_leaving_a_cusp_beside_a_tangent_box_builds() {
    let top = LEN * 60f64.to_radians().sin();
    let outside = 0.5 * LEN * LEN * 60f64.to_radians().sin() - area_in_disc(60.0, 120.0);
    let in_box = top * top / (2.0 * 60f64.to_radians().tan());
    // The pose and its mirror in `y = 0`, each box wound counter-clockwise.
    for (pose, (y0, y1), (ta, tb)) in [
        ("above", (0.0, 0.5), (60.0, 120.0)),
        ("mirror", (-0.5, 0.0), (-120.0, -60.0)),
    ] {
        let v = |x: f64, y: f64| (Point2::new(x, y), 0.0);
        let block = prism_at(
            vec![v(1.0, y0), v(1.5, y0), v(1.5, y1), v(1.0, y1)],
            0.5,
            1.0,
            t(),
        );
        let line = match topo::union(&rod(), &finished("the box", block, t()), t()).unwrap() {
            BooleanResult::Body(r) => r.body,
            other => panic!("{pose}: the line body: {other:?}"),
        };
        builds_at(
            &format!("{pose}: tool ∖ line"),
            topo::subtract(&wedge(ta, tb, 0.2, 1.6), &line, t()),
            outside * 1.6 - in_box * 1.0,
        );
    }
}
