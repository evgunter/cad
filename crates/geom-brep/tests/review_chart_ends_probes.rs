//! Reviewer probes for the stated-ends unit (PR 6959): what the
//! certificate says about a stated end that sits in band of its carrier
//! but off the map's own end.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::tol::{band, eps};
use geom::{Curve3, NurbsSurface, Surface};
use geom_brep::{Pcurve, PcurveCache};
use geom_core::spline::KnotVector;
use geom_core::{Bounds, Decide, Point3, Real, Vec3};
use std::sync::Arc;

fn unit_chart<T: Real>() -> Surface<T> {
    let k = || KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control = [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)]
        .map(|(x, y)| Point3::new(x, y, 0.0))
        .to_vec();
    let net = NurbsSurface::new(k(), k(), control, vec![1.0; 4]).unwrap();
    Surface::Nurbs(Arc::new(net.map_scalar(T::from_f64)))
}

/// **The shipped mint's shape of the same gap is padded.** The boundary
/// row schedule states the chart's corners and derives the map FROM them
/// (`iso_row(v, (u0, u1), (t0, t1))`), whatever the carrier does: over an
/// edge that stops at `u = 1 − δ`, δ = ε/2, the map runs `0 → 1` over the
/// span and so departs the carrier by up to δ. Check 4's envelope is the
/// map's, so it must bound that departure for the rectangle lane's pad to
/// cover the region it reads past the edge.
#[test]
fn review_a_corner_stated_by_the_mint_over_a_short_edge_is_in_the_envelope() {
    fn at<T: Decide + Bounds>(scalar: &str) {
        let f = T::from_f64;
        let delta = 0.5 * eps();
        let carrier = Curve3::line(
            Point3::new(T::zero(), T::zero(), T::zero()),
            Vec3::new(T::one(), T::zero(), T::zero()),
        );
        let span = (T::zero(), f(1.0 - delta));
        let row = Pcurve::iso_row(T::zero(), (T::zero(), T::one()), span);
        let got = PcurveCache::certify(row, span.0, span.1, &carrier, &unit_chart(), band());
        match got {
            Ok(cache) => {
                let c = cache.certificate();
                eprintln!(
                    "[review] {scalar}: shipped shape certified; envelope = {:e}, \
                     max_residual = {:e}, delta = {delta:e}",
                    c.envelope.hi(),
                    c.max_residual.hi()
                );
                assert!(
                    c.envelope.hi() >= 0.99 * delta,
                    "{scalar}: the map departs the carrier by δ at its end and the envelope \
                     does not say so: {:e} < {delta:e}",
                    c.envelope.hi()
                );
            }
            Err(e) => eprintln!("[review] {scalar}: shipped shape refused: {e:?}"),
        }
    }
    at::<f64>("f64");
    at::<geom_core::Interval>("Interval");
}
