//! **A loft whose rims are split and whose rows are minted again holds
//! its volume.** The rectangle certificate reads the chart lines a wall's
//! rows hold and their meets, never an image's end, so a split's cut and
//! a kept corner a re-mint reads through its measured map are never asked
//! to be exact. Before that, the `f64` lofts below refused once re-minted
//! (the re-minted ends missed the certificate's join slack) and every
//! `Interval` split refused at its cut.
//!
//! The lofts are of quadrilaterals `(−1, −1), (1, −1), (h, 1), (−1, 1)`
//! scaled by `s`, stacked `0.7` apart with `h` rising by `0.1` a station:
//! each section's area is `(3 + h)·s²`, linear in the height, so the
//! volume is `(3 + h̄)·s²·L`, `h̄` the mean of the stations' `h` and `L` the
//! loft's length, independent of the body the kernel builds.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Bounds, Decide, Interval, Point2, Point3, Real, Tol};
use profile::test_support::bulge_loop;
use profile::{ProfileLoop, SketchPlane};
use topo::construction::Construct;
use topo::{Body, Pcurve};

fn tol() -> Tol {
    Tol::witness()
}

/// The quadrilateral at `h`, scaled by `s`.
fn quad<T: Real>(h: f64, s: f64) -> Vec<ProfileLoop<T>> {
    let c = |x: f64, y: f64| {
        (
            Point2::new(T::from_f64(x * s), T::from_f64(y * s)),
            T::zero(),
        )
    };
    vec![bulge_loop(vec![
        c(-1.0, -1.0),
        c(1.0, -1.0),
        c(h, 1.0),
        c(-1.0, 1.0),
    ])]
}

/// One loft: its first station's `h`, its scale, its station count and
/// its degree.
type Loft = (f64, f64, usize, usize);

/// The loft's analytic volume.
fn volume((h, s, n, _): Loft) -> f64 {
    #[allow(clippy::cast_precision_loss)]
    let (mean, length) = (h + 0.05 * (n - 1) as f64, 0.7 * (n - 1) as f64);
    (3.0 + mean) * s * s * length
}

/// The loft's body at `T`, in construction state.
fn loft<T: Decide + Bounds + topo::AtRestPolicy>((h, s, n, degree): Loft) -> Body<T> {
    #[allow(clippy::cast_precision_loss)]
    let sections: Vec<_> = (0..n).map(|i| quad::<T>(h + 0.1 * i as f64, s)).collect();
    #[allow(clippy::cast_precision_loss)]
    let places: Vec<_> = (0..n)
        .map(|i| {
            SketchPlane::xy_at(Point3::new(
                T::zero(),
                T::zero(),
                T::from_f64(0.7 * i as f64),
            ))
        })
        .collect();
    sweep::test_support::loft_body_construction::<T>(&sections, &places, degree, tol())
        .expect("the loft builds")
        .body
}

/// Split up to `count` of the body's rim edges at `frac` of their span.
fn split_rims<T: Decide + Bounds + topo::AtRestPolicy>(
    body: &mut Body<T>,
    frac: f64,
    count: usize,
) {
    let rims: Vec<_> = body
        .pcurves()
        .filter(|(_, c)| match c.pcurve() {
            Pcurve::IsoLine { pl, .. } => (pl.y.lo(), pl.y.hi()) == (0.0, 0.0),
            Pcurve::IsoArc { .. } => true,
            _ => false,
        })
        .map(|(he, _)| he)
        .collect();
    let mut split = Vec::new();
    for he in rims {
        if split.len() >= count {
            break;
        }
        let edge = body.get_half_edge(he).unwrap().edge;
        if split.contains(&edge) {
            continue;
        }
        split.push(edge);
        let (t0, t1) = body.pcurve(he).unwrap().params();
        body.split_edge(edge, t0 + (t1 - t0) * T::from_f64(frac), tol())
            .expect("the rim splits");
    }
}

/// `body` finished and measured: its certified volume as `[lo, hi]`.
fn measured<T: Decide + Bounds + geom_core::CertifiedBounds + topo::AtRestPolicy>(
    body: Body<T>,
    what: &str,
) -> (f64, f64) {
    let finished = topo::finish(body, tol()).unwrap_or_else(|e| panic!("{what} finishes: {e:?}"));
    let m = topo::mass_properties(
        &finished,
        tol(),
        topo::test_support::pinned_resolution(tol()),
    )
    .unwrap_or_else(|e| panic!("{what} measures: {e:?}"))
    .enclosure;
    (m.volume.lo() - m.volume_pad, m.volume.hi() + m.volume_pad)
}

/// **At `f64`, five lofts with eight rims cut at a seventh and their rows
/// minted again**, each holding its analytic volume. On main before the
/// rectangle runs each refused once re-minted: "does not traverse its UV
/// rectangle exactly once".
#[test]
fn split_and_reminted_lofts_hold_their_volumes_at_f64() {
    for loft_at in [
        (1.2, 1.0, 3, 2),
        (0.7, 1.0, 3, 2),
        (2.5, 0.3, 4, 3),
        (1.0, 1.0, 2, 1),
        (1.9, 7.0, 3, 2),
    ] {
        let mut body = loft::<f64>(loft_at);
        split_rims(&mut body, 1.0 / 7.0, 8);
        topo::mint_pcurves(&mut body, tol()).expect("the split loft mints again");
        let (lo, hi) = measured(body, &format!("{loft_at:?}, split and minted again"));
        let want = volume(loft_at);
        let slack = 1e-12 * want;
        assert!(
            lo - slack <= want && want <= hi + slack,
            "{loft_at:?}: the certified volume [{lo}, {hi}] holds {want}"
        );
    }
}

/// **At `Interval`, a sample of split lofts**, as split and minted again,
/// each certified around its analytic volume: one rim cut at a third, and
/// three rims at `0.37`. Before the rectangle runs each refused at its cut.
#[test]
fn split_lofts_hold_their_volumes_at_interval() {
    for loft_at in [(1.2, 1.0, 3, 2), (1.0, 1.0, 2, 1)] {
        for (frac, count) in [(1.0 / 3.0, 1), (0.37, 3)] {
            let mut body = loft::<Interval>(loft_at);
            split_rims(&mut body, frac, count);
            let want = volume(loft_at);
            let what = format!("{loft_at:?} split {count} at {frac}");
            let (lo, hi) = measured(body.clone(), &what);
            assert!(
                lo <= want && want <= hi,
                "{what}: [{lo}, {hi}] holds {want}"
            );
            topo::mint_pcurves(&mut body, tol()).expect("the split loft mints again");
            let (lo, hi) = measured(body, &format!("{what}, minted again"));
            assert!(
                lo <= want && want <= hi && hi - lo < 1e-6,
                "{what}, minted again: [{lo}, {hi}] holds {want}"
            );
        }
    }
}
