//! **A shaft set in a bore, every bore × shaft wall pair declared
//! `Rest`, answers every op in closed form.**
//!
//! The interiors are disjoint, so `∪` is the two volumes summed, `∩` is
//! empty and each difference is its minuend whole. Two seam layouts:
//!
//! - **arc-split**: an extruded annulus whose bore is three 120° faces,
//!   against a three-arc shaft (three wall thirds) at an azimuth `a`
//!   from the bore's first split;
//! - **full-turn**: a rectangle revolved a full turn, whose bore is ONE
//!   face with a self-mated seam, against the same shaft turned onto
//!   the revolve's axis.
//!
//! The matrix varies the common radius, the collar's length, the
//! shaft's span (through both rims, flush at both, flush at one and
//! proud of the other, and blind: in through one rim, ending inside the
//! bore), the azimuth and the pose. `∩` and both differences run with
//! the declarations in their own operand order. The oracle is the closed
//! form, never the kernel: `π(R² − r²)L` for the collar, `πr²h` for the
//! shaft.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::three_arc;
use crate::mate2_common::{continuations, walls_at};
use core::f64::consts::{FRAC_PI_2, PI};
use geom_core::{Affine3, Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use sweep::test_support::{extruded, sketch_at};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{
    Body, BooleanDeclarations, BooleanOp, BooleanResult, ContactClass, FacePairDeclaration,
    mass_properties,
};

/// The collar's wall thickness, outside its bore.
const WALL: f64 = 1.0;

/// The collar's lower rim, along its axis.
const BASE: f64 = 1.0;

#[derive(Clone, Copy, Debug)]
enum Layout {
    ArcSplit,
    FullTurn,
}

/// One mate: the layout, the common radius, the collar's length, the
/// shaft's span `(start, length)` along the axis and its azimuth in
/// degrees.
#[derive(Clone, Copy, Debug)]
struct Mate {
    layout: Layout,
    r: f64,
    len: f64,
    span: (f64, f64),
    deg: f64,
}

impl Mate {
    fn collar_volume(self) -> f64 {
        let outer = self.r + WALL;
        PI * (outer * outer - self.r * self.r) * self.len
    }

    fn shaft_volume(self) -> f64 {
        PI * self.r * self.r * self.span.1
    }

    fn collar(self) -> Body<f64> {
        let (r, outer) = (self.r, self.r + WALL);
        match self.layout {
            Layout::ArcSplit => {
                let o = Point2::new(0.0, 0.0);
                extruded(
                    sketch_at(BASE),
                    vec![three_arc(o, outer, 0.0), three_arc(o, r, 0.0)],
                    self.len,
                    Tol::witness(),
                )
            }
            Layout::FullTurn => {
                let (y0, y1) = (BASE, BASE + self.len);
                let lp = ProfileLoop::polygon([
                    Point2::new(r, y0),
                    Point2::new(outer, y0),
                    Point2::new(outer, y1),
                    Point2::new(r, y1),
                ]);
                let vp = Profile::new(SketchPlane::xy(), vec![lp])
                    .validate(Tol::witness())
                    .unwrap();
                let axis = RevolveAxis {
                    origin: Point2::new(0.0, 0.0),
                    dir: Vec2::new(0.0, 1.0),
                };
                revolve(&vp, axis, Revolution::Full, Tol::witness())
                    .unwrap()
                    .body
            }
        }
    }

    fn shaft(self) -> Body<f64> {
        let (z0, h) = self.span;
        let peg = extruded(
            sketch_at(z0),
            vec![three_arc(Point2::new(0.0, 0.0), self.r, self.deg)],
            h,
            Tol::witness(),
        );
        match self.layout {
            Layout::ArcSplit => peg,
            // A quarter turn about `x` takes the peg's `z` axis to the
            // revolve's `y` axis.
            Layout::FullTurn => {
                let up = Affine3::rotation_about_axis(
                    Point3::new(0.0, 0.0, 0.0),
                    Vec3::new(1.0, 0.0, 0.0),
                    -FRAC_PI_2,
                );
                topo::transform_rigid(&peg, &up, Tol::witness()).unwrap()
            }
        }
    }
}

/// The shaft spans for a collar of length `len`, by name.
fn spans(len: f64) -> [(&'static str, (f64, f64)); 5] {
    [
        ("through", (BASE - 0.5, len + 1.0)),
        ("flush", (BASE, len)),
        ("proud above", (BASE, len + 0.5)),
        ("proud below", (BASE - 0.5, len + 0.5)),
        ("blind", (BASE + 0.5 * len, len)),
    ]
}

/// Every (bore wall × shaft wall) pair declared `Rest`, plus every
/// continuation the parts have.
fn decls(a: &Body<f64>, b: &Body<f64>, r: f64) -> BooleanDeclarations {
    let mut d = continuations(a, b);
    for &fa in &walls_at(a, r) {
        for &fb in &walls_at(b, r) {
            d.coincident_faces
                .push(FacePairDeclaration::new(fa, fb, ContactClass::Rest));
        }
    }
    d
}

fn volume(b: &Body<f64>) -> f64 {
    mass_properties(b, Tol::witness()).unwrap().volume
}

fn placed(b: &Body<f64>, pose: &Affine3<f64>) -> Body<f64> {
    topo::transform_rigid(b, pose, Tol::witness()).unwrap()
}

/// A body answer: its volume against the closed form `want`, relative
/// to `scale`; one shell; tier 3; the census against its own contacts.
fn holds(out: Result<BooleanResult<f64>, topo::BooleanError>, want: f64, scale: f64, tag: &str) {
    let tol = Tol::witness();
    let bb = match out {
        Ok(BooleanResult::Body(bb)) => bb,
        Ok(BooleanResult::Empty) => panic!("{tag}: empty, want volume {want}"),
        Err(e) => panic!("{tag}: refused {e:?}"),
    };
    let got = volume(&bb.body);
    assert!(
        (got - want).abs() <= 1e-12 * scale,
        "{tag}: volume {got} vs the closed form {want}"
    );
    assert_eq!(bb.body.shells().count(), 1, "{tag}: one shell");
    assert_eq!(
        topo::validate_geometric(&bb.body, tol),
        Ok(()),
        "{tag}: tier 3"
    );
    assert_eq!(
        topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol),
        Ok(()),
        "{tag}: the census"
    );
}

/// `∩` in both operand orders, and `c ∖ s`, `s ∖ c`, each with the
/// declarations in its own order.
fn intersect_and_differences(m: Mate, c: &Body<f64>, s: &Body<f64>, tag: &str) {
    let tol = Tol::witness();
    let scale = m.collar_volume().max(m.shaft_volume());
    let (cs, sc) = (decls(c, s, m.r), decls(s, c, m.r));
    let op = |op, a, b, d| topo::boolean_op_with(op, a, b, d, topo::SweepStrategy::Realized, tol);
    for (order, a, b, d) in [("collar ∩ shaft", c, s, &cs), ("shaft ∩ collar", s, c, &sc)] {
        match op(BooleanOp::Intersect, a, b, d) {
            Ok(BooleanResult::Empty) => {}
            Ok(BooleanResult::Body(bb)) => panic!(
                "{tag}: {order}: a body of volume {}, want empty",
                volume(&bb.body)
            ),
            Err(e) => panic!("{tag}: {order}: refused {e:?}"),
        }
    }
    holds(
        op(BooleanOp::Subtract, c, s, &cs),
        m.collar_volume(),
        scale,
        &format!("{tag}: collar ∖ shaft"),
    );
    holds(
        op(BooleanOp::Subtract, s, c, &sc),
        m.shaft_volume(),
        scale,
        &format!("{tag}: shaft ∖ collar"),
    );
}

/// `∪` in both operand orders.
fn unions(m: Mate, c: &Body<f64>, s: &Body<f64>, tag: &str) {
    let tol = Tol::witness();
    let want = m.collar_volume() + m.shaft_volume();
    for (order, a, b) in [("collar ∪ shaft", c, s), ("shaft ∪ collar", s, c)] {
        holds(
            topo::union_with(a, b, &decls(a, b, m.r), tol),
            want,
            want,
            &format!("{tag}: {order}"),
        );
    }
}

/// The two poses: the identity, and an oblique turn about an oblique
/// axis off the origin.
fn poses() -> [(&'static str, Affine3<f64>); 2] {
    [
        (
            "identity",
            Affine3::rotation_about_axis(Point3::origin(), Vec3::new(1.0, 0.0, 0.0), 0.0),
        ),
        (
            "1.1 about (1,2,3)",
            Affine3::rotation_about_axis(
                Point3::new(-0.2, 0.1, 0.4),
                Vec3::new(1.0, 2.0, 3.0).normalize(),
                1.1,
            ),
        ),
    ]
}

/// Every span, azimuth and pose of `layout` at radius `r` and collar
/// length `len`, through `check`.
fn each_mate(
    layout: Layout,
    r: f64,
    len: f64,
    spans_taken: &[&str],
    check: fn(Mate, &Body<f64>, &Body<f64>, &str),
) {
    for deg in [0.0, 60.0] {
        for (span_name, span) in spans(len) {
            if !spans_taken.contains(&span_name) {
                continue;
            }
            let m = Mate {
                layout,
                r,
                len,
                span,
                deg,
            };
            let (c0, s0) = (m.collar(), m.shaft());
            for (pose_name, pose) in poses() {
                let tag = format!(
                    "{layout:?}, r {r}, length {len}, {span_name}, azimuth {deg}, pose {pose_name}"
                );
                check(m, &placed(&c0, &pose), &placed(&s0, &pose), &tag);
            }
        }
    }
}

const EVERY_SPAN: [&str; 5] = ["through", "flush", "proud above", "proud below", "blind"];

#[test]
fn arc_split_bore_intersect_and_differences_are_the_closed_form() {
    for (r, len) in [(0.5, 1.0), (0.2, 2.5), (1.3, 0.75)] {
        each_mate(
            Layout::ArcSplit,
            r,
            len,
            &EVERY_SPAN,
            intersect_and_differences,
        );
    }
}

#[test]
fn full_turn_bore_intersect_and_differences_are_the_closed_form() {
    for (r, len) in [(0.5, 1.0), (0.2, 2.5), (1.3, 0.75)] {
        each_mate(
            Layout::FullTurn,
            r,
            len,
            &EVERY_SPAN,
            intersect_and_differences,
        );
    }
}

/// The spans a union builds at: a blind shaft is
/// `work/zip/blind-shaft-in-a-full-turn-bore-revisits-the-seam-vertex.md`.
const UNION_SPANS: [&str; 4] = ["through", "flush", "proud above", "proud below"];

#[test]
fn arc_split_bore_unions_are_the_closed_form() {
    for (r, len) in [(0.5, 1.0), (0.2, 2.5), (1.3, 0.75)] {
        each_mate(Layout::ArcSplit, r, len, &UNION_SPANS, unions);
    }
}

#[test]
fn full_turn_bore_unions_are_the_closed_form() {
    for (r, len) in [(0.5, 1.0), (0.2, 2.5), (1.3, 0.75)] {
        each_mate(Layout::FullTurn, r, len, &UNION_SPANS, unions);
    }
}
