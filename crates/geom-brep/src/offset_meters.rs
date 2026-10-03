//! **The two meters the offset fit needs** — the regularity floor and
//! the collapse headroom (`crates/geom-brep/README.md` O3).
//!
//! Both are read off [`crate::patch_bound`]'s per-cell enclosures, and
//! both are **f64-substrate**: certification arithmetic produces an `f64` certified
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
//! ([`geom_core::spline::SplineCoeffs::sup_norm_bound`] and its family); the
//! curve side's inf meter (`NurbsCurve3::speed_lower_bound`) is
//! one dimension down and does not lift directly, because the
//! quantity here is a **product** of two coefficient nets, not one.
//!
//! ## The derivation
//!
//! On a cell, `S_u` and `S_v` each lie componentwise in the signed
//! hull of their active derived coefficients ([`crate::patch_bound`]).
//! Interval-multiplying those three-component enclosures gives an
//! enclosure `M ⊇ { S_u(u,v) × S_v(u,v) : (u,v) ∈ cell }`. THREE
//! sound lower bounds on `‖m‖` are assembled and the largest wins —
//! the speed meter's two-assembly join, lifted and extended:
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
//! - **The Gram determinant**: Lagrange's identity
//!   `‖S_u × S_v‖² = EG − F²`, evaluated on the first fundamental
//!   form's own enclosures. `E` and `G` are DEPENDENT squares — no
//!   cancellation to lose — so on a near-orthogonal chart (`F ≈ 0`)
//!   this is far tighter than either cross-product assembly, whose
//!   three components each carry the full width of two factors. It is
//!   also the only assembly that tightens the SUP side.
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
//! ## The margin and its lever, and why the lever is NOT `d`
//!
//! The certified quantity is `floor`, in **m² per unit parameter
//! area** — the natural units of `‖S_u × S_v‖`, and the number the
//! certificate actually consumes (it is what makes `1/‖S_u × S_v‖`
//! boundable). The *predicate* cannot classify an area rate against a
//! linear band, so it needs a lever, and the choice of lever is the
//! whole content of this paragraph.
//!
//! **The lever is the patch's own faster chart speed**:
//! [`offset_normal_floor`] classifies
//! `Margin::over_lever_down(floor, max(sup‖S_u‖, sup‖S_v‖))` — the
//! `over_lever` door's own named case, a chart-orientation area over
//! the length that scales it, with the quotient rounded down so the
//! lower bound stays one. The quotient is
//! `min(‖S_u‖, ‖S_v‖)·sin∠(S_u, S_v)` up to the sup-side slack: the
//! **thinness of the chart parallelogram**, in metres, and exactly
//! the length that goes to zero as the normal degenerates.
//!
//! **What it deliberately is not.** An earlier spelling levered the
//! dimensionless sine floor by `|d|`, reasoning that a normal
//! ambiguity of angle `θ` displaces the offset point by `|d|·θ`. That
//! reasoning is sound about the *certificate* and wrong about *this
//! predicate*, in the direction that matters: it makes the margin
//! grow with `|d|`, so the same fixed geometry is admitted at a large
//! offset and refused at a small one — permissive exactly where the
//! displacement is largest, and refusing a perfectly regular patch
//! for no reason but a small `d`. The subject of this meter — does
//! the chart normal degenerate? — does not depend on `d` at all, and
//! the margin must not either. The `|d|` dependence belongs where it
//! already is: the certificate consumes `floor` directly (`τ` and the
//! sign witness both divide by `‖m‖`), so nothing unsound reaches it
//! from a `d`-independent door.
//!
//! Sup-side denominators push the quotient DOWN, so the margin is
//! conservative in the same direction as the floor.
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
//! same cell enclosures, in the **closed form** rather than as a
//! quotient of separately bounded quadratic forms
//! (`cell_curvature` carries the derivation and the measured reason):
//!
//! ```text
//! II: L = n·S_uu,  M = n·S_uv,  N = n·S_vv
//! I:  E = S_u·S_u, F = S_u·S_v, G = S_v·S_v
//! A = EG − F² = ‖S_u × S_v‖²   B = LG − 2MF + NE   C = LN − M²
//! κ± = H ± √(H² − K),   H = B/2A,   K = C/A
//! ```
//!
//! **This is the one place the two meters compose**: the normal `n` in
//! `II` is the normalized normal, enclosed as `M_c / [floor, sup]`,
//! and `A` is taken as `[floor², sup²]` — meter 1's floor is what
//! makes both exist.
//!
//! This is the fillet battery's radius-headroom shape one dimension
//! up: there, a blend radius against a spine's curvature; here, an
//! offset distance against a patch's.

use geom_core::Bounds;
use geom_core::interval::Interval;
use geom_core::interval::certification::Certification;
use geom_core::interval::{div_down, norm_sq, norm_sup};
use geom_core::{Band, Indeterminate, Margin, SupSpeed};

use crate::dihedral::decide_reported;
use crate::patch_bound::{PatchBoundError, PatchCell, patch_cells_refined};
use crate::recourse::{
    AtZero, Reading, Refused, RefusedArm, SizedDecision, SizedPass, StoredDefinite,
};

/// The refinement ladder the door walks, coarsest first (D9: a fixed
/// geometric sequence in a fixed order — no value branch chooses it).
/// The first rung on which BOTH meters certify wins; if none does,
/// the finest rung's refusal is the answer.
///
/// A ladder rather than one fixed schedule because the cost is real
/// and the need is not uniform: a cylinder certifies at the first
/// rung and pays nothing more, while an exactly-umbilic patch (a
/// sphere band) needs the second before its certified curvature range
/// is inside a factor of two of the single value the surface actually
/// has — see [`patch_cells_refined`] for why the widths shrink only
/// linearly in the span size.
///
/// Two rungs and not more because a rung costs `splits²` cells: the
/// second already answers every fixture measured, and a third is a
/// decision for the first consumer that meets a patch needing it —
/// the refusal names the numbers, so that consumer will know.
pub const OFFSET_METER_LADDER: [usize; 2] = [16, 64];

/// The two offset meters: each is one decision, a clearance that passes
/// on a positive margin (D4 ¶1 (i)), and this is its closed type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Meter {
    /// [`offset_normal_floor`]: is the chart normal bounded away from
    /// degeneracy?
    NormalFloor,
    /// [`offset_curvature_headroom`]: does `|d|` stay inside the fold
    /// radius on the side the face bends toward?
    CurvatureHeadroom,
}

impl Meter {
    /// The predicate name the meter's classification carries into its
    /// escalation's payload.
    #[must_use]
    pub const fn predicate(self) -> &'static str {
        match self {
            Self::NormalFloor => "offset_normal_floor",
            Self::CurvatureHeadroom => "offset_curvature_headroom",
        }
    }

    /// The meter's decision: its lever, the size its margin measures,
    /// and what it passes on. A definite refusal read over a body at
    /// rest is still the lever's: the stored description's base and
    /// distance are what it edits.
    fn decision(self) -> SizedDecision {
        match self {
            // A floor of exactly zero on a face with no pole, cusp or
            // pinch is the refinement ladder running out of rungs on a
            // regular patch, not the geometry: worth a report.
            Self::NormalFloor => SizedDecision {
                lever: "split the face clear of any pole, cusp or pinch",
                size: "thinness",
                passes: SizedPass::Positive,
                stored: StoredDefinite::Lever,
                at_zero: Some(AtZero::same(
                    "if it has none, this may indicate a kernel bug worth reporting",
                )),
            },
            Self::CurvatureHeadroom => SizedDecision {
                lever: "use an offset distance of smaller magnitude, or offset to the other side",
                size: "clearance",
                passes: SizedPass::Positive,
                stored: StoredDefinite::Lever,
                at_zero: None,
            },
        }
    }

    /// The one ending this meter's refusal carries on `arm`, read at
    /// `reading` ([`SizedDecision::recourse`]).
    #[must_use]
    pub fn recourse(self, arm: RefusedArm<'_>, reading: Reading) -> String {
        self.decision().recourse(arm, reading)
    }
}

/// A typed refusal of one of the two offset meters (D4 ¶3). Scalar
/// payloads echo the classified margin's ingredients — data, not a
/// decision.
///
/// `Display` renders the payload alone: where a refusal is read decides
/// its ending (D4 ¶1 (i)), so the door that reports it appends
/// [`MeterError::ending`], or renders both through
/// [`MeterError::render`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MeterError {
    /// `offset_normal_floor`: the patch's chart normal could not be
    /// bounded away from degeneracy, so the offset locus is not
    /// defined on it (or the bound is too weak to prove that it is).
    ///
    /// A `floor` of exactly zero is the loud answer of a cell the
    /// normal turns too far inside, which splitting the face clear of
    /// the degeneracy (a pole, cusp or pinch — a point the user can
    /// see) answers. A regular patch refusing here means
    /// [`OFFSET_METER_LADDER`] ran out of rungs on it, which is why the
    /// ending of a zero floor says a face with none of those is worth
    /// reporting; the payload renders the numbers that decide a further
    /// rung.
    NormalFloor {
        /// The certified lower bound on `‖S_u × S_v‖` (m² per unit
        /// parameter area) — zero when no cell could be certified.
        floor: f64,
        /// The lever the margin divided by, in metres per unit
        /// parameter — the patch's faster chart speed.
        speed_lever: f64,
        /// The verdict on the classified margin, the chart
        /// parallelogram's certified thinness in metres,
        /// `floor / max(sup‖S_u‖, sup‖S_v‖)`. The thinness is never
        /// negative, so it is [`Refused::Zero`] on every input the meter
        /// classifies.
        verdict: Refused,
    },
    /// `offset_curvature_headroom`: `|d|` reaches the patch's
    /// smallest certified curvature radius on the folding side, so
    /// the offset self-intersects (or the bound is too weak to prove
    /// that it does not). The message names `reach`, the distance a
    /// request must stay strictly inside; the margin and `kappa` ride
    /// in the payload.
    CurvatureHeadroom {
        /// The certified critical distance on the folding side, in
        /// metres (`+∞` when the patch does not curve that way).
        reach: f64,
        /// The certified principal-curvature range, in 1/m.
        kappa: (f64, f64),
        /// The verdict on the classified margin `reach − |d|`, in
        /// metres.
        verdict: Refused,
    },
    /// A meter escalated: the margin landed in the ambiguity band or
    /// was refused (escalate-never-guess, D4 ¶3).
    ///
    /// It renders the classifier's payload without `Indeterminate`'s
    /// shared coincidence tail — a meter decides one face's own normal
    /// or curvature, so "declare the coincidence" has no object.
    Escalated {
        /// The meter that escalated.
        meter: Meter,
        /// The predicate-layer escalation.
        source: Indeterminate,
    },
}

impl core::fmt::Display for MeterError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NormalFloor {
                floor,
                speed_lever,
                verdict,
            } => write!(
                f,
                "the face's normal cannot be proved non-zero (a chart {} m thin: normal \
                 length {floor} m² per unit parameter area over speed {speed_lever} m), so it \
                 has no offset",
                verdict.margin()
            ),
            Self::CurvatureHeadroom {
                reach,
                verdict: Refused::Negative { .. },
                ..
            } => write!(
                f,
                "the offset distance's magnitude passes the face's radius of curvature on the \
                 side it bends toward ({reach} m), so the offset folds over itself"
            ),
            Self::CurvatureHeadroom {
                reach,
                verdict: Refused::Zero(_),
                ..
            } => write!(
                f,
                "the offset distance's magnitude is within tolerance of the face's radius of \
                 curvature on the side it bends toward ({reach} m), so the offset may fold"
            ),
            Self::Escalated { source, .. } => write!(
                f,
                "the face's offset is too close to call: {}",
                source.payload()
            ),
        }
    }
}

impl MeterError {
    /// The meter that refused.
    #[must_use]
    pub fn meter(&self) -> Meter {
        match self {
            Self::NormalFloor { .. } => Meter::NormalFloor,
            Self::CurvatureHeadroom { .. } => Meter::CurvatureHeadroom,
            Self::Escalated { meter, .. } => *meter,
        }
    }

    /// The ending this refusal's meter gives its verdict, read at
    /// `reading` ([`Meter::recourse`]).
    #[must_use]
    pub fn ending(&self, reading: Reading) -> String {
        let arm = match self {
            Self::NormalFloor { verdict, .. } | Self::CurvatureHeadroom { verdict, .. } => {
                verdict.arm()
            }
            Self::Escalated { source, .. } => RefusedArm::Undecided(source),
        };
        self.meter().recourse(arm, reading)
    }

    /// The payload and the ending read at `reading`.
    #[must_use]
    pub fn render(&self, reading: Reading) -> String {
        format!("{self}. {}", self.ending(reading))
    }
}

impl core::error::Error for MeterError {}

/// The smallest `|x|` over the enclosure — zero when it straddles, and
/// zero when it is refused, which is the conservative answer.
///
/// **The refusal arm is asked by name.** Interval arithmetic's refusal is its
/// decoration, not a NaN pair, so a refused enclosure carries ordinary
/// endpoints and `i.lo() > 0.0` can be TRUE of one — a quotient by a
/// divisor not proven away from zero is the shape that reaches here.
///
/// The **mignitude**, and the same quantity `ssi::enclose`'s
/// `zero_free_lower_bound` reads for the transversality margin. Kept
/// separate rather than shared: that one is a *decision* helper on a
/// residual channel and answers `0.0` for a straddling interval
/// because a straddling residual proves nothing; this one is a
/// coefficient-hull *assembly* term and answers `0.0` for the same
/// arithmetic reason. One spelling would have to pick one of the two
/// docs, and the shared body is four comparisons.
pub fn mig(i: Interval) -> f64 {
    if !i.is_certified() {
        return 0.0;
    }
    if i.lo() > 0.0 {
        i.lo()
    } else if i.hi() < 0.0 {
        -i.hi()
    } else {
        0.0
    }
}

/// Interval dot product, fixed ascending order (D9).
fn dot(a: &[Interval; 3], b: &[Interval; 3]) -> Interval {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// Interval cross product, fixed component order (D9).
///
/// Component-for-component `ssi::enclose::cross3`, which is private
/// to that module and takes its operands by value. [`dot`] and this
/// are the borrow-shaped pair; if a third consumer ever wants them,
/// the pair collapses into one `geom_core` home beside
/// [`norm_sq`], which already has — noted at both sites so the
/// duplication is a decision rather than an accident.
fn cross(a: &[Interval; 3], b: &[Interval; 3]) -> [Interval; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// One cell's chart-normal facts: the enclosure of `S_u × S_v` and
/// certified bounds on its magnitude (module docs, meter 1).
#[derive(Clone, Copy, Debug)]
pub struct CellNormal {
    /// Componentwise enclosure of `m = S_u × S_v` on the cell.
    pub m: [Interval; 3],
    /// Certified LOWER bound on `‖m‖` over the cell, in
    /// [`PatchRegularity::floor`]'s units (m² per unit parameter
    /// area) — the regularity floor. Exactly `0.0` when neither
    /// assembly could separate the cell's normal from zero.
    pub floor: f64,
    /// Certified UPPER bound on `‖m‖` over the cell, same units.
    pub sup: f64,
}

/// Meter 1, per cell: the three-assembly join (module docs).
///
/// **Reads the cell's signed enclosures inf-side**, and they enclose
/// the DESCRIBED patch on both arms ([`PatchCell`], "What the enclosure
/// encloses"), so an inf-side read needs no caveat about which patch it
/// is about. What it still inherits is the refinement's WIDTH: the
/// enclosures carry the insertion's outward rounding, which subtracts
/// from a floor rather than being ignored by it. Every claim here is a
/// magnitude claim at ε scale, decades above that width.
pub fn cell_normal(cell: &PatchCell) -> CellNormal {
    let m = cross(&cell.s_u, &cell.s_v);
    // Assembly A: componentwise mignitude.
    let sq = Interval::point(mig(m[0])).sqr()
        + Interval::point(mig(m[1])).sqr()
        + Interval::point(mig(m[2])).sqr();
    // A refused enclosure separates nothing from zero, and `0.0` is
    // the floor's conservative answer — asked by name, because a
    // refusal here carries real endpoints.
    let root_a = sq.sqrt();
    let a = if !root_a.is_certified() {
        0.0
    } else {
        root_a.lo()
    };
    // Assembly B: projection onto the enclosure's midpoint direction.
    // The direction is STRUCTURE (any direction is sound, and that is
    // why this midpoint needs no refusal of its own); the division by
    // a certified upper bound on `‖d̂‖` is what keeps the projection a
    // bound when `d̂` is unit only to rounding.
    let mid = |i: Interval| (i.lo() + i.hi()) * 0.5;
    let dv = [mid(m[0]), mid(m[1]), mid(m[2])];
    let dn = norm_sup(&dv.map(Interval::point));
    let b = if dn > 0.0 && dn.is_finite() {
        let proj = Interval::point(dv[0]) * m[0]
            + Interval::point(dv[1]) * m[1]
            + Interval::point(dv[2]) * m[2];
        // The quotient stays IN INTERVAL ARITHMETIC and `.lo()` is read once, so
        // the outward rounding of the division is interval arithmetic's rather
        // than this function's: a bare `lo / dn` would be a
        // correctly-rounded f64 quotient, which is not a lower bound
        // on the real one. `dn` is a certified UPPER bound on `‖d̂‖`,
        // so dividing by it is the sound side.
        //
        // The quotient's refusal is asked by name: `f64::max(NaN, 0.0)`
        // used to absorb a NaN quotient, and a refused quotient
        // now carries real endpoints instead.
        let q = proj / Interval::point(dn);
        if !q.is_certified() {
            0.0
        } else {
            q.lo().max(0.0)
        }
    } else {
        0.0
    };
    // Assembly C: the Gram determinant, `‖m‖² = EG − F²` (Lagrange's
    // identity). It bounds BOTH ends, and on a chart whose parameter
    // directions are near-orthogonal it is dramatically tighter than
    // either cross-product assembly — `E` and `G` are DEPENDENT
    // squares with no cancellation to lose, while the cross product's
    // three components each carry the full width of two factors. On
    // the sphere-band fixture it is the assembly that moves the
    // certified curvature range from tens to fractions.
    //
    // The difference does not cancel in interval arithmetic, so its
    // enclosure may reach below zero; Lagrange's identity is the
    // outside fact that clamps it.
    let gram = norm_sq(&cell.s_u) * norm_sq(&cell.s_v) - dot(&cell.s_u, &cell.s_v).sqr();
    let root_c = gram.clamped_to(0.0, f64::INFINITY).sqrt();
    let (c, gram_sup) = if !root_c.is_certified() {
        (0.0, f64::NAN)
    } else {
        (root_c.lo(), root_c.hi())
    };
    let floor = if a > b { a } else { b };
    CellNormal {
        m,
        floor: if floor > c { floor } else { c },
        sup: norm_sup(&m).min(gram_sup),
    }
}

/// The whole-patch reading of meter 1 — every field a certified bound
/// (module docs).
#[derive(Clone, Copy, Debug)]
pub struct PatchRegularity {
    /// `inf ‖S_u × S_v‖` from below, over the whole patch — an AREA
    /// RATE, m² per unit parameter area, which is `‖S_u × S_v‖`'s own
    /// unit and the module docs' one spelling of it.
    pub floor: f64,
    /// `sup ‖S_u × S_v‖` from above, in [`PatchRegularity::floor`]'s
    /// units.
    pub sup: f64,
    /// `sup ‖S_u‖` (m per unit parameter) — a [`SupSpeed`] by
    /// signature: every consumer of it meters an overshoot (the
    /// regularity lever below, the refinement schedule's split
    /// selection), where over-stating the speed refuses and
    /// under-stating admits a fold.
    pub speed_u: SupSpeed<f64>,
    /// `sup ‖S_v‖` (m per unit parameter); see
    /// [`PatchRegularity::speed_u`] for the bound direction.
    pub speed_v: SupSpeed<f64>,
    /// `floor / (speed_u · speed_v)` — a dimensionless lower bound on
    /// `sin∠(S_u, S_v)`. A DIAGNOSTIC: the predicate classifies
    /// [`PatchRegularity::thinness`], not this (module docs).
    pub sine_floor: f64,
    /// How many cells the fold ran over.
    pub cells: u32,
}

impl PatchRegularity {
    /// The lever the regularity predicate divides by: the patch's
    /// faster chart speed, in metres per unit parameter, folded on the
    /// rate type so a refused axis refuses the lever.
    pub fn speed_lever(&self) -> SupSpeed<f64> {
        self.speed_u.max(self.speed_v)
    }

    /// The margin [`offset_normal_floor`] classifies — the chart
    /// parallelogram's certified thinness in metres,
    /// `floor / max(sup‖S_u‖, sup‖S_v‖)`, which is
    /// `min(‖S_u‖, ‖S_v‖)·sin∠(S_u, S_v)` up to the sup-side slack.
    /// The quotient rounds DOWN ([`div_down`]): a lower bound over an
    /// upper bound stays a lower bound only that way.
    ///
    /// Deliberately UNGUARDED, so this and the predicate are the same
    /// number on every input: a zero lever leaves `0/0`, which
    /// escalates rather than certifying, and an infinite one leaves a
    /// zero margin, which refuses. Both are the loud answer.
    /// **Why the tag comes off here.** The rate pair's
    /// [`to_param`](SupSpeed::to_param) door crosses a model-space
    /// LENGTH to parameter units, and `floor` is not one: it is an
    /// area rate (m² per unit parameter area), so `floor / lever` is
    /// m per unit parameter — itself a rate, and an inf-side one
    /// (`≤ min(‖S_u‖, ‖S_v‖)·sin∠`, under-stated by the sup in the
    /// denominator). What makes it the metres the predicate
    /// classifies is the module's own unit-parameter-cell convention
    /// (module docs, *The margin and its lever*), which is
    /// [`Margin::over_lever_down`]'s argument and not the rate pair's. So
    /// the quotient leaves this door untyped rather than reaching for
    /// one whose dimensional argument does not cover it.
    pub fn thinness(&self) -> f64 {
        div_down(self.floor, self.speed_lever().get())
    }
}

/// Meter 1, whole patch: the min of the per-cell floors, the max of
/// the per-cell sups (module docs).
pub fn patch_regularity(cells: &[PatchCell]) -> PatchRegularity {
    let mut floor = f64::INFINITY;
    let mut sup = 0.0f64;
    let mut speed_u = SupSpeed::new(0.0f64);
    let mut speed_v = SupSpeed::new(0.0f64);
    for cell in cells {
        let n = cell_normal(cell);
        // `cell_normal` never answers a NaN floor — its assemblies
        // clamp at zero, which is the conservative reading of a
        // refused cell — so a plain `<` is the whole fold. The sup
        // CAN be NaN (it reads `mag`), and the explicit refusal step
        // below is what keeps that from being dropped by `max`.
        if n.floor < floor {
            floor = n.floor;
        }
        sup = sup.max(n.sup);
        speed_u = speed_u.max(SupSpeed::new(norm_sup(&cell.s_u)));
        speed_v = speed_v.max(SupSpeed::new(norm_sup(&cell.s_v)));
        if n.sup.is_nan() {
            sup = f64::NAN;
        }
    }
    if cells.is_empty() {
        floor = 0.0;
    }
    let denom = speed_u.get() * speed_v.get();
    let sine_floor = if denom > 0.0 && denom.is_finite() {
        div_down(div_down(floor, speed_u.get()), speed_v.get())
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

/// **`offset_normal_floor`** — the regularity predicate (module
/// docs). Margin: the chart parallelogram's certified thinness,
/// `floor / max(sup‖S_u‖, sup‖S_v‖)` in metres; classified against
/// the run's linear band. **It takes no `d`**, deliberately — see the
/// module docs' lever paragraph for why an earlier `|d|` lever was
/// inverted with respect to the risk it was supposed to meter.
///
/// # Errors
///
/// [`MeterError::NormalFloor`] when the margin is certifiably at or
/// below zero, [`MeterError::Escalated`] when it lands in the
/// ambiguity band or is refused.
pub fn offset_normal_floor(reg: &PatchRegularity, band: Band) -> Result<(), MeterError> {
    let meter = Meter::NormalFloor;
    let margin = Margin::over_lever_down(reg.floor, reg.speed_lever().get());
    let decided = decide_reported(meter.predicate(), margin, band)
        .map_err(|source| MeterError::Escalated { meter, source })?;
    match Refused::of(decided, band) {
        None => Ok(()),
        Some(verdict) => Err(MeterError::NormalFloor {
            floor: reg.floor,
            speed_lever: reg.speed_lever().get(),
            verdict,
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
///
/// The principal curvatures are the eigenvalues of the shape operator
/// `W = I⁻¹·II`, equivalently the roots of `det(II − κ·I) = 0`, i.e.
/// `κ² A − κ B + C = 0` with
///
/// ```text
/// A = EG − F² = ‖S_u × S_v‖²      B = L G − 2 M F + N E      C = L N − M²
/// H = B / 2A   (mean)             K = C / A   (Gaussian)
/// κ± = H ± √(H² − K)
/// ```
///
/// **TWO sound assemblies, the tighter end of each winning** — the
/// regularity floor's join, one level up. Neither alone is good
/// enough, and the reason is instructive:
///
/// - The closed form's `√(H² − K)` is a **square root of a
///   cancellation**. On an umbilic patch `H² − K` is identically
///   zero, so its enclosure is pure interval slack `δ` and the root
///   contributes `√δ` — an amplifier, not an attenuator. Measured on
///   the sphere-band fixture it alone reported `κ ∈ [−11.6, 6.4]`
///   where the surface has the single value `−0.5`.
/// - Gershgorin on the shape operator has no root at all: its radius
///   is `|W₁₂|`, which for an umbilic patch is a two-term
///   cancellation (`GM − FN = 0`) whose enclosure is `O(δ)`, linear.
///   That is what brings the same fixture inside a factor of three.
///
/// The closed form still wins where the off-diagonal entries are
/// genuinely large and the discriminant genuinely positive (a patch
/// with well-separated principal curvatures in a skew chart), so both
/// run and the intersection is taken.
///
/// A quadratic-form estimate — `|II| ≤ max(L, N) + |M|` over
/// `λ_min(I) ≥ det/tr` — is worse than either: it throws away every
/// correlation between the two forms at once.
///
/// `A` is taken from meter 1's own bounds (`[floor², sup²]`) rather
/// than re-derived as `E·G − F·F`, because that difference does not
/// cancel in interval arithmetic and the floor is the tighter — and
/// already certified — fact.
fn cell_curvature(cell: &PatchCell) -> Option<(f64, f64)> {
    let n = cell_normal(cell);
    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    if !(n.floor > 0.0) || !n.sup.is_finite() {
        return None;
    }
    // The normalized normal, componentwise: `n_c = m_c / ‖m‖` with
    // `‖m‖ ∈ [floor, sup]` — meter 1's floor is exactly what makes
    // this division legal (interval arithmetic refuses a zero-touching divisor).
    let mag = Interval::from_bounds(n.floor, n.sup);
    let unit = [n.m[0] / mag, n.m[1] / mag, n.m[2] / mag];
    let (l, m, nn) = (
        dot(&unit, &cell.s_uu),
        dot(&unit, &cell.s_uv),
        dot(&unit, &cell.s_vv),
    );
    let e = norm_sq(&cell.s_u);
    let f = dot(&cell.s_u, &cell.s_v);
    let g = norm_sq(&cell.s_v);
    let two = Interval::point(2.0);
    let a = Interval::from_bounds(n.floor, n.sup).sqr();
    // Assembly A — the closed form `κ± = H ± √(H² − K)`.
    let b = l * g - two * m * f + nn * e;
    let c = l * nn - m.sqr();
    let h = b / (two * a);
    let k = c / a;
    // **The refusal is asked here, not left to the finiteness check at
    // the end.** Both divisions above are by `A`, which is not proven
    // away from zero on a cell whose normal barely separated, and a
    // refused quotient carries real endpoints: `k_hi.is_finite()`
    // would pass on one. Worse, the joins below are `f64::min`/`max`,
    // which DROP a NaN operand — so one assembly's refusal would be
    // covered by the other assembly's number.
    if !h.is_certified() || !k.is_certified() {
        return None;
    }
    // `H² − K` is nonnegative at every real point (the principal
    // curvatures are real): the outside fact that clamps the radicand.
    // A sound enclosure of it reaches zero, so a refused root is a
    // refused enclosure upstream, and it refuses the cell.
    let root = (h.sqr() - k).clamped_to(0.0, f64::INFINITY).sqrt().mag();
    if root.is_nan() {
        return None;
    }
    let (a_hi, a_lo) = (h.hi() + root, h.lo() - root);
    // Assembly B — Gershgorin on the shape operator `W = I⁻¹·II`,
    // `I⁻¹ = (1/A)·[[G, −F], [−F, E]]`. Its eigenvalues ARE the
    // principal curvatures (real, since `W` is similar to a symmetric
    // matrix), so every one lies within `|W₁₂|` of `W₁₁` or within
    // `|W₂₁|` of `W₂₂`.
    let w11 = (g * l - f * m) / a;
    let w12 = (g * m - f * nn) / a;
    let w21 = (e * m - f * l) / a;
    let w22 = (e * nn - f * m) / a;
    // The same refusal, for the same reason, over Gershgorin's four
    // entries.
    if !w11.is_certified() || !w12.is_certified() || !w21.is_certified() || !w22.is_certified() {
        return None;
    }
    let b_hi = (w11.hi() + w12.mag()).max(w22.hi() + w21.mag());
    let b_lo = (w11.lo() - w12.mag()).min(w22.lo() - w21.mag());
    // Both assemblies are sound, so the tighter end of each wins.
    let (k_hi, k_lo) = (a_hi.min(b_hi), a_lo.max(b_lo));
    if !k_hi.is_finite() || !k_lo.is_finite() {
        return None;
    }
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

/// Both door meters, over the refinement ladder (module docs): the
/// first rung on which BOTH certify wins, and its readings are what
/// the certificate carries. A rung that fails escalates to the next;
/// the finest rung's refusal is returned as the door's.
///
/// # Errors
///
/// [`MeterError`] from the finest rung tried, or the patch-bound
/// assembly's own refusal.
pub fn meter_patch(
    base: &geom::surfaces::NurbsSurface<f64>,
    d: f64,
    band: Band,
) -> Result<(PatchRegularity, PatchCollapse), MeterResult> {
    // One rung: a patch-bound refusal ends the ladder (the outer
    // `Result`); a meter's verdict is the rung's own (the inner one).
    let rung = |splits| -> Result<Result<(PatchRegularity, PatchCollapse), MeterError>, _> {
        let cells = patch_cells_refined(base, splits).map_err(MeterResult::PatchBound)?;
        let reg = patch_regularity(&cells);
        let coll = patch_collapse(&cells, d);
        Ok(offset_normal_floor(&reg, band)
            .and_then(|()| offset_curvature_headroom(&coll, band))
            .map(|()| (reg, coll)))
    };
    // The ladder is a non-empty array, so this pattern is irrefutable
    // and the finest rung's verdict is the answer by construction.
    let [coarser @ .., finest] = OFFSET_METER_LADDER;
    for splits in coarser {
        if let Ok(readings) = rung(splits)? {
            return Ok(readings);
        }
    }
    rung(finest)?.map_err(MeterResult::Meter)
}

/// What [`meter_patch`] refuses with: a meter's own verdict, or the
/// patch-bound assembly's structural refusal underneath it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MeterResult {
    /// A meter refused (or escalated) at the finest rung tried.
    Meter(MeterError),
    /// The per-cell assembly could not be built at all.
    PatchBound(PatchBoundError),
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
/// ambiguity band or is refused.
pub fn offset_curvature_headroom(coll: &PatchCollapse, band: Band) -> Result<(), MeterError> {
    let meter = Meter::CurvatureHeadroom;
    let decided = decide_reported(meter.predicate(), Margin::of(coll.headroom), band)
        .map_err(|source| MeterError::Escalated { meter, source })?;
    match Refused::of(decided, band) {
        None => Ok(()),
        Some(verdict) => Err(MeterError::CurvatureHeadroom {
            reach: coll.reach,
            kappa: (coll.kappa_lo, coll.kappa_hi),
            verdict,
        }),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use geom_core::MarginDiag;
    use geom_core::predicate::COINCIDENCE_RECOURSE;

    use super::*;
    use crate::recourse::Classified;

    const SPLIT: &str = "Recourse: split the face clear of any pole, cusp or pinch";
    const DISTANCE: &str =
        "Recourse: use an offset distance of smaller magnitude, or offset to the other side";
    const REPORT: &str = "; if it has none, this may indicate a kernel bug worth reporting";
    const UNREAD: &str = "; an unreadable or collapsed margin may indicate a kernel bug worth \
                          reporting";

    /// `K = 10`: a margin `m` passes at every tolerance below `m/10`.
    fn band() -> Band {
        Band::new(1e-9, 1e-8).unwrap()
    }

    fn zero(margin: f64) -> Refused {
        Refused::Zero(Classified {
            margin: MarginDiag::value(margin),
            band: band(),
        })
    }

    fn negative(margin: f64) -> Refused {
        Refused::Negative {
            margin: MarginDiag::value(margin),
        }
    }

    /// `floor` is twice the thinness `m` and the lever is 2, so a
    /// payload that rendered one number for the other would read
    /// differently.
    fn floor_of(m: f64, verdict: Refused) -> MeterError {
        MeterError::NormalFloor {
            floor: 2.0 * m,
            speed_lever: 2.0,
            verdict,
        }
    }

    fn floor_zero(m: f64) -> MeterError {
        floor_of(m, zero(m))
    }

    fn headroom(verdict: Refused) -> MeterError {
        MeterError::CurvatureHeadroom {
            reach: 1e-3,
            kappa: (-1e3, 0.0),
            verdict,
        }
    }

    fn escalated(meter: Meter, margin: MarginDiag) -> MeterError {
        MeterError::Escalated {
            meter,
            source: Indeterminate {
                margin,
                band: band(),
                predicate: Some(meter.predicate()),
                terminal_sliver: false,
            },
        }
    }

    /// Every arm of both meters ends in its decision's one recourse,
    /// pinned whole at a build and at rest (D4 ¶1 (i)/(iv)): the lever
    /// on every arm; the conditional tighten, valued at `m/K`, only on
    /// a band-decided arm whose margin is positive (a zero verdict
    /// inside the band, or an undecided margin); the lever alone on the
    /// sign-certain arm, on a margin on the refused side, and straddling
    /// zero; the report clause on a zero floor, which no tolerance
    /// resolves; and a poisoned margin keeps the lever and says what it
    /// may mean.
    #[test]
    fn each_meter_arm_ends_in_its_decisions_recourse() {
        let straddle = MarginDiag::enclosure(-2e-9, 4e-9);
        let rows: [(MeterError, String); 16] = [
            (
                floor_zero(5e-10),
                format!(
                    "{SPLIT}, or, if this thinness is intended, tighten the tolerance below \
                     5e-11 m"
                ),
            ),
            (floor_zero(0.0), format!("{SPLIT}{REPORT}")),
            (floor_of(-1e-3, negative(-1e-3)), SPLIT.to_owned()),
            (
                escalated(Meter::NormalFloor, MarginDiag::value(5e-9)),
                format!(
                    "{SPLIT}, or, if this thinness is intended, tighten the tolerance below \
                     5e-10 m"
                ),
            ),
            (escalated(Meter::NormalFloor, straddle), SPLIT.to_owned()),
            (
                escalated(Meter::NormalFloor, MarginDiag::value(-5e-9)),
                SPLIT.to_owned(),
            ),
            (
                escalated(Meter::NormalFloor, MarginDiag::INVALID),
                format!("{SPLIT}{UNREAD}"),
            ),
            (
                headroom(zero(5e-10)),
                format!(
                    "{DISTANCE}, or, if this clearance is intended, tighten the tolerance below \
                     5e-11 m"
                ),
            ),
            (headroom(zero(-5e-10)), DISTANCE.to_owned()),
            (headroom(zero(0.0)), DISTANCE.to_owned()),
            (headroom(negative(-1e-4)), DISTANCE.to_owned()),
            (
                escalated(Meter::CurvatureHeadroom, MarginDiag::value(5e-9)),
                format!(
                    "{DISTANCE}, or, if this clearance is intended, tighten the tolerance below \
                     5e-10 m"
                ),
            ),
            (
                escalated(Meter::CurvatureHeadroom, MarginDiag::enclosure(2e-9, 4e-9)),
                format!(
                    "{DISTANCE}, or, if this clearance is intended, tighten the tolerance below \
                     2e-10 m"
                ),
            ),
            (
                escalated(Meter::CurvatureHeadroom, straddle),
                DISTANCE.to_owned(),
            ),
            (
                escalated(Meter::CurvatureHeadroom, MarginDiag::value(-5e-9)),
                DISTANCE.to_owned(),
            ),
            (
                escalated(Meter::CurvatureHeadroom, MarginDiag::INVALID),
                format!("{DISTANCE}{UNREAD}"),
            ),
        ];
        for (error, want) in rows {
            for reading in [Reading::Build, Reading::AtRest] {
                assert_eq!(error.ending(reading), want, "{error:?} at {reading:?}");
                let text = error.render(reading);
                assert_eq!(text, format!("{error}. {want}"), "{reading:?}");
                assert_eq!(text.matches("Recourse:").count(), 1, "{text}");
                assert!(!text.contains(COINCIDENCE_RECOURSE), "{text}");
            }
        }
    }

    /// Each definite payload pinned whole: the floor, the lever and the
    /// thinness each render as themselves, and the fold's lead follows
    /// its verdict.
    #[test]
    fn each_meter_payload_renders_its_own_numbers() {
        let rows = [
            (
                floor_zero(5e-10),
                "the face's normal cannot be proved non-zero (a chart 0.0000000005 m thin: \
                 normal length 0.000000001 m² per unit parameter area over speed 2 m), so it \
                 has no offset",
            ),
            (
                headroom(zero(5e-10)),
                "the offset distance's magnitude is within tolerance of the face's radius of \
                 curvature on the side it bends toward (0.001 m), so the offset may fold",
            ),
            (
                headroom(negative(-1e-4)),
                "the offset distance's magnitude passes the face's radius of curvature on the \
                 side it bends toward (0.001 m), so the offset folds over itself",
            ),
        ];
        for (error, want) in rows {
            assert_eq!(error.to_string(), want);
        }
    }

    /// **The ruling's prohibitions, over every arm and reading.** No
    /// ending advises lowering or loosening the tolerance; every ending
    /// names its meter's lever first; and the tighten appears exactly on
    /// a band-decided arm whose margin a smaller tolerance passes (a
    /// zero verdict or an undecided margin, positive), never on a
    /// sign-certain arm, a margin on the refused side, at zero,
    /// straddling zero, or poisoned — and never at adoption.
    #[test]
    fn no_meter_ending_lowers_or_tightens_where_the_ruling_forbids() {
        let mut rows = Vec::new();
        for meter in [Meter::NormalFloor, Meter::CurvatureHeadroom] {
            let lever = meter.decision().lever;
            let build = |m, verdict| match meter {
                Meter::NormalFloor => floor_of(m, verdict),
                Meter::CurvatureHeadroom => headroom(verdict),
            };
            for margin in [5e-10, 1e-10, 0.0, -0.0, -5e-10, -1e-3] {
                rows.push((build(margin, zero(margin)), lever, margin > 0.0));
                rows.push((build(margin, negative(margin)), lever, false));
            }
            let escalations = [
                (MarginDiag::value(5e-9), true),
                (MarginDiag::enclosure(2e-9, 4e-9), true),
                (MarginDiag::enclosure(-2e-9, 4e-9), false),
                (MarginDiag::value(-5e-9), false),
                (MarginDiag::value(0.0), false),
                (MarginDiag::INVALID, false),
            ];
            for (margin, tightens) in escalations {
                rows.push((escalated(meter, margin), lever, tightens));
            }
        }
        for (error, lever, tightens) in rows {
            for reading in [Reading::Build, Reading::AtRest, Reading::Adopt] {
                let ending = error.ending(reading);
                assert!(
                    ending.starts_with(&format!("Recourse: {lever}")),
                    "{error:?} at {reading:?} lacks its lever: {ending}"
                );
                for word in ["lower", "loosen"] {
                    assert!(!ending.contains(word), "{error:?}: {ending}");
                }
                assert_eq!(
                    ending.contains("tighten"),
                    tightens && reading != Reading::Adopt,
                    "{error:?} at {reading:?}: {ending}"
                );
            }
        }
    }

    /// Each meter carries its classified verdict, margin and band
    /// together: a margin inside the zero band refuses as
    /// [`Refused::Zero`], a definitely negative one as
    /// [`Refused::Negative`], and a positive one passes.
    #[test]
    fn each_meter_carries_its_verdict() {
        let band = band();
        // A lever of 2 makes the thinness half the floor, so a door that
        // put one where the other belongs reads differently.
        let reg = |floor| PatchRegularity {
            floor,
            sup: 4.0,
            speed_u: SupSpeed::new(2.0),
            speed_v: SupSpeed::new(1.0),
            sine_floor: floor / 2.0,
            cells: 1,
        };
        let coll = |headroom| PatchCollapse {
            kappa_lo: -1e3,
            kappa_hi: 0.0,
            reach: 1e-3,
            headroom,
        };
        let verdict = |e: MeterError| match e {
            MeterError::NormalFloor { verdict, .. }
            | MeterError::CurvatureHeadroom { verdict, .. } => Some(verdict),
            MeterError::Escalated { .. } => None,
        };
        for floor in [0.0, 1e-9] {
            let got = offset_normal_floor(&reg(floor), band).unwrap_err();
            assert_eq!(
                got,
                MeterError::NormalFloor {
                    floor,
                    speed_lever: 2.0,
                    verdict: zero(floor / 2.0),
                },
                "{floor}"
            );
        }
        assert!(offset_normal_floor(&reg(1.0), band).is_ok());
        for (margin, want) in [
            (5e-10, zero(5e-10)),
            (-5e-10, zero(-5e-10)),
            (-1e-3, negative(-1e-3)),
        ] {
            let got = offset_curvature_headroom(&coll(margin), band).unwrap_err();
            assert_eq!(verdict(got), Some(want), "{margin}");
        }
        assert!(offset_curvature_headroom(&coll(1e-3), band).is_ok());
        let escalation = offset_curvature_headroom(&coll(5e-9), band).unwrap_err();
        assert!(
            matches!(
                escalation,
                MeterError::Escalated {
                    meter: Meter::CurvatureHeadroom,
                    ..
                }
            ),
            "{escalation:?}"
        );
    }

    /// **A refused cell refuses the patch's chart speed.** One cell
    /// whose `S_u` enclosure is refused, beside a healthy one, in both
    /// orders: the fold answers NaN for `sup ‖S_u‖`, the lever NaN with
    /// it, and the predicate escalates. Red under the inherent
    /// `f64::max`, which returns the healthy cell's speed and so
    /// certifies a lever over the cells it could read.
    #[test]
    fn a_refused_cell_poisons_the_chart_speed_fold() {
        let p =
            |x: f64, y: f64, z: f64| [Interval::point(x), Interval::point(y), Interval::point(z)];
        let healthy = PatchCell {
            u: (0.0, 1.0),
            v: (0.0, 1.0),
            s_u: p(1.0, 0.0, 0.0),
            s_v: p(0.0, 1.0, 0.0),
            s_uu: p(0.0, 0.0, 0.0),
            s_uv: p(0.0, 0.0, 0.0),
            s_vv: p(0.0, 0.0, 0.0),
        };
        let refused = PatchCell {
            u: (1.0, 2.0),
            s_u: [
                Interval::refused(),
                Interval::point(0.0),
                Interval::point(0.0),
            ],
            ..healthy
        };
        for (order, cells) in [
            ("refused last", [healthy, refused]),
            ("refused first", [refused, healthy]),
        ] {
            let reg = patch_regularity(&cells);
            assert!(
                reg.speed_u.get().is_nan(),
                "{order}: sup ‖S_u‖ = {:e} dropped the refused cell",
                reg.speed_u.get()
            );
            assert!(
                reg.speed_v.get().is_finite(),
                "{order}: the healthy axis must stay readable"
            );
            assert!(
                reg.speed_lever().get().is_nan(),
                "{order}: the lever dropped the refused axis"
            );
            assert!(
                matches!(
                    offset_normal_floor(&reg, band()),
                    Err(MeterError::Escalated { .. })
                ),
                "{order}: a refused lever must escalate"
            );
        }
        // The lever folds the two axes on the type: a NaN on either
        // side survives, whichever side it is on.
        for (su, sv) in [(f64::NAN, 1.0), (1.0, f64::NAN)] {
            let reg = PatchRegularity {
                floor: 1.0,
                sup: 1.0,
                speed_u: SupSpeed::new(su),
                speed_v: SupSpeed::new(sv),
                sine_floor: 0.0,
                cells: 1,
            };
            assert!(
                reg.speed_lever().get().is_nan(),
                "speed_lever({su:e}, {sv:e}) dropped the NaN"
            );
        }
    }
}
