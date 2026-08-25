//! **The two meters the offset fit needs** — the regularity floor and
//! the collapse headroom (`docs/OFFSET-DESIGN.md` O3).
//!
//! Both are read off [`crate::patch_bound`]'s per-cell enclosures, and
//! both are **f64-substrate**: the C9 ring produces an `f64` certified
//! enclosure, which is what a hull bound IS, so the numbers below are
//! the same on every lane (the `SsiCertificate::hull_sup` posture).
//!
//! # Meter 1 — the regularity floor (the tree's first inf-side
//! surface bound)
//!
//! The offset `S + d·n` is **undefined** where the chart normal
//! degenerates, so the fit door must refuse — never degrade — on a
//! patch whose `‖S_u × S_v‖` cannot be bounded away from zero. Every
//! surface bound the kernel had until now was sup-side
//! ([`geom_core::spline::hull::sup_norm_bound`] and its family); the
//! curve side's inf meter (`NurbsCurve3::speed_lower_bound`) is
//! one dimension down and does not lift directly, because the
//! quantity here is a **product** of two coefficient nets, not one.
//!
//! ## The derivation
//!
//! On a cell, `S_u` and `S_v` each lie componentwise in the signed
//! hull of their active derived coefficients ([`crate::patch_bound`]).
//! Interval-multiplying those three-component enclosures gives an
//! enclosure `M ⊇ { S_u(u,v) × S_v(u,v) : (u,v) ∈ cell }`. Two sound
//! lower bounds on `‖m‖` are read off `M`, and the larger wins — the
//! speed meter's two-assembly join, lifted:
//!
//! - **Componentwise mignitude**: `‖m‖ = √(Σ_c m_c²) ≥ √(Σ_c mig(M_c)²)`,
//!   where `mig(I)` is the smallest `|x|` for `x ∈ I` (zero when `I`
//!   straddles). Tight when no component's enclosure straddles zero;
//!   collapses to zero when they all do.
//! - **Fixed-direction projection**: for ANY unit `d̂`,
//!   `‖m‖ ≥ d̂·m ≥ lo(Σ_c d̂_c·M_c)`. The direction is the normalized
//!   midpoint of `M` — structure, chosen deterministically, and the
//!   bound is sound for whatever direction it picks (`d̂` is unit only
//!   to rounding, so the projection is divided by a certified upper
//!   bound on `‖d̂‖`). This is the assembly that survives a cell where
//!   every component straddles but the vector does not.
//!
//! ## Conservatism direction, stated
//!
//! `floor ≤ inf_cell ‖S_u × S_v‖`, always: the interval product is an
//! enclosure of a superset, both assemblies bound from below, and the
//! ring rounds outward. So the meter can **refuse a regular patch it
//! failed to certify; it can never accept a degenerate one**. It
//! collapses to exactly zero — the loud answer — as soon as a cell is
//! coarse enough that the normal turns through a right angle inside
//! it, which is why the rational arm's fixed refinement
//! ([`crate::patch_bound::RATIONAL_CERT_SPLITS`]) matters here more
//! than it does for a sup bound.
//!
//! ## The shared shape (#528's chart-region stretch)
//!
//! #528 wants a certified **inf-side** bound on a different surface
//! quantity — the chart's metric stretch, `inf ‖S_u‖` and
//! `inf ‖S_v‖` over a region — for the same structural reason: a
//! sup-side bound cannot tell a consumer that a chart does not fold.
//! The shape both need is: *signed componentwise coefficient
//! enclosures over a cell → a fixed-direction projection whose lower
//! endpoint is the bound, joined with a componentwise-mignitude
//! assembly*. [`cell_normal`] is that shape at the cross product;
//! #528's consumer would instantiate it at the two speeds (which
//! [`PatchRegularity::speed_u`] already carries the sup-side twin of).
//! Naming it is the whole obligation here — #528's consumer is not
//! built, and building it without a caller would guess its region
//! vocabulary.
//!
//! ## The margin and its lever
//!
//! The certified quantity is `floor`, in **m² per unit parameter
//! area** — the natural units of `‖S_u × S_v‖`, and the number the
//! certificate actually consumes (it is what makes `1/‖S_u × S_v‖`
//! boundable). The *predicate* cannot classify an area rate against a
//! linear band, so [`offset_normal_floor`] meters the dimensionless
//! **sine floor** `floor / (sup‖S_u‖ · sup‖S_v‖) ≤ sin∠(S_u, S_v)`
//! levered by `|d|`: the lever is the offset distance itself, because
//! the displacement a normal ambiguity of angle `θ` inflicts on the
//! offset point is exactly `|d|·θ`. Sup-side denominators push the
//! sine floor DOWN, so the levered margin is conservative in the same
//! direction as the floor.
//!
//! # Meter 2 — the collapse headroom
//!
//! The offset of a surface with principal curvature `κ` (in the
//! chart-normal sign convention: `κ = II(t,t)/I(t,t)` with
//! `II = n·S_dd`, so an outward-normalled sphere of radius `r` has
//! `κ = −1/r`) folds exactly where `1 − d·κ = 0`. The meter certifies
//! `[κ_lo, κ_hi]` over the patch and refuses when `|d|` reaches the
//! critical distance on the folding side.
//!
//! `κ` is bounded through the two fundamental forms, both read off the
//! same cell enclosures:
//!
//! ```text
//! II(a,b) = L a² + 2M ab + N b²   L = n·S_uu,  M = n·S_uv,  N = n·S_vv
//! I(a,b)  = E a² + 2F ab + G b²   E = S_u·S_u, F = S_u·S_v, G = S_v·S_v
//! λ_min(I) ≥ det(I)/tr(I) = ‖S_u × S_v‖² / (E + G) ≥ floor²/(E+G)
//! ```
//!
//! so over `a² + b² = 1`, `min(lo L, lo N) − |M| ≤ II ≤
//! max(hi L, hi N) + |M|` and `κ = II/I` is bounded by dividing by
//! `λ_min` (for the same-signed side) or by `tr(I)` (for the
//! opposite-signed side, where the larger denominator is the
//! conservative one). **This is the one place the two meters compose**:
//! the normal `n` in `II` is the normalized normal, enclosed as
//! `M_c / [floor, sup]` — meter 1's floor is what makes it exist.
//!
//! This is the fillet battery's radius-headroom shape one dimension
//! up: there, a blend radius against a spine's curvature; here, an
//! offset distance against a patch's.

use geom_core::ring_interval::RingInterval;
use geom_core::{Band, Indeterminate, Margin, Sign};

use crate::dihedral::decide;
use crate::patch_bound::PatchCell;

/// A typed refusal of one of the two offset meters (D4 ¶3). Scalar
/// payloads echo the classified margin's ingredients — data, not a
/// decision.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MeterError {
    /// `offset_normal_floor`: the patch's chart normal could not be
    /// bounded away from degeneracy, so the offset locus is not
    /// defined on it (or the bound is too weak to prove that it is).
    NormalFloor {
        /// The certified lower bound on `‖S_u × S_v‖` (m² per unit
        /// parameter area) — zero when no cell could be certified.
        floor: f64,
        /// The dimensionless sine floor the margin levered.
        sine_floor: f64,
        /// The lever the margin used, in metres (`|d|`).
        lever: f64,
    },
    /// `offset_curvature_headroom`: `|d|` reaches the patch's
    /// smallest certified curvature radius on the folding side, so
    /// the offset self-intersects (or the bound is too weak to prove
    /// that it does not).
    CurvatureHeadroom {
        /// The certified critical distance on the folding side, in
        /// metres (`+∞` when the patch does not curve that way).
        reach: f64,
        /// The classified margin `reach − |d|`, in metres.
        headroom: f64,
        /// The certified principal-curvature range, in 1/m.
        kappa: (f64, f64),
    },
    /// A meter escalated: the margin landed in the ambiguity band or
    /// was poisoned (escalate-never-guess, D4 ¶3).
    Escalated {
        /// The predicate-layer escalation.
        source: Indeterminate,
    },
}

impl core::fmt::Display for MeterError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NormalFloor {
                floor,
                sine_floor,
                lever,
            } => write!(
                f,
                "offset_normal_floor: the patch's chart normal is not certifiably \
                 non-degenerate — the certified floor on ‖S_u × S_v‖ is {floor} \
                 (sine floor {sine_floor}, levered by |d| = {lever} m); the offset \
                 locus is undefined where the normal degenerates, so nothing is fitted"
            ),
            Self::CurvatureHeadroom {
                reach,
                headroom,
                kappa,
            } => write!(
                f,
                "offset_curvature_headroom: |d| reaches the patch's certified \
                 curvature radius on the folding side (reach {reach} m, headroom \
                 {headroom} m, principal curvature in [{}, {}] 1/m) — the offset \
                 folds, so nothing is fitted",
                kappa.0, kappa.1
            ),
            Self::Escalated { source } => write!(f, "offset meter escalated: {source}"),
        }
    }
}

impl core::error::Error for MeterError {}

/// The smallest `|x|` over the enclosure — zero when it straddles (or
/// is poisoned, whose comparisons are all false: the conservative
/// answer).
fn mig(i: RingInterval) -> f64 {
    if i.lo() > 0.0 {
        i.lo()
    } else if i.hi() < 0.0 {
        -i.hi()
    } else {
        0.0
    }
}

/// `√x` rounded DOWN (a lower bound), zero for a non-positive or
/// non-finite argument.
fn sqrt_down(x: f64) -> f64 {
    if x > 0.0 { x.sqrt().next_down() } else { 0.0 }
}

/// `√x` rounded UP (an upper bound); NaN flows.
fn sqrt_up(x: f64) -> f64 {
    if x > 0.0 { x.sqrt().next_up() } else { x }
}

/// Interval dot product, fixed ascending order (D9).
fn dot(a: &[RingInterval; 3], b: &[RingInterval; 3]) -> RingInterval {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// Interval cross product, fixed component order (D9).
fn cross(a: &[RingInterval; 3], b: &[RingInterval; 3]) -> [RingInterval; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// A certified upper bound on `‖v‖` for a componentwise enclosure.
fn norm_sup(v: &[RingInterval; 3]) -> f64 {
    let s = v[0].sqr() + v[1].sqr() + v[2].sqr();
    sqrt_up(s.hi())
}

/// One cell's chart-normal facts: the enclosure of `S_u × S_v` and
/// certified bounds on its magnitude (module docs, meter 1).
#[derive(Clone, Copy, Debug)]
pub struct CellNormal {
    /// Componentwise enclosure of `m = S_u × S_v` on the cell.
    pub m: [RingInterval; 3],
    /// Certified LOWER bound on `‖m‖` over the cell (m²) — the
    /// regularity floor. Exactly `0.0` when neither assembly could
    /// separate the cell's normal from zero.
    pub floor: f64,
    /// Certified UPPER bound on `‖m‖` over the cell (m²).
    pub sup: f64,
}

/// Meter 1, per cell: the two-assembly join (module docs).
pub fn cell_normal(cell: &PatchCell) -> CellNormal {
    let m = cross(&cell.s_u, &cell.s_v);
    // Assembly A: componentwise mignitude.
    let sq = RingInterval::point(mig(m[0])).sqr()
        + RingInterval::point(mig(m[1])).sqr()
        + RingInterval::point(mig(m[2])).sqr();
    let a = sqrt_down(sq.lo());
    // Assembly B: projection onto the enclosure's midpoint direction.
    // The direction is STRUCTURE (any direction is sound); the
    // division by a certified upper bound on `‖d̂‖` is what keeps the
    // projection a bound when `d̂` is unit only to rounding.
    let mid = |i: RingInterval| (i.lo() + i.hi()) * 0.5;
    let dv = [mid(m[0]), mid(m[1]), mid(m[2])];
    let dn = sqrt_up(dv[0].mul_add(dv[0], dv[1].mul_add(dv[1], dv[2] * dv[2])));
    let b = if dn > 0.0 && dn.is_finite() {
        let proj = RingInterval::point(dv[0]) * m[0]
            + RingInterval::point(dv[1]) * m[1]
            + RingInterval::point(dv[2]) * m[2];
        let lo = proj.lo();
        if lo > 0.0 { (lo / dn).next_down() } else { 0.0 }
    } else {
        0.0
    };
    CellNormal {
        m,
        floor: if a > b { a } else { b },
        sup: norm_sup(&m),
    }
}

/// The whole-patch reading of meter 1 — every field a certified bound
/// (module docs).
#[derive(Clone, Copy, Debug)]
pub struct PatchRegularity {
    /// `inf ‖S_u × S_v‖` from below, over the whole patch (m²).
    pub floor: f64,
    /// `sup ‖S_u × S_v‖` from above (m²).
    pub sup: f64,
    /// `sup ‖S_u‖` (m per unit parameter).
    pub speed_u: f64,
    /// `sup ‖S_v‖` (m per unit parameter).
    pub speed_v: f64,
    /// `floor / (speed_u · speed_v)` — a dimensionless lower bound on
    /// `sin∠(S_u, S_v)`, the quantity the predicate levers.
    pub sine_floor: f64,
    /// How many cells the fold ran over.
    pub cells: u32,
}

/// Meter 1, whole patch: the min of the per-cell floors, the max of
/// the per-cell sups (module docs).
pub fn patch_regularity(cells: &[PatchCell]) -> PatchRegularity {
    let mut floor = f64::INFINITY;
    let mut sup = 0.0f64;
    let mut speed_u = 0.0f64;
    let mut speed_v = 0.0f64;
    for cell in cells {
        let n = cell_normal(cell);
        // NaN-catching: `!(x < floor)` keeps a poisoned cell from
        // being silently skipped — a NaN sup poisons `sup` below.
        if n.floor < floor {
            floor = n.floor;
        }
        sup = sup.max(n.sup);
        speed_u = speed_u.max(norm_sup(&cell.s_u));
        speed_v = speed_v.max(norm_sup(&cell.s_v));
        if n.sup.is_nan() {
            sup = f64::NAN;
        }
    }
    if cells.is_empty() {
        floor = 0.0;
    }
    let denom = speed_u * speed_v;
    let sine_floor = if denom > 0.0 && denom.is_finite() {
        (floor / denom).next_down()
    } else {
        0.0
    };
    #[allow(clippy::cast_possible_truncation)]
    PatchRegularity {
        floor,
        sup,
        speed_u,
        speed_v,
        sine_floor,
        cells: cells.len() as u32,
    }
}

/// **`offset_normal_floor`** — the regularity predicate (module docs).
/// Margin: the dimensionless sine floor levered by `|d|`, in metres;
/// classified against the run's linear band.
///
/// # Errors
///
/// [`MeterError::NormalFloor`] when the margin is certifiably at or
/// below zero, [`MeterError::Escalated`] when it lands in the
/// ambiguity band or is poisoned.
pub fn offset_normal_floor(reg: &PatchRegularity, d: f64, band: Band) -> Result<(), MeterError> {
    let lever = d.abs();
    let margin = Margin::levered(reg.sine_floor, lever);
    match decide("offset_normal_floor", margin, band)
        .map_err(|source| MeterError::Escalated { source })?
    {
        Sign::Positive => Ok(()),
        Sign::Zero | Sign::Negative => Err(MeterError::NormalFloor {
            floor: reg.floor,
            sine_floor: reg.sine_floor,
            lever,
        }),
    }
}

/// The whole-patch reading of meter 2 (module docs).
#[derive(Clone, Copy, Debug)]
pub struct PatchCollapse {
    /// Certified lower bound on the principal curvatures (1/m).
    pub kappa_lo: f64,
    /// Certified upper bound on the principal curvatures (1/m).
    pub kappa_hi: f64,
    /// The critical distance on the FOLDING side for this `d`'s sign,
    /// in metres: `+∞` when the patch does not curve that way.
    pub reach: f64,
    /// `reach − |d|` — the margin the predicate classifies.
    pub headroom: f64,
}

/// One cell's certified principal-curvature range, or `None` when its
/// normal could not be bounded away from zero (meter 1's refusal is
/// the caller's, and it fires first).
fn cell_curvature(cell: &PatchCell) -> Option<(f64, f64)> {
    let n = cell_normal(cell);
    if !(n.floor > 0.0) || !n.sup.is_finite() {
        return None;
    }
    // The normalized normal, componentwise: `n_c = m_c / ‖m‖` with
    // `‖m‖ ∈ [floor, sup]` — meter 1's floor is exactly what makes
    // this division legal (the ring refuses a zero-touching divisor).
    let mag = RingInterval::from_bounds(n.floor, n.sup);
    let unit = [n.m[0] / mag, n.m[1] / mag, n.m[2] / mag];
    let ii_l = dot(&unit, &cell.s_uu);
    let ii_m = dot(&unit, &cell.s_uv);
    let ii_n = dot(&unit, &cell.s_vv);
    let e = dot(&cell.s_u, &cell.s_u);
    let g = dot(&cell.s_v, &cell.s_v);
    let trace_hi = e.hi() + g.hi();
    if !(trace_hi > 0.0) || !trace_hi.is_finite() {
        return None;
    }
    // λ_min(I) ≥ det/tr = ‖m‖²/(E + G) ≥ floor²/(E + G).
    let lambda_lo = (n.floor * n.floor / trace_hi).next_down();
    if !(lambda_lo > 0.0) {
        return None;
    }
    let mag_m = ii_m.mag();
    let hi = ii_l.hi().max(ii_n.hi()) + mag_m;
    let lo = ii_l.lo().min(ii_n.lo()) - mag_m;
    if !hi.is_finite() || !lo.is_finite() {
        return None;
    }
    // Dividing a nonnegative II-bound by the SMALLEST admissible
    // `I` is the conservative direction; a negative one divides by
    // the largest (`tr(I) ≥ λ_max`).
    let k_hi = if hi >= 0.0 {
        (hi / lambda_lo).next_up()
    } else {
        (hi / trace_hi).next_up()
    };
    let k_lo = if lo <= 0.0 {
        (lo / lambda_lo).next_down()
    } else {
        (lo / trace_hi).next_down()
    };
    Some((k_lo, k_hi))
}

/// Meter 2, whole patch: the certified curvature range and the
/// folding-side headroom for this `d` (module docs).
///
/// A cell whose normal is not certifiably non-degenerate contributes
/// an unbounded range — meter 1 is the predicate that names that
/// case, and it runs first.
pub fn patch_collapse(cells: &[PatchCell], d: f64) -> PatchCollapse {
    let mut kappa_lo = f64::INFINITY;
    let mut kappa_hi = f64::NEG_INFINITY;
    for cell in cells {
        match cell_curvature(cell) {
            Some((lo, hi)) => {
                kappa_lo = kappa_lo.min(lo);
                kappa_hi = kappa_hi.max(hi);
            }
            None => {
                kappa_lo = f64::NEG_INFINITY;
                kappa_hi = f64::INFINITY;
            }
        }
    }
    if cells.is_empty() {
        kappa_lo = f64::NEG_INFINITY;
        kappa_hi = f64::INFINITY;
    }
    // The fold is `1 − d·κ = 0`. For `d > 0` only positive κ folds;
    // for `d < 0` only negative κ does. `κ⁺` is the folding-side
    // curvature magnitude, and `1/κ⁺` the critical distance.
    let k_fold = if d > 0.0 {
        kappa_hi.max(0.0)
    } else {
        (-kappa_lo).max(0.0)
    };
    let reach = if k_fold > 0.0 {
        (1.0 / k_fold).next_down()
    } else {
        f64::INFINITY
    };
    PatchCollapse {
        kappa_lo,
        kappa_hi,
        reach,
        headroom: reach - d.abs(),
    }
}

/// **`offset_curvature_headroom`** — the collapse predicate (module
/// docs). Margin: `reach − |d|` in metres, the distance between the
/// requested offset and the patch's certified fold radius on the
/// folding side; classified against the run's linear band.
///
/// # Errors
///
/// [`MeterError::CurvatureHeadroom`] when the margin is certifiably
/// at or below zero, [`MeterError::Escalated`] when it lands in the
/// ambiguity band or is poisoned.
pub fn offset_curvature_headroom(coll: &PatchCollapse, band: Band) -> Result<(), MeterError> {
    match decide("offset_curvature_headroom", Margin::of(coll.headroom), band)
        .map_err(|source| MeterError::Escalated { source })?
    {
        Sign::Positive => Ok(()),
        Sign::Zero | Sign::Negative => Err(MeterError::CurvatureHeadroom {
            reach: coll.reach,
            headroom: coll.headroom,
            kappa: (coll.kappa_lo, coll.kappa_hi),
        }),
    }
}
