//! **ROUND measurement M1: does a declared joint survive a ray-extension
//! fillet after a straight leg?**
//!
//! A bare `fillet(r)` on a straight leg's end, and the same verb after
//! `.tangent()` or `.cusp()`, all open a ray-incoming fillet with
//! `by_tangent` set, and the ray-extension emission moves the leg's end
//! vertex to the incoming trim point (`extend_leg_to`). For `.cusp()`
//! the ray points BACK along the leg, so "extending" it moves the
//! leg's end backwards.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::{ClosedLoop, ConstructedProfile, SketchPlane, Start, replay_recording};

fn tol() -> Tol {
    crate::common::tol()
}

fn dump(what: &str, closed: &ClosedLoop<f64>) {
    let lp = closed.loop_.as_loop();
    println!("{what}: vertices {:?}", lp.vertices());
    println!("{what}: segments {:?}", lp.segments());
    println!(
        "{what}: constructed joints {:?}",
        closed.loop_.constructed_joints()
    );
    match ConstructedProfile::new(SketchPlane::xy(), vec![closed.loop_.clone()]).validate(tol()) {
        Ok(v) => println!(
            "{what}: validates; canonical vertices {:?} tangent {:?} cusp {:?}",
            v.loops()[0].vertices(),
            v.loops()[0].tangent_joints(),
            v.loops()[0].cusp_joints()
        ),
        Err(e) => println!("{what}: validation refuses: {e:?}"),
    }
}

fn bits(vs: &[Point2<f64>]) -> Vec<(u64, u64)> {
    vs.iter().map(|p| (p.x.to_bits(), p.y.to_bits())).collect()
}

fn on_path(closed: &ClosedLoop<f64>, p: Point2<f64>) -> bool {
    // A straight-leg probe: is `p` on some line segment of the loop?
    let lp = closed.loop_.as_loop();
    let vs = lp.vertices();
    let n = vs.len();
    (0..n).any(|i| {
        let (a, b) = (vs[i], vs[(i + 1) % n]);
        let ab = b - a;
        let ap = p - a;
        let cross = ab.perp_dot(ap);
        let t = ap.dot(ab) / ab.norm_squared();
        matches!(lp.segments()[i], profile::Segment::Line) && cross.abs() < 1e-12 && (-1e-12..=1.0 + 1e-12).contains(&t)
    })
}

/// (a) `line(2)` east from the origin, bare `fillet(0.5)`, arrival up the
/// line x = 3 to (3, 2), closed. Corner (3, 0); trims (2.5, 0), (3, 0.5).
fn bare() -> ClosedLoop<f64> {
    let t = Tol::witness();
    profile::Open
        .at(Point2::new(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(2.0, t)
        .unwrap()
        .fillet(0.5, t)
        .unwrap()
        .toward(0.0, 1.0, t)
        .unwrap()
        .to(Point2::new(3.0, 2.0), t)
        .unwrap()
        .line_to(Start, t)
        .unwrap()
}

/// (b) the same with `.tangent()` declared at the leg end.
fn tangent() -> ClosedLoop<f64> {
    let t = Tol::witness();
    profile::Open
        .at(Point2::new(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(2.0, t)
        .unwrap()
        .tangent()
        .fillet(0.5, t)
        .unwrap()
        .toward(0.0, 1.0, t)
        .unwrap()
        .to(Point2::new(3.0, 2.0), t)
        .unwrap()
        .line_to(Start, t)
        .unwrap()
}

/// (c) `.cusp()` at the leg end: the ray runs WEST from (2, 0). The
/// arrival is the line x = `x_arr` up to (`x_arr`, 2), so the corner is
/// (`x_arr`, 0), ahead of the reversed ray.
fn cusp(x_arr: f64) -> Result<ClosedLoop<f64>, profile::PathError<f64>> {
    let t = Tol::witness();
    profile::Open
        .at(Point2::new(0.0, 0.0))
        .angle(0.0, t)?
        .line(2.0, t)?
        .cusp()
        .fillet(0.5, t)?
        .toward(0.0, 1.0, t)?
        .to(Point2::new(x_arr, 2.0), t)?
        .line_to(Start, t)
}

#[test]
fn m1a_bare_fillet_after_a_straight_leg() {
    let c = bare();
    dump("M1a", &c);
    let vs = c.loop_.as_loop().vertices().to_vec();
    assert!(vs[1].x == 2.5 && vs[1].y == 0.0, "the leg end moved forward to t1");
    assert_eq!(c.loop_.constructed_joints(), &[1, 2]);
    assert!(on_path(&c, Point2::new(2.0, 0.0)), "the authored end stays on the path");
    let (re, _) = replay_recording(&c.program, Tol::witness()).expect("replays");
    assert_eq!(bits(re.as_loop().vertices()), bits(c.loop_.as_loop().vertices()));
}

#[test]
fn m1b_tangent_then_fillet_after_a_straight_leg() {
    let c = tangent();
    dump("M1b", &c);
    let b = bare();
    assert_eq!(
        bits(c.loop_.as_loop().vertices()),
        bits(b.loop_.as_loop().vertices()),
        "identical to the bare fillet"
    );
    assert_eq!(c.loop_.constructed_joints(), b.loop_.constructed_joints());
    assert!(on_path(&c, Point2::new(2.0, 0.0)));
}

fn validated_cusps(c: &ClosedLoop<f64>) -> Vec<Point2<f64>> {
    let v = ConstructedProfile::new(SketchPlane::xy(), vec![c.loop_.clone()])
        .validate(tol())
        .expect("measured: the loop validates");
    let l = &v.loops()[0];
    l.cusp_joints().iter().map(|&j| l.vertices()[j]).collect()
}

/// (c), corner (1, 0) inside the leg: setback 0.5 < 2. MEASURED: builds
/// and validates; the leg's end vertex moves BACK from (2, 0) to
/// (1.5, 0) — the authored `line(2)` is 1.5 m long on the final path
/// and (2, 0) is not on it; the cusp survives, re-sited at (1.5, 0).
/// Red-first on the authored end.
#[test]
fn m1c_cusp_then_fillet_corner_inside_the_leg() {
    let c = cusp(1.0).expect("measured: builds");
    dump("M1c(x=1)", &c);
    let (re, _) = replay_recording(&c.program, Tol::witness()).expect("replays");
    assert_eq!(bits(re.as_loop().vertices()), bits(c.loop_.as_loop().vertices()));
    let cusps = validated_cusps(&c);
    assert_eq!(cusps.len(), 1, "the declared cusp is present");
    assert!(cusps[0].x == 1.5 && cusps[0].y == 0.0, "re-sited at t1: {cusps:?}");
    assert!(
        on_path(&c, Point2::new(2.0, 0.0)),
        "the authored leg end (2, 0) is off the final path: vertices {:?}",
        c.loop_.as_loop().vertices()
    );
}

/// (c), corner (-1, 0) behind the leg's start: the trim lands at
/// (-0.5, 0), behind the start too. MEASURED: builds and validates; the
/// leg's end vertex moves to (-0.5, 0), so `line(2)` at heading 0 is a
/// 0.5 m line at heading π, the arc then departs SMOOTHLY, and the
/// declared cusp is silently dropped (validated `cusp_joints` empty).
/// Red-first on both.
#[test]
fn m1c_cusp_then_fillet_corner_behind_the_leg_start() {
    let c = cusp(-1.0).expect("measured: builds");
    dump("M1c(x=-1)", &c);
    let cusps = validated_cusps(&c);
    assert_eq!(cusps.len(), 1, "the declared cusp was dropped: {cusps:?}");
    assert!(
        on_path(&c, Point2::new(2.0, 0.0)),
        "the authored leg end (2, 0) is off the final path: vertices {:?}",
        c.loop_.as_loop().vertices()
    );
}

/// (c), corner (-0.5, 0): the trim lands exactly on the leg's START.
#[test]
fn m1c_cusp_then_fillet_trim_on_the_leg_start() {
    match cusp(-0.5) {
        Ok(c) => {
            dump("M1c(x=-0.5)", &c);
            panic!("measured: builds with the leg collapsed onto its start");
        }
        Err(e) => println!("M1c(x=-0.5): refuses {e:?}"),
    }
}

/// (c) through the fused `fillet_arc`: the ray west from (2, 0) meets
/// the circle about (0.8, 0.6) of radius 1 at (1.6, 0), arrival Ccw.
#[test]
fn m1d_cusp_then_fillet_arc() {
    let t = Tol::witness();
    let c = Point2::new(0.8, 0.6);
    let p = Point2::new(c.x + 0.5_f64.cos(), c.y + 0.5_f64.sin());
    let built = profile::Open
        .at(Point2::new(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(2.0, t)
        .unwrap()
        .cusp()
        .fillet_arc(
            0.2,
            profile::Center {
                c,
                winding: profile::ArcSweep::Ccw,
                p,
            },
            t,
        )
        .and_then(|x| x.line_to(Start, t));
    match built {
        Ok(cl) => {
            dump("M1d", &cl);
            assert!(
                on_path(&cl, Point2::new(2.0, 0.0)),
                "the authored leg end (2, 0) is off the final path: vertices {:?}",
                cl.loop_.as_loop().vertices()
            );
        }
        Err(e) => println!("M1d: refuses {e:?}"),
    }
}
