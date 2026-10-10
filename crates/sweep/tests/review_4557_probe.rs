#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
//! Reviewer probe (PR 4557): the ringed-wall rows under a tilted,
//! off-origin cylinder frame, and the ring-dropping mutant witness.
use core::f64::consts::PI;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane};
use sweep::test_support::{brick, finished};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::AtRestBody;

fn pipe_r(r: f64) -> AtRestBody<f64> {
    let tol = Tol::witness();
    let lp = profile::circle(Point2::new(0.0, 0.0), r, tol).unwrap();
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, -2.0 * r)));
    let profile = Profile::new(plane, vec![lp.into()]).validate(tol).unwrap();
    let pipe = extrude(&profile, Extrusion::Distance { depth: 4.0 * r, side: ExtrudeSide::Along }, tol)
        .unwrap()
        .body;
    finished("the pipe", pipe, tol)
}

fn moved(b: &AtRestBody<f64>, frame: &[Affine3<f64>]) -> AtRestBody<f64> {
    let tol = Tol::witness();
    let mut b = b.clone();
    for f in frame {
        b = finished("moved", topo::transform_rigid(&b, f, tol).expect("rigid"), tol);
    }
    b
}

fn cap(theta: f64, r: f64) -> f64 {
    // same oracle as the PR row, by plain midpoint on a fine grid scaled by r
    let (c, s) = (theta.cos(), theta.sin());
    let n = 2000;
    let mut acc = 0.0;
    for i in 0..n {
        let a = -0.25 + 0.5 * (i as f64 + 0.5) / n as f64;
        for j in 0..n / 2 {
            let b = -0.15 + 0.3 * (j as f64 + 0.5) / (n / 2) as f64;
            let y = 0.45 + a * c - b * s;
            acc += 2.0 * (1.0 - y * y).sqrt();
        }
    }
    acc * (0.5 / n as f64) * (0.3 / (n / 2) as f64) * r * r * r
}

fn run(theta: f64, r: f64, frame: &[Affine3<f64>], label: &str) -> Vec<String> {
    let tol = Tol::witness();
    let v_pipe = 4.0 * PI * r * r * r;
    let v_bar = 2.2 * 0.5 * 0.3 * r * r * r;
    let pipe = pipe_r(r);
    let bar = brick((-1.1 * r, 1.1 * r), (0.2 * r, 0.7 * r), (-0.15 * r, 0.15 * r), tol);
    let bar = topo::transform_rigid(
        &bar,
        &Affine3::rotation_about_axis(Point3::new(0.0, 0.45 * r, 0.0), Vec3::new(1.0, 0.0, 0.0), theta),
        tol,
    )
    .unwrap();
    let bar = finished("bar", bar, tol);
    let (pipe, bar) = (moved(&pipe, frame), moved(&bar, frame));
    let vc = cap(theta, r);
    let mut out = Vec::new();
    for (op, res, exact) in [
        ("∪", topo::union(&pipe, &bar, tol), v_pipe + v_bar - vc),
        ("pipe∖bar", topo::subtract(&pipe, &bar, tol), v_pipe - vc),
    ] {
        let line = match res {
            Ok(topo::BooleanResult::Body(b)) => match topo::mass_properties(&b.body, tol) {
                Ok(m) => {
                    let err = (m.volume - exact).abs();
                    let ok = err <= m.volume_pad + 1e-7 * exact;
                    format!("{label} θ={theta} r={r} {op}: {} vol {} exact {exact} err {err:.3e} pad {:.3e}",
                        if ok { "OK" } else { "WRONG" }, m.volume, m.volume_pad)
                }
                Err(e) => format!("{label} θ={theta} r={r} {op}: MP-REFUSED {e:?}"),
            },
            Ok(_) => format!("{label} θ={theta} r={r} {op}: EMPTY"),
            Err(e) => format!("{label} θ={theta} r={r} {op}: BOOL-REFUSED {e:?}"),
        };
        eprintln!("{line}");
        out.push(line);
    }
    out
}

#[test]
fn probe_tilted_offorigin_frames() {
    let tilt = Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 2.0, 3.0), 0.9);
    let shift = Affine3::translation(Vec3::new(5.0, -7.0, 2.0));
    let mut wrong = Vec::new();
    for theta in [0.3, 0.7, 1.3] {
        for (label, frame) in [
            ("id", vec![]),
            ("tilt", vec![tilt]),
            ("tilt+shift", vec![tilt, shift]),
        ] {
            for l in run(theta, 1.0, &frame, label) {
                if l.contains("WRONG") { wrong.push(l); }
            }
        }
    }
    for r in [0.05, 0.01, 20.0] {
        for l in run(0.7, r, &[tilt, shift], "scaled") {
            if l.contains("WRONG") { wrong.push(l); }
        }
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}
