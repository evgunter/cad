//! R2 review probe, adopted and converted to a pin: the BOOLEAN door
//! (`topo::boolean_op_with`, the (Plane, Sphere) germ arm wired since
//! M5 S13). Measured: a near-pole slab intersect whose cut lies within
//! a few ε of the pole refuses, so this door cannot mint issue 896's
//! guard state; one cut a macroscopic distance below it builds the
//! sphere face the cut leaves, meets its closed form and meshes with
//! the guard quiet. Part of the door enumeration whose single home is
//! `step-import/tests/poleguard.rs`.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout
)]

use crate::common;
use common::*;
use geom_core::Tol;
use topo::{BooleanDeclarations, BooleanOp, boolean_op_with};

fn slab(y0: f64) -> topo::Body<f64> {
    sweep::test_support::brick((-2.0, 2.0), (-2.0, y0), (0.0, 2.0), Tol::witness())
}

#[test]
fn r2_bool_door_near_pole() {
    let eps = common::eps();
    let mut lines = vec![format!("eps = {eps:e}")];
    for rho in [0.9 * eps, 5.0 * eps, 1e-6, 1e-3, 0.1] {
        let y0 = (1.0f64 - rho * rho).sqrt();
        let r = boolean_op_with(
            BooleanOp::Intersect,
            &ball(),
            &slab(y0),
            &BooleanDeclarations::default(),
            topo::SweepStrategy::Realized,
            Tol::witness(),
        );
        match r {
            Err(e) => {
                // The pinned invariant is the REFUSAL: the shape is
                // `CurvedPierceUnsupported` at the default band and
                // an earlier escalation at coarser bands (measured
                // `split_conic_crossing_root` at 1e-6); the door
                // being shut is band-invariant, its bar is not.
                lines.push(format!("rho={rho:.3e}: boolean refused {e:?}"));
            }
            Ok(br) => {
                let Some(bb) = br.body() else {
                    lines.push(format!("rho={rho:.3e}: boolean EMPTY"));
                    continue;
                };
                let b = bb.body.clone();
                // The half ball `z ≥ 0` less the half of its cap above
                // `y = y0`.
                let h = 1.0 - y0;
                let want = (4.0 / 3.0 - h * h * (3.0 - h) / 3.0) * core::f64::consts::PI / 2.0;
                let got = topo::mass_properties(&b, Tol::witness())
                    .unwrap_or_else(|e| panic!("rho={rho:.3e}: mass properties, got {e:?}"))
                    .volume;
                assert!(
                    (got - want).abs() <= 1e-9,
                    "rho={rho:.3e}: volume {got} against the closed form {want}"
                );
                let t = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    mesh::tessellate(&b, 0.05, Tol::witness())
                }));
                match t {
                    Ok(Ok(m)) => lines.push(format!(
                        "rho={rho:.3e}: BOOL OK, meshed {} positions, guard QUIET",
                        m.positions.len()
                    )),
                    Ok(Err(e)) => {
                        lines.push(format!("rho={rho:.3e}: BOOL OK, tessellate refused {e:?}"))
                    }
                    Err(p) => {
                        let s = p
                            .downcast_ref::<String>()
                            .cloned()
                            .or_else(|| p.downcast_ref::<&str>().map(|s| (*s).to_string()))
                            .unwrap_or_default();
                        lines.push(format!(
                            "rho={rho:.3e}: BOOL OK, *** PANIC *** {}",
                            &s[..s.len().min(240)]
                        ));
                    }
                }
            }
        }
    }
    println!("R2 BOOL DOOR\n{}", lines.join("\n"));
    // The first two rows cut within 5ε of the pole: the door stays shut
    // there. A row it admits meshes with the guard quiet.
    for (i, l) in lines.iter().filter(|l| l.starts_with("rho=")).enumerate() {
        if i < 2 {
            assert!(
                l.contains("boolean refused"),
                "the boolean door admitted a body within the band of the pole — the \
                 issue-896 route question must be re-asked: {l}"
            );
        } else {
            assert!(
                l.contains("boolean refused") || l.contains("guard QUIET"),
                "a body the boolean door admitted does not mesh quietly: {l}"
            );
        }
    }
}
