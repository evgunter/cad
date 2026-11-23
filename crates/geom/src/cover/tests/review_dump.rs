//! Review differential (PR 7510, reviewer r1): the bits of every reading
//! a converted site reaches, printed one per line, for the merge base
//! and the frozen head to be compared. Uses only doors both trees have.
#![allow(clippy::unwrap_used, clippy::print_stdout)]

use core::f64::consts::{FRAC_PI_2, PI, TAU};

use geom_core::{Bounds, Interval, Point3, Real, Vec3};

use super::{BoxAxes, ParamWindow, Site, ipt, iv, ivec};
use crate::curves::boxes::{conic_arc_aabb, conic_arc_extent_along, conic_arc_extent_along_bracketed};
use crate::{Curve3, Ellipse, Frame, Line};

fn fixtures() -> Vec<(String, Curve3<f64>, Curve3<Interval>)> {
    let mut out = Vec::new();
    let frames = [
        (Point3::new(0.5, -1.0, 2.0), Vec3::new(2.0, 2.0, 1.0) / 3.0, Vec3::new(1.0, -2.0, 2.0) / 3.0),
        (Point3::new(40.25, -7.5, 3.125), Vec3::new(1.0, 2.0, 2.0) / 3.0, Vec3::new(2.0, 1.0, -2.0) / 3.0),
        (Point3::new(1e3, 2e3, -3e3), Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.6, 0.8, 0.0)),
    ];
    for (fi, (c, n, u)) in frames.into_iter().enumerate() {
        for r in [0.7, 22.0, 2.2e5] {
            out.push((
                format!("circle f{fi} r{r:e}"),
                Curve3::circle(c, n, r, u),
                Curve3::circle(ipt(c), ivec(n), iv(r), ivec(u)),
            ));
        }
        for (a, b) in [(0.9, 0.4), (0.4, 0.9), (2.2e4, 1.76e4), (2.2e7, 1.76e7)] {
            for seam in [None, Some(FRAC_PI_2), Some(PI), Some(2.3)] {
                let s = |e: Ellipse<f64>| seam.map_or(e, |t| e.seamed_near(t));
                let is = |e: Ellipse<Interval>| seam.map_or(e, |t| e.seamed_near(iv(t)));
                out.push((
                    format!("ellipse-centre f{fi} a{a:e} b{b:e} seam {seam:?}"),
                    Curve3::Ellipse(s(Ellipse::new(Frame::new(c, n, u), a, b))),
                    Curve3::Ellipse(is(Ellipse::new(Frame::new(ipt(c), ivec(n), ivec(u)), iv(a), iv(b)))),
                ));
                out.push((
                    format!("ellipse-ARC f{fi} a{a:e} b{b:e} seam {seam:?}"),
                    Curve3::Ellipse(s(Ellipse::anchored(c, n, u, a, b))),
                    Curve3::Ellipse(is(Ellipse::anchored(ipt(c), ivec(n), ivec(u), iv(a), iv(b)))),
                ));
            }
        }
    }
    out
}

fn bits(x: f64) -> String {
    format!("{:016x}", x.to_bits())
}

fn ibits(x: Interval) -> String {
    format!("[{} {}]", bits(Bounds::lo(x)), bits(Bounds::hi(x)))
}

#[test]
fn review_dump_bits() {
    let windows = [(-1e-3, 1e-3), (0.0, 0.3), (-0.2, FRAC_PI_2), (1.0, 4.5), (5.0, 0.5), (0.0, TAU), (-1.0, 9.0)];
    let dirs = [
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.6, -0.8, 0.0),
        Vec3::new(1.0, 2.0, 2.0) / 3.0,
        Vec3::new(-2.0, 3.0, 0.5),
    ];
    for (name, c, ic) in fixtures() {
        let k = c.conic().unwrap();
        let p0 = c.eval(0.1);
        let sites = [
            Site::Point(p0 + Vec3::new(0.5, 0.25, -0.125)),
            Site::Point(Point3::new(1.0, 2.0, 3.0)),
            Site::Line(Line { origin: Point3::new(0.0, 1.0, 0.0), dir: Vec3::new(0.0, 0.6, 0.8) }),
        ];
        for d in dirs {
            let (p, q) = k.along(d);
            let (x, y) = k.unit_of(d);
            println!("DUMP {name} | along {d:?} | {} {} unit_of {} {}", bits(p), bits(q), bits(x), bits(y));
        }
        let (su, sv) = k.semi_axis_vectors();
        println!("DUMP {name} | semi_axis_vectors | {} {} {} {} {} {}", bits(su.x), bits(su.y), bits(su.z), bits(sv.x), bits(sv.y), bits(sv.z));
        let az = k.param_in(sites_point(&sites[0]), (0.0, 0.2));
        println!("DUMP {name} | param_in | {}", bits(az));
        for (t0, t1) in windows {
            let w = format!("[{t0},{t1}]");
            let cover = c.cover_over(ParamWindow::new(t0, t1)).unwrap();
            let (m, r) = cover.ball();
            println!("DUMP {name} | {w} ball | {} {} {} {}", bits(m.x), bits(m.y), bits(m.z), bits(r));
            let (lo, hi) = cover.box_in(&BoxAxes::World);
            println!("DUMP {name} | {w} box | {} {} {} {} {} {}", bits(lo.x), bits(lo.y), bits(lo.z), bits(hi.x), bits(hi.y), bits(hi.z));
            let bb = cover.aabb(0.0);
            println!("DUMP {name} | {w} aabb | {} {} {} {} {} {}", bits(bb.min_x), bits(bb.min_y), bits(bb.min_z), bits(bb.max_x), bits(bb.max_y), bits(bb.max_z));
            for (i, s) in sites.iter().enumerate() {
                let (near, far) = cover.distance_range(s);
                println!("DUMP {name} | {w} range site{i} | {} {}", bits(near), bits(far));
            }
            for d in dirs {
                println!("DUMP {name} | {w} support {d:?} | {}", bits(cover.support(d)));
                let (lo, hi) = conic_arc_extent_along(&c, t0, t1, Point3::new(0.25, 0.5, -1.0), d).unwrap();
                println!("DUMP {name} | {w} extent {d:?} | {} {}", bits(lo), bits(hi));
                let (lo, hi) = conic_arc_extent_along_bracketed(&c, t0, t1, Point3::new(0.25, 0.5, -1.0), d).unwrap();
                println!("DUMP {name} | {w} extent-bracketed {d:?} | {} {}", bits(lo), bits(hi));
            }
            let bb = conic_arc_aabb(&c, t0, t1, c.eval(t0), c.eval(t1)).unwrap();
            println!("DUMP {name} | {w} conic_arc_aabb | {} {} {} {} {} {}", bits(bb.min_x), bits(bb.min_y), bits(bb.min_z), bits(bb.max_x), bits(bb.max_y), bits(bb.max_z));
            if let Some(d) = c.distance_within(sites_point(&sites[1]), (t0, t1)) {
                println!("DUMP {name} | {w} distance_within | {}", bits(d));
            }
            // The Interval lane.
            let icover = ic.cover_over(ParamWindow::new(iv(t0), iv(t1))).unwrap();
            let (m, r) = icover.ball();
            println!("IDUMP {name} | {w} ball | {} {} {} {}", ibits(m.x), ibits(m.y), ibits(m.z), ibits(r));
            let bb = icover.aabb(0.0);
            println!("IDUMP {name} | {w} aabb | {} {} {} {} {} {}", bits(bb.min_x), bits(bb.min_y), bits(bb.min_z), bits(bb.max_x), bits(bb.max_y), bits(bb.max_z));
            for (i, s) in sites.iter().enumerate() {
                let is = match s {
                    Site::Point(q) => Site::Point(ipt(*q)),
                    Site::Line(l) => Site::Line(Line { origin: ipt(l.origin), dir: ivec(l.dir) }),
                };
                let (near, far) = icover.distance_range(&is);
                println!("IDUMP {name} | {w} range site{i} | {} {}", ibits(near), ibits(far));
            }
            for d in dirs {
                println!("IDUMP {name} | {w} support {d:?} | {}", ibits(icover.support(ivec(d))));
                let (lo, hi) = conic_arc_extent_along(&ic, iv(t0), iv(t1), ipt(Point3::new(0.25, 0.5, -1.0)), ivec(d)).unwrap();
                println!("IDUMP {name} | {w} extent {d:?} | {} {}", ibits(lo), ibits(hi));
            }
        }
        if let Some(h) = crate::pole_form::half_turn_curve(&c, Line { origin: Point3::new(0.0, 0.0, 1.0), dir: Vec3::new(0.0, 0.0, 1.0) }, |p| p, |v| v) {
            let q = h.eval(0.3);
            println!("DUMP {name} | half_turn_curve eval | {} {} {}", bits(q.x), bits(q.y), bits(q.z));
        }
        if let Some(d) = c.clearance(sites_point(&sites[1])) {
            println!("DUMP {name} | clearance | {}", bits(d));
        }
    }
}

fn sites_point(s: &Site<f64>) -> Point3<f64> {
    match s {
        Site::Point(q) => *q,
        Site::Line(l) => l.origin,
    }
}
