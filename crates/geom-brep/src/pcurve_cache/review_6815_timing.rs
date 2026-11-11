//! Review timing probe for PR 6815 LL4 (base vs head): per-call cost of
//! `EdgeCurve::certify` for knot-rich nets on the ring's column.
#![allow(clippy::float_cmp)]

use std::sync::Arc;

use geom::{Curve3, NurbsCurve2, NurbsCurve3, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Decide, Interval, Point2, Point3};

use super::Pcurve;
use super::spline_door_rows::{band, in_band, ring};
use crate::{EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec, NurbsLane, SurfaceKey};

fn net(p: usize, k: usize, d: f64) -> NurbsCurve3<f64> {
    let mut knots = vec![0.0; p + 1];
    // Irregular interior knots, pinned.
    for j in 1..=k {
        let x = (j as f64 + 0.37 * ((j * 7 % 5) as f64 - 2.0) / 5.0) / (k as f64 + 1.0);
        knots.push(x);
    }
    knots.extend(vec![1.0; p + 1]);
    let n = knots.len() - p - 1;
    let a: f64 = 0.6;
    let r = 1.0 + d;
    let control = (0..n)
        .map(|i| {
            let xi = knots[i + 1..=i + p].iter().sum::<f64>() / p as f64;
            Point3::new(r * a.cos(), r * a.sin(), xi)
        })
        .collect();
    NurbsCurve3::new(KnotVector::clamped(knots, p).unwrap(), control, vec![1.0; n]).unwrap()
}

fn time_at<T: Decide + geom_core::CertifiedBounds>() {
    let b = band();
    for (what, d) in [("on", 0.0), ("zero", b.zero()), ("half-band", in_band()), ("0.999esc", 0.999 * b.escalate())] {
        for (p, k) in [(3usize, 11usize), (1, 3)] {
            let carrier = Curve3::Nurbs(Arc::new(net(p, k, d).map_scalar(T::from_f64)));
            let image = NurbsCurve2::new(
                KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
                vec![Point2::new(0.5, 0.0), Point2::new(0.5, 1.0)],
                vec![1.0, 1.0],
            )
            .unwrap();
            let mut table: slotmap::SlotMap<SurfaceKey, Surface<T>> = slotmap::SlotMap::with_key();
            let key = table.insert(ring::<T>());
            let spec = EdgeCurveSpec {
                description: EdgeDescriptionSpec::chart_image(
                    key,
                    Pcurve::General(Arc::new(image.map_scalar(T::from_f64))),
                ),
                carrier: carrier.clone(),
                param_start: T::zero(),
                param_end: T::one(),
            };
            let (s, e) = (carrier.eval(T::zero()), carrier.eval(T::one()));
            let reps = if d == b.zero() { 1 } else { 5 };
            let t = std::time::Instant::now();
            let mut ok = false;
            for _ in 0..reps {
                ok = EdgeCurve::certify(
                    spec.clone(),
                    s,
                    e,
                    |k| table.get(k).cloned(),
                    geom_core::test_support::fixture_lever(T::one()),
                    b,
                    NurbsLane::certified(),
                )
                .is_ok();
            }
            eprintln!(
                "TIMING {} {what} p={p} k={k}: {:.3} ms/call ok={ok}",
                T::NAME,
                t.elapsed().as_secs_f64() * 1e3 / reps as f64
            );
        }
    }
}

#[test]
fn review_6815_timing() {
    time_at::<f64>();
    time_at::<Interval>();
}
