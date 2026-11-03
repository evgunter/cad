//! **A line on a cone** (D3's lane choice on the cone's ruling reading,
//! `pcurve_cone_chart_nappe`): a ruling whose nappe reading lands in band
//! (its anchor at the apex's height within the band) takes the projected
//! image, whose check-4 incidence term is [`projected::line_off_cone_form`]
//! converted on the cone's lever; a line definitely off ends as its
//! incidence read. Every row holds at `f64` and at `Interval`, and at
//! every ε of the run.
//!
//! Built with `PCERT_RADIAL_PAIR_MUTANT` naming `line-cylinder-ends`
//! (the segment's form read at its ends only), the chord row refuses.

use geom_core::{Bounds, Interval};

use super::harmonic_sup_rows::{band, covers, sampled_residual, zero};
use super::*;

fn cone(half_angle: f64) -> Surface<f64> {
    Surface::Cone {
        apex: Point3::new(0.4, 0.1, -0.3),
        axis: Vec3::new(0.1, -0.2, 1.0).normalize(),
        half_angle,
        u_ref: Vec3::new(1.0, 0.0, -0.1).normalize(),
    }
}

/// The generator of `surface`'s nappe `sigma` at azimuth `phi`, and the
/// cone's outward normal there, in the world.
fn generator(surface: &Surface<f64>, phi: f64, sigma: f64) -> (Vec3<f64>, Vec3<f64>) {
    let Surface::Cone { half_angle, .. } = *surface else {
        unreachable!("the rows' chart is a cone")
    };
    let Some((_, e)) = projected::chart_frame(surface) else {
        unreachable!("a cone has a frame")
    };
    let (sa, ca) = half_angle.sin_cos();
    let radial = e[0] * phi.cos() + e[1] * phi.sin();
    (
        (radial * sa + e[2] * ca) * sigma,
        radial * ca - e[2] * (sa * sigma),
    )
}

/// The ruling of nappe `sigma` at azimuth `phi`, anchored at the point of
/// it at height `h` (along the axis from the apex), lifted `off` along the
/// cone's normal, at unit speed.
fn ruling(surface: &Surface<f64>, phi: f64, sigma: f64, h: f64, off: f64) -> Curve3<f64> {
    let Surface::Cone {
        apex, half_angle, ..
    } = *surface
    else {
        unreachable!("the rows' chart is a cone")
    };
    let (g, n) = generator(surface, phi, sigma);
    Curve3::line(apex + g * (h / half_angle.cos()) + n * off, g)
}

/// A height whose nappe reading lies in band: the geometric mean of the
/// band's two thresholds.
fn in_band() -> f64 {
    (band().zero() * band().escalate()).sqrt()
}

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

/// The projected line image certifies at both scalars with envelopes
/// inside the band over its sampled residual; answers both envelopes.
fn certifies_projected(
    what: &str,
    carrier: &Curve3<f64>,
    span: (f64, f64),
    surface: &Surface<f64>,
) -> (f64, f64) {
    let derivation = derive_harmonic(
        carrier,
        surface,
        Placement::over(span, carrier, surface),
        band(),
    );
    assert!(
        matches!(
            derivation,
            Err(PcurveCertifyError::Escalated {
                check: PcurveCheck::ChartClass,
                ..
            })
        ),
        "{what}: the nappe reading is in band: {derivation:?}"
    );
    let (at_f64, at_interval) = derived(carrier, span, surface);
    let image = at_f64.unwrap_or_else(|e| panic!("{what}: derives at f64: {e:?}"));
    let image_i = at_interval.unwrap_or_else(|e| panic!("{what}: derives at Interval: {e:?}"));
    assert!(
        image
            .projected()
            .is_some_and(|p| matches!(p.carrier(), projected::FramedCarrier::Line { .. }))
            && image_i
                .projected()
                .is_some_and(|p| matches!(p.carrier(), projected::FramedCarrier::Line { .. })),
        "{what}: takes the projected line image at both scalars"
    );
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
    let sup = sampled_residual(&image, span, carrier, surface, 4096);
    let slack = 64.0 * f64::EPSILON;
    assert!(
        envelope < zero() && hi < zero() && envelope >= sup - slack && hi >= sup - slack,
        "{what}: the envelopes {envelope:e} (f64) and {hi:e} (Interval) cover the residual \
         {sup:e} inside the band"
    );
    if sup > zero() / 2.0 {
        covers(what, &image, span, carrier, surface, envelope, hi);
    }
    (envelope, hi)
}

/// A ruling anchored at the apex's height within the band reads its
/// nappe in band and takes the projected lane over an interval clear of
/// the apex, on both nappes, a narrow and a wide cone, and on both sides
/// of the azimuth window's cut; and one standing `0.6·zero` off the cone
/// certifies carrying its miss.
#[test]
fn a_ruling_whose_nappe_reads_in_band_certifies_through_the_projected_lane() {
    for alpha in [0.2, 1.2] {
        let surface = cone(alpha);
        for sigma in [1.0, -1.0] {
            for phi in [0.4, core::f64::consts::PI - 1e-3, -2.0] {
                for off in [0.0, 0.6 * zero()] {
                    let carrier = ruling(&surface, phi, sigma, in_band(), off);
                    certifies_projected(
                        &format!("α {alpha}, σ {sigma}, φ {phi}, off {off:e}"),
                        &carrier,
                        (0.5, 2.0),
                        &surface,
                    );
                }
            }
        }
    }
}

/// A ruling anchored in band but standing `4·escalate` off the cone ends
/// as its incidence read, before any lane.
#[test]
fn a_line_definitely_off_the_cone_ends_as_its_incidence_read() {
    let surface = cone(0.5);
    let carrier = ruling(&surface, 1.0, 1.0, in_band(), 4.0 * band().escalate());
    let (at_f64, at_interval) = derived(&carrier, (0.5, 2.0), &surface);
    assert!(
        matches!(at_f64, Err(PcurveCertifyError::CarrierOffChart { .. }))
            && matches!(at_interval, Err(PcurveCertifyError::CarrierOffChart { .. })),
        "ends as its incidence read: {at_f64:?} / {at_interval:?}"
    );
}

/// [`projected::line_off_cone_form`] covers the cone's form densely
/// sampled along a segment, at `f64` and at `Interval`: a chord between
/// two points of a rim (its middle inside the cone: red under
/// `line-cylinder-ends`), a chord across the two nappes' gap outside, a
/// ruling off the cone (tight there), and a segment crossing the cone.
#[test]
fn the_cone_form_sup_covers_the_segments_form() {
    let alpha = 0.6_f64;
    let (sa, ca) = alpha.sin_cos();
    let at =
        |phi: f64, h: f64| Vec3::new(h * alpha.tan() * phi.cos(), h * alpha.tan() * phi.sin(), h);
    let form = |q: Vec3<f64>| (q.x * q.x + q.y * q.y) * ca * ca - q.z * q.z * sa * sa;
    let rows = [
        ("a rim chord", at(-0.4, 1.0), at(0.5, 1.0)),
        (
            "a chord outside",
            at(0.3, 1.0) * 1.2 + Vec3::new(0.1, 0.0, 0.0),
            at(2.0, 1.5) + Vec3::new(0.3, 0.1, 0.0),
        ),
        (
            "a ruling off",
            at(1.0, 0.5) + Vec3::new(ca, 0.0, -sa) * 1e-7,
            at(1.0, 2.0) + Vec3::new(ca, 0.0, -sa) * 1e-7,
        ),
        ("a crossing", at(0.2, 1.0) * 0.9, at(0.2, 1.4) * 1.05),
    ];
    for (what, q0, q1) in rows {
        let sampled = (0..=4096)
            .map(|i| form(q0 + (q1 - q0) * (f64::from(i) / 4096.0)).abs())
            .fold(0.0, f64::max);
        let at_f64 = projected::line_off_cone_form(q0, q1, sa, ca);
        let f = Interval::from_f64;
        let lift = |v: Vec3<f64>| Vec3::new(f(v.x), f(v.y), f(v.z));
        let at_interval =
            projected::line_off_cone_form(lift(q0), lift(q1), f(alpha).sin(), f(alpha).cos()).hi();
        let slack = 16.0 * f64::EPSILON;
        assert!(
            at_f64 >= sampled - slack && at_interval >= sampled - slack,
            "{what}: the sup {at_f64:e} (f64), {at_interval:e} (Interval) covers the sampled \
             form {sampled:e}"
        );
    }
}
