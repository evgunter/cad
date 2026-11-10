//! **A fillet on a straight leg's end keeps the leg's end.**
//!
//! A bare `fillet(r)` and `.tangent().fillet(r)` on a straight leg's end
//! open a ray that continues the leg, and the incoming run is emitted as
//! a segment of its own from the leg's authored end, which stays a
//! vertex with a constructed tangent joint. `.cusp()` reverses that ray:
//! the run doubles back from the leg's end, emitted the same way, and
//! the joint keeps the declared cusp — exactly what `.cusp().line(..)`
//! emits. On a straight leg that run lies on the leg's own carrier, so
//! the door refuses the close, naming the cusp at the leg's end
//! (`PathError::CuspDoublesBackOverLeg`), as it refuses
//! `.cusp().line(..)`; at an exact incoming fit there is no run, the arc
//! springs off the leg's end and the cusp validates.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Interval, Point2, Tol};
use profile::{
    ArcSweep, Center, ClosedLoop, ConstructedProfile, PathError, ProfileError, Segment,
    SketchPlane, Start, ValidatedProfile, replay_recording,
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

/// The refusal a reversed run off a straight leg gets at the close:
/// it names the cusp, the leg's authored end `at`.
fn assert_doubles_back<T: profile::ArcCarrierScalar + geom_core::Bounds>(
    built: Result<impl core::fmt::Debug, PathError<T>>,
    at: (f64, f64),
) {
    match built {
        Err(PathError::CuspDoublesBackOverLeg { at: got }) => {
            let got = (got.x.lo(), got.y.lo());
            assert!(
                (got.0 - at.0).abs() < 1e-12 && (got.1 - at.1).abs() < 1e-12,
                "the cusp at the leg's end {at:?}: {got:?}"
            );
        }
        other => panic!("a run back over the leg is refused at the close: {other:?}"),
    }
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

/// The forward ray's run is its own segment: the leg keeps its authored
/// end (2, 0) as vertex 1 with a constructed joint, and the run goes on
/// to t1 = (2.5, 0), where the arc's joint is constructed too. Both
/// validate as tangent joints, on one carrier at vertex 1.
#[test]
fn a_bare_fillet_keeps_the_straight_legs_end() {
    let c = build(Before::Nothing, 3.0).expect("builds");
    assert_vertices(
        &c,
        &[(0.0, 0.0), (2.0, 0.0), (2.5, 0.0), (3.0, 0.5), (3.0, 2.0)],
    );
    assert_eq!(kinds(&c), ["line", "line", "arc", "line", "line"]);
    assert_eq!(c.loop_.constructed_joints(), &[1, 2, 3]);
    assert_replays(&c);
    let v = validated(&c).expect("validates");
    assert_eq!(v.loops()[0].tangent_joints(), &[1, 2, 3]);
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

/// Corners inside the leg (1, 0), behind its start (-1, 0), and with
/// t1 at the start itself (-0.5, 0): each reversed run lies back over
/// the leg from its authored end (2, 0), where the cusp is declared,
/// and the close refuses there.
#[test]
fn cusp_then_fillet_doubles_back_from_the_leg_end() {
    for x in [1.0, -1.0, -0.5] {
        assert_doubles_back(build(Before::Cusp, x), (2.0, 0.0));
    }
}

/// The Interval lane refuses the same program at the same cusp.
#[test]
fn cusp_then_fillet_on_the_interval_lane() {
    use profile::{Step, Target};
    for x in [1.0, -1.0] {
        let program = [
            Step::At(Point2::new(0.0, 0.0)),
            Step::Angle(0.0),
            Step::Line(2.0),
            Step::Cusp,
            Step::Fillet { radius: 0.5 },
            Step::Toward { dx: 0.0, dy: 1.0 },
            Step::FarEndTo(Point2::new(x, 2.0)),
            Step::LineTo(Target::Start),
        ];
        let refused =
            crate::common::try_replay_at::<Interval>(&program).map_err(|e| match e.kind {
                profile::ReplayErrorKind::Path(source) => source,
                other => panic!("x = {x}: {other:?}"),
            });
        assert_doubles_back(refused, (2.0, 0.0));
    }
}

/// The `.at` binder resolves the same pending ray, and the run it
/// emits from the leg's end (2, 0) is refused the same way.
#[test]
fn cusp_then_fillet_resolved_by_at_doubles_back_from_the_leg_end() {
    let t = Tol::witness();
    let built = profile::Open
        .at(Point2::new(0.0, 0.0))
        .angle(0.0, t)
        .and_then(|x| x.line(2.0, t))
        .and_then(|x| x.cusp().fillet(0.5, t))
        .and_then(|x| x.toward(0.0, 1.0, t))
        .and_then(|x| x.at(Point2::new(1.0, 1.0), t))
        .and_then(|x| x.line(1.0, t))
        .and_then(|x| x.line_to(Start, t));
    assert_doubles_back(built, (2.0, 0.0));
}

/// A seam close: the cusp at (4, 3) sends the ray back down the leg
/// from (2, 2), to meet the start's carrier y = 0 behind the start; the
/// run back is refused at the cusp, the leg's end (4, 3).
#[test]
fn cusp_then_fillet_closing_at_the_seam_doubles_back_from_the_leg_end() {
    let t = Tol::witness();
    let built = profile::Open
        .at(Point2::new(0.0, 0.0))
        .angle(0.0, t)
        .and_then(|x| x.line(2.0, t))
        .and_then(|x| x.line_to(Point2::new(2.0, 2.0), t))
        .and_then(|x| x.line_to(Point2::new(4.0, 3.0), t))
        .and_then(|x| x.cusp().fillet(0.3, t))
        .and_then(|x| x.to(Start, t));
    assert_doubles_back(built, (4.0, 3.0));
}

/// An exact incoming fit (corner (1.5, 0), t1 = the leg's end) emits no
/// run: the arc springs off (2, 0), and the declared cusp validates
/// there.
#[test]
fn cusp_then_fillet_at_an_exact_fit_validates_its_cusp() {
    let c = build(Before::Cusp, 1.5).expect("builds");
    assert_vertices(&c, &[(0.0, 0.0), (2.0, 0.0), (1.5, 0.5), (1.5, 2.0)]);
    assert_eq!(kinds(&c), ["line", "arc", "line", "line"]);
    assert_eq!(c.loop_.constructed_joints(), &[1, 2]);
    assert_replays(&c);
    let v = validated(&c).expect("validates");
    let l = &v.loops()[0];
    let cusps: Vec<Point2<f64>> = l.cusp_joints().iter().map(|&j| l.vertices()[j]).collect();
    assert_eq!(bits(&cusps), bits(&[Point2::new(2.0, 0.0)]));
}

/// The fused `fillet_arc` after `.cusp()` takes the same emission: the
/// ray west from (2, 0) meets the circle about (0.8, 0.6) of radius 1 at
/// (1.6, 0), arriving counterclockwise, and its run back is refused.
#[test]
fn cusp_then_fillet_arc_doubles_back_from_the_leg_end() {
    let t = Tol::witness();
    let centre = Point2::new(0.8, 0.6);
    let built = profile::Open
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
        .and_then(|x| x.line_to(Start, t));
    assert_doubles_back(built, (2.0, 0.0));
}

/// The reversed run is the one `.cusp().line(..)` emits off the same
/// leg end, and the close refuses both the same way.
#[test]
fn cusp_then_line_doubles_back_the_same_way() {
    let t = Tol::witness();
    let built = profile::Open
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
        .line_to(Start, t);
    assert_doubles_back(built, (2.0, 0.0));
}

/// A cusp on an ARC's end is not this refusal: the straight run off it
/// leaves the arc's tangent, and the loop it closes is the door's to
/// build.
#[test]
fn a_cusp_off_an_arc_leaves_on_a_line() {
    let t = Tol::witness();
    let built = profile::Open
        .at(Point2::new(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .tangent_arc_to(Point2::new(2.0, 2.0), t)
        .unwrap()
        .cusp()
        .line(1.0, t)
        .unwrap()
        .line_to(Start, t);
    assert!(
        !matches!(built, Err(PathError::CuspDoublesBackOverLeg { .. })),
        "{built:?}"
    );
}
