//! The global curve-fitting stack (The NURBS Book §9.4, M5 PR 4 spec
//! §c): chord-length global **interpolation** (the A9.1 shape, exact
//! collocation solve) and **approximation-with-bound** (the A9.10
//! Type-2 loop shape: fit low degree, knot-remove under PR 3's removal
//! bound, degree-elevate, refit) for 3-D curves and 2-D pcurves — one
//! generic core over the point dimension (the `NurbsCurve` macro
//! precedent).
//!
//! # C6, verbatim and binding
//!
//! Every branch in this module — knot counts, degrees, removal
//! acceptance, convergence — is **f64 STRUCTURE selection**:
//! deterministic per D9 (same inputs ⇒ same knots, degrees, and
//! control bits), never routed through `k_stats` (no topology decision
//! exists here), never consulted by topology (cache shape is invisible
//! to predicates). Raw `f64` comparisons below are all structure
//! selection under that rule.
//!
//! # OUR bound semantics (not Eq. 9.77's)
//!
//! [`FitOutcome::bound`] is a **certified sup-norm steering bound on
//! the deviation from the exact chord interpolant of the data** — the
//! degree-1 polyline through the samples — measured **directly**, not
//! accumulated: the candidate and the interpolant are brought onto one
//! shared knot vector (degree elevation + knot refinement, both exact
//! in ℝ — PR 3's plans), where the deviation is bounded by the
//! partition-of-unity control-difference formula
//! (`NurbsCurve::same_structure_deviation_bound` — the same convexity
//! fact as every hull bound, with the removal-bound machinery's
//! weight-perturbation term). A knot removal is **accepted only if the
//! candidate's direct bound stays within tolerance**; the final
//! least-squares refit is accepted under the same test (otherwise the
//! pipeline curve, whose bound already holds, is returned unchanged).
//! PR 3's per-removal `remove_knot` bound steers nothing here, on
//! measurement: triangle-inequality accumulation across dozens of
//! removals proved an order too conservative to permit ANY compression
//! at realistic tolerances — the direct bound certifies the same
//! deviation tightly, from the same Eq. 9.81-family machinery.
//!
//! Because the interpolant passes through every sample, the bound also
//! bounds every data deviation `|C(ū_k) − Q_k|`. It is *steering*
//! grade — evaluated in `f64` arithmetic over exact-in-ℝ identities —
//! exactly the C1 rung-3 note's division of labor: the Book's bound
//! (Eq. 9.77's sampled data deviation) merely steers the Book's loop;
//! ours certifies against the interpolant, and the C2 certificate
//! against the locus stays the consumer's, via
//! `geom_core::spline::compose` hull bounds.
//!
//! **Worked example** (pinned in `tests/fit_certify.rs`): 65 samples
//! of a radius-2.5 m quarter arc, degree 3, input tolerance `1e-2` ⇒
//! 30 control points (the loop removed over half the structure), the
//! achieved bound ≈ `9.9e-3` ≤ the tolerance (the input tolerance is
//! the acceptance budget; the achieved value is what the accepted
//! state actually measures), and the compose hull bound of the fitted
//! curve's sphere residual is consistent with that order. Denser data
//! (129 samples) lets the refit engage and the achieved bound drop to
//! the data's own chordal scale (≈ `6e-5`).
//!
//! # The budget (never an unbounded iteration)
//!
//! The loop's removal work is capped by the named constant
//! [`FIT_REMOVAL_BUDGET`] (counting `remove_knot` attempts); expiry is
//! the typed [`FitError::BudgetExhausted`] **carrying the achieved
//! bound** — the Book's own "both can fail to converge and this
//! eventuality must be dealt with" honesty, as a type.

use geom_core::linalg::lsq::{self, LsqError};
use geom_core::spline::{KnotAlgebraError, KnotVector, KnotVectorIssue, SplineError, basis};
use geom_core::{Point2, Point3};

use crate::{NurbsCurve2, NurbsCurve3};

/// Fixed cap on `remove_knot` attempts across the whole Type-2 loop
/// (D9: a named constant, never data-dependent tuning). Generous for
/// fitting-sized inputs (hundreds of samples); expiry is typed.
pub const FIT_REMOVAL_BUDGET: usize = 16_384;

/// A typed fitting refusal (fail-loud; the kernel never panics).
#[derive(Clone, Debug, PartialEq)]
pub enum FitError {
    /// Fewer data points than the fit shape needs.
    TooFewPoints {
        /// Points supplied.
        have: usize,
        /// Points required.
        need: usize,
    },
    /// A data point has a non-finite coordinate.
    NonFinitePoint {
        /// Index of the offending point.
        index: usize,
    },
    /// Two consecutive data points coincide: chord-length
    /// parameterization has no valid parameter step there.
    DegenerateChord {
        /// Index of the second point of the zero-length chord.
        index: usize,
    },
    /// The tolerance is not a finite positive number.
    InvalidTolerance {
        /// The offending value.
        tolerance: f64,
    },
    /// A linear solve refused (degenerate collocation/normal system).
    Lsq(LsqError),
    /// Spline structure construction refused.
    Structure(SplineError),
    /// A knot-algebra operation refused unexpectedly.
    KnotAlgebra(KnotAlgebraError),
    /// The Type-2 loop spent [`FIT_REMOVAL_BUDGET`] removal attempts
    /// without finishing; carries the bound achieved so far (OUR
    /// semantics — module docs).
    BudgetExhausted {
        /// The attempt budget that expired.
        budget: usize,
        /// The certified bound accumulated before expiry.
        achieved: f64,
    },
}

impl core::fmt::Display for FitError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FitError::TooFewPoints { have, need } => {
                write!(f, "fit: {have} points, need at least {need}")
            }
            FitError::NonFinitePoint { index } => {
                write!(f, "fit: point {index} has a non-finite coordinate")
            }
            FitError::DegenerateChord { index } => {
                write!(f, "fit: zero-length chord ending at point {index}")
            }
            FitError::InvalidTolerance { tolerance } => {
                write!(f, "fit: invalid tolerance {tolerance}")
            }
            FitError::Lsq(e) => write!(f, "fit: {e}"),
            FitError::Structure(e) => write!(f, "fit: {e}"),
            FitError::KnotAlgebra(e) => write!(f, "fit: {e}"),
            FitError::BudgetExhausted { budget, achieved } => write!(
                f,
                "fit: removal budget {budget} exhausted (achieved bound {achieved:e})"
            ),
        }
    }
}

impl core::error::Error for FitError {}

/// Why the final least-squares refit was NOT applied (deterministic
/// structure information — every skip path is a documented branch, and
/// the worked-example test pins which one fires).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RefitSkip {
    /// More control points than data samples (an LSQ over the interior
    /// would be underdetermined) or too few of either to build the
    /// pinned-endpoint system.
    StructureRicherThanData,
    /// The refit normal system was degenerate under the fixed
    /// elimination order — a structural statement (some control point's
    /// basis support contains no interior data site; C⁰ corners from
    /// full-multiplicity interior knots do this). The certified
    /// pipeline curve is returned instead.
    DegenerateSystem,
    /// The refit solved, but its direct certified deviation bound
    /// exceeded the tolerance (or poisoned) — the pipeline curve, whose
    /// bound already holds, is returned instead.
    BoundExceeded,
}

/// An approximation result: the fitted curve plus the certified
/// steering bound OUR semantics defines (module docs), and whether the
/// final least-squares refit was accepted within tolerance.
#[derive(Clone, Debug)]
pub struct FitOutcome<C> {
    /// The fitted curve.
    pub curve: C,
    /// The achieved sup-norm bound on deviation from the exact chord
    /// interpolant of the data (≤ the input tolerance on success).
    pub bound: f64,
    /// Whether the final LSQ refit was applied.
    pub refit_applied: bool,
    /// The skip reason when `refit_applied` is `false` (`None` exactly
    /// when it is `true`).
    pub refit_skip: Option<RefitSkip>,
}

/// The rational basis row at `t`: `(first, values)` with
/// `values[j] = N_j(t)·w_{first+j} / Σ_j N_j(t)·w_{first+j}` — the
/// `p + 1` nonzero rational basis functions, denominator accumulated
/// ascending `j` (D9).
fn rational_row(kv: &KnotVector, weights: &[f64], t: f64) -> (usize, Vec<f64>) {
    let span = kv.find_span(t);
    let p = kv.degree();
    let n = basis::basis_funs(kv, span, t);
    let first = span - p;
    let mut den = 0.0f64;
    for (j, nj) in n.iter().enumerate() {
        den += nj * weights[first + j];
    }
    let vals = n
        .iter()
        .enumerate()
        .map(|(j, nj)| nj * weights[first + j] / den)
        .collect();
    (first, vals)
}

/// The averaged clamped knot vector for interpolation at `degree` from
/// chord parameters (Book Eq. 9.8): `p + 1` zeros, interior
/// `u_{j+p} = (ū_j + … + ū_{j+p−1})/p` (sum ascending), `p + 1` ones.
fn averaged_knots(params: &[f64], degree: usize) -> Result<KnotVector, FitError> {
    let n = params.len();
    let mut knots = vec![0.0f64; degree + 1];
    for j in 1..(n - degree) {
        let mut s = 0.0f64;
        for u in &params[j..j + degree] {
            s += *u;
        }
        #[allow(clippy::cast_precision_loss)]
        knots.push(s / degree as f64);
    }
    knots.extend(core::iter::repeat_n(1.0, degree + 1));
    KnotVector::clamped(knots, degree).map_err(FitError::Structure)
}

/// The distinct interior knot values of `kv`, ascending (structure
/// scan, exact `f64` identity) — the removal sweep's candidate list.
fn distinct_interior(kv: &KnotVector) -> Vec<f64> {
    let p = kv.degree();
    let knots = kv.knots();
    let mut out: Vec<f64> = Vec::new();
    for u in &knots[p + 1..knots.len() - p - 1] {
        if out.last() != Some(u) {
            out.push(*u);
        }
    }
    out
}

macro_rules! nurbs_fit {
    ($Curve:ident, $Point:ident, $($c:ident),+) => {
        impl $Curve<f64> {
            /// Chord-length parameters (Book Eqs. 9.4–9.5): `ū₀ = 0`,
            /// `ū_k = ū_{k−1} + |Q_k − Q_{k−1}|/total`, last pinned to
            /// exactly 1. Ascending accumulation (D9).
            fn chord_params(points: &[$Point<f64>]) -> Result<Vec<f64>, FitError> {
                for (index, q) in points.iter().enumerate() {
                    if $(!q.$c.is_finite())||+ {
                        return Err(FitError::NonFinitePoint { index });
                    }
                }
                let mut chords = Vec::with_capacity(points.len() - 1);
                let mut total = 0.0f64;
                for (k, pair) in points.windows(2).enumerate() {
                    let d = (pair[1] - pair[0]).norm();
                    // `!(d > 0)` is NaN-catching (structure selection).
                    #[allow(clippy::neg_cmp_op_on_partial_ord)]
                    if !(d > 0.0) {
                        return Err(FitError::DegenerateChord { index: k + 1 });
                    }
                    chords.push(d);
                    total += d;
                }
                let mut params = Vec::with_capacity(points.len());
                params.push(0.0);
                let mut acc = 0.0f64;
                for d in &chords {
                    acc += d / total;
                    params.push(acc);
                }
                // Pin the end exactly (accumulation rounds).
                params[points.len() - 1] = 1.0;
                Ok(params)
            }

            /// Global chord-length interpolation (the A9.1 shape): the
            /// curve of `degree` through every point, on the averaged
            /// knot vector, via the exact square collocation solve
            /// (`geom_core::linalg::lsq::solve_square`, fixed-order LU).
            /// All weights are exactly 1 (fitting produces integral
            /// B-splines; rational data is not a fitting output).
            ///
            /// Structure selection is deterministic bit-for-bit (D9):
            /// same points and degree ⇒ same knots, same control bits.
            ///
            /// # Errors
            ///
            /// [`FitError`]: too few points (`degree + 1` minimum),
            /// non-finite input, a zero-length chord, a degenerate
            /// collocation system, or structure refusals.
            pub fn interpolate(points: &[$Point<f64>], degree: usize) -> Result<Self, FitError> {
                if degree == 0 {
                    return Err(FitError::Structure(SplineError::KnotVectorInvalid {
                        reason: KnotVectorIssue::DegreeZero,
                    }));
                }
                let n = points.len();
                if n < degree + 1 || n < 2 {
                    return Err(FitError::TooFewPoints {
                        have: n,
                        need: (degree + 1).max(2),
                    });
                }
                let params = Self::chord_params(points)?;
                let kv = averaged_knots(&params, degree)?;
                let weights = vec![1.0f64; n];
                let mut matrix = vec![vec![0.0f64; n]; n];
                for (k, u) in params.iter().enumerate() {
                    let (first, vals) = rational_row(&kv, &weights, *u);
                    for (j, v) in vals.iter().enumerate() {
                        matrix[k][first + j] = *v;
                    }
                }
                let rhs: Vec<Vec<f64>> = points.iter().map(|q| vec![$(q.$c),+]).collect();
                let sol = lsq::solve_square(&matrix, &rhs).map_err(FitError::Lsq)?;
                let control: Vec<$Point<f64>> = sol
                    .iter()
                    .map(|row| {
                        let mut it = row.iter().copied();
                        $(let $c = it.next().unwrap_or(f64::NAN);)+
                        $Point::new($($c),+)
                    })
                    .collect();
                Self::new(kv, control, weights).map_err(FitError::Structure)
            }

            /// Approximation with a certified steering bound — the
            /// A9.10 Type-2 loop shape under OUR bound semantics
            /// (module docs): degree-1 interpolation, then per degree
            /// `1..=degree` repeated knot-removal sweeps accepting a
            /// removal only while the accumulated bound stays within
            /// `tolerance`, degree elevation between rungs (exact in
            /// ℝ), and a final least-squares refit on the selected
            /// structure accepted under the same budget.
            ///
            /// Deterministic bit-for-bit (D9); removal work capped by
            /// [`FIT_REMOVAL_BUDGET`].
            ///
            /// # Errors
            ///
            /// [`FitError`]: the interpolation refusals,
            /// [`FitError::BudgetExhausted`] with the achieved bound,
            /// or propagated structure/solve refusals.
            pub fn approximate(
                points: &[$Point<f64>],
                degree: usize,
                tolerance: f64,
            ) -> Result<FitOutcome<Self>, FitError> {
                Self::approximate_budgeted(points, degree, tolerance, FIT_REMOVAL_BUDGET)
            }

            /// [`Self::approximate`] with an explicit attempt budget —
            /// crate-internal so the public entry keeps the ONE named
            /// constant (D9) while the refusal path stays testable.
            pub(crate) fn approximate_budgeted(
                points: &[$Point<f64>],
                degree: usize,
                tolerance: f64,
                budget: usize,
            ) -> Result<FitOutcome<Self>, FitError> {
                // `!(tolerance > 0)` is NaN-catching (NaN refuses;
                // `<= 0` would pass NaN) — the fail-loud direction.
                #[allow(clippy::neg_cmp_op_on_partial_ord)]
                if !(tolerance > 0.0) || !tolerance.is_finite() {
                    return Err(FitError::InvalidTolerance { tolerance });
                }
                if degree == 0 {
                    return Err(FitError::Structure(SplineError::KnotVectorInvalid {
                        reason: KnotVectorIssue::DegreeZero,
                    }));
                }
                let params = Self::chord_params(points)?;
                let interp = Self::interpolate(points, 1)?;
                let mut cur = interp.clone();
                // The reference at the current rung's degree: the
                // exact-in-ℝ elevation of the chord interpolant.
                let mut reference = interp;
                let mut bound = 0.0f64;
                let mut attempts = 0usize;
                for d in 1..=degree {
                    // Removal sweeps: retry until a full sweep accepts
                    // nothing (each removed copy shrinks the knot list,
                    // so this terminates; the budget caps it anyway).
                    loop {
                        let mut removed_any = false;
                        for u in distinct_interior(cur.knots()) {
                            if attempts >= budget {
                                return Err(FitError::BudgetExhausted {
                                    budget,
                                    achieved: bound,
                                });
                            }
                            attempts += 1;
                            match cur.remove_knot(u, 1) {
                                Ok((cand, _)) => {
                                    // Direct certified deviation vs
                                    // the interpolant (module docs:
                                    // OUR bound semantics).
                                    let b = cand.deviation_from(&reference)?;
                                    if b <= tolerance {
                                        cur = cand;
                                        bound = b;
                                        removed_any = true;
                                    }
                                }
                                // A weight collapsing out of the
                                // positive regime refuses THAT removal;
                                // the curve stands and the sweep goes on.
                                Err(KnotAlgebraError::WeightCollapse { .. }) => {}
                                Err(e) => return Err(FitError::KnotAlgebra(e)),
                            }
                        }
                        if !removed_any {
                            break;
                        }
                    }
                    if d < degree {
                        cur = cur.elevate_degree(1).map_err(FitError::KnotAlgebra)?;
                        reference = reference.elevate_degree(1).map_err(FitError::KnotAlgebra)?;
                    }
                }
                let (curve, bound, refit_skip) =
                    Self::refit_within(cur, &reference, points, &params, bound, tolerance)?;
                Ok(FitOutcome {
                    curve,
                    bound,
                    refit_applied: refit_skip.is_none(),
                    refit_skip,
                })
            }

            /// The direct certified deviation bound vs `reference`
            /// (same degree): union-refine both onto the shared knot
            /// vector (exact in ℝ), then the partition-of-unity
            /// control-difference bound. `f64` structure arithmetic
            /// throughout (steering grade — module docs).
            fn deviation_from(&self, reference: &Self) -> Result<f64, FitError> {
                // Union interior multiplicities (values are exact f64
                // structure shared by construction: both curves'
                // interior knots descend from the same chord params).
                let mut need: Vec<(f64, usize)> = Vec::new();
                for kv in [self.knots(), reference.knots()] {
                    for u in distinct_interior(kv) {
                        let m = kv.multiplicity_of(u).map_or(0, |(s, _)| s);
                        match need.iter_mut().find(|(v, _)| *v == u) {
                            Some(entry) => entry.1 = entry.1.max(m),
                            None => need.push((u, m)),
                        }
                    }
                }
                let refine_to = |c: &Self| -> Result<Self, FitError> {
                    let mut add: Vec<f64> = Vec::new();
                    for (u, m) in &need {
                        let have = c.knots().multiplicity_of(*u).map_or(0, |(s, _)| s);
                        for _ in have..*m {
                            add.push(*u);
                        }
                    }
                    if add.is_empty() {
                        Ok(c.clone())
                    } else {
                        c.refine_knots(&add).map_err(FitError::KnotAlgebra)
                    }
                };
                let a = refine_to(self)?;
                let b = refine_to(reference)?;
                Ok(a.same_structure_deviation_bound(&b))
            }

            /// The final LSQ refit (Eqs. 9.63–9.67 shape: endpoints
            /// pinned to the end samples, interior control points by
            /// least squares on the pipeline structure), accepted only
            /// if ITS direct certified deviation from the interpolant
            /// stays within tolerance (module docs). A skip returns
            /// the pipeline curve and names its path as a
            /// [`RefitSkip`] (pinned by the worked-example test).
            fn refit_within(
                cur: Self,
                reference: &Self,
                points: &[$Point<f64>],
                params: &[f64],
                bound: f64,
                tolerance: f64,
            ) -> Result<(Self, f64, Option<RefitSkip>), FitError> {
                let n_ctrl = cur.knots().control_count();
                let n_data = points.len();
                if n_ctrl > n_data || n_ctrl < 3 || n_data < 3 {
                    return Ok((cur, bound, Some(RefitSkip::StructureRicherThanData)));
                }
                let kv = cur.knots().clone();
                let weights = cur.weights().to_vec();
                let q0 = points[0];
                let qm = points[n_data - 1];
                let mut matrix = Vec::with_capacity(n_data - 2);
                let mut rhs = Vec::with_capacity(n_data - 2);
                for (u, q) in params[1..n_data - 1].iter().zip(points[1..].iter()) {
                    let (first, vals) = rational_row(&kv, &weights, *u);
                    let mut row = vec![0.0f64; n_ctrl - 2];
                    $(let mut $c = q.$c;)+
                    for (j, v) in vals.iter().enumerate() {
                        let i = first + j;
                        if i == 0 {
                            $($c -= v * q0.$c;)+
                        } else if i == n_ctrl - 1 {
                            $($c -= v * qm.$c;)+
                        } else {
                            row[i - 1] = *v;
                        }
                    }
                    matrix.push(row);
                    rhs.push(vec![$($c),+]);
                }
                // A DEGENERATE normal system skips the refit rather
                // than failing the fit: it is a structural statement
                // (some control point's basis support contains no
                // interior data site — C⁰ corners from full-mult
                // interior knots do this), the pipeline curve is
                // already certified, and the skip is deterministic
                // structure selection. Other solve refusals propagate.
                let sol = match lsq::solve_normal(&matrix, &rhs) {
                    Ok(sol) => sol,
                    Err(LsqError::LsqDegenerate { .. }) => {
                        return Ok((cur, bound, Some(RefitSkip::DegenerateSystem)));
                    }
                    Err(e) => return Err(FitError::Lsq(e)),
                };
                let mut control = cur.control().to_vec();
                for (i, row) in sol.iter().enumerate() {
                    let mut it = row.iter().copied();
                    $(let $c = it.next().unwrap_or(f64::NAN);)+
                    control[i + 1] = $Point::new($($c),+);
                }
                // Endpoints re-pinned to the exact end samples.
                control[0] = q0;
                control[n_ctrl - 1] = qm;
                let refit = Self::new(kv, control, weights).map_err(FitError::Structure)?;
                let refit_bound = refit.deviation_from(reference)?;
                // `!(≤)` is NaN-catching: a poisoned refit bound keeps
                // the certified pipeline curve.
                #[allow(clippy::neg_cmp_op_on_partial_ord)]
                if !(refit_bound <= tolerance) {
                    return Ok((cur, bound, Some(RefitSkip::BoundExceeded)));
                }
                Ok((refit, refit_bound, None))
            }
        }
    };
}

nurbs_fit!(NurbsCurve2, Point2, x, y);
nurbs_fit!(NurbsCurve3, Point3, x, y, z);

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    fn quarter_arc_samples(n: usize) -> Vec<Point3<f64>> {
        (0..n)
            .map(|k| {
                #[allow(clippy::cast_precision_loss)]
                let theta = core::f64::consts::FRAC_PI_2 * (k as f64) / ((n - 1) as f64);
                let (s, c) = theta.sin_cos();
                Point3::new(2.5 * c, 2.5 * s, 0.0)
            })
            .collect()
    }

    /// The typed budget refusal, exercised through the crate-internal
    /// budgeted entry (the public entry keeps the named constant):
    /// a one-attempt budget on removal-rich data must expire, carrying
    /// the achieved bound (zero — nothing was accepted yet).
    #[test]
    fn budget_exhaustion_is_typed_and_carries_the_achieved_bound() {
        let pts = quarter_arc_samples(16);
        match NurbsCurve3::approximate_budgeted(&pts, 3, 1e-3, 1) {
            Err(FitError::BudgetExhausted {
                budget: 1,
                achieved,
            }) => {
                assert_eq!(achieved, 0.0);
            }
            other => panic!("expected BudgetExhausted, got {other:?}"),
        }
    }

    #[test]
    fn typed_input_refusals() {
        let pts = quarter_arc_samples(3);
        assert!(matches!(
            NurbsCurve3::interpolate(&pts, 3),
            Err(FitError::TooFewPoints { have: 3, need: 4 })
        ));
        assert!(matches!(
            NurbsCurve3::interpolate(&pts, 0),
            Err(FitError::Structure(SplineError::KnotVectorInvalid { .. }))
        ));
        let mut dup = quarter_arc_samples(5);
        dup[2] = dup[1];
        assert!(matches!(
            NurbsCurve3::interpolate(&dup, 2),
            Err(FitError::DegenerateChord { index: 2 })
        ));
        let mut bad = quarter_arc_samples(5);
        bad[3] = Point3::new(f64::NAN, 0.0, 0.0);
        assert!(matches!(
            NurbsCurve3::interpolate(&bad, 2),
            Err(FitError::NonFinitePoint { index: 3 })
        ));
        assert!(matches!(
            NurbsCurve3::approximate(&quarter_arc_samples(8), 3, f64::NAN),
            Err(FitError::InvalidTolerance { .. })
        ));
        assert!(matches!(
            NurbsCurve3::approximate(&quarter_arc_samples(8), 3, -1.0),
            Err(FitError::InvalidTolerance { .. })
        ));
    }
}
