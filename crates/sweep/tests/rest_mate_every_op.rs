//! **A part seated in another, every pair of mating faces declared
//! `Rest`, answers every op in closed form.**
//!
//! The interiors are disjoint, so `∪` is the two volumes summed, `∩` is
//! empty and each difference is its minuend whole. A shaft set in a
//! bore comes in two seam layouts:
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
//!
//! A ball filling a spherical cavity exactly has no rim at all: the
//! contact is the cavity's whole wall. Its oracle is `4πr³/3` per ball.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::mate2_common::{
    collar_of, declare_rest, full_turn_collar, onto_y, peg_of, spheres_at, wall_decls_at,
};
use core::f64::consts::PI;
use geom_core::{Affine3, Point3, Tol, Vec3};
use sweep::test_support::ball_poled_y;
use topo::{Body, BooleanDeclarations, BooleanOp, BooleanResult, mass_properties};

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
        let outer = self.r + WALL;
        match self.layout {
            Layout::ArcSplit => collar_of(self.r, outer, 0.0, BASE, self.len),
            Layout::FullTurn => full_turn_collar(self.r, outer, (BASE, BASE + self.len)),
        }
    }

    fn shaft(self) -> Body<f64> {
        let (z0, h) = self.span;
        let peg = peg_of(self.r, self.deg, z0, h);
        match self.layout {
            Layout::ArcSplit => peg,
            Layout::FullTurn => onto_y(&peg),
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

fn volume(b: &Body<f64>) -> f64 {
    mass_properties(b, Tol::witness()).unwrap().volume
}

fn placed(b: &Body<f64>, pose: &Affine3<f64>) -> Body<f64> {
    topo::transform_rigid(b, pose, Tol::witness()).unwrap()
}

/// A body answer: its volume against the closed form `want`, relative
/// to `scale`; `shells` shells; tier 3; the census against its own
/// contacts.
fn holds(
    out: Result<BooleanResult<f64>, topo::BooleanError>,
    (want, shells): (f64, usize),
    scale: f64,
    tag: &str,
) {
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
    assert_eq!(bb.body.shells().count(), shells, "{tag}: shells");
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
    let (cs, sc) = (wall_decls_at(c, s, m.r), wall_decls_at(s, c, m.r));
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
        (m.collar_volume(), 1),
        scale,
        &format!("{tag}: collar ∖ shaft"),
    );
    holds(
        op(BooleanOp::Subtract, s, c, &sc),
        (m.shaft_volume(), 1),
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
            topo::union_with(a, b, &wall_decls_at(a, b, m.r), tol),
            (want, 1),
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

#[test]
fn arc_split_bore_unions_are_the_closed_form() {
    for (r, len) in [(0.5, 1.0), (0.2, 2.5), (1.3, 0.75)] {
        each_mate(Layout::ArcSplit, r, len, &EVERY_SPAN, unions);
    }
}

/// A blind shaft in a full-turn bore does not union:
/// `work/zip/blind-shaft-in-a-full-turn-bore-revisits-the-seam-vertex.md`.
#[test]
fn full_turn_bore_unions_are_the_closed_form() {
    for (r, len) in [(0.5, 1.0), (0.2, 2.5), (1.3, 0.75)] {
        each_mate(
            Layout::FullTurn,
            r,
            len,
            &["through", "flush", "proud above", "proud below"],
            unions,
        );
    }
}

/// A ball of radius `r` about the origin, poles on `y`.
fn ball(r: f64) -> Body<f64> {
    ball_poled_y(r, Vec3::new(0.0, 0.0, 0.0), Tol::witness())
}

/// The ball of radius `outer` with a ball of radius `r` taken out of
/// its middle: one outer shell and one cavity.
fn hollow(r: f64, outer: f64) -> Body<f64> {
    match topo::subtract(&ball(outer), &ball(r), Tol::witness()) {
        Ok(BooleanResult::Body(bb)) => bb.body,
        other => panic!("the hollow ball builds: {other:?}"),
    }
}

/// Every pair of sphere faces of radius `r` across `a` and `b`
/// declared `Rest`.
fn sphere_decls(a: &Body<f64>, b: &Body<f64>, r: f64) -> BooleanDeclarations {
    let mut d = BooleanDeclarations::none();
    declare_rest(&mut d, &spheres_at(a, r), &spheres_at(b, r));
    assert!(!d.coincident_faces.is_empty(), "a declared sphere pair");
    d
}

#[test]
fn a_ball_filling_a_spherical_cavity_answers_every_op_in_closed_form() {
    let tol = Tol::witness();
    let ball_volume = |r: f64| 4.0 / 3.0 * PI * r * r * r;
    for (r, outer) in [(0.5, 1.0), (0.2, 1.5), (1.3, 2.0)] {
        for (pose_name, pose) in poses() {
            let tag = format!("ball {r} in a cavity of a ball {outer}, pose {pose_name}");
            let (h, b) = (placed(&hollow(r, outer), &pose), placed(&ball(r), &pose));
            let (hb, bh) = (sphere_decls(&h, &b, r), sphere_decls(&b, &h, r));
            let scale = ball_volume(outer);
            for (order, x, y, d) in [("hollow", &h, &b, &hb), ("ball", &b, &h, &bh)] {
                holds(
                    topo::union_with(x, y, d, tol),
                    (ball_volume(outer), 1),
                    scale,
                    &format!("{tag}: {order} first, ∪"),
                );
                assert!(
                    matches!(topo::intersect_with(x, y, d, tol), Ok(BooleanResult::Empty)),
                    "{tag}: {order} first, ∩ is empty"
                );
            }
            holds(
                topo::subtract_with(&h, &b, &hb, tol),
                (ball_volume(outer) - ball_volume(r), 2),
                scale,
                &format!("{tag}: hollow ∖ ball"),
            );
            holds(
                topo::subtract_with(&b, &h, &bh, tol),
                (ball_volume(r), 1),
                scale,
                &format!("{tag}: ball ∖ hollow"),
            );
        }
    }
}

/// **A cavity with one of its sphere pairs undeclared keeps the
/// refusal.** The hollow's cavity and the ball each carry two sphere
/// faces, face `i` of each on the same side of the same seam. With a
/// pair `(i, j)`, `i ≠ j`, left undeclared, every face still has a
/// declared partner, the crossing layer passes, and the extent scan's
/// sphere pair refuses `SpheresMeet`: it asks every face of a carrier,
/// each on its exact pair, and a declaration on a face speaks for that
/// face against its own partner only. (Leaving out a pair `(i, i)`
/// refuses earlier, at the crossing layer along the seam the two faces
/// share.)
#[test]
fn a_cavity_with_one_sphere_pair_undeclared_keeps_the_sphere_refusal() {
    let tol = Tol::witness();
    let (h, b) = (hollow(0.5, 1.0), ball(0.5));
    let (fh, fb) = (spheres_at(&h, 0.5), spheres_at(&b, 0.5));
    assert_eq!((fh.len(), fb.len()), (2, 2), "two sphere faces each");
    for (oh, ob) in [(0, 1), (1, 0)] {
        let (mut hb, mut bh) = (BooleanDeclarations::none(), BooleanDeclarations::none());
        for (i, &x) in fh.iter().enumerate() {
            for (j, &y) in fb.iter().enumerate() {
                if (i, j) != (oh, ob) {
                    declare_rest(&mut hb, &[x], &[y]);
                    declare_rest(&mut bh, &[y], &[x]);
                }
            }
        }
        for (op, out) in [
            ("hollow ∪ ball", topo::union_with(&h, &b, &hb, tol)),
            ("hollow ∩ ball", topo::intersect_with(&h, &b, &hb, tol)),
            ("hollow ∖ ball", topo::subtract_with(&h, &b, &hb, tol)),
            ("ball ∖ hollow", topo::subtract_with(&b, &h, &bh, tol)),
        ] {
            assert!(
                matches!(out, Err(topo::BooleanError::SpheresMeet { .. })),
                "pair ({oh}, {ob}) undeclared: {op}: {:?}",
                out.as_ref().err()
            );
        }
    }
}

/// **A second shell beside the mate is classified as any shell is.**
/// The shaft through the arc-split collar, declared as everywhere here,
/// with a pebble (a ball of radius 0.2) buried in the collar's wall
/// and joined to the shaft as a second solid of one operand: `∩` is
/// the pebble, the collar minus it keeps it as a cavity, the operand
/// minus the collar is the shaft, and the union is collar and shaft.
#[test]
fn a_pebble_buried_in_the_collar_beside_the_mate_is_its_own_shell() {
    let tol = Tol::witness();
    let pebble_volume = 4.0 / 3.0 * PI * 0.2 * 0.2 * 0.2;
    let (collar_volume, shaft_volume) = (2.0 * PI, 0.5 * PI);
    for (pose_name, pose) in poses() {
        let tag = format!("pose {pose_name}");
        let c = placed(&collar_of(0.5, 1.5, 0.0, 1.0, 1.0), &pose);
        let pebble = topo::transform_rigid(
            &ball(0.2),
            &Affine3::translation(Vec3::new(1.0, 0.0, 1.5)),
            tol,
        )
        .unwrap();
        let two = match topo::union(&peg_of(0.5, 0.0, 0.5, 2.0), &pebble, tol) {
            Ok(BooleanResult::Body(bb)) => placed(&bb.body, &pose),
            other => panic!("{tag}: shaft and pebble: {other:?}"),
        };
        assert_eq!(two.shells().count(), 2, "{tag}: two solids");
        let (cb, bc) = (wall_decls_at(&c, &two, 0.5), wall_decls_at(&two, &c, 0.5));
        let scale = collar_volume;
        holds(
            topo::intersect_with(&c, &two, &cb, tol),
            (pebble_volume, 1),
            scale,
            &format!("{tag}: collar ∩ shaft and pebble"),
        );
        holds(
            topo::subtract_with(&c, &two, &cb, tol),
            (collar_volume - pebble_volume, 2),
            scale,
            &format!("{tag}: collar ∖ shaft and pebble"),
        );
        holds(
            topo::subtract_with(&two, &c, &bc, tol),
            (shaft_volume, 1),
            scale,
            &format!("{tag}: shaft and pebble ∖ collar"),
        );
        for (order, x, y, d) in [("collar", &c, &two, &cb), ("shaft", &two, &c, &bc)] {
            holds(
                topo::union_with(x, y, d, tol),
                (collar_volume + shaft_volume, 1),
                scale,
                &format!("{tag}: {order} first, ∪"),
            );
        }
    }
}
