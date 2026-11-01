//! **`topo`'s period sites agree with the spellings they replaced** (the
//! PRED charter's agreement test, `geom_core::period`'s turn rule). Each
//! old spelling is an oracle here, bit for bit and generic over the
//! scalar as the shipped code was, run against the shipped site on the
//! cases it was written for: a full turn; `τ ± k·e/rate` for
//! `k ∈ {0.5, 3, 20}` at both band edges `e`; a short window; and a
//! zero-width hull. Every site here reads a hull, so no case is
//! reversed: a hull's ends are folded by `Extent::between`, whatever order
//! they arrive in. The certified readers' rows are in `geom-brep`
//! (`tests/certify_turn_rule.rs`).
#![allow(clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, TAU};

use geom_core::k_stats::decide;
use geom_core::period::Extent;
use geom_core::{Band, Decide, Indeterminate, Margin, Sign, Tol};

use crate::boolean::solid_contain::trim_window_short;
use crate::chart_region::{ChartRegionError, cyl_wrap_whole, seam_span_over};

fn band() -> Band {
    Band::linear(Tol::witness()).expect("the witness band")
}

/// The defects a turn site is read at, labelled by `k·e/ε`, in radians
/// at `rate`: none (a full turn), and `±k·e/rate` for `k ∈ {0.5, 3, 20}`
/// at both band edges. Positive is past a turn.
fn defects(band: Band, rate: f64) -> Vec<(f64, f64)> {
    let mut out = vec![(0.0, 0.0)];
    for edge in [band.zero(), band.escalate()] {
        for k in [0.5, 3.0, 20.0] {
            for sign in [1.0, -1.0] {
                out.push((
                    sign * k * (edge / band.zero()).round(),
                    sign * k * edge / rate,
                ));
            }
        }
    }
    out
}

/// The windows a hull site is read at, starting at `lo`: every defect, a
/// quarter turn and none.
fn windows(band: Band, rate: f64, lo: f64) -> Vec<(f64, (f64, f64))> {
    let mut out: Vec<_> = defects(band, rate)
        .into_iter()
        .map(|(k, d)| (k, (lo, lo + TAU + d)))
        .collect();
    out.push((f64::NAN, (lo, lo + FRAC_PI_2)));
    out.push((f64::NAN, (lo, lo)));
    out
}

/// An escalation's point margin.
fn margin(cause: Indeterminate) -> Option<f64> {
    cause.margin.diagnostic_f64_for_error_text().value()
}

fn hull(lo: f64, hi: f64) -> Extent<f64> {
    Extent::between(lo, hi)
}

/// **`bool_sphere_trim_period` agrees with its old spelling, bit for
/// bit**: the headroom `(τ − (raw.1 − raw.0))·R` definitely positive.
#[test]
fn the_sphere_trim_window_agrees_with_the_old_headroom() {
    fn old<T: Decide>(raw: (T, T), radius: T, band: Band) -> Result<bool, Option<f64>> {
        let headroom = Margin::levered(T::tau() - (raw.1 - raw.0), radius);
        decide("bool_sphere_trim_period", headroom, band)
            .map(|sign| sign == Sign::Positive)
            .map_err(margin)
    }
    let band = band();
    let radius = 4.0;
    for (k, raw) in windows(band, radius, 0.9) {
        let new = trim_window_short(raw, radius, band).map_err(margin);
        assert_eq!(new, old(raw, radius, band), "k = {k}, {raw:?}");
    }
}

fn chart_margin(error: ChartRegionError) -> Option<f64> {
    match error {
        ChartRegionError::Escalated(cause) => margin(cause),
        other => panic!("a turn site escalates or decides: {other:?}"),
    }
}

/// **`chart_region`'s two excess forms agree with their old spellings,
/// but for the listed delta**: `chart_region_cyl_wrap` (the excess
/// `(span − τ)·r` within the zero band) and `chart_region_seam_span`
/// (the excess definitely positive).
///
/// **The expected delta**, each site's: the margin is the headroom
/// `τ − span` where it was the excess `span − τ`, so every in-band
/// escalation carries the old margin's negation. No classification
/// moves.
#[test]
fn the_chart_region_excess_forms_agree_with_the_old_excess() {
    fn old_wrap<T: Decide>(span: T, radius: T, band: Band) -> Result<bool, Option<f64>> {
        decide(
            "chart_region_cyl_wrap",
            Margin::levered(span - T::tau(), radius),
            band,
        )
        .map(|sign| sign == Sign::Zero)
        .map_err(margin)
    }
    fn old_seam<T: Decide>(span: T, radius: T, band: Band) -> Result<bool, Option<f64>> {
        decide(
            "chart_region_seam_span",
            Margin::levered(span - T::tau(), radius),
            band,
        )
        .map(|sign| sign == Sign::Positive)
        .map_err(margin)
    }
    let band = band();
    let radius = 0.6;
    let mut negated = 0;
    for (k, (lo, hi)) in windows(band, radius, 2.0) {
        let span = hi - lo;
        let pairs = [
            (
                "wrap",
                cyl_wrap_whole(hull(lo, hi), radius, band).map_err(chart_margin),
                old_wrap(span, radius, band),
            ),
            (
                "seam",
                seam_span_over(hull(lo, hi), radius, band).map_err(chart_margin),
                old_seam(span, radius, band),
            ),
        ];
        for (site, new, old) in pairs {
            match (new, old) {
                (Err(Some(m_new)), Err(Some(m_old))) => {
                    negated += 1;
                    assert_eq!(m_new, -m_old, "{site}, k = {k}: the margin is negated");
                }
                (new, old) => assert_eq!(new, old, "{site}, k = {k}"),
            }
        }
    }
    assert_eq!(
        negated, 8,
        "the in-band cases: 3·ε and 5·ε, either side, both sites"
    );
}

/// **A wrap band's rectangle agrees with its old read, through the
/// forward door**: `wrap_band` on a rectangle whose azimuth span is a
/// turn off by every defect, or near zero by every multiple. The old read
/// was the excess `(span − τ)·r` within the zero band; a span near zero
/// read definitely short and was no band.
///
/// **The expected delta**: an in-band escalation carries the old
/// margin's negation (the headroom, where it was the excess). A span in
/// the band near zero is no band, as it was: the forward door's reading
/// is in the band there, and the answer supersedes its escalation, so
/// none is left on the record — escalating would refuse a rectangle that
/// is certainly no whole turn.
#[test]
fn the_wrap_band_agrees_with_the_old_excess_through_its_forward_door() {
    use crate::chart_region::wrap_band;
    use geom_core::Point2;
    use geom_core::k_stats::Bracket;
    fn old<T: Decide>(span: T, radius: T, band: Band) -> Result<bool, Option<f64>> {
        decide(
            "chart_region_cyl_wrap",
            Margin::levered(span - T::tau(), radius),
            band,
        )
        .map(|sign| sign == Sign::Zero)
        .map_err(margin)
    }
    let band = band();
    let radius = 0.8;
    let (u0, v0, v1) = (0.4, -1.0, 2.0);
    let mut spans: Vec<f64> = defects(band, radius)
        .iter()
        .map(|&(_, d)| TAU + d)
        .collect();
    let near_zero: Vec<f64> = defects(band, radius)
        .iter()
        .filter(|&&(k, _)| k > 0.0)
        .map(|&(_, d)| d)
        .collect();
    spans.extend(&near_zero);
    let mut negated = 0;
    for span in spans {
        let u1 = u0 + span;
        let corners = [(u0, v0), (u1, v0), (u1, v1), (u0, v1)].map(|(u, v)| Point2::new(u, v));
        let bracket = Bracket::open();
        let new = wrap_band(&corners, radius, band)
            .map(|band| band.is_some())
            .map_err(chart_margin);
        let recorded = bracket.finish();
        match (new, old(u1 - u0, radius, band)) {
            (Err(Some(m_new)), Err(Some(m_old))) => {
                negated += 1;
                assert_eq!(m_new, -m_old, "span {span}: the margin is negated");
            }
            (new, old) => assert_eq!(new, old, "span {span}"),
        }
        if near_zero.contains(&span) {
            assert!(
                recorded.escalations.is_empty(),
                "span {span}: no band leaves no escalation"
            );
        }
    }
    assert_eq!(negated, 4, "the in-band defects: 3·ε and 5·ε, either side");
}
