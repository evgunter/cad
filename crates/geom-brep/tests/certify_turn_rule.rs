//! **Certification's winding check is the turn rule** (`geom_core::period`):
//! an ellipse's extent is a whole turn only where the sup rate over the
//! turn's defect reads it within the band, and over one only where the
//! speed floor reads it definitely past.
//!
//! The fixture is the plane `z = 0` cutting a unit cylinder leaning
//! `acos(1/20)` in xz: the ellipse `a = 20`, `b = 1` about the origin, whose
//! speed is `b` at `θ = 0` and `a` at `θ = π/2`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::{
    CertifyError, EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec, StatedSpan, SurfaceKey,
    SurfacePair, WindingReading,
};
use geom_core::{Band, Point3, Vec3};
use slotmap::SlotMap;

const TAU: f64 = core::f64::consts::TAU;
const HALF_PI: f64 = core::f64::consts::FRAC_PI_2;

/// The plane, the leaning cylinder, and the ellipse they cut.
struct Steep {
    surfaces: SlotMap<SurfaceKey, Surface<f64>>,
    pair: SurfacePair,
    ellipse: Curve3<f64>,
}

fn steep() -> Steep {
    let alpha = (1.0f64 / 20.0).acos();
    let mut surfaces: SlotMap<SurfaceKey, Surface<f64>> = SlotMap::with_key();
    let plane = surfaces.insert(Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    });
    let cyl = surfaces.insert(Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::new(alpha.sin(), 0.0, alpha.cos()),
        radius: 1.0,
        u_ref: Vec3::new(0.0, 1.0, 0.0),
    });
    let ellipse = Curve3::Ellipse {
        center: Point3::origin(),
        axis: Vec3::new(0.0, 0.0, 1.0),
        semi_u: 1.0 / alpha.cos(),
        semi_v: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    Steep {
        surfaces,
        pair: SurfacePair::new(plane, cyl),
        ellipse,
    }
}

impl Steep {
    fn certify(&self, t0: f64, t1: f64, band: Band) -> Result<EdgeCurve<f64>, CertifyError> {
        let e = &self.ellipse;
        let spec = EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                pair: self.pair,
                witness: e.eval(0.5 * (t0 + t1)),
            },
            carrier: e.clone(),
            param_start: t0,
            param_end: t1,
        };
        EdgeCurve::certify(
            spec,
            e.eval(t0),
            e.eval(t1),
            |k| self.surfaces.get(k).cloned(),
            1.0,
            band,
        )
    }
}

/// The escalation a winding check raised, at a door (`StatedInterval`,
/// where its enclosure is wholly on the refused side) or as it stands.
fn winding_escalation(
    got: &Result<EdgeCurve<f64>, CertifyError>,
) -> Option<geom_core::Indeterminate> {
    match got {
        Err(CertifyError::StatedInterval {
            refused: StatedSpan::Winding(WindingReading::InBand(cause)),
        })
        | Err(CertifyError::Escalated {
            check: geom_brep::CertCheck::ParamWinding,
            cause,
            ..
        }) => Some(*cause),
        _ => None,
    }
}

/// **An ellipse wound `0.9e-9` rad past τ on its fast side does not
/// certify** (`certify-admits-an-ellipse-wound-past-its-period-on-the-slow-rate`).
/// At ε = 1e-9 the doubled stretch, centred at `θ = π/2` where the edge runs
/// at speed 20, is about `1.8e-8` m long, definitely past the band; read at
/// the speed floor `b = 1` it is `0.9e-9`, within it. The two readings
/// straddle the band, so the winding escalates on its straddle row instead
/// of certifying a doubled stretch as one that is not there.
#[test]
fn an_ellipse_wound_past_its_period_on_the_fast_side_does_not_certify() {
    let band = Band::new(1e-9, 1e-8).unwrap();
    let fixture = steep();
    let over = 0.9e-9;
    let t0 = HALF_PI - 0.5 * over;
    let got = fixture.certify(t0, t0 + TAU + over, band);
    let Some(cause) = winding_escalation(&got) else {
        panic!(
            "the over-wound edge certifies or refuses otherwise: {:?}",
            got.map(|_| "certified")
        );
    };
    assert_eq!(
        cause.predicate,
        Some("interval_span_winding_straddle"),
        "the escalation is the straddle of the two readings: {cause:?}"
    );
}
