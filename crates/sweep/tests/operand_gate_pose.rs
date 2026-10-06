//! Measurement probe: an unarmed face clear of a planar operand, the
//! pair turned together. Prints a table; asserts nothing yet.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::test_support::brick;
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{AtRestBody, Body, BooleanError, BooleanResult};

fn frustum(s: f64, tol: Tol) -> Body<f64> {
    let lp = bulge_loop(
        [(0.2, 0.0), (0.6, 0.0), (0.4, 0.6), (0.2, 0.6)]
            .into_iter()
            .map(|(r, y)| (Point2::new(r * s, y * s), 0.0))
            .collect(),
    );
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol)
        .unwrap();
    revolve(
        &profile,
        RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        tol,
    )
    .unwrap()
    .body
}

fn posed(what: &str, body: &Body<f64>, map: &Affine3<f64>, tol: Tol) -> AtRestBody<f64> {
    let b = topo::transform_rigid(body, map, tol).unwrap_or_else(|e| panic!("{what}: {e:?}"));
    topo::test_support::finished(what, b, tol)
}

fn verdict(r: Result<BooleanResult<f64>, BooleanError>) -> String {
    match r {
        Ok(BooleanResult::Body(b)) => {
            let v = topo::mass_properties(&b.body, Tol::witness())
                .map(|m| format!("{:.6}", m.volume))
                .unwrap_or_else(|e| format!("mp {e:?}"));
            format!("ok v={v}")
        }
        Ok(other) => format!("ok {:?}", std::mem::discriminant(&other)),
        Err(BooleanError::CurvedPairUnsupported { site, kind, .. }) => {
            format!("REFUSE {site:?} {kind:?}")
        }
        Err(e) => {
            let s = format!("{e:?}");
            format!("ERR {}", &s[..s.len().min(90)])
        }
    }
}

#[test]
fn measure_operand_gate_pose_sweep() {
    let tol = Tol::witness();
    let axes = [
        ("x", Vec3::new(1.0, 0.0, 0.0)),
        ("y", Vec3::new(0.0, 1.0, 0.0)),
        ("z", Vec3::new(0.0, 0.0, 1.0)),
        ("(1,2,3)", Vec3::new(1.0, 2.0, 3.0).normalize()),
    ];
    let angles = [0.0f64, 0.3, 0.7854, 1.2];
    for s in [0.1, 1.0, 10.0] {
        let cone = frustum(s, tol);
        let bar = brick::<f64>(
            (0.7 * s, 1.2 * s),
            (0.1 * s, 0.5 * s),
            (-0.2 * s, 0.2 * s),
            tol,
        );
        for (an, ax) in axes {
            for th in angles {
                let map = Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), ax, th);
                let a = posed("cone", &cone, &map, tol);
                let b = posed("bar", &bar, &map, tol);
                println!(
                    "eps={:e} s={s} axis={an} th={th}: U[{}] I[{}] A-B[{}] B-A[{}]",
                    tol.eps(),
                    verdict(topo::union(&a, &b, tol)),
                    verdict(topo::intersect(&a, &b, tol)),
                    verdict(topo::subtract(&a, &b, tol)),
                    verdict(topo::subtract(&b, &a, tol)),
                );
            }
        }
    }
}
