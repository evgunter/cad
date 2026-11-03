//! **A line on a cylinder** (D3's lane choice on the ruling reading,
//! `pcurve_chart_radial_moving`): a ruling shown takes the meridian
//! form; a ruling in band, and a line shown off its rulings that the
//! incidence read does not show off the wall, take the projected image,
//! whose check-4 incidence term is [`projected::line_off_cylinder_sup`];
//! a line definitely off ends as its incidence read. Every row holds at
//! `f64` and at `Interval`, and at every ε of the run.
//!
//! Built with `PCERT_RADIAL_PAIR_MUTANT` naming `radial-fold` (an in-band
//! ruling folded onto the meridian form), the in-band rows refuse; naming
//! `line-cylinder-ends` (the incidence sup read at the segment's ends
//! only), the chord row refuses.
use core::f64::consts::PI;

use geom_core::{Bounds, Interval};

use super::harmonic_sup_rows::{band, covers, zero};
use super::*;

fn cylinder(radius: f64) -> Surface<f64> {
    Surface::Cylinder {
        origin: Point3::new(0.25, -0.5, 1.0),
        axis: Vec3::unit_z(),
        radius,
        u_ref: Vec3::unit_x(),
    }
}

/// A tilt angle whose radial reading on a unit-speed line lies in the
/// band: the geometric mean of its two thresholds.
fn in_band() -> f64 {
    (band().zero() * band().escalate()).sqrt()
}

/// The line through the wall point at azimuth `phi` lifted `off` past
/// the radius, tilted along the wall's tangent by `tilt` off the axis,
/// at unit speed. Over `t ∈ [−h, h]` it leaves the radius `R + off` by
/// at most `tilt²·h²/(2R)`.
fn line_at(surface: &Surface<f64>, phi: f64, off: f64, tilt: f64) -> Curve3<f64> {
    let Surface::Cylinder { origin, radius, .. } = *surface else {
        unreachable!("the rows' chart is a cylinder")
    };
    let (s, c) = phi.sin_cos();
    let at = origin + Vec3::new(c, s, 0.0) * (radius + off) + Vec3::new(0.0, 0.0, 0.3);
    Curve3::line(
        at,
        (Vec3::new(-s, c, 0.0) * tilt + Vec3::unit_z()).normalize(),
    )
}

/// The image `chart_pcurve_over` derives for `carrier` over `span`, at
/// `f64` and at `Interval`.
fn derived(
    carrier: &Curve3<f64>,
    (t0, t1): (f64, f64),
    surface: &Surface<f64>,
) -> (
    Result<Pcurve<f64>, PcurveCertifyError>,
    Result<Pcurve<Interval>, PcurveCertifyError>,
) {
    let f = Interval::from_f64;
    (
        chart_pcurve_over(carrier, t0, t1, surface, band()),
        chart_pcurve_over(
            &carrier.map_scalar(f),
            f(t0),
            f(t1),
            &surface.map_scalar(f),
            band(),
        ),
    )
}

/// The projected line image certifies at both scalars, with an envelope
/// inside the band that covers its sampled residual; answers the f64
/// envelope.
fn certifies_projected(
    what: &str,
    carrier: &Curve3<f64>,
    span: (f64, f64),
    surface: &Surface<f64>,
) -> f64 {
    let (at_f64, at_interval) = derived(carrier, span, surface);
    let image = at_f64.unwrap_or_else(|e| panic!("{what}: derives at f64: {e:?}"));
    let image_i = at_interval.unwrap_or_else(|e| panic!("{what}: derives at Interval: {e:?}"));
    for (scalar, line) in [
        (
            "f64",
            image
                .projected()
                .is_some_and(|p| matches!(p.carrier(), projected::FramedCarrier::Line { .. })),
        ),
        (
            "Interval",
            image_i
                .projected()
                .is_some_and(|p| matches!(p.carrier(), projected::FramedCarrier::Line { .. })),
        ),
    ] {
        assert!(line, "{what}: {scalar} takes the projected line image");
    }
    let f = Interval::from_f64;
    let cache = PcurveCache::certify(image.clone(), span.0, span.1, carrier, surface, band())
        .unwrap_or_else(|e| panic!("{what}: certifies at f64: {e:?}"));
    let cache_i = PcurveCache::certify(
        image_i,
        f(span.0),
        f(span.1),
        &carrier.map_scalar(f),
        &surface.map_scalar(f),
        band(),
    )
    .unwrap_or_else(|e| panic!("{what}: certifies at Interval: {e:?}"));
    let (envelope, hi) = (
        cache.certificate().envelope,
        cache_i.certificate().envelope.hi(),
    );
    assert!(
        envelope < zero() && hi < zero(),
        "{what}: the envelopes {envelope:e} (f64) and {hi:e} (Interval) are inside the band"
    );
    let sup = sampled_residual(&image, span, carrier, surface);
    let slack = 64.0 * f64::EPSILON;
    assert!(
        envelope >= sup - slack && hi >= sup - slack,
        "{what}: the envelopes {envelope:e} (f64) and {hi:e} (Interval) cover the sampled \
         residual {sup:e}"
    );
    envelope
}

fn sampled_residual(
    image: &Pcurve<f64>,
    (t0, t1): (f64, f64),
    carrier: &Curve3<f64>,
    surface: &Surface<f64>,
) -> f64 {
    (0..=4096)
        .map(|i| {
            let t = t0 + (t1 - t0) * f64::from(i) / 4096.0;
            let q = image.eval(t);
            surface.eval(q.x, q.y).distance(carrier.eval(t))
        })
        .fold(0.0, f64::max)
}

/// A ruling whose tilt along the wall reads in the band takes the
/// projected lane and certifies, on both sides of the azimuth window's
/// cut and on a thin and a wide cylinder. Red under `radial-fold`, which
/// takes the meridian form there.
#[test]
fn a_ruling_in_band_certifies_through_the_projected_lane() {
    for radius in [0.05, 3.0] {
        let surface = cylinder(radius);
        for phi in [0.3, PI - 1e-3, -PI + 1e-3, -1.2] {
            let carrier = line_at(&surface, phi, 0.0, in_band());
            let Some(form) = carrier_harmonic(&carrier) else {
                unreachable!("a line has a harmonic form")
            };
            let moving = radial_amplitude(&form, Vec3::unit_z());
            assert!(
                (band().zero()..band().escalate()).contains(&moving),
                "R {radius}, φ {phi}: the ruling reading {moving:e} is in the band"
            );
            certifies_projected(
                &format!("R {radius}, φ {phi}"),
                &carrier,
                (-0.5, 0.5),
                &surface,
            );
        }
    }
}

/// A ruling in band standing `0.6·zero` off the wall: its incidence
/// reads on, and the projected envelope carries the miss whole, inside
/// the band, at both scalars.
#[test]
fn a_ruling_in_band_off_the_wall_inside_the_band_certifies_carrying_its_miss() {
    let surface = cylinder(1.5);
    let carrier = line_at(&surface, 2.0, 0.6 * zero(), in_band());
    let span = (-0.5, 0.5);
    let (Ok(image), Ok(image_i)) = derived(&carrier, span, &surface) else {
        panic!("the ruling in band derives at both scalars")
    };
    assert!(
        image.projected().is_some() && image_i.projected().is_some(),
        "the ruling in band takes the projected lane"
    );
    let f = Interval::from_f64;
    let cache = PcurveCache::certify(image.clone(), span.0, span.1, &carrier, &surface, band())
        .unwrap_or_else(|e| panic!("certifies at f64: {e:?}"));
    let cache_i = PcurveCache::certify(
        image_i,
        f(span.0),
        f(span.1),
        &carrier.map_scalar(f),
        &surface.map_scalar(f),
        band(),
    )
    .unwrap_or_else(|e| panic!("certifies at Interval: {e:?}"));
    covers(
        "a ruling in band 0.6·zero off",
        &image,
        span,
        &carrier,
        &surface,
        cache.certificate().envelope,
        cache_i.certificate().envelope.hi(),
    );
}

/// A ruling shown (no tilt at all), on the wall or `0.6·zero` off it,
/// takes the meridian form and certifies at both scalars; its family is
/// the rulings'.
#[test]
fn a_shown_ruling_takes_the_meridian_form() {
    let surface = cylinder(1.5);
    for off in [0.0, 0.6 * zero()] {
        let carrier = line_at(&surface, -2.5, off, 0.0);
        let span = (-0.5, 0.5);
        let (Ok(image), Ok(image_i)) = derived(&carrier, span, &surface) else {
            panic!("off {off:e}: the shown ruling derives at both scalars")
        };
        assert!(
            matches!(image, Pcurve::Harmonic { .. }) && matches!(image_i, Pcurve::Harmonic { .. }),
            "off {off:e}: the shown ruling takes the meridian form: {image:?}"
        );
        let derivation = derive_harmonic(
            &carrier,
            &surface,
            Placement::over(span, &carrier, &surface),
            band(),
        )
        .map(|(_, d)| d);
        assert!(
            matches!(derivation, Ok(Derivation::CylinderMeridian)),
            "off {off:e}: {derivation:?}"
        );
        if off > 0.0 {
            super::harmonic_sup_rows::certifies(
                "a shown ruling off the wall",
                &image,
                span,
                &carrier,
                &surface,
            );
        } else {
            let f = Interval::from_f64;
            PcurveCache::certify(image.clone(), span.0, span.1, &carrier, &surface, band())
                .unwrap_or_else(|e| panic!("a shown ruling certifies at f64: {e:?}"));
            PcurveCache::certify(
                image_i,
                f(span.0),
                f(span.1),
                &carrier.map_scalar(f),
                &surface.map_scalar(f),
                band(),
            )
            .unwrap_or_else(|e| panic!("a shown ruling certifies at Interval: {e:?}"));
        }
        assert_eq!(
            chart_iso_family(&carrier, &surface, band()),
            Ok(Some(IsoFamily::U)),
            "off {off:e}: a shown ruling's family"
        );
    }
}

/// A line whose speed along the wall's tangent is `100·escalate` per
/// unit of its parameter shows it is no ruling, yet over its own
/// interval `[0, 1]` it leaves the wall by `(100·escalate)²/8R`, inside
/// the zero band: the general lane, not the moving class's form, which
/// a line has none of. Its family is none.
#[test]
fn a_line_shown_off_its_rulings_on_the_wall_takes_the_projected_lane() {
    let surface = cylinder(2.0);
    let wall = line_at(&surface, 0.7, 0.0, 0.0);
    let Curve3::Line(geom::Line { origin: at, .. }) = wall else {
        unreachable!("line_at builds a line")
    };
    let (s, c) = 0.7_f64.sin_cos();
    let dir = Vec3::unit_z() + Vec3::new(-s, c, 0.0) * (100.0 * band().escalate());
    let carrier = Curve3::line(at - dir * 0.5, dir);
    let Some(form) = carrier_harmonic(&carrier) else {
        unreachable!("a line has a harmonic form")
    };
    assert!(
        radial_amplitude(&form, Vec3::unit_z()) > band().escalate(),
        "the ruling reading is definite"
    );
    certifies_projected("a line off its rulings", &carrier, (0.0, 1.0), &surface);
    assert_eq!(chart_iso_family(&carrier, &surface, band()), Ok(None));
}

/// A line definitely off the wall ends as its incidence read, before any
/// class reading: parallel to the axis past the band, and tilted far.
#[test]
fn a_line_definitely_off_ends_as_its_incidence_read() {
    let surface = cylinder(1.0);
    for (what, carrier) in [
        (
            "parallel",
            line_at(&surface, 1.0, 4.0 * band().escalate(), 0.0),
        ),
        (
            "in-band tilt",
            line_at(&surface, 1.0, 4.0 * band().escalate(), in_band()),
        ),
        ("tilted", line_at(&surface, 1.0, 0.0, 0.2)),
    ] {
        let (at_f64, at_interval) = derived(&carrier, (-0.5, 0.5), &surface);
        assert!(
            matches!(at_f64, Err(PcurveCertifyError::CarrierOffChart { .. }))
                && matches!(at_interval, Err(PcurveCertifyError::CarrierOffChart { .. })),
            "{what}: ends as its incidence read: {at_f64:?} / {at_interval:?}"
        );
    }
}

/// An in-band ruling's family is the question the join asks, undecided;
/// red under `radial-fold`, which names it a ruling.
#[test]
fn a_ruling_in_band_names_no_family_undecided() {
    let surface = cylinder(1.0);
    let carrier = line_at(&surface, 0.4, 0.0, in_band());
    assert!(
        matches!(
            chart_iso_family(&carrier, &surface, band()),
            Err(IsoFamilyRefusal::Undecided(_))
        ),
        "{:?}",
        chart_iso_family(&carrier, &surface, band())
    );
}

/// [`projected::line_off_cylinder_sup`] covers the segment's densely
/// sampled distance from the cylinder at `f64` and at `Interval`, on a
/// chord dipping toward the axis (its ends on the wall, its middle
/// `R(1 − cos a)` inside: red under `line-cylinder-ends`), a chord
/// outside, a ruling off the wall (tight there), and a segment crossing
/// the wall.
#[test]
fn the_line_incidence_sup_covers_the_segments_distance() {
    let r = 1.3;
    let a = 0.2_f64;
    let (s, c) = a.sin_cos();
    let rows = [
        (
            "a chord inside",
            Vec3::new(r * c, -r * s, 0.1),
            Vec3::new(r * c, r * s, 0.4),
        ),
        (
            "a chord outside",
            Vec3::new(1.2 * r, -0.3, 0.0),
            Vec3::new(1.1 * r, 0.4, 2.0),
        ),
        (
            "a ruling off",
            Vec3::new(0.0, r + 1e-7, -1.0),
            Vec3::new(0.0, r + 1e-7, 1.0),
        ),
        (
            "a crossing",
            Vec3::new(0.9 * r, 0.0, 0.0),
            Vec3::new(1.05 * r, 0.2, 0.5),
        ),
    ];
    for (what, q0, q1) in rows {
        let sampled = (0..=4096)
            .map(|i| {
                let t = f64::from(i) / 4096.0;
                let q = q0 + (q1 - q0) * t;
                (q.x.hypot(q.y) - r).abs()
            })
            .fold(0.0, f64::max);
        let at_f64 = projected::line_off_cylinder_sup(q0, q1, r);
        let f = Interval::from_f64;
        let lift = |v: Vec3<f64>| Vec3::new(f(v.x), f(v.y), f(v.z));
        let at_interval = projected::line_off_cylinder_sup(lift(q0), lift(q1), f(r)).hi();
        let slack = 16.0 * f64::EPSILON * r;
        assert!(
            at_f64 >= sampled - slack && at_interval >= sampled,
            "{what}: the sup {at_f64:e} (f64), {at_interval:e} (Interval) covers the sampled \
             distance {sampled:e}"
        );
        // The middle coefficient sits `|q1 − q0|²/4` past the form's
        // least value, so a ruling's sup is tight and a long chord's is
        // not.
        if what == "a ruling off" {
            assert!(
                at_f64 <= 1.01 * sampled + slack,
                "{what}: the sup {at_f64:e} is tight to the sampled {sampled:e}"
            );
        }
    }
}
