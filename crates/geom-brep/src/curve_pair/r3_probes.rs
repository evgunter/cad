//! Review r3 probes for PR 5227 (the window certificate).

#![allow(clippy::unwrap_used, clippy::panic, clippy::print_stderr)]

use core::f64::consts::TAU;

use geom::Curve3;
use geom_core::{Point3, Vec3};

use super::meet_rows::band;
use super::{MeetFault, MeetRefusal, Meetings, Span, meet};

fn verdict(
    label: &str,
    got: &Result<Meetings<f64>, MeetRefusal<f64>>,
    want: &[Point3<f64>],
) -> Result<(), String> {
    match got {
        Ok(m) => {
            for w in want {
                if !m.points.iter().any(|p| (p.point - *w).norm() < 1e-6) {
                    return Err(format!("{label}: decided with {w:?} missing: {m:?}"));
                }
            }
            eprintln!("{label}: ok, {} meetings", m.points.len());
            Ok(())
        }
        Err(r) => {
            eprintln!("{label}: refused {:?}", r.fault);
            if matches!(r.fault, MeetFault::Escalated(_) | MeetFault::Unsettled(_)) {
                Ok(())
            } else {
                Err(format!("{label}: kernel fault {r:?}"))
            }
        }
    }
}

/// The tilted unit circle of `plane_rows::two_circles_at_a_shallow_tilt…`,
/// its seam (`u_ref = x̂`) on the plane root at `θ = 0`, read as a whole
/// closed span; the flat circle's span ends at one meeting (found first,
/// as `b`'s end) and runs through the other, which lies across `a`'s seam.
#[test]
fn probe_seam_inside_the_window() {
    let t = 3e-8f64;
    let a = Curve3::Circle {
        center: Point3::origin(),
        axis: Vec3::new(0.0, -t.sin(), t.cos()),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let cb = 0.5 + 1e-4;
    let b = Curve3::Circle {
        center: Point3::new(cb, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 0.5,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let x = (1.0 + cb * cb - 0.25) / (2.0 * cb);
    let y = (1.0 - x * x).sqrt();
    let th = (y / t.cos()).asin();
    let want = [a.eval(th), a.eval(TAU - th)];
    let phi = y.atan2(x - cb);
    let mut bad = Vec::new();
    for (sl, sb) in [("J above", (-0.5, phi)), ("J below", (-phi, 0.5))] {
        for (el, ended) in [("endless", false), ("ended", true)] {
            for (ol, a0) in [("seam 0", 0.0), ("seam -pi", -core::f64::consts::PI)] {
                let s1 = Span::new(&a, (a0, a0 + TAU));
                let s1 = if ended { s1 } else { s1.endless() };
                let s2 = Span::new(&b, sb);
                for (order, got) in [
                    ("a first", meet(&s1, &s2, band())),
                    ("b first", meet(&s2, &s1, band())),
                ] {
                    if let Err(e) = verdict(&format!("{sl}, {el}, {ol}, {order}"), &got, &want) {
                        bad.push(e);
                    }
                }
            }
        }
    }
    assert!(bad.is_empty(), "{bad:#?}");
}

/// `probe_seam_inside_the_window` at a steeper tilt and a larger flat
/// circle, so the window is narrow enough for the feet guard to pass.
#[test]
fn probe_seam_inside_a_narrow_window() {
    let mut bad = Vec::new();
    for &(t, rb, y) in &[(1e-7_f64, 0.9_f64, 4e-3_f64), (2e-7, 0.9, 2e-3), (1e-7, 0.5, 4e-3), (5e-8, 0.95, 8e-3)] {
        let a = Curve3::Circle {
            center: Point3::origin(),
            axis: Vec3::new(0.0, -f64::sin(t), f64::cos(t)),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let cb = (1.0 - y * y).sqrt() - (rb * rb - y * y).sqrt();
        let b = Curve3::Circle {
            center: Point3::new(cb, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: rb,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let th = (y / t.cos()).asin();
        let want = [a.eval(th), a.eval(TAU - th)];
        let phi = y.atan2((1.0 - y * y).sqrt() - cb);
        for (sl, sb) in [("J above", (-0.5, phi)), ("J below", (-phi, 0.5))] {
            for (el, ended) in [("endless", false), ("ended", true)] {
                for (ol, a0) in [("seam 0", 0.0), ("no seam", -core::f64::consts::PI)] {
                    let s1 = Span::new(&a, (a0, a0 + TAU));
                    let s1 = if ended { s1 } else { s1.endless() };
                    let s2 = Span::new(&b, sb);
                    for (order, got) in [
                        ("a first", meet(&s1, &s2, band())),
                        ("b first", meet(&s2, &s1, band())),
                    ] {
                        let l = format!("t {t:e} rb {rb} y {y:e}: {sl}, {el}, {ol}, {order}");
                        if let Err(e) = verdict(&l, &got, &want) {
                            bad.push(e);
                        }
                    }
                }
            }
        }
    }
    assert!(bad.is_empty(), "{bad:#?}");
}

/// A closed rim whose seam vertex is the joint (both of its ends at `J`,
/// on the other's plane), and a second meeting `δ` before the seam, in
/// the plane root's window: the shape of a closed edge whose one vertex
/// is a fillet joint.
#[test]
fn probe_seam_vertex_at_the_joint() {
    let mut bad = Vec::new();
    for &(t, rb, delta) in &[(1e-7_f64, 0.9_f64, 4e-3_f64), (1e-7, 1.2, 4e-3), (2e-7, 0.9, 2e-3)] {
        let (s, c) = t.sin_cos();
        let a = Curve3::Circle {
            center: Point3::origin(),
            axis: Vec3::new(0.0, -s, c),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let j = Point3::new(1.0, 0.0, 0.0);
        let pp = Point3::new(delta.cos(), -c * delta.sin(), 0.0);
        let m = Point3::new(0.5 * (j.x + pp.x), 0.5 * (j.y + pp.y), 0.0);
        let chord = pp - j;
        let half = 0.5 * chord.norm();
        let n = Vec3::new(chord.y, -chord.x, 0.0).normalize();
        let n = if n.dot(Point3::origin() - m) > 0.0 { n } else { n * -1.0 };
        let centre = m + n * (rb * rb - half * half).sqrt();
        let b = Curve3::Circle {
            center: centre,
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: rb,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let ang = |q: Point3<f64>| (q.y - centre.y).atan2(q.x - centre.x);
        let (pj, pq) = (ang(j), ang(pp));
        let want = [j, a.eval(TAU - delta)];
        for (el, ended) in [("ended", true), ("endless", false)] {
            let s1 = Span::new(&a, (0.0, TAU));
            let s1 = if ended { s1 } else { s1.endless() };
            let s2 = Span::new(&b, (pj.min(pq) - 0.3, pj.max(pq) + 0.3));
            for (order, got) in [
                ("a first", meet(&s1, &s2, band())),
                ("b first", meet(&s2, &s1, band())),
            ] {
                let l = format!("t {t:e} rb {rb} delta {delta:e}: {el}, {order}");
                if let Err(e) = verdict(&l, &got, &want) {
                    bad.push(e);
                }
            }
        }
    }
    assert!(bad.is_empty(), "{bad:#?}");
}
