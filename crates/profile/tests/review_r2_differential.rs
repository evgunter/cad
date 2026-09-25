//! R2's own two-tree differential over the fillet doors.
//!
//! Drives a grid of fillet requests through the public doors and dumps,
//! per request, either the built loop's vertex positions and bulges AS
//! BITS plus its declared tangent joints and its `Profile::validate`
//! verdict, or the refusal's `Debug` (variant and payload). Nothing here
//! reads a `Display`, so the dump is invariant under a rendered-text
//! change and any diff between two trees is a behaviour change.
//!
//! Ignored by default; run with `CAD_R2_DUMP=<path>`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::fmt::Write as _;

use geom_core::{Point2, Tol};
use profile::{ArcSweep, Center, Open, PathError, Profile, ProfileLoop, SketchPlane, Start};

fn tol() -> Tol {
    Tol::witness()
}

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn record(out: &mut String, key: &str, lp: &Result<ProfileLoop<f64>, PathError<f64>>) {
    match lp {
        Ok(lp) => {
            let _ = write!(out, "{key} BUILT joints={:?}", lp.tangent_joints());
            for (v, b) in lp.vertices().iter().zip(lp.bulges()) {
                let _ = write!(
                    out,
                    " [{:016x},{:016x},{:016x}]",
                    v.x.to_bits(),
                    v.y.to_bits(),
                    b.to_bits()
                );
            }
            let verdict = match Profile::new(SketchPlane::xy(), vec![lp.clone()]).validate(tol()) {
                Ok(_) => "ok".to_string(),
                Err(e) => format!("{e:?}"),
            };
            let _ = writeln!(out, " validate={verdict}");
        }
        Err(e) => {
            let _ = writeln!(out, "{key} REFUSED {e:?}");
        }
    }
}

fn line_arc(radius: f64, carrier: f64) -> Result<ProfileLoop<f64>, PathError<f64>> {
    Open.at(p2(0.0, carrier))
        .line_to(p2(0.0, 0.0), tol())?
        .toward(carrier, 0.0, tol())?
        .fillet_arc(
            radius,
            Center {
                c: p2(0.0, 0.0),
                winding: ArcSweep::Ccw,
                p: Start,
            },
            tol(),
        )
        .map(|c| c.loop_)
}

fn lobes(r_carrier: f64, d: f64, radius: f64) -> Result<ProfileLoop<f64>, PathError<f64>> {
    let h = (r_carrier * r_carrier - 0.25 * d * d).sqrt();
    Open.arc_fillet_arc(
        Center {
            c: p2(-0.5 * d, 0.0),
            winding: ArcSweep::Ccw,
            p: p2(0.0, -h),
        },
        radius,
        Center {
            c: p2(0.5 * d, 0.0),
            winding: ArcSweep::Ccw,
            p: Start,
        },
        tol(),
    )
    .map(|c| c.loop_)
}

fn mixed(r_carrier: f64, radius: f64) -> Result<ProfileLoop<f64>, PathError<f64>> {
    Open.arc_fillet_arc(
        Center {
            c: p2(-0.5 * r_carrier, 0.0),
            winding: ArcSweep::Ccw,
            p: p2(0.5 * r_carrier, 0.0),
        },
        radius,
        Center {
            c: p2(0.5 * r_carrier, 0.0),
            winding: ArcSweep::Cw,
            p: p2(1.5 * r_carrier, 0.0),
        },
        tol(),
    )?
    .line_to(Start, tol())
    .map(|closed| closed.loop_)
}

fn bend(start_x: f64, theta: f64, radius: f64) -> Result<ProfileLoop<f64>, PathError<f64>> {
    let anchor = p2(4.0 + 3.0 * theta.cos(), 3.0 * theta.sin());
    Open.at(p2(start_x, 0.0))
        .angle(0.0, tol())?
        .fillet(radius, tol())?
        .at(anchor, tol())?
        .angle(theta, tol())?
        .line(1.0, tol())?
        .line_to(Start, tol())
        .map(|c| c.loop_)
}

fn corner_out(
    r_carrier: f64,
    sin_turn: f64,
    arm: f64,
    radius: f64,
) -> Result<ProfileLoop<f64>, PathError<f64>> {
    let corner = p2(r_carrier, 0.0);
    let dir = p2(sin_turn, (1.0 - sin_turn * sin_turn).sqrt());
    let start = p2(corner.x - arm * dir.x, corner.y - arm * dir.y);
    Open.at(start)
        .toward(dir.x, dir.y, tol())?
        .fillet_arc(
            radius,
            Center {
                c: p2(0.0, 0.0),
                winding: ArcSweep::Ccw,
                p: p2(0.0, r_carrier),
            },
            tol(),
        )?
        .line_to(Start, tol())
        .map(|c| c.loop_)
}

#[test]
#[ignore = "differential harness; set CAD_R2_DUMP"]
fn r2_fillet_differential_dump() {
    let Ok(path) = std::env::var("CAD_R2_DUMP") else {
        panic!("set CAD_R2_DUMP to the dump path");
    };
    let eps = tol().eps();
    let mut out = String::new();

    // The four families, each swept across the band-relative window and
    // across ordinary radii.
    for i in 0..121 {
        let t = f64::from(i);
        for (name, carrier) in [("line_arc2", 2.0), ("line_arc3", 3.0), ("line_arc9", 9.0)] {
            let r = 0.5 * carrier * (1.0 + (t - 60.0) * 2.5 * eps) - 0.004 * t;
            record(&mut out, &format!("{name}/{i}"), &line_arc(r, carrier));
        }
        for (name, rc, d) in [("lobes1", 1.0, 1.0), ("lobes2", 2.0, 2.0)] {
            let r = 0.5 * rc * (1.0 + (t - 60.0) * 2.5 * eps) - 0.003 * t;
            record(&mut out, &format!("{name}/{i}"), &lobes(rc, d, r));
            let r_enc = rc * (1.0 + (t - 60.0) * 2.5 * eps);
            record(&mut out, &format!("{name}enc/{i}"), &lobes(rc, d, r_enc));
        }
        let r = 1.5 * (1.0 + (t - 60.0) * 2.5 * eps) - 0.005 * t;
        record(&mut out, &format!("mixed/{i}"), &mixed(3.0, r));
    }
    // The line x line door, across the start-x and turn grid.
    for i in 0..24 {
        for j in 0..12 {
            let start_x = -2.0 + 0.25 * f64::from(i);
            let theta = 0.05 + 0.25 * f64::from(j);
            for radius in [0.05, 0.5, 2.0] {
                record(
                    &mut out,
                    &format!("bend/{i}/{j}/{radius}"),
                    &bend(start_x, theta, radius),
                );
            }
        }
    }
    // The short-leg corners, where the turn gate lives.
    for i in 0..60 {
        let arm = (1.0 + 0.5 * f64::from(i)) * eps;
        for sin_turn in [0.9, 0.5, 0.05] {
            record(
                &mut out,
                &format!("short/{i}/{sin_turn}"),
                &corner_out(3.0, sin_turn, arm, 0.3),
            );
        }
    }
    // The scene-scaled conditioning corners.
    for i in 0..60 {
        let scale = 1e3 * (1.0 + 0.4 * f64::from(i));
        let rho = 5.01;
        record(
            &mut out,
            &format!("lever/{i}"),
            &lobes(scale, 10.0, scale - rho),
        );
    }
    std::fs::write(&path, out).expect("the dump is writable");
}
