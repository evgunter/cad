//! Review probes for PR 5008 (ignored): the wedge on a block's top at a
//! fixed tilt `PROBE_THETA` (m/m) under the run's ε (`CAD_TOLERANCE_EPS`),
//! every op's refusal, its predicate, margin and offered tolerance, one
//! line each, for a driver to re-run just below the offer.

#![allow(clippy::expect_used, clippy::panic, clippy::print_stderr)]

use geom_core::test_support::upper;
use geom_core::{Band, Tol};
use geom_core::{Point3, Vec3};
use topo::test_support::{brick, face_facing, finished, mapped_cube, wedge_on_a_block_top};

/// The wedge's apex on the block top's corner (0.5, 0.2, 1): the two
/// faces' sectors share the vertex, so the corner is `recl`'s
/// vertex-vertex `require_same`, with the block's bounds off the wedge's
/// plane by ~θ and the wedge's off the block's by θ·sin φ.
fn corner_pose(
    opening_deg: f64,
    tilt: f64,
    height: f64,
    depth: f64,
    tol: Tol,
) -> (topo::Body<f64>, topo::Body<f64>) {
    let phi = opening_deg.to_radians();
    let (ea, eb) = (
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(phi.cos(), phi.sin(), tilt * phi.sin()),
    );
    let p = Point3::new(0.5, 0.2, 1.0);
    (
        brick((0.5, 3.0), (0.2, 2.5), (0.0, 1.0), tol),
        mapped_cube(
            move |u, v, w| p + ea * u + eb * v + Vec3::new(0.0, 0.0, height * w - depth),
            tol,
        ),
    )
}

#[test]
#[ignore = "probe"]
fn probe_corner_names_glue() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("band");
    let theta: f64 = std::env::var("PROBE_THETA")
        .expect("PROBE_THETA")
        .parse()
        .expect("θ");
    let openings: Vec<f64> = std::env::var("PROBE_OPENINGS")
        .unwrap_or_else(|_| "2,5,20,60,90".into())
        .split(',')
        .map(|s| s.parse().expect("opening"))
        .collect();
    for (pose, h, d, facing) in [("standing", 1.0, 0.0, -1.0), ("sunk", 0.5, 0.5, 1.0)] {
        for &opening in &openings {
            let (block, tool) = if std::env::var("PROBE_CORNER").is_ok() {
                corner_pose(opening, theta, h, d, tol)
            } else {
                wedge_on_a_block_top(opening, theta, h, d, tol)
            };
            let (block, tool) = (finished("block", block, tol), finished("tool", tool, tol));
            let glue = match topo::flush_pair_relation(
                &block,
                face_facing(&block, 1.0),
                &tool,
                face_facing(&tool, facing),
                false,
                band,
            ) {
                Ok(Ok(rel)) => format!("{rel:?}"),
                Ok(Err(topo::PlaneEqError::Escalated { rung, diag })) => {
                    format!("Esc({rung:?} {:.3e})", upper(diag.margin))
                }
                other => format!("{other:?}"),
            };
            let ops = [
                ("AuB", topo::union(&block, &tool, tol).map(|_| ())),
                ("BuA", topo::union(&tool, &block, tol).map(|_| ())),
                ("AnB", topo::intersect(&block, &tool, tol).map(|_| ())),
                ("BnA", topo::intersect(&tool, &block, tol).map(|_| ())),
                ("A-B", topo::subtract(&block, &tool, tol).map(|_| ())),
                ("B-A", topo::subtract(&tool, &block, tol).map(|_| ())),
            ];
            for (op, out) in ops {
                let line = match out {
                    Ok(()) => "BUILD".to_string(),
                    Err(e) => {
                        let (key, defect) = topo::test_support::offer_key(&e);
                        let (pred, m) = match &e {
                            topo::BooleanError::Escalated { diag, .. } => {
                                (format!("{:?}", diag.predicate), upper(diag.margin))
                            }
                            _ => ("-".into(), f64::NAN),
                        };
                        let text = e.to_string();
                        if std::env::var("PROBE_TEXT").is_ok() {
                            eprintln!("TEXT {text}");
                        }
                        let offer = test_utils::offer::offered_below(&text)
                            .map_or("none".to_string(), |v| format!("{v:e}"));
                        format!(
                            "{key}{} pred={pred} m={m:.3e} offer={offer}",
                            if defect { " DEFECT" } else { "" }
                        )
                    }
                };
                eprintln!(
                    "CNG eps={:e} theta={theta:e} {pose} phi={opening} glue={glue} {op} {line}",
                    band.zero()
                );
            }
        }
    }
}
