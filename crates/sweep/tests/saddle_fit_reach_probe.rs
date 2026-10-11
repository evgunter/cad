//! Probe: how far the twisted loft's wall offset fits get toward ε,
//! per wall, per signed distance, per target. Measurement only.
//! Run: cargo nextest run -p sweep --test all saddle_fit_reach_probe --run-ignored only --no-capture
//! (PROBE_ALL_WALLS=1 for all four walls; they measure the same by symmetry.)

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::approx::{band, nurbs_walls, twisted_loft};
use geom_brep::offset_fit::fit_offset_at;

#[test]
#[ignore = "measurement probe; prints, asserts nothing"]
fn saddle_fit_reach_probe() {
    let body = twisted_loft(0.3);
    for (i, (face, wall)) in nurbs_walls(&body).iter().enumerate() {
        eprintln!(
            "wall {i} {face:?}: degree ({}, {}), ctrl {:?}, u knots {:?}, v knots {:?}",
            wall.knots_u().degree(),
            wall.knots_v().degree(),
            wall.control_counts(),
            wall.knots_u().knots(),
            wall.knots_v().knots(),
        );
        if i > 0 && std::env::var("PROBE_ALL_WALLS").is_err() {
            continue;
        }
        for d in [0.05_f64, -0.05, 0.01, -0.01] {
            for tol in [1e-9_f64, 1e-12] {
                eprintln!("== wall {i} d {d} tol {tol:e}");
                match fit_offset_at(wall, d, tol, band()) {
                    Ok((_, cert)) => eprintln!(
                        "RESULT wall {i} d {d} tol {tol:e}: certified, hull_sup {:e} rounds {}",
                        cert.hull_sup, cert.rounds
                    ),
                    Err(e) => eprintln!("RESULT wall {i} d {d} tol {tol:e}: {e:?}"),
                }
            }
        }
    }
}
