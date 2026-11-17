//! **The periodic cubic**: a closed C² cubic B-spline with no start.
//!
//! # The data
//!
//! `n` knots `u₀ < … < uₙ₋₁` of one turn, a period `T > uₙ₋₁ − u₀`,
//! and `n` control points `d₀ … dₙ₋₁`, all read cyclically: the knot
//! sequence continues `u_{i+kn} = u_i + k·T` and the control points
//! repeat. `d_i` is the de Boor point over the three knots centred on
//! `u_i`, the blossom at `(u_{i−1}, u_i, u_{i+1})`. Every knot is
//! simple, so the curve is C² at every parameter by its knot structure,
//! and no knot is a start: the data is the same under a cyclic shift.
//!
//! # Reading it
//!
//! The substrate is clamped ([`geom_core::spline`]), so a reader that
//! wants a net asks for **the clamped view over a span**
//! ([`PeriodicNurbs3::clamped_view`]): the curve's restriction to
//! `[a, b]` as a clamped cubic, by knot insertion at the two ends. Its
//! interior knots are the curve's own inside the span, each simple. The
//! knots, control points and weights of the periodic form itself have no
//! accessor: a clamped reader handed them would misread them.
//!
//! Two views are held. **The base opening** is the view over
//! `[u₀, u₀ + T]`: its knots are the stored ones with no arithmetic on
//! them, and its control points come from the cyclic ones by the knot
//! gaps alone, so its first and last control points are one value. A
//! certificate of the whole loop is taken on it. **The two-turn
//! opening** is the view over `[u₀, u₀ + 2T]`, whose second turn's knots
//! are `u_i + T` as the scalar rounds them; every span of at most one
//! turn that starts in the base turn lies in it, and
//! [`PeriodicNurbs3::clamped_view`] cuts each such view out of it
//! ([`NurbsCurve3::split_at`]). Evaluation reads it too, at the
//! parameter less whole periods.
//!
//! A view that crosses `u₀ + T` therefore stands on rounded knots: it is
//! the loop there to the rounding of `u_i + T`, and whoever certifies an
//! edge on such a span certifies it on that span's own view.
//!
//! # A view that starts beside a knot
//!
//! A view starting exactly at a knot starts there with nothing
//! inserted. One starting within a few ulps below a knot keeps that knot
//! as its first interior one, so its first span is that narrow: the
//! insertion is exact, and the narrow span's control points differ by
//! less than their own rounding, which a derivative hull over that span
//! reports as a wide bound. Nothing here snaps the start onto the knot;
//! a reader that cannot bound the narrow span refuses it.

use geom_core::spline::{KnotVector, SpanLocate, SplineError};
use geom_core::{Point3, Real, Vec3};

use super::fit::FitError;
use super::nurbs::NurbsCurve3;

/// The periodic form's degree.
pub const PERIODIC_DEGREE: usize = 3;

/// The fewest knots a turn holds: fewer, and a clamped view's end
/// formulas would read one control point twice.
pub const PERIODIC_MIN_KNOTS: usize = 3;

/// Why periodic data is not a periodic cubic.
#[derive(Clone, Debug, PartialEq)]
pub enum PeriodicError {
    /// The knots and the control points differ in number, or are fewer
    /// than [`PERIODIC_MIN_KNOTS`].
    Count {
        /// Knots given.
        knots: usize,
        /// Control points given.
        control: usize,
    },
    /// A knot, the period or a control coordinate is not finite.
    NonFinite,
    /// The knots do not ascend strictly, or the period does not exceed
    /// their extent.
    Order {
        /// The first knot not above its predecessor; the knot count
        /// where the period is the fault.
        index: usize,
    },
    /// An opening's clamped net refused structure.
    Opening(SplineError),
}

impl core::fmt::Display for PeriodicError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Count { knots, control } => write!(
                f,
                "a periodic cubic needs one control point per knot and at least \
                 {PERIODIC_MIN_KNOTS} of each (got {knots} knots, {control} control points)"
            ),
            Self::NonFinite => write!(f, "a periodic cubic's data is not finite"),
            Self::Order { index } => write!(
                f,
                "a periodic cubic's knots ascend strictly within one period (knot {index})"
            ),
            Self::Opening(e) => write!(f, "a periodic cubic's opening refused: {e}"),
        }
    }
}

impl std::error::Error for PeriodicError {}

/// Why a span of a periodic cubic has no clamped view.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PeriodicViewError {
    /// The span is not forward, an end is not a number, or it does not
    /// lie in the two turns from the base knot.
    Span {
        /// The span asked for, in the carrier's parameter.
        asked: (f64, f64),
        /// The two turns a view is cut from, in the same parameter.
        window: (f64, f64),
    },
    /// The knot insertion at an end refused.
    Insertion {
        /// The span asked for, in the carrier's parameter.
        asked: (f64, f64),
        /// The end whose insertion refused, in the stored data's
        /// parameter.
        at: f64,
    },
}

impl core::fmt::Display for PeriodicViewError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Span { asked, window } => write!(
                f,
                "a periodic cubic has no clamped view over [{}, {}]: a view is a forward span \
                 inside [{}, {}]",
                asked.0, asked.1, window.0, window.1
            ),
            Self::Insertion { asked, at } => write!(
                f,
                "a periodic cubic has no clamped view over [{}, {}]: the knot insertion at {at} \
                 refused",
                asked.0, asked.1
            ),
        }
    }
}

impl std::error::Error for PeriodicViewError {}

/// A periodic cubic (module docs).
#[derive(Clone)]
pub struct PeriodicNurbs3<T: Real> {
    knots: Vec<T>,
    control: Vec<Point3<T>>,
    period: T,
    /// The view over `[u₀, u₀ + T]`.
    base: NurbsCurve3<T>,
    /// The view over `[u₀, u₀ + 2T]`.
    two_turns: NurbsCurve3<T>,
    /// Whether the carrier's parameter is the data's negated: the same
    /// loop run back.
    flipped: bool,
}

impl<T: Real> core::fmt::Debug for PeriodicNurbs3<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PeriodicNurbs3")
            .field("knots", &self.knots)
            .field("control", &self.control)
            .field("period", &self.period)
            .field("flipped", &self.flipped)
            .finish_non_exhaustive()
    }
}

/// The two control points a clamped opening starts with and the one
/// before its last, from the cyclic data about the base knot: the
/// blossoms at `(u₀, u₀, u₀)`, `(u₀, u₀, u₁)` and `(u₋₁, u₀, u₀)`. Each
/// is a convex combination by the knot gaps alone.
fn opening_ends(knots: &[f64], period: f64, control: &[Point3<f64>]) -> [Point3<f64>; 3] {
    let n = knots.len();
    let gap = |i: usize| {
        if i + 1 < n {
            knots[i + 1] - knots[i]
        } else {
            (knots[0] + period) - knots[n - 1]
        }
    };
    // The gaps about the base knot: before it h₋₂, h₋₁, after it h₀, h₁.
    let (hm2, hm1, h0, h1) = (gap(n - 2), gap(n - 1), gap(0), gap(1));
    let mix = |p: Point3<f64>, q: Point3<f64>, wp: f64, wq: f64| {
        let s = wp + wq;
        Point3::new(
            (p.x * wp + q.x * wq) / s,
            (p.y * wp + q.y * wq) / s,
            (p.z * wp + q.z * wq) / s,
        )
    };
    let (dm1, d0, d1) = (control[n - 1], control[0], control[1]);
    // f[u₀, u₀, u₁]: between d₀ = f[u₋₁, u₀, u₁] and d₁ = f[u₀, u₁, u₂].
    let after = mix(d0, d1, h0 + h1, hm1);
    // f[u₋₁, u₀, u₀]: between d₋₁ = f[u₋₂, u₋₁, u₀] and d₀.
    let before = mix(dm1, d0, h0, hm2 + hm1);
    // f[u₀, u₀, u₀]: between the two.
    let at = mix(before, after, h0, hm1);
    [at, after, before]
}

impl PeriodicNurbs3<f64> {
    /// The periodic cubic on `knots` (one turn's, ascending), `period`
    /// and `control` (module docs for the convention).
    ///
    /// # Errors
    ///
    /// [`PeriodicError`], naming the clause.
    pub fn new(
        knots: Vec<f64>,
        control: Vec<Point3<f64>>,
        period: f64,
    ) -> Result<Self, PeriodicError> {
        let n = knots.len();
        if n != control.len() || n < PERIODIC_MIN_KNOTS {
            return Err(PeriodicError::Count {
                knots: n,
                control: control.len(),
            });
        }
        let finite = knots.iter().all(|k| k.is_finite())
            && period.is_finite()
            && control
                .iter()
                .all(|p| p.x.is_finite() && p.y.is_finite() && p.z.is_finite());
        if !finite {
            return Err(PeriodicError::NonFinite);
        }
        if let Some(i) = (1..n).find(|&i| knots[i] <= knots[i - 1]) {
            return Err(PeriodicError::Order { index: i });
        }
        if knots[0] + period <= knots[n - 1] {
            return Err(PeriodicError::Order { index: n });
        }
        let [at, after, before] = opening_ends(&knots, period, &control);
        let opening = |turns: usize| -> Result<NurbsCurve3<f64>, PeriodicError> {
            let end = knots[0] + period * turns as f64;
            let mut kv = vec![knots[0]; PERIODIC_DEGREE + 1];
            let mut net = vec![at, after];
            for turn in 0..turns {
                let shift = period * turn as f64;
                for i in 0..n {
                    if turn + i > 0 {
                        // The base turn's knots enter as stored.
                        kv.push(if turn == 0 {
                            knots[i]
                        } else {
                            knots[i] + shift
                        });
                    }
                    // d₁ … dₙ₋₁ of each turn, then d₀ of the next but the last.
                    let j = (i + 1) % n;
                    if !(turn + 1 == turns && j == 0) {
                        net.push(control[j]);
                    }
                }
            }
            kv.extend(core::iter::repeat_n(end, PERIODIC_DEGREE + 1));
            net.extend([before, at]);
            let weights = vec![1.0; net.len()];
            let kv = KnotVector::clamped(kv, PERIODIC_DEGREE).map_err(PeriodicError::Opening)?;
            NurbsCurve3::new(kv, net, weights).map_err(PeriodicError::Opening)
        };
        Ok(Self {
            base: opening(1)?,
            two_turns: opening(2)?,
            knots,
            control,
            period,
            flipped: false,
        })
    }

    /// **The periodic cubic through `points`**, a closed loop's samples
    /// in order with the first not repeated: knots at the chord-length
    /// parameters, the closing chord from the last point to the first
    /// included, normalized to the period 1
    /// ([`NurbsCurve3::chord_parameters`] of the loop with its first
    /// point appended, so a clamped fit of that list shares these
    /// parameters). The interpolant is the C² periodic cubic spline on
    /// those knots, found through its second derivatives at the knots (a
    /// cyclic tridiagonal system, strictly diagonally dominant for any
    /// gaps).
    ///
    /// # Errors
    ///
    /// [`FitError::TooFewPoints`] under [`PERIODIC_MIN_KNOTS`];
    /// [`FitError::NonFinitePoint`] and [`FitError::DegenerateChord`] as
    /// the chord parameters refuse; [`FitError::Assembly`] where the
    /// fitted data's openings refuse structure.
    pub fn interpolate(points: &[Point3<f64>]) -> Result<Self, FitError> {
        let n = points.len();
        if n < PERIODIC_MIN_KNOTS {
            return Err(FitError::TooFewPoints {
                have: n,
                need: PERIODIC_MIN_KNOTS,
            });
        }
        let mut closed = points.to_vec();
        closed.push(points[0]);
        let params = NurbsCurve3::<f64>::chord_parameters(&closed)?;
        let period = params[n];
        let knots = params[..n].to_vec();
        let h: Vec<f64> = (0..n).map(|i| params[i + 1] - params[i]).collect();
        let prev = |i: usize| (i + n - 1) % n;
        let next = |i: usize| (i + 1) % n;
        let coords = |p: Point3<f64>| [p.x, p.y, p.z];
        // Row i: h_{i−1}·M_{i−1} + 2(h_{i−1} + h_i)·M_i + h_i·M_{i+1} = r_i.
        let lower: Vec<f64> = (0..n).map(|i| h[prev(i)]).collect();
        let diag: Vec<f64> = (0..n).map(|i| 2.0 * (h[prev(i)] + h[i])).collect();
        let upper: Vec<f64> = h.clone();
        let rhs = |c: usize| -> Vec<f64> {
            (0..n)
                .map(|i| {
                    let (p, q, r) = (
                        coords(points[prev(i)])[c],
                        coords(points[i])[c],
                        coords(points[next(i)])[c],
                    );
                    6.0 * ((r - q) / h[i] - (q - p) / h[prev(i)])
                })
                .collect()
        };
        let second: [Vec<f64>; 3] = [0, 1, 2].map(|c| cyclic_solve(&lower, &diag, &upper, &rhs(c)));
        let control: Vec<Point3<f64>> = (0..n)
            .map(|i| {
                let (hb, ha) = (h[prev(i)], h[i]);
                let at = |c: usize| {
                    let (m, m_next) = (second[c][i], second[c][next(i)]);
                    let q = coords(points[i])[c];
                    let slope =
                        (coords(points[next(i)])[c] - q) / ha - ha * (2.0 * m + m_next) / 6.0;
                    q + slope * (ha - hb) / 3.0 - m * hb * ha / 6.0
                };
                Point3::new(at(0), at(1), at(2))
            })
            .collect();
        Self::new(knots, control, period).map_err(|e| match e {
            PeriodicError::Opening(e) => FitError::Assembly(geom_core::spline::Built::new(e)),
            PeriodicError::NonFinite => FitError::NonFinitePoint { index: 0 },
            PeriodicError::Count { knots, .. } => FitError::TooFewPoints {
                have: knots,
                need: PERIODIC_MIN_KNOTS,
            },
            PeriodicError::Order { index } => FitError::DegenerateChord { index },
        })
    }
}

/// The solution of the cyclic tridiagonal system whose row `i` is
/// `lower[i]·x[i−1] + diag[i]·x[i] + upper[i]·x[i+1] = rhs[i]`, indices
/// cyclic: the corner entries folded into a rank-one update of the
/// tridiagonal part (Sherman–Morrison), each tridiagonal solve the
/// pivot-free elimination a strictly diagonally dominant system admits.
fn cyclic_solve(lower: &[f64], diag: &[f64], upper: &[f64], rhs: &[f64]) -> Vec<f64> {
    let n = diag.len();
    // Top-right and bottom-left corners.
    let (beta, alpha) = (lower[0], upper[n - 1]);
    let gamma = -diag[0];
    let mut d = diag.to_vec();
    d[0] -= gamma;
    d[n - 1] -= alpha * beta / gamma;
    let solve = |r: &[f64]| -> Vec<f64> {
        let mut c = vec![0.0; n];
        let mut x = vec![0.0; n];
        let mut pivot = d[0];
        x[0] = r[0] / pivot;
        for i in 1..n {
            c[i] = upper[i - 1] / pivot;
            pivot = d[i] - lower[i] * c[i];
            x[i] = (r[i] - lower[i] * x[i - 1]) / pivot;
        }
        for i in (0..n - 1).rev() {
            x[i] -= c[i + 1] * x[i + 1];
        }
        x
    };
    let y = solve(rhs);
    let mut u = vec![0.0; n];
    u[0] = gamma;
    u[n - 1] = alpha;
    let z = solve(&u);
    let fact = (y[0] + beta * y[n - 1] / gamma) / (1.0 + z[0] + beta * z[n - 1] / gamma);
    y.iter().zip(&z).map(|(y, z)| y - fact * z).collect()
}

impl<T: Real> PeriodicNurbs3<T> {
    /// The period, in the parameter.
    pub fn period(&self) -> T {
        self.period
    }

    /// How many knots, and control points, a turn holds.
    pub fn knot_count(&self) -> usize {
        self.knots.len()
    }

    /// The same loop run back: the carrier whose point at `t` is this
    /// one's at `−t`.
    #[must_use]
    pub fn reversed(&self) -> Self {
        Self {
            flipped: !self.flipped,
            ..self.clone()
        }
    }

    /// The same curve read at another scalar: `f` applied to every
    /// stored value and to the two openings' ([`NurbsCurve3::map_scalar`],
    /// which says what `f` must be).
    #[must_use]
    pub fn map_scalar<U: Real>(&self, f: impl Fn(T) -> U) -> PeriodicNurbs3<U> {
        PeriodicNurbs3 {
            knots: self.knots.iter().map(|&k| f(k)).collect(),
            control: self.control.iter().map(|p| p.map(&f)).collect(),
            period: f(self.period),
            base: self.base.map_scalar(&f),
            two_turns: self.two_turns.map_scalar(&f),
            flipped: self.flipped,
        }
    }

    /// The same curve with every control point carried through `f`,
    /// which the caller owes affine ([`NurbsCurve3::map_points`]): the
    /// openings' control points are affine combinations of the cyclic
    /// ones, so they map with them.
    #[must_use]
    pub fn map_points(&self, f: impl Fn(Point3<T>) -> Point3<T>) -> Self {
        Self {
            knots: self.knots.clone(),
            control: self.control.iter().map(|p| f(*p)).collect(),
            period: self.period,
            base: self.base.map_points(&f),
            two_turns: self.two_turns.map_points(&f),
            flipped: self.flipped,
        }
    }

    /// **Whether `other` is this curve by content**: the cyclic data
    /// and the sense, every value one by `same`.
    pub fn same_content(&self, other: &Self, same: impl Fn(T, T) -> bool) -> bool {
        self.flipped == other.flipped
            && self.knots.len() == other.knots.len()
            && same(self.period, other.period)
            && self
                .knots
                .iter()
                .zip(&other.knots)
                .all(|(&a, &b)| same(a, b))
            && self
                .control
                .iter()
                .zip(&other.control)
                .all(|(p, q)| same(p.x, q.x) && same(p.y, q.y) && same(p.z, q.z))
    }

    /// The base turn `[u₀, u₀ + T]` in the carrier's parameter, on the
    /// value channel: the span of [`Self::base_opening`].
    pub fn base_span(&self) -> (f64, f64) {
        self.oriented_span(self.base.domain())
    }

    /// The two turns a clamped view lies in, in the carrier's
    /// parameter, on the value channel.
    pub fn view_window(&self) -> (f64, f64) {
        self.oriented_span(self.two_turns.domain())
    }

    fn oriented_span(&self, (lo, hi): (f64, f64)) -> (f64, f64) {
        if self.flipped { (-hi, -lo) } else { (lo, hi) }
    }

    /// The base opening in the stored data's own sense, for a reading
    /// that depends on neither the sense nor the parameter: a bound over
    /// the whole loop, or the hull of its net.
    pub fn whole_loop(&self) -> &NurbsCurve3<T> {
        &self.base
    }

    /// The parameter of the stored data at the carrier's `t`.
    fn data_param(&self, t: T) -> T {
        if self.flipped { T::zero() - t } else { t }
    }
}

impl<T: SpanLocate> PeriodicNurbs3<T> {
    /// **The base turn's knot spans' ends**, ascending in the carrier's
    /// parameter, on the value channel: the schedule of a reader that
    /// samples the loop span by span. Not a clamped knot vector: the
    /// first and the last are one knot a period apart.
    pub fn turn_breaks(&self) -> Vec<f64> {
        let breaks = self
            .base
            .knot_values()
            .breakpoint_values()
            .unwrap_or_default();
        if self.flipped {
            breaks.into_iter().rev().map(|k| -k).collect()
        } else {
            breaks
        }
    }

    /// A stored view in the carrier's sense.
    fn oriented(&self, net: NurbsCurve3<T>) -> NurbsCurve3<T> {
        if !self.flipped {
            return net;
        }
        NurbsCurve3::new(
            net.knot_values().negated(),
            net.control().iter().rev().copied().collect(),
            net.weights().iter().rev().copied().collect(),
        )
        .unwrap_or_else(|e| unreachable!("a valid net reversed onto its negated knots: {e}"))
    }

    /// A certified lower bound on the speed over the whole loop
    /// ([`NurbsCurve3::speed_lower_bound`] of the base opening).
    pub fn speed_lower_bound(&self) -> geom_core::InfSpeed<T> {
        self.base.speed_lower_bound()
    }

    /// **The base opening**: the clamped view over [`Self::base_span`],
    /// whose knots are the stored ones (module docs). The whole loop,
    /// once.
    #[must_use]
    pub fn base_opening(&self) -> NurbsCurve3<T> {
        self.oriented(self.base.clone())
    }

    /// **The clamped view over `[a, b]`** (module docs): the curve's
    /// restriction to the span as a clamped cubic in the carrier's own
    /// parameter, its interior knots the curve's own, each simple. The
    /// span lies in [`Self::view_window`].
    ///
    /// # Errors
    ///
    /// [`PeriodicViewError::Span`] for a span that is not forward or
    /// leaves the window; [`PeriodicViewError::Insertion`] where the
    /// knot insertion at an end refuses.
    pub fn clamped_view(&self, (a, b): (f64, f64)) -> Result<NurbsCurve3<T>, PeriodicViewError> {
        let (lo, hi) = self.two_turns.domain();
        let (s0, s1) = if self.flipped { (-b, -a) } else { (a, b) };
        if !(lo <= s0 && s0 < s1 && s1 <= hi) {
            return Err(PeriodicViewError::Span {
                asked: (a, b),
                window: self.view_window(),
            });
        }
        let (base_lo, base_hi) = self.base.domain();
        if s0 == base_lo && s1 == base_hi {
            return Ok(self.base_opening());
        }
        let refused = |at: f64| PeriodicViewError::Insertion { asked: (a, b), at };
        let mut piece = self.two_turns.clone();
        if s0 > lo {
            piece = piece.split_at(s0).map_err(|_| refused(s0))?.1;
        }
        if s1 < hi {
            piece = piece.split_at(s1).map_err(|_| refused(s1))?.0;
        }
        Ok(self.oriented(piece))
    }

    /// The data's parameter for the carrier's `t`, less the whole
    /// periods that bring its bracket's lower end into the base turn,
    /// so a value under a period wide reads inside the two-turn opening.
    /// Poison is returned as it is.
    fn reduced(&self, t: T) -> T {
        let s = self.data_param(t);
        let (lo, _) = self.base.domain();
        let Some(period) = self.period.point_value() else {
            return s;
        };
        let Some((at, _)) = s.structure_bracket() else {
            return s;
        };
        let turns = ((at - lo) / period).floor();
        if turns == 0.0 || !turns.is_finite() {
            return s;
        }
        s - self.period * T::from_f64(turns)
    }

    fn oriented_vec(&self, v: Vec3<T>) -> Vec3<T> {
        if self.flipped { -v } else { v }
    }

    /// The point at `t`.
    pub fn eval(&self, t: T) -> Point3<T> {
        self.two_turns.eval(self.reduced(t))
    }

    /// The first derivative at `t`.
    pub fn deriv(&self, t: T) -> Vec3<T> {
        self.oriented_vec(self.two_turns.deriv(self.reduced(t)))
    }

    /// The point and the first derivative at `t`.
    pub fn ders1(&self, t: T) -> (Point3<T>, Vec3<T>) {
        let (p, d) = self.two_turns.ders1(self.reduced(t));
        (p, self.oriented_vec(d))
    }

    /// The second derivative at `t`.
    pub fn deriv2(&self, t: T) -> Vec3<T> {
        self.two_turns.deriv2(self.reduced(t))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::float_cmp)]
mod tests {
    use super::*;

    /// An uneven loop: points of a tilted, wobbling oval at uneven angles.
    fn loop_points(n: usize) -> Vec<Point3<f64>> {
        (0..n)
            .map(|i| {
                let s = i as f64 / n as f64;
                let a = core::f64::consts::TAU
                    * (s + 0.07 * (3.0 * s * core::f64::consts::TAU).sin() / 3.0);
                Point3::new(
                    2.0 * a.cos(),
                    a.sin() + 0.3 * (2.0 * a).cos(),
                    0.5 * (3.0 * a).sin(),
                )
            })
            .collect()
    }

    fn dist(p: Point3<f64>, q: Point3<f64>) -> f64 {
        (p - q).norm()
    }

    /// A net's interior knots with their multiplicities.
    fn interior(net: &NurbsCurve3<f64>) -> Vec<(f64, usize)> {
        let runs: Vec<(f64, usize)> = net.knot_values().runs().collect();
        runs[1..runs.len() - 1].to_vec()
    }

    /// The fit passes through its points at its knots, closes on one
    /// control point, and its openings hold only simple interior knots.
    #[test]
    fn the_fit_interpolates_and_its_openings_are_simple() {
        let points = loop_points(23);
        let c = PeriodicNurbs3::interpolate(&points).unwrap();
        assert_eq!(c.period(), 1.0);
        let base = c.base_opening();
        assert_eq!(base.domain(), (0.0, 1.0));
        assert_eq!(
            dist(base.control()[0], base.control()[base.control().len() - 1]),
            0.0,
            "the loop closes on one value"
        );
        for net in [&base, &c.two_turns] {
            assert!(
                interior(net).into_iter().all(|(_, m)| m == 1),
                "an opening's interior knots are simple"
            );
        }
        let two: Vec<f64> = interior(&c.two_turns).into_iter().map(|(k, _)| k).collect();
        assert_eq!(two.len(), 2 * points.len() - 1);
        for (i, p) in points.iter().enumerate() {
            let t = c.knots[i];
            assert!(
                dist(c.eval(t), *p) < 1e-13,
                "knot {i}: {}",
                dist(c.eval(t), *p)
            );
            assert!(dist(c.eval(t + 1.0), *p) < 1e-13, "knot {i}, a turn on");
            assert!(
                dist(c.eval(t - 2.0), *p) < 1e-13,
                "knot {i}, two turns back"
            );
        }
    }

    /// The loop is C² across its base knot: the two ends of the base
    /// opening agree in point, first and second derivative.
    #[test]
    fn the_base_knot_is_no_start() {
        let c = PeriodicNurbs3::interpolate(&loop_points(17)).unwrap();
        let base = c.base_opening();
        let (p0, d0, a0) = base.ders(0.0);
        let (p1, d1, a1) = base.ders(1.0);
        assert_eq!(dist(p0, p1), 0.0);
        assert!((d0 - d1).norm() < 1e-11 * d0.norm(), "{:?} {:?}", d0, d1);
        assert!((a0 - a1).norm() < 1e-9 * a0.norm(), "{:?} {:?}", a0, a1);
        // And a shifted loop is the same curve: the fit has no start.
        let mut shifted = loop_points(17);
        shifted.rotate_left(5);
        let s = PeriodicNurbs3::interpolate(&shifted).unwrap();
        let offset = c.knots[5];
        for k in 0..40 {
            let t = k as f64 / 40.0;
            let gap = dist(c.eval(offset + t), s.eval(t));
            assert!(gap < 1e-12, "t = {t}: {gap}");
        }
    }

    /// A clamped view is the curve over its span, on simple knots,
    /// across the base knot's copy and from a knot exactly.
    #[test]
    fn a_clamped_view_is_the_curve_over_its_span() {
        let c = PeriodicNurbs3::interpolate(&loop_points(23)).unwrap();
        let knot = c.knots[9];
        for (a, b) in [
            (0.3, 1.2),
            (0.71, 1.71),
            (knot, knot + 1.0),
            (0.0, 0.4),
            (1.5, 2.0),
        ] {
            let view = c.clamped_view((a, b)).unwrap();
            assert_eq!(view.domain(), (a, b));
            assert!(interior(&view).into_iter().all(|(_, m)| m == 1));
            for k in 0..=50 {
                let t = a + (b - a) * f64::from(k) / 50.0;
                let gap = dist(view.eval(t), c.eval(t));
                assert!(gap < 1e-13, "[{a}, {b}] at {t}: {gap}");
            }
        }
        // A view from a knot holds that knot as its end, not inside.
        let view = c.clamped_view((knot, knot + 1.0)).unwrap();
        assert_eq!(interior(&view).into_iter().count(), c.knot_count() - 1);
        assert!(matches!(
            c.clamped_view((1.5, 2.5)),
            Err(PeriodicViewError::Span { .. })
        ));
        assert!(matches!(
            c.clamped_view((0.5, 0.5)),
            Err(PeriodicViewError::Span { .. })
        ));
    }

    /// The reversed carrier is the same loop at the negated parameter,
    /// its views the forward views run back.
    #[test]
    fn the_reversed_carrier_reads_the_negated_parameter() {
        let c = PeriodicNurbs3::interpolate(&loop_points(19)).unwrap();
        let r = c.reversed();
        assert_eq!(r.base_span(), (-1.0, 0.0));
        for k in 0..=30 {
            let t = -1.7 + 1.9 * f64::from(k) / 30.0;
            assert_eq!(dist(r.eval(t), c.eval(-t)), 0.0);
            assert_eq!((r.deriv(t) + c.deriv(-t)).norm(), 0.0);
        }
        let view = r.clamped_view((-1.4, -0.6)).unwrap();
        assert_eq!(view.domain(), (-1.4, -0.6));
        for k in 0..=20 {
            let t = -1.4 + 0.8 * f64::from(k) / 20.0;
            assert!(dist(view.eval(t), c.eval(-t)) < 1e-13);
        }
        assert!(r.reversed().same_content(&c, |a, b| a == b));
        assert!(!r.same_content(&c, |a, b| a == b));
    }

    /// A view that starts an ulp under a knot keeps the knot inside, as
    /// a first span that narrow; one an ulp over it leaves the knot out.
    #[test]
    fn a_view_beside_a_knot_keeps_the_exact_insertion() {
        let c = PeriodicNurbs3::interpolate(&loop_points(23)).unwrap();
        let knot = c.knots[9];
        let (under, over) = (knot.next_down(), knot.next_up());
        let below = c.clamped_view((under, under + 0.5)).unwrap();
        let first = interior(&below).into_iter().next().unwrap().0;
        assert_eq!(first, knot, "the knot is the view's first interior knot");
        let above = c.clamped_view((over, over + 0.5)).unwrap();
        let first = interior(&above).into_iter().next().unwrap().0;
        assert_eq!(first, c.knots[10], "the knot is outside the view");
        for view in [&below, &above] {
            let (a, _) = view.domain();
            assert!(dist(view.eval(a), c.eval(a)) < 1e-13);
        }
    }

    #[test]
    fn data_that_is_no_periodic_cubic_is_refused_by_clause() {
        let p = Point3::new(0.0, 0.0, 0.0);
        assert!(matches!(
            PeriodicNurbs3::new(vec![0.0, 0.5], vec![p, p], 1.0),
            Err(PeriodicError::Count {
                knots: 2,
                control: 2
            })
        ));
        assert_eq!(
            PeriodicNurbs3::new(vec![0.0, 0.5, 0.4, 0.8], vec![p; 4], 1.0).err(),
            Some(PeriodicError::Order { index: 2 })
        );
        assert_eq!(
            PeriodicNurbs3::new(vec![0.0, 0.2, 0.4, 0.8], vec![p; 4], 0.8).err(),
            Some(PeriodicError::Order { index: 4 })
        );
        assert_eq!(
            PeriodicNurbs3::new(vec![0.0, 0.2, 0.4, f64::NAN], vec![p; 4], 1.0).err(),
            Some(PeriodicError::NonFinite)
        );
        assert!(matches!(
            PeriodicNurbs3::interpolate(&[p, p]),
            Err(FitError::TooFewPoints { have: 2, need: 3 })
        ));
    }
}
