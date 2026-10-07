//! Probe (PR 4227 confirming review): a whole Villarceau circle as one
//! closed edge on a torus face, through public Euler doors only.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use geom_core::{Point3, Tol, Vec3};
use topo::Body;

#[test]
fn probe_whole_villarceau_loop_via_mvfs_and_mef_lone() {
    let tol = Tol::witness();
    let (big, r) = (2.0_f64, 0.7_f64);
    let z = Vec3::new(0.0, 0.0, 1.0);
    let x = Vec3::new(1.0, 0.0, 0.0);
    let tilt = (r / big).asin();
    let lean = x.cross(z) * tilt.cos() + z * tilt.sin();
    let n = x.cross(lean);
    let center = Point3::new(r, 0.0, 0.0);
    let start = center + x * big; // the outer-equator point (R + r, 0, 0)
    let carrier = geom::Curve3::Circle {
        center,
        axis: n,
        radius: big,
        u_ref: x,
    };
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(start, true).unwrap();
    body.set_face_surface(
        seed.face,
        topo::FaceSurface::New {
            surface: geom::Surface::Torus {
                center: Point3::new(0.0, 0.0, 0.0),
                axis: z,
                major_radius: big,
                minor_radius: r,
                u_ref: x,
            },
            sense: true,
        },
    )
    .unwrap();
    let got = body.mef(
        topo::MefSite::Lone {
            r#loop: seed.r#loop,
        },
        geom_brep::EdgeCurveSpec::arc_of_circle(carrier, 0.0, core::f64::consts::TAU).unwrap(),
        topo::FaceSurface::Inherit,
        tol,
    );
    println!("mef: {got:?}");
    if let Ok(made) = got {
        println!("plus row: {:?}", body.pcurve(made.he_plus));
        println!("minus row: {:?}", body.pcurve(made.he_minus));
        let band = geom_core::Band::linear(tol).unwrap();
        let m = topo::mint_pcurves(&mut body, tol);
        println!("mint: {m:?}");
        println!("plus row after mint: {:?}", body.pcurve(made.he_plus));
        println!("minus row after mint: {:?}", body.pcurve(made.he_minus));
        println!(
            "tier3 pcurves: {:?}",
            topo::pcurves::validate_pcurves(&body, band)
        );
    }
}
