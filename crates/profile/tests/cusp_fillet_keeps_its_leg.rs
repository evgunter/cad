//! **A fillet on a straight leg's end extends the leg only along the
//! leg's own forward tangent.**
//!
//! A bare `fillet(r)` and `.tangent().fillet(r)` on a straight leg's end
//! open a ray that continues the leg, so the incoming run's emission
//! moves the leg's end vertex forward to the trim point and the leg's
//! authored end stays on the path, interior to the extended segment.
//! `.cusp()` reverses that ray: the run doubles back from the leg's end,
//! so it is emitted as its own segment from the leg's end, which keeps
//! the declared cusp — exactly what `.cusp().line(..)` emits. On a
//! straight leg that run lies on the leg's own carrier, so the loop is
//! a collinear spike and validation refuses it as an overlap, as it
//! refuses `.cusp().line(..)`; at an exact incoming fit there is no run,
//! the arc springs off the leg's end and the cusp validates.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::{
    ArcSweep, Center, ClosedLoop, ConstructedProfile, ContactKind, PathError, ProfileError,
    Segment, SegmentRef, SketchPlane, Start, ValidatedProfile, replay_recording,
};

fn validated(c: &ClosedLoop<f64>) -> Result<ValidatedProfile<f64>, ProfileError> {
    ConstructedProfile::new(SketchPlane::xy(), vec![c.loop_.clone()]).validate(crate::common::tol())
}

fn bits(vs: &[Point2<f64>]) -> Vec<(u64, u64)> {
    vs.iter().map(|p| (p.x.to_bits(), p.y.to_bits())).collect()
}

fn assert_vertices(c: &ClosedLoop<f64>, want: &[(f64, f64)]) {
    let want: Vec<Point2<f64>> = want.iter().map(|&(x, y)| Point2::new(x, y)).collect();
    assert_eq!(
        bits(c.loop_.as_loop().vertices()),
        bits(&want),
        "vertices {:?}",
        c.loop_.as_loop().vertices()
    );
}

fn kinds(c: &ClosedLoop<f64>) -> Vec<&'static str> {
    c.loop_
        .as_loop()
        .segments()
        .iter()
        .map(|s| match s {
            Segment::Line => "line",
            Segment::Arc(_) => "arc",
        })
        .collect()
}

fn assert_replays(c: &ClosedLoop<f64>) {
    let (re, _) = replay_recording(&c.program, Tol::witness()).expect("replays");
    assert_eq!(
        bits(re.as_loop().vertices()),
        bits(c.loop_.as_loop().vertices())
    );
    assert_eq!(re.constructed_joints(), c.loop_.constructed_joints());
}

/// The collinear spike a reversed run off a straight leg makes: the
/// leg (segment 0) and the run (segment 1) overlap.
fn assert_spike(c: &ClosedLoop<f64>) {
    let sref = |segment_index| SegmentRef {
        loop_index: 0,
        segment_index,
    };
    assert_eq!(
        validated(c).expect_err("a run back over the leg is not simple"),
        ProfileError::NonSimple {
            first: sref(0),
            second: sref(1),
            kind: ContactKind::Overlap,
        }
    );
}

#[derive(Clone, Copy)]
enum Before {
    Nothing,
    Tangent,
    Cusp,
}

/// `line(2)` east from the origin, then `before`, then `fillet(0.5)`,
/// arriving up the line x = `x_arr` to (`x_arr`, 2), closed.
fn build(before: Before, x_arr: f64) -> Result<ClosedLoop<f64>, PathError<f64>> {
    let t = Tol::witness();
    let leg = profile::Open
        .at(Point2::new(0.0, 0.0))
        .angle(0.0, t)?
        .line(2.0, t)?;
    match before {
        Before::Nothing => leg.fillet(0.5, t)?,
        Before::Tangent => leg.tangent().fillet(0.5, t)?,
        Before::Cusp => leg.cusp().fillet(0.5, t)?,
    }
    .toward(0.0, 1.0, t)?
    .to(Point2::new(x_arr, 2.0), t)?
    .line_to(Start, t)
}

/// The forward ray extends the leg: its end moves to t1 = (2.5, 0) and
/// the joint declared there is the leg→arc tangency.
#[test]
fn a_bare_fillet_extends_the_straight_leg() {
    let c = build(Before::Nothing, 3.0).expect("builds");
    assert_vertices(&c, &[(0.0, 0.0), (2.5, 0.0), (3.0, 0.5), (3.0, 2.0)]);
    assert_eq!(kinds(&c), ["line", "arc", "line", "line"]);
    assert_eq!(c.loop_.constructed_joints(), &[1, 2]);
    assert_replays(&c);
    let v = validated(&c).expect("validates");
    assert_eq!(v.loops()[0].tangent_joints(), &[1, 2]);
    assert!(v.loops()[0].cusp_joints().is_empty());
}

/// `.tangent()` before the fillet declares the same joint the bare
/// fillet's ray extension does, and lowers to the same loop bit for bit.
#[test]
fn tangent_then_fillet_is_the_bare_fillet() {
    let c = build(Before::Tangent, 3.0).expect("builds");
    let b = build(Before::Nothing, 3.0).expect("builds");
    assert_eq!(
        bits(c.loop_.as_loop().vertices()),
        bits(b.loop_.as_loop().vertices())
    );
    assert_eq!(c.loop_.constructed_joints(), b.loop_.constructed_joints());
}

/// Corner (1, 0) inside the leg. The leg keeps its authored end (2, 0)
/// as vertex 1, where the cusp is declared; the reversed run (2, 0) →
/// t1 = (1.5, 0) is its own segment, declared tangent to the arc at its
/// end. That run lies back over the leg, and validation says so.
#[test]
fn cusp_then_fillet_keeps_the_leg_end_and_its_cusp() {
    let c = build(Before::Cusp, 1.0).expect("builds");
    assert_vertices(
        &c,
        &[(0.0, 0.0), (2.0, 0.0), (1.5, 0.0), (1.0, 0.5), (1.0, 2.0)],
    );
    assert_eq!(kinds(&c), ["line", "line", "arc", "line", "line"]);
    assert_eq!(c.loop_.constructed_joints(), &[1, 2, 3]);
    assert_replays(&c);
    assert_spike(&c);
}

/// Corner (-1, 0) behind the leg's start: the trim (-0.5, 0) lies behind
/// the start too, and the leg still ends at (2, 0) heading east rather
/// than being turned round to end at the trim.
#[test]
fn cusp_then_fillet_behind_the_leg_start_does_not_reverse_the_leg() {
    let c = build(Before::Cusp, -1.0).expect("builds");
    assert_vertices(
        &c,
        &[
            (0.0, 0.0),
            (2.0, 0.0),
            (-0.5, 0.0),
            (-1.0, 0.5),
            (-1.0, 2.0),
        ],
    );
    assert_eq!(c.loop_.constructed_joints(), &[1, 2, 3]);
    assert_spike(&c);
}

/// An exact incoming fit (corner (1.5, 0), t1 = the leg's end) emits no
/// run: the arc springs off (2, 0), and the declared cusp validates
/// there.
#[test]
fn cusp_then_fillet_at_an_exact_fit_validates_its_cusp() {
    let c = build(Before::Cusp, 1.5).expect("builds");
    assert_vertices(&c, &[(0.0, 0.0), (2.0, 0.0), (1.5, 0.5), (1.5, 2.0)]);
    assert_eq!(kinds(&c), ["line", "arc", "line", "line"]);
    assert_replays(&c);
    let v = validated(&c).expect("validates");
    let l = &v.loops()[0];
    let cusps: Vec<Point2<f64>> = l.cusp_joints().iter().map(|&j| l.vertices()[j]).collect();
    assert_eq!(bits(&cusps), bits(&[Point2::new(2.0, 0.0)]));
}

/// The fused `fillet_arc` after `.cusp()` takes the same emission: the
/// ray west from (2, 0) meets the circle about (0.8, 0.6) of radius 1 at
/// (1.6, 0), arriving counterclockwise.
#[test]
fn cusp_then_fillet_arc_keeps_the_leg_end() {
    let t = Tol::witness();
    let centre = Point2::new(0.8, 0.6);
    let c = profile::Open
        .at(Point2::new(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(2.0, t)
        .unwrap()
        .cusp()
        .fillet_arc(
            0.2,
            Center {
                c: centre,
                winding: ArcSweep::Ccw,
                p: Point2::new(centre.x + 0.5_f64.cos(), centre.y + 0.5_f64.sin()),
            },
            t,
        )
        .and_then(|x| x.line_to(Start, t))
        .expect("builds");
    let vs = c.loop_.as_loop().vertices();
    assert_eq!(vs.len(), 5, "{vs:?}");
    assert!(
        vs[1].x == 2.0 && vs[1].y == 0.0,
        "the leg keeps its end: {vs:?}"
    );
    assert!(
        (vs[2].x - 1.931_370_849_898_476).abs() < 1e-12 && vs[2].y == 0.0,
        "the reversed run ends at t1: {vs:?}"
    );
    assert_eq!(kinds(&c), ["line", "line", "arc", "arc", "line"]);
    assert_eq!(c.loop_.constructed_joints(), &[1, 2, 3]);
    assert_spike(&c);
}

/// The reversed run is the one `.cusp().line(..)` emits off the same
/// leg end, and validation gives both the same verdict.
#[test]
fn cusp_then_line_is_the_same_spike() {
    let t = Tol::witness();
    let c = profile::Open
        .at(Point2::new(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(2.0, t)
        .unwrap()
        .cusp()
        .line(0.5, t)
        .unwrap()
        .line_to(Point2::new(1.0, 1.0), t)
        .unwrap()
        .line_to(Start, t)
        .unwrap();
    assert_vertices(&c, &[(0.0, 0.0), (2.0, 0.0), (1.5, 0.0), (1.0, 1.0)]);
    assert_eq!(c.loop_.constructed_joints(), &[1]);
    assert_spike(&c);
}
