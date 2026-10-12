//! **A whole spiric oval bounds a face on its torus's mirror**, through
//! public Euler doors alone. Reflecting a torus through a plane
//! parallel to its axis gives the one other torus that holds the
//! plane's section: `mvfs` seeds a lone vertex on the oval,
//! `set_face_surface` puts the face on the mirror torus, and `mef` at
//! the lone loop closes one edge on the whole oval, described in that
//! chart. The mint then stores both halves' wall rows — the azimuth
//! sign opposed to the minor angle's, which is what names the mirror —
//! and tier 3 reads the body clean, with the chart axis either way.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, Pcurve, SpiricImage};
use geom_core::{Band, Point3, Tol, Vec3};
use topo::{Body, FaceSurface, MefSite};

#[test]
fn a_whole_spiric_oval_closes_one_edge_on_its_mirror_torus() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let (big, r, d) = (0.09375_f64, 0.0703125_f64, 0.0078125_f64);
    let (n, a) = (Vec3::unit_x(), Vec3::unit_y());
    let carrier = geom::Curve3::spiric(Point3::origin(), a, n, big, r, d, band).unwrap();
    for chart_axis in [a, -a] {
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(carrier.eval(0.0), true).unwrap();
        let mirror = body
            .set_face_surface(
                seed.face,
                FaceSurface::New {
                    surface: geom::Surface::Torus {
                        center: Point3::origin() + n * (2.0 * d),
                        axis: chart_axis,
                        major_radius: big,
                        minor_radius: r,
                        u_ref: n,
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
                EdgeCurveSpec {
                    description: EdgeDescriptionSpec::chart(mirror),
                    carrier: carrier.clone(),
                    param_start: 0.0,
                    param_end: core::f64::consts::TAU,
                },
                FaceSurface::Inherit,
                tol,
            )
            .unwrap_or_else(|e| panic!("axis {chart_axis:?}: the closed oval edge: {e}"));
        topo::mint_pcurves(&mut body, tol)
            .unwrap_or_else(|e| panic!("axis {chart_axis:?}: the mint refuses: {e}"));
        let sigma = chart_axis.dot(a);
        for he in [made.he_plus, made.he_minus] {
            let row = body
                .pcurve(he)
                .unwrap_or_else(|| panic!("axis {chart_axis:?}: {he:?} has no row"));
            let Pcurve::Spiric {
                image:
                    SpiricImage::Wall {
                        u_sense, v_sense, ..
                    },
                ..
            } = *row.pcurve()
            else {
                panic!("axis {chart_axis:?}: a wall row: {:?}", row.pcurve());
            };
            assert_eq!(
                (u_sense, v_sense),
                (-sigma, sigma),
                "axis {chart_axis:?}: the mirror's signs"
            );
        }
        let findings = topo::pcurves::validate_pcurves(&body, band);
        assert!(
            findings.is_empty(),
            "axis {chart_axis:?}: tier 3 reads the body clean: {findings:?}"
        );
    }
}
