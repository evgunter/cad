//! Check 4's `cos t`/`sin t` pairs read at their exact sup
//! ([`harmonic_sup`], [`harmonic_sup_scalar`]). Each row stands its
//! carrier, or its stored image, `d ∈ (zero/2, zero)` off through one
//! pair whose coefficient sum is over the band, and certifies at `f64`
//! and at `Interval` with an envelope that covers its sampled residual.
//!
//! Built with `PCERT_HARMONIC_SUM_MUTANT` naming one of
//! `crate::harmonic`'s wrong forms: under `vector` (or `scalar`) the
//! form is the coefficient sum again and the rows through that form
//! refuse; under an under-reading form (`*-max`, `*-rms`,
//! `vector-inner`, `vector-dot`) the rows on an unequal or skew pair
//! read under their residual; under `square-mul` the straddling pair of
//! [`the_closed_forms_are_the_sampled_sup`] poisons.

use core::f64::consts::{FRAC_1_SQRT_2, PI, TAU};

use geom_core::{Bounds, Interval, Tol};

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
    let at_interval = PcurveCache::certify(
        lift_pcurve(stored),
        f(t0),
        f(t1),
        &carrier.map_scalar(f),
        &surface.map_scalar(f),
        band(),
    )
    .unwrap_or_else(|e| panic!("{what}: certifies at Interval: {e:?}"));
    covers(
        what,
        stored,
        (t0, t1),
        carrier,
        surface,
        cache.certificate().envelope,
        at_interval.certificate().envelope.hi(),
    );
}

/// The sampled residual of `stored` against `carrier` is over half the
/// band, the `f64` envelope covers it inside the band up to the
/// samples' own rounding (coordinates of unit size, a few ulps each),
/// and the `Interval` envelope's upper end covers it too.
fn covers(
    what: &str,
    stored: &Pcurve<f64>,
    (t0, t1): (f64, f64),
    carrier: &Curve3<f64>,
    surface: &Surface<f64>,
    envelope: f64,
    interval_hi: f64,
) {
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
    let slack = 64.0 * f64::EPSILON;
    assert!(
        (sup - slack..zero()).contains(&envelope),
        "{what}: envelope {envelope:e} covers the residual {sup:e} inside the band"
    );
    assert!(
        interval_hi >= sup - slack,
        "{what}: the Interval envelope {interval_hi:e} covers the residual {sup:e}"
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

/// `vector`, the plane arm on a skew, unequal pair: a stored image whose
/// `pa` and `pb` move by `(1, 0)·k` and `(0.8, 0.3)·k`, so `a·b ≠ 0` and
/// `‖a‖ ≠ ‖b‖`. Its sup is the pair's largest singular value, `1.29·k`,
/// over the larger coefficient (`k`), the root mean square, and the
/// forms without the inner root or without `a·b`; the sum reads
/// `1.85·k`.
#[test]
fn plane_a_skew_unequal_pair_reads_its_singular_value() {
    let (origin, normal, u_ref) = z_frame();
    let plane = Surface::Plane {
        origin,
        normal,
        u_ref,
    };
    let carrier = circle(origin, normal, 0.5, u_ref);
    let Pcurve::Harmonic { p0, pa, pb, pl } = chart_pcurve(&carrier, &plane, band()).unwrap()
    else {
        panic!("a circle on a plane images harmonic")
    };
    let (da, db) = (Vec2::new(1.0, 0.0), Vec2::new(0.8, 0.3));
    let sigma = harmonic_sup(Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.8, 0.3, 0.0));
    let k = 0.8 * zero() / sigma;
    let stored = Pcurve::Harmonic {
        p0,
        pa: pa + da * k,
        pb: pb + db * k,
        pl,
    };
    certifies("plane, skew pair", &stored, (0.0, TAU), &carrier, &plane);
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
/// `√2` to its share. The two shares peak at different `t`, so the
/// residual sits under the envelope by more than an under-read of the
/// remainder alone: the under-reading scalar forms die on the other
/// scalar rows, not here.
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

/// `stored` (a projected image) certifies over `span` at `f64` and at
/// `Interval`, and its envelopes cover its residual inside the band.
fn certifies_projected(
    what: &str,
    stored: &projected::ProjectedImage<f64>,
    (t0, t1): (f64, f64),
    carrier: &Curve3<f64>,
    surface: &Surface<f64>,
) {
    let cache =
        PcurveCache::certify_projected(stored.clone(), t0, t1, carrier, surface, band(), None)
            .unwrap_or_else(|e| panic!("{what}: certifies at f64: {e:?}"));
    let f = Interval::from_f64;
    let carrier_i = carrier.map_scalar(f);
    let surface_i = surface.map_scalar(f);
    let image_i = projected::project(&carrier_i, Some((f(t0), f(t1))), &surface_i, band())
        .unwrap_or_else(|e| panic!("{what}: projects at Interval: {e:?}"));
    let image_i = shift_frame(image_i, frame_moves(stored, carrier, surface, (t0, t1)));
    let at_interval =
        PcurveCache::certify_projected(image_i, f(t0), f(t1), &carrier_i, &surface_i, band(), None)
            .unwrap_or_else(|e| panic!("{what}: certifies at Interval: {e:?}"));
    covers(
        what,
        &Pcurve::Projected(Box::new(stored.clone())),
        (t0, t1),
        carrier,
        surface,
        cache.certificate().envelope,
        at_interval.certificate().envelope.hi(),
    );
}

/// How far `stored`'s circle frame stands from the frame `project`
/// derives from `carrier`: `(Δcentre, Δa, Δb)`.
fn frame_moves(
    stored: &projected::ProjectedImage<f64>,
    carrier: &Curve3<f64>,
    surface: &Surface<f64>,
    (t0, t1): (f64, f64),
) -> [Vec3<f64>; 3] {
    let derived = projected::project(carrier, Some((t0, t1)), surface, band()).unwrap();
    match (&stored.carrier, &derived.carrier) {
        (
            projected::FramedCarrier::Circle {
                centre: c1,
                a: a1,
                b: b1,
                ..
            },
            projected::FramedCarrier::Circle {
                centre: c0,
                a: a0,
                b: b0,
                ..
            },
        ) => [*c1 - *c0, *a1 - *a0, *b1 - *b0],
        _ => unreachable!("the rows store circle frames"),
    }
}

/// `image`'s circle frame moved by `(Δcentre, Δa, Δb)`.
fn shift_frame<T: Real>(
    mut image: projected::ProjectedImage<T>,
    [dc, da, db]: [Vec3<f64>; 3],
) -> projected::ProjectedImage<T> {
    let lift = |v: Vec3<f64>| Vec3::new(T::from_f64(v.x), T::from_f64(v.y), T::from_f64(v.z));
    if let projected::FramedCarrier::Circle {
        ref mut centre,
        ref mut a,
        ref mut b,
        ..
    } = image.carrier
    {
        *centre = *centre + lift(dc);
        *a = *a + lift(da);
        *b = *b + lift(db);
    }
    image
}

/// A circle on the unit sphere, its plane `0.6` from the centre along
/// `(sin 0.7, 0, cos 0.7)`: radius `0.8`, neither a parallel nor a
/// meridian, so it images projected.
fn general_sphere_circle(shift: Vec3<f64>) -> Curve3<f64> {
    let tilt: f64 = 0.7;
    let n = Vec3::new(tilt.sin(), 0.0, tilt.cos());
    Curve3::Circle {
        center: Point3::origin() + n * 0.6 + shift,
        axis: n,
        radius: 0.8,
        u_ref: Vec3::new(tilt.cos(), 0.0, -tilt.sin()),
    }
}

fn unit_sphere() -> Surface<f64> {
    Surface::Sphere {
        center: Point3::origin(),
        radius: 1.0,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    }
}

/// `vector`, the projected circle's fidelity: a sphere's general circle
/// whose stored frame has `a` moved `δ·b̂` and `b` moved `−δ·â`, so the
/// move at `t` is `δ` along the circle's tangent, which is the sphere's
/// too: the image stands `δ` off where the coefficient sum reads `2δ`.
#[test]
fn projected_circle_a_moved_frame_reads_once() {
    let sphere = unit_sphere();
    let carrier = general_sphere_circle(Vec3::zero());
    let span = (0.3, 2.1);
    let mut stored = projected::project(&carrier, Some(span), &sphere, band()).unwrap();
    let d = 0.75 * zero();
    if let projected::FramedCarrier::Circle {
        ref mut a,
        ref mut b,
        ..
    } = stored.carrier
    {
        let (ua, ub) = (a.normalize(), b.normalize());
        *a = *a + ub * d;
        *b = *b - ua * d;
    }
    certifies_projected("projected circle, frame", &stored, span, &carrier, &sphere);
}

/// `scalar`, the projected circle's incidence: the same circle with its
/// centre moved `s` in its own plane at 45° between `a` and `b`, so
/// `|C(t) − s|² − R² = s² + 2·r·s·cos(t − π/4)`, a first-harmonic pair
/// of equal parts whose sum reads `√2` times its sup. The stored image
/// is the moved circle's own, so only incidence reads the move.
#[test]
fn projected_circle_an_off_sphere_pair_reads_once() {
    let sphere = unit_sphere();
    let Curve3::Circle { u_ref, axis, .. } = general_sphere_circle(Vec3::zero()) else {
        unreachable!()
    };
    let v_ref = axis.cross(u_ref);
    let s = 0.75 * zero() / 0.8;
    let carrier = general_sphere_circle((u_ref + v_ref) * (s * FRAC_1_SQRT_2));
    let span = (0.3, 2.1);
    let stored = projected::project(&carrier, Some(span), &sphere, band()).unwrap();
    certifies_projected(
        "projected circle, incidence",
        &stored,
        span,
        &carrier,
        &sphere,
    );
}

/// The axial class gates read the tilt at its exact sup, so a tilt in
/// `[escalate/√2, escalate)`, which the sum read past `escalate`, is
/// now in the band and escalates at the gate: a sphere parallel and a
/// cone rim, which the sum sent to the general-circle arm's incidence
/// escalation, and a torus parallel, which it sent to the oblique arm's
/// `CarrierGrazesChart { TorusCircle }`. The gate decides first, and
/// reaching the torus's typed refusal from here would decide the class
/// on an in-band margin.
#[test]
fn a_tilt_in_the_band_escalates_its_class_gate() {
    let (center, axis, u_ref) = z_frame();
    let half_angle: f64 = 0.45;
    let cone = Surface::Cone {
        apex: center,
        axis,
        half_angle,
        u_ref,
    };
    let torus = Surface::Torus {
        center,
        axis,
        major_radius: 1.0,
        minor_radius: 0.25,
        u_ref,
    };
    let off = 0.8 * band().escalate();
    for (what, surface, rim_center, radius) in [
        (
            "cone rim",
            &cone,
            Point3::new(0.0, 0.0, 1.0),
            half_angle.tan(),
        ),
        (
            "sphere parallel",
            &unit_sphere(),
            Point3::new(0.0, 0.0, 0.6),
            0.8,
        ),
        ("torus parallel", &torus, center, 1.25),
    ] {
        let carrier = tilted_circle(rim_center, radius, (off / radius).asin());
        let got = chart_pcurve(&carrier, surface, band());
        assert!(
            matches!(
                got,
                Err(PcurveCertifyError::Escalated {
                    check: PcurveCheck::ChartWinding,
                    ..
                })
            ),
            "{what}: a tilt of 0.8·escalate escalates its class gate: {got:?}"
        );
    }
}

/// Both forms against a dense sampling of `a·cos t + b·sin t` — `⊥`
/// and of one length (the lever-2 case), parallel, one zero, and a
/// general pair — and at `Interval` on coefficients straddling zero,
/// where the result must still decide (a square spelled `x * x` would
/// dip below zero there and poison the root).
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
    let fuzz = Interval::from_bounds(-1e-20, 1e-20);
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
