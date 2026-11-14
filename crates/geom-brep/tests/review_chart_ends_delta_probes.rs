//! Reviewer probes, delta pass on PR 6959: does check 4's stated-end
//! term cover every way a hand-stated end can sit off its map? Each row
//! states ends off the map (the map is the carrier's own), certifies, and
//! compares the envelope with the true metric gap `|S(end) − C(t_end)|`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::tol::{band, eps};
use geom::{Curve3, NurbsSurface, Surface};
use geom_brep::{Pcurve, PcurveCache};
use geom_core::spline::KnotVector;
use geom_core::{Bounds, Decide, Interval, Point2, Point3, Real, Vec3};
use std::sync::Arc;

/// `(u, v) ↦ (u, k·v, 0)` over `[0, 1]²`.
fn chart<T: Real>(k: f64) -> Surface<T> {
    let kv = || KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control = [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)]
        .map(|(x, y)| Point3::new(x, k * y, 0.0))
        .to_vec();
    let net = NurbsSurface::new(kv(), kv(), control, vec![1.0; 4]).unwrap();
    Surface::Nurbs(Arc::new(net.map_scalar(T::from_f64)))
}

/// The row `v = 0` over the edge `u = t ∈ [a, b]` (map = carrier), its
/// stated ends `ends` (as f64 chart points), certified on `chart(k)`.
/// Returns `Ok((envelope.hi, gap))`, `gap` the true metric distance of a
/// stated end from the carrier's end, or the refusal's text.
fn run<T: Decide + Bounds>(
    k: f64,
    (a, b): (f64, f64),
    ends: [(f64, f64); 2],
) -> Result<(f64, f64), String> {
    let f = T::from_f64;
    let carrier = Curve3::line(
        Point3::new(T::zero(), T::zero(), T::zero()),
        Vec3::new(T::one(), T::zero(), T::zero()),
    );
    let span = (f(a), f(b));
    let Pcurve::IsoLine { p0, pl, .. } = Pcurve::iso_row(T::zero(), (f(a), f(b)), span) else {
        unreachable!()
    };
    let image = Pcurve::IsoLine {
        p0,
        pl,
        ends: Some(ends.map(|(x, y)| Point2::new(f(x), f(y)))),
    };
    let surface = chart::<T>(k);
    let cache = PcurveCache::certify(image, span.0, span.1, &carrier, &surface, band())
        .map_err(|e| format!("{e:?}"))?;
    let s = chart::<f64>(k);
    let gap = ends
        .iter()
        .zip([a, b])
        .map(|(&(x, y), t)| s.eval(x, y).distance(Point3::new(t, 0.0, 0.0)))
        .fold(0.0, f64::max);
    Ok((cache.certificate().envelope.hi(), gap))
}

fn case(name: &str, k: f64, span: (f64, f64), ends: [(f64, f64); 2]) {
    for (scalar, got) in [
        ("f64", run::<f64>(k, span, ends)),
        ("Interval", run::<Interval>(k, span, ends)),
    ] {
        match got {
            Ok((envelope, gap)) => {
                eprintln!(
                    "[review] {name} {scalar}: certified, envelope {envelope:e}, true gap {gap:e}"
                );
                assert!(
                    envelope >= gap * (1.0 - 1e-9),
                    "{name} {scalar}: envelope {envelope:e} < true gap {gap:e}"
                );
            }
            Err(e) => eprintln!(
                "[review] {name} {scalar}: refused {}",
                &e[..e.len().min(160)]
            ),
        }
    }
}

#[test]
fn review_delta_stated_ends_off_the_map_are_padded() {
    let d = 0.5 * eps();
    case(
        "along the row",
        1.0,
        (0.0, 1.0 - d),
        [(0.0, 0.0), (1.0, 0.0)],
    );
    case(
        "across, the fixed coordinate",
        1.0,
        (0.0, 0.9),
        [(0.0, d), (0.9, d)],
    );
    case(
        "opposite senses",
        1.0,
        (0.1, 0.9),
        [(0.1 - d, 0.0), (0.9 + d, 0.0)],
    );
    case(
        "one end at the band",
        1.0,
        (0.0, 0.9),
        [(0.0, 0.0), (0.9 + eps(), 0.0)],
    );
    case(
        "twice the band",
        1.0,
        (0.0, 0.9),
        [(0.0, 0.0), (0.9 + 2.0 * eps(), 0.0)],
    );
    case(
        "anisotropic, across (k = 100)",
        100.0,
        (0.0, 0.9),
        [(0.0, d / 100.0), (0.9, d / 100.0)],
    );
    case(
        "anisotropic, along (k = 1e-3)",
        1e-3,
        (0.0, 0.9),
        [(0.0, 0.0), (0.9 + d, 0.0)],
    );
    case(
        "past the chart's domain",
        1.0,
        (0.0, 1.0),
        [(0.0, 0.0), (1.0 + d, 0.0)],
    );
    case(
        "before the chart's domain",
        1.0,
        (0.0, 0.9),
        [(-d, 0.0), (0.9, 0.0)],
    );
}
