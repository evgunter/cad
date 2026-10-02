//! probe
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use geom_core::{Affine3, Point2, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::{Body, BooleanOp};

fn ball(r: f64, c: Vec3<f64>) -> Body<f64> {
    let b = revolved_about_y(
        vec![(Point2::new(0.0, -r), 1.0), (Point2::new(0.0, r), 0.0)],
        Revolution::Full,
        Tol::witness(),
    );
    topo::transform_rigid(&b, &Affine3::translation(c), Tol::witness()).unwrap()
}

#[test]
fn probe() {
    let a = ball(1.0, Vec3::new(2.0, 2.0, 0.5));
    let b = ball(1.0, Vec3::new(3.4, 2.0, 0.5));
    for (label, op, x, y) in [
        ("A u B", BooleanOp::Union, &a, &b),
        ("A n B", BooleanOp::Intersect, &a, &b),
        ("A - B", BooleanOp::Subtract, &a, &b),
        ("B - A", BooleanOp::Subtract, &b, &a),
    ] {
        let out = match op {
            BooleanOp::Union => topo::boolean::union(x, y, Tol::witness()),
            BooleanOp::Intersect => topo::boolean::intersect(x, y, Tol::witness()),
            BooleanOp::Subtract => topo::boolean::subtract(x, y, Tol::witness()),
        };
        let out = match out {
            Ok(o) => o,
            Err(e) => {
                eprintln!("PROBE {label}: refused {e:?}");
                continue;
            }
        };
        let body = &out.body().unwrap().body;
        eprintln!("PROBE {label}: faces {}", body.faces().count());
        eprintln!("PROBE {label}: validate {:?}", topo::validate(body));
        eprintln!("PROBE {label}: closed {:?}", topo::validate_closed(body));
        eprintln!("PROBE {label}: geometric {:?}", topo::validate_geometric(body, Tol::witness()));
        match mesh::tessellate(body, 1e-4, Tol::witness()) {
            Ok(m) => eprintln!("PROBE {label}: mesh {:?} vol {}", mesh::validate::check_mesh(&m), mesh::validate::signed_volume(&m)),
            Err(e) => eprintln!("PROBE {label}: mesh err {e:?}"),
        }
        eprintln!("PROBE {label}: props {:?}", topo::mass_properties(body, Tol::witness()).map(|p| (p.volume, p.volume_pad)));
    }
    let h = 0.3;
    let cap = PI * h * h * (3.0 - h) / 3.0;
    eprintln!("PROBE want lens {} union {} diff {}", 2.0 * cap, 8.0 * PI / 3.0 - 2.0 * cap, 4.0 * PI / 3.0 - 2.0 * cap);
}
