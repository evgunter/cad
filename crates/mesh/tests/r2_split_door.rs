//! R2 review probe, adopted and converted to a pin: the plane-SPLIT
//! door. Cut a unit ball just below its north pole; the BELOW piece
//! would be a sphere face whose boundary excludes the pole with a top
//! rim of radius `rho` — issue 896's motivating shape. Measured: the
//! split door refuses EVERY sphere-face cut (`CurvedBooleanUnsupported`,
//! rho-independent — even macroscopic rims), so it cannot mint the
//! guard's state at all. Part of the door enumeration whose single
//! home is `step-import/tests/poleguard.rs`.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout
)]

mod common;
use common::*;
use geom_core::Tol;
use topo::splitting::{SplitPlane, split};

#[test]
fn r2_split_door_near_pole() {
    let eps = Tol::witness().get().eps;
    let mut lines = vec![format!("eps = {eps:e}")];
    for rho in [0.9 * eps, 5.0 * eps, 1e3 * eps, 1e-6, 1e-3, 0.01, 0.1] {
        let y0 = (1.0f64 - rho * rho).sqrt();
        let plane = SplitPlane {
            origin: geom_core::Point3::new(0.0, y0, 0.0),
            normal: geom_core::Vec3::new(0.0, 1.0, 0.0),
        };
        let r = split(&ball(), &plane, Tol::witness());
        match r {
            Err(e) => {
                let shape = format!("{e:?}");
                assert!(
                    shape.contains("CurvedBooleanUnsupported"),
                    "rho={rho:.3e}: the split door refuses sphere-face cuts typed; got {shape}"
                );
                lines.push(format!("rho={rho:.3e}: split refused {shape}"));
            }
            Ok(sr) => {
                let Some(b) = sr.below.body() else {
                    lines.push(format!("rho={rho:.3e}: below EMPTY"));
                    continue;
                };
                let t = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    mesh::tessellate(b, 0.05, Tol::witness())
                }));
                match t {
                    Ok(Ok(m)) => lines.push(format!(
                        "rho={rho:.3e}: SPLIT OK, meshed {} positions, guard QUIET",
                        m.positions.len()
                    )),
                    Ok(Err(e)) => {
                        lines.push(format!("rho={rho:.3e}: SPLIT OK, tessellate refused {e:?}"))
                    }
                    Err(p) => {
                        let s = p
                            .downcast_ref::<String>()
                            .cloned()
                            .or_else(|| p.downcast_ref::<&str>().map(|s| (*s).to_string()))
                            .unwrap_or_default();
                        lines.push(format!(
                            "rho={rho:.3e}: SPLIT OK, *** PANIC *** {}",
                            &s[..s.len().min(220)]
                        ));
                    }
                }
            }
        }
    }
    // Any admitted-and-meshed (or panicking) outcome above is a route
    // to the guard opening: fail loud rather than report quietly.
    for l in lines.iter().filter(|l| l.starts_with("rho=")) {
        assert!(
            l.contains("split refused"),
            "the split door admitted a cut — the issue-896 route question must be \
             re-asked: {l}"
        );
    }
    println!("R2 SPLIT DOOR\n{}", lines.join("\n"));
}
