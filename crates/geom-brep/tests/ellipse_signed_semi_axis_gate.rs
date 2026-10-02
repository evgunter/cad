//! The certify gate on an ellipse's stored semi-axes: it meters either
//! ORDER (the smaller magnitude is the speed floor), refuses a
//! non-positive `minor` as it always has, and admits a negative `major`
//! as it always has (tier 3 refuses that one on its value).

#![allow(clippy::unwrap_used)]

use geom::{Curve3, Surface};
use geom_brep::{CertifyError, EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec, SurfaceKey};
use geom_core::{Band, Point3, Tol, Vec3};
use slotmap::SlotMap;

/// **The gate is not widened: a negative `minor` is refused, either
/// order is metered.** The plane `z = 0` cuts a unit cylinder leaning
/// 0.5 rad in xz in the ellipse `a = 1/cos 0.5`, `b = 1` about the
/// origin. Stored as `(a, b)` or swapped as `(b, a)` with `u_ref` along
/// the minor axis, its quarter arc certifies; stored with `minor = −1`
/// (the same locus, traversed the other way) it is refused
/// `IntervalNotForward`, as the gate refused it before the span meter
/// read semi-axis magnitudes — read at `min(|a|, |b|)` it certified,
/// which widened the gate. A negative `major` certifies as it always
/// has (`tier3_tests` mints one so check 1 can refuse it).
#[test]
fn the_gate_meters_either_order_and_refuses_a_signed_semi_axis() {
    let band = Band::linear(Tol::witness()).unwrap();
    let alpha: f64 = 0.5;
    let (a, b) = (1.0 / alpha.cos(), 1.0);
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
    let (x, y) = (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    let certify = |major: f64, minor: f64, u_ref: Vec3<f64>| {
        let e = Curve3::Ellipse {
            center: Point3::origin(),
            axis: Vec3::new(0.0, 0.0, 1.0),
            major,
            minor,
            u_ref,
        };
        let spec = EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                s1: plane,
                s2: cyl,
                witness: e.eval(core::f64::consts::FRAC_PI_4),
            },
            carrier: e.clone(),
            param_start: 0.0,
            param_end: core::f64::consts::FRAC_PI_2,
        };
        EdgeCurve::certify(
            spec,
            e.eval(0.0),
            e.eval(core::f64::consts::FRAC_PI_2),
            |k| surfaces.get(k).cloned(),
            band,
        )
    };
    assert!(certify(a, b, x).is_ok(), "the ordinary frame certifies");
    // Swapped: `u_ref` along the minor axis, `v_ref = axis × u_ref = −x̂`
    // along the major, so the semi-axis stored first is `b`.
    assert!(certify(b, a, y).is_ok(), "the swapped frame certifies");
    let got = certify(a, -b, x);
    assert!(
        matches!(got, Err(CertifyError::IntervalNotForward { .. })),
        "a negative minor: {:?}",
        got.map(|_| "certified")
    );
    assert!(
        certify(-a, b, -x).is_ok(),
        "a negative major, u_ref flipped, certifies"
    );
}
