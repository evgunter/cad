//! The general-circle arc as a RUNG-3 carrier, for the rows about the
//! fitted door's SSI certificate.
//!
//! An exact `Curve3::Circle` on a sphere certifies against its chart in
//! closed form and runs no SSI certificate, so a row whose subject is
//! that certificate — its limbs, its escalation payloads, its tube —
//! needs a fitted carrier. This one is the circle's locus-exact
//! rational-quadratic chain (Book §7.3: ≤ 90° segments, middle weight
//! `cos(θ/2)`, middle point the tangent intersection), knotted on the
//! arc's own angular span and built at `T`, with its chart image
//! interpolated on the CHAIN's parameter (the OQ4 identity).

use geom::{Curve3, NurbsCurve2, NurbsCurve3};
use geom_core::spline::KnotVector;
use geom_core::{Point2, Real};

/// The chain of the `circle` arc over `[f0, f1]` (radians), at `T`.
pub fn chain<T: Real>(circle: &Curve3<T>, f0: f64, f1: f64) -> NurbsCurve3<T> {
    let &Curve3::Circle {
        center,
        axis,
        radius,
        u_ref,
    } = circle
    else {
        panic!("the arc chain is a circle's");
    };
    let span = f1 - f0;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let n = (span / core::f64::consts::FRAC_PI_2).ceil().max(1.0) as usize;
    let cv = axis.cross(u_ref);
    let at = |t: f64| {
        let (s, c) = T::from_f64(t).sin_cos();
        center + (u_ref * c + cv * s) * radius
    };
    #[allow(clippy::cast_precision_loss)]
    let seg = span / n as f64;
    let w_mid = (seg / 2.0).cos();
    let mut control = vec![at(f0)];
    let mut weights = vec![1.0];
    let mut knots = vec![f0, f0, f0];
    for i in 0..n {
        #[allow(clippy::cast_precision_loss)]
        let a = f0 + seg * i as f64;
        #[allow(clippy::cast_precision_loss)]
        let b = if i + 1 == n {
            f1
        } else {
            f0 + seg * (i + 1) as f64
        };
        let (s, c) = T::from_f64(0.5 * (a + b)).sin_cos();
        control.push(center + (u_ref * c + cv * s) * (radius / T::from_f64(w_mid)));
        weights.push(w_mid);
        control.push(at(b));
        weights.push(1.0);
        knots.extend(if i + 1 == n {
            vec![b, b, b]
        } else {
            vec![b, b]
        });
    }
    let kv = KnotVector::clamped(knots, 2).expect("the chain's knots");
    NurbsCurve3::new(kv, control, weights).expect("the arc's chain")
}

/// The chain's chart image on a sphere of `radius` centred at the
/// origin with polar axis `+z` and seam `+x`, interpolated at `f64` on
/// the chain's own parameter over `[f0, f1]`, azimuth continued along
/// the arc.
pub fn image(chain: &NurbsCurve3<f64>, radius: f64, f0: f64, f1: f64) -> NurbsCurve2<f64> {
    let n = 65usize;
    let mut params = Vec::with_capacity(n);
    let mut pts = Vec::with_capacity(n);
    let mut prev_u: Option<f64> = None;
    for i in 0..n {
        #[allow(clippy::cast_precision_loss)]
        let s = i as f64 / (n - 1) as f64;
        let p = chain.eval(f0 + (f1 - f0) * s);
        let raw = p.y.atan2(p.x);
        let u = prev_u.map_or(raw, |pu| {
            use core::f64::consts::{PI, TAU};
            pu + ((raw - pu + PI).rem_euclid(TAU) - PI)
        });
        prev_u = Some(u);
        params.push(s);
        pts.push(Point2::new(u, (p.z / radius).asin()));
    }
    let fit = NurbsCurve2::interpolate_with_params(&pts, 3, &params).expect("the chart image fits");
    let knots: Vec<f64> = fit
        .knots()
        .knots()
        .iter()
        .map(|k| f0 + (f1 - f0) * k)
        .collect();
    let kv = KnotVector::clamped(knots, fit.knots().degree()).expect("affine knot rescale");
    NurbsCurve2::new(kv, fit.control().to_vec(), fit.weights().to_vec()).expect("rescaled image")
}
