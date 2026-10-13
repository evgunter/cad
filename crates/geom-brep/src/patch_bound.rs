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
//! **The signed reading is the only one**: a consumer wanting a
//! magnitude takes `norm_sq` of a signed enclosure and never a
//! magnitude recurrence.
//!
//! # The rational arm
//!
//! A RATIONAL patch's partials are quotients with polynomial
//! numerators — `S_u = Ã_u/W²` and `S_u × S_v = M̃/W³` at the first
//! order, `S_uu`, `S_uv`, `S_vv` as `M/W³` at the second — and the
//! arm reads every one of them from one statement,
//! [`geom_core::spline::cell_second_quotients`]
//! ([`geom_core::spline::derivative`], "The surface statement" and
//! "The second order"): on each exact dyadic piece of a span cell,
//! numerator and denominator are raised to one degree, and the
//! quantity is a convex combination of the coefficient ratios
//! `m_k/b_k`. So per piece:
//!
//! - each coordinate lies between its least and greatest ratio — the
//!   signed enclosures [`PatchCell::s_u`] … [`PatchCell::s_vv`]
//!   ([`QuotientPiece::coordinate_range`]);
//! - the norm is at most the largest `‖m_k‖/b_k` —
//!   [`PatchCell::s_u_sup`], [`PatchCell::s_v_sup`],
//!   [`PatchCell::m_sup`];
//! - the area's norm is at least the least ratio projected on the
//!   piece's own direction — [`PatchCell::m_inf`].
//!
//! The cells are those pieces, [`RATIONAL_CERT_SPLITS`] per span
//! direction rounded down to a count exact at the second order's
//! degree `3p` ([`geom_core::spline::pieces::piece_count`]). Each
//! piece is cut from the described net in interval arithmetic,
//! restriction being exact in ℝ, so what the cells enclose is the
//! described patch; there is no refined net on this arm. A
//! direction of degree above 18 puts the second order over
//! [`geom_core::spline::QUOTIENT_MAX_DEGREE`], and the arm refuses
//! ([`PatchBoundError::RefinementFailed`]).
//!
//! # Conservatism
//!
//! The answer is a bound, not an estimate. Its slack is the spread of
//! the coefficient ratios over a piece, which shrinks with the piece;
//! extreme weight ratios widen it. The cost is only how finely a
//! consumer must subdivide; the bound is never wrong.
//!
//! # Refusal (fail-loud, D4 ¶2)
//!
//! Structural refusals are typed ([`PatchBoundError`]); arithmetic
//! failures are refusals, and a refused hull fails every `≤ ε`
//! comparison it reaches.

use geom_core::Bounds;
use std::ops::RangeInclusive;

use geom::surfaces::NurbsSurface;
use geom_core::interval::certification::Certification;
use geom_core::interval::{Interval, certified_hi, cross, max_bound, norm_sup};
use geom_core::spline::net::{TensorChannels, TensorCoeffs, TensorNet};
use geom_core::spline::pieces::piece_count;
use geom_core::spline::{
    C1Break, CellAxis, CellBlock, Derived, KnotVector, QuotientPiece, Span, cell_quotients,
    cell_second_quotients,
};

/// The fixed piece count of the RATIONAL arm: every nonempty span of
/// every direction is read on this many exact dyadic pieces (rounded
/// down to a count exact at the second order's degree, module docs). A
/// CONSTANT (D9: structure, never a data-dependent iteration), and this
/// arm's own: `geom::curves`' rational meters count their pieces by
/// their own constant, which prices one curve bound against the
/// refusal frontier its own tests pin, while this one prices the
/// per-cell partial readings every [`patch_cells`] consumer reads.
/// Cutting is exact in ℝ, so it changes no geometry; it only narrows
/// each piece's spread of coefficient ratios.
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
/// **The DESCRIBED patch, on both arms, with no carve-out.** Where
/// the integral arm refines it inserts knots into certification
/// enclosures of the described net
/// ([`geom_core::spline::net::TensorNet::refine_u`]), each Boehm ratio
/// an outward-rounded quotient of the knots it is made of, so the
/// insertion widens like every later step instead of rounding the net
/// to a nearby one; the rational arm cuts its pieces from the described
/// net the same way (module docs, "The rational arm").
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
/// What it costs is width, and the width is the honest one: an
/// enclosure says what the arithmetic it was formed in can say. On
/// that ruled face the quotient's coefficients of `S_vv` are exact
/// zeros, so the zero reads as one; a reading formed through more
/// arithmetic would carry its rounding as dust around zero. A consumer
/// asking whether zero is in there gets the right answer either way.
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
    /// Euclidean norms of coefficient VECTORS (D4 ¶2), so a rigid map
    /// moves it only by rounding: on the integral arm the derived
    /// control vectors active on the cell; on the rational arm the
    /// largest `‖ã_k‖/b_k` of the quotient `Ã_u/W²` on the piece
    /// (module docs, "The rational arm"). `NaN` when refused.
    pub s_u_sup: f64,
    /// Certified upper bound on `‖S_v‖` over the cell, read as
    /// [`Self::s_u_sup`] is.
    pub s_v_sup: f64,
    /// Certified upper bound on `‖S_u × S_v‖` over the cell, read from
    /// coefficient vectors as [`Self::s_u_sup`] is: on the integral arm
    /// the cross product of the two hulls of active derived control
    /// vectors, bounded about their centres; on the rational arm the
    /// largest `‖m̃_k‖/b_k` of the area quotient `M̃/W³`. `NaN` when
    /// refused.
    pub m_sup: f64,
    /// Certified LOWER bound on `‖S_u × S_v‖` over the cell: the area
    /// quotient's own floor ([`QuotientPiece::inf_norm`], read from
    /// below), on the rational arm over `W³` and on the integral arm
    /// over unit weights, one piece per span cell. `0.0` — the floor's
    /// conservative answer — when the reading does not certify or
    /// separates nothing from zero.
    pub m_inf: f64,
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
/// cell's enclosure is as wide as its coefficients spread — on the
/// integral arm a hull over the `(p + 1)`-span support of the basis
/// functions active there, on the rational arm the coefficient ratios
/// of a piece — and that width shrinks only LINEARLY in the cell size.
/// A sup-side consumer barely notices; an inf-side one does, because
/// the width is subtracted from the quantity it is trying to prove
/// positive. The measured case is the exactly-umbilic sphere band
/// (`r = 2`): at 16 splits the first fundamental form's own `F` —
/// identically zero on an orthogonal chart — encloses as
/// `[−0.094, 0.094]` on its widest cell, and the certified curvature
/// range `[−0.75, −0.27]` is about as wide as the surface's single
/// value `−0.5`.
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

/// A coefficient net as certification enclosures — the shared tensor assembly,
/// homed in [`geom_core::spline::net`] (issue 1006). The alias is kept
/// so this module's own prose and its consumers keep naming the thing
/// they read; the differencing is not this module's any more.
pub type Net = TensorNet;

/// The signed hull of `net[i][j]` over the window `wu × wv` —
/// [`TensorNet::window_hull`], re-exported under the name this
/// module's consumers already use.
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
/// **The ARITHMETIC diverges, and that is what is left.** This module
/// extracts `w·P`; `offset_fit::channel` extracts `w·(P − c)` against a
/// WHOLE-PATCH recentring origin, because its net feeds polynomial
/// products formed once over the merged break structure, where interval
/// arithmetic's rounding scales with the coordinate. This module's
/// rational cells read the quotient pieces
/// ([`geom_core::spline::cell_second_quotients`]) in the frame of each
/// cell's first active point. What the two still share is the ORDER
/// (`weight · coordinate`), and a change to THAT is a change to both.
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
/// (`S_u, S_v, S_uu, S_uv, S_vv`, in that order), the upper bounds
/// on `‖S_u‖`, `‖S_v‖` and `‖S_u × S_v‖`, and the lower bound on
/// `‖S_u × S_v‖`.
fn cell_from(
    uv: ((f64, f64), (f64, f64)),
    signed: [[Interval; 3]; 5],
    [s_u_sup, s_v_sup, m_sup, m_inf]: [f64; 4],
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
        m_inf,
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

/// [`vectors_norm_sup`] over the vector B-spline whose coordinates are
/// `nets`, on the cell whose active window is `wu × wv`.
fn window_norm_sup(nets: [&Net; 3], wu: &RangeInclusive<usize>, wv: &RangeInclusive<usize>) -> f64 {
    vectors_norm_sup(&window_vectors(wu, wv, |c, i, j| nets[c].get(i, j)))
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
                    .hull(w.u_d2.as_ref(), |net, wu2| window_hull(net, wu2, &w.v_val));
                let g02 = d
                    .d02
                    .hull(w.v_d2.as_ref(), |net, wv2| window_hull(net, &w.u_val, wv2));
                let g11 = window_hull(&d.d11, &w.u_d1, &w.v_d1);
                let g10 = window_hull(&d.d10, &w.u_d1, &w.v_val);
                let g01 = window_hull(&d.d01, &w.u_val, &w.v_d1);
                s_uu[c] = g20;
                s_vv[c] = g02;
                s_uv[c] = g11;
                s_u[c] = g10;
                s_v[c] = g01;
            }
            let s_u_sup = window_norm_sup(
                [&nets[0].d10, &nets[1].d10, &nets[2].d10],
                &w.u_d1,
                &w.v_val,
            );
            let s_v_sup = window_norm_sup(
                [&nets[0].d01, &nets[1].d01, &nets[2].d01],
                &w.u_val,
                &w.v_d1,
            );
            let m_sup = cross_norm_sup(
                &window_vectors(&w.u_d1, &w.v_val, |c, i, j| nets[c].d10.get(i, j)),
                &window_vectors(&w.u_val, &w.v_d1, |c, i, j| nets[c].d01.get(i, j)),
            );
            let m_inf = integral_area_floor(base_nets, span_u, span_v)?;
            cells.push(cell_from(
                (span_extent(kv_u, su), span_extent(kv_v, sv)),
                [s_u, s_v, s_uu, s_uv, s_vv],
                [s_u_sup, s_v_sup, m_sup, m_inf],
            ));
        }
    }
    Ok(cells)
}

/// The RATIONAL arm: the quotient statement on each nonempty cell's
/// exact dyadic pieces, `splits` per direction (module docs).
fn rational_cells(n: &NurbsSurface<f64>, splits: usize) -> Result<Vec<PatchCell>, PatchBoundError> {
    // The licence, on f64 STRUCTURE: `NurbsSurface::new` refuses these at
    // the door; re-checked here so THIS bound never reads an unproven
    // weight.
    if !n
        .weights()
        .iter()
        .all(|w| geom_core::spline::weight_in_regime(*w))
    {
        return Err(PatchBoundError::NonPositiveWeight);
    }
    let (kv_u, kv_v) = (n.knots_u(), n.knots_v());
    let held = |kv: &KnotVector| -> Vec<Interval> {
        kv.knots().iter().map(|k| Interval::point(*k)).collect()
    };
    let (held_u, held_v) = (held(kv_u), held(kv_v));
    let nv = kv_v.control_count();
    let mut cells: Vec<PatchCell> = Vec::new();
    for su in (kv_u.first_span()..=kv_u.last_span()).filter_map(|i| kv_u.span(i)) {
        for sv in (kv_v.first_span()..=kv_v.last_span()).filter_map(|i| kv_v.span(i)) {
            let mut points = Vec::with_capacity((su.degree() + 1) * (sv.degree() + 1));
            let mut weights = Vec::with_capacity(points.capacity());
            for iu in su.window() {
                for iv in sv.window() {
                    let (pt, w) = (n.control()[iu * nv + iv], n.weights()[iu * nv + iv]);
                    points.push([pt.x, pt.y, pt.z].map(Interval::point));
                    weights.push(Interval::point(w));
                }
            }
            let pieces = cell_second_quotients(
                (CellAxis::Span(su, &held_u), CellAxis::Span(sv, &held_v)),
                CellBlock::Points {
                    origin: points[0],
                    points: &points,
                    weights: &weights,
                },
                (splits, splits),
            )
            .ok_or(PatchBoundError::RefinementFailed)?;
            // The pieces are counted for the second-order quotients'
            // degree (`cell_second_quotients`).
            let counts = (
                piece_count(3 * su.degree(), splits),
                piece_count(3 * sv.degree(), splits),
            );
            let (eu, ev) = (span_extent(kv_u, su.index()), span_extent(kv_v, sv.index()));
            for (k, (first, second)) in pieces.iter().enumerate() {
                let (iu, iv) = (k / counts.1, k % counts.1);
                cells.push(cell_from(
                    (
                        piece_extent(eu, iu, counts.0),
                        piece_extent(ev, iv, counts.1),
                    ),
                    [
                        signed_box(&first.u),
                        signed_box(&first.v),
                        signed_box(&second.uu),
                        signed_box(&second.uv),
                        signed_box(&second.vv),
                    ],
                    [
                        certified_sup(&first.u),
                        certified_sup(&first.v),
                        first.area.as_ref().map_or(f64::NAN, certified_sup),
                        first.area.as_ref().map_or(0.0, certified_inf),
                    ],
                ));
            }
        }
    }
    Ok(cells)
}

/// Piece `i` of `count` equal pieces of `[lo, hi]`, in `f64`: the
/// dyadic points the pieces are cut at, rounded as
/// [`geom_core::spline::algebra::equal_split_points`] rounds them.
fn piece_extent((lo, hi): (f64, f64), i: usize, count: usize) -> (f64, f64) {
    #[allow(clippy::cast_precision_loss)]
    let at = |k: usize| {
        if k == 0 {
            lo
        } else if k == count {
            hi
        } else {
            lo + (hi - lo) * (k as f64 / count as f64)
        }
    };
    (at(i), at(i + 1))
}

/// The signed componentwise box of a quotient over its piece: each
/// coordinate between its least and greatest coefficient ratio
/// ([`QuotientPiece::coordinate_range`]).
fn signed_box(q: &QuotientPiece<Interval, 3>) -> [Interval; 3] {
    q.coordinate_range().map(|(lo, hi)| Interval::hull(lo, hi))
}

/// The quotient's norm sup over its piece, read from above; `NaN` when
/// it does not certify.
fn certified_sup(q: &QuotientPiece<Interval, 3>) -> f64 {
    let s = q.sup_norm();
    if s.is_certified() { s.hi() } else { f64::NAN }
}

/// The quotient's norm inf over its piece, read from below; `0.0` when
/// it does not certify or does not separate the piece from zero.
fn certified_inf(q: &QuotientPiece<Interval, 3>) -> f64 {
    let s = q.inf_norm();
    if s.is_certified() {
        s.lo().max(0.0)
    } else {
        0.0
    }
}

/// The integral arm's [`PatchCell::m_inf`] on the span cell
/// `span_u × span_v`: the area quotient over unit weights, one piece
/// ([`cell_quotients`] at splits `(1, 1)`), so both arms read the floor
/// by one statement.
///
/// # Errors
///
/// [`PatchBoundError::RefinementFailed`] when the statement refuses
/// the block (a degree above its ceiling).
fn integral_area_floor(
    nets: &TensorChannels<'_, 3>,
    span_u: Span<'_>,
    span_v: Span<'_>,
) -> Result<f64, PatchBoundError> {
    let held = |kv: &KnotVector| -> Vec<Interval> {
        kv.knots().iter().map(|k| Interval::point(*k)).collect()
    };
    let (held_u, held_v) = (held(nets.knots_u()), held(nets.knots_v()));
    let channels = nets.channels();
    let mut points = Vec::new();
    for iu in span_u.window() {
        for iv in span_v.window() {
            points.push(core::array::from_fn(|c| channels[c].net().get(iu, iv)));
        }
    }
    let weights = vec![Interval::point(1.0); points.len()];
    let pieces = cell_quotients(
        (
            CellAxis::Span(span_u, &held_u),
            CellAxis::Span(span_v, &held_v),
        ),
        CellBlock::Points {
            origin: points[0],
            points: &points,
            weights: &weights,
        },
        (1, 1),
        true,
    )
    .ok_or(PatchBoundError::RefinementFailed)?;
    Ok(pieces
        .iter()
        .map(|q| q.area.as_ref().map_or(0.0, certified_inf))
        .fold(f64::INFINITY, f64::min))
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;

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
    /// parameter. Dropping any of the integral arm's centred cross
    /// bound's three spread terms reads under a sampled magnitude; a
    /// speed or `m_sup` loosened by a fifth reads over its box ceiling.
    /// The rational arm's terms are the quotient statement's, whose own
    /// rows drop them (`geom_core::spline::derivative`).
    ///
    /// The samples are `f64` evaluations, so the soundness comparison
    /// allows `1e-12` of the sampled value for their rounding, decades
    /// under what a dropped term costs.
    #[test]
    fn the_meter_sups_bound_every_sampled_magnitude() {
        use geom_core::interval::{dot, norm_sq};
        // Measured worst over the fixtures at four splits per span, against
        // the box reading in the fixture's own frame: the speeds at `1.0`×
        // on every fixture (to an ulp: on the rational arm the box is the
        // same quotient's coordinate range), `m_sup` at `1.024`× (the
        // integral arm; at most `1.0`× on the rational fixtures, `0.90`×
        // strong-`v`, where the box cross product loses the correlation
        // the area quotient keeps). Each ceiling is that worst with about
        // a tenth to spare.
        const SPEED_CEILING: f64 = 1.1;
        const M_CEILING: f64 = 1.13;
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
}
