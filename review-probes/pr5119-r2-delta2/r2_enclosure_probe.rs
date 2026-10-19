//! Review R2 (PR 5119, second delta): dump each probe body's planar faces
//! (stored normal, ring points) and the re-derivation's volume interval,
//! for an exact rational oracle outside Rust. Run with
//! `--ignored --nocapture r2_enclosure`.
#![allow(clippy::expect_used, clippy::panic, clippy::print_stdout, clippy::unwrap_used)]

use super::*;
use geom_core::{Bounds, Tol};

fn runs(body: &Body<f64>, band: Band, tol: Tol) -> Vec<FaceRun<f64>> {
    let faces: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
    let hook = reporting_hook(Some(QuadLane::<f64>::certified()));
    decide_faces_serially(&faces, |&f| face_flux(body, f, band, &hook, tol, RoundWindow::SCHEDULE))
        .unwrap()
}

fn rot(axis: [f64; 3], ang: f64) -> [[f64; 3]; 3] {
    let n = (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt();
    let (x, y, z) = (axis[0] / n, axis[1] / n, axis[2] / n);
    let (s, c) = ang.sin_cos();
    let k = 1.0 - c;
    [
        [c + x * x * k, x * y * k - z * s, x * z * k + y * s],
        [y * x * k + z * s, c + y * y * k, y * z * k - x * s],
        [z * x * k - y * s, z * y * k + x * s, c + z * z * k],
    ]
}

fn needle(w: f64, r: [[f64; 3]; 3], c: [f64; 3], tol: Tol) -> Body<f64> {
    crate::test_support_fixtures::mapped_cube::<f64>(
        move |x, y, z| {
            let p = [x, (y - 0.5) * w, (z - 0.5) * w];
            Point3::new(
                c[0] + r[0][0] * p[0] + r[0][1] * p[1] + r[0][2] * p[2],
                c[1] + r[1][0] * p[0] + r[1][1] * p[1] + r[1][2] * p[2],
                c[2] + r[2][0] * p[0] + r[2][1] * p[1] + r[2][2] * p[2],
            )
        },
        tol,
    )
}

#[test]
#[ignore = "review probe"]
fn r2_enclosure_dump() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let lane = QuadLane::<f64>::certified();
    let e = tol.eps();
    let poses: [([f64; 3], f64); 4] = [
        ([0.0, 0.0, 1.0], 0.0),
        ([1.0, 2.0, 3.0], 1.1),
        ([0.3, -0.7, 0.2], 2.4),
        ([1.0, 1.0, 1.0], 0.9553166181245093),
    ];
    let centres: [[f64; 3]; 3] = [[0.0, 0.0, 0.0], [37.0, -21.0, 13.0], [1234.5, -987.25, 3333.0]];
    for m in [10.5, 30.0] {
        for (i, (axis, ang)) in poses.iter().enumerate() {
            for (j, c) in centres.iter().enumerate() {
                for inverted in [false, true] {
                    let built = std::panic::catch_unwind(|| needle(m * e, rot(*axis, *ang), *c, tol));
                    let Ok(mut body) = built else {
                        println!("R2E m={m} pose={i} c={j} inv={inverted} BUILD-FAILED");
                        continue;
                    };
                    if inverted {
                        body = body.revert().expect("revert");
                    }
                    let rs = runs(&body, band, tol);
                    let got = rederive(&body, band, tol, lane, &rs, true);
                    let Ok(got) = got else {
                        println!("R2E m={m} pose={i} c={j} inv={inverted} REDERIVE-ERR {got:?}");
                        continue;
                    };
                    println!(
                        "R2E m={m} pose={i} c={j} inv={inverted} lo={:?} hi={:?} recentred={}",
                        got.volume.lo(),
                        got.volume.hi(),
                        got.recentred
                    );
                    for run in &rs {
                        let (face, surface) = resolve_face(&body, run.face);
                        let Surface::Plane { normal, .. } = surface else {
                            println!("R2F nonplanar");
                            continue;
                        };
                        let rings = vertex_rings(&body, surface, &flat_loops(&body, face).unwrap(), lane)
                            .unwrap()
                            .expect("line-bounded");
                        let pts: Vec<String> = rings
                            .iter()
                            .flatten()
                            .map(|p| format!("{:?},{:?},{:?}", p.x.lo(), p.y.lo(), p.z.lo()))
                            .collect();
                        println!(
                            "R2F n={:?},{:?},{:?} rings={} pts={}",
                            normal.x,
                            normal.y,
                            normal.z,
                            rings.len(),
                            pts.join(";")
                        );
                    }
                }
            }
        }
    }
}
