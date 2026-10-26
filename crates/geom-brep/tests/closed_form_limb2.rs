//! **Limb 2 is owed by every `Intersection` carrier of every kind** (C2):
//! a line, circle, ellipse or spiric between two analytic surfaces owes
//! the composite hull bound between its samples, read through its
//! certified enclosing net, exactly as a spline carrier does.
//!
//! The teeth: a closed-form carrier whose stored fields put it off its
//! surface by more than the band's escalation threshold between two
//! schedule samples, while every one of the nine samples (and the
//! witness) reads inside the zero band on the certifier's own meter,
//! refuses on limb 2. The miss is placed between the two samples that
//! straddle the carrier's farthest point from the surface, so the
//! samples read it at a fraction of its peak. The band is chosen from
//! that fraction, measured here, so the row says what it tests rather
//! than leaning on a constant: the samples read under `zero`, the peak
//! lies past `escalate`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::surf::table;
use crate::shared::tol::band;
use core::f64::consts::PI;
use geom::{Curve3, Surface};
use geom_brep::intersect::{PlaneTorusSection, plane_torus_section};
use geom_brep::{
    AnalyticRung3Refusal, CERT_SAMPLES, CertifyError, EdgeCurve, EdgeCurveSpec,
    EdgeDescriptionSpec, NurbsLane, Rung3Limb, SurfacePair, implicit_residual, sample_param,
};
use geom_core::{Band, Point3, Vec3};

/// The torus of the spiric rows: centre `(1, 2, 3)`, axis `y`.
fn torus() -> Surface<f64> {
    Surface::Torus {
        center: Point3::new(1.0, 2.0, 3.0),
        axis: Vec3::unit_y(),
        major_radius: 0.75,
        minor_radius: 0.3,
        u_ref: Vec3::unit_x(),
    }
}

/// The axis-parallel plane `x = 1.1`, off the torus's axis by `0.1`.
fn spiric_plane() -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(1.1, 2.0, 3.0),
        normal: Vec3::unit_x(),
        u_ref: Vec3::unit_y(),
    }
}

/// One of the two exact ovals `x = 1.1` cuts from [`torus`].
fn spiric() -> Curve3<f64> {
    let section = plane_torus_section(&spiric_plane(), &torus(), 1.0, band()).unwrap();
    let PlaneTorusSection::SpiricOvals { s1, .. } = section else {
        panic!("expected the two spiric ovals, got {section:?}");
    };
    s1
}

/// The cylinder `x² + y² = 1` and the plane `z = y·tan φ` through it.
fn ellipse_pair(phi: f64) -> (Surface<f64>, Surface<f64>, Curve3<f64>) {
    let (s, c) = phi.sin_cos();
    let normal = Vec3::new(0.0, -s, c);
    let cylinder = Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    let plane = Surface::Plane {
        origin: Point3::origin(),
        normal,
        u_ref: Vec3::unit_x(),
    };
    let ellipse = Curve3::Ellipse {
        center: Point3::origin(),
        axis: normal,
        semi_u: 1.0,
        semi_v: 1.0 / c,
        u_ref: Vec3::unit_x(),
    };
    (cylinder, plane, ellipse)
}

/// The edge `carrier` over `[t0, t1]` as the intersection of `a` and
/// `b`, certified at `band` through the lane.
fn certify(
    a: &Surface<f64>,
    b: &Surface<f64>,
    carrier: &Curve3<f64>,
    (t0, t1): (f64, f64),
    band: Band,
) -> Result<EdgeCurve<f64>, CertifyError> {
    let (keys, lookup) = table(vec![a.clone(), b.clone()]);
    let spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::Intersection {
            pair: SurfacePair::new(keys[0], keys[1]),
            witness: carrier.mid_point(t0, t1),
        },
        carrier: carrier.clone(),
        param_start: t0,
        param_end: t1,
    };
    EdgeCurve::certify(
        spec,
        carrier.eval(t0),
        carrier.eval(t1),
        &lookup,
        1.0,
        band,
        NurbsLane::certified(),
    )
}

/// The worst reading the certifier's per-sample meter takes of
/// `carrier` against `a` and `b` over its schedule, the witness included.
fn sampled_max(
    a: &Surface<f64>,
    b: &Surface<f64>,
    carrier: &Curve3<f64>,
    (t0, t1): (f64, f64),
) -> f64 {
    (0..CERT_SAMPLES)
        .map(|i| carrier.eval(sample_param(t0, t1, i)))
        .chain([carrier.mid_point(t0, t1)])
        .map(|p| {
            implicit_residual(a, p)
                .abs()
                .max(implicit_residual(b, p).abs())
        })
        .fold(0.0, f64::max)
}

/// A span whose two middle samples straddle `t = 0`, the farthest
/// point of each perturbed row below, with both its ends well inside
/// the stretch where the miss is smaller than at those two samples.
const SPAN: (f64, f64) = (-7.0 * 1.5 * PI / 16.0, 9.0 * 1.5 * PI / 16.0);

/// The band the teeth rows read at: `zero` just above the samples'
/// worst reading, `escalate` halfway from it to the carrier's true
/// distance at `t = 0`, which the row asserts lies past it.
fn teeth_band(sampled: f64, peak: f64) -> Band {
    let zero = sampled * (1.0 + 1e-3);
    let escalate = 0.5 * (zero + peak);
    assert!(
        zero < escalate && escalate < peak,
        "the samples ({sampled:e}) read a fraction of the peak ({peak:e})"
    );
    Band::new(zero, escalate).unwrap()
}

/// Whether `got` is limb 2's definite refusal against the operand `kind`.
fn refused_on_limb2(got: &Result<EdgeCurve<f64>, CertifyError>, kind: geom::SurfaceKind) -> bool {
    matches!(
        got,
        Err(CertifyError::AnalyticRung3(AnalyticRung3Refusal::Limb {
            operand,
            limb: Rung3Limb::HullSup | Rung3Limb::HullValue,
            ..
        })) if *operand == kind
    )
}

/// Every closed-form kind certifies through the lane on the pair that
/// mints it — plane × cylinder ruling, plane × cone and plane × torus
/// circles (the per-part axis lever), the tilted plane × cylinder
/// ellipse and the axis-parallel plane × torus spiric.
#[test]
fn every_closed_form_kind_owes_limb_2_through_the_lane() {
    let cylinder = Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    let side = Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::unit_y(),
        u_ref: Vec3::unit_x(),
    };
    let ruling = Curve3::Line {
        origin: Point3::new(1.0, 0.0, -1.0),
        dir: Vec3::unit_z(),
    };
    let cone = Surface::Cone {
        apex: Point3::origin(),
        axis: Vec3::unit_z(),
        half_angle: PI / 6.0,
        u_ref: Vec3::unit_x(),
    };
    let level = |z: f64| Surface::Plane {
        origin: Point3::new(0.0, 0.0, z),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let circle = |z: f64, radius: f64| Curve3::Circle {
        center: Point3::new(0.0, 0.0, z),
        axis: Vec3::unit_z(),
        radius,
        u_ref: Vec3::unit_x(),
    };
    let cone_circle = circle(2.0, 2.0 * (PI / 6.0).tan());
    let flat_torus = Surface::Torus {
        center: Point3::origin(),
        axis: Vec3::unit_z(),
        major_radius: 0.75,
        minor_radius: 0.3,
        u_ref: Vec3::unit_x(),
    };
    let equator = circle(0.0, 1.05);
    let (ecyl, eplane, ellipse) = ellipse_pair(PI / 5.0);
    let rows = [
        ("ruling", cylinder.clone(), side, ruling, (0.0, 2.0)),
        ("cone circle", cone, level(2.0), cone_circle, (0.3, 2.9)),
        (
            "torus circle",
            flat_torus,
            level(0.0),
            equator,
            (0.0, 1.5 * PI),
        ),
        ("ellipse", ecyl, eplane, ellipse, SPAN),
        ("spiric", torus(), spiric_plane(), spiric(), SPAN),
    ];
    for (name, a, b, carrier, span) in rows {
        let (keys, lookup) = table(vec![a.clone(), b.clone()]);
        let (t0, t1) = span;
        let spec = EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                pair: SurfacePair::new(keys[0], keys[1]),
                witness: carrier.mid_point(t0, t1),
            },
            carrier: carrier.clone(),
            param_start: t0,
            param_end: t1,
        };
        let (start, end) = (carrier.eval(t0), carrier.eval(t1));
        let lane = NurbsLane::certified();
        let edge = EdgeCurve::certify(spec, start, end, &lookup, 1.0, band(), lane)
            .unwrap_or_else(|e| panic!("{name}: the exact carrier certifies: {e:?}"));
        let at_rest = edge
            .recertify(start, end, &lookup, 1.0, band(), lane)
            .unwrap_or_else(|e| panic!("{name}: rest re-derives the certificate: {e:?}"));
        assert_eq!(
            format!("{at_rest:?}"),
            format!("{:?}", edge.certificate()),
            "{name}: rest re-derives the door's certificate"
        );
    }
}

/// A spiric whose stored major radius is `δ` past its torus's sits `δ·cos t`
/// off the torus (to second order) and exactly on its plane: the nine
/// samples straddle its farthest point, read inside the zero band, and
/// limb 2 refuses it against the torus.
#[test]
fn a_spiric_off_its_torus_between_samples_refuses_on_limb_2() {
    let Curve3::Spiric {
        center,
        axis,
        u_ref,
        major_radius,
        minor_radius,
        offset,
    } = spiric()
    else {
        unreachable!("the section's oval is a spiric")
    };
    let delta = 1e-4;
    let moved = Curve3::Spiric {
        center,
        axis,
        u_ref,
        major_radius: major_radius + delta,
        minor_radius,
        offset,
    };
    let (tor, plane) = (torus(), spiric_plane());
    let sampled = sampled_max(&tor, &plane, &moved, SPAN);
    let peak = implicit_residual(&tor, moved.eval(0.0)).abs();
    let got = certify(&tor, &plane, &moved, SPAN, teeth_band(sampled, peak));
    assert!(
        refused_on_limb2(&got, geom::SurfaceKind::Torus),
        "limb 2 against the torus refuses the moved spiric: {got:?}"
    );
}

/// An ellipse whose stored `semi_u` is `δ` past the cylinder's radius
/// sits `δ·cos² θ` off the cylinder and exactly on its plane: the same
/// teeth as the spiric's. The ellipse's residual against a quadric is a
/// trigonometric polynomial of degree 2, so the samples bound it up to
/// a fixed factor and no ellipse is far off between samples while near
/// on all nine; the factor is what the band reads past here.
#[test]
fn an_ellipse_off_its_cylinder_between_samples_refuses_on_limb_2() {
    let (cylinder, plane, ellipse) = ellipse_pair(PI / 5.0);
    let Curve3::Ellipse {
        center,
        axis,
        semi_u,
        semi_v,
        u_ref,
    } = ellipse
    else {
        unreachable!("built as an ellipse")
    };
    let moved = Curve3::Ellipse {
        center,
        axis,
        semi_u: semi_u + 1e-4,
        semi_v,
        u_ref,
    };
    let sampled = sampled_max(&cylinder, &plane, &moved, SPAN);
    let peak = implicit_residual(&cylinder, moved.eval(0.0)).abs();
    let got = certify(&cylinder, &plane, &moved, SPAN, teeth_band(sampled, peak));
    assert!(
        refused_on_limb2(&got, geom::SurfaceKind::Cylinder),
        "limb 2 against the cylinder refuses the moved ellipse: {got:?}"
    );
}
