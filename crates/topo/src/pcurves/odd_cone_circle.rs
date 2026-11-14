use super::{PcurveMintError, analytic_derive, not_owed};
use crate::entity::HalfEdgeKey;
use geom::{Curve3, Surface};
use geom_brep::{PcurveCertifyError, PcurveCheck};
use geom_core::{Band, Point3, Tol, Vec3};

/// **An odd cone circle is owed at `f64`.** A circle about a cone's
/// axis at its rim's height and radius, tilted off the rim's plane
/// through the band and past it, on a grid of quarter decades about
/// ε: untilted it is the rim and mints; tilted inside the band it
/// still reads as the rim; where the class reading is in band, or
/// definite while the incidence read does not show it off, it is
/// none of the cone's circles and takes the general lane, its
/// projected image; past the band it is off the chart. Every refusal
/// is a check-8 finding: the one excuse ([`not_owed`]) is a scalar
/// with no fitted door, and `f64` holds one. The grid must reach the
/// general lane, or the row has not read the arm it is about.
#[test]
fn an_odd_cone_circle_mints_or_is_a_finding_never_excused() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let alpha = 0.5_f64;
    let cone = Surface::Cone {
        apex: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        half_angle: alpha,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let (h, tau) = (1.0_f64, core::f64::consts::TAU);
    let tilts = core::iter::once(0.0)
        .chain((-12..=16).map(|k| tol.eps() * 10f64.powf(f64::from(k) / 4.0)))
        .chain([0.3]);
    let mut general = 0;
    for tilt in tilts {
        let circle = Curve3::circle(
            Point3::new(0.0, 0.0, h),
            Vec3::new(0.0, -tilt.sin(), tilt.cos()),
            h * alpha.tan(),
            Vec3::new(1.0, 0.0, 0.0),
        );
        match analytic_derive(&circle, 0.0, tau, &cone, band, HalfEdgeKey::default()) {
            Ok(image) => {
                assert!(
                    tilt < 0.3,
                    "tilt {tilt:e}: a circle well off the rim minted"
                );
                if matches!(image, geom_brep::Pcurve::Projected(_)) {
                    general += 1;
                }
            }
            Err(e) => {
                assert!(tilt > 0.0, "tilt {tilt:e}: the rim itself refused: {e:?}");
                assert!(!not_owed::<f64>(&e), "tilt {tilt:e}: excused: {e:?}");
                assert!(
                    !matches!(
                        e,
                        PcurveMintError::Certify {
                            error: PcurveCertifyError::Escalated {
                                check: PcurveCheck::ChartClass,
                                ..
                            } | PcurveCertifyError::NoGeneralLane { .. },
                            ..
                        }
                    ),
                    "tilt {tilt:e}: a class reading ends as the general lane, not as a refusal: \
                         {e:?}"
                );
            }
        }
    }
    assert!(general > 0, "no tilt reached the general lane");
}
