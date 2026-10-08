//! **A whole Villarceau circle bounds a torus face**, through public
//! Euler doors alone (PR 4227's confirming review). `mvfs` seeds a lone
//! vertex on the torus's outer equator, `set_face_surface` puts the face
//! on the torus, and `mef` at the lone loop closes one edge on the whole
//! circle. The mint then stores both halves' focal-section rows — the
//! closed loop's joint is the `(±1, ±1)` winding the loop invariant
//! admits — and tier 3 reads the body clean, on either family.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::{EdgeCurveSpec, Pcurve};
use geom_core::{Band, Point3, Tol, Vec3};
use topo::{Body, FaceSurface, MefSite};

#[test]
fn a_whole_villarceau_circle_closes_one_edge_on_a_torus_face() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let (big, r) = (2.0_f64, 0.7_f64);
    let (x, z) = (Vec3::unit_x(), Vec3::unit_z());
    let tilt = (r / big).asin();
    for family in [1.0_f64, -1.0] {
        let lean = x.cross(z) * tilt.cos() + z * (family * tilt.sin());
        let center = Point3::new(r, 0.0, 0.0);
        let carrier = geom::Curve3::Circle {
            center,
            axis: x.cross(lean),
            radius: big,
            u_ref: x,
        };
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(center + x * big, true).unwrap();
        body.set_face_surface(
            seed.face,
            FaceSurface::New {
                surface: geom::Surface::Torus {
                    center: Point3::origin(),
                    axis: z,
                    major_radius: big,
                    minor_radius: r,
                    u_ref: x,
                },
                sense: true,
            },
        )
        .unwrap();
        let made = body
            .mef(
                MefSite::Lone {
                    r#loop: seed.r#loop,
                },
                EdgeCurveSpec::arc_of_circle(carrier, 0.0, core::f64::consts::TAU).unwrap(),
                FaceSurface::Inherit,
                tol,
            )
            .unwrap_or_else(|e| panic!("family {family}: the closed Villarceau edge: {e}"));
        topo::mint_pcurves(&mut body, tol)
            .unwrap_or_else(|e| panic!("family {family}: the mint refuses: {e}"));
        for he in [made.he_plus, made.he_minus] {
            let row = body
                .pcurve(he)
                .unwrap_or_else(|| panic!("family {family}: {he:?} has no row"));
            assert!(
                matches!(row.pcurve(), Pcurve::FocalSection(_)),
                "family {family}: a focal-section row: {:?}",
                row.pcurve()
            );
        }
        let findings = topo::pcurves::validate_pcurves(&body, band);
        assert!(
            findings.is_empty(),
            "family {family}: tier 3 reads the body clean: {findings:?}"
        );
    }
}
