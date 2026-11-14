//! **An iso line's stated ends** ([`Pcurve::iso_row`],
//! [`Pcurve::span_ends`]): a mint that places a boundary row by its two
//! chart ends stores them, and they read back as placed at every scalar,
//! where the map `p0 + pl·t` read at a computed span end only encloses
//! them. What the reader trusts is pinned here from the other side too:
//! an end placed short of the chart's edge reads short, a split keeps a
//! stated end only on the child that keeps it, and a stated end the
//! carrier does not reach is refused at certification.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::tol::band;
use geom::{Curve3, NurbsSurface, Surface};
use geom_brep::{Pcurve, PcurveCache, PcurveCertifyError, PcurveCheck, StoredSpan};
use geom_core::spline::KnotVector;
use geom_core::{Bounds, Decide, Interval, Point2, Point3, Real, Vec3};
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

/// **A row placed by its ends reads them as placed**, at `f64` and at
/// `Interval`, over a span whose end is a computed length. The map read
/// at that end encloses `1` without being it at `Interval`, which is the
/// refusal the stated ends retire.
#[test]
fn a_row_placed_by_its_ends_reads_them_as_placed() {
    fn at<T: Decide + Bounds>(scalar: &str) {
        let span = (T::zero(), computed_length::<T>());
        let row = Pcurve::iso_row(T::one(), (T::zero(), T::one()), span);
        let [a, b] = row.span_ends(StoredSpan::for_tests(span.0, span.1));
        let read = [a.x, a.y, b.x, b.y].map(point);
        assert_eq!(
            read,
            [Some(0.0), Some(1.0), Some(1.0), Some(1.0)],
            "{scalar}: the row's ends read as placed: {a:?} {b:?}"
        );
        let mapped = row.eval(span.1).x;
        assert!(
            mapped.lo() <= 1.0 && 1.0 <= mapped.hi(),
            "{scalar}: the map at the span's end encloses the end: {mapped:?}"
        );
    }
    at::<f64>("f64");
    at::<Interval>("Interval");
    let span = (Interval::zero(), computed_length::<Interval>());
    let mapped = Pcurve::iso_row(Interval::one(), (Interval::zero(), Interval::one()), span)
        .eval(span.1)
        .x;
    assert_eq!(
        point(mapped),
        None,
        "at Interval the map at a computed end is no point: {mapped:?}"
    );
}

/// **An end placed short of the chart's edge reads short**, by any
/// amount down to an ulp: one ulp below `1` reads as that, never as `1`,
/// at both scalars, and an image with no stated ends reads its map.
#[test]
fn an_end_placed_short_of_the_charts_edge_reads_short() {
    fn at<T: Decide + Bounds>(scalar: &str) {
        let short = 1.0f64.next_down();
        let span = (T::zero(), computed_length::<T>());
        let row = Pcurve::iso_row(T::zero(), (T::zero(), T::from_f64(short)), span);
        let [_, b] = row.span_ends(StoredSpan::for_tests(span.0, span.1));
        assert_eq!(
            point(b.x).map(f64::to_bits),
            Some(short.to_bits()),
            "{scalar}: the end one ulp short reads one ulp short: {b:?}"
        );
        let Pcurve::IsoLine { p0, pl, .. } = row else {
            unreachable!("iso_row mints an IsoLine")
        };
        let unstated = Pcurve::IsoLine { p0, pl, ends: None };
        let [_, mapped] = unstated.span_ends(StoredSpan::for_tests(span.0, span.1));
        let want = unstated.eval(span.1);
        assert_eq!(
            [mapped.x.lo(), mapped.x.hi()],
            [want.x.lo(), want.x.hi()],
            "{scalar}: an image with no stated ends reads its map"
        );
    }
    at::<f64>("f64");
    at::<Interval>("Interval");
}

/// **A split keeps a stated end only on the child that keeps it**: the
/// first child keeps the start and the second the end, and the cut is the
/// map's value there on both, which is neither chart end.
#[test]
fn a_split_keeps_a_stated_end_only_on_the_child_that_keeps_it() {
    let span = (0.0, 2.2);
    let row = Pcurve::iso_row(1.0, (0.0, 1.0), span);
    let cut = 0.8;
    let (first, second) = row.split_at(cut);
    let [fa, fb] = first.span_ends(StoredSpan::for_tests(span.0, cut));
    let [sa, sb] = second.span_ends(StoredSpan::for_tests(cut, span.1));
    let mapped = row.eval(cut);
    assert_eq!((fa.x, fa.y), (0.0, 1.0), "the first child keeps the start");
    assert_eq!((sb.x, sb.y), (1.0, 1.0), "the second child keeps the end");
    for (which, end) in [("first's end", fb), ("second's start", sa)] {
        assert_eq!(
            (end.x, end.y),
            (mapped.x, mapped.y),
            "{which}: the cut is the map's value"
        );
        assert!(
            end.x > 0.0 && end.x < 1.0,
            "{which}: the cut is no chart end: {end:?}"
        );
    }
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

/// **A stated end the carrier does not reach is refused**: the row
/// `v = 0` over an edge that stops at `u = 0.9`, its map the edge's own,
/// certifies with its ends read through the map or placed where the edge
/// ends; stating the chart's corner `u = 1` as its end, the edge does not
/// reach it, and check 3 refuses at the span's last station.
#[test]
fn a_stated_end_the_carrier_does_not_reach_is_refused() {
    fn at<T: Decide>(scalar: &str) {
        let f = T::from_f64;
        let carrier = Curve3::line(
            Point3::new(T::zero(), T::zero(), T::zero()),
            Vec3::new(T::one(), T::zero(), T::zero()),
        );
        let span = (T::zero(), f(0.9));
        let certify = |image: Pcurve<T>| {
            PcurveCache::certify(image, span.0, span.1, &carrier, &unit_chart(), band())
        };
        let placed = Pcurve::iso_row(T::zero(), (T::zero(), f(0.9)), span);
        assert!(
            certify(placed.clone()).is_ok(),
            "{scalar}: the row placed where its edge ends certifies"
        );
        let Pcurve::IsoLine { p0, pl, .. } = placed else {
            unreachable!("iso_row mints an IsoLine")
        };
        assert!(
            certify(Pcurve::IsoLine { p0, pl, ends: None }).is_ok(),
            "{scalar}: the same map with no stated ends certifies"
        );
        let corner = [
            Point2::new(T::zero(), T::zero()),
            Point2::new(T::one(), T::zero()),
        ];
        let got = certify(Pcurve::IsoLine {
            p0,
            pl,
            ends: Some(corner),
        });
        assert!(
            matches!(
                got,
                Err(PcurveCertifyError::ResidualExceeded {
                    check: PcurveCheck::MapResidual,
                    sample: 8,
                    ..
                })
            ),
            "{scalar}: a stated end past where its edge ends is refused at the last station: \
             {got:?}"
        );
    }
    at::<f64>("f64");
    at::<Interval>("Interval");
}

/// The row `v = 0` from `u = 0` to `u = 1 − δ`, δ = ε/2, as `image`
/// states it, certified on the unit chart (whose metric is the identity,
/// so a chart gap is that many metres): its certificate's envelope.
fn envelope_over_a_short_edge<T: Decide + Bounds>(
    image: impl Fn((T, T)) -> Pcurve<T>,
    scalar: &str,
) -> (f64, f64) {
    let delta = 0.5 * crate::shared::tol::eps();
    let carrier = Curve3::line(
        Point3::new(T::zero(), T::zero(), T::zero()),
        Vec3::new(T::one(), T::zero(), T::zero()),
    );
    let span = (T::zero(), T::from_f64(1.0 - delta));
    let cache = PcurveCache::certify(image(span), span.0, span.1, &carrier, &unit_chart(), band())
        .unwrap_or_else(|e| panic!("{scalar}: the row certifies in band: {e:?}"));
    (cache.certificate().envelope.hi(), delta)
}

/// **A stated end off its own map is in the envelope.** The map is the
/// edge's own, stopping at `u = 1 − δ`; the stated ends are the chart's
/// corners. Check 3 reads the stated end, in band, and passes; the
/// rectangle lane reads `u = 1` and pads by `perimeter·envelope`, so the
/// envelope must cover the stated end's δ from the map, or the pad does
/// not cover the region the lane reads past the edge.
#[test]
fn a_stated_end_off_its_map_is_in_the_envelope() {
    fn at<T: Decide + Bounds>(scalar: &str) {
        let (envelope, delta) = envelope_over_a_short_edge::<T>(
            |span| {
                let Pcurve::IsoLine { p0, pl, .. } =
                    Pcurve::iso_row(T::zero(), (T::zero(), span.1), span)
                else {
                    unreachable!("iso_row mints an IsoLine")
                };
                Pcurve::IsoLine {
                    p0,
                    pl,
                    ends: Some([
                        Point2::new(T::zero(), T::zero()),
                        Point2::new(T::one(), T::zero()),
                    ]),
                }
            },
            scalar,
        );
        assert!(
            envelope >= 0.99 * delta,
            "{scalar}: the envelope {envelope:e} covers the stated end's {delta:e} off its map"
        );
    }
    at::<f64>("f64");
    at::<Interval>("Interval");
}

/// **A corner the mint states over a short edge is in the envelope.**
/// `iso_row` derives the map from the stated corners, so over an edge
/// that stops at `u = 1 − δ` the map runs `0 → 1` and departs the carrier
/// by up to δ: check 4's envelope of the map says so.
#[test]
fn a_corner_the_mint_states_over_a_short_edge_is_in_the_envelope() {
    fn at<T: Decide + Bounds>(scalar: &str) {
        let (envelope, delta) = envelope_over_a_short_edge::<T>(
            |span| Pcurve::iso_row(T::zero(), (T::zero(), T::one()), span),
            scalar,
        );
        assert!(
            envelope >= 0.99 * delta,
            "{scalar}: the envelope {envelope:e} covers the map's {delta:e} past the carrier"
        );
    }
    at::<f64>("f64");
    at::<Interval>("Interval");
}
