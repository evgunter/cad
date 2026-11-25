//! **An iso image holds a line of its chart as structure, and states no
//! ends** ([`Pcurve::IsoLine`], [`geom_brep::ChartLine`]): the held value
//! reads as the mint placed it at every scalar, an end is the map's value
//! there (an enclosure, at an enclosure scalar), and a map that runs to
//! where its carrier does not is refused at certification. A seam column
//! moved one period of a closed chart is its domain's other end itself
//! ([`Period::of_domain`]).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::tol::band;
use geom::{Curve3, NurbsCurve2, NurbsSurface, Surface};
use geom_brep::{ChartAxis, Pcurve, PcurveCache, PcurveCertifyError, PcurveCheck, Period};
use geom_core::spline::KnotVector;
use geom_core::{Bounds, Decide, Interval, Point2, Point3, Real};
use std::sync::Arc;

/// `x` as a point bracket, or `None` where it is wider.
fn point<T: Bounds>(x: T) -> Option<f64> {
    (x.lo() == x.hi()).then(|| x.lo())
}

/// A span end that is a computed length, `1.2 + 1`: at `Interval` an
/// enclosure of `2.2` some ulps wide, as a line rim's own end is.
fn computed_length<T: Real>() -> T {
    T::from_f64(1.2) + T::one()
}

/// **A minted row holds its line as a point, and its end is its map's**,
/// at `f64` and at `Interval`, over a span whose end is a computed
/// length: the held `v` is exactly the value placed, with the axis read
/// off the image and no slope asked; the map read at the span's end
/// encloses `u = 1`, and at `Interval` is no point, which is why a corner
/// is the meet of two held lines and never an image's end.
///
/// Red under a mint that places the held value through arithmetic
/// (`Pcurve::iso_row` holding `v·(t1 − t0)/(t1 − t0)`: at `Interval` the
/// held value stops being a point).
#[test]
fn a_minted_row_holds_its_line_exactly_and_its_end_is_its_maps() {
    fn at<T: Decide + Bounds>(scalar: &str) -> bool {
        let span = (T::zero(), computed_length::<T>());
        let row = Pcurve::iso_row(T::one(), (T::zero(), T::one()), span);
        let line = row.held_line().expect("a minted row holds a line").line;
        assert_eq!(line.axis, ChartAxis::V, "{scalar}: a row holds v");
        assert_eq!(
            point(line.value),
            Some(1.0),
            "{scalar}: the held value is the one placed"
        );
        let end = row.eval(span.1);
        assert_eq!(
            point(end.y),
            Some(1.0),
            "{scalar}: every point of the image holds the line"
        );
        assert!(
            end.x.lo() <= 1.0 && 1.0 <= end.x.hi(),
            "{scalar}: the map's end encloses the corner: {:?}",
            (end.x.lo(), end.x.hi())
        );
        point(end.x).is_some()
    }
    at::<f64>("f64");
    assert!(
        !at::<Interval>("Interval"),
        "at Interval the map's end over a computed span is an enclosure with width"
    );
}

/// **The held axis is the variant's, never a zero read off a slope.** A
/// straight curve in UV whose controls share their `v` exactly lies on a
/// row of the chart, and its `v` slope is an exact zero; it is a
/// [`Pcurve::General`], which no mint held to a line, and it holds none.
/// An iso image whose rate is an exact zero still holds its line: the
/// line is what it stores.
///
/// Red under a `held_line` that reads a `General` image's shared control
/// coordinate (or any slope's zero) as a held line.
#[test]
fn an_image_no_mint_held_to_a_line_holds_none() {
    let flat = NurbsCurve2::new(
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        vec![Point2::new(0.0, 0.25), Point2::new(1.0, 0.25)],
        vec![1.0, 1.0],
    )
    .unwrap();
    assert!(
        Pcurve::General(Arc::new(flat)).held_line().is_none(),
        "a curve in UV with a computed-zero slope is no held line"
    );
    let still = Pcurve::IsoLine {
        line: geom_brep::ChartLine::column(0.5_f64),
        at0: 0.25,
        rate: 0.0,
    };
    let held = still.held_line().expect("an iso image holds its line");
    assert_eq!(
        (held.line.axis, held.line.value, held.rate),
        (ChartAxis::U, 0.5, 0.0),
        "an iso image with a zero rate holds the line it stores"
    );
}

/// The unit square `(u, v) ↦ (u, v, 0)`, bilinear, its domain `[0, 1]²`.
fn unit_chart<T: Real>() -> Surface<T> {
    let k = || KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control = [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)]
        .map(|(x, y)| Point3::new(x, y, 0.0))
        .to_vec();
    let net = NurbsSurface::new(k(), k(), control, vec![1.0; 4]).unwrap();
    Surface::Nurbs(Arc::new(net.map_scalar(T::from_f64)))
}

/// The line `(t, 0, 0)`: the unit chart's row `v = 0` at unit speed.
fn row_carrier<T: Real>() -> Curve3<T> {
    Curve3::line(
        Point3::new(T::zero(), T::zero(), T::zero()),
        geom_core::UnitVec3::exact_x(),
    )
}

/// A refusal as its variant, its check and its sample; `None` for any
/// other outcome.
fn refusal<T: Real>(
    got: &Result<PcurveCache<T>, PcurveCertifyError>,
) -> Option<(&'static str, PcurveCheck, u32)> {
    match got {
        Err(PcurveCertifyError::ResidualExceeded { check, sample }) => {
            Some(("exceeded", *check, *sample))
        }
        Err(PcurveCertifyError::Escalated { check, sample, .. }) => {
            Some(("escalated", *check, *sample))
        }
        _ => None,
    }
}

/// The unit square `(u, v) ↦ (u, v, 0)` with a knot at `u = 7/8`: the
/// unit chart, in the spline space whose rows break there.
fn chart_knotted_at_seven_eighths<T: Real>() -> Surface<T> {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.875, 1.0, 1.0], 1).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control = [0.0, 0.875, 1.0]
        .into_iter()
        .flat_map(|x| [Point3::new(x, 0.0, 0.0), Point3::new(x, 1.0, 0.0)])
        .collect();
    let net = NurbsSurface::new(ku, kv, control, vec![1.0; 6]).unwrap();
    Surface::Nurbs(Arc::new(net.map_scalar(T::from_f64)))
}

/// The row `v = 0` of [`chart_knotted_at_seven_eighths`] in that row's
/// own spline space, keeping unit speed to `t = 7/8` and ending at
/// `u = end`: the chart's own row for `end = 1`, and for a lesser `end`
/// the carrier of an edge whose last eighth stops short of the chart's
/// corner.
fn row_carrier_ending_at<T: Real>(end: f64) -> Curve3<T> {
    let c = geom::NurbsCurve3::new(
        KnotVector::clamped(vec![0.0, 0.0, 0.875, 1.0, 1.0], 1).unwrap(),
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.875, 0.0, 0.0),
            Point3::new(end, 0.0, 0.0),
        ],
        vec![1.0; 3],
    )
    .unwrap();
    Curve3::Nurbs(Arc::new(c.map_scalar(T::from_f64)))
}

/// **An image whose carrier misses its edge's end is refused.** The row
/// `v = 0` over an edge that stops at `u = 0.9`: minted where its edge
/// ends it certifies; minted to the chart's corner `u = 1` its map runs
/// to where the carrier does not, and check 3 refuses it on the schedule
/// at both scalars. The map is affine, so against a line carrier it
/// leaves from the first station on, and the schedule stops at the
/// first station past the band, sample 1.
///
/// **The end station alone**: on the chart knotted at `u = 7/8`, a
/// carrier in the row's own spline space that keeps the row's speed to
/// `t = 7/8` and stops a twentieth short in its last eighth agrees with
/// the map at the first eight stations and misses it only at the ninth,
/// so the refusal names sample 8: the span's end is a station of the
/// schedule, read through the map like every other. An image states no
/// end apart from its map, so there is no corner to state past the edge
/// any other way.
///
/// Red under check 3 reading no station (`schedule_residuals` returning
/// before its loop): the refusal is then check 4's envelope, not the
/// schedule's; and under the schedule stopping a station early (eight
/// stations of the nine): the end-station body certifies or ends on the
/// envelope.
#[test]
fn an_image_whose_carrier_misses_its_edges_end_is_refused() {
    fn at<T: Decide>(scalar: &str) {
        let f = T::from_f64;
        let span = (T::zero(), f(0.9));
        let certify = |image: Pcurve<T>| {
            PcurveCache::certify(image, span.0, span.1, &row_carrier(), &unit_chart(), band())
        };
        let on = certify(Pcurve::iso_row(T::zero(), (T::zero(), f(0.9)), span));
        assert_eq!(
            on.as_ref().err().map(|e| format!("{e:?}")),
            None,
            "{scalar}: the row minted where its edge ends certifies"
        );
        let got = certify(Pcurve::iso_row(T::zero(), (T::zero(), T::one()), span));
        assert_eq!(
            refusal(&got),
            Some(("exceeded", PcurveCheck::MapResidual, 1)),
            "{scalar}: a map that runs past where its edge ends is refused on the \
             schedule, at its first station off the carrier: {:?}",
            got.as_ref().err()
        );
        // The end station alone.
        let whole = (T::zero(), T::one());
        let image = || Pcurve::iso_row(T::zero(), (T::zero(), T::one()), whole);
        let certify_on = |carrier: Curve3<T>| {
            PcurveCache::certify(
                image(),
                whole.0,
                whole.1,
                &carrier,
                &chart_knotted_at_seven_eighths(),
                band(),
            )
        };
        let reaches = certify_on(row_carrier_ending_at(1.0));
        assert_eq!(
            reaches.as_ref().err().map(|e| format!("{e:?}")),
            None,
            "{scalar}: on the carrier that reaches the corner the row certifies"
        );
        let short = certify_on(row_carrier_ending_at(0.95));
        assert_eq!(
            refusal(&short),
            Some(("exceeded", PcurveCheck::MapResidual, 8)),
            "{scalar}: a carrier short of the corner in its last eighth alone is \
             refused at the end station: {:?}",
            short.as_ref().err()
        );
    }
    at::<f64>("f64");
    at::<Interval>("Interval");
}

/// **A map that leaves its chart's domain in band is gated.** The row
/// `v = 0` (and the column `u = 0`) of the unit chart, minted from the
/// corner to three quarters of a tolerance PAST the chart's far end (and
/// from as far before its near end) over an edge that runs the whole
/// side: the certificate does not certify it. The departure from the
/// carrier alone is in band, and a departure of that size inside the
/// domain certifies with the departure as its envelope
/// ([`a_map_past_its_carrier_in_band_is_in_the_envelope`], at half a
/// tolerance); off the domain the chart's values are its end span's
/// extension, and the domain gate adds the reach past the domain to the
/// envelope, which then stands past the band's zero and escalates as the
/// envelope. On a line carrier and on a spline one, at both scalars.
///
/// Red under the domain gate's reach dropped from the envelope (the seam
/// envelope returning its hull and slack without `stretch·over`): the
/// image certifies.
#[test]
fn a_map_that_leaves_its_charts_domain_in_band_is_gated() {
    fn at<T: Decide>(scalar: &str) {
        let f = T::from_f64;
        let past = 0.75 * crate::shared::tol::eps();
        let whole = (T::zero(), T::one());
        let line = |along_u: bool| -> Curve3<T> {
            let dir = if along_u {
                geom_core::UnitVec3::exact_x()
            } else {
                geom_core::UnitVec3::exact_y()
            };
            Curve3::line(Point3::new(T::zero(), T::zero(), T::zero()), dir)
        };
        let spline = |along_u: bool| -> Curve3<T> {
            let far = if along_u {
                Point3::new(1.0, 0.0, 0.0)
            } else {
                Point3::new(0.0, 1.0, 0.0)
            };
            let c = geom::NurbsCurve3::new(
                KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
                vec![Point3::new(0.0, 0.0, 0.0), far],
                vec![1.0, 1.0],
            )
            .unwrap();
            Curve3::Nurbs(Arc::new(c.map_scalar(T::from_f64)))
        };
        for along_u in [true, false] {
            for (carrier_name, carrier) in [("line", line(along_u)), ("spline", spline(along_u))] {
                for (which, placed) in [
                    ("past the far end", (0.0, 1.0 + past)),
                    ("before the near end", (-past, 1.0)),
                ] {
                    let placed = (f(placed.0), f(placed.1));
                    let image = if along_u {
                        Pcurve::iso_row(T::zero(), placed, whole)
                    } else {
                        Pcurve::iso_column(T::zero(), placed, whole)
                    };
                    let got = PcurveCache::certify(
                        image,
                        whole.0,
                        whole.1,
                        &carrier,
                        &unit_chart(),
                        band(),
                    );
                    assert_eq!(
                        refusal(&got),
                        Some(("escalated", PcurveCheck::Envelope, 0)),
                        "{scalar}, a {} on a {carrier_name} carrier, {which}: {:?}",
                        if along_u { "row" } else { "column" },
                        got.as_ref().err()
                    );
                }
                // The same images held to the domain certify.
                let image = if along_u {
                    Pcurve::iso_row(T::zero(), whole, whole)
                } else {
                    Pcurve::iso_column(T::zero(), whole, whole)
                };
                let on =
                    PcurveCache::certify(image, whole.0, whole.1, &carrier, &unit_chart(), band());
                assert_eq!(
                    on.as_ref().err().map(|e| format!("{e:?}")),
                    None,
                    "{scalar}, a {} on a {carrier_name} carrier, on its domain",
                    if along_u { "row" } else { "column" }
                );
            }
        }
    }
    at::<f64>("f64");
    at::<Interval>("Interval");
}

/// The row `v = 0` minted from `u = from` to `u = to` over the edge
/// `u = t ∈ [0, stop]` of the unit chart (whose metric is the identity,
/// so a chart gap is that many metres): its certificate's envelope, or
/// the refusal.
fn envelope_of<T: Decide + Bounds>(
    (from, to): (f64, f64),
    stop: f64,
) -> Result<f64, PcurveCertifyError> {
    let f = T::from_f64;
    let span = (T::zero(), f(stop));
    PcurveCache::certify(
        Pcurve::iso_row(T::zero(), (f(from), f(to)), span),
        span.0,
        span.1,
        &row_carrier(),
        &unit_chart(),
        band(),
    )
    .map(|cache| cache.certificate().envelope.hi())
}

/// **A map that departs its carrier inside the band is in the envelope.**
/// Over an edge that stops at `u = 1 − δ`, δ = ε/2, the row minted to the
/// corner runs `0 → 1` and departs the carrier by up to δ: check 4's
/// envelope says so, so a reader that pads by the envelope covers the
/// region the map reaches past the edge.
///
/// Red under the line limb's hull dropped from the envelope (the seam
/// envelope's fold of a line carrier's reach).
#[test]
fn a_map_past_its_carrier_in_band_is_in_the_envelope() {
    fn at<T: Decide + Bounds>(scalar: &str) {
        let delta = 0.5 * crate::shared::tol::eps();
        let envelope = envelope_of::<T>((0.0, 1.0), 1.0 - delta)
            .unwrap_or_else(|e| panic!("{scalar}: the row certifies in band: {e:?}"));
        assert!(
            envelope >= 0.99 * delta,
            "{scalar}: the envelope {envelope:e} covers the map's {delta:e} past the carrier"
        );
    }
    at::<f64>("f64");
    at::<Interval>("Interval");
}

/// **A seam column moved one period is its domain's other end, exactly**,
/// at `f64` and at `Interval`, on domains that do not start at 0: the
/// column held at `lo`, lifted one period stated by the domain's two
/// ends, is held at `hi` itself, and lifted back is `lo`. The sum
/// `lo + (hi − lo)` misses `hi` at `f64` on three of these four domains
/// and has width at `Interval` on all of them. A column an ulp off an end
/// is laid off from the other end by that ulp (`hi + (x − lo)`: at `f64`
/// an offset under `hi`'s own rounding rounds onto `hi`, at `Interval`
/// it is an enclosure and no point); a row's moving coordinate moves by
/// the period's length.
///
/// Red under [`Period::moved`] adding `k·(hi − lo)` to every value.
#[test]
fn a_period_moved_seam_column_is_its_other_end() {
    fn at<T: Decide + Bounds>(scalar: &str) {
        let f = T::from_f64;
        for (lo, hi) in [(0.3, 1.7), (0.4, 1.7), (0.6, 1.7), (0.7, 3.1)] {
            let period = Period::of_domain(f(lo), f(hi));
            let held = |image: &Pcurve<T>| image.held_line().expect("a column").line.value;
            let seam = Pcurve::iso_column(f(lo), (T::zero(), T::one()), (T::zero(), T::one()));
            let lifted = seam.shift_branch(1, period);
            assert_eq!(
                point(held(&lifted)),
                Some(hi),
                "{scalar} [{lo}, {hi}]: the seam lifted one period is the other end"
            );
            assert_eq!(
                point(held(&lifted.shift_branch(-1, period))),
                Some(lo),
                "{scalar} [{lo}, {hi}]: and lifted back is the end it started at"
            );
            // An ulp inside the domain is no end: it is laid off from
            // the other end by its own offset.
            let inside = f64::from_bits(lo.to_bits() + 1);
            let near = Pcurve::iso_column(f(inside), (T::zero(), T::one()), (T::zero(), T::one()));
            let moved = held(&near.shift_branch(1, period));
            let want = f(hi) + (f(inside) - f(lo));
            assert_eq!(
                (moved.lo(), moved.hi()),
                (want.lo(), want.hi()),
                "{scalar} [{lo}, {hi}]: a column an ulp off the end is laid off from the \
                 other end by that ulp"
            );
            // A row's line is not the channel moved: its map's origin
            // takes the period as a length.
            let row = Pcurve::iso_row(f(0.5), (f(lo), f(hi)), (T::zero(), T::one()));
            let lifted = row.shift_branch(1, period);
            assert_eq!(
                point(held(&lifted)),
                Some(0.5),
                "{scalar}: the row keeps its line"
            );
            let (start, want) = (lifted.eval(T::zero()).x, f(lo) + (f(hi) - f(lo)));
            assert_eq!(
                (start.lo(), start.hi()),
                (want.lo(), want.hi()),
                "{scalar} [{lo}, {hi}]: a row's start moves by the period's length"
            );
        }
    }
    at::<f64>("f64");
    at::<Interval>("Interval");
    // The sum the stated ends replace, at `f64`: on these domains it is
    // not the other end.
    for (lo, hi) in [(0.4_f64, 1.7_f64), (0.6, 1.7), (0.7, 3.1)] {
        assert_ne!(
            lo + (hi - lo),
            hi,
            "[{lo}, {hi}]: the computed sum misses hi"
        );
    }
}
