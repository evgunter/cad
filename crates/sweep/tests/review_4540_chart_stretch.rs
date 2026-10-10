//! Review probe for PR 4540: the sibling inf producer
//! `geom_brep::chart_stretch_inf` on the swaying loft's NURBS walls.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Tol, Vec3};
use profile::{ProfileLoop, RawLoop};
use sweep::{Section, loft_body};

fn swaying(a: f64) -> (Vec<Section>, Vec<Affine3<f64>>) {
    let square = || {
        vec![ProfileLoop::polygon(
            [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]
                .iter()
                .map(|&(x, y)| Point2::new(x, y)),
        )]
    };
    let places = [-a, a, -a, a]
        .into_iter()
        .zip([0.0, 1.0, 2.0, 3.0])
        .map(|(dx, z)| Affine3::translation(Vec3::new(dx, 0.0, z)))
        .collect();
    (vec![square(), square(), square(), square()], places)
}

#[test]
fn review_4540_wall_chart_stretch_inf() {
    for (vd, a) in [(2, 0.25), (2, 0.75), (2, 1.0), (2, 2.0), (3, 0.5)] {
        let (s, p) = swaying(a);
        let lofted = loft_body::<f64>(&s, &p, vd, Tol::witness()).unwrap();
        let body = &lofted.body;
        for (_, face) in body.faces() {
            let surf = body.get_surface(face.surface).expect("surface");
            if let geom::Surface::Nurbs(_) = surf {
                let inf = geom_brep::chart_stretch_inf(surf);
                println!(
                    "CHART vd{vd} a{a}: inf_u {:e} inf_v {:e} area_inf {:e}",
                    inf.inf_u, inf.inf_v, inf.area_inf
                );
            }
        }
    }
}
