//! **Certified control-hull bounds on a NURBS patch's partials**, per
//! knot-span cell — the surface-side companion of
//! [`geom_core::spline::hull`]'s scalar span hulls.
//!
//! # The convexity fact, one dimension up
//!
//! For a NON-RATIONAL tensor-product B-spline
//! `S(u,v) = Σᵢⱼ Nᵢ(u)·Nⱼ(v)·Pᵢⱼ`, every partial is itself a
//! tensor-product B-spline whose coefficient net comes from knot
//! differencing per direction (The NURBS Book Eq. 3.24 — exactly
//! [`geom_core::spline::net::TensorNet::diff_u_knots`], which is
//! [`geom_core::spline::SplineCoeffs::derivative_coeffs`] iterated across the
//! net's lines; that iteration is the ONE spelling, here and in every
//! other tensor consumer). Both
//! bases are nonnegative partitions of unity, so on a knot-span cell
//! every value of a partial is a convex combination of the derived
//! coefficients active there and lies in their **signed** hull.
//!
//! # One reading
//!
//! Every cell reports **signed componentwise enclosures**
//! ([`PatchCell::s_u`] … [`PatchCell::s_vv`]) of the DESCRIBED patch
//! — on both arms and with no carve-out, because the refinement an arm
//! performs is itself part of the enclosure ([`PatchCell`], "What the
//! enclosure encloses"). An inf-side consumer
//! needs them as such — a magnitude sup cannot bound `‖S_u × S_v‖`
//! from below, because the cross product's sign structure is exactly
//! the information a magnitude throws away — and a sup-side consumer
//! reads a vector magnitude off them with [`sq_norm`], whose
//! `√hi` is a sup bound on the norm.
//!
//! **The signed reading is the only one**, and it is a reading of the
//! quotient rule itself: the true `−` signs, divided by the whole
//! weight hull. The cancellations that survive that are real — on a
//! quarter cylinder they are worth an order of magnitude on
//! `sup‖S_uv‖` — so a consumer wanting a magnitude takes [`sq_norm`]
//! of a signed enclosure and never a magnitude recurrence.
//!
//! # The rational arm
//!
//! A RATIONAL patch is `S = A/w` with `A = ΣΣ Nᵢ Nⱼ wᵢⱼ Pᵢⱼ` and
//! `w = ΣΣ Nᵢ Nⱼ wᵢⱼ` — both POLYNOMIAL tensor-product B-splines, so
//! every ingredient is the same control-hull fact taken on the
//! homogeneous nets. With `Ã = A − c·w` for a cell-local centre `c`
//! (so `S − c = Ã/w`, and knot differencing is linear:
//! `d(A − c·w) = dA − c·dw`), the quotient rule — exactly
//! `NurbsSurface::ders_in_span`'s corrections — reads
//!
//! ```text
//! S_u  = (Ã_u  − (S − c)·w_u) / w                        (v symmetric)
//! S_uu = (Ã_uu − 2·S_u·w_u − (S − c)·w_uu) / w           (v symmetric)
//! S_uv = (Ã_uv − S_u·w_v − S_v·w_u − (S − c)·w_uv) / w
//! ```
//!
//! **The divisor is the cell's weight hull, argued not assumed.** On
//! the cell `w` is a convex combination of the active weights, so
//! `w ∈ [w_min, w_max]`; interval arithmetic's division refuses a zero-touching
//! divisor, so a net whose positivity was never proven poisons rather
//! than answering.
//!
//! **Recentring keeps the cross terms cell-sized**: with the cell's
//! control centroid as `c`, `sup|S − c|` is a cell-of-control-net
//! fact rather than a whole-patch one, so `(S − c)·w_dd` does not
//! inflate with the patch's distance from the origin.
//!
//! A degree-1 direction's `Ã_dd` and `w_dd` are exactly zero, but its
//! CROSS terms survive — a rational degree-1 direction genuinely
//! curves in parameter — and the recurrences carry that.
//!
//! **The refinement the arm performs first is itself part of the
//! enclosure.** The homogeneous nets are refined IN INTERVAL ARITHMETIC from ring
//! points of the described net
//! ([`geom_core::spline::net::TensorNet::refine_u`]), each Boehm ratio
//! an outward-rounded quotient of the knots it is made of, so what the
//! cells enclose is the described patch and not a rounded neighbour of
//! it. There is no refined `f64` surface on this arm: the cell extents
//! come from the refined knot vectors, which are exact (the inserted
//! knots are the `f64`s the schedule chose), and a refined control
//! point is interval arithmetic quotient `A / w`.
//!
//! # Conservatism
//!
//! The answer is a bound, not an estimate. Ordinary walls measure
//! within a small factor of the true sup; extreme weight ratios can
//! leave it orders above, because the product terms lose the sign
//! correlation a steep ramp lives in. The cost is only how finely a
//! consumer must subdivide; the bound is never wrong.
//!
//! # Poison (fail-loud, D4 ¶2)
//!
//! Structural refusals are typed ([`PatchBoundError`]); arithmetic
//! failures are refusals, and a poisoned hull fails every `≤ ε`
//! comparison it reaches.

use geom_core::Bounds;
use std::ops::RangeInclusive;

use geom::surfaces::NurbsSurface;
use geom_core::interval::Interval;
use geom_core::spline::net::TensorNet;
use geom_core::spline::{CurvePlan, KnotVector};

/// The fixed refinement schedule of the RATIONAL arm: every nonempty
/// span of every direction splits into this many equal pieces before
/// the per-cell assembly. A CONSTANT (D9: structure, never a
/// data-dependent iteration) — the `RATIONAL_METER_SPLITS = 16`
/// precedent of `geom::curves`' rational speed meter, mirrored. Knot
/// insertion is evaluation-invariant in ℝ, so it changes no geometry;
/// it only shrinks every hull the bound is assembled from, which is
/// what keeps the `sup‖S − c‖·sup|w_dd|` cross terms cell-sized.
///
/// **The schedule is not free of the arithmetic, which is why it is
/// applied in certification arithmetic.** Each insertion the count buys is one more
/// affine combination, and the count therefore also sets how much
/// outward rounding the refined net carries. That width grows with the
/// NUMBER of insertions rather than by a factor per insertion, which is
/// what makes 16 affordable ([`geom_core::spline::CurvePlan::apply_ring`]
/// argues the form that buys it).
pub const RATIONAL_CERT_SPLITS: usize = 16;

/// A typed refusal of the patch-bound assembly (fail-loud). Each
/// carries the prose its consumers print, so a lifted consumer's
/// message is this module's message.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PatchBoundError {
    /// A degree-0 direction — a degenerate patch description.
    DegreeZero,
    /// A degree-1 direction carrying interior knots: a C⁰ crease.
    Degree1Crease,
    /// A direction whose interior multiplicity equals its degree: a
    /// C⁰ crease.
    Crease,
    /// A rational description with a non-positive or non-finite
    /// weight — the convex-combination licence never held.
    NonPositiveWeight,
    /// The same, discovered after the fixed rational refinement.
    RefinedWeightLostPositivity,
    /// The fixed rational refinement failed to materialise.
    RefinementFailed,
    /// A direction whose once-differenced knot vector failed to
    /// materialise.
    DerivedKnots,
}

impl PatchBoundError {
    /// The refusal's prose — one spelling, shared by every consumer
    /// that reports a patch-bound refusal in its own error type.
    pub fn note(self) -> &'static str {
        match self {
            Self::DegreeZero => {
                "degree-0 NURBS direction (a degenerate face description) — a degree-0 \
                 locus is a step function rather than a surface direction, and the form \
                 is a designed absence: describe the direction at degree 1 or above"
            }
            Self::Degree1Crease => {
                "degree-1 NURBS direction with interior knots (a C⁰ crease) — \
                 the interpolation Taylor bound needs C¹; split the face at \
                 the crease"
            }
            Self::Crease => {
                "NURBS direction with a C⁰ crease (interior multiplicity = \
                 degree) — the interpolation Taylor bound needs C¹; split \
                 the face at the crease"
            }
            Self::NonPositiveWeight => {
                "rational NURBS face with a non-positive or non-finite weight — an \
                 illegal rational description: the convex-combination licence every \
                 hull fact rests on requires strictly positive weights, so supply them \
                 and the face certifies through the rational arm. The door that mints a \
                 NURBS surface refuses these already, so a face that reaches this bound \
                 carrying one is worth reporting too"
            }
            Self::RefinedWeightLostPositivity => {
                "rational NURBS face whose refined weight ENCLOSURE reaches zero — outside \
                 the certified inventory: positivity survives knot insertion in ℝ, and the \
                 refinement's two barycentric ratios are both non-negative, so no weight \
                 RATIO can reach this; what does is a weight so small that its product with \
                 a ratio UNDERFLOWS to zero, which needs a subnormal near the bottom of the \
                 f64 range. Describe the face at a weight scale f64 can hold — scaling \
                 every weight by one constant describes the same surface — or report the \
                 description"
            }
            Self::RefinementFailed => {
                "NURBS face whose refinement fails to materialise — outside the certified \
                 inventory: the fixed schedule inserts knots into a direction that already \
                 passed the C¹ gate, and insertion into a valid clamped vector is total, \
                 so report the description that reached this rather than repairing one"
            }
            Self::DerivedKnots => {
                "NURBS direction whose derivative knot vector fails to materialise — \
                 outside the certified inventory: a direction that passed the C¹ gate has \
                 a valid once-differenced vector, so report the description that reached \
                 this rather than repairing one"
            }
        }
    }
}

impl core::fmt::Display for PatchBoundError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.note())
    }
}

impl core::error::Error for PatchBoundError {}

/// One knot-span cell's certified bounds on the patch's partials, with
/// the UV rectangle they hold on.
///
/// # What the enclosure encloses
///
/// **The DESCRIBED patch, on both arms, with no carve-out.** Refinement
/// is part of the enclosure: where an arm inserts knots it inserts them
/// into certification enclosures of the described homogeneous net
/// ([`geom_core::spline::net::TensorNet::refine_u`]), each Boehm ratio
/// an outward-rounded quotient of the knots it is made of, so the
/// insertion widens like every later step instead of rounding the net
/// to a nearby one.
///
/// That makes a STRUCTURAL predicate sound to read off these cells. A
/// component whose true value is identically zero comes back as an
/// enclosure CONTAINING zero on every cell, which is what a
/// `contains(0)` or an exact-sign test needs and what a bound on a
/// nearby patch could never give. The witnessed case is a quarter
/// cylinder's `S_vv`: degree 1 in `v` with the weights CONSTANT along
/// `v` and the control points affine in it, which together make the
/// described surface affine in `v`. Degree 1 alone does not — a
/// degree-1 direction whose weights vary is a Möbius
/// reparameterization and genuinely curves in parameter, as the module
/// header says of the cross terms.
///
/// What it costs is width, and the width is the honest one. On that
/// same ruled face the zero `S_vv` is reported as dust at the scale the
/// insertion contributed rather than as an exact zero, because that is
/// all an enclosure of the refinement can say. A consumer sizing a grid
/// pays that in the last decades of a bound; a consumer asking whether
/// zero is in there gets the right answer.
#[derive(Clone, Copy, Debug)]
pub struct PatchCell {
    /// The cell's `u` extent, `[lo, hi]`.
    pub u: (f64, f64),
    /// The cell's `v` extent, `[lo, hi]`.
    pub v: (f64, f64),
    /// Signed componentwise enclosure of `S_u` on the cell — of the
    /// DESCRIBED patch (see the type's docs, "What the enclosure
    /// encloses").
    pub s_u: [Interval; 3],
    /// Signed componentwise enclosure of `S_v` on the cell.
    pub s_v: [Interval; 3],
    /// Signed componentwise enclosure of `S_uu` on the cell.
    pub s_uu: [Interval; 3],
    /// Signed componentwise enclosure of `S_uv` on the cell.
    pub s_uv: [Interval; 3],
    /// Signed componentwise enclosure of `S_vv` on the cell.
    pub s_vv: [Interval; 3],
}

/// Whether a patch is rational under the kernel's definition (any
/// weight not bitwise `1.0`) — the arm selection, on f64 STRUCTURE.
pub fn is_rational(n: &NurbsSurface<f64>) -> bool {
    n.weights().iter().any(|w| *w != 1.0)
}

/// **The per-cell certified bounds** on a described NURBS patch's
/// first and second partials (module docs).
///
/// Cell granularity is the arm's own: the integral arm reports the
/// raw knot-span cells, the rational arm the cells of the fixed
/// [`RATIONAL_CERT_SPLITS`] refinement.
///
/// # Errors
///
/// [`PatchBoundError`] — a C⁰-creased or degree-0 direction, an
/// illegal rational description, or a refinement/derived-knot
/// construction that fails to materialise.
pub fn patch_cells(n: &NurbsSurface<f64>) -> Result<Vec<PatchCell>, PatchBoundError> {
    check_direction(n.knots_u())?;
    check_direction(n.knots_v())?;
    if is_rational(n) {
        rational_cells(n, RATIONAL_CERT_SPLITS)
    } else {
        integral_cells(n)
    }
}

/// [`patch_cells`] on an EXPLICIT refinement schedule: every nonempty
/// span of every direction is cut into `splits` equal pieces first,
/// in **both** arms.
///
/// Why a consumer would ask for more than [`patch_cells`] gives: a
/// B-spline coefficient hull over one knot span covers the whole
/// `(p + 1)`-span support of the basis functions active there, so its
/// width shrinks only LINEARLY in the span size, with a constant
/// `p + 1` times the cell. A sup-side consumer barely notices; an
/// inf-side one does, because the width is subtracted from the
/// quantity it is trying to prove positive. The measured case is the
/// exactly-umbilic sphere band: at 16 splits the first fundamental
/// form's own `F` — identically zero on an orthogonal chart —
/// encloses as `[−0.82, 0.81]`, and the certified curvature range
/// comes out twenty times wider than the surface's single value.
/// Refinement is exact in ℝ, so this buys tightness and nothing else.
///
/// # Errors
///
/// As [`patch_cells`], plus [`PatchBoundError::RefinementFailed`].
pub fn patch_cells_refined(
    n: &NurbsSurface<f64>,
    splits: usize,
) -> Result<Vec<PatchCell>, PatchBoundError> {
    check_direction(n.knots_u())?;
    check_direction(n.knots_v())?;
    if is_rational(n) {
        rational_cells(n, splits)
    } else {
        integral_cells_refined(n, splits)
    }
}

/// The refinement schedule of one direction and the knot vector it
/// lands on: the plan chain that cuts every nonempty span into `splits`
/// equal pieces, built from STRUCTURE alone
/// ([`geom_core::spline::algebra::refine_plan_homogeneous`] — the
/// homogeneous nets this module refines are polynomial, so their weights
/// are unit).
///
/// One schedule, two arithmetics: this is the same plan the `f64`
/// surface refinement applies through
/// [`geom_core::spline::CurvePlan::apply_points`], and interval arithmetic applier
/// re-derives each insertion ratio from the knots it is made of instead
/// of widening the plan's `f64` `λ`.
///
/// # Errors
///
/// [`PatchBoundError::RefinementFailed`] — insertion into a direction
/// that already passed the C¹ gate is total, so a refusal here is a
/// description worth reporting rather than one to repair.
fn refine_chain(
    kv: &KnotVector,
    splits: usize,
) -> Result<(KnotVector, Vec<CurvePlan>), PatchBoundError> {
    let plans = geom_core::spline::algebra::refine_plan_homogeneous(kv, &split_points(kv, splits))
        .map_err(|_| PatchBoundError::RefinementFailed)?;
    let refined = plans
        .last()
        .map_or_else(|| kv.clone(), |p| p.knots().clone());
    Ok((refined, plans))
}

/// The C¹ gate per direction: degree 0 refuses; degree 1 must be
/// single-span (an interior knot is a C⁰ crease); degree ≥ 2 needs
/// interior multiplicities ≤ p − 1.
///
/// # Errors
///
/// [`PatchBoundError::DegreeZero`], [`PatchBoundError::Degree1Crease`],
/// [`PatchBoundError::Crease`].
pub fn check_direction(kv: &KnotVector) -> Result<(), PatchBoundError> {
    let p = kv.degree();
    if p == 0 {
        return Err(PatchBoundError::DegreeZero);
    }
    if p == 1 {
        return if kv.interior_knots().next().is_none() {
            Ok(())
        } else {
            Err(PatchBoundError::Degree1Crease)
        };
    }
    if kv.interior_knots().any(|(_, m)| m > p - 1) {
        return Err(PatchBoundError::Crease);
    }
    Ok(())
}

/// The once-differenced knot vector (drop the outer knot pair, degree
/// − 1).
///
/// # Errors
///
/// [`PatchBoundError::DerivedKnots`] when the result is not a valid
/// clamped vector.
pub fn derived_knots(kv: &KnotVector) -> Result<KnotVector, PatchBoundError> {
    let inner = kv.derivative_knot_slice().to_vec();
    KnotVector::clamped(inner, kv.degree() - 1).map_err(|_| PatchBoundError::DerivedKnots)
}

/// The interior split points of the fixed rational refinement
/// schedule for one knot vector ([`RATIONAL_CERT_SPLITS`] equal
/// pieces per nonempty span), skipping any split point floating point
/// collapses onto a span end — refinement is a tightening, never a
/// correctness condition.
pub fn rational_split_points(kv: &KnotVector) -> Vec<f64> {
    split_points(kv, RATIONAL_CERT_SPLITS)
}

/// **Near-twin, recorded and deliberately not unified**:
/// `geom_brep::props::quad`'s `knot_aligned_cuts` builds the same
/// concept for the rational patch-flux composite — a knot-aligned
/// subdivision of a parameter range, with its own sliver guard — and
/// arrived at the same sliver lesson independently. Unifying the two
/// is Track R's consolidation ground (C-m/D30, gated behind #723),
/// not either caller's.
///
/// The interior split points that cut every nonempty span of `kv`
/// into `splits` equal pieces, skipping any point floating point
/// collapses onto a span end — refinement is a tightening, never a
/// correctness condition (the speed meter's rule, verbatim).
pub fn split_points(kv: &KnotVector, splits: usize) -> Vec<f64> {
    let mut add = Vec::new();
    for span in kv.first_span()..=kv.last_span() {
        if !kv.span_is_nonempty(span) {
            continue;
        }
        let (Some(&lo), Some(&hi)) = (kv.knots().get(span), kv.knots().get(span + 1)) else {
            continue;
        };
        for k in 1..splits {
            #[allow(clippy::cast_precision_loss)]
            let f = k as f64 / splits as f64;
            let u = lo + (hi - lo) * f;
            if u > lo && u < hi {
                add.push(u);
            }
        }
    }
    add
}

/// A coefficient net as certification enclosures — the shared tensor assembly,
/// homed in [`geom_core::spline::net`] (issue 1006). The alias is kept
/// so this module's own prose and its consumers keep naming the thing
/// they read; the differencing is not this module's any more.
pub type Net = TensorNet;

/// The signed hull of `a[i][j] − c·w[i][j]` over the window
/// `wu × wv` — the recentred homogeneous net `Ã = A − c·w` read
/// through the linearity of knot differencing (`d(A − c·w) = dA −
/// c·dw`, entrywise, same knots). Out-of-range indices poison.
///
/// This module's own READING, not the shared assembly: no other
/// consumer of a tensor net recentres at the hull read, because no
/// other one has a cell-local centre to recentre against.
///
/// Fixed association (D9): `u`-major, `i` outer and `j` inner, hulled
/// left to right — [`TensorNet::window_hull`]'s order, so the two
/// readings of one net agree on which coefficient came first.
pub fn window_tilde_hull(
    a: &Net,
    w: &Net,
    c: Interval,
    wu: &RangeInclusive<usize>,
    wv: &RangeInclusive<usize>,
) -> Interval {
    let mut acc: Option<Interval> = None;
    for i in wu.clone() {
        for j in wv.clone() {
            let e = a.get(i, j) - c * w.get(i, j);
            acc = Some(match acc {
                None => e,
                Some(h) => Interval::hull(h, e),
            });
        }
    }
    acc.unwrap_or_else(Interval::poison)
}

/// The signed hull of `net[i][j]` over the window `wu × wv` —
/// [`TensorNet::window_hull`], re-exported under the name this
/// module's consumers already use.
///
/// Distinct from [`window_tilde_hull`] with a zero centre on purpose:
/// that spelling computes `a − 0·w`, and interval arithmetic's outward rounding
/// makes the subtraction widen the answer by an ulp — enough to put a
/// CELL's bound above the whole-patch hull it is a subset of.
pub fn window_hull(net: &Net, wu: &RangeInclusive<usize>, wv: &RangeInclusive<usize>) -> Interval {
    net.window_hull(wu, wv)
}

/// **The squared-sum collapse of one signed componentwise enclosure**:
/// `sum over c of sup squared`, whose `sqrt(hi)` is a sup bound on the
/// vector's norm. One spelling, consumed wherever a vector partial's
/// magnitude is read off its signed enclosure.
///
/// Fixed association (D9): channel order `x, y, z`, accumulated left
/// to right from interval arithmetic zero. Poison in one channel poisons the sum.
#[must_use]
pub fn sq_norm(v: [Interval; 3]) -> Interval {
    v.iter().fold(Interval::zero(), |acc, c| acc + c.sqr())
}

/// A span's `[knot, next knot]` extent (the caller has already
/// established the span is nonempty, so both knots exist).
fn span_extent(kv: &KnotVector, span: usize) -> (f64, f64) {
    let k = kv.knots();
    (
        k.get(span).copied().unwrap_or(f64::NAN),
        k.get(span + 1).copied().unwrap_or(f64::NAN),
    )
}

/// The three spatial channels of a control net, as enclosure points.
///
/// **The SHAPE is shared** with `offset_fit::channel`: both build a
/// [`Net`], and the flat/nested bridge the two used to need is gone —
/// [`geom_core::spline::net::TensorNet`] is row-major and hands out
/// both a flat slice (what `PatchSpans::decompose` consumes) and
/// indexed windows (what [`window_hull`] reads).
///
/// **The ARITHMETIC still diverges, and that is what is left.** This
/// one extracts `w·P`; `offset_fit::channel` extracts `w·(P − c)`
/// against a WHOLE-PATCH recentring origin, because its net feeds
/// polynomial products formed once over the merged break structure,
/// where interval arithmetic's rounding scales with the coordinate. This site
/// recentres too, but LATER and per cell ([`window_tilde_hull`]), off
/// the cell's own control window — the tighter centre, available here
/// because a cell-local hull is what is being read. So a change to one
/// is not automatically a change to both. What they still share is the
/// ORDER (`weight · coordinate`), and a change to THAT is a change to
/// both.
///
/// **Unifying the two CENTRES is open, and it is not this seam's own
/// to close.** The patch-hull consolidation (issue 1006) unified the
/// storage and left the centres deliberately apart: they are different
/// centres because they are read at different granularities, and
/// making them one means deciding whether the composite lane can
/// afford a per-cell centre or the cell lane must give up its tighter
/// one — a measurement on `offset_fit`'s numbers, not a refactor. It
/// is filed rather than assigned here, because a residue whose owner
/// is the unit that chose to keep it has no owner at all.
fn comp_nets(n: &NurbsSurface<f64>, weighted: bool) -> Vec<Net> {
    let (nu, nv) = n.control_counts();
    (0..3)
        .map(|c| {
            Net::from_fn(nu, nv, |i, j| {
                // Row-major layout: control[iu·nv + iv] — the net's own.
                let p = n.control()[i * nv + j];
                let x = Interval::point(match c {
                    0 => p.x,
                    1 => p.y,
                    _ => p.z,
                });
                if weighted {
                    Interval::point(n.weights()[i * nv + j]) * x
                } else {
                    x
                }
            })
        })
        .collect()
}

/// The five per-direction derivative nets one channel needs.
struct DNets {
    d10: Net,
    d01: Net,
    d11: Net,
    d20: Option<Net>,
    d02: Option<Net>,
}

impl DNets {
    fn build(
        base: &Net,
        kv_u: &KnotVector,
        kv_v: &KnotVector,
        kv_u1: Option<&KnotVector>,
        kv_v1: Option<&KnotVector>,
    ) -> Self {
        let d10 = base.diff_u_knots(kv_u);
        let d01 = base.diff_v_knots(kv_v);
        let d11 = d10.diff_v_knots(kv_v);
        let d20 = kv_u1.map(|k1| d10.diff_u_knots(k1));
        let d02 = kv_v1.map(|k1| d01.diff_v_knots(k1));
        Self {
            d10,
            d01,
            d11,
            d20,
            d02,
        }
    }
}

/// The active windows of one cell, both directions, all three orders.
struct CellWindows {
    u_val: RangeInclusive<usize>,
    v_val: RangeInclusive<usize>,
    u_d1: RangeInclusive<usize>,
    v_d1: RangeInclusive<usize>,
    u_d2: Option<RangeInclusive<usize>>,
    v_d2: Option<RangeInclusive<usize>>,
}

/// Assembles a cell from the five signed componentwise enclosures
/// (`S_u, S_v, S_uu, S_uv, S_vv`, in that order).
fn cell_from(uv: ((f64, f64), (f64, f64)), signed: [[Interval; 3]; 5]) -> PatchCell {
    PatchCell {
        u: uv.0,
        v: uv.1,
        s_u: signed[0],
        s_v: signed[1],
        s_uu: signed[2],
        s_uv: signed[3],
        s_vv: signed[4],
    }
}

/// The INTEGRAL arm (all weights bitwise `1.0`): the plain hull
/// assembly on the spatial nets — no quotient rule intervenes, so the
/// enclosure IS the coefficient hull.
fn integral_cells(n: &NurbsSurface<f64>) -> Result<Vec<PatchCell>, PatchBoundError> {
    integral_cells_on(&comp_nets(n, false), n.knots_u(), n.knots_v())
}

/// [`integral_cells`] after refining every nonempty span into `splits`
/// equal pieces, IN INTERVAL ARITHMETIC: an integral net's weights are unit, so the
/// net is already homogeneous and [`refine_chain`]'s schedule applies to
/// it directly. The cells therefore enclose the described patch, where an
/// `f64` refinement would have them enclose the refined-`f64` one.
///
/// # Errors
///
/// As [`integral_cells`], plus [`PatchBoundError::RefinementFailed`].
fn integral_cells_refined(
    n: &NurbsSurface<f64>,
    splits: usize,
) -> Result<Vec<PatchCell>, PatchBoundError> {
    let (kv_u, plans_u) = refine_chain(n.knots_u(), splits)?;
    let (kv_v, plans_v) = refine_chain(n.knots_v(), splits)?;
    let nets: Vec<Net> = comp_nets(n, false)
        .iter()
        .map(|net| net.refine_u(&plans_u).refine_v(&plans_v))
        .collect();
    integral_cells_on(&nets, &kv_u, &kv_v)
}

/// The integral arm's per-cell assembly over ALREADY-BUILT spatial nets
/// and their directions — the one body [`integral_cells`] and
/// [`integral_cells_refined`] share, so refinement changes what is
/// assembled and nothing about how.
///
/// # Errors
///
/// [`PatchBoundError::DerivedKnots`].
fn integral_cells_on(
    base_nets: &[Net],
    kv_u: &KnotVector,
    kv_v: &KnotVector,
) -> Result<Vec<PatchCell>, PatchBoundError> {
    let kv_u1 = (kv_u.degree() >= 2)
        .then(|| derived_knots(kv_u))
        .transpose()?;
    let kv_v1 = (kv_v.degree() >= 2)
        .then(|| derived_knots(kv_v))
        .transpose()?;
    let nets: Vec<DNets> = base_nets
        .iter()
        .map(|base| DNets::build(base, kv_u, kv_v, kv_u1.as_ref(), kv_v1.as_ref()))
        .collect();
    let zero = Interval::zero();
    let mut cells = Vec::new();
    for su in kv_u.first_span()..=kv_u.last_span() {
        let Some(span_u) = kv_u.span(su) else {
            continue;
        };
        for sv in kv_v.first_span()..=kv_v.last_span() {
            let Some(span_v) = kv_v.span(sv) else {
                continue;
            };
            let w = CellWindows {
                u_val: span_u.window(),
                v_val: span_v.window(),
                u_d1: span_u.first_derived_window(),
                v_d1: span_v.first_derived_window(),
                u_d2: span_u.derived_window(2),
                v_d2: span_v.derived_window(2),
            };
            let mut s_u = [zero; 3];
            let mut s_v = [zero; 3];
            let mut s_uu = [zero; 3];
            let mut s_uv = [zero; 3];
            let mut s_vv = [zero; 3];
            for (c, d) in nets.iter().enumerate() {
                let g20 = d
                    .d20
                    .as_ref()
                    .zip(w.u_d2.as_ref())
                    .map_or(zero, |(net, wu2)| window_hull(net, wu2, &w.v_val));
                let g02 = d
                    .d02
                    .as_ref()
                    .zip(w.v_d2.as_ref())
                    .map_or(zero, |(net, wv2)| window_hull(net, &w.u_val, wv2));
                let g11 = window_hull(&d.d11, &w.u_d1, &w.v_d1);
                let g10 = window_hull(&d.d10, &w.u_d1, &w.v_val);
                let g01 = window_hull(&d.d01, &w.u_val, &w.v_d1);
                s_uu[c] = g20;
                s_vv[c] = g02;
                s_uv[c] = g11;
                s_u[c] = g10;
                s_v[c] = g01;
            }
            cells.push(cell_from(
                (span_extent(kv_u, su), span_extent(kv_v, sv)),
                [s_u, s_v, s_uu, s_uv, s_vv],
            ));
        }
    }
    Ok(cells)
}

/// The RATIONAL arm: the quotient-rule assembly over the homogeneous
/// nets, on the cells of the fixed [`RATIONAL_CERT_SPLITS`]
/// refinement (module docs).
#[allow(clippy::too_many_lines)]
fn rational_cells(n: &NurbsSurface<f64>, splits: usize) -> Result<Vec<PatchCell>, PatchBoundError> {
    // The convex-combination licence, on f64 STRUCTURE. `!(w > 0.0)`
    // catches NaN. (`NurbsSurface::new` refuses these at the door;
    // re-checked here so THIS bound never divides by an unproven
    // denominator.)
    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    if n.weights().iter().any(|w| !(*w > 0.0) || !w.is_finite()) {
        return Err(PatchBoundError::NonPositiveWeight);
    }
    // THE REFINEMENT IS PART OF THE ENCLOSURE. The homogeneous nets `w`
    // and `w·P` are refined IN INTERVAL ARITHMETIC from point intervals of the
    // DESCRIBED net, so insertion widens outward like every later step
    // and the cells enclose the described patch. An `f64` refinement
    // here would make them enclose the refined-`f64` patch instead, and
    // the described one escapes that by insertion rounding amplified by
    // the knot differencing.
    let (kv_u, plans_u) = refine_chain(n.knots_u(), splits)?;
    let (kv_v, plans_v) = refine_chain(n.knots_v(), splits)?;
    let (kv_u, kv_v) = (&kv_u, &kv_v);
    let (pu, pv) = (kv_u.degree(), kv_v.degree());
    let (nu0, nv0) = n.control_counts();
    let refine = |net: &Net| net.refine_u(&plans_u).refine_v(&plans_v);
    let w_grid = refine(&Net::from_fn(nu0, nv0, |i, j| {
        Interval::point(n.weights()[i * nv0 + j])
    }));
    let (nu, nv) = (w_grid.nu(), w_grid.nv());
    // Positivity survives insertion in ℝ (convex combinations); this
    // code may not assume the ARITHMETIC proved it, so the refined
    // licence is read off the enclosure's own `lo` — a weight hull that
    // touches or straddles zero voids the convex-combination licence
    // just as a described non-positive weight does, and poison is not a
    // proof of positivity either.
    for i in 0..nu {
        for j in 0..nv {
            let w = w_grid.get(i, j);
            #[allow(clippy::neg_cmp_op_on_partial_ord)]
            if !w.is_certified() || !(w.lo() > 0.0) || !w.lo().is_finite() {
                return Err(PatchBoundError::RefinedWeightLostPositivity);
            }
        }
    }
    // Second derivatives along a degree-1 direction are EXACTLY zero
    // in ℝ for the polynomial nets A and w (the direction is a single
    // linear span pre-refinement — the C¹ gate — and refinement's
    // inserted knots are removable), so those nets are `None` and
    // their terms exact zeros; the CROSS terms stay.
    let kv_u1 = (pu >= 2).then(|| derived_knots(kv_u)).transpose()?;
    let kv_v1 = (pv >= 2).then(|| derived_knots(kv_v)).transpose()?;
    let w_nets = DNets::build(&w_grid, kv_u, kv_v, kv_u1.as_ref(), kv_v1.as_ref());
    let a_base: Vec<Net> = comp_nets(n, true).iter().map(refine).collect();
    let a_nets: Vec<DNets> = a_base
        .iter()
        .map(|g| DNets::build(g, kv_u, kv_v, kv_u1.as_ref(), kv_v1.as_ref()))
        .collect();
    // The refined control points, `P = A / w` per channel, ONCE for the
    // whole net. Each is read by every cell whose window covers it —
    // `(pu + 1)(pv + 1)` cells at the interior, twice over (the centroid
    // and the value hull) — so computing it per cell paid the same ring
    // division up to `2(pu + 1)(pv + 1)` times for one coefficient.
    // Entrywise, so each point is enclosed at its own weight rather than
    // at the cell's weight hull.
    let p_nets: Vec<Net> = a_base
        .iter()
        .map(|a| Net::from_fn(nu, nv, |i, j| a.get(i, j) / w_grid.get(i, j)))
        .collect();
    let zero = Interval::zero();
    let two = Interval::point(2.0);
    let mut cells: Vec<PatchCell> = Vec::new();
    for su in kv_u.first_span()..=kv_u.last_span() {
        for sv in kv_v.first_span()..=kv_v.last_span() {
            // Emptiness skip and span validation, both directions. The
            // spans come from the REFINED knot vectors rather than from
            // a refined surface: there is no refined `f64` surface on
            // this arm any more, only refined enclosure nets.
            let (Some(span_u), Some(span_v)) = (kv_u.span(su), kv_v.span(sv)) else {
                continue;
            };
            let w = CellWindows {
                u_val: span_u.window(),
                v_val: span_v.window(),
                u_d1: span_u.first_derived_window(),
                v_d1: span_v.first_derived_window(),
                u_d2: span_u.derived_window(2),
                v_d2: span_v.derived_window(2),
            };
            let point_at = |comp: usize, i: usize, j: usize| {
                p_nets
                    .get(comp)
                    .map_or_else(Interval::poison, |p| p.get(i, j))
            };
            // The cell centroid — a translation CHOICE, so ANY finite
            // value is sound and none of it has to be enclosed. Taken
            // from the midpoint of each enclosed control point, in a
            // fixed order; a non-finite midpoint (an overflowed or
            // poisoned enclosure) contributes `0`, which is still a
            // finite centre and leaves the widened hulls to report the
            // trouble.
            let mut c = [0.0f64; 3];
            for (comp, slot) in c.iter_mut().enumerate() {
                let mut sum = 0.0f64;
                let mut count = 0.0f64;
                for i in *w.u_val.start()..=*w.u_val.end() {
                    for j in *w.v_val.start()..=*w.v_val.end() {
                        let p = point_at(comp, i, j);
                        let mid = (p.lo() + p.hi()) / 2.0;
                        sum += if mid.is_finite() { mid } else { 0.0 };
                        count += 1.0;
                    }
                }
                *slot = sum / count;
            }
            // The cell's weight hull — the divisor (module docs).
            let w_cell = window_hull(&w_grid, &w.u_val, &w.v_val);
            // Weight-net hulls on the cell.
            let w10s = window_hull(&w_nets.d10, &w.u_d1, &w.v_val);
            let w01s = window_hull(&w_nets.d01, &w.u_val, &w.v_d1);
            let w11s = window_hull(&w_nets.d11, &w.u_d1, &w.v_d1);
            let w20s = w_nets
                .d20
                .as_ref()
                .zip(w.u_d2.as_ref())
                .map_or(zero, |(net, wu2)| window_hull(net, wu2, &w.v_val));
            let w02s = w_nets
                .d02
                .as_ref()
                .zip(w.v_d2.as_ref())
                .map_or(zero, |(net, wv2)| window_hull(net, &w.u_val, wv2));
            let mut s_u = [zero; 3];
            let mut s_v = [zero; 3];
            let mut s_uu = [zero; 3];
            let mut s_uv = [zero; 3];
            let mut s_vv = [zero; 3];
            for (comp, a) in a_nets.iter().enumerate() {
                let cc = Interval::point(c[comp]);
                // The rational VALUE hull on the cell: positive
                // weights make the rational basis a nonnegative
                // partition of unity over the ACTIVE control points,
                // so `S − c` lies in the hull of `P − c`.
                let mut v0h: Option<Interval> = None;
                for i in *w.u_val.start()..=*w.u_val.end() {
                    for j in *w.v_val.start()..=*w.v_val.end() {
                        let e = point_at(comp, i, j) - cc;
                        v0h = Some(match v0h {
                            None => e,
                            Some(h) => Interval::hull(h, e),
                        });
                    }
                }
                let v0s = v0h.unwrap_or_else(Interval::poison);
                // Recentred homogeneous derivative hulls
                // `Ã_kl = A_kl − c·w_kl` on the cell.
                let at =
                    |an: &Net, wn: &Net, wu: &RangeInclusive<usize>, wv: &RangeInclusive<usize>| {
                        window_tilde_hull(an, wn, cc, wu, wv)
                    };
                let a10s = at(&a.d10, &w_nets.d10, &w.u_d1, &w.v_val);
                let a01s = at(&a.d01, &w_nets.d01, &w.u_val, &w.v_d1);
                let a11s = at(&a.d11, &w_nets.d11, &w.u_d1, &w.v_d1);
                let a20s = match (a.d20.as_ref(), w_nets.d20.as_ref(), w.u_d2.as_ref()) {
                    (Some(an), Some(wn), Some(wu2)) => at(an, wn, wu2, &w.v_val),
                    _ => zero,
                };
                let a02s = match (a.d02.as_ref(), w_nets.d02.as_ref(), w.v_d2.as_ref()) {
                    (Some(an), Some(wn), Some(wv2)) => at(an, wn, &w.u_val, wv2),
                    _ => zero,
                };
                // The quotient rule itself, in certification arithmetic, divided by
                // the whole weight hull.
                let s1u = (a10s - v0s * w10s) / w_cell;
                let s1v = (a01s - v0s * w01s) / w_cell;
                s_u[comp] = s1u;
                s_v[comp] = s1v;
                s_uu[comp] = (a20s - two * s1u * w10s - v0s * w20s) / w_cell;
                s_vv[comp] = (a02s - two * s1v * w01s - v0s * w02s) / w_cell;
                s_uv[comp] = (a11s - s1u * w01s - s1v * w10s - v0s * w11s) / w_cell;
            }
            cells.push(cell_from(
                (span_extent(kv_u, su), span_extent(kv_v, sv)),
                [s_u, s_v, s_uu, s_uv, s_vv],
            ));
        }
    }
    Ok(cells)
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;

    /// **`RefinedWeightLostPositivity` is reachable, and only by
    /// underflow.** The arm's prose claims a weight RATIO cannot reach
    /// it, which is a claim about the refinement's arithmetic: both
    /// barycentric ratios are non-negative, so a convex combination of
    /// positive weights is positive unless one of the PRODUCTS rounds to
    /// zero. That needs a weight within a few ulps of the smallest
    /// subnormal, whatever the other weights are.
    ///
    /// So this row walks the weight scale beside a fixed `1e2` — a ratio
    /// of 1e304 at the bottom end — and demands the refusal at the
    /// minimum subnormal and coverage everywhere a real description
    /// could sit. It is the row that would catch the arm becoming
    /// unreachable (a refusal nothing can produce is not a refusal) or
    /// becoming reachable from an ordinary extreme-weight face, which is
    /// what its prose tells a caller it is not.
    #[test]
    fn refined_weight_lost_positivity_is_reached_only_by_underflow() {
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).expect("kv");
        let control: Vec<geom_core::Point3<f64>> = (0..9)
            .map(|i| geom_core::Point3::new(f64::from(i), 0.0, 0.0))
            .collect();
        let refuse = |w: f64| {
            let mut weights = vec![w; 9];
            weights[4] = 1.0e2;
            let s = NurbsSurface::new(kv.clone(), kv.clone(), control.clone(), weights)
                .expect("the door admits any positive finite weight");
            matches!(
                patch_cells(&s),
                Err(PatchBoundError::RefinedWeightLostPositivity)
            )
        };
        assert!(
            refuse(f64::from_bits(1)),
            "the minimum subnormal weight beside 1e2 must refuse: a product of it with a \
             ratio below 1 underflows, and the weight hull's `lo` reaches zero"
        );
        for w in [1.0e-320, 1.0e-300, 1.0e-200, 1.0e-30, 1.0e-2, 1.0, 1.0e2] {
            assert!(
                !refuse(w),
                "weight {w:e} beside 1e2 is a ratio of {:e} and must still certify: \
                 a ratio cannot reach this refusal, only an underflow can",
                1.0e2 / w
            );
        }
    }

    /// Every patch-bound refusal names what the caller changes in the
    /// description, not only which structural fact refused. Nothing
    /// here delegates: the prose is this module's at every arm, which
    /// is what its consumers print.
    #[test]
    fn every_patch_bound_error_arm_names_a_recourse() {
        // A vocabulary, not a part-of-speech test: an arm that names
        // the thing a caller supplies satisfies the claim the same way
        // an imperative does.
        const RECOURSE_WORDS: &[&str] = &["describe", "supply", "report", "split"];
        let arms = [
            PatchBoundError::DegreeZero,
            PatchBoundError::Degree1Crease,
            PatchBoundError::Crease,
            PatchBoundError::NonPositiveWeight,
            PatchBoundError::RefinedWeightLostPositivity,
            PatchBoundError::RefinementFailed,
            PatchBoundError::DerivedKnots,
        ];
        assert_eq!(arms.len(), 7, "an arm was added without a row here");
        for arm in arms {
            let msg = arm.to_string();
            assert_eq!(msg, arm.note(), "Display is the shared note");
            let lower = msg.to_lowercase();
            assert!(
                RECOURSE_WORDS.iter().any(|w| lower.contains(w)),
                "no recourse in: {msg}"
            );
        }
    }
}
