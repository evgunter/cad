//! Review probe (PR 7120): the cover's `box_in` against `aabb(0.0)` and the
//! old whole-span `eval`, per carrier arm. Prints a table; asserts nothing
//! beyond the closed forms' box_in matching eval bit for bit.

use std::sync::Arc;
use geom::cover::ParamWindow;
use geom::{Circle, Curve3, Frame, Line, NurbsCurve3};
use geom_core::spline::KnotVector;
use geom_core::{Bounds, Interval, Point3, Real, Vec3};

fn show(what: &str, c: &Curve3<f64>, t0: f64, t1: f64) {
    let ci: Curve3<Interval> = c.map_scalar(Interval::from_f64);
    let cov = ci.cover_over(ParamWindow::new(Interval::from_f64(t0), Interval::from_f64(t1)));
    let ev = ci.eval(Interval::from_bounds(t0, t1));
    let e0 = c.eval(t0); let e1 = c.eval(t1);
    match cov {
        Ok(cv) => {
            let b = cv.aabb(0.0);
            let (lo,hi)=cv.box_in(&geom::cover::BoxAxes::World);
            println!("{what}\n  boxin x[{:e},{:e}] y[{:e},{:e}] z[{:e},{:e}]", lo.x.lo(),hi.x.hi(),lo.y.lo(),hi.y.hi(),lo.z.lo(),hi.z.hi());
            if !matches!(c, Curve3::Nurbs(_)) && ev.x.lo().is_finite() {
                assert_eq!((lo.x.lo(), hi.x.hi(), lo.y.lo(), hi.y.hi()), (ev.x.lo(), ev.x.hi(), ev.y.lo(), ev.y.hi()), "{what}: box_in is the old evaluation");
            }
            println!("  cover x[{:e},{:e}] y[{:e},{:e}] z[{:e},{:e}]", b.min_x,b.max_x,b.min_y,b.max_y,b.min_z,b.max_z);
        }
        Err(e) => println!("{what}\n  cover ERR {e:?}"),
    }
    println!("  eval  x[{:e},{:e}] y[{:e},{:e}] z[{:e},{:e}]", ev.x.lo(),ev.x.hi(),ev.y.lo(),ev.y.hi(),ev.z.lo(),ev.z.hi());
    println!("  ends  {:?} {:?}", e0, e1);
}

#[test]
fn tightness_table() {
    let o = Point3::new(0.0, 0.0, 0.0);
    show("segment y=2 along x", &Curve3::Line(Line { origin: Point3::new(0.0,2.0,0.0), dir: Vec3::new(1.0,0.0,0.0) }), -1.0, 1.0);
    show("segment diag", &Curve3::Line(Line { origin: Point3::new(0.1,0.2,0.3), dir: Vec3::new(0.3,0.7,-0.2) }), 0.0, 1.0);
    let f = Frame::new(o, Vec3::unit_z(), Vec3::unit_x());
    let circ = Curve3::Circle(Circle::new(f, 0.5));
    show("circle r=.5 half [0,pi]", &circ, 0.0, std::f64::consts::PI);
    show("circle r=.5 quarter [0,pi/2]", &circ, 0.0, std::f64::consts::FRAC_PI_2);
    show("circle r=.5 small [0.1,0.2]", &circ, 0.1, 0.2);
    show("circle r=.5 >turn [-1,7]", &circ, -1.0, 7.0);
    show("circle r=.5 reversed [pi,0]", &circ, std::f64::consts::PI, 0.0);
    let f2 = Frame::new(Point3::new(-1.0,0.0,0.0), Vec3::unit_z(), Vec3::unit_x());
    show("circle c=(-1,0) r=1 [pi/2, 3pi/2]", &Curve3::Circle(Circle::new(f2, 1.0)), std::f64::consts::FRAC_PI_2, 3.0*std::f64::consts::FRAC_PI_2);
    // rational with spread weights
    let kv = KnotVector::clamped(vec![0.0,0.0,0.0,1.0,1.0,1.0],2).unwrap();
    for w in [1.0, 1e3, 1e8, 1e-8] {
        let s = NurbsCurve3::new(kv.clone(), vec![Point3::new(0.0,0.0,0.0),Point3::new(1.0,1.0,0.0),Point3::new(2.0,0.0,0.0)], vec![1.0,w,1.0]).unwrap();
        let c = Curve3::Nurbs(Arc::new(s.clone()));
        show(&format!("rational quad w_mid={w} [0,1]"), &c, 0.0, 1.0);
        show(&format!("rational quad w_mid={w} sub [0.0,0.1]"), &c, 0.0, 0.1);
        // sample max y over [0,.1]
        let m = (0..=1000).map(|k| s.eval(0.1*f64::from(k)/1000.0).y).fold(0.0f64, f64::max);
        println!("  sampled max y on [0,.1] = {m:e}");
    }
    let s = NurbsCurve3::new(kv.clone(), vec![Point3::new(0.0,0.0,0.0),Point3::new(1.0,1.0,0.0),Point3::new(2.0,0.0,0.0)], vec![1e8,1.0,1e-8]).unwrap();
    let c = Curve3::Nurbs(Arc::new(s.clone()));
    show("rational ends 1e8,1,1e-8 [0,1]", &c, 0.0, 1.0);
    let m = (0..=100000).map(|k| s.eval(f64::from(k)/100000.0)).fold((0.0f64,0.0f64), |a,p|(a.0.max(p.x),a.1.max(p.y)));
    println!("  sampled max x,y = {m:?}");
    // straight spline (segment as spline)
    let line = NurbsCurve3::new(KnotVector::clamped(vec![0.0,0.0,1.0,1.0],1).unwrap(), vec![Point3::new(0.0,2.0,0.0),Point3::new(1.0,2.0,0.0)], vec![1.0,1.0]).unwrap();
    show("linear spline y=2", &Curve3::Nurbs(Arc::new(line)), 0.0, 1.0);
}
