//! Knot-algebra **plans**: insertion, refinement, removal, and degree
//! elevation computed entirely as `f64` STRUCTURE (new knot vectors,
//! new weights, and affine combination schedules), applied to
//! generically-typed control points by a tiny fold
//! ([`CurvePlan::apply_points`]).
//!
//! # Why plans
//!
//! Every algorithm here (Book §5.2–§5.5) combines control points with
//! coefficients that depend only on knots and weights — both `f64`
//! structure (C6). Splitting each operation into a structure-computed
//! *plan* plus a generic *applier* keeps the C6 boundary a module
//! boundary: raw `f64` comparisons live in the plan constructors,
//! while the applier does nothing but `from_f64`-lifted two-point
//! affine combinations (`lerp`, fixed association `x + (y − x)·λ`) in
//! plan order. One plan implementation serves 2-D curves, 3-D curves,
//! and each row/column of a surface net.
//!
//! # Projective form (rational)
//!
//! All combinations are homogeneous: a two-term combo
//! `Q = a·(wᵢPᵢ, wᵢ) + b·(wⱼPⱼ, wⱼ)` becomes the structure weight
//! `w_Q = a·wᵢ + b·wⱼ` and the **affine point combination**
//! `P_Q = lerp(Pⱼ, Pᵢ, λ)` with `λ = a·wᵢ / w_Q` — exact in ℝ because
//! the two projective coefficients sum to 1 by construction. Weights
//! stay `f64` forever; only points are generic.

use super::knots::{KnotVector, SplineError};

/// A typed knot-algebra refusal (fail-loud; the kernel never panics).
#[derive(Clone, Debug, PartialEq)]
pub enum KnotAlgebraError {
    /// The inputs fail basic spline structure validation (weight
    /// count/positivity/finiteness against the knot vector).
    Structure(SplineError),
    /// Insertion parameter not strictly inside the knot domain (or
    /// not finite). Boundary insertion is meaningless for a clamped
    /// vector (end multiplicity is already `degree + 1`).
    ParameterOutsideDomain {
        /// The offending parameter.
        u: f64,
    },
    /// Inserting would push an interior multiplicity past `degree`.
    MultiplicityOverflow {
        /// The value whose multiplicity would overflow.
        u: f64,
        /// Its current multiplicity.
        have: usize,
        /// The interior budget (`degree`).
        budget: usize,
    },
    /// Removal of a value that is not an interior knot (exact `f64`
    /// equality — structure identity; end values are not removable
    /// from a clamped vector).
    KnotNotPresent {
        /// The requested value.
        u: f64,
    },
    /// Removing more copies than the knot's multiplicity.
    RemovalExceedsMultiplicity {
        /// The value being removed.
        u: f64,
        /// Its current multiplicity.
        have: usize,
        /// The requested removal count.
        requested: usize,
    },
    /// A removal chain produced a non-positive or non-finite weight —
    /// the candidate polygon leaves the positive-weight regime, so the
    /// removal is refused rather than returning an invalid curve.
    WeightCollapse {
        /// New-polygon index of the collapsed weight.
        index: usize,
    },
}

impl core::fmt::Display for KnotAlgebraError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            KnotAlgebraError::Structure(e) => write!(f, "knot algebra: {e}"),
            KnotAlgebraError::ParameterOutsideDomain { u } => {
                write!(
                    f,
                    "knot algebra: parameter {u} is not strictly inside the domain"
                )
            }
            KnotAlgebraError::MultiplicityOverflow { u, have, budget } => write!(
                f,
                "knot algebra: inserting {u} (multiplicity {have}) exceeds the interior budget {budget}"
            ),
            KnotAlgebraError::KnotNotPresent { u } => {
                write!(f, "knot algebra: {u} is not an interior knot")
            }
            KnotAlgebraError::RemovalExceedsMultiplicity { u, have, requested } => write!(
                f,
                "knot algebra: removing {u} {requested} times exceeds its multiplicity {have}"
            ),
            KnotAlgebraError::WeightCollapse { index } => write!(
                f,
                "knot algebra: removal collapsed weight {index} out of the positive regime"
            ),
        }
    }
}

impl core::error::Error for KnotAlgebraError {}

/// A source operand of a plan step: an index into the old polygon or
/// into the already-built portion of the new polygon.
#[derive(Clone, Copy, Debug)]
enum Src {
    Old(usize),
    New(usize),
}

/// One plan step: assign new-polygon slot `target`.
#[derive(Clone, Debug)]
enum Step {
    /// `new[target] = old[from]`.
    Carry { target: usize, from: usize },
    /// `new[target] = lerp(x, y, lambda)` — fixed association
    /// `x + (y − x)·λ` with the caller's lifted `λ`.
    Combo {
        target: usize,
        x: Src,
        y: Src,
        lambda: f64,
    },
}

/// One structure-computed polygon rewrite: the new knot vector, the
/// new weights, and the point combination schedule. Produced only by
/// this module's constructors (invariants: every `target` is assigned
/// exactly once, `New` sources refer to already-assigned slots).
#[derive(Clone, Debug)]
pub struct CurvePlan {
    knots: KnotVector,
    weights: Vec<f64>,
    steps: Vec<Step>,
}

impl CurvePlan {
    /// The knot vector after this plan.
    pub fn knots(&self) -> &KnotVector {
        &self.knots
    }

    /// The weights after this plan (all strictly positive — checked at
    /// plan construction).
    pub fn weights(&self) -> &[f64] {
        &self.weights
    }

    /// Applies the point schedule: `old` is the previous control
    /// polygon, `lerp(x, y, λ)` the caller's affine combination
    /// (`x + (y − x)·λ` with `λ` lifted via `from_f64` — the fixed
    /// association every consumer documents), and `poison` the
    /// caller's poison point — the total fallback for a malformed
    /// plan, which the constructors rule out but the applier does not
    /// trust (D4: fail loud, never panic).
    pub fn apply_points<P: Copy>(
        &self,
        old: &[P],
        poison: P,
        lerp: impl Fn(P, P, f64) -> P,
    ) -> Vec<P> {
        let n_new = self.knots.control_count();
        let mut new: Vec<Option<P>> = vec![None; n_new];
        let fetch = |new: &[Option<P>], s: Src| -> Option<P> {
            match s {
                Src::Old(i) => old.get(i).copied(),
                Src::New(i) => new.get(i).copied().flatten(),
            }
        };
        for step in &self.steps {
            match *step {
                Step::Carry { target, from } => {
                    if target < n_new {
                        new[target] = old.get(from).copied();
                    }
                }
                Step::Combo {
                    target,
                    x,
                    y,
                    lambda,
                } => {
                    if target < n_new {
                        let combined = match (fetch(&new, x), fetch(&new, y)) {
                            (Some(px), Some(py)) => Some(lerp(px, py, lambda)),
                            _ => None,
                        };
                        new[target] = combined;
                    }
                }
            }
        }
        new.into_iter().map(|slot| slot.unwrap_or(poison)).collect()
    }
}

/// Validates weights against a knot vector: count, positivity,
/// finiteness (the shared precondition of every plan constructor).
// `!(x > 0)`-shaped guards below are deliberate: the negated form is
// NaN-catching (`NaN > 0` is false, so NaN refuses), where `x <= 0`
// would silently pass NaN through — the fail-loud direction.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
fn check_weights(kv: &KnotVector, weights: &[f64]) -> Result<(), KnotAlgebraError> {
    if weights.len() != kv.control_count() {
        return Err(KnotAlgebraError::Structure(
            SplineError::WeightCountMismatch {
                weights: weights.len(),
                control: kv.control_count(),
            },
        ));
    }
    for (index, w) in weights.iter().enumerate() {
        if !(*w > 0.0) {
            return Err(KnotAlgebraError::Structure(
                SplineError::NonPositiveWeight { index, weight: *w },
            ));
        }
        if !w.is_finite() {
            return Err(KnotAlgebraError::Structure(SplineError::NonFiniteWeight {
                index,
                weight: *w,
            }));
        }
    }
    Ok(())
}

/// Single knot insertion (Book §5.2, Boehm), `times`-fold: returns a
/// chain of plans, each built on the previous plan's structure. The
/// resulting multiplicity must stay within the interior budget
/// (`degree`); the parameter must be strictly inside the domain.
///
/// # Errors
///
/// [`KnotAlgebraError`] on structure mismatch, out-of-domain `u`, or
/// multiplicity overflow. `times == 0` is a no-op (empty chain).
// NaN-catching negated comparisons — see `check_weights`' note.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
pub fn insert_knot_plan(
    kv: &KnotVector,
    weights: &[f64],
    u: f64,
    times: usize,
) -> Result<Vec<CurvePlan>, KnotAlgebraError> {
    check_weights(kv, weights)?;
    let (lo, hi) = kv.domain();
    if !u.is_finite() || !(u > lo) || !(u < hi) {
        return Err(KnotAlgebraError::ParameterOutsideDomain { u });
    }
    let have = kv.multiplicity_of(u).map_or(0, |(s, _)| s);
    if have + times > kv.degree() {
        return Err(KnotAlgebraError::MultiplicityOverflow {
            u,
            have,
            budget: kv.degree(),
        });
    }
    let mut plans = Vec::with_capacity(times);
    let mut cur_kv = kv.clone();
    let mut cur_w = weights.to_vec();
    for _ in 0..times {
        let plan = insert_once(&cur_kv, &cur_w, u);
        cur_kv = plan.knots.clone();
        cur_w = plan.weights.clone();
        plans.push(plan);
    }
    Ok(plans)
}

/// One insertion pass — preconditions established by the callers
/// (`u` strictly interior, multiplicity budget available, weights
/// validated).
///
/// **The Boehm structure is shared with
/// [`super::compose`]'s `insert_once_ring`; the coefficient arithmetic
/// is deliberately not** — see that function's docs for why. In short:
/// this one exists to emit a replayable [`CurvePlan`] over `f64`
/// weights; that one folds `RingInterval` coefficients with an
/// outward-rounding quotient and has no weights to form `λ` from.
fn insert_once(kv: &KnotVector, weights: &[f64], u: f64) -> CurvePlan {
    let p = kv.degree();
    let knots = kv.knots();
    let k = kv.find_span(u);
    let s = kv.multiplicity_of(u).map_or(0, |(s, _)| s);
    let n_old = kv.control_count();
    let n_new = n_old + 1;

    let mut new_knots = Vec::with_capacity(knots.len() + 1);
    new_knots.extend_from_slice(&knots[..=k]);
    new_knots.push(u);
    new_knots.extend_from_slice(&knots[k + 1..]);
    let new_kv = KnotVector::from_algebra(new_knots, p);

    let mut new_w = vec![0.0f64; n_new];
    let mut steps = Vec::with_capacity(n_new);
    // Prefix carries: Q_j = A_j for j ≤ k − p.
    for j in 0..=(k - p) {
        steps.push(Step::Carry { target: j, from: j });
        new_w[j] = weights[j];
    }
    // The combined band: Q_j = α_j A_j + (1−α_j) A_{j−1},
    // α_j = (u − U[j]) / (U[j+p] − U[j]) ∈ (0, 1) — denominator > 0
    // because U[j] < u (j ≤ k − s, below the copy run) and
    // U[j+p] ≥ U[k+1] > u (nonempty span k). Projective form: module
    // docs.
    for j in (k - p + 1)..=(k - s) {
        let alpha = (u - knots[j]) / (knots[j + p] - knots[j]);
        let wq = alpha * weights[j] + (1.0 - alpha) * weights[j - 1];
        let lambda = alpha * weights[j] / wq;
        new_w[j] = wq;
        steps.push(Step::Combo {
            target: j,
            x: Src::Old(j - 1),
            y: Src::Old(j),
            lambda,
        });
    }
    // Suffix carries: Q_j = A_{j−1} for j ≥ k − s + 1.
    for j in (k - s + 1)..n_new {
        steps.push(Step::Carry {
            target: j,
            from: j - 1,
        });
        new_w[j] = weights[j - 1];
    }
    CurvePlan {
        knots: new_kv,
        weights: new_w,
        steps,
    }
}

/// Knot refinement (Book §5.3) as a fold of single insertions in
/// ascending parameter order (ties inserted consecutively) — a
/// deliberately simple, deterministic composition of §5.2 rather than
/// the one-pass A5.4 (documented implementation choice; the
/// evaluation-invariance obligations are identical).
///
/// # Errors
///
/// As [`insert_knot_plan`], evaluated against the *cumulative*
/// structure (earlier insertions count toward multiplicity budgets).
pub fn refine_plan(
    kv: &KnotVector,
    weights: &[f64],
    new_knots: &[f64],
) -> Result<Vec<CurvePlan>, KnotAlgebraError> {
    check_weights(kv, weights)?;
    let mut sorted = new_knots.to_vec();
    // Structure sort: refuse NaN up front, then total order is the
    // plain f64 order.
    for u in &sorted {
        if !u.is_finite() {
            return Err(KnotAlgebraError::ParameterOutsideDomain { u: *u });
        }
    }
    sorted.sort_by(f64::total_cmp);
    let mut plans = Vec::with_capacity(sorted.len());
    let mut cur_kv = kv.clone();
    let mut cur_w = weights.to_vec();
    for u in sorted {
        let mut chain = insert_knot_plan(&cur_kv, &cur_w, u, 1)?;
        // insert_knot_plan(times = 1) returns exactly one plan.
        if let Some(plan) = chain.pop() {
            cur_kv = plan.knots.clone();
            cur_w = plan.weights.clone();
            plans.push(plan);
        }
    }
    Ok(plans)
}

/// One bounded-removal pass: the removal plan plus the **reinsertion**
/// plan that puts the removed copy back on the *new* structure. The
/// reinsertion is exact in ℝ, so applying `plan` then `reinsert` yields
/// a polygon on the ORIGINAL knot vector whose pointwise difference
/// from the original polygon bounds `|C − Ĉ|` by partition of unity —
/// the Eq. 9.81 mechanism; the projected bound itself is computed by
/// the curve/surface layer (it needs control-point norms at `T`).
#[derive(Clone, Debug)]
pub struct RemovalStep {
    /// The removal rewrite (one multiplicity step down).
    pub plan: CurvePlan,
    /// The exact reinsertion of the removed copy, built on
    /// `plan`'s structure.
    pub reinsert: CurvePlan,
}

/// Bounded knot removal (Book §5.4), `times`-fold: each pass removes
/// one copy of `u` and pairs the rewrite with its reinsertion plan for
/// the caller's error-bound computation ([`RemovalStep`]). Removal is
/// **total on the arithmetic and bounded, never silent**: no
/// removability tolerance test happens here — the caller receives the
/// rewritten polygon and the data to bound its deviation, and decides.
///
/// The chain solves the reinsertion equations
/// `A_i = α_i·Â_i + (1−α_i)·Â_{i−1}` (i = r−p ..= r−s,
/// `α_i = (u − U[i])/(U[i+p+1] − U[i]) ∈ (0,1)`) forward for the first
/// `⌈(p−s)/2⌉` unknowns and backward for the rest — the one leftover
/// equation's residual is exactly what the returned bound captures.
///
/// # Errors
///
/// [`KnotAlgebraError::KnotNotPresent`] if `u` is not an interior knot
/// (exact `f64` identity), [`KnotAlgebraError::RemovalExceedsMultiplicity`],
/// [`KnotAlgebraError::WeightCollapse`] if a pass leaves the
/// positive-weight regime, plus the shared structure refusals.
pub fn remove_knot_plan(
    kv: &KnotVector,
    weights: &[f64],
    u: f64,
    times: usize,
) -> Result<Vec<RemovalStep>, KnotAlgebraError> {
    check_weights(kv, weights)?;
    let (lo, hi) = kv.domain();
    if u == lo || u == hi {
        return Err(KnotAlgebraError::KnotNotPresent { u });
    }
    let have = kv.multiplicity_of(u).map_or(0, |(s, _)| s);
    if have == 0 {
        return Err(KnotAlgebraError::KnotNotPresent { u });
    }
    if times > have {
        return Err(KnotAlgebraError::RemovalExceedsMultiplicity {
            u,
            have,
            requested: times,
        });
    }
    let mut steps = Vec::with_capacity(times);
    let mut cur_kv = kv.clone();
    let mut cur_w = weights.to_vec();
    for _ in 0..times {
        let plan = remove_once(&cur_kv, &cur_w, u)?;
        let mut re = insert_knot_plan(&plan.knots, &plan.weights, u, 1)?;
        cur_kv = plan.knots.clone();
        cur_w = plan.weights.clone();
        // insert_knot_plan(times = 1) returns exactly one plan.
        if let Some(reinsert) = re.pop() {
            steps.push(RemovalStep { plan, reinsert });
        }
    }
    Ok(steps)
}

/// One removal pass (fn docs on [`remove_knot_plan`] for the chain
/// derivation; preconditions established there).
// NaN-catching negated comparisons — see `check_weights`' note.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
fn remove_once(kv: &KnotVector, weights: &[f64], u: f64) -> Result<CurvePlan, KnotAlgebraError> {
    let p = kv.degree();
    let knots = kv.knots();
    // Present with multiplicity s, last copy at index r (caller
    // checked presence; map_or keeps this total anyway).
    let (s, r) = kv.multiplicity_of(u).map_or((1, p + 1), |sr| sr);
    let first = r - p;
    let last = r - s;
    let n_old = kv.control_count();
    let n_new = n_old - 1;

    let mut new_knots = Vec::with_capacity(knots.len() - 1);
    new_knots.extend_from_slice(&knots[..r]);
    new_knots.extend_from_slice(&knots[r + 1..]);
    let new_kv = KnotVector::from_algebra(new_knots, p);

    // α_i ∈ (0,1): U[i] < u for i ≤ r−s (below the copy run) and
    // U[i+p+1] ≥ U[r+1] > u for i ≥ r−p (above it) — validated
    // structure, so both divisions below are safe.
    let alpha = |i: usize| (u - knots[i]) / (knots[i + p + 1] - knots[i]);

    let mut new_w = vec![0.0f64; n_new];
    let mut steps = Vec::with_capacity(n_new);
    for j in 0..first {
        steps.push(Step::Carry { target: j, from: j });
        new_w[j] = weights[j];
    }
    let n_unknown = last - first; // = p − s
    let nf = n_unknown.div_ceil(2);
    // Forward chain: Â_i = (A_i − (1−α_i)·Â_{i−1}) / α_i.
    for i in first..first + nf {
        let a = alpha(i);
        let (prev_src, prev_w) = if i == first {
            (Src::Old(first - 1), weights[first - 1])
        } else {
            (Src::New(i - 1), new_w[i - 1])
        };
        let wq = (weights[i] - (1.0 - a) * prev_w) / a;
        if !(wq > 0.0) || !wq.is_finite() {
            return Err(KnotAlgebraError::WeightCollapse { index: i });
        }
        // λ overflow note: wq passed the guard, but a subnormal-
        // positive wq can still overflow `…/wq` to +∞ here; the lifted
        // infinite λ then poisons the combined point (NaN through
        // `x + (y − x)·λ`) rather than raising a typed error. Accepted:
        // validated inputs (finite weights ≥ DBL_MIN-scale, α ∈ (0,1))
        // cannot reach a subnormal wq without first tripping the
        // WeightCollapse guard in a preceding pass, and poison fails
        // certification loudly downstream (D4 ¶2) if they somehow do.
        let lambda = (weights[i] / a) / wq;
        steps.push(Step::Combo {
            target: i,
            x: prev_src,
            y: Src::Old(i),
            lambda,
        });
        new_w[i] = wq;
    }
    // Backward chain: Â_i = (A_{i+1} − α_{i+1}·Â_{i+1}) / (1−α_{i+1}).
    for i in (first + nf..last).rev() {
        let a = alpha(i + 1);
        let (next_src, next_w) = if i == last - 1 {
            (Src::Old(last + 1), weights[last + 1])
        } else {
            (Src::New(i + 1), new_w[i + 1])
        };
        let wq = (weights[i + 1] - a * next_w) / (1.0 - a);
        if !(wq > 0.0) || !wq.is_finite() {
            return Err(KnotAlgebraError::WeightCollapse { index: i });
        }
        // λ overflow note: as in the forward chain above.
        let lambda = (weights[i + 1] / (1.0 - a)) / wq;
        steps.push(Step::Combo {
            target: i,
            x: next_src,
            y: Src::Old(i + 1),
            lambda,
        });
        new_w[i] = wq;
    }
    // Suffix carries (the dropped slot is `last`; everything above
    // shifts down by one).
    for j in last..n_new {
        steps.push(Step::Carry {
            target: j,
            from: j + 1,
        });
        new_w[j] = weights[j + 1];
    }
    Ok(CurvePlan {
        knots: new_kv,
        weights: new_w,
        steps,
    })
}

/// Degree elevation by one (Book §5.5), via the Bézier route:
/// (1) refine every distinct interior knot to multiplicity `p`,
/// (2) elevate each Bézier segment with the binomial combination
/// `Q_i = c_i·A_{i−1} + (1−c_i)·A_i`, `c_i = i/(p+1)` (homogeneous;
/// projective form per the module docs), (3) remove each interior
/// breakpoint back down to its original multiplicity plus one. The
/// recomposition removals are **exact in ℝ** (the elevated curve has
/// full continuity there), so no bound is surfaced; the
/// evaluation-invariance tests pin the floating-point agreement.
///
/// # Errors
///
/// The shared structure refusals; a [`KnotAlgebraError::WeightCollapse`]
/// from step (3) is possible only through floating-point degeneracy and
/// is surfaced honestly rather than clamped.
pub fn elevate_plan(kv: &KnotVector, weights: &[f64]) -> Result<Vec<CurvePlan>, KnotAlgebraError> {
    check_weights(kv, weights)?;
    let p = kv.degree();
    // Collected, not iterated: `cur_kv` below is rebuilt inside the
    // loop, so the list must outlive the borrow of `kv`.
    let interior: Vec<(f64, usize)> = kv.interior_knots().collect();

    let mut plans: Vec<CurvePlan> = Vec::new();
    let mut cur_kv = kv.clone();
    let mut cur_w = weights.to_vec();
    // (1) Bézier decomposition.
    for (v, m) in &interior {
        if *m < p {
            for plan in insert_knot_plan(&cur_kv, &cur_w, *v, p - m)? {
                cur_kv = plan.knots.clone();
                cur_w = plan.weights.clone();
                plans.push(plan);
            }
        }
    }
    // (2) Per-segment elevation.
    let stage = elevate_bezier_stage(&cur_kv, &cur_w);
    cur_kv = stage.knots.clone();
    cur_w = stage.weights.clone();
    plans.push(stage);
    // (3) Recomposition: interior breakpoints from multiplicity p+1
    // down to original + 1.
    for (v, m) in &interior {
        let excess = p - m;
        if excess > 0 {
            for step in remove_knot_plan(&cur_kv, &cur_w, *v, excess)? {
                cur_kv = step.plan.knots.clone();
                cur_w = step.plan.weights.clone();
                plans.push(step.plan);
            }
        }
    }
    Ok(plans)
}

/// The Bézier-segment elevation stage: every interior knot is at
/// multiplicity `p` (established by [`elevate_plan`] step 1).
fn elevate_bezier_stage(kv: &KnotVector, weights: &[f64]) -> CurvePlan {
    let p = kv.degree();
    let knots = kv.knots();
    // Distinct values in order (ends included).
    let mut values: Vec<f64> = Vec::new();
    for k in knots {
        if values.last() != Some(k) {
            values.push(*k);
        }
    }
    let nseg = values.len() - 1;
    let n_new = nseg * (p + 2) - (nseg - 1);

    let mut new_knots = Vec::with_capacity(knots.len() + values.len());
    for (vi, v) in values.iter().enumerate() {
        let mult = if vi == 0 || vi == nseg { p + 2 } else { p + 1 };
        for _ in 0..mult {
            new_knots.push(*v);
        }
    }
    let new_kv = KnotVector::from_algebra(new_knots, p + 1);

    let mut new_w = vec![0.0f64; n_new];
    let mut steps = Vec::with_capacity(n_new);
    steps.push(Step::Carry { target: 0, from: 0 });
    new_w[0] = weights[0];
    for seg in 0..nseg {
        let o = seg * p; // old segment offset (Bézier points o ..= o+p)
        let o2 = seg * (p + 1); // new segment offset
        for i in 1..=p {
            let c = i as f64 / (p + 1) as f64;
            let wq = c * weights[o + i - 1] + (1.0 - c) * weights[o + i];
            let lambda = c * weights[o + i - 1] / wq;
            steps.push(Step::Combo {
                target: o2 + i,
                x: Src::Old(o + i),
                y: Src::Old(o + i - 1),
                lambda,
            });
            new_w[o2 + i] = wq;
        }
        steps.push(Step::Carry {
            target: o2 + p + 1,
            from: o + p,
        });
        new_w[o2 + p + 1] = weights[o + p];
    }
    CurvePlan {
        knots: new_kv,
        weights: new_w,
        steps,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::spline::basis::basis_funs;

    /// 1-D rational evaluation oracle: x(t) = Σ N w x / Σ N w — the
    /// plan machinery is dimension-agnostic, so scalar control points
    /// are a complete test bed.
    fn eval1(kv: &KnotVector, w: &[f64], x: &[f64], t: f64) -> f64 {
        let span = kv.span_at(t);
        let n = basis_funs(kv, span, t);
        let (mut num, mut den) = (0.0, 0.0);
        for (j, nj) in n.iter().enumerate() {
            let i = span.first_control() + j;
            num += nj * w[i] * x[i];
            den += nj * w[i];
        }
        num / den
    }

    fn lerp1(x: f64, y: f64, l: f64) -> f64 {
        x + (y - x) * l
    }

    fn fixture() -> (KnotVector, Vec<f64>, Vec<f64>) {
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0], 2).unwrap();
        let w = vec![1.0, 2.0, 0.5, 1.5, 1.0];
        let x = vec![0.0, 1.0, 4.0, 2.0, -1.0];
        (kv, w, x)
    }

    fn apply_chain(plans: &[CurvePlan], x: &[f64]) -> Vec<f64> {
        let mut cur = x.to_vec();
        for plan in plans {
            cur = plan.apply_points(&cur, f64::NAN, lerp1);
        }
        cur
    }

    #[test]
    fn insertion_is_evaluation_invariant() {
        let (kv, w, x) = fixture();
        for (u, times) in [(0.5, 1), (1.0, 1), (2.5, 2), (1.5, 2)] {
            let plans = insert_knot_plan(&kv, &w, u, times).unwrap();
            let x2 = apply_chain(&plans, &x);
            let last = plans.last().unwrap();
            assert_eq!(x2.len(), kv.control_count() + times);
            for i in 0..=60 {
                let t = 3.0 * f64::from(i) / 60.0;
                let a = eval1(&kv, &w, &x, t);
                let b = eval1(last.knots(), last.weights(), &x2, t);
                // Tight-but-not-bitwise: floating point moves under
                // re-association; the values agree to ~1e-13 of the
                // O(1) coordinate scale here.
                assert!(
                    (a - b).abs() < 1e-12,
                    "u={u} times={times} t={t}: {a} vs {b}"
                );
            }
        }
    }

    #[test]
    fn insertion_refusals_are_typed() {
        let (kv, w, x) = fixture();
        let _ = x;
        assert_eq!(
            insert_knot_plan(&kv, &w, 3.5, 1).unwrap_err(),
            KnotAlgebraError::ParameterOutsideDomain { u: 3.5 }
        );
        assert_eq!(
            insert_knot_plan(&kv, &w, 0.0, 1).unwrap_err(),
            KnotAlgebraError::ParameterOutsideDomain { u: 0.0 }
        );
        assert_eq!(
            insert_knot_plan(&kv, &w, 1.0, 2).unwrap_err(),
            KnotAlgebraError::MultiplicityOverflow {
                u: 1.0,
                have: 1,
                budget: 2
            }
        );
        assert_eq!(
            insert_knot_plan(&kv, &[1.0, 1.0], 0.5, 1).unwrap_err(),
            KnotAlgebraError::Structure(SplineError::WeightCountMismatch {
                weights: 2,
                control: 5
            })
        );
        assert_eq!(
            insert_knot_plan(&kv, &[1.0, -1.0, 1.0, 1.0, 1.0], 0.5, 1).unwrap_err(),
            KnotAlgebraError::Structure(SplineError::NonPositiveWeight {
                index: 1,
                weight: -1.0
            })
        );
    }

    #[test]
    fn refinement_is_evaluation_invariant_and_sorts() {
        let (kv, w, x) = fixture();
        let plans = refine_plan(&kv, &w, &[2.5, 0.5, 1.5, 0.5]).unwrap();
        let x2 = apply_chain(&plans, &x);
        let last = plans.last().unwrap();
        assert_eq!(last.knots().knots().len(), kv.knots().len() + 4);
        for i in 0..=60 {
            let t = 3.0 * f64::from(i) / 60.0;
            let a = eval1(&kv, &w, &x, t);
            let b = eval1(last.knots(), last.weights(), &x2, t);
            assert!((a - b).abs() < 1e-12, "t={t}: {a} vs {b}");
        }
    }

    #[test]
    fn insert_then_remove_round_trips_evaluation() {
        let (kv, w, x) = fixture();
        let ins = insert_knot_plan(&kv, &w, 1.4, 2).unwrap();
        let xi = apply_chain(&ins, &x);
        let last_ins = ins.last().unwrap();
        let rem = remove_knot_plan(last_ins.knots(), last_ins.weights(), 1.4, 2).unwrap();
        let mut xr = xi;
        let mut final_kv = last_ins.knots().clone();
        let mut final_w = last_ins.weights().to_vec();
        for step in &rem {
            xr = step.plan.apply_points(&xr, f64::NAN, lerp1);
            final_kv = step.plan.knots().clone();
            final_w = step.plan.weights().to_vec();
        }
        assert_eq!(final_kv.knots(), kv.knots());
        for i in 0..=60 {
            let t = 3.0 * f64::from(i) / 60.0;
            let a = eval1(&kv, &w, &x, t);
            let b = eval1(&final_kv, &final_w, &xr, t);
            assert!((a - b).abs() < 1e-10, "t={t}: {a} vs {b}");
        }
    }

    #[test]
    fn removal_refusals_are_typed() {
        let (kv, w, _x) = fixture();
        assert_eq!(
            remove_knot_plan(&kv, &w, 0.25, 1).unwrap_err(),
            KnotAlgebraError::KnotNotPresent { u: 0.25 }
        );
        assert_eq!(
            remove_knot_plan(&kv, &w, 0.0, 1).unwrap_err(),
            KnotAlgebraError::KnotNotPresent { u: 0.0 }
        );
        assert_eq!(
            remove_knot_plan(&kv, &w, 1.0, 2).unwrap_err(),
            KnotAlgebraError::RemovalExceedsMultiplicity {
                u: 1.0,
                have: 1,
                requested: 2
            }
        );
    }

    #[test]
    fn elevation_is_evaluation_invariant() {
        let (kv, w, x) = fixture();
        let plans = elevate_plan(&kv, &w).unwrap();
        let x2 = apply_chain(&plans, &x);
        let last = plans.last().unwrap();
        assert_eq!(last.knots().degree(), kv.degree() + 1);
        // Elevation adds one copy of every distinct value: interior
        // count 2 (values 1, 2) + both ends ⇒ +4 knots, +3 control.
        assert_eq!(last.knots().knots().len(), kv.knots().len() + 4);
        assert_eq!(x2.len(), x.len() + 3);
        for i in 0..=90 {
            let t = 3.0 * f64::from(i) / 90.0;
            let a = eval1(&kv, &w, &x, t);
            let b = eval1(last.knots(), last.weights(), &x2, t);
            assert!((a - b).abs() < 1e-10, "t={t}: {a} vs {b}");
        }
    }

    #[test]
    fn determinism_bitwise_across_repeats() {
        let (kv, w, x) = fixture();
        let p1 = insert_knot_plan(&kv, &w, 1.7, 2).unwrap();
        let p2 = insert_knot_plan(&kv, &w, 1.7, 2).unwrap();
        let x1 = apply_chain(&p1, &x);
        let x2 = apply_chain(&p2, &x);
        let bits = |v: &[f64]| v.iter().map(|f| f.to_bits()).collect::<Vec<_>>();
        assert_eq!(bits(&x1), bits(&x2));
        assert_eq!(
            bits(p1.last().unwrap().weights()),
            bits(p2.last().unwrap().weights())
        );
        assert_eq!(p1.last().unwrap().knots(), p2.last().unwrap().knots());
    }
}
