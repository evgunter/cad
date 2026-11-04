//! Review probe (PR 6485): an on-cone ruling read near the apex. Not for merge.
use geom::{Curve3, Surface};
use geom_core::{Band, Point3, Tol, Vec3};
use geom_brep::recourse::Reading;

#[test]
fn review_probe_6485_ruling_near_the_apex() {
    let band = Band::linear(Tol::witness()).unwrap();
    let line = Curve3::line(
        Point3::new(1.239792126505943e-10, -9.746673200094881e-10, -1.939344479653521e-9),
        Vec3::new(-0.031367901511575216, 0.38332687822435824, 0.9230799310923882),
    );
    let cone = Surface::Cone {
        apex: Point3::new(0.0, -0.0, 0.0),
        axis: Vec3::new(0.036582851423955946, -0.3691082383934535, -0.9286661420186344),
        half_angle: 0.016142392389299435,
        u_ref: Vec3::new(-0.7808453755737287, -0.590498825246728, 0.203940277599489),
    };
    let (t0, t1) = (0.0009067415940370447, 2.4187216369829954);
    let got = geom_brep::chart_pcurve_over(&line, t0, t1, &cone, band);
    println!("[apex] mint: {:?}", got.as_ref().map(|p| format!("{p:?}").chars().take(40).collect::<String>()));
    if let Err(e) = &got {
        println!("[apex] mint ending at build: {}", e.ending(Reading::Build));
    }
    if let Ok(image) = got {
        let cert = geom_brep::PcurveCache::certify(image, t0, t1, &line, &cone, band);
        match cert {
            Ok(c) => println!("[apex] certified, envelope {:e}", c.certificate().envelope),
            Err(e) => println!("[apex] certify: {e:?}\n[apex] ending at build: {}", e.ending(Reading::Build)),
        }
    }
}
