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
//! — on both arms and with no carve-out, because the cut or the
//! refinement an arm performs is itself part of the enclosure
//! ([`PatchCell`], "What the enclosure encloses"). An inf-side consumer
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
//! direction, halved from the cell's quotients in interval arithmetic,
//! restriction being exact in ℝ, so what the cells enclose is the
//! described patch; there is no refined net on this arm. A
//! direction of degree above 18 puts the second order over
//! [`geom_core::spline::QUOTIENT_MAX_DEGREE`], and the arm refuses
//! typed ([`PatchBoundError::DegreeAboveCeiling`]; module docs,
//! "Refusal").
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
//!
//! **The degree ceilings.** Every quotient here is an exact Bernstein
//! product of degree at most [`QUOTIENT_MAX_DEGREE`] (`56`) per
//! direction. The rational arm reads its second partials at `3p`, so a
//! rational direction of degree above 18 refuses typed
//! ([`PatchBoundError::DegreeAboveCeiling`]). The integral arm needs a
//! quotient only for its area floor, `A_u × A_v` at `2p − 1`, so it
//! certifies to degree 28 with a floor and past that with the
//! conservative floor `0.0` ([`PatchCell::m_inf`]).
//!
//! **The practical ceiling can be lower on a wide weight spread.** The
//! weight power's coefficients are interval enclosures, and a spread
//! wide enough at a high enough degree lets the rounding of their
//! products reach zero (`geom_core::spline::derivative`'s module docs,
//! "The practical ceiling", which says why where is not pinned): the
//! quotient then does not certify, its sups
//! read `NaN` and its boxes refused, and every consumer's `≤ ε` check
//! refuses the face.

use geom_core::Bounds;
use std::borrow::Cow;
use std::ops::RangeInclusive;

#[cfg(test)]
use crate::recourse::{NARROW_SPAN, NARROW_SPAN_RECOURSE};
use geom::surfaces::NurbsSurface;
use geom_core::interval::certification::Certification;
use geom_core::interval::{Interval, certified_hi, cross, max_bound, norm_sup};
use geom_core::spline::net::{TensorChannels, TensorCoeffs, TensorNet};
use geom_core::spline::weight_scale::unit_scale_exact;
use geom_core::spline::{
    C1Break, CellAxis, CellBlock, Derived, KnotVector, QUOTIENT_MAX_DEGREE, QuotientPiece, Span,
    cell_second_quotients, polynomial_area_pieces,
};

/// The fixed piece count of the RATIONAL arm: every nonempty span of
/// every direction is read on this many equal pieces (module docs).
/// **Certificate resolution** (`crates/mesh/README.md`): it sets how
/// tight the rational bound is, and sizes no mesh directly. A
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
// The variant roster `topo`'s sample list and this module's tests
// iterate (this crate's `test-support` feature, or its own test build):
// fieldless, so the enum is its own discriminant.
#[cfg_attr(any(test, feature = "test-support"), derive(strum::EnumIter))]
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
    /// A refinement's knot-insertion chain refused: the narrow-span
    /// decision ([`NARROW_SPAN`](crate::recourse::NARROW_SPAN)), as [`Self::DerivedKnots`] is.
    RefinementFailed,
    /// A rational direction of degree above 18: the rational arm reads
    /// the second partials as quotients of degree `3p`, past the
    /// [`QUOTIENT_MAX_DEGREE`] of `56` its exact Bernstein products
    /// reach (module docs, "Refusal").
    DegreeAboveCeiling,
    /// A refined direction whose first derivative is discontinuous, a
    /// multiplicity the C¹ gate never saw: the narrow-span decision
    /// ([`NARROW_SPAN`](crate::recourse::NARROW_SPAN)), as [`Self::RefinementFailed`] is. The answer
    /// is a typed refusal, never a second partial read as zero.
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
            Self::RefinementFailed | Self::DerivedKnots => concat!(
                "NURBS face with ",
                crate::recourse::narrow_span!(),
                " for bounding. ",
                crate::recourse::narrow_span_recourse!()
            ),
            Self::DegreeAboveCeiling => {
                "rational NURBS face of degree above 18 in a direction, past the degree its \
                 second-derivative bound reaches (three times the degree, at most 56). \
                 Recourse: describe the face at degree 18 or below in each direction"
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
    /// below), on the rational arm `M̃/W³` on the piece and on the
    /// integral arm `A_u × A_v` over one, one piece per span cell
    /// ([`polynomial_area_pieces`]). `0.0` — the floor's conservative
    /// answer — when the reading does not certify, separates nothing
    /// from zero, or is past the integral arm's degree 28 (module docs,
    /// "Refusal").
    pub m_inf: f64,
}

/// `n` with its weights at unit scale
/// ([`geom_core::spline::weight_scale::unit_scale_exact`]), or `n`
/// itself where that rescale is a no-op or would drop a bit. The
/// rescaled net describes the same surface; what it buys is range for
/// the weight powers the enclosures form (`w²` here, up to `w⁴` in
/// [`crate::offset_fit`]'s composite). Which arm reads a net is decided
/// on the DESCRIBED weights ([`NurbsSurface::constant_weight`]), before
/// this runs.
pub(crate) fn unit_scaled_weights(n: &NurbsSurface<f64>) -> Cow<'_, NurbsSurface<f64>> {
    let Some((_, scaled)) = unit_scale_exact(n.weights()) else {
        return Cow::Borrowed(n);
    };
    match NurbsSurface::new(
        n.knots_u_f64_interim().clone(),
        n.knots_v_f64_interim().clone(),
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
/// raw knot-span cells, the rational arm each span cell's exact dyadic
/// pieces, [`RATIONAL_CERT_SPLITS`] per direction at most (module docs,
/// "The rational arm").
///
/// # Errors
///
/// [`PatchBoundError`] — a C⁰-creased or degree-0 direction, an
/// illegal rational description, a rational direction of degree above
/// 18 ([`PatchBoundError::DegreeAboveCeiling`], module docs "Refusal").
pub fn patch_cells(n: &NurbsSurface<f64>) -> Result<Vec<PatchCell>, PatchBoundError> {
    gate(n, false)?;
    if n.constant_weight().is_none() {
        rational_cells(n, RATIONAL_CERT_SPLITS)
    } else {
        integral_cells(n)
    }
}

/// [`patch_cells`] for a consumer that keeps its own reads off a C⁰
/// crease: the rational arm admits an interior knot of multiplicity
/// equal to the degree (degree ≥ 2), where [`check_direction`]
/// refuses one.
///
/// **Why the rational arm can, and the integral arm cannot.** The
/// rational arm reads each span cell's own control window
/// ([`rational_cells`]): a span's Bernstein block, its weights and its
/// derivatives all live inside the span, so a crease at the span's end
/// is the end of one block and the start of the next, and each cell
/// bounds the one-sided partials of its own side exactly. The integral
/// arm differences the whole net first, and a derivative across a knot
/// of multiplicity `p` is discontinuous there, which no clamped vector
/// describes; it keeps [`check_direction`]'s refusal. Which arm reads
/// a net is [`NurbsSurface::constant_weight`]'s, so a creased net of one
/// constant weight `c`, `1` or any other, is the integral arm's and
/// refuses: the polynomial patch it is has the same crease the unit net
/// does.
///
/// **What the caller owns.** Every cell bounds its own span; no bound
/// holds for a region that crosses a crease, so the max over cells on
/// both sides does not certify a box straddling one. The mesher's
/// `NurbsCellGrid::cert` refuses such a box, and its trimmed lane cuts
/// every triangle at the crease.
///
/// # Errors
///
/// As [`patch_cells`], with [`PatchBoundError::Crease`] left only to
/// the integral arm.
pub fn patch_cells_across_creases(
    n: &NurbsSurface<f64>,
) -> Result<Vec<PatchCell>, PatchBoundError> {
    if n.constant_weight().is_some() {
        return patch_cells(n);
    }
    gate(n, true)?;
    rational_cells(n, RATIONAL_CERT_SPLITS)
}

/// [`check_direction`] on both directions, with a degree-≥ 2 crease
/// let through when `admit_crease`.
fn gate(n: &NurbsSurface<f64>, admit_crease: bool) -> Result<(), PatchBoundError> {
    for kv in [n.knots_u_f64_interim(), n.knots_v_f64_interim()] {
        match check_direction(kv) {
            Err(PatchBoundError::Crease) if admit_crease => {}
            other => other?,
        }
    }
    Ok(())
}

/// [`patch_cells`] on an EXPLICIT schedule, in **both** arms: the
/// integral arm refines every nonempty span of every direction into
/// `splits` equal pieces first, and the rational arm reads the largest
/// power of two at most `splits` equal pieces per span direction
/// (module docs).
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
/// Refinement and cutting are exact in ℝ, so this buys tightness and
/// nothing else.
///
/// # Errors
///
/// As [`patch_cells`], plus the narrow-span refusals on the integral
/// arm's refinement: [`PatchBoundError::RefinementFailed`] (its
/// insertion chain) and [`PatchBoundError::DerivedKnots`] (its refined
/// vector).
pub fn patch_cells_refined(
    n: &NurbsSurface<f64>,
    splits: usize,
) -> Result<Vec<PatchCell>, PatchBoundError> {
    check_direction(n.knots_u_f64_interim())?;
    check_direction(n.knots_v_f64_interim())?;
    if n.constant_weight().is_none() {
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
/// [`PatchBoundError::RefinementFailed`] — a refused insertion chain,
/// which only [`NARROW_SPAN`](crate::recourse::NARROW_SPAN) reaches: the points lie strictly inside
/// their spans and the weights are unit.
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
    let nv = n.knots_v_f64_interim().control_count();
    TensorChannels::from_fn(n.knots_u_f64_interim(), n.knots_v_f64_interim(), |i, j| {
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
    /// ([`NARROW_SPAN`](crate::recourse::NARROW_SPAN)).
    fn of(
        derived: Derived<TensorCoeffs<'_>>,
        diff: impl FnOnce(&TensorCoeffs<'_>) -> Net,
    ) -> Result<Self, PatchBoundError> {
        match derived {
            Derived::Vector(t) => Ok(Self::Net(diff(&t))),
            Derived::SpanConstant(_) => Ok(Self::Zero),
            Derived::Discontinuous(_) => Err(PatchBoundError::DerivedKnots),
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

/// The INTEGRAL arm (one constant weight,
/// [`NurbsSurface::constant_weight`]): the plain hull assembly on the
/// spatial nets — no quotient rule intervenes, so the enclosure IS the
/// coefficient hull.
fn integral_cells(n: &NurbsSurface<f64>) -> Result<Vec<PatchCell>, PatchBoundError> {
    integral_cells_on(&comp_nets(n))
}

/// [`integral_cells`] after refining every nonempty span into `splits`
/// equal pieces, IN INTERVAL ARITHMETIC: an integral net's spatial net
/// is the polynomial patch itself ([`NurbsSurface::constant_weight`]),
/// so [`refine_split`]'s schedule applies to it directly. The cells
/// therefore enclose the described patch, where an `f64` refinement
/// would have them enclose the refined-`f64` one.
///
/// # Errors
///
/// As [`integral_cells`], plus [`PatchBoundError::RefinementFailed`];
/// [`PatchBoundError::DerivedKnots`] is reachable only here.
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
    let (held_u, held_v) = (held(kv_u), held(kv_v));
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
            let m_inf = integral_area_floor(base_nets, (&held_u, &held_v), span_u, span_v);
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
    // The weights at unit scale: the same surface, with range for the
    // weight powers (`W³` here) at either end of the `f64` range.
    let scaled = unit_scaled_weights(n);
    let n = &*scaled;
    let (kv_u, kv_v) = (n.knots_u_f64_interim(), n.knots_v_f64_interim());
    // The second order is read at three times each degree.
    if 3 * kv_u.degree().max(kv_v.degree()) > QUOTIENT_MAX_DEGREE {
        return Err(PatchBoundError::DegreeAboveCeiling);
    }
    let (held_u, held_v) = (held(kv_u), held(kv_v));
    let nv = kv_v.control_count();
    let mut cells: Vec<PatchCell> = Vec::new();
    for su in (kv_u.first_span()..=kv_u.last_span()).filter_map(|i| kv_u.span(i)) {
        for sv in (kv_v.first_span()..=kv_v.last_span()).filter_map(|i| kv_v.span(i)) {
            let points = on_window(su, sv, |iu, iv| {
                let pt = n.control()[iu * nv + iv];
                [pt.x, pt.y, pt.z].map(Interval::point)
            });
            let weights = on_window(su, sv, |iu, iv| Interval::point(n.weights()[iu * nv + iv]));
            let (counts, pieces) = cell_second_quotients(
                (CellAxis::Span(su, &held_u), CellAxis::Span(sv, &held_v)),
                CellBlock::Points {
                    origin: points[0],
                    points: &points,
                    weights: &weights,
                },
                (splits, splits),
            )
            .unwrap_or_else(|| {
                unreachable!(
                    "`cell_second_quotients` refused a block it always forms: its refusals \
                     (geom-core `cell_pieces`, `cell_whole`, `product`) are structural, and \
                     this block is a span window of a patch of degrees ({}, {}) \
                     whose `3·max(p, q)` passed the {QUOTIENT_MAX_DEGREE} ceiling above",
                    kv_u.degree(),
                    kv_v.degree()
                )
            });
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
pub(crate) fn certified_sup(q: &QuotientPiece<Interval, 3>) -> f64 {
    let s = q.sup_norm();
    if s.is_certified() { s.hi() } else { f64::NAN }
}

/// The quotient's norm inf over its piece, read from below; `0.0` when
/// it does not certify or does not separate the piece from zero.
pub(crate) fn certified_inf(q: &QuotientPiece<Interval, 3>) -> f64 {
    let s = q.inf_norm();
    if s.is_certified() {
        s.lo().max(0.0)
    } else {
        0.0
    }
}

/// The integral arm's [`PatchCell::m_inf`] on the span cell
/// `span_u × span_v`: the area's least ratio along each piece's own
/// direction, `A_u × A_v` over one on a single piece
/// ([`polynomial_area_pieces`] at splits `(1, 1)`). Past the statement's
/// ceiling (`2p − 1` above [`QUOTIENT_MAX_DEGREE`], a degree above 28)
/// the floor is the conservative `0.0`: the cell's sup-side readings do
/// not need it, so the arm still certifies, and an inf-side consumer
/// reads a floor that separates nothing.
fn integral_area_floor(
    nets: &TensorChannels<'_, 3>,
    (held_u, held_v): (&[Interval], &[Interval]),
    span_u: Span<'_>,
    span_v: Span<'_>,
) -> f64 {
    let over = |sp: &Span<'_>| 2 * sp.degree() - 1 > QUOTIENT_MAX_DEGREE;
    if over(&span_u) || over(&span_v) {
        // The conservative floor: `A_u × A_v` is past the statement's
        // degree ceiling, so no floor can be formed, and `0.0` separates
        // nothing from zero. The sups need no quotient, so the cell still
        // certifies (`the_integral_floor_is_zero_past_degree_28`).
        return 0.0;
    }
    let channels = nets.channels();
    let points = on_window(span_u, span_v, |iu, iv| {
        core::array::from_fn(|c| channels[c].net().get(iu, iv))
    });
    let pieces = polynomial_area_pieces(
        (
            CellAxis::Span(span_u, held_u),
            CellAxis::Span(span_v, held_v),
        ),
        &points,
        (1, 1),
    )
    .unwrap_or_else(|| {
        unreachable!(
            "`polynomial_area_pieces` refused a block it always forms: its refusals \
             (geom-core `polynomial_area_pieces`, `product`) are structural, and this block \
             is a span window of a C¹-gated patch of degrees ({}, {}) whose `2p − 1` passed \
             the {QUOTIENT_MAX_DEGREE} ceiling above",
            span_u.degree(),
            span_v.degree()
        )
    });
    pieces
        .iter()
        .map(certified_inf)
        .fold(f64::INFINITY, f64::min)
}

/// A knot vector's knots held as certification points.
fn held(kv: &KnotVector) -> Vec<Interval> {
    kv.knots().iter().map(|k| Interval::point(*k)).collect()
}

/// `at(i, j)` over the control window of the span cell `su × sv`,
/// row-major in `u` then `v`: the block a cell's quotients read.
fn on_window<X>(su: Span<'_>, sv: Span<'_>, at: impl Fn(usize, usize) -> X) -> Vec<X> {
    su.window()
        .flat_map(|iu| sv.window().map(move |iv| (iu, iv)))
        .map(|(iu, iv)| at(iu, iv))
        .collect()
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
pub(crate) mod tests {
    use super::*;
    use geom_core::spline::weight_scale::pow2_scaled;

    /// **The rational arm reads a crease as the end of one span and the
    /// start of the next.** A half cylinder, two rational quarter arcs
    /// in `u` joined at a double knot (a C⁰ crease at degree 2), has
    /// exactly the cells of its two halves built as separate patches:
    /// each cell reads its own span's window and nothing across the
    /// crease. [`patch_cells`] refuses the whole (`Crease`), and
    /// [`patch_cells_across_creases`] still refuses an integral one,
    /// whose arm differences the whole net.
    #[test]
    fn a_rational_crease_reads_as_two_patches() {
        let w = core::f64::consts::FRAC_1_SQRT_2;
        let arc = [(1.0, 0.0), (1.0, 1.0), (0.0, 1.0), (-1.0, 1.0), (-1.0, 0.0)];
        let patch = |ku: Vec<f64>, rows: &[(f64, f64)], ws: &[f64]| {
            let ku = KnotVector::clamped(ku, 2).expect("kv");
            let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).expect("kv");
            let mut control = Vec::new();
            let mut weights = Vec::new();
            for (&(x, y), &wt) in rows.iter().zip(ws) {
                for z in [0.0, 1.0] {
                    control.push(geom_core::Point3::new(x, y, z));
                    weights.push(wt);
                }
            }
            NurbsSurface::new(ku, kv, control, weights).expect("ok")
        };
        let whole = patch(
            vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0],
            &arc,
            &[1.0, w, 1.0, w, 1.0],
        );
        let left = patch(
            vec![0.0, 0.0, 0.0, 0.5, 0.5, 0.5],
            &arc[..3],
            &[1.0, w, 1.0],
        );
        let right = patch(
            vec![0.5, 0.5, 0.5, 1.0, 1.0, 1.0],
            &arc[2..],
            &[1.0, w, 1.0],
        );
        assert!(matches!(patch_cells(&whole), Err(PatchBoundError::Crease)));
        let read = |cells: Vec<PatchCell>| -> Vec<String> {
            cells.iter().map(|c| format!("{c:?}")).collect()
        };
        let mut halves = read(patch_cells(&left).expect("left"));
        halves.extend(read(patch_cells(&right).expect("right")));
        assert_eq!(
            read(patch_cells_across_creases(&whole).expect("whole")),
            halves
        );
        // A constant weight is the integral arm's, `1` or not.
        for c in [1.0, 2.0] {
            let integral = patch(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], &arc, &[c; 5]);
            assert!(
                matches!(
                    patch_cells_across_creases(&integral),
                    Err(PatchBoundError::Crease)
                ),
                "weights {c}"
            );
        }
    }

    /// **The degree ceilings are the statement's, and each arm meets its
    /// own.** An integral bidegree `(19, 2)` Bézier certifies: its area
    /// floor reads `A_u × A_v` at degree `(37, 3)`, under the `56`
    /// ceiling, and its sups need no quotient at all. The same net with
    /// one weight off `1` refuses typed, `DegreeAboveCeiling`, because the
    /// rational arm reads its second partials at `3 · 19 = 57`; the
    /// refusal is a description the caller changes, not a kernel defect.
    #[test]
    fn each_arm_meets_its_own_degree_ceiling() {
        let ku = KnotVector::clamped([vec![0.0; 20], vec![1.0; 20]].concat(), 19).expect("kv");
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).expect("kv");
        let mut control = Vec::new();
        for i in 0..20u32 {
            for j in 0..3u32 {
                let (u, v) = (f64::from(i) / 19.0, f64::from(j) / 2.0);
                control.push(geom_core::Point3::new(u, v, 0.2 * u * v + 0.1 * u * u));
            }
        }
        let integral =
            NurbsSurface::new(ku.clone(), kv.clone(), control.clone(), vec![1.0; 60]).expect("ok");
        let cells = patch_cells(&integral).expect("an integral (19, 2) patch certifies");
        assert!(
            cells.iter().all(|c| c.m_inf > 0.0 && c.m_inf <= c.m_sup),
            "the integral floor read {:?}",
            cells.iter().map(|c| c.m_inf).collect::<Vec<_>>()
        );
        let mut weights = vec![1.0; 60];
        weights[7] = 1.5;
        let rational = NurbsSurface::new(ku, kv, control, weights).expect("ok");
        let refusal = patch_cells(&rational).map(|c| c.len());
        assert_eq!(refusal, Err(PatchBoundError::DegreeAboveCeiling));
        let msg = PatchBoundError::DegreeAboveCeiling.to_string();
        assert!(
            msg.contains("18")
                && msg.contains("56")
                && !msg.ends_with(geom_core::KERNEL_DEFECT_ENDING),
            "{msg}"
        );
    }

    /// **Past degree 28 the integral floor is the conservative zero.**
    /// The integral area `A_u × A_v` sits at degree `2p − 1`, so a
    /// degree-28 direction reads it at `55` and floors it, and a
    /// degree-29 one would read `57`, past [`QUOTIENT_MAX_DEGREE`]. There
    /// the arm still certifies (its sups need no quotient) and every
    /// cell's `m_inf` is exactly `0.0`: a floor that separates nothing,
    /// so an inf-side consumer refuses rather than reading a number the
    /// statement cannot form.
    #[test]
    fn the_integral_floor_is_zero_past_degree_28() {
        let floors = |p: usize| -> Vec<f64> {
            let ku =
                KnotVector::clamped([vec![0.0; p + 1], vec![1.0; p + 1]].concat(), p).expect("kv");
            let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).expect("kv");
            let mut control = Vec::new();
            for i in 0..=p {
                for j in 0..2u32 {
                    #[allow(clippy::cast_precision_loss)]
                    let u = i as f64 / p as f64;
                    let v = f64::from(j);
                    control.push(geom_core::Point3::new(u, v, 0.1 * u * u + 0.2 * u * v));
                }
            }
            let n = NurbsSurface::new(ku, kv, control, vec![1.0; 2 * (p + 1)]).expect("ok");
            let cells = patch_cells(&n).expect("an integral patch certifies at any degree");
            assert!(cells.iter().all(|c| c.m_sup.is_finite()), "p = {p}");
            cells.iter().map(|c| c.m_inf).collect()
        };
        assert!(
            floors(28).iter().all(|m| *m > 0.0),
            "p = 28 floors: {:?}",
            floors(28)
        );
        assert!(
            floors(29).iter().all(|m| *m == 0.0),
            "p = 29 floors: {:?}",
            floors(29)
        );
    }

    /// `n` with every weight multiplied by `2^k`.
    pub(crate) fn weights_times_pow2(n: &NurbsSurface<f64>, k: i32) -> NurbsSurface<f64> {
        NurbsSurface::new(
            n.knots_u_f64_interim().clone(),
            n.knots_v_f64_interim().clone(),
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
            cylinder.knots_u_f64_interim().clone(),
            cylinder.knots_v_f64_interim().clone(),
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

    /// **A constant weight is the integral arm's.** The quarter
    /// cylinder's net with every weight one constant (`2`, `3`, `0.1`) is
    /// the polynomial patch on that net (`constant_weight`). Its cells,
    /// plain and refined, are the unit-weight net's to the bit, since
    /// that arm reads the control net alone; the rational arm would cut
    /// each span into its own pieces.
    #[test]
    fn a_constant_weight_takes_the_integral_arm() {
        let cylinder = quarter_cylinder();
        let constant = |w: f64| {
            NurbsSurface::new(
                cylinder.knots_u_f64_interim().clone(),
                cylinder.knots_v_f64_interim().clone(),
                cylinder.control().to_vec(),
                vec![w; 6],
            )
            .expect("door")
        };
        let unit = constant(1.0);
        let want = format!("{:?}", patch_cells(&unit).expect("unit"));
        let want_refined = format!("{:?}", patch_cells_refined(&unit, 4).expect("unit"));
        for w in [2.0, 3.0, 0.1] {
            let net = constant(w);
            assert_eq!(
                format!("{:?}", patch_cells(&net).expect("constant")),
                want,
                "weight {w}: the cells are not the unit net's"
            );
            assert_eq!(
                format!("{:?}", patch_cells_refined(&net, 4).expect("constant")),
                want_refined,
                "weight {w}: the refined cells are not the unit net's"
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
        const RECOURSE_WORDS: &[&str] = &["describe", "supply", "widen", "split"];
        use strum::IntoEnumIterator;
        for arm in PatchBoundError::iter() {
            let msg = arm.to_string();
            assert_eq!(msg, arm.note(), "Display is the shared note");
            // One ending, a labelled repair: a valid face reaches every
            // arm, so none ends on the kernel-defect dead end.
            assert_eq!(
                test_utils::refusal::recourse_markers(&msg),
                1,
                "not exactly one ending: {msg}"
            );
            assert!(
                !msg.contains(geom_core::KERNEL_DEFECT_ENDING),
                "a refusal input reaches ends on the dead end: {msg}"
            );
            // The narrow-span decision tells one story on both arms.
            let narrow = matches!(
                arm,
                PatchBoundError::RefinementFailed | PatchBoundError::DerivedKnots
            );
            assert_eq!(
                msg.contains(NARROW_SPAN) && msg.ends_with(NARROW_SPAN_RECOURSE),
                narrow,
                "the narrow span's story on the wrong arms: {msg}"
            );
            let lower = msg.to_lowercase();
            assert!(
                RECOURSE_WORDS.iter().any(|w| lower.contains(w)),
                "no recourse in: {msg}"
            );
        }
    }

    /// **Colliding split points do not zero a second partial.** On a
    /// [`NARROW_SPAN`] refinement can raise a multiplicity to `p` after
    /// the C¹ gate passed, and the derived vector stops being clamped.
    /// That must refuse typed or bound the partial — never read it as
    /// zero.
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
                    // refuses, `RefinementFailed`. Either names the span
                    // as the lever.
                    Err(e) => {
                        assert_eq!(
                            e,
                            if splits == 8 {
                                PatchBoundError::DerivedKnots
                            } else {
                                PatchBoundError::RefinementFailed
                            },
                            "{arm} @ {splits}"
                        );
                        assert!(
                            e.to_string().ends_with(NARROW_SPAN_RECOURSE),
                            "{arm} @ {splits}: {e}"
                        );
                    }
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

    /// An integral biquadratic with interior knots, or its rational twin
    /// with weights varying along both directions.
    fn biquadratic(rational: bool) -> NurbsSurface<f64> {
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
    }

    /// A rational biquadratic whose weights vary strongly along `v` (the
    /// middle `v` column carries the large weights), found by a seeded
    /// search for a patch on which the `w_v` term of the retired `m_sup`
    /// recipe kept the bound above the sample.
    fn strong_v() -> NurbsSurface<f64> {
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
    }

    /// A rational patch of bidegree `(2, 1)` whose weights vary along the
    /// degree-1 direction.
    fn ruled() -> NurbsSurface<f64> {
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
    }

    /// **The area floor reads each cell's own normal.** Each cell's
    /// [`PatchCell::m_inf`] is checked against `‖S_u × S_v‖` on a 5×5
    /// grid over the cell: never above the least certified upper end
    /// of the samples (soundness: the true inf is at most any sampled
    /// value), and at least a measured share of the least sampled
    /// value (the floor follows the normal as it turns). The quarter
    /// cylinder's normal turns through a right angle, so a floor read
    /// along one fixed direction collapses to zero on the cells where
    /// the normal is across it; the other fixtures are the meter row's.
    #[test]
    fn the_area_floor_reads_each_cells_own_normal() {
        // Measured worst share: `0.937` (the degree-1 fixture; `0.996`
        // the rational twin, `0.999` the integral arm and the cylinder),
        // with about a tenth to spare.
        const FLOOR_SHARE: f64 = 0.85;
        for (name, n, splits, share) in [
            ("integral", biquadratic(false), 4, true),
            ("rational", biquadratic(true), 4, true),
            ("rational degree-1 v", ruled(), 4, true),
            (
                "quarter cylinder",
                quarter_cylinder(),
                RATIONAL_CERT_SPLITS,
                true,
            ),
            // Soundness only: near the corner `(0.31, 0.13)` the area
            // falls to `0.036` of its scale and the normal turns inside a
            // piece, so the cell there reads the conservative zero.
            ("rational strong v", strong_v(), RATIONAL_CERT_SPLITS, false),
        ] {
            let lifted = n.map_scalar(<Interval as geom_core::Real>::from_f64);
            let cells = patch_cells_refined(&n, splits).expect("covered");
            let mut worst = f64::INFINITY;
            for c in &cells {
                let (mut least_hi, mut least) = (f64::INFINITY, f64::INFINITY);
                for a in 0..5u32 {
                    for b in 0..5u32 {
                        let u = c.u.0 + (c.u.1 - c.u.0) * f64::from(a) / 4.0;
                        let v = c.v.0 + (c.v.1 - c.v.0) * f64::from(b) / 4.0;
                        let jet = lifted.ders(
                            <Interval as geom_core::Real>::from_f64(u),
                            <Interval as geom_core::Real>::from_f64(v),
                        );
                        let m = geom_core::interval::norm_sq(&cross(
                            &jet.du.to_array(),
                            &jet.dv.to_array(),
                        ))
                        .sqrt();
                        least_hi = least_hi.min(m.hi());
                        let f = n.ders(u, v);
                        least = least.min(f.du.cross(f.dv).norm());
                    }
                }
                assert!(
                    c.m_inf <= least_hi,
                    "{name}: m_inf {:e} above a sampled ‖S_u × S_v‖ {least_hi:e} on {:?} × {:?}",
                    c.m_inf,
                    c.u,
                    c.v
                );
                worst = worst.min(c.m_inf / least);
            }
            assert!(
                !share || worst >= FLOOR_SHARE,
                "{name}: a cell's floor reads {worst} of its least sampled ‖S_u × S_v‖"
            );
        }
    }

    /// **A rational cell reads only its own points.** On a rational
    /// biquadratic with two nonempty `u` spans, the last `u` row of
    /// control points and weights is active on the second span alone:
    /// moving it a hundredfold leaves every first-span cell bit-identical
    /// (its extents, its five signed boxes and its four meters).
    #[test]
    fn a_rational_cell_reads_only_its_own_points() {
        let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 2.0, 2.0, 2.0], 2).expect("valid");
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).expect("valid");
        let (nu, nv) = (ku.control_count(), kv.control_count());
        let patch = |far: f64| {
            let mut control = Vec::new();
            let mut weights = Vec::new();
            for i in 0..nu {
                for j in 0..nv {
                    #[allow(clippy::cast_precision_loss)]
                    let (x, y) = (i as f64, j as f64);
                    let last = i + 1 == nu;
                    control.push(geom_core::Point3::new(
                        x * if last { far } else { 1.0 },
                        y,
                        0.3 * x * y,
                    ));
                    weights.push(if last { far } else { 1.0 + 0.2 * (x + y) });
                }
            }
            NurbsSurface::new(ku.clone(), kv.clone(), control, weights).expect("valid")
        };
        let first_span = |far: f64| -> Vec<Vec<u64>> {
            patch_cells_refined(&patch(far), 4)
                .expect("covered")
                .iter()
                .filter(|c| c.u.1 <= 1.0)
                .map(|c| {
                    let boxes = [c.s_u, c.s_v, c.s_uu, c.s_uv, c.s_vv];
                    [c.u.0, c.u.1, c.v.0, c.v.1]
                        .into_iter()
                        .chain(boxes.iter().flatten().flat_map(|i| [i.lo(), i.hi()]))
                        .chain([c.s_u_sup, c.s_v_sup, c.m_sup, c.m_inf])
                        .map(f64::to_bits)
                        .collect()
                })
                .collect()
        };
        let (near, far) = (first_span(1.0), first_span(100.0));
        assert_eq!(near.len(), 16, "the first span's cells at four splits");
        assert_eq!(near, far, "a first-span cell read the last u row");
    }

    /// **The cells are the second order's pieces, at the schedule's
    /// count at any degree.** The rational arm halves each cell's
    /// quotients, so a biquintic, whose second order sits at degree
    /// `15`, reads all `16` pieces per span direction at `16` splits.
    /// The row pins the count and that the cells tile the domain, each
    /// extent meeting the next end to end, so extents counted otherwise
    /// than the pieces leave a gap or an overlap.
    #[test]
    fn the_rational_cells_tile_each_span_at_the_second_orders_count() {
        let kv = KnotVector::clamped([vec![0.0; 6], vec![1.0; 6]].concat(), 5).expect("valid");
        let mut control = Vec::new();
        let mut weights = Vec::new();
        for i in 0..6u32 {
            for j in 0..6u32 {
                let (u, v) = (f64::from(i) * 0.2, f64::from(j) * 0.2);
                control.push(geom_core::Point3::new(u, v, 0.4 * u * v - 0.2 * u * u));
                weights.push(1.0 + 0.3 * f64::from((i + 2 * j) % 3));
            }
        }
        let n = NurbsSurface::new(kv.clone(), kv, control, weights).expect("valid");
        let cells = patch_cells_refined(&n, RATIONAL_CERT_SPLITS).expect("covered");
        assert_eq!(cells.len(), 16 * 16, "a biquintic's pieces at 16 splits");
        let ends = |axis: fn(&PatchCell) -> (f64, f64)| {
            let mut e: Vec<(f64, f64)> = cells.iter().map(axis).collect();
            e.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.total_cmp(&b.1)));
            e.dedup();
            e
        };
        for (what, e) in [("u", ends(|c| c.u)), ("v", ends(|c| c.v))] {
            assert_eq!(e.len(), 16, "{what}: {e:?}");
            assert_eq!(e.first().map(|x| x.0), Some(0.0), "{what}: {e:?}");
            assert_eq!(e.last().map(|x| x.1), Some(1.0), "{what}: {e:?}");
            assert!(
                e.windows(2).all(|w| w[0].1 == w[1].0),
                "{what}: the extents do not meet end to end: {e:?}"
            );
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
    /// speed loosened by `1e-9` of itself reads over its box ceiling
    /// (one far below the ceiling's `1e-12`, `1e-13` say, does not), and
    /// `m_sup` loosened by a fifth.
    /// The rational arm's terms are the quotient statement's, whose own
    /// rows drop them (`geom_core::spline::derivative`).
    ///
    /// The samples are `f64` evaluations, so the soundness comparison
    /// allows `1e-12` of the sampled value for their rounding, decades
    /// under what a dropped term costs.
    #[test]
    fn the_meter_sups_bound_every_sampled_magnitude() {
        use geom_core::interval::{dot, norm_sq};
        // The speeds are never above the box in ℝ, on either arm: each
        // coefficient vector (the integral arm's derived control vectors,
        // the rational arm's ratios `ã_k/b_k`) lies in the box, whose
        // norm sup is the box reading. So the speed ceiling is the claim
        // itself, `1` plus rounding: measured at most `1 + 4.5e-16`.
        // `m_sup` has no such claim on the integral arm — the cross of two
        // hulls is not the box of the cross — and measures `1.024`× there
        // (at most `1.0`× on the rational fixtures, `0.90`× strong-`v`,
        // where the box cross product loses the correlation the area
        // quotient keeps); its ceiling is that worst with about a tenth to
        // spare.
        const SPEED_CEILING: f64 = 1.0 + 1e-12;
        const M_CEILING: f64 = 1.13;
        let names = ["‖S_u‖", "‖S_v‖", "‖S_u × S_v‖"];
        for (name, n) in [
            ("integral", biquadratic(false)),
            ("rational", biquadratic(true)),
            ("rational strong v", strong_v()),
            ("rational degree-1 v", ruled()),
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
