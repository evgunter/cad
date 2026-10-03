//! The tilted plane × cone ellipse's exact chart image on its cone
//! (`Pcurve::ConeSection`): derived by `chart_pcurve`, exact against
//! the carrier through the chart map, certified with a rounding-scale
//! envelope, and refused when any of its numbers is wrong.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::tol::band;
use geom::{Curve3, Surface};
use geom_brep::intersect::{PlaneConeSection, plane_cone_section};
use geom_brep::{EnvelopeStatement, Pcurve, PcurveCache, chart_pcurve};
use geom_core::{Point3, Vec3};

/// A cone off the origin about a skew axis, given with either axis
/// sign. The two are the same locus; the cut below lies on the `+axis`
/// nappe of the first and the mirror nappe of the second.
fn cone(axis_sign: f64) -> Surface<f64> {
    cone_at(axis_sign, 0.45)
}

/// [`cone`] at another half-angle.
fn cone_at(axis_sign: f64, half_angle: f64) -> Surface<f64> {
    let axis = Vec3::new(0.2, -0.3, 1.0).normalize() * axis_sign;
    let seed = Vec3::unit_x();
    Surface::Cone {
        apex: Point3::new(0.4, -0.7, 0.2),
        axis,
        half_angle,
        u_ref: (seed - axis * seed.dot(axis)).normalize(),
    }
}

/// The section of `cone(+1)` by a plane tilted 0.5 rad off its axis,
/// 2.5 along it from the apex: an ellipse on the `+axis` nappe.
fn section() -> Curve3<f64> {
    section_of(0.45, 0.5)
}

/// The section of `cone_at(1, half_angle)` by a plane tilted `tilt` off
/// its axis, 2.5 along it from the apex.
fn section_of(half_angle: f64, tilt: f64) -> Curve3<f64> {
    let surface = cone_at(1.0, half_angle);
    let Surface::Cone { apex, axis, .. } = surface else {
        unreachable!()
    };
    let side = axis.cross(Vec3::unit_x()).normalize();
    let normal = (axis * tilt.cos() + side * tilt.sin()).normalize();
    let plane = Surface::Plane {
        origin: apex + axis * 2.5,
        normal,
        u_ref: side.cross(normal),
    };
    let Ok(PlaneConeSection::TiltedEllipse(e)) = plane_cone_section(&plane, &surface, 3.0, band())
    else {
        panic!("a {tilt} rad tilt of a {half_angle} rad cone is an ellipse");
    };
    e
}

/// The same locus traversed backwards (the arc-side rule's cw arm).
fn reversed(e: &Curve3<f64>) -> Curve3<f64> {
    let &Curve3::Ellipse {
        center,
        axis,
        major,
        minor,
        u_ref,
    } = e
    else {
        unreachable!()
    };
    Curve3::Ellipse {
        center,
        axis: -axis,
        major,
        minor,
        u_ref,
    }
}

/// Every pairing of nappe and traversal: the image is the
/// `ConeSection` form, maps onto the carrier at rounding scale over two
/// periods (the azimuth's branch never jumps), and certifies over a
/// span longer than a half-turn with a rounding-scale envelope.
#[test]
fn the_image_is_exact_on_both_nappes_and_both_traversals() {
    for axis_sign in [1.0, -1.0] {
        let surface = cone(axis_sign);
        for carrier in [section(), reversed(&section())] {
            let what = format!("axis sign {axis_sign}, carrier {carrier:?}");
            let image = chart_pcurve(&carrier, &surface, band()).unwrap();
            let Pcurve::ConeSection { beta, sense, .. } = image else {
                panic!("{what}: expected the cone-section image, got {image:?}");
            };
            assert!(
                beta.abs() > 0.05,
                "{what}: a genuinely eccentric projection"
            );
            assert_eq!(sense.abs(), 1.0, "{what}");
            for i in 0..=64 {
                let t = -3.0 + 9.0 * f64::from(i) / 64.0;
                let uv = image.eval(t);
                let d = surface.eval(uv.x, uv.y).distance(carrier.eval(t));
                assert!(d < 1e-12, "{what}: t {t}: {d:e}");
            }
            let cache = PcurveCache::certify(image, 0.3, 4.0, &carrier, &surface, band())
                .unwrap_or_else(|e| panic!("{what}: {e}"));
            let cert = cache.certificate();
            assert!(
                cert.envelope < 1e-12,
                "{what}: envelope {:e}",
                cert.envelope
            );
            assert!(
                matches!(cert.statement, EnvelopeStatement::MapResidualClosedForm),
                "{what}"
            );
        }
    }
}

/// Each number of the image is load-bearing: one moved by `1e-6`, or
/// the image offered against a different cone, refuses certification.
#[test]
fn a_wrong_number_in_the_image_refuses() {
    let surface = cone(1.0);
    let carrier = section();
    let image = chart_pcurve(&carrier, &surface, band()).unwrap();
    let Pcurve::ConeSection {
        u0,
        v0,
        va,
        vb,
        beta,
        sense,
    } = image
    else {
        unreachable!()
    };
    let h = 1e-6;
    let wrong = [
        ("u0", (u0 + h, v0, va, vb, beta, sense)),
        ("v0", (u0, v0 + h, va, vb, beta, sense)),
        ("va", (u0, v0, va + h, vb, beta, sense)),
        ("vb", (u0, v0, va, vb + h, beta, sense)),
        ("beta", (u0, v0, va, vb, beta + h, sense)),
        ("sense", (u0, v0, va, vb, beta, -sense)),
    ];
    for (name, (u0, v0, va, vb, beta, sense)) in wrong {
        let bad = Pcurve::ConeSection {
            u0,
            v0,
            va,
            vb,
            beta,
            sense,
        };
        assert!(
            PcurveCache::certify(bad, 0.3, 4.0, &carrier, &surface, band()).is_err(),
            "{name} moved by {h} still certifies"
        );
    }
    let Surface::Cone {
        apex,
        axis,
        half_angle,
        u_ref,
    } = surface
    else {
        unreachable!()
    };
    let other = Surface::Cone {
        apex,
        axis,
        half_angle: half_angle + h,
        u_ref,
    };
    assert!(
        PcurveCache::certify(image, 0.3, 4.0, &carrier, &other, band()).is_err(),
        "the image certified against another cone"
    );
}

/// The envelope BOUNDS what it certifies. An image with `va` or `vb`
/// moved by a quarter of ε still certifies, and its residual is then
/// the moved slant `δ·(cos t | sin t)` along a unit generator — the
/// harmonic part carries only `cos α` of it, the Kepler remainder the
/// `sin α` rest. The certified envelope must cover the residual sampled
/// densely over the span. At `α = 1.2`, `cos α + sin α / 2 < 1`, so even
/// a HALVED remainder leaves the envelope under the residual: a
/// remainder dropped or shrunk is red here.
#[test]
fn the_envelope_covers_an_admitted_slant_error() {
    for (half_angle, tilt) in [(0.45, 0.5), (1.2, 0.2)] {
        let surface = cone_at(1.0, half_angle);
        let carrier = section_of(half_angle, tilt);
        let image = chart_pcurve(&carrier, &surface, band()).unwrap();
        let Pcurve::ConeSection {
            u0,
            v0,
            va,
            vb,
            beta,
            sense,
        } = image
        else {
            unreachable!()
        };
        let h = 0.25 * crate::shared::tol::eps();
        for (name, va, vb) in [("va", va + h, vb), ("vb", va, vb + h)] {
            let what = format!("alpha {half_angle}, {name} moved by eps/4");
            let moved = Pcurve::ConeSection {
                u0,
                v0,
                va,
                vb,
                beta,
                sense,
            };
            let (t0, t1) = (0.3, 4.0);
            let cache = PcurveCache::certify(moved.clone(), t0, t1, &carrier, &surface, band())
                .unwrap_or_else(|e| panic!("{what}: {e}"));
            let envelope = cache.certificate().envelope;
            let sup = (0..=4096)
                .map(|i| {
                    let t = t0 + (t1 - t0) * f64::from(i) / 4096.0;
                    let uv = moved.eval(t);
                    surface.eval(uv.x, uv.y).distance(carrier.eval(t))
                })
                .fold(0.0, f64::max);
            assert!(sup > 0.9 * h, "{what}: off its carrier by {sup:e}");
            assert!(
                envelope >= sup,
                "{what}: envelope {envelope:e} under the sampled residual {sup:e}"
            );
        }
    }
}
