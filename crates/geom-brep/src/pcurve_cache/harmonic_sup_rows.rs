//! Check 4's `cos t`/`sin t` pairs read at their exact sup
//! ([`harmonic_sup`], [`harmonic_sup_scalar`]). Each row stands its
//! carrier, or its stored image, `d ∈ (zero/2, zero)` off through one
//! pair whose coefficient sum is over the band, and certifies at `f64`
//! and at `Interval`.
//!
//! Built with `PCERT_HARMONIC_SUM_MUTANT=vector` the vector form is the
//! coefficient sum again and every `vector` row refuses; with `scalar`,
//! every `scalar` row. The focal cone row reads both forms and refuses
//! under either.

use core::f64::consts::{FRAC_1_SQRT_2, PI};

use geom_core::{Interval, Tol};

use super::*;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn zero() -> f64 {
    band().zero()
}

fn lift_pcurve(p: &Pcurve<f64>) -> Pcurve<Interval> {
    let f = Interval::from_f64;
    let v = |q: Vec2<f64>| Vec2::new(f(q.x), f(q.y));
    match *p {
        Pcurve::Harmonic { p0, pa, pb, pl } => Pcurve::Harmonic {
            p0: Point2::new(f(p0.x), f(p0.y)),
            pa: v(pa),
            pb: v(pb),
            pl: v(pl),
        },
        Pcurve::FocalSection(FocalImage {
            u0,
            t0,
            v0,
            va,
            vb,
            vl,
            beta,
            sense,
        }) => Pcurve::FocalSection(FocalImage {
            u0: f(u0),
            t0: f(t0),
            v0: f(v0),
            va: f(va),
            vb: f(vb),
            vl: f(vl),
            beta: f(beta),
            sense: f(sense),
        }),
        _ => unreachable!("the rows store harmonic and focal-section images"),
    }
}

/// `stored` certifies over `span` at `f64` and at `Interval`, its
/// sampled residual is over half the band, and the `f64` envelope stays
/// under the band and covers the residual up to the samples' own
/// rounding (coordinates of unit size, a few ulps each).
fn certifies(
    what: &str,
    stored: &Pcurve<f64>,
    (t0, t1): (f64, f64),
    carrier: &Curve3<f64>,
    surface: &Surface<f64>,
) {
    let cache = PcurveCache::certify(stored.clone(), t0, t1, carrier, surface, band())
        .unwrap_or_else(|e| panic!("{what}: certifies at f64: {e:?}"));
    let f = Interval::from_f64;
    PcurveCache::certify(
        lift_pcurve(stored),
        f(t0),
        f(t1),
        &carrier.map_scalar(f),
        &surface.map_scalar(f),
        band(),
    )
    .unwrap_or_else(|e| panic!("{what}: certifies at Interval: {e:?}"));
    let envelope = cache.certificate().envelope;
    let sup = (0..=4096)
        .map(|i| {
            let t = t0 + (t1 - t0) * f64::from(i) / 4096.0;
            let q = stored.eval(t);
            surface.eval(q.x, q.y).distance(carrier.eval(t))
        })
        .fold(0.0, f64::max);
    assert!(
        sup > zero() / 2.0,
        "{what}: stands {sup:e} off, not past half the band"
    );
    assert!(
        (sup - 64.0 * f64::EPSILON..zero()).contains(&envelope),
        "{what}: envelope {envelope:e} covers the residual {sup:e} inside the band"
    );
}

fn circle(center: Point3<f64>, axis: Vec3<f64>, radius: f64, u_ref: Vec3<f64>) -> Curve3<f64> {
    Curve3::Circle {
        center,
        axis,
        radius,
        u_ref,
    }
}

/// The circle about `+z` through `center`, tilted by `phi` about
/// `(1, −1, 0)/√2`: both `a·z` and `b·z` are `radius·sin φ/√2`, so its
/// axial deviation from the untilted circle is `radius·sin φ` while the
/// coefficient sum reads `√2` times that.
fn tilted_circle(center: Point3<f64>, radius: f64, phi: f64) -> Curve3<f64> {
    let k = Vec3::new(FRAC_1_SQRT_2, -FRAC_1_SQRT_2, 0.0);
    let (s, c) = phi.sin_cos();
    let turn = |v: Vec3<f64>| v * c + k.cross(v) * s + k * (k.dot(v) * (1.0 - c));
    circle(center, turn(Vec3::unit_z()), radius, turn(Vec3::unit_x()))
}

fn z_frame() -> (Point3<f64>, Vec3<f64>, Vec3<f64>) {
    (Point3::origin(), Vec3::unit_z(), Vec3::unit_x())
}

/// `vector`, the plane arm: a circle imaged at radius `R` against a
/// carrier of `R + δ`, `Δa ⊥ Δb` of length `δ` each.
#[test]
fn plane_a_radius_offset_reads_once() {
    let (origin, normal, u_ref) = z_frame();
    let plane = Surface::Plane {
        origin,
        normal,
        u_ref,
    };
    let (r, d) = (0.5, 0.75 * zero());
    let image = chart_pcurve(&circle(origin, normal, r, u_ref), &plane, band()).unwrap();
    let carrier = circle(origin, normal, r + d, u_ref);
    certifies("plane", &image, (0.0, PI), &carrier, &plane);
}

/// `vector`, the cylinder meridian's drift: an ellipse whose `a` runs
/// up the wall and whose radial part is a circle of radius `δ` about a
/// point of the wall. Its radial amplitude is in band, so it takes the
/// meridian (the D9 tie-break), and the image stands `δ` off it at
/// every `t`. The parameter interval meters at the minor axis `δ`, so
/// the span is long enough to clear the escalation band.
#[test]
fn cylinder_meridian_a_radial_circle_reads_once() {
    let (r, d, rise) = (0.5, 0.75 * zero(), 0.3);
    let cyl = Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: r,
        u_ref: Vec3::unit_x(),
    };
    let a = Vec3::new(d, 0.0, rise);
    let u_ref = a.normalize();
    let carrier = Curve3::Ellipse {
        center: Point3::new(r, 0.0, 1.0),
        axis: u_ref.cross(Vec3::unit_y()),
        major: a.norm(),
        minor: d,
        u_ref,
    };
    let image = chart_pcurve(&carrier, &cyl, band()).unwrap();
    let span = (0.0, 2.0 * band().escalate() / d);
    certifies("cylinder meridian", &image, span, &carrier, &cyl);
}

/// `vector`, a winding decided Zero on the cone rim: a circle of radius
/// `δ` about the axis of a thin cone, at a height whose rim radius is a
/// fraction of the band. The class holds only sub-band carriers, so
/// the span is long enough for the parameter interval to meter past
/// the escalation band.
#[test]
fn cone_rim_a_still_winding_reads_its_drift_once() {
    let (half_angle, d) = (0.01, 0.7 * zero());
    let h = 15.0 * zero();
    let cone = Surface::Cone {
        apex: Point3::origin(),
        axis: Vec3::unit_z(),
        half_angle,
        u_ref: Vec3::unit_x(),
    };
    let carrier = circle(Point3::new(0.0, 0.0, h), Vec3::unit_z(), d, Vec3::unit_x());
    let image = chart_pcurve(&carrier, &cone, band()).unwrap();
    let span = (0.0, 2.0 * band().escalate() / d);
    certifies("cone rim, winding Zero", &image, span, &carrier, &cone);
}

/// `vector`, a winding decided Zero on the sphere parallel: a circle of
/// radius `δ` about the pole, where the parallel's own radius is zero.
#[test]
fn sphere_parallel_a_still_winding_reads_its_drift_once() {
    let d = 0.75 * zero();
    let sphere = Surface::Sphere {
        center: Point3::origin(),
        radius: 1.0,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let carrier = circle(
        Point3::new(0.0, 0.0, 1.0),
        Vec3::unit_z(),
        d,
        Vec3::unit_x(),
    );
    let image = chart_pcurve(&carrier, &sphere, band()).unwrap();
    let span = (0.0, 2.0 * band().escalate() / d);
    certifies(
        "sphere parallel, winding Zero",
        &image,
        span,
        &carrier,
        &sphere,
    );
}

/// `vector`, the winding-Zero drifts no carrier reaches: the cylinder's
/// class needs a radial amplitude decided past the escalation band,
/// and the torus's reads the full `R + r` beside it. `incidence` is
/// pinned on a form whose radial coefficients are `⊥` and of length
/// `δ`: the drift is `δ`.
#[test]
fn moving_classes_still_windings_read_their_drift_once() {
    let d = 0.75 * zero();
    let form = Harmonic3 {
        c: Point3::origin(),
        a: Vec3::new(d, 0.0, 0.0),
        b: Vec3::new(0.0, d, 0.0),
        l: Vec3::zero(),
    };
    let cyl = Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: 0.5,
        u_ref: Vec3::unit_x(),
    };
    let torus = Surface::Torus {
        center: Point3::origin(),
        axis: Vec3::unit_z(),
        major_radius: 1.0,
        minor_radius: 0.25,
        u_ref: Vec3::unit_x(),
    };
    for (what, surface, derivation) in [
        (
            "cylinder",
            &cyl,
            Derivation::CylinderMoving {
                beta: Winding::Zero,
            },
        ),
        (
            "torus",
            &torus,
            Derivation::TorusParallel {
                beta: Winding::Zero,
            },
        ),
    ] {
        let mut terms = EnvelopeTerms::new();
        incidence(form, surface, derivation, 1.0, &mut terms);
        let drift = terms.0[EnvelopeTerm::Drift.slot()];
        assert!(
            (drift - d).abs() <= 4.0 * f64::EPSILON * d,
            "{what}: drift {drift:e} for a radial circle of {d:e}"
        );
    }
}

/// `scalar`, the tilt of a cone rim, a sphere parallel and a torus
/// parallel, and the axial gate each class is chosen by: a circle on
/// the class tilted until it stands `0.85·zero` off axially, with
/// `|a·e| = |b·e|`.
#[test]
fn a_tilted_parallel_reads_its_tilt_once() {
    let (center, axis, u_ref) = z_frame();
    let h = 1.0;
    let half_angle: f64 = 0.45;
    let cone = Surface::Cone {
        apex: center,
        axis,
        half_angle,
        u_ref,
    };
    let sphere = Surface::Sphere {
        center,
        radius: 1.0,
        axis,
        u_ref,
    };
    let torus = Surface::Torus {
        center,
        axis,
        major_radius: 1.0,
        minor_radius: 0.25,
        u_ref,
    };
    let off = 0.85 * zero();
    for (what, surface, rim_center, radius) in [
        (
            "cone rim",
            &cone,
            Point3::new(0.0, 0.0, h),
            h * half_angle.tan(),
        ),
        ("sphere parallel", &sphere, Point3::new(0.0, 0.0, 0.6), 0.8),
        ("torus parallel", &torus, center, 1.25),
    ] {
        let flat = circle(rim_center, axis, radius, u_ref);
        let image = chart_pcurve(&flat, surface, band()).unwrap();
        let carrier = tilted_circle(rim_center, radius, (off / radius).asin());
        certifies(what, &image, (0.0, PI), &carrier, surface);
    }
}

/// `scalar`, check 4's fidelity: a cylinder rim's stored image with
/// its `cos t` and `sin t` coefficients each moved `δ/√2`, in the
/// axial channel and in the azimuth channel (`δ/(r·√2)`), so the image
/// stands up to `δ` off the one re-derived.
#[test]
fn fidelity_a_drifted_pair_reads_once() {
    let r = 0.5;
    let cyl = Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: r,
        u_ref: Vec3::unit_x(),
    };
    let carrier = circle(Point3::origin(), Vec3::unit_z(), r, Vec3::unit_x());
    let Pcurve::Harmonic { p0, pa, pb, pl } = chart_pcurve(&carrier, &cyl, band()).unwrap() else {
        panic!("a rim images harmonic")
    };
    let d = 0.85 * zero() * FRAC_1_SQRT_2;
    for (what, step) in [
        ("fidelity v", Vec2::new(0.0, d)),
        ("fidelity u", Vec2::new(d / r, 0.0)),
    ] {
        let stored = Pcurve::Harmonic {
            p0,
            pa: pa + step,
            pb: pb + step,
            pl,
        };
        certifies(what, &stored, (0.0, PI), &carrier, &cyl);
    }
}

/// `vector` and `scalar`, a cone's focal section with its stored `va`
/// and `vb` each moved `h/√2`: the harmonic part reads the move at
/// `cos α` through a parallel pair, the Kepler remainder at `sin α`,
/// so the envelope is `(cos α + sin α)·h` and either sum adds a
/// `√2` to its share.
#[test]
fn focal_cone_a_drifted_slant_pair_reads_once() {
    let half_angle: f64 = 0.45;
    let cone = Surface::Cone {
        apex: Point3::origin(),
        axis: Vec3::unit_z(),
        half_angle,
        u_ref: Vec3::unit_x(),
    };
    let tilt: f64 = 0.5;
    let side = Vec3::unit_y();
    let normal = Vec3::unit_z() * tilt.cos() + side * tilt.sin();
    let plane = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 2.5),
        normal,
        u_ref: side.cross(normal),
    };
    let reach = crate::Reach::Measured {
        at: Point3::origin(),
        lever: 3.0,
    };
    let Ok(crate::intersect::PlaneConeSection::TiltedEllipse(carrier)) =
        crate::intersect::plane_cone_section(&plane, &cone, &reach, band())
    else {
        panic!("a 0.5 rad tilt of a 0.45 rad cone is an ellipse")
    };
    let Pcurve::FocalSection(image) = chart_pcurve(&carrier, &cone, band()).unwrap() else {
        panic!("a cone's ellipse images as a focal section")
    };
    let h = 0.7 * zero() * FRAC_1_SQRT_2;
    let stored = Pcurve::FocalSection(FocalImage {
        va: image.va + h,
        vb: image.vb + h,
        ..image
    });
    certifies("focal cone", &stored, (0.3, 4.0), &carrier, &cone);
}

/// `scalar`, a Villarceau circle's focal section with its stored `va`
/// and `vb` each moved `h/(r·√2)`: the meridional drift reads it at the
/// tube radius, `h`.
#[test]
fn focal_torus_a_drifted_meridional_pair_reads_once() {
    let (major, minor) = (1.0, 0.25);
    let torus = Surface::Torus {
        center: Point3::origin(),
        axis: Vec3::unit_z(),
        major_radius: major,
        minor_radius: minor,
        u_ref: Vec3::unit_x(),
    };
    let tilt = (minor / major).asin();
    let carrier = circle(
        Point3::new(minor, 0.0, 0.0),
        Vec3::new(0.0, -tilt.sin(), tilt.cos()),
        major,
        Vec3::unit_x(),
    );
    let Pcurve::FocalSection(image) = chart_pcurve(&carrier, &torus, band()).unwrap() else {
        panic!("a Villarceau circle images as a focal section")
    };
    let h = 0.8 * zero() * FRAC_1_SQRT_2 / minor;
    let stored = Pcurve::FocalSection(FocalImage {
        va: image.va + h,
        vb: image.vb + h,
        ..image
    });
    certifies("focal torus", &stored, (0.0, PI), &carrier, &torus);
}

/// Both forms against a dense sampling of `a·cos t + b·sin t` — `⊥`
/// and of one length (the lever-2 case), parallel, one zero, and a
/// general pair — and at `Interval` on coefficients straddling zero,
/// where the result must still decide.
#[test]
fn the_closed_forms_are_the_sampled_sup() {
    let sampled = |a: Vec3<f64>, b: Vec3<f64>| {
        (0..=1 << 16)
            .map(|i| {
                let (s, c) = (f64::from(i) * core::f64::consts::TAU / 65536.0).sin_cos();
                (a * c + b * s).norm()
            })
            .fold(0.0, f64::max)
    };
    let rows = [
        (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)),
        (Vec3::new(1.0, 2.0, 0.5), Vec3::new(-2.0, -4.0, -1.0)),
        (Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.3, -0.4, 1.2)),
        (Vec3::new(0.7, -0.2, 0.4), Vec3::new(0.1, 0.9, -0.3)),
    ];
    for (a, b) in rows {
        let want = sampled(a, b);
        let got = harmonic_sup(a, b);
        assert!(
            (got - want).abs() <= 1e-9 * want,
            "vector {a:?}, {b:?}: {got} against the sampled {want}"
        );
        let want = sampled(Vec3::new(a.x, 0.0, 0.0), Vec3::new(b.x, 0.0, 0.0));
        let got = harmonic_sup_scalar(a.x, b.x);
        assert!(
            (got - want).abs() <= 1e-9 * want.max(1e-300),
            "scalar {}, {}: {got} against the sampled {want}",
            a.x,
            b.x
        );
    }
    let fuzz = Interval::from_f64(1e-17) - Interval::from_f64(1e-17 * (1.0 + f64::EPSILON));
    let tiny = Vec3::new(fuzz, fuzz, fuzz);
    for (what, value) in [
        ("vector", harmonic_sup(tiny, tiny)),
        ("scalar", harmonic_sup_scalar(fuzz, fuzz)),
    ] {
        assert_eq!(
            decide("harmonic_sup_rows", Margin::of(value), band()),
            Ok(Sign::Zero),
            "{what}: a rounding-width pair decides Zero"
        );
    }
}
