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
//! [`geom_core::spline::net::TensorCoeffs::diff_u`], which is
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
//! reads a vector magnitude off them with
//! [`norm_sq`](geom_core::interval::norm_sq), whose
//! `√hi` is a sup bound on the norm.
//!
//! **The signed reading is the only one**, and it is a reading of the
//! quotient rule itself: the true `−` signs, divided by the whole
//! weight hull. The cancellations that survive that are real — on a
//! quarter cylinder they are worth an order of magnitude on
//! `sup‖S_uv‖` — so a consumer wanting a magnitude takes `norm_sq`
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
//! divisor, so a net whose positivity was never proven is refused
//! rather than answering.
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
//! # Refusal (fail-loud, D4 ¶2)
//!
//! Structural refusals are typed ([`PatchBoundError`]); arithmetic
//! failures are refusals, and a refused hull fails every `≤ ε`
//! comparison it reaches.

use geom_core::Bounds;
use std::borrow::Cow;
use std::ops::RangeInclusive;

use geom::surfaces::NurbsSurface;
use geom_core::interval::certification::Certification;
use geom_core::interval::{Interval, certified_hi, cross, max_bound, norm_sup};
use geom_core::spline::algebra::equal_split_points;
use geom_core::spline::net::{TensorChannels, TensorCoeffs, TensorNet};
use geom_core::spline::weight_scale::unit_scale_exact;
use geom_core::spline::{C1Break, Derived, KnotVector};

/// The fixed refinement schedule of the RATIONAL arm: every nonempty
/// span of every direction splits into this many equal pieces before
/// the per-cell assembly. A CONSTANT (D9: structure, never a
/// data-dependent iteration), and this arm's own: `geom::curves`'
/// rational speed meter refines by a count of the same value, but that
/// count prices one curve bound against the refusal frontier its own
/// tests pin, while this one prices the per-cell partial hulls every
/// [`patch_cells`] consumer reads (against the insertion rounding
/// below), and `mesh::chords`' rational carrier `sup‖C″‖` bound
/// through [`rational_split_points`]. Neither follows the other. Knot
/// insertion is evaluation-invariant in ℝ, so it changes no geometry;
/// it only shrinks every hull the bound is assembled from, which is
/// what keeps the `sup‖S − c‖·sup|w_dd|` cross terms cell-sized.
///
/// **The schedule is not free of the arithmetic, which is why it is
/// applied in certification arithmetic.** Each insertion the count buys is one more
/// affine combination, and the count therefore also sets how much
/// outward rounding the refined net carries. That width grows with the
/// NUMBER of insertions rather than by a factor per insertion, which is
/// what makes 16 affordable ([`geom_core::spline::CurvePlan::apply_certified`]
/// argues the form that buys it).
pub const RATIONAL_CERT_SPLITS: usize = 16;

/// A typed refusal of the patch-bound assembly (fail-loud). Each
/// carries the prose its consumers print, so a lifted consumer's
/// message is this module's message.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
// The variant roster `topo`'s sample list iterates (this crate's
// `test-support` feature, test builds only): fieldless, so the enum is
// its own discriminant.
#[cfg_attr(feature = "test-support", derive(strum::EnumIter))]
pub enum PatchBoundError {
    /// A degree-0 direction — a degenerate patch description: a
    /// degree-0 locus is a step function rather than a surface
    /// direction, and the form is a designed absence.
    DegreeZero,
    /// A degree-1 direction carrying interior knots: a C⁰ crease. The
    /// interpolation Taylor bound needs C¹.
    Degree1Crease,
    /// A direction whose interior multiplicity equals its degree: a
    /// C⁰ crease. The interpolation Taylor bound needs C¹.
    Crease,
    /// A rational description with a non-positive or non-finite
    /// weight — the convex-combination licence every hull fact rests
    /// on never held. Supplying strictly positive weights lets the face
    /// certify through the rational arm; the door that mints a NURBS
    /// surface refuses these already, so a face reaching this bound
    /// carrying one is also a finding.
    NonPositiveWeight,
    /// The same, discovered after the fixed rational refinement: the
    /// refined weight ENCLOSURE reaches zero. Outside the certified
    /// inventory: every insertion step is met with the hull of its two
    /// sources, so each refined weight's enclosure lies within the
    /// described weights' range, whose lower end the door has already
    /// proven positive — even at the smallest subnormal, where a
    /// product with a ratio below one underflows to zero.
    RefinedWeightLostPositivity,
    /// The fixed rational refinement failed to materialise. Outside
    /// the certified inventory: the fixed schedule inserts knots into a
    /// direction that already passed the C¹ gate, and insertion into a
    /// valid clamped vector is total, so the description that reached
    /// this is reported rather than repaired.
    RefinementFailed,
    /// A direction whose differenced knot vector failed to
    /// materialise. In the patch lanes a valid face reaches this when a
    /// span only a few ulps wide is cut by the equal-split schedule:
    /// its split points collide with the span's ends, the refined
    /// vector carries a knot of full multiplicity, and its
    /// twice-differenced vector is not clamped. The answer is a typed
    /// refusal, never a second partial read as zero.
    DerivedKnots,
}

impl PatchBoundError {
    /// The refusal's prose — one spelling, shared by every consumer
    /// that reports a patch-bound refusal in its own error type.
    pub fn note(self) -> &'static str {
        match self {
            Self::DegreeZero => {
                "NURBS face of degree 0 in one direction, which is a step rather than a \
                 surface. Recourse: describe that direction at degree 1 or above"
            }
            Self::Degree1Crease => {
                "NURBS face of degree 1 with a sharp crease inside it. Recourse: split the \
                 face at the crease"
            }
            Self::Crease => {
                "NURBS face with a sharp crease inside it. Recourse: split the face at the \
                 crease"
            }
            Self::NonPositiveWeight => {
                "rational NURBS face with a non-positive or non-finite weight, which \
                 describes no valid surface. Recourse: supply strictly positive, finite weights"
            }
            Self::RefinedWeightLostPositivity => concat!(
                "rational NURBS face whose refined weights lost positivity, which the \
                 refinement of positive weights never does. ",
                geom_core::kernel_defect_ending!()
            ),
            Self::RefinementFailed => concat!(
                "NURBS face that could not be subdivided for bounding, which a valid face \
                 always allows. ",
                geom_core::kernel_defect_ending!()
            ),
            Self::DerivedKnots => concat!(
                "NURBS face whose derivative could not be formed for bounding (a knot span \
                 a few ulps wide, cut by the fixed subdivision, reaches this). ",
                geom_core::kernel_defect_ending!()
            ),
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
    /// Certified upper bound on `‖S_u‖` over the cell, read from the
    /// Euclidean norms of VECTORS (D4 ¶2), so a rigid map moves it only
    /// by rounding: the cell's quotient form's
    /// `max_{i,j} ‖k_i − g_i·q_j‖ / inf w`, each `k_i − g_i·q_j` one
    /// term of the quotient rule's numerator `Ã_u − (S − c)·w_u` read at
    /// one active control point's offset `q_j = P_j − c`. On the integral
    /// arm every `g_i` is zero and the bound is the largest norm of an
    /// active derived control vector. `NaN` when refused.
    pub s_u_sup: f64,
    /// Certified upper bound on `‖S_v‖` over the cell, read as
    /// [`Self::s_u_sup`] is.
    pub s_v_sup: f64,
    /// Certified upper bound on `‖S_u × S_v‖` over the cell, read from
    /// the norms of coefficient vectors (D4 ¶2): on the integral arm
    /// the cross product of the two hulls of active derived control
    /// vectors, bounded about their centres; on the rational arm the same over
    /// `S_u × S_v = (Ã_u × Ã_v − w_v·Ã_u × (S − c) − w_u·(S − c) × Ã_v)/w²`,
    /// the `(S − c) × (S − c)` term being identically zero. `NaN` when
    /// refused.
    pub m_sup: f64,
}

/// Whether a patch is rational under the kernel's definition (any
/// weight not bitwise `1.0`) — the arm selection, on f64 STRUCTURE.
pub fn is_rational(n: &NurbsSurface<f64>) -> bool {
    n.weights().iter().any(|w| *w != 1.0)
}

/// `n` with its weights at unit scale
/// ([`geom_core::spline::weight_scale::unit_scale_exact`]), or `n`
/// itself where that rescale is a no-op or would drop a bit. The
/// rescaled net describes the same surface; what it buys is range for
/// the weight powers the enclosures form (`w²` here, up to `w⁴` in
/// [`crate::offset_fit`]'s composite). Which arm reads a net is decided
/// on the DESCRIBED weights ([`is_rational`]), before this runs.
pub(crate) fn unit_scaled_weights(n: &NurbsSurface<f64>) -> Cow<'_, NurbsSurface<f64>> {
    let Some((_, scaled)) = unit_scale_exact(n.weights()) else {
        return Cow::Borrowed(n);
    };
    match NurbsSurface::new(
        n.knots_u().clone(),
        n.knots_v().clone(),
        n.control().to_vec(),
        scaled,
    ) {
        Ok(rescaled) => Cow::Owned(rescaled),
        Err(e) => unreachable!(
            "the door refused an exact power-of-two rescale of weights it admitted: {e}"
        ),
    }
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

/// The equal-split refinement of every channel of `nets`
/// ([`TensorChannels::refine_equal_split`]), IN INTERVAL ARITHMETIC. The
/// chains are built from structure alone (the homogeneous nets this
/// module refines are polynomial, so their weights are unit): the same
/// chain the surface refinement applies through
/// [`geom_core::spline::CurvePlan::apply_rational`], with interval
/// arithmetic re-deriving each insertion ratio from the knots it is made
/// of instead of widening the plan's `f64` `λ`.
///
/// # Errors
///
/// [`PatchBoundError::RefinementFailed`] — a refused insertion chain.
fn refine_split<'s, const C: usize>(
    nets: &'s TensorChannels<'_, C>,
    splits: usize,
) -> Result<TensorChannels<'s, C>, PatchBoundError> {
    nets.refine_equal_split(splits)
        .map_err(|_| PatchBoundError::RefinementFailed)
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
    if kv.degree() == 0 {
        return Err(PatchBoundError::DegreeZero);
    }
    kv.c1().map_err(|b| match b {
        C1Break::Degree1Crease => PatchBoundError::Degree1Crease,
        C1Break::Crease => PatchBoundError::Crease,
    })
}

/// The interior split points of the fixed rational refinement
/// schedule for one knot vector: [`equal_split_points`] at
/// [`RATIONAL_CERT_SPLITS`] pieces per nonempty span.
pub fn rational_split_points(kv: &KnotVector) -> Vec<f64> {
    equal_split_points(kv, RATIONAL_CERT_SPLITS)
}

/// A coefficient net as certification enclosures — the shared tensor assembly,
/// homed in [`geom_core::spline::net`] (issue 1006). The alias is kept
/// so this module's own prose and its consumers keep naming the thing
/// they read; the differencing is not this module's any more.
pub type Net = TensorNet;

/// The signed hull of `a[i][j] − c·w[i][j]` over the window
/// `wu × wv` — the recentred homogeneous net `Ã = A − c·w` read
/// through the linearity of knot differencing (`d(A − c·w) = dA −
/// c·dw`, entrywise, same knots). Out-of-range indices are refused.
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
            let e = tilde_entry(a, w, c, i, j);
            acc = Some(match acc {
                None => e,
                Some(h) => Interval::hull(h, e),
            });
        }
    }
    acc.unwrap_or_else(Interval::refused)
}

/// One entry of the recentred homogeneous net, `a[i][j] − c·w[i][j]` —
/// the coefficient [`window_tilde_hull`] hulls and the rational cells'
/// vector readings take norms of.
fn tilde_entry(a: &Net, w: &Net, c: Interval, i: usize, j: usize) -> Interval {
    a.get(i, j) - c * w.get(i, j)
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

/// A span's `[knot, next knot]` extent (the caller has already
/// established the span is nonempty, so both knots exist).
fn span_extent(kv: &KnotVector, span: usize) -> (f64, f64) {
    let k = kv.knots();
    (
        k.get(span).copied().unwrap_or(f64::NAN),
        k.get(span + 1).copied().unwrap_or(f64::NAN),
    )
}

/// The three spatial channels of a control net, as enclosure points
/// over the surface's one pair of vectors — the integral arm's nets;
/// [`rational_cells`] builds the homogeneous ones beside `w`.
///
/// **The SHAPE is shared** with `offset_fit::channel`: both build
/// [`TensorCoeffs`] against the surface's own knot vectors, which
/// `PatchSpans::decompose` consumes and whose net [`window_hull`]
/// reads.
///
/// **The ARITHMETIC still diverges, and that is what is left.** This
/// module extracts `w·P`; `offset_fit::channel` extracts `w·(P − c)`
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
fn comp_nets(n: &NurbsSurface<f64>) -> TensorChannels<'_, 3> {
    let nv = n.knots_v().control_count();
    TensorChannels::from_fn(n.knots_u(), n.knots_v(), |i, j| {
        // Row-major layout: control[iu·nv + iv] — the net's own.
        let p = n.control()[i * nv + j];
        [p.x, p.y, p.z].map(Interval::point)
    })
}

/// A second partial along one direction: identically zero (a degree-1
/// direction, which the C¹ gate admits only as one linear span, so its
/// refinement's knots are removable), or the twice-differenced net.
/// There is no third case: a degree ≥ 2 direction whose derived vector
/// is not clamped is refused when the nets are built, never read as
/// zero.
enum Second {
    Zero,
    Net(Net),
}

impl Second {
    /// The second partial along a direction whose first partial is
    /// `derived`, `diff` differencing that partial again along the same
    /// direction.
    ///
    /// # Errors
    ///
    /// [`PatchBoundError::DerivedKnots`] at [`Derived::Discontinuous`]
    /// — a multiplicity the C¹ gate never saw, which equal-split
    /// refinement can raise when its split points collide on a span a
    /// few ulps wide.
    fn of(
        derived: Derived<'_, TensorCoeffs<'_>>,
        diff: impl FnOnce(&TensorCoeffs<'_>) -> Net,
    ) -> Result<Self, PatchBoundError> {
        match derived {
            Derived::Vector(t) => Ok(Self::Net(diff(&t))),
            Derived::SpanConstant(_) => Ok(Self::Zero),
            Derived::Discontinuous { .. } => Err(PatchBoundError::DerivedKnots),
        }
    }

    /// The hull over a cell: exactly zero for [`Second::Zero`]; for a
    /// net, `read` over the span's second derived window, which every
    /// degree ≥ 2 span has — refused, never zero, if it were missing.
    fn hull(
        &self,
        d2: Option<&RangeInclusive<usize>>,
        read: impl Fn(&Net, &RangeInclusive<usize>) -> Interval,
    ) -> Interval {
        match self {
            Self::Zero => Interval::zero(),
            Self::Net(net) => d2.map_or_else(Interval::refused, |w| read(net, w)),
        }
    }
}

/// The five per-direction derivative nets one channel needs.
struct DNets {
    d10: Net,
    d01: Net,
    d11: Net,
    d20: Second,
    d02: Second,
}

impl DNets {
    /// # Errors
    ///
    /// As [`Second::of`].
    fn build(base: &TensorCoeffs<'_>) -> Result<Self, PatchBoundError> {
        Ok(Self {
            d10: base.diff_u(),
            d01: base.diff_v(),
            d11: base.diff_uv(),
            d20: Second::of(base.derived_u(), |t| t.diff_u())?,
            d02: Second::of(base.derived_v(), |t| t.diff_v())?,
        })
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
/// (`S_u, S_v, S_uu, S_uv, S_vv`, in that order) and the upper bounds
/// on `‖S_u‖`, `‖S_v‖` and `‖S_u × S_v‖`.
fn cell_from(
    uv: ((f64, f64), (f64, f64)),
    signed: [[Interval; 3]; 5],
    [s_u_sup, s_v_sup, m_sup]: [f64; 3],
) -> PatchCell {
    PatchCell {
        u: uv.0,
        v: uv.1,
        s_u: signed[0],
        s_v: signed[1],
        s_uu: signed[2],
        s_uv: signed[3],
        s_vv: signed[4],
        s_u_sup,
        s_v_sup,
        m_sup,
    }
}

/// The control VECTORS active on the cell whose window is `wu × wv`,
/// one per `(i, j)`, `u`-major: entry `c` of each is `vector(c, i, j)`.
fn window_vectors(
    wu: &RangeInclusive<usize>,
    wv: &RangeInclusive<usize>,
    vector: impl Fn(usize, usize, usize) -> Interval,
) -> Vec<[Interval; 3]> {
    wu.clone()
        .flat_map(|i| wv.clone().map(move |j| (i, j)))
        .map(|(i, j)| core::array::from_fn(|c| vector(c, i, j)))
        .collect()
}

/// The terms `k − g·q` of a cell's quotient form: `ks` the active
/// coefficient vectors of `Ã_d` on the window `wu × wv` in
/// [`window_vectors`]' order, each paired with the entry of the weight's
/// derived net `wd` at the same index.
fn window_terms(
    ks: &[[Interval; 3]],
    wd: &Net,
    wu: &RangeInclusive<usize>,
    wv: &RangeInclusive<usize>,
) -> Vec<Affine> {
    let gs = wu
        .clone()
        .flat_map(|i| wv.clone().map(move |j| wd.get(i, j)));
    ks.iter().zip(gs).map(|(&k, g)| Affine { k, g }).collect()
}

/// A certified upper bound on `‖x‖` over every `x` in the convex hull of
/// `vectors`: the largest norm of one of them, the norm being convex. On
/// one knot span a VECTOR B-spline is a convex combination of its active
/// control vectors, so this bounds the form's norm over the cell. `NaN`
/// for no vectors, or when one is refused.
fn vectors_norm_sup(vectors: &[[Interval; 3]]) -> f64 {
    vectors
        .iter()
        .map(norm_sup)
        .reduce(max_bound)
        .unwrap_or(f64::NAN)
}

/// A certified upper bound on `‖x × y‖` over every `x` in the convex
/// hull of `a` and every `y` in that of `b` — the cross product of two
/// vector B-splines on one cell, each a convex combination of its
/// active coefficient vectors there.
///
/// Centred: for any centres `ā` and `b̄`, with `δ = x − ā` and
/// `η = y − b̄`, `x × y = ā × b̄ + ā × η + δ × b̄ + δ × η`, and the norm
/// being convex, `‖δ‖ ≤ δ̂ = max_i ‖a_i − ā‖` and `‖η‖ ≤ η̂ = max_j
/// ‖b_j − b̄‖`; so the bound is `‖ā × b̄‖ + ‖ā‖·η̂ + δ̂·‖b̄‖ + δ̂·η̂`.
/// Every term is a norm of vectors, so a rotation moves it only by
/// rounding; the slack is the hulls' reach about their
/// centres, which shrinks with the cell. The centres are a translation
/// CHOICE — each coordinate's mean of the vectors' midpoints, any finite
/// value being sound — and a non-finite one is taken as `0`. `NaN` when
/// either list is empty or a term is refused.
fn cross_norm_sup(a: &[[Interval; 3]], b: &[[Interval; 3]]) -> f64 {
    let reach = |vs: &[[Interval; 3]], c: &[Interval; 3]| {
        let about: Vec<[Interval; 3]> = vs
            .iter()
            .map(|v| [v[0] - c[0], v[1] - c[1], v[2] - c[2]])
            .collect();
        vectors_norm_sup(&about)
    };
    let (ca, cb) = (centre(a), centre(b));
    let terms = [
        norm_sup(&cross(&ca, &cb)),
        norm_sup(&ca),
        reach(b, &cb),
        reach(a, &ca),
        norm_sup(&cb),
    ];
    if terms.iter().any(|t| !t.is_finite()) {
        return f64::NAN;
    }
    let [m, na, eb, da, nb] = terms.map(Interval::point);
    certified_hi(m + na * eb + da * nb + da * eb)
}

/// A centre for `vectors` — a translation CHOICE, so any finite value is
/// sound and none of it has to be enclosed: per coordinate, the mean of
/// the vectors' midpoints in their order. A non-finite midpoint (an
/// overflowed or refused enclosure) contributes `0`, and a non-finite
/// mean is taken as `0`, so the centre is always finite and the reach
/// about it is what reports the trouble.
fn centre(vectors: &[[Interval; 3]]) -> [Interval; 3] {
    core::array::from_fn(|c| {
        let sum: f64 = vectors
            .iter()
            .map(|v| (v[c].lo() + v[c].hi()) / 2.0)
            .map(|m| if m.is_finite() { m } else { 0.0 })
            .sum();
        #[allow(clippy::cast_precision_loss)]
        let m = sum / vectors.len() as f64;
        Interval::point(if m.is_finite() { m } else { 0.0 })
    })
}

/// The signed hull of coordinate `c` over `vectors`, folded in their
/// order; refused for none.
fn coordinate_hull(vectors: &[[Interval; 3]], c: usize) -> Interval {
    vectors
        .iter()
        .map(|v| v[c])
        .reduce(Interval::hull)
        .unwrap_or_else(Interval::refused)
}

/// One cell's quotient-numerator form along one direction `d`.
///
/// On the cell, `S_d = (Ã_d − (S − c)·w_d)/w` with `Ã_d = A_d − c·w_d`
/// (module docs), and the numerator is a convex combination, under the
/// derived basis, of the terms `k_i − g_i·(S − c)`: `k_i` the active
/// coefficient vectors of `Ã_d`, `g_i` the matching ones of `w_d`. The
/// offsets `q_j = P_j − c` of the cell's active control points hold
/// `S − c` in their hull (positive weights make the rational basis a
/// nonnegative partition of unity over them), and the weight hull
/// holds `w`. An integral cell is the same form with every `g_i`
/// exactly zero over unit weight.
pub(crate) struct QuotientForm<'q> {
    /// The terms `k_i − g_i·q`, one per active derived coefficient.
    pub(crate) terms: Vec<Affine>,
    /// The offsets `q_j = P_j − c`.
    pub(crate) offsets: &'q [[Interval; 3]],
    /// The weight hull over the cell.
    pub(crate) weight: Interval,
}

impl QuotientForm<'_> {
    /// The weight hull's lower end, the divisor every norm reads; `None`
    /// unless the hull is certified positive.
    pub(crate) fn floor(&self) -> Option<Interval> {
        (self.weight.is_certified() && self.weight.lo() > 0.0)
            .then(|| Interval::point(self.weight.lo()))
    }

    /// A certified upper bound on `‖S_d‖` over the cell,
    /// `max_{i,j} ‖k_i − g_i·q_j‖ / inf w`; `NaN` when the weight hull is
    /// not positive or a read refuses.
    ///
    /// Each term is affine in `S`, so its norm, convex in `S`, is largest
    /// at one of the offsets: the maximum is the numerator's norm bound
    /// with no box in it, and a rigid map carries every `k_i − g_i·q_j`
    /// to its image. The terms are divided by the weight's lower bound
    /// before any norm squares them, so a subnormal weight's products are
    /// not squared away, and the maximum is read through
    /// [`pair_norm_sup`].
    pub(crate) fn speed_sup(&self) -> f64 {
        let Some(floor) = self.floor() else {
            return f64::NAN;
        };
        let terms: Vec<Affine> = self.terms.iter().map(|t| t.per(floor)).collect();
        pair_norm_sup(self.offsets, &terms, &[])
    }

    /// The signed componentwise enclosure of `S_d` over the cell with
    /// `S − c` ranging over the box `q`: the hull of every term at `q`,
    /// over the weight hull. Refused for no terms.
    pub(crate) fn derivative_box(&self, q: [Interval; 3]) -> [Interval; 3] {
        self.terms
            .iter()
            .map(|t| t.at(q))
            .reduce(|a, b| core::array::from_fn(|k| Interval::hull(a[k], b[k])))
            .map_or([Interval::refused(); 3], |n| n.map(|x| x / self.weight))
    }
}

/// A vector affine in the offset `q = S − c`: `k − g·q`.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Affine {
    pub(crate) k: [Interval; 3],
    pub(crate) g: Interval,
}

impl Affine {
    pub(crate) fn at(&self, q: [Interval; 3]) -> [Interval; 3] {
        core::array::from_fn(|i| self.k[i] - self.g * q[i])
    }

    pub(crate) fn times(&self, e: Interval) -> Self {
        Self {
            k: self.k.map(|x| x * e),
            g: self.g * e,
        }
    }

    pub(crate) fn per(&self, floor: Interval) -> Self {
        Self {
            k: self.k.map(|x| x / floor),
            g: self.g / floor,
        }
    }
}

/// `max ‖x(q) + y(q)‖` over the terms `x` of `xs`, `y` of `ys` and
/// the offsets `q`, rounded outward; `NaN` when a read refuses.
///
/// Every pair is screened in `f64` first ([`Centred::of_affine`], then
/// [`screen_bound`]: an upper bound on the sum's norm from centres and
/// radii), and only the best-screened pair, and then each pair whose
/// screen is above the maximum read so far, is read in certification
/// arithmetic. A pair skipped has a real norm at most that maximum, so
/// the result is at most the full enumeration's and below it only by
/// rounding. A screen that is `+∞` or `NaN` (an unbounded or
/// unformable radius or centre) rules nothing out, so such a pair is
/// read. Every input is asked for its refusal first, as the full
/// enumeration's `norm_sup` would ask every pair.
///
/// An empty `ys` reads `max ‖x(q)‖`. When no term reads the offset
/// (every `g` an exact zero, as on a polynomial net) every offset reads
/// alike, and only the first is read.
pub(crate) fn pair_norm_sup(qs: &[[Interval; 3]], xs: &[Affine], ys: &[Affine]) -> f64 {
    let certified = |a: &Affine| a.g.is_certified() && a.k.iter().all(|c| c.is_certified());
    if !(xs.iter().all(certified)
        && ys.iter().all(certified)
        && qs.iter().flatten().all(|c| c.is_certified()))
    {
        return f64::NAN;
    }
    let flat = xs
        .iter()
        .chain(ys)
        .all(|t| t.g.lo() == 0.0 && t.g.hi() == 0.0);
    let qs = if flat { &qs[..qs.len().min(1)] } else { qs };
    let xa: Vec<(Centred, Centred)> = xs.iter().map(Centred::affine_parts).collect();
    let ya: Vec<(Centred, Centred)> = ys.iter().map(Centred::affine_parts).collect();
    let none = [Centred {
        mid: [0.0; 3],
        rad: 0.0,
    }];
    let mut screens: Vec<(f64, (usize, usize, usize))> =
        Vec::with_capacity(qs.len() * xs.len() * ys.len().max(1));
    let mut cy: Vec<Centred> = Vec::with_capacity(ys.len());
    for (p, q) in qs.iter().enumerate() {
        let q = Centred::of(q);
        cy.clear();
        cy.extend(ya.iter().map(|(k, g)| Centred::of_affine(k, g, &q)));
        let cy = if ys.is_empty() { &none[..] } else { &cy[..] };
        for (i, (k, g)) in xa.iter().enumerate() {
            let a = Centred::of_affine(k, g, &q);
            for (j, b) in cy.iter().enumerate() {
                screens.push((screen_bound(&a, b), (p, i, j)));
            }
        }
    }
    let norm = |(p, i, j): (usize, usize, usize)| {
        let x = xs[i].at(qs[p]);
        match ys.get(j) {
            Some(y) => {
                let y = y.at(qs[p]);
                norm_sup(&core::array::from_fn(|k| x[k] + y[k]))
            }
            None => norm_sup(&x),
        }
    };
    let Some(&(_, best)) = screens.iter().max_by(|a, b| a.0.total_cmp(&b.0)) else {
        return f64::NAN;
    };
    let mut sup = norm(best);
    for &(bound, pair) in &screens {
        // A `NaN` screen or maximum rules nothing out.
        let ruled_out = bound <= sup;
        if !ruled_out && pair != best {
            sup = max_bound(sup, norm(pair));
        }
    }
    sup
}

/// An enclosure as an `f64` centre and a radius rounded up: every
/// point of it is within the radius of the centre on each axis. A
/// radius that overflowed reads `+∞` and one that cannot be formed
/// reads `NaN`; [`screen_bound`] screens nothing out on either.
#[derive(Clone, Copy)]
struct Centred {
    mid: [f64; 3],
    rad: f64,
}

impl Centred {
    /// A certified interval vector's centre and radius.
    fn of(v: &[Interval; 3]) -> Self {
        let mid = v.map(|c| 0.5 * c.lo() + 0.5 * c.hi());
        let rad = (0..3)
            .map(|k| max_bound(v[k].hi() - mid[k], mid[k] - v[k].lo()).next_up())
            .fold(0.0, max_bound);
        Self { mid, rad }
    }

    /// An affine term's `k` and `g` (the scalar in the first axis).
    fn affine_parts(a: &Affine) -> (Self, Self) {
        (Self::of(&a.k), Self::of(&[a.g, a.g, a.g]))
    }

    /// `k − g·q` from the parts' centres and radii, in `f64`. A point of
    /// the image is within `r_k + |g_c|·r_q + r_g·(|q_c| + r_q)` of
    /// `k_c − g_c·q_c` on each axis, and computing that centre rounds
    /// it by at most `3u·(|k_c| + |g_c·q_c|)`; the radius is that sum
    /// with `4ε` for the centre's rounding, enlarged by `1 + 8ε` for
    /// its own and by the smallest normal for any subnormal step. A zero
    /// factor is an exact zero ([`times_or_zero`]), so an unbounded offset beside
    /// a term that does not read it adds nothing rather than `0·∞`.
    fn of_affine(k: &Self, g: &Self, q: &Self) -> Self {
        let (gm, gr) = (g.mid[0], g.rad);
        let mid: [f64; 3] = core::array::from_fn(|i| k.mid[i] - times_or_zero(gm, q.mid[i]));
        let rad = (0..3)
            .map(|i| {
                let qa = q.mid[i].abs();
                k.rad
                    + times_or_zero(gm.abs(), q.rad)
                    + times_or_zero(gr, qa + q.rad)
                    + 4.0 * f64::EPSILON * (k.mid[i].abs() + times_or_zero(gm.abs(), qa))
            })
            .fold(0.0, max_bound);
        Self {
            mid,
            rad: rad * (1.0 + 8.0 * f64::EPSILON) + f64::MIN_POSITIVE,
        }
    }
}

/// `a·b` with a zero factor an exact zero: a bound times a factor that
/// is exactly zero contributes nothing, whatever the bound.
fn times_or_zero(a: f64, b: f64) -> f64 {
    if a == 0.0 || b == 0.0 { 0.0 } else { a * b }
}

/// An upper bound on `‖x + y‖` over `x` and `y` in the two boxes,
/// computed in `f64`: `(‖m‖ + √3·(r_x + r_y))·(1 + 32ε)`, with `m` the
/// rounded sum of the centres. The roundings it covers — the sum of the
/// centres (`u` relative), the three-term norm away from underflow (at
/// most `4u`), the radius sum and its product with `√3`, and the final
/// sum and product — total well under `32ε`. `+∞`, which screens
/// nothing out, wherever that argument does not hold: a `NaN` or
/// infinite centre or radius, or a norm below `1e-150` (squares near
/// the subnormal range, which can read `0` for a centre that is not)
/// unless the centre sum is exactly zero, or a norm above `1e150`.
fn screen_bound(a: &Centred, b: &Centred) -> f64 {
    const SQRT_3_UP: f64 = 1.732_050_807_568_877_4;
    const SLACK: f64 = 1.0 + 32.0 * f64::EPSILON;
    let m: [f64; 3] = core::array::from_fn(|k| a.mid[k] + b.mid[k]);
    let n = m.iter().map(|c| c.powi(2)).sum::<f64>().sqrt();
    let rad = a.rad + b.rad;
    let zero = m.iter().all(|&c| c == 0.0);
    if !(n.is_finite() && rad.is_finite()) || (!zero && n < 1e-150) || n > 1e150 {
        return f64::INFINITY;
    }
    (n + SQRT_3_UP * rad) * SLACK
}

/// The INTEGRAL arm (all weights bitwise `1.0`): the plain hull
/// assembly on the spatial nets — no quotient rule intervenes, so the
/// enclosure IS the coefficient hull.
fn integral_cells(n: &NurbsSurface<f64>) -> Result<Vec<PatchCell>, PatchBoundError> {
    integral_cells_on(&comp_nets(n))
}

/// [`integral_cells`] after refining every nonempty span into `splits`
/// equal pieces, IN INTERVAL ARITHMETIC: an integral net's weights are unit, so the
/// net is already homogeneous and [`refine_split`]'s schedule applies to
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
    integral_cells_on(&refine_split(&comp_nets(n), splits)?)
}

/// The integral arm's per-cell assembly over ALREADY-BUILT spatial nets
/// and their directions — the one body [`integral_cells`] and
/// [`integral_cells_refined`] share, so refinement changes what is
/// assembled and nothing about how.
///
/// # Errors
///
/// [`PatchBoundError::DerivedKnots`] ([`Second::of`]).
fn integral_cells_on(base_nets: &TensorChannels<'_, 3>) -> Result<Vec<PatchCell>, PatchBoundError> {
    let (kv_u, kv_v) = (base_nets.knots_u(), base_nets.knots_v());
    let nets = base_nets
        .channels()
        .iter()
        .map(DNets::build)
        .collect::<Result<Vec<_>, _>>()?;
    let zero = Interval::zero();
    let origin = [[zero; 3]];
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
            let mut s_uu = [zero; 3];
            let mut s_uv = [zero; 3];
            let mut s_vv = [zero; 3];
            for (c, d) in nets.iter().enumerate() {
                s_uu[c] = d
                    .d20
                    .hull(w.u_d2.as_ref(), |net, wu2| window_hull(net, wu2, &w.v_val));
                s_vv[c] = d
                    .d02
                    .hull(w.v_d2.as_ref(), |net, wv2| window_hull(net, &w.u_val, wv2));
                s_uv[c] = window_hull(&d.d11, &w.u_d1, &w.v_d1);
            }
            let ku = window_vectors(&w.u_d1, &w.v_val, |c, i, j| nets[c].d10.get(i, j));
            let kv = window_vectors(&w.u_val, &w.v_d1, |c, i, j| nets[c].d01.get(i, j));
            // The quotient form with `g ≡ 0` over unit weight: its box is
            // the coefficient hull and its speed the largest coefficient
            // norm.
            let form = |ks: &[[Interval; 3]]| QuotientForm {
                terms: ks.iter().map(|&k| Affine { k, g: zero }).collect(),
                offsets: &origin,
                weight: Interval::point(1.0),
            };
            let (fu, fv) = (form(&ku), form(&kv));
            cells.push(cell_from(
                (span_extent(kv_u, su), span_extent(kv_v, sv)),
                [
                    fu.derivative_box(origin[0]),
                    fv.derivative_box(origin[0]),
                    s_uu,
                    s_uv,
                    s_vv,
                ],
                [fu.speed_sup(), fv.speed_sup(), cross_norm_sup(&ku, &kv)],
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
    let scaled = unit_scaled_weights(n);
    let n = &*scaled;
    // THE REFINEMENT IS PART OF THE ENCLOSURE. The homogeneous nets `w`
    // and `w·P` are refined IN INTERVAL ARITHMETIC from point intervals of the
    // DESCRIBED net, so insertion widens outward like every later step
    // and the cells enclose the described patch. An `f64` refinement
    // here would make them enclose the refined-`f64` patch instead, and
    // the described one escapes that by insertion rounding amplified by
    // the knot differencing.
    // One refinement of the four homogeneous channels `w·x, w·y, w·z, w`
    // over the surface's one pair of vectors.
    let nv0 = n.knots_v().control_count();
    let described = TensorChannels::from_fn(n.knots_u(), n.knots_v(), |i, j| {
        let (p, w) = (
            n.control()[i * nv0 + j],
            Interval::point(n.weights()[i * nv0 + j]),
        );
        [
            w * Interval::point(p.x),
            w * Interval::point(p.y),
            w * Interval::point(p.z),
            w,
        ]
    });
    let refined = refine_split(&described, splits)?;
    let (kv_u, kv_v) = (refined.knots_u(), refined.knots_v());
    let [ax, ay, az, w_pair] = refined.channels();
    let w_grid = w_pair.net();
    let (nu, nv) = (w_grid.nu(), w_grid.nv());
    // Positivity survives insertion in ℝ (convex combinations); this
    // code may not assume the ARITHMETIC proved it, so the refined
    // licence is read off the enclosure's own `lo` — a weight hull that
    // touches or straddles zero voids the convex-combination licence
    // just as a described non-positive weight does, and a refusal is not a
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
    // inserted knots are removable), so those partials are
    // `Second::Zero` and their terms exact zeros; the CROSS terms
    // stay. A degree ≥ 2 direction with no derived vector refuses
    // (`DerivedKnots`) rather than reading zero.
    let w_nets = DNets::build(&w_pair)?;
    let a_base = [ax, ay, az];
    let a_nets = a_base
        .iter()
        .map(DNets::build)
        .collect::<Result<Vec<_>, _>>()?;
    // The refined control points, `P = A / w` per channel, ONCE for the
    // whole net. Each is read by every cell whose window covers it —
    // `(pu + 1)(pv + 1)` cells at the interior, twice over (the centroid
    // and the value hull) — so computing it per cell paid the same ring
    // division up to `2(pu + 1)(pv + 1)` times for one coefficient.
    // Entrywise, so each point is enclosed at its own weight rather than
    // at the cell's weight hull.
    let p_nets: Vec<Net> = a_base
        .iter()
        .map(|a| Net::from_fn(nu, nv, |i, j| a.net().get(i, j) / w_grid.get(i, j)))
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
                    .map_or_else(Interval::refused, |p| p.get(i, j))
            };
            // The cell centroid ([`centre`]) of the active control
            // points, and every reading below recentred on it: the
            // active `P − c`, whose hull holds `S − c` (positive weights
            // make the rational basis a nonnegative partition of unity
            // over them), and the active coefficients of `Ã_u` and `Ã_v`.
            let cc = centre(&window_vectors(&w.u_val, &w.v_val, point_at));
            let about = window_vectors(&w.u_val, &w.v_val, |k, i, j| point_at(k, i, j) - cc[k]);
            let au = window_vectors(&w.u_d1, &w.v_val, |k, i, j| {
                tilde_entry(&a_nets[k].d10, &w_nets.d10, cc[k], i, j)
            });
            let av = window_vectors(&w.u_val, &w.v_d1, |k, i, j| {
                tilde_entry(&a_nets[k].d01, &w_nets.d01, cc[k], i, j)
            });
            // The cell's weight hull — the divisor (module docs).
            let w_cell = window_hull(w_grid, &w.u_val, &w.v_val);
            let fu = QuotientForm {
                terms: window_terms(&au, &w_nets.d10, &w.u_d1, &w.v_val),
                offsets: &about,
                weight: w_cell,
            };
            let fv = QuotientForm {
                terms: window_terms(&av, &w_nets.d01, &w.u_val, &w.v_d1),
                offsets: &about,
                weight: w_cell,
            };
            let q = core::array::from_fn(|k| coordinate_hull(&about, k));
            let (s_u, s_v) = (fu.derivative_box(q), fv.derivative_box(q));
            // Weight-net hulls on the cell.
            let w10s = window_hull(&w_nets.d10, &w.u_d1, &w.v_val);
            let w01s = window_hull(&w_nets.d01, &w.u_val, &w.v_d1);
            let w11s = window_hull(&w_nets.d11, &w.u_d1, &w.v_d1);
            let w20s = w_nets
                .d20
                .hull(w.u_d2.as_ref(), |net, wu2| window_hull(net, wu2, &w.v_val));
            let w02s = w_nets
                .d02
                .hull(w.v_d2.as_ref(), |net, wv2| window_hull(net, &w.u_val, wv2));
            let mut s_uu = [zero; 3];
            let mut s_uv = [zero; 3];
            let mut s_vv = [zero; 3];
            for (comp, a) in a_nets.iter().enumerate() {
                let v0s = q[comp];
                // Recentred homogeneous derivative hulls
                // `Ã_kl = A_kl − c·w_kl` on the cell.
                let at =
                    |an: &Net, wn: &Net, wu: &RangeInclusive<usize>, wv: &RangeInclusive<usize>| {
                        window_tilde_hull(an, wn, cc[comp], wu, wv)
                    };
                let a11s = at(&a.d11, &w_nets.d11, &w.u_d1, &w.v_d1);
                // `A` and `w` are channels of one pair, so their second
                // partials are both `Zero` or both nets.
                let a20s = match (&a.d20, &w_nets.d20) {
                    (Second::Zero, Second::Zero) => zero,
                    (Second::Net(an), Second::Net(wn)) => w
                        .u_d2
                        .as_ref()
                        .map_or_else(Interval::refused, |wu2| at(an, wn, wu2, &w.v_val)),
                    _ => unreachable!("A and w share one pair of vectors"),
                };
                let a02s = match (&a.d02, &w_nets.d02) {
                    (Second::Zero, Second::Zero) => zero,
                    (Second::Net(an), Second::Net(wn)) => w
                        .v_d2
                        .as_ref()
                        .map_or_else(Interval::refused, |wv2| at(an, wn, &w.u_val, wv2)),
                    _ => unreachable!("A and w share one pair of vectors"),
                };
                // The quotient rule itself, in certification arithmetic, divided by
                // the whole weight hull.
                let (s1u, s1v) = (s_u[comp], s_v[comp]);
                s_uu[comp] = (a20s - two * s1u * w10s - v0s * w20s) / w_cell;
                s_vv[comp] = (a02s - two * s1v * w01s - v0s * w02s) / w_cell;
                s_uv[comp] = (a11s - s1u * w01s - s1v * w10s - v0s * w11s) / w_cell;
            }
            // `‖S_u × S_v‖` over coefficient-vector norms (the field's
            // docs): the cross term with the two `w_d·Ã_d × (S − c)` terms
            // over `inf w²`.
            let vectors = [
                vectors_norm_sup(&au),
                vectors_norm_sup(&av),
                vectors_norm_sup(&about),
                cross_norm_sup(&au, &av),
            ];
            let weights = [w_cell, w10s, w01s];
            let m_sup = if vectors.iter().all(|t| t.is_finite())
                && weights.iter().all(|i| i.is_certified())
            {
                let [au_sup, av_sup, reach, cross_sup] = vectors.map(Interval::point);
                let (w_u, w_v) = (Interval::point(w10s.mag()), Interval::point(w01s.mag()));
                let w_lo = Interval::point(w_cell.lo());
                certified_hi((cross_sup + au_sup * reach * w_v + reach * av_sup * w_u) / w_lo.sqr())
            } else {
                f64::NAN
            };
            cells.push(cell_from(
                (span_extent(kv_u, su), span_extent(kv_v, sv)),
                [s_u, s_v, s_uu, s_uv, s_vv],
                [fu.speed_sup(), fv.speed_sup(), m_sup],
            ));
        }
    }
    Ok(cells)
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
pub(crate) mod tests {
    use super::*;
    use geom_core::spline::weight_scale::pow2_scaled;

    /// **`RefinedWeightLostPositivity` is not reached from a door-valid
    /// face, even at the minimum subnormal.** The arm's prose claims the
    /// refinement keeps every weight's enclosure inside the described
    /// weights' range. The sharpest witness is the smallest subnormal
    /// beside `1e2` — a ratio of 1e304 — where `β·w` alone underflows
    /// to zero: only the step's meet with its sources' hull keeps the
    /// enclosure's lower end at `w`. The walk up the scale covers
    /// everywhere a real description could sit.
    #[test]
    fn refined_weight_lost_positivity_is_not_reached_from_a_valid_face() {
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
        for w in [
            f64::from_bits(1),
            1.0e-320,
            1.0e-300,
            1.0e-200,
            1.0e-30,
            1.0e-2,
            1.0,
            1.0e2,
        ] {
            assert!(
                !refuse(w),
                "weight {w:e} beside 1e2 lost positivity in the refinement: a step reached \
                 below the hull of its two sources"
            );
        }
    }

    /// `n` with every weight multiplied by `2^k`.
    pub(crate) fn weights_times_pow2(n: &NurbsSurface<f64>, k: i32) -> NurbsSurface<f64> {
        NurbsSurface::new(
            n.knots_u().clone(),
            n.knots_v().clone(),
            n.control().to_vec(),
            n.weights().iter().map(|&w| pow2_scaled(w, k)).collect(),
        )
        .expect("an exact power-of-two scale of door-valid weights")
    }

    /// The rational quarter cylinder, weights `1, 1, s, s, 1, 1`.
    fn quarter_cylinder() -> NurbsSurface<f64> {
        let s = core::f64::consts::FRAC_PI_4.cos();
        let kv2 = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).expect("kv2");
        let kv1 = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).expect("kv1");
        let control = vec![
            geom_core::Point3::new(1.0, 0.0, 0.0),
            geom_core::Point3::new(1.0, 0.0, 1.0),
            geom_core::Point3::new(1.0, 1.0, 0.0),
            geom_core::Point3::new(1.0, 1.0, 1.0),
            geom_core::Point3::new(0.0, 1.0, 0.0),
            geom_core::Point3::new(0.0, 1.0, 1.0),
        ];
        NurbsSurface::new(kv2, kv1, control, vec![1.0, 1.0, s, s, 1.0, 1.0]).expect("door")
    }

    /// **One power of two on every weight moves no cell**, at either
    /// end of the `f64` range. At `2⁻³⁰⁰` the `w²` the cross-product
    /// sup divides by underflows to zero, at `2³⁰⁰` it overflows, and
    /// at `2⁻¹⁰⁷⁰` the weights themselves are subnormal — on the
    /// dyadic net, whose weights keep every bit there. Each scale
    /// describes the same surface, so every cell is the unit face's to
    /// the bit.
    #[test]
    fn a_uniform_weight_scale_moves_no_cell() {
        let cylinder = quarter_cylinder();
        let dyadic = NurbsSurface::new(
            cylinder.knots_u().clone(),
            cylinder.knots_v().clone(),
            cylinder.control().to_vec(),
            vec![1.0, 1.0, 0.75, 0.75, 1.0, 1.0],
        )
        .expect("door");
        for (name, unit, scales) in [
            ("cylinder", &cylinder, &[-1000, -300, -1, 1, 300, 1000][..]),
            ("dyadic", &dyadic, &[-1070][..]),
        ] {
            let want = format!("{:?}", patch_cells(unit).expect("the unit face's cells"));
            for &k in scales {
                let scaled = weights_times_pow2(unit, k);
                let got = patch_cells(&scaled).unwrap_or_else(|e| {
                    panic!("{name} at 2^{k}: a uniformly scaled face refused: {e}")
                });
                assert_eq!(format!("{got:?}"), want, "{name} at 2^{k}: the cells moved");
            }
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
            // One ending: a labelled repair, or on the two arms only a
            // kernel defect reaches, the shared dead end.
            assert_eq!(
                test_utils::refusal::recourse_markers(&msg),
                1,
                "not exactly one ending: {msg}"
            );
            if matches!(
                arm,
                PatchBoundError::RefinementFailed | PatchBoundError::DerivedKnots
            ) {
                assert!(msg.ends_with(geom_core::KERNEL_DEFECT_ENDING), "{msg}");
            }
            let lower = msg.to_lowercase();
            assert!(
                RECOURSE_WORDS.iter().any(|w| lower.contains(w)),
                "no recourse in: {msg}"
            );
        }
    }

    /// **Colliding split points do not zero a second partial.**
    /// Equal-split points collide with each other on a span a few ulps
    /// wide, so refinement can raise a multiplicity to `p` after the C¹
    /// gate passed, and the derived vector stops being clamped. That
    /// must refuse typed or bound the partial — never read it as zero.
    /// `x = Σ i³·N_i(u)` on a degree-3 `u` with knots `½, ½ + 3 ulp` has
    /// `S_uu ≠ 0`; both arms (unit and non-unit weights), at two split
    /// counts.
    #[test]
    #[allow(clippy::unwrap_used)]
    fn colliding_split_points_do_not_zero_the_second_partial() {
        let (a, b) = (0.5_f64, f64::from_bits(0.5_f64.to_bits() + 3));
        let ku =
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, a, b, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let (nu, nv) = (ku.control_count(), kv.control_count());
        let mut control = Vec::new();
        for i in 0..nu {
            for j in 0..nv {
                #[allow(clippy::cast_precision_loss)]
                let (x, y) = (i as f64, j as f64);
                control.push(geom_core::Point3::new(x * x * x, y, 0.0));
            }
        }
        for (arm, w) in [("integral", 1.0), ("rational", 1.5)] {
            let mut weights = vec![1.0; nu * nv];
            weights[nv] = w;
            let n = NurbsSurface::new(ku.clone(), kv.clone(), control.clone(), weights).unwrap();
            for splits in [8, 12] {
                match patch_cells_refined(&n, splits) {
                    // A typed refusal is an honest answer: `DerivedKnots`
                    // at 8 splits; at 12 the insertion chain itself
                    // refuses, `RefinementFailed`.
                    Err(e) => assert_eq!(
                        e,
                        if splits == 8 {
                            PatchBoundError::DerivedKnots
                        } else {
                            PatchBoundError::RefinementFailed
                        },
                        "{arm} @ {splits}"
                    ),
                    Ok(cells) => assert!(
                        cells.iter().any(|c| c.s_uu[0].hi() > 0.0),
                        "{arm} @ {splits}: x = Σ i³·N_i(u) has S_uu ≠ 0, but every cell's \
                         S_uu.x is {:?}",
                        cells[0].s_uu[0]
                    ),
                }
            }
        }
    }

    /// **The meters' vector sups bound every sampled magnitude, and sit
    /// within a measured factor of the box readings they replace**, on
    /// both arms and through a degree-1 direction. Each cell's
    /// `s_u_sup`, `s_v_sup` and `m_sup` is checked against `‖S_u‖`,
    /// `‖S_v‖` and `‖S_u × S_v‖` evaluated on a 5×5 grid over the cell,
    /// and against the cell's box readings in the fixture's own frame
    /// (`norm_sup` of the componentwise enclosures, and for `m` the
    /// smaller of the box cross product's and the Gram determinant's upper
    /// end).
    ///
    /// The fixtures: an integral biquadratic with interior knots, its
    /// rational twin with weights varying along both directions, a
    /// rational biquadratic whose weights vary strongly along `v` and
    /// whose `S − c` is not collinear with `Ã_u`, and a rational patch of
    /// bidegree `(2, 1)` whose weights vary along the degree-1 direction,
    /// which a Möbius reparameterization makes genuinely curve in
    /// parameter. Dropping the `(S − c)·w_d` term of either rational
    /// speed, either `w_d·Ã_d × (S − c)` term of `m_sup`, or any of the
    /// centred cross bound's three spread terms reads under a sampled
    /// magnitude; a speed loosened past rounding (the triangle relaxation
    /// `(max‖k‖ + max|g|·max‖q‖)/inf w` reads up to `1.125`× the box) or an
    /// `m_sup` loosened by half reads over its box ceiling.
    ///
    /// The samples are `f64` evaluations, so the soundness comparison
    /// allows `1e-12` of the sampled value for their rounding, decades
    /// under what a dropped term costs.
    #[test]
    fn the_meter_sups_bound_every_sampled_magnitude() {
        use geom_core::interval::{dot, norm_sq};
        // Against the box reading in the fixture's own frame, at four splits
        // per span. The speeds read every term the box holds at every
        // offset it holds, so in ℝ they are never above it, and measure at
        // most `1 + 2.2e-16` of it: the ceiling is rounding's. `m_sup` takes
        // the triangle inequality where the box keeps the cancellation
        // between `Ã_d` and the `S − c` terms; it measures at worst `1.44`×
        // (strong-`v`; `1.35`× the degree-1 fixture, `1.21`× the rational
        // twin, `1.02`× integral), and its ceiling is that with about a
        // tenth to spare.
        const SPEED_CEILING: f64 = 1.0 + 1e-12;
        const M_CEILING: f64 = 1.6;
        let biquadratic = |rational: bool| {
            let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.25, 0.5, 0.75, 1.0, 1.0, 1.0], 2)
                .expect("valid");
            let mut control = Vec::new();
            let mut weights = Vec::new();
            for i in 0..6u32 {
                for j in 0..6u32 {
                    let (u, v) = (f64::from(i) * 0.2, f64::from(j) * 0.2);
                    control.push(geom_core::Point3::new(
                        u + 0.1 * v * v,
                        v,
                        0.3 * u * (1.0 - u) + 0.2 * v * v,
                    ));
                    weights.push(if rational {
                        1.0 + 0.6 * f64::from((i * 6 + j) % 5) / 4.0
                    } else {
                        1.0
                    });
                }
            }
            NurbsSurface::new(kv.clone(), kv, control, weights).expect("valid")
        };
        // Found by a seeded search for a patch on which the `w_v` term of
        // `m_sup` is what keeps the bound above the sample (the corner
        // `(0, 1)` at four splits). The middle `v` column carries the
        // large weights.
        let strong_v = {
            let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).expect("valid");
            let control = [
                (-0.891, -0.073, -0.646),
                (0.257, -0.311, -0.081),
                (0.577, -0.584, 0.283),
                (-0.841, -0.157, 0.184),
                (0.203, 0.789, 0.876),
                (0.146, 0.698, 0.284),
                (0.739, -0.405, 0.519),
                (0.801, -0.122, -0.214),
                (0.903, -0.323, -0.794),
            ]
            .map(|(x, y, z)| geom_core::Point3::new(x, y, z))
            .to_vec();
            let weights = vec![
                0.287, 2.163, 0.446, 0.227, 0.857, 0.908, 0.399, 7.160, 0.861,
            ];
            NurbsSurface::new(kv.clone(), kv, control, weights).expect("valid")
        };
        let ruled = {
            let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).expect("valid");
            let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).expect("valid");
            let control = [
                (1.0, 0.0, 0.0),
                (1.0, 0.0, 1.0),
                (1.0, 1.0, 0.0),
                (1.0, 1.0, 1.2),
                (0.0, 1.0, 0.0),
                (0.0, 1.0, 0.9),
            ]
            .map(|(x, y, z)| geom_core::Point3::new(x, y, z))
            .to_vec();
            let weights = vec![1.0, 2.5, 0.7, 1.6, 1.0, 0.5];
            NurbsSurface::new(ku, kv, control, weights).expect("valid")
        };
        let names = ["‖S_u‖", "‖S_v‖", "‖S_u × S_v‖"];
        for (name, n) in [
            ("integral", biquadratic(false)),
            ("rational", biquadratic(true)),
            ("rational strong v", strong_v),
            ("rational degree-1 v", ruled),
        ] {
            let cells = patch_cells_refined(&n, 4).expect("covered");
            assert!(cells.len() >= 16, "{name}: {} cells", cells.len());
            for c in &cells {
                let sups = [c.s_u_sup, c.s_v_sup, c.m_sup];
                for a in 0..5u32 {
                    for b in 0..5u32 {
                        let u = c.u.0 + (c.u.1 - c.u.0) * f64::from(a) / 4.0;
                        let v = c.v.0 + (c.v.1 - c.v.0) * f64::from(b) / 4.0;
                        let jet = n.window_at(u, v).expect("in domain").ders_in_span(u, v);
                        let got = [jet.du.norm(), jet.dv.norm(), jet.du.cross(jet.dv).norm()];
                        for (q, g) in got.iter().enumerate() {
                            assert!(
                                *g <= sups[q] + 1e-12 * g,
                                "{name}: {} = {g:e} at ({u}, {v}) above the cell's sup {:e}",
                                names[q],
                                sups[q]
                            );
                        }
                    }
                }
                let gram = (norm_sq(&c.s_u) * norm_sq(&c.s_v) - dot(&c.s_u, &c.s_v).sqr())
                    .clamped_to(0.0, f64::INFINITY)
                    .sqrt()
                    .hi();
                let boxed = [
                    norm_sup(&c.s_u),
                    norm_sup(&c.s_v),
                    norm_sup(&cross(&c.s_u, &c.s_v)).min(gram),
                ];
                for q in 0..3 {
                    let ceiling = if q == 2 { M_CEILING } else { SPEED_CEILING };
                    assert!(
                        sups[q] <= ceiling * boxed[q],
                        "{name}: sup {} = {:e} is {}× the box reading {:e} on {:?} × {:?}",
                        names[q],
                        sups[q],
                        sups[q] / boxed[q],
                        boxed[q],
                        c.u,
                        c.v
                    );
                }
            }
        }
    }

    /// **The screened pair norm is the full enumeration's.** Random
    /// affine terms and offsets, some with enclosures a tenth wide so the
    /// screen's radius terms carry weight, some with near-tied terms: the
    /// screened maximum is never above the enumeration's `norm_sup` over
    /// every pair (it reads a subset) and never below it by more than
    /// rounding (what it skips is bounded by what it read). One refused
    /// coordinate anywhere refuses the read, as the enumeration does.
    #[test]
    fn the_screened_pair_norm_is_the_full_enumerations() {
        use test_utils::fuzz;
        let mut rng = fuzz::start("patch_bound::screened_pair_norm");
        let iv = |rng: &mut fuzz::Rng, scale: f64, wide: bool| {
            let c = rng.range(-scale, scale);
            let r = if wide {
                rng.range(0.0, 0.1 * scale)
            } else {
                0.0
            };
            Interval::from_bounds(c - r, c + r)
        };
        for case in 0..fuzz::scaled(400) {
            let wide = case % 2 == 0;
            let tie = case % 3 == 0;
            let affine = |rng: &mut fuzz::Rng| Affine {
                k: core::array::from_fn(|_| iv(rng, 3.0, wide)),
                g: iv(rng, if tie { 1e-3 } else { 2.0 }, wide),
            };
            let (nx, ny, nq) = (1 + rng.below(12), 1 + rng.below(12), 1 + rng.below(16));
            let xs: Vec<Affine> = (0..nx).map(|_| affine(&mut rng)).collect();
            let ys: Vec<Affine> = (0..ny).map(|_| affine(&mut rng)).collect();
            let qs: Vec<[Interval; 3]> = (0..nq)
                .map(|_| core::array::from_fn(|_| iv(&mut rng, 0.5, wide)))
                .collect();
            let got = pair_norm_sup(&qs, &xs, &ys);
            let mut all = f64::NEG_INFINITY;
            for q in &qs {
                for x in &xs {
                    for y in &ys {
                        let (a, b) = (x.at(*q), y.at(*q));
                        all = all.max(norm_sup(&core::array::from_fn(|k| a[k] + b[k])));
                    }
                }
            }
            assert!(
                got <= all && got >= all * (1.0 - 64.0 * f64::EPSILON),
                "case {case} (wide {wide}, tied {tie}, {nx}×{ny} terms at {nq} points): \
                 screened {got} against the enumeration's {all} — {}",
                fuzz::replay()
            );
            let mut refused = xs.clone();
            refused[0].k[1] = Interval::refused();
            assert!(
                pair_norm_sup(&qs, &refused, &ys).is_nan(),
                "case {case}: a refused coordinate must refuse the read — {}",
                fuzz::replay()
            );
        }
    }

    /// The full enumeration [`pair_norm_sup`] screens: `norm_sup` of every
    /// pair at every offset, folded keeping `NaN`.
    fn enumerated_pair_norm(qs: &[[Interval; 3]], xs: &[Affine], ys: &[Affine]) -> f64 {
        let mut all: Option<f64> = None;
        for q in qs {
            for x in xs {
                for y in ys {
                    let (a, b) = (x.at(*q), y.at(*q));
                    let m = norm_sup(&core::array::from_fn(|k| a[k] + b[k]));
                    all = Some(all.map_or(m, |s| max_bound(s, m)));
                }
            }
        }
        all.unwrap_or(f64::NAN)
    }

    /// **An unbounded or unformable radius screens nothing out.** Written
    /// down, not searched: offsets spanning the whole finite range (whose
    /// radius overflows to `+∞`) beside terms that do not read them
    /// (`g` exactly zero, where `0·∞` would be `NaN` and a `NaN`-dropping
    /// fold would shrink the radius to nothing); the same with terms that
    /// do read them; offsets near overflow; and terms whose own enclosure
    /// is unbounded. On each the screened read must be the enumeration's:
    /// both refused, or the screened one at most the enumeration's and
    /// below it only by rounding.
    #[test]
    fn an_unbounded_radius_screens_nothing_out() {
        let iv = Interval::from_bounds;
        let pt = Interval::point;
        let zero = Affine {
            k: [pt(0.0); 3],
            g: pt(0.0),
        };
        let big = f64::MAX;
        let whole = [[iv(-big, big); 3]];
        let near = [[iv(-big / 4.0, big / 4.0); 3], [pt(0.0); 3]];
        let wide_k = Affine {
            k: [iv(-10.0, 10.0), pt(0.0), pt(0.0)],
            g: pt(0.0),
        };
        let unit_k = Affine {
            k: [pt(1.0), pt(0.0), pt(0.0)],
            g: pt(0.0),
        };
        let reads_q = Affine {
            k: [pt(1.0), pt(0.0), pt(0.0)],
            g: pt(1e-300),
        };
        let unbounded_k = Affine {
            k: [iv(-big, big), pt(0.0), pt(0.0)],
            g: pt(0.0),
        };
        type Case<'a> = (&'a str, &'a [[Interval; 3]], Vec<Affine>);
        let cases: [Case<'_>; 5] = [
            (
                "whole-range offsets, terms that do not read them",
                &whole,
                vec![wide_k, unit_k],
            ),
            ("the same, the wide term last", &whole, vec![unit_k, wide_k]),
            (
                "whole-range offsets, a term that reads them",
                &whole,
                vec![unit_k, reads_q],
            ),
            (
                "offsets near overflow",
                &near,
                vec![unit_k, wide_k, reads_q],
            ),
            ("an unbounded term", &near, vec![unit_k, unbounded_k]),
        ];
        for (name, qs, xs) in cases {
            let got = pair_norm_sup(qs, &xs, &[zero]);
            let all = enumerated_pair_norm(qs, &xs, &[zero]);
            assert!(
                (got.is_nan() && all.is_nan())
                    || (got <= all && got >= all * (1.0 - 64.0 * f64::EPSILON)),
                "{name}: screened {got} against the enumeration's {all}"
            );
        }
    }

    /// `a + b` exactly, as an unevaluated sum.
    fn two_sum(a: f64, b: f64) -> (f64, f64) {
        let s = a + b;
        let v = s - a;
        (s, (a - (s - v)) + (b - v))
    }

    /// `a·b` exactly, as an unevaluated sum.
    fn two_prod(a: f64, b: f64) -> (f64, f64) {
        let p = a * b;
        (p, a.mul_add(b, -p))
    }

    /// The sign of `Σ terms` read in double-double, with anything within
    /// `1e-28` of the largest term's magnitude called zero: far below
    /// the one-ulp misses the checks below are built to see.
    fn dd_sign(terms: &[f64]) -> core::cmp::Ordering {
        let (mut hi, mut lo) = (0.0_f64, 0.0_f64);
        for &t in terms {
            let (s, e) = two_sum(hi, t);
            hi = s;
            lo += e;
        }
        let total = hi + lo;
        let scale = terms.iter().fold(0.0_f64, |m, t| m.max(t.abs()));
        if total.abs() <= 1e-28 * scale {
            core::cmp::Ordering::Equal
        } else {
            total.total_cmp(&0.0)
        }
    }

    /// **The `f64` screen's centres, radii and bound hold against exact
    /// arithmetic.** Random enclosures over magnitudes from `1e-100` to
    /// `1e100`, with near-tie cases (the bound's own rounding the only
    /// margin):
    ///
    /// * [`Centred::of`]: both ends of every axis lie within the radius
    ///   of the centre, checked exactly;
    /// * [`Centred::of_affine`]: every corner `k − g·q` of the three
    ///   enclosures (the extremes of a bilinear form), formed exactly
    ///   with a fused multiply, lies within its radius of its centre;
    /// * [`screen_bound`]: its square is at least the exact squared norm
    ///   of the far corner of the two boxes' sum, `Σ(|m_k| + r)²`, with
    ///   the centres' sum formed exactly.
    ///
    /// What a pure-`f64` row can and cannot see: dropping the screen's
    /// `1 + 32ε`, `Centred::of`'s rounding up, or the screen's refusal
    /// of a nonzero centre whose norm underflows to `0` goes red here
    /// (the mutations were run). [`Centred::of_affine`]'s `1 + 8ε` and its
    /// smallest-normal term sit beside a `4ε` centre term that already
    /// covers the rounding they guard on every normal-range input, so
    /// dropping either one alone stays green: they guard against the
    /// radius sum's own rounding, which this row cannot isolate.
    #[test]
    fn the_screen_holds_against_exact_arithmetic() {
        use core::cmp::Ordering::Less;
        use test_utils::fuzz;
        let mut rng = fuzz::start("enclose::screen_exact");
        let mag = |rng: &mut fuzz::Rng| {
            let m = 10f64.powf(rng.range(-100.0, 100.0));
            if rng.below(2) == 0 { m } else { -m }
        };
        for case in 0..fuzz::scaled(4000) {
            let tie = case % 3 == 0;
            // Components whose squares underflow to zero: a norm that
            // reads `0` for a centre that is not.
            let tiny = case % 7 == 0;
            let scale = 10f64.powf(rng.range(-100.0, 100.0));
            let side = |rng: &mut fuzz::Rng| {
                if tiny {
                    return Interval::point(rng.range(-1.0, 1.0) * 1e-165);
                }
                let c = if tie {
                    scale * rng.range(0.5, 1.0)
                } else {
                    mag(rng)
                };
                match case % 4 {
                    0 => Interval::point(c),
                    1 => Interval::from_bounds(c - c.abs() * 1e-17, c + c.abs() * 1e-17),
                    // Ends of unrelated magnitude, where `hi − mid` rounds.
                    2 => {
                        let d = mag(rng);
                        Interval::from_bounds(c.min(d), c.max(d))
                    }
                    _ => {
                        let r = c.abs() * rng.range(0.0, 0.5);
                        Interval::from_bounds(c - r, c + r)
                    }
                }
            };
            let k: [Interval; 3] = core::array::from_fn(|_| side(&mut rng));
            let g = if tiny {
                Interval::point(0.0)
            } else {
                side(&mut rng)
            };
            let q: [Interval; 3] = core::array::from_fn(|_| side(&mut rng));
            if !(k.iter().chain(&q).all(|c| c.is_certified()) && g.is_certified()) {
                continue;
            }
            let (ck, cg, cq) = (Centred::of(&k), Centred::of(&[g, g, g]), Centred::of(&q));
            for (v, c) in [(&k, &ck), (&q, &cq)] {
                for (i, (vi, mid)) in v.iter().zip(c.mid).enumerate() {
                    for end in [vi.lo(), vi.hi()] {
                        let (d, e) = two_sum(end, -mid);
                        assert!(
                            dd_sign(&[c.rad, -d.abs(), -e * d.signum()]) != Less,
                            "case {case}: end {end} of axis {i} is outside {:?} ± {} — {}",
                            c.mid,
                            c.rad,
                            fuzz::replay()
                        );
                    }
                }
            }
            let img = Centred::of_affine(&ck, &cg, &cq);
            for i in 0..3 {
                for kv in [k[i].lo(), k[i].hi()] {
                    for gv in [g.lo(), g.hi()] {
                        for qv in [q[i].lo(), q[i].hi()] {
                            let (p, pe) = two_prod(gv, qv);
                            let (d, de) = two_sum(kv, -p);
                            let (d2, de2) = two_sum(d, -img.mid[i]);
                            let lo_part = de - pe + de2;
                            let sign = if d2 + lo_part < 0.0 { -1.0 } else { 1.0 };
                            assert!(
                                dd_sign(&[img.rad, -sign * d2, -sign * de, sign * pe, -sign * de2])
                                    != Less,
                                "case {case}: corner k {kv} − g {gv}·q {qv} on axis {i} is \
                                 outside {} ± {} — {}",
                                img.mid[i],
                                img.rad,
                                fuzz::replay()
                            );
                        }
                    }
                }
            }
            let other = if tiny {
                Centred {
                    mid: [0.0; 3],
                    rad: 0.0,
                }
            } else {
                Centred::of(&core::array::from_fn(|_| side(&mut rng)))
            };
            let bound = screen_bound(&img, &other);
            if !bound.is_finite() {
                continue;
            }
            // Scaled up by an exact power of two so no square below
            // underflows: the comparison is homogeneous.
            let top = img
                .mid
                .iter()
                .chain(&other.mid)
                .chain([&img.rad, &other.rad, &bound])
                .fold(0.0_f64, |m, x| m.max(x.abs()));
            let up = if top > 0.0 && top < 1.0 {
                2f64.powi(-(top.log2().floor() as i32))
            } else {
                1.0
            };
            let (b2, b2e) = two_prod(bound * up, bound * up);
            let (r, re) = two_sum(img.rad * up, other.rad * up);
            let mut terms = vec![b2, b2e];
            for i in 0..3 {
                let (m, me) = two_sum(img.mid[i] * up, other.mid[i] * up);
                let (a, ae) = two_sum(m.abs(), r);
                let ae = ae + me * m.signum() + re;
                let (s, se) = two_prod(a, a);
                terms.extend([-s, -se, -2.0 * a * ae]);
            }
            assert!(
                dd_sign(&terms) != Less,
                "case {case}: the screen {bound} is below the far corner of {:?} ± {} \
                 plus {:?} ± {} — {}",
                img.mid,
                img.rad,
                other.mid,
                other.rad,
                fuzz::replay()
            );
        }
    }
}
