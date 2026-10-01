//! D36 review probes: carriers that LIE ON their chart, built exactly,
//! handed to `chart_pcurve`, and checked against the split's buckets.
//! The claim under test is the PR's "every OFF-CHART site is truly off
//! the chart": a misbucketed on-chart carrier turns a valid body loud
//! at the mint.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::tol::{band, eps};
use geom::{Curve3, Surface};
use geom_brep::{PcurveCertifyError, chart_pcurve};
use geom_core::{Point3, Tol, Vec3};

/// A small circle exactly on the unit sphere near its pole, whose plane
/// is tilted off ⊥-the-axis by an angle `θ` small enough that the
/// polar class's axial gate (`|a·z| + |b·z| = ρ·sin θ`) reads ZERO, but
/// whose centre's radial offset (`h·sin θ`, `h ≈ R`) is `h/ρ` times
/// larger and reads nonzero. The circle lies on the sphere (its centre
/// is the foot of the sphere's centre on its plane, `ρ² + h² = R²`), so
/// any `CarrierOffChart` here refuses a valid edge.
#[test]
fn a_near_pole_small_circle_tilted_inside_the_axial_band_is_on_the_sphere() {
    let (e, k) = (eps(), Tol::witness().k());
    let sphere = Surface::Sphere {
        center: Point3::origin(),
        radius: 1.0,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    for rho in [1e-2_f64, 1e-3, 1e-4] {
        let h = (1.0 - rho * rho).sqrt();
        // ρ·sin θ = ε/2: inside the axial gate's zero half.
        let s = 0.5 * e / rho;
        let c = (1.0 - s * s).sqrt();
        let n = Vec3::new(s, 0.0, c);
        let carrier = Curve3::Circle {
            center: Point3::new(h * s, 0.0, h * c),
            axis: n,
            radius: rho,
            u_ref: Vec3::new(c, 0.0, -s),
        };
        // Measured on the sphere.
        let worst = (0..64)
            .map(|i| {
                let p = carrier.eval(f64::from(i) * core::f64::consts::TAU / 64.0);
                ((p.x * p.x + p.y * p.y + p.z * p.z).sqrt() - 1.0).abs()
            })
            .fold(0.0_f64, f64::max);
        assert!(worst < 1e-14, "on the sphere: {worst:e}");
        let radial_offset = h * s;
        let got = chart_pcurve(&carrier, &sphere, band());
        eprintln!(
            "rho={rho:e} eps={e:e} K={k} axial={:e} centre-radial={radial_offset:e} -> {}",
            rho * s,
            match &got {
                Ok(_) => "Ok(image)".to_string(),
                Err(err) => format!("{err:?}"),
            }
        );
        assert!(
            !matches!(got, Err(PcurveCertifyError::CarrierOffChart { .. })),
            "rho = {rho:e}: an on-sphere circle was called off the chart: {got:?}"
        );
    }
}
