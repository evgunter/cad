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

pub(super) fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

pub(super) fn zero() -> f64 {
    band().zero()
}

pub(super) fn lift_pcurve(p: &Pcurve<f64>) -> Pcurve<Interval> {
    let f = Interval::from_f64;
    match *p {
        Pcurve::Harmonic { u, v: w } => {
            let lift = |c: Channel<f64>| match c {
                Channel::Angle { phase, winding } => Channel::Angle {
                    phase: f(phase),
                    winding,
                },
                Channel::Free { c, a, b, l } => Channel::Free {
                    c: f(c),
                    a: f(a),
                    b: f(b),
                    l: f(l),
                },
            };
            Pcurve::Harmonic {
                u: lift(u),
                v: lift(w),
            }
        }
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
pub(super) fn certifies(
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
pub(super) fn covers(
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

pub(super) fn circle(
    center: Point3<f64>,
    axis: Vec3<f64>,
    radius: f64,
    u_ref: Vec3<f64>,
) -> Curve3<f64> {
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
pub(super) fn tilted_circle(center: Point3<f64>, radius: f64, phi: f64) -> Curve3<f64> {
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
    let Some((p0, pa, pb, pl)) = chart_pcurve(&carrier, &plane, band())
        .unwrap()
        .harmonic_form()
    else {
        panic!("a circle on a plane images harmonic")
    };
    let (da, db) = (Vec2::new(1.0, 0.0), Vec2::new(0.8, 0.3));
    let sigma = harmonic_sup(Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.8, 0.3, 0.0));
    let k = 0.8 * zero() / sigma;
    let stored = Pcurve::free_harmonic(p0, pa + da * k, pb + db * k, pl);
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
        semi_u: a.norm(),
        semi_v: d,
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
        ("cylinder", &cyl, Derivation::CylinderMoving { beta: 0 }),
        ("torus", &torus, Derivation::TorusParallel { beta: 0 }),
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
/// its axial channel's `cos t` and `sin t` coefficients each moved
/// `δ/√2`, so the image stands up to `δ` off the one re-derived. (The
/// azimuth is an angle channel, a phase and its windings, with no
/// `cos`/`sin` part to move.)
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
    let Pcurve::Harmonic {
        u,
        v: Channel::Free { c, a, b, l },
    } = chart_pcurve(&carrier, &cyl, band()).unwrap()
    else {
        panic!("a rim images harmonic, its axial channel free")
    };
    let d = 0.85 * zero() * FRAC_1_SQRT_2;
    let stored = Pcurve::Harmonic {
        u,
        v: Channel::Free {
            c,
            a: a + d,
            b: b + d,
            l,
        },
    };
    certifies("fidelity v", &stored, (0.0, PI), &carrier, &cyl);
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
/// in the band at the gate: a sphere parallel, a cone rim and a torus
/// parallel. A class reading in band is a lane choice not shown (D3), so
/// no class is decided on the in-band margin. These tilted circles also
/// leave the sphere and the cone at first order, so there the incidence
/// test, read before any lane, escalates on its own (`ChartIncidence`);
/// the torus parallel stays within the band of its torus, so its class
/// gate is reached and escalates lever-only "not yet" (`ChartClass`: a
/// circle's general lane on a torus is not written yet). Pinned per pose,
/// at `f64` and at `Interval`. An on-chart circle's general lane is
/// `an_in_band_class_on_the_sphere_takes_the_projected_lane`'s.
#[test]
fn a_tilt_in_the_band_decides_no_class() {
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
    for (what, surface, rim_center, radius, want) in [
        (
            "cone rim",
            &cone,
            Point3::new(0.0, 0.0, 1.0),
            half_angle.tan(),
            PcurveCheck::ChartIncidence,
        ),
        (
            "sphere parallel",
            &unit_sphere(),
            Point3::new(0.0, 0.0, 0.6),
            0.8,
            PcurveCheck::ChartIncidence,
        ),
        (
            "torus parallel",
            &torus,
            center,
            1.25,
            PcurveCheck::ChartClass,
        ),
    ] {
        let carrier = tilted_circle(rim_center, radius, (off / radius).asin());
        let at = geom_core::Interval::from_f64;
        for (lane, got) in [
            ("f64", chart_pcurve(&carrier, surface, band()).map(|_| ())),
            (
                "Interval",
                chart_pcurve(&carrier.map_scalar(at), &surface.map_scalar(at), band()).map(|_| ()),
            ),
        ] {
            assert!(
                matches!(
                    got,
                    Err(PcurveCertifyError::Escalated { check, .. }) if check == want
                ),
                "{what} at {lane}: a tilt of 0.8·escalate decides no class, as {want:?}: {got:?}"
            );
        }
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

/// **Where the general lane cannot image an on-sphere circle near a pole,
/// it refuses "not yet", never a defect** (D3; the PCERT E1 review's P3).
/// Three circles on the unit sphere passing within the band of its pole,
/// whose class reading sends them to the projected lane: a great circle
/// tilted mid-band off a meridian (its meridian reading in band), a
/// small circle on a vertical plane mid-band off the axis (definitely no
/// meridian, passing mid-band from the pole), and a small circle through
/// the pole itself. The projected lane's partition cannot put a piece
/// clear of the axis there, so it escalates or refuses its sector; the
/// lane choice reads that as its own capability gap
/// (`PcurveCheck::GeneralLane`, `NoGeneralLane { gap: LaneGap::Pose }`).
/// Every outcome, at `f64` and at `Interval`, is a certified image or a
/// "not yet" ending with no defect and no tolerance in it; and at least
/// one pose reaches the general lane's own refusal, so a routing of
/// `GeneralLane` to the defect ending reds the row.
#[test]
fn a_circle_near_a_pole_the_general_lane_cannot_image_ends_not_yet() {
    use crate::recourse::Reading;
    fn read<T: geom_core::Decide>(
        at: impl Fn(f64) -> T,
        n: Vec3<f64>,
        h: f64,
    ) -> Result<(), PcurveCertifyError> {
        let carrier = Curve3::Circle {
            center: Point3::origin() + n * h,
            axis: n,
            radius: (1.0 - h * h).sqrt(),
            u_ref: Vec3::unit_y(),
        }
        .map_scalar(&at);
        let sphere = unit_sphere().map_scalar(&at);
        let image = chart_pcurve(&carrier, &sphere, band())?;
        let tau = at(core::f64::consts::TAU);
        PcurveCache::certify(image, at(0.0), tau, &carrier, &sphere, band()).map(|_| ())
    }
    let mid = 0.5 * (band().zero() + band().escalate());
    let through_pole = Vec3::new(1.0, 0.0, 1.0).normalize();
    let poses = [
        (
            "a great circle mid-band off a meridian",
            Vec3::new(1.0, 0.0, mid).normalize(),
            0.0,
        ),
        ("a small circle mid-band off the axis", Vec3::unit_x(), mid),
        (
            "a small circle through the pole",
            through_pole,
            through_pole.z,
        ),
    ];
    let mut wrong = Vec::new();
    let mut lane_gaps = 0;
    for (what, n, h) in poses {
        for (lane, got) in [
            ("f64", read(|x| x, n, h)),
            (
                "Interval",
                read(<geom_core::Interval as geom_core::Real>::from_f64, n, h),
            ),
        ] {
            let Err(e) = got else { continue };
            if matches!(
                e,
                PcurveCertifyError::Escalated {
                    check: PcurveCheck::GeneralLane,
                    ..
                } | PcurveCertifyError::NoGeneralLane {
                    gap: LaneGap::Pose { .. },
                    ..
                }
            ) {
                lane_gaps += 1;
            }
            for reading in [Reading::Build, Reading::AtRest] {
                let ending = e.ending(reading).unwrap_or_default();
                let not_yet = ending.starts_with(geom_core::NOT_YET_ENDING)
                    && !ending.contains("defect")
                    && !ending.contains("tolerance");
                if !not_yet {
                    wrong.push(format!(
                        "{what} at {lane}, {reading:?}: {e:?} ends {ending}"
                    ));
                }
            }
        }
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
    assert!(
        lane_gaps > 0,
        "no pose reached the general lane's own refusal"
    );
}

/// **An in-band class on the sphere takes the projected lane** (D3's lane
/// choice): a circle exactly on the sphere, tilted off a parallel so its
/// axial reading (`pcurve_sphere_chart_axial`) sits mid-band, is neither
/// decided a parallel on that margin nor refused: the incidence test
/// reads it on the sphere, and it takes the general lane, the projected
/// image, which certifies over the turn, at `f64` and at `Interval`. Three
/// parallels, well clear of the poles: a northern one, the equator (a
/// great circle) and a southern one. Near a pole the lane's partition
/// refuses instead, "not yet"
/// (`a_circle_near_a_pole_the_general_lane_cannot_image_ends_not_yet`).
#[test]
fn an_in_band_class_on_the_sphere_takes_the_projected_lane() {
    fn read<T: geom_core::Decide>(
        at: impl Fn(f64) -> T,
        h: f64,
    ) -> Result<Pcurve<T>, PcurveCertifyError> {
        let rho = (1.0 - h * h).sqrt();
        let mid = 0.5 * (band().zero() + band().escalate());
        let theta = (mid / rho).asin();
        let (s, c) = theta.sin_cos();
        let n = Vec3::new(0.0, -s, c);
        let carrier = Curve3::Circle {
            center: Point3::origin() + n * h,
            axis: n,
            radius: rho,
            u_ref: Vec3::unit_x(),
        }
        .map_scalar(&at);
        let sphere = unit_sphere().map_scalar(&at);
        let image = chart_pcurve(&carrier, &sphere, band())?;
        let tau = at(core::f64::consts::TAU);
        PcurveCache::certify(image.clone(), at(0.0), tau, &carrier, &sphere, band())?;
        Ok(image)
    }
    for h in [0.6, 0.0, -0.6] {
        for (lane, projected) in [
            ("f64", matches!(read(|x| x, h), Ok(Pcurve::Projected(_)))),
            (
                "Interval",
                matches!(
                    read(<geom_core::Interval as geom_core::Real>::from_f64, h),
                    Ok(Pcurve::Projected(_))
                ),
            ),
        ] {
            assert!(
                projected,
                "{lane}, height {h}: the in-band class takes the projected lane"
            );
        }
    }
}
