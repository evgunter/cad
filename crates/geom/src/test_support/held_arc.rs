//! **A rational arc whose whole definition is held in the scalar**: a
//! circular arc of radius `r` and sweep `θ` about the origin, split
//! into `m` equal sub-arcs as the skin converts a profile arc (Book
//! §7.5 / A7.1), with every knot, weight and control coordinate built
//! in `T`. At `Sym<f64>` over a variable `θ`, the interior weights
//! `cos(Δ/2)` (`Δ = θ/m`) are functions of `θ`, which is what a
//! carrier holding its weights in `T` has to keep and one holding them
//! in `f64` freezes.

use core::num::NonZeroUsize;

use geom_core::spline::{Knots, SpanLocate, SplineError};
use geom_core::{Point2, Real};

/// The arc's data: degree 2, `2m + 1` control points, one double knot
/// per interior sub-arc boundary.
#[derive(Clone, Debug)]
pub struct HeldArc<T: Real> {
    /// The breakpoints `0` (triple), `k/m` (double) and `1` (triple).
    pub knots: Knots<T>,
    /// On-arc points at the sub-arc ends; tangent-intersection points
    /// between them.
    pub control: Vec<Point2<T>>,
    /// `1` at the on-arc points, `cos(Δ/2)` between them.
    pub weights: Vec<T>,
}

impl<T: SpanLocate> HeldArc<T> {
    /// The arc of radius `r` from angle `0` to `θ`, in `m` equal
    /// sub-arcs.
    ///
    /// # Errors
    ///
    /// [`Knots::new`]'s: a `θ` too small for `k/m` to increase, or not
    /// a point.
    pub fn new(r: T, theta: T, m: NonZeroUsize) -> Result<Self, SplineError> {
        let m = m.get();
        #[allow(clippy::cast_precision_loss)]
        let steps = T::from_f64(m as f64);
        let delta = theta / steps;
        let (_, half_w) = (delta * T::from_f64(0.5)).sin_cos();
        let at = |angle: T, radius: T| {
            let (s, c) = angle.sin_cos();
            Point2::new(radius * c, radius * s)
        };
        let mut control = vec![at(T::zero(), r)];
        let mut weights = vec![T::one()];
        for k in 0..m {
            #[allow(clippy::cast_precision_loss)]
            let base = delta * T::from_f64(k as f64);
            control.push(at(base + delta * T::from_f64(0.5), r / half_w));
            weights.push(half_w);
            control.push(at(base + delta, r));
            weights.push(T::one());
        }
        let (double, triple) = (
            NonZeroUsize::MIN.saturating_add(1),
            NonZeroUsize::MIN.saturating_add(2),
        );
        let mut runs = vec![(T::zero(), triple)];
        for k in 1..m {
            #[allow(clippy::cast_precision_loss)]
            runs.push((T::from_f64(k as f64) / steps, double));
        }
        runs.push((T::one(), triple));
        Ok(Self {
            knots: Knots::new(runs, 2)?,
            control,
            weights,
        })
    }

    /// The arc's point at `t ∈ [0, 1]`: `Σ Nᵢwᵢ Pᵢ / Σ Nᵢwᵢ` over the
    /// span `t` locates in, every factor read as held. `None` for a
    /// poison `t`, or an enclosure that straddles two spans.
    pub fn eval(&self, t: T) -> Option<Point2<T>> {
        let (span, last) = self.knots.locate(t)?;
        if span.span().index() != last.span().index() {
            return None;
        }
        let base = span.span().first_control();
        let row = geom_core::spline::basis::basis_funs_in(span, t);
        let (mut x, mut y, mut w) = (T::zero(), T::zero(), T::zero());
        for (j, n) in row.iter().enumerate() {
            let nw = *n * self.weights[base + j];
            let p = self.control[base + j];
            x = x + nw * p.x;
            y = y + nw * p.y;
            w = w + nw;
        }
        Some(Point2::new(x / w, y / w))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::sym::revalue::revalue;
    use geom_core::sym::with_session;
    use geom_core::{ParamSymbol, Sym, SymBudget};

    type S = Sym<f64>;
    const THETA: u64 = 0x7e7a;

    fn budget() -> SymBudget {
        SymBudget {
            max_terms: 4096,
            max_degree: 128,
        }
    }

    #[test]
    fn the_arc_lies_on_its_circle_at_f64() {
        let arc = HeldArc::new(2.0, 2.5, NonZeroUsize::new(3).unwrap()).unwrap();
        for t in [0.0, 0.1, 0.33, 0.5, 0.9, 1.0] {
            let p = arc.eval(t).unwrap();
            let rr = (p.x * p.x + p.y * p.y).sqrt();
            assert!((rr - 2.0).abs() < 1e-14, "|P({t})| = {rr}");
        }
        let end = arc.eval(1.0).unwrap();
        assert!(
            (end.x - 2.0 * 2.5f64.cos()).abs() < 1e-14,
            "the arc ends at θ"
        );
    }

    /// Built at `θ₀` over a variable `θ` and re-valued at `θ₁`, a point
    /// of the arc is bit for bit the point the `θ₁` build computes: the
    /// weights and control are functions of `θ`. The same arc with its
    /// weights read out to `f64` and lifted back (a stored-`f64`
    /// weight) re-values to a point off the `θ₁` arc.
    #[test]
    fn a_point_of_the_held_arc_re_values_with_its_angle() {
        let (t0, t1) = (1.9, 2.6);
        let ((held, frozen), _) = with_session(budget(), || {
            let theta = S::param(ParamSymbol::new(THETA), t0);
            let arc =
                HeldArc::new(S::from_f64(1.5), theta, NonZeroUsize::MIN.saturating_add(1)).unwrap();
            let t = S::from_f64(0.3);
            let held = arc.eval(t).unwrap();
            let frozen = HeldArc {
                weights: arc.weights.iter().map(|w| S::from_f64(w.value)).collect(),
                ..arc.clone()
            }
            .eval(t)
            .unwrap();
            let env = |s: ParamSymbol| (s == ParamSymbol::new(THETA)).then_some(t1);
            let re = |p: Point2<S>| (revalue(p.x.node(), &env), revalue(p.y.node(), &env));
            (re(held), re(frozen))
        });
        let want = HeldArc::new(1.5, t1, NonZeroUsize::MIN.saturating_add(1))
            .unwrap()
            .eval(0.3)
            .unwrap();
        let bits = |(x, y): (Option<f64>, Option<f64>)| (x.map(f64::to_bits), y.map(f64::to_bits));
        assert_eq!(
            bits(held),
            (Some(want.x.to_bits()), Some(want.y.to_bits())),
            "the held arc re-valued at θ₁ is the θ₁ arc"
        );
        let (fx, fy) = frozen;
        let off = (fx.unwrap() - want.x).hypot(fy.unwrap() - want.y);
        assert!(
            off > 1e-6,
            "a weight frozen at θ₀ moved with θ: off the θ₁ arc by {off}"
        );
    }

    /// **The pin through the carrier**: the held arc's knots, control
    /// and weights handed to a `NurbsCurve2` at `Sym`, whose point
    /// re-values with `θ` to the point a `θ₁` carrier computes, bit for
    /// bit. The same carrier with its weights entered as `f64`
    /// constants (`from_f64` of the value channel: the stored weight
    /// before `nurbs-weights-and-knots-are-the-scalar`) re-values off
    /// the `θ₁` arc.
    #[test]
    fn a_point_of_a_carrier_holding_the_arc_re_values_with_its_angle() {
        use crate::NurbsCurve2;
        let (t0, t1) = (1.9, 2.6);
        let m = NonZeroUsize::new(3).unwrap();
        let ((held, frozen), _) = with_session(budget(), || {
            let theta = S::param(ParamSymbol::new(THETA), t0);
            let arc = HeldArc::new(S::from_f64(1.5), theta, m).unwrap();
            let carrier =
                NurbsCurve2::new(arc.knots.clone(), arc.control.clone(), arc.weights.clone())
                    .unwrap();
            let frozen = NurbsCurve2::new(
                arc.knots.clone(),
                arc.control.clone(),
                carrier
                    .weights_f64_interim()
                    .iter()
                    .map(|w| S::from_f64(*w))
                    .collect(),
            )
            .unwrap();
            let t = S::from_f64(0.45);
            let env = |s: ParamSymbol| (s == ParamSymbol::new(THETA)).then_some(t1);
            let re = |p: Point2<S>| (revalue(p.x.node(), &env), revalue(p.y.node(), &env));
            (re(carrier.eval(t)), re(frozen.eval(t)))
        });
        let arc = HeldArc::new(1.5, t1, m).unwrap();
        let want = NurbsCurve2::new(arc.knots, arc.control, arc.weights)
            .unwrap()
            .eval(0.45);
        let bits = |(x, y): (Option<f64>, Option<f64>)| (x.map(f64::to_bits), y.map(f64::to_bits));
        assert_eq!(
            bits(held),
            (Some(want.x.to_bits()), Some(want.y.to_bits())),
            "the carrier re-valued at θ₁ is the θ₁ carrier"
        );
        let (fx, fy) = frozen;
        let off = (fx.unwrap() - want.x).hypot(fy.unwrap() - want.y);
        assert!(
            off > 1e-6,
            "a carrier's weight frozen at θ₀ moved with θ: off by {off}"
        );
    }
}
