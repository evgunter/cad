//! R2 review probe: the BOOLEAN door (`topo::boolean_op_with`), whose
//! (Plane, Sphere) germ arm is wired since M5 S13 and which MESH-3's
//! no-route verdict does not enumerate.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;
use common::*;
use geom_core::{Point2, Tol};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::{BooleanDeclarations, BooleanOp, boolean_op_with};

fn slab(y0: f64) -> topo::Body<f64> {
    let lp = ProfileLoop::new(
        [(-2.0, -2.0), (2.0, -2.0), (2.0, y0), (-2.0, y0)]
            .into_iter()
            .map(|(x, y)| ProfileVertex::new(Point2::new(x, y), 0.0))
            .collect(),
    );
    let p = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&p, Extrusion::Distance(2.0), Tol::witness())
        .unwrap()
        .body
}

#[test]
fn r2_bool_door_near_pole() {
    let eps = Tol::witness().get().eps;
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
            Err(e) => lines.push(format!("rho={rho:.3e}: boolean refused {e:?}")),
            Ok(br) => {
                let Some(bb) = br.body() else {
                    lines.push(format!("rho={rho:.3e}: boolean EMPTY"));
                    continue;
                };
                let b = bb.body.clone();
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
    panic!("R2 BOOL DOOR\n{}", lines.join("\n"));
}
