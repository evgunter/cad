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

/// Review delta (PR 5244, H3): a Villarceau edge and a spiric wall edge
/// whose span runs past one period, through the Euler doors. The pcurve
/// period gates are gone; the edge certificate's `interval_span_winding`
/// is what should refuse, before any row is minted.
#[test]
fn review_delta_spans_past_a_period_through_a_body() {
    use geom_brep::{EdgeDescriptionSpec, WallTorus};
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let tau = core::f64::consts::TAU;
    let (big, r) = (2.0_f64, 0.7_f64);
    let (x, z) = (Vec3::unit_x(), Vec3::unit_z());
    let tilt = (r / big).asin();
    let lean = x.cross(z) * tilt.cos() + z * tilt.sin();
    let center = Point3::new(r, 0.0, 0.0);
    let carrier = geom::Curve3::Circle {
        center,
        axis: x.cross(lean),
        radius: big,
        u_ref: x,
    };
    let mut out = Vec::new();
    for extra in [0.5, tau, 0.1 * band.zero() / big, 0.5 * band.escalate() / big, 3.0 * band.escalate() / big] {
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
        let made = body.mef(
            MefSite::Lone { r#loop: seed.r#loop },
            EdgeCurveSpec::arc_of_circle(carrier.clone(), 0.0, tau + extra).unwrap(),
            FaceSurface::Inherit,
            tol,
        );
        let line = match made {
            Err(e) => format!("villarceau τ+{extra:e}: mef refuses: {e}"),
            Ok(m) => {
                let mint = topo::mint_pcurves(&mut body, tol);
                let rows: Vec<_> = [m.he_plus, m.he_minus]
                    .iter()
                    .map(|he| body.pcurve(*he).map(|c| format!("{:?}", core::mem::discriminant(c.pcurve()))))
                    .collect();
                let findings = topo::pcurves::validate_pcurves(&body, band);
                format!("villarceau τ+{extra:e}: mef ok; mint {:?}; rows {rows:?}; findings {findings:?}", mint.map(|_| ()))
            }
        };
        eprintln!("{line}");
        out.push(line);
    }
    // the spiric wall on its mirror torus
    let (big, r, d) = (0.09375_f64, 0.0703125_f64, 0.0078125_f64);
    let (n, a) = (Vec3::<f64>::unit_x(), Vec3::<f64>::unit_y());
    let carrier = geom::Curve3::spiric(Point3::origin(), a, n, big, r, d, band).unwrap();
    for extra in [0.5, tau, 0.1 * band.zero() / r, 0.5 * band.escalate() / r, 3.0 * band.escalate() / r] {
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(carrier.eval(0.0), true).unwrap();
        let mirror = body
            .set_face_surface(
                seed.face,
                FaceSurface::New {
                    surface: geom::Surface::Torus {
                        center: WallTorus::Mirror.center(Point3::origin(), n, d),
                        axis: a,
                        major_radius: big,
                        minor_radius: r,
                        u_ref: n,
                    },
                    sense: true,
                },
            )
            .unwrap();
        let made = body.mef(
            MefSite::Lone { r#loop: seed.r#loop },
            EdgeCurveSpec {
                description: EdgeDescriptionSpec::chart(mirror),
                carrier: carrier.clone(),
                param_start: 0.0,
                param_end: tau + extra,
            },
            FaceSurface::Inherit,
            tol,
        );
        let line = match made {
            Err(e) => format!("spiric τ+{extra:e}: mef refuses: {e}"),
            Ok(m) => {
                let mint = topo::mint_pcurves(&mut body, tol);
                let rows: Vec<_> = [m.he_plus, m.he_minus]
                    .iter()
                    .map(|he| body.pcurve(*he).map(|c| format!("{:?}", c.pcurve()).chars().take(60).collect::<String>()))
                    .collect();
                let findings = topo::pcurves::validate_pcurves(&body, band);
                format!("spiric τ+{extra:e}: mef ok; mint {:?}; rows {rows:?}; findings {findings:?}", mint.map(|_| ()))
            }
        };
        eprintln!("{line}");
        out.push(line);
    }
    for l in &out {
        if l.contains("τ+5e-1") || l.contains("τ+6.28") {
            assert!(l.contains("refuses"), "{l}");
        }
    }
}
