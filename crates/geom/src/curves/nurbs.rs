//! NURBS curves — [`NurbsCurve2`] (the future pcurve substrate; not
//! wired into any enum) and [`NurbsCurve3`] (the [`crate::curves::Curve3::Nurbs`]
//! payload), M5 PR 3.
//!
//! # Data model (binding conventions)
//!
//! Control points, weights and knots are **held in the scalar** `T`
//! (C6): the knots as [`geom_core::spline::Knots`], a breakpoint per
//! run, and the weights as `Vec<T>`. The **structure** (degree, counts,
//! multiplicities, which span a parameter lies in, and every decision
//! the knot algebra takes) is decided in `f64` on the values' value
//! channel: the [`geom_core::spline::KnotVector`] the held knots carry,
//! and the weights' channel read off them at construction. Construction
//! is validated and typed-error: **clamped-v1** knot vectors (the
//! [`KnotVector`] carries the exact invariants), weights that are each
//! one point,
//! **strictly positive weights** (the convex-hull property every C9
//! hull bound stands on), and count coherence. Unlike the analytic
//! variants' public conventional fields, these invariants are
//! *load-bearing for indexing safety*, so fields are private and
//! construction goes through [`NurbsCurve3::new`] /
//! [`NurbsCurve2::new`].
//!
//! ## Control points are EUCLIDEAN, and what follows from it
//!
//! The net stores Euclidean control points with the weights in a
//! channel beside them — never the weighted (homogeneous) `wⱼPⱼ` form.
//! That is a storage decision, and it is the premise of the whole
//! affine-map story on this page and on the surface's:
//!
//! Evaluation normalizes, `sum(Nⱼ wⱼ Pⱼ) / sum(Nⱼ wⱼ)`, so the
//! coefficients sum to one and every evaluated point is an **affine
//! combination** of the control points. An affine map commutes with an
//! affine combination. So for affine `f`, mapping the net **pointwise
//! and leaving knots and weights alone** yields exactly the image of
//! this curve — `f(C(t))` at every `t` — and not an approximation of
//! it. `NurbsCurve3::map_points` and `NurbsSurface::map_points` are
//! that map; `topo::transform_rigid` is its consumer, a rigid map
//! being affine.
//!
//! It reaches the derivatives too, which is why nothing there needs a
//! separate argument: the rational correction is
//! `C′ = (A⁽¹⁾ − C·w⁽¹⁾)/w⁽⁰⁾` with `C` a position, so under
//! `P ↦ RP + t` the translation cancels and `C̃′ = R·C′` exactly.
//!
//! **Were the storage weighted instead, that all fails**: an affine
//! map's translation limb would have to be scaled by `wⱼ` per point,
//! and applying it unscaled bends the geometry rather than moving it.
//! Anyone changing the storage owes this section and both
//! `map_points` doors a revisit.
//!
//! # Evaluation contract
//!
//! - **Core, generic** (`*_in_span`): rational evaluation restricted to
//!   one span — ring ops + `from_f64` only, total for every scalar. For
//!   `t` outside the span's knot interval the result is the span's
//!   **polynomial extension** (documented garbage-out — detecting it
//!   would need the comparison [`Real`] deliberately lacks). These
//!   doors live on [`CurveWindow2`]/[`CurveWindow3`], which **borrow
//!   the curve**: the basis comes from the window's own span of the
//!   held knots ([`KnotSpan`]) and the control points and weights from
//!   the curve that span was drawn from, one borrow, so the window base is an addition that cannot
//!   underflow, the reads are in range by construction, and there is no
//!   pairing to check and no refusal to answer. The two mints are
//!   [`NurbsCurve3::span`] and [`NurbsCurve3::span_at`], both `&self`.
//! - **Full evaluators** (`eval`/`deriv`/`deriv2`, and the jets `ders1`
//!   and `ders`): span selection via the sealed [`SpanLocate`] seam
//!   (per-instantiation semantics documented in
//!   `geom_core::spline::locate`), then the core per overlapped span,
//!   hulled channel-independently for interval-natured scalars.
//!
//! # What does not typecheck
//!
//! Library doctests — they run under `cargo test -p geom --doc`, so
//! these claims redden if the borrow is undone.
//!
//! **A curve's span doors take no curve.** A span located in one
//! curve's knot vector cannot be evaluated against a different curve,
//! because no door takes both:
//!
//! ```compile_fail,E0599
//! use geom::NurbsCurve3;
//! use geom_core::spline::KnotVector;
//! use geom_core::Point3;
//! let long = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 4.0], 2).unwrap();
//! let short = NurbsCurve3::<f64>::new(
//!     KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap(),
//!     vec![Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 1.0, 0.0), Point3::new(2.0, 0.0, 0.0)],
//!     vec![1.0, 1.0, 1.0],
//! ).unwrap();
//! let foreign = long.span(5).unwrap();
//! let _ = short.eval_in_span(foreign, 3.5f64);
//! ```
//!
//! The twin differs in one identifier — the window is minted by the
//! curve it evaluates, so the door exists and the call resolves:
//!
//! ```
//! use geom::NurbsCurve3;
//! use geom_core::spline::KnotVector;
//! use geom_core::Point3;
//! let short = NurbsCurve3::<f64>::new(
//!     KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap(),
//!     vec![Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 1.0, 0.0), Point3::new(2.0, 0.0, 0.0)],
//!     vec![1.0, 1.0, 1.0],
//! ).unwrap();
//! let own = short.span_at(0.5).unwrap();
//! let p = own.eval_in_span(0.5f64);
//! assert!(p.x.is_finite());
//! ```
//!
//! **Nor can a window outlive its curve**, so it cannot be carried to
//! where another curve is in scope:
//!
//! ```compile_fail,E0597
//! use geom::NurbsCurve3;
//! use geom_core::spline::KnotVector;
//! use geom_core::Point3;
//! let win = {
//!     let c = NurbsCurve3::<f64>::new(
//!         KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap(),
//!         vec![Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 1.0, 0.0), Point3::new(2.0, 0.0, 0.0)],
//!         vec![1.0, 1.0, 1.0],
//!     ).unwrap();
//!     c.span_at(0.5).unwrap()
//! };
//! let _ = win.eval_in_span(0.5f64);
//! ```
//!
//! The twin differs in one identifier — the curve is bound outside the
//! block, so the borrow lives as long as the window:
//!
//! ```
//! use geom::NurbsCurve3;
//! use geom_core::spline::KnotVector;
//! use geom_core::Point3;
//! let c = NurbsCurve3::<f64>::new(
//!     KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap(),
//!     vec![Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 1.0, 0.0), Point3::new(2.0, 0.0, 0.0)],
//!     vec![1.0, 1.0, 1.0],
//! ).unwrap();
//! let win = { c.span_at(0.5).unwrap() };
//! let _ = win.eval_in_span(0.5f64);
//! ```
//!
//! **What these rows do and do not check.** Stable rustdoc checks only
//! that a `compile_fail` block fails to build; the `,E0599` / `,E0597`
//! annotation is **not** verified there (a nightly rustdoc feature), so
//! a row could be red for a typo. Each twin is what rules that out: it
//! differs in exactly one respect and it compiles. The codes were read
//! off `rustc` directly on each snippet at the pinned toolchain.
//!
//! # Fixed association (D9)
//!
//! Homogeneous combination is a **single ascending-index pass**: for
//! each basis index `j` (ascending), `cw = N_j · w_j`, then
//! each homogeneous accumulator adds its term in the written order
//! (`acc = acc + cw · coord`), followed by **one division per
//! coordinate** by the accumulated weight. Rational derivative
//! corrections are evaluated exactly as parenthesized at each method.
//! Knot-algebra point combinations are `lerp(x, y, λ) = x + (y − x)·λ`
//! with `λ` and the new weight formed at `T` from the held knots and
//! weights, once per combination
//! ([`geom_core::spline::CurvePlan::apply_rational`]).

use core::num::NonZeroUsize;
use geom_core::spline::ProjectiveScalar;
use geom_core::spline::{
    self, KnotAlgebraError, KnotSpan, KnotVector, Knots, Span, SpanLocate, SplineError, Weights,
    derivative_net,
};
use geom_core::{Interval, Point2, Point3, Real, Vec2, Vec3};
use std::sync::OnceLock;

use crate::net;

/// The speed meter's piece schedule (D9: structure, never a decision):
/// both of `speed_lower_bound`'s piece readings and `speed_upper_bound`
/// read each nonempty span as this many equal pieces, at every degree,
/// halved from the quotient formed once over the span
/// ([`geom_core::spline::derivative::speed_pieces`]).
/// `tests/curves/swaying_corner_meter.rs` pins what it buys on a
/// swaying corner, where each doubling of the count roughly doubles
/// the reach (on other nets a finer schedule can read lower: each
/// piece takes its own direction, so refinement is not monotone), and
/// `tests/curves/piece_coverage_meter.rs` that every piece is read.
const SPEED_PIECE_SPLITS: usize = 32;

/// A spline's memoised meter: `compute` afresh inside a symbolic
/// session, `cell`'s value outside one (filled on first read). A
/// session's table holds the nodes it saw built, and a node minted
/// anywhere else freezes in it to an unknown of its own
/// (`geom_core::sym`, "No session, no tier"), so a memo served across
/// that boundary would change what the session decides; inside one
/// nothing the meter mints is kept.
pub(crate) fn session_memo<V: Copy>(cell: &OnceLock<V>, compute: impl FnOnce() -> V) -> V {
    if geom_core::sym::session_installed() {
        return compute();
    }
    *cell.get_or_init(compute)
}

/// A curve and a plan chain to apply to its net
/// (`apply_plans`, at either half of [`spline::ProjectiveSplit`]).
struct PlanChain<'a, C, T> {
    curve: &'a C,
    plans: &'a [spline::CurvePlan],
    fresh: &'a dyn Fn(f64) -> T,
}

macro_rules! nurbs_curve {
    ($Curve:ident, $Window:ident, $Point:ident, $Vector:ident, $dim:literal, $($c:ident),+) => {
        #[doc = concat!("The control window one knot span selects on **one** [`", stringify!($Curve), "`]")]
        /// — a borrow of the curve, exactly as `geom_core`'s [`Span`] is
        /// a borrow of the knot vector it indexes, and the same shape
        /// [`crate::SurfaceWindow`] has one dimension up.
        ///
        /// It carries the curve and the [`Span`], whose `first_control`
        /// (`index − degree`) was subtracted once at construction — no
        /// use site can underflow it. Evaluation then reads
        /// `first_control + j` for `j ∈ [0, degree]`, which is inside
        /// this curve's control array because the span's own vector IS
        /// this curve's knot vector and `new` pins
        /// `control.len() == knots.control_count()`.
        ///
        /// **Branded to its curve by the borrow.** The only mints are
        #[doc = concat!("[`", stringify!($Curve), "::span`] and [`", stringify!($Curve), "::span_at`],")]
        /// both taking `&self`, and evaluation lives here rather than on
        /// the curve: a door taking `(&curve, span)` has a second
        /// structure for the span to disagree with, and a borrow cannot
        /// make two live references a type error. With no such parameter
        /// there is nothing to disagree — a window evaluates the curve
        /// it names, and the window a curve mints names that curve.
        ///
        /// `Copy`, one reference and one `Span` wide, allocation-free.
        #[derive(Clone, Copy)]
        pub struct $Window<'a, T: Real> {
            curve: &'a $Curve<T>,
            span: KnotSpan<'a, T>,
        }

        /// Equality is address equality on the curve, plus the span
        /// (itself address-equal on its vector): a window is a proof
        /// about *that* control net, and a curve is not [`Eq`] — its
        /// data is held in a scalar that has no equality.
        ///
        /// **This walk and the `Debug` beside it destructure `Self`
        /// exhaustively**, so a field added to the declaration is an
        /// E0027 unbound-pattern error rather than a value silently
        /// outside equality and outside the dump.
        impl<T: Real> PartialEq for $Window<'_, T> {
            fn eq(&self, other: &Self) -> bool {
                let Self { curve, span } = self;
                let Self { curve: other_curve, span: other_span } = other;
                core::ptr::eq(*curve, *other_curve) && span.span() == other_span.span()
            }
        }

        impl<T: Real> Eq for $Window<'_, T> {}

        /// The borrow is printed as an ADDRESS, never followed. A
        /// derived `Debug` would dump the whole control net and knot
        /// vector through the reference at every `{:?}`, which is the
        /// one cost a borrow-carrying token can impose by accident.
        impl<T: Real> core::fmt::Debug for $Window<'_, T> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let Self { curve, span } = self;
                f.debug_struct(stringify!($Window))
                    .field("curve", &core::ptr::from_ref(*curve))
                    .field("span", &span.span())
                    .finish()
            }
        }

        impl<'a, T: Real> $Window<'a, T> {
            /// The curve this window names — what every door here reads
            /// its knots, control points and weights from.
            pub fn curve(self) -> &'a $Curve<T> {
                self.curve
            }

            /// The knot span this window selects.
            pub fn span(self) -> Span<'a> {
                self.span.span()
            }

            /// The span paired with the curve's held knot values: what
            /// the basis reads.
            pub fn knot_span(self) -> KnotSpan<'a, T> {
                self.span
            }

            /// The span index (`Span::index`).
            pub fn index(self) -> usize {
                self.span().index()
            }

            /// The degree (`Span::degree`).
            pub fn degree(self) -> usize {
                self.span().degree()
            }

            /// The first control point of the window (`Span::first_control`).
            pub fn first_control(self) -> usize {
                self.span().first_control()
            }

            /// The inclusive control-point window, `Span::window`.
            pub fn window(self) -> core::ops::RangeInclusive<usize> {
                self.span().window()
            }
        }

        /// A validated NURBS curve (module docs: data model, evaluation
        /// contract, fixed association orders). Immutable after
        /// construction; every knot-algebra operation returns a new
        /// curve (D9-clean value semantics).
        #[derive(Clone)]
        pub struct $Curve<T: Real> {
            knots: Knots<T>,
            control: Vec<$Point<T>>,
            /// The weights, held in `T` with their value channel
            /// ([`Weights`]): a built carrier's are points
            /// (`net::weight_channel`), so the channel is always there.
            weights: Weights<T>,
            /// [`Self::speed_lower_bound`], filled on first read outside
            /// a symbolic session. It is a function of the four fields
            /// above, which nothing mutates after construction, so it
            /// cannot go stale; every door that changes them builds a
            /// new curve with an empty memo, and a clone carries the
            /// memo of the same value.
            speed: OnceLock<geom_core::InfSpeed<T>>,
            /// [`Self::speed_upper_bound`], filled on first read, on the
            /// same terms as `speed`.
            speed_sup: OnceLock<geom_core::SupSpeed<T>>,
        }

        /// The curve's value, without the speed memo: whether it has
        /// been read yet is not part of what the curve is.
        impl<T: Real> core::fmt::Debug for $Curve<T> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let Self { knots, control, weights, speed: _, speed_sup: _ } = self;
                f.debug_struct(stringify!($Curve))
                    .field("knots", knots)
                    .field("control", control)
                    .field("weights", weights)
                    .finish()
            }
        }

        impl<T: Real> $Window<'_, T> {
            /// The point at `t`, evaluated **in the given span** — the
            /// generic core (module docs: the span contract; the fixed
            /// single-ascending-pass association).
            ///
            /// **Total, with no refusal.** The basis comes from this
            /// window's own span and the control points and weights
            /// from the curve that span was drawn from, one borrow, so
            /// `first_control + j` is inside the array by construction.
            ///
            /// `t` outside the span's interval still yields the span's
            /// polynomial extension (documented garbage-out).
            pub fn eval_in_span(self, t: T) -> $Point<T> {
                let basis = spline::basis::basis_funs_in(self.span, t);
                // The window's base is subtracted once inside `Span`,
                // so what remains here is an addition. Indexing (not
                // `zip`) deliberately: `basis` is `degree + 1` long and
                // the window is `degree + 1` wide — one `degree`, the
                // span's — and if that ever ceased to hold this must
                // PANIC rather than silently drop control points.
                let base = self.first_control();
                $(let mut $c = T::zero();)+
                let mut w_acc = T::zero();
                for (j, nj) in basis.iter().enumerate() {
                    let i = base + j;
                    let cw = *nj * self.curve.weights.values()[i];
                    let pt = self.curve.control[i];
                    $($c = $c + cw * pt.$c;)+
                    w_acc = w_acc + cw;
                }
                $Point::new($($c / w_acc),+)
            }

            /// The homogeneous accumulators through order `N − 1` —
            /// per coordinate channel `A⁽ᵏ⁾ = Σⱼ N⁽ᵏ⁾ⱼ·wⱼ·xⱼ` and the
            /// weight channel `w⁽ᵏ⁾ = Σⱼ N⁽ᵏ⁾ⱼ·wⱼ`, `k < N` — from
            /// one basis pass of order `N − 1`, read from this window's
            /// own span and curve. Total: there is no pairing to refuse.
            ///
            /// The one homogeneous pass every derivative evaluator
            /// reads; the ORDER is the caller's, so a first derivative
            /// runs an order-1 basis and never computes the order-2
            /// row it would discard.
            fn homogeneous<const N: usize>(self, t: T) -> ([[T; N]; $dim], [T; N]) {
                let ders = spline::basis::ders_basis_funs_in(self.span, t, N - 1);
                // Indexed off the window base, exactly as
                // [`Self::eval_in_span`]. A `zip` against a window slice
                // would be the wrong shape here: `ders`' row length and
                // the window's length are two derivations of the same
                // `degree`, and a `zip` would answer a disagreement by
                // silently dropping control points where indexing
                // panics.
                let base = self.first_control();
                $(let mut $c = [T::zero(); N];)+
                let mut w_hom = [T::zero(); N];
                for (k, row) in ders.iter().enumerate() {
                    for (j, nkj) in row.iter().enumerate() {
                        let i = base + j;
                        let cw = *nkj * self.curve.weights.values()[i];
                        let pt = self.curve.control[i];
                        $($c[k] = $c[k] + cw * pt.$c;)+
                        w_hom[k] = w_hom[k] + cw;
                    }
                }
                ([$($c),+], w_hom)
            }

            /// Point, first, and second derivative at `t` in the given
            /// span — one homogeneous pass (orders 0..=2 of the basis),
            /// then the rational corrections
            /// ([`rational_corrections`]), exactly as written:
            /// `C = A⁰/w⁰`, `C′ = (A¹ − C·w¹)/w⁰`,
            /// `C″ = (A² − C·w² − C′·w¹·2)/w⁰`.
            /// Same totality contract as [`Self::eval_in_span`].
            pub fn ders_in_span(self, t: T) -> ($Point<T>, $Vector<T>, $Vector<T>) {
                let ([$($c),+], w_hom) = self.homogeneous::<3>(t);
                $(let $c = rational_corrections($c, w_hom);)+
                (
                    $Point::new($($c[0]),+),
                    $Vector::new($($c[1]),+),
                    $Vector::new($($c[2]),+),
                )
            }

            /// Point and first derivative at `t` in the given span from
            /// ONE order-1 basis pass — the jet door for a consumer that
            /// wants both (a quadrature rule integrating `P × P′`, say)
            /// and would otherwise run [`Self::eval_in_span`] and
            /// [`Self::deriv_in_span`] as two passes, or
            /// [`Self::ders_in_span`] and discard `C″`.
            ///
            /// The point is [`Self::eval_in_span`]'s bit for bit (the
            /// order-0 row of the derivative recursion is the same
            /// recursion, same accumulation order, same division) and
            /// the derivative is [`Self::deriv_in_span`]'s bit for bit
            /// (it IS this, projected). Same totality contract.
            pub fn ders1_in_span(self, t: T) -> ($Point<T>, $Vector<T>) {
                let ([$($c),+], w_hom) = self.homogeneous::<2>(t);
                $(let $c = rational_corrections($c, w_hom);)+
                ($Point::new($($c[0]),+), $Vector::new($($c[1]),+))
            }

            /// First derivative in the given span: the derivative half
            /// of [`Self::ders1_in_span`] — an order-1 basis pass and
            /// the `C′` correction, the same arithmetic
            /// [`Self::ders_in_span`]'s middle component runs, bit for
            /// bit, without the order-2 row that component would
            /// discard. Same totality contract.
            pub fn deriv_in_span(self, t: T) -> $Vector<T> {
                self.ders1_in_span(t).1
            }

            /// Second derivative in the given span (the last component
            /// of [`Self::ders_in_span`] — `C″`'s correction consumes
            /// `C` and `C′`, so nothing the pass computes is surplus to
            /// it; the point and first derivative are only not
            /// returned).
            pub fn deriv2_in_span(self, t: T) -> $Vector<T> {
                self.ders_in_span(t).2
            }

        }

        impl<T: SpanLocate> $Curve<T> {
            /// Validated construction (module docs for the invariants).
            /// The knots are held in `T` ([`Knots`]; at `f64` a
            /// [`KnotVector`] converts), and so are the weights; the
            /// weights' value channel is read off them here, and every
            /// structure decision on them reads it.
            ///
            /// # Errors
            ///
            /// [`SplineError`] naming the exact violation: a weight that
            /// is no single point ([`SplineError::CarrierWeightNotAPoint`],
            /// or [`SplineError::WeightSignUndecided`] where its sign is
            /// undecided), a breakpoint that is no single point
            /// ([`SplineError::CarrierKnotNotAPoint`]), count
            /// mismatches, or a non-positive/non-finite weight.
            /// (Knot vector violations are refused earlier, by
            /// [`KnotVector::clamped`] or [`Knots::new`].)
            pub fn new(
                knots: impl Into<Knots<T>>,
                control: Vec<$Point<T>>,
                weights: Vec<T>,
            ) -> Result<Self, SplineError> {
                let knots = knots.into();
                net::carrier_knots(&knots)?;
                let weight_channel = net::weight_channel(&weights)?;
                net::validate_counts(knots.structure().control_count(), control.len(), &weight_channel)?;
                let weights = Weights::from_point_parts(weights, weight_channel);
                Ok(Self { knots, control, weights, speed: OnceLock::new(), speed_sup: OnceLock::new() })
            }

            /// The same curve on another parameter domain: the knots
            /// re-expressed on `[lo, hi]` in `T` by [`Knots::on_domain`]
            /// (ends exact, interior affine over the held values), the
            /// control net and the weights carried over verbatim.
            /// Construction goes through [`Self::from_validated_parts`],
            /// which states why no re-validation is run.
            ///
            /// A reparameterization, not a change of locus: the result
            /// at `lo + (hi − lo)·s` is this curve at `a + (b − a)·s`,
            /// to the rounding of the knot map. The domain's own
            /// validity is the knot door's question, and its `Result`
            /// is that door's and nothing else.
            ///
            /// # Errors
            ///
            /// [`Knots::on_domain`]'s: the domain is not a finite
            /// increasing interval, rounding put two breakpoints on one
            /// value, or (at an enclosure scalar) a re-expressed
            /// interior breakpoint is wider than a point
            /// ([`SplineError::CarrierKnotNotAPoint`]): the map is formed in
            /// the scalar, and an inexact one rounds outward.
            pub fn on_domain(&self, lo: f64, hi: f64) -> Result<Self, SplineError> {
                let knots = self.knots.on_domain(lo, hi)?;
                net::carrier_knots(&knots)?;
                Ok(Self::from_validated_parts(
                    knots,
                    self.control.clone(),
                    self.weights.clone(),
                ))
            }
        }

        impl<T: Real> $Curve<T> {
            /// **C6's interim**: the `f64` knot vector, where every held
            /// knot is a point ([`Knots::structure_f64_interim`], which
            /// refuses loud, by the reader's location, on a carrier with
            /// a knot wider than a point). A reader inside this macro is
            /// named by the macro's invocation, the location `rustc`
            /// gives every line it expands. Unreachable from a built
            /// carrier, whose knots are points (`net::carrier_knots`).
            /// Counted by `scripts/gates/knots-f64-interim.sh`; it goes
            /// in C6's S5.
            #[track_caller]
            pub fn knots_f64_interim(&self) -> &KnotVector {
                self.knots.structure_f64_interim()
            }

            /// The knots held in `T`, with their structure.
            pub fn knot_values(&self) -> &Knots<T> {
                &self.knots
            }

            /// The control points.
            pub fn control(&self) -> &[$Point<T>] {
                &self.control
            }

            /// The weights, held in `T` (strictly positive on their
            /// value channel).
            pub fn weights(&self) -> &[T] {
                self.weights.values()
            }

            /// **C6's interim**: the weights' value channel, the `f64`
            /// every structure decision on them reads (the knot
            /// algebra's plans, the certificates, the rational test).
            /// Arithmetic reads [`Self::weights`]. A carrier's door
            /// admits only point weights
            /// ([`SplineError::CarrierWeightNotAPoint`]), so a built
            /// carrier always has it. Counted by
            /// `scripts/gates/knots-f64-interim.sh`; it goes in C6's S5.
            #[track_caller]
            pub fn weights_f64_interim(&self) -> &[f64] {
                self.weights.weights_f64_interim()
            }

            /// The degree `p` (carried by the knot vector).
            pub fn degree(&self) -> usize {
                self.knots.degree()
            }

            /// The parameter domain (first knot at multiplicity
            /// `p + 1` to last), on the value channel.
            pub fn domain(&self) -> (f64, f64) {
                self.knots_f64_interim().domain()
            }

            /// Construction from parts whose invariants are ALREADY
            /// established — the door a structural map takes instead
            /// of [`Self::new`].
            ///
            /// The invariants are load-bearing for indexing (module
            /// docs), so skipping the check needs an argument. That
            /// argument has ONE home for the whole crate,
            /// `crate::scalar_lift`'s module docs: what a structural
            /// map is, why no shape of it changes a count or a weight
            /// value, and which part of it the `debug_assert` below
            /// cannot check. `knots` and `weights`
            /// here are a validated curve's own (or its knots on
            /// another domain, or all three under a lift that keeps
            /// every point) and `control` is that curve's net under
            /// such a map — or, from [`Self::apply_plans`], a
            /// knot-algebra plan's own knots and weights, whose every
            /// written weight the plan admitted (same module docs).
            fn from_validated_parts(
                knots: Knots<T>,
                control: Vec<$Point<T>>,
                weights: Weights<T>,
            ) -> Self {
                debug_assert!(
                    control.len() == knots.structure().control_count()
                        && weights.len() == control.len(),
                    "from_validated_parts: a structural map changed a count \
                     (control {}, knots want {}, weights {})",
                    control.len(),
                    knots.structure().control_count(),
                    weights.len()
                );
                Self { knots, control, weights, speed: OnceLock::new(), speed_sup: OnceLock::new() }
            }

            /// The same curve read at another scalar: `f` applied to
            /// every control coordinate, every weight and once per knot
            /// breakpoint ([`Knots::lift`]), the knot structure and the
            /// weights' value channel carried over. Construction goes
            /// through [`Self::from_validated_parts`], which states why
            /// no re-validation is run.
            ///
            /// A structural map, not an evaluation: `f` is a lift that
            /// keeps every value's point (`Real::from_f64`,
            /// `Dual::constant`, an interval's point), and then the
            /// lifted curve evaluates to the source and its structure is
            /// still its values' value channel. What the placeholder and
            /// a poisoned net lift to is argued once, in
            /// `crate::scalar_lift`'s module docs.
            #[must_use]
            pub fn map_scalar<U: Real>(&self, f: impl Fn(T) -> U) -> $Curve<U> {
                $Curve::from_validated_parts(
                    self.knots.lift(&f),
                    self.control.iter().map(|p| p.map(&f)).collect(),
                    self.weights.lift(&f),
                )
            }

            /// The same curve with every control point carried
            /// through `f`, the knots and the weights verbatim.
            /// Construction goes through
            /// [`Self::from_validated_parts`], which states why no
            /// re-validation is run.
            ///
            /// # What the caller owes
            ///
            /// The result is the POINTWISE IMAGE of this curve —
            /// `f(C(t))` at every `t` — exactly when `f` is
            /// **affine**, and nothing here checks that. Why an affine
            /// `f` suffices, and why the weights are therefore
            /// untouched, is the Euclidean-storage section of this
            /// module's data model, which is the one home for that
            /// rule.
            ///
            /// The consequence worth repeating at the call site: were
            /// the net stored WEIGHTED (`wj Pj`, homogeneous), this
            /// call would be wrong — an affine map's translation limb
            /// has to be scaled by `wj` there, and applying it
            /// unscaled bends the curve. Read the storage before
            /// reaching for this door.
            ///
            /// A non-affine `f` is not refused and not meaningless —
            /// it is a map of the CONTROL NET, whose curve is some
            /// other curve — but it is not this curve's image, and
            /// calling it one is the error this paragraph exists to
            /// name.
            #[must_use]
            pub fn map_points(&self, f: impl Fn($Point<T>) -> $Point<T>) -> Self {
                Self::from_validated_parts(
                    self.knots.clone(),
                    self.control.iter().map(|p| f(*p)).collect(),
                    self.weights.clone(),
                )
            }

            /// The [`Span`] of this curve's knot vector at `index`, as
            /// a window on **this** curve — `None` when the index is out
            /// of range or names an empty span (interior multiplicity).
            /// The emptiness check and the window construction are one
            /// operation.
            pub fn span(&self, index: usize) -> Option<$Window<'_, T>> {
                Some(self.window_of(self.knots.span(index)?))
            }

            /// The window containing `t`, or `None` at NaN, exactly as
            /// [`KnotVector::span_at`] locates it (out-of-domain clamps
            /// to an end span).
            pub fn span_at(&self, t: f64) -> Option<$Window<'_, T>> {
                self.span(self.knots_f64_interim().span_at(t)?.index())
            }

            /// The one primitive constructor, behind [`Self::span`] and
            /// [`Self::span_at`] and the located-span walk behind the
            /// full evaluators. It is private because it is the single
            /// place where a span and a curve are put together, and
            /// every caller draws the span from `self.knots`.
            fn window_of<'a>(&'a self, span: KnotSpan<'a, T>) -> $Window<'a, T> {
                $Window { curve: self, span }
            }

        }

        /// The two halves of the curve's `apply_plans`: the projective
        /// applier at an evaluation scalar, the homogeneous one at an
        /// enclosure scalar (C6 S4).
        impl<T: SpanLocate> spline::ProjectiveSplit<T> for PlanChain<'_, $Curve<T>, T> {
            type Out = Result<$Curve<T>, KnotAlgebraError>;

            /// Each step's coefficient, weight and `λ` formed at `T`
            /// from the held knots and weights
            /// ([`spline::CurvePlan::apply_rational`]), points via
            /// `lerp(x, y, λ)`, the fixed association. The weights'
            /// value channel is the final plan's, which the plans
            /// computed on the channel with the same operations, and
            /// whose every written weight the plan admitted
            /// (`geom_core::spline::algebra`'s written-weight check:
            /// strictly positive and finite, or the plan refused). That
            /// holds of reference plans, the only ones a carrier at a
            /// projective scalar makes (its knots and weights are points);
            /// a plan decided against a stand-in refuses
            /// [`KnotAlgebraError::StandInPlan`] rather than leave the
            /// stand-in's unit weights as the channel.
            fn projective(self) -> Self::Out
            where
                T: ProjectiveScalar,
            {
                let PlanChain { curve, plans, fresh } = self;
                let Some(last) = plans.last() else {
                    return Ok(curve.clone());
                };
                if !plans.iter().all(spline::CurvePlan::is_reference) {
                    return Err(KnotAlgebraError::StandInPlan);
                }
                let mut knots = curve.knots.clone();
                let (mut control, mut weights) = (curve.control.clone(), curve.weights.values().to_vec());
                for plan in plans {
                    (control, weights) = plan.apply_rational(
                        knots.values(),
                        &control,
                        &weights,
                        (net::poison_point::<T, $Point<T>>(), fresh),
                        |x, y, l: T| x.lerp(y, l),
                    );
                    knots = knots.restructure(plan.knots().clone(), fresh);
                }
                Ok($Curve::from_validated_parts(knots, control, Weights::from_point_parts(weights, last.weights().to_vec())))
            }

            /// The net as `(w·P, w)` through
            /// [`spline::CurvePlan::apply_homogeneous`], each step's
            /// coefficient formed at `T` from the held knots, then
            /// de-homogenized once. No weight quotient `λ` is formed. The
            /// result enters through the carrier's door: written weights
            /// certified positive for every member
            /// ([`SplineError::WeightSignUndecided`] where some are not),
            /// and refused as [`SplineError::CarrierWeightNotAPoint`]
            /// where one is wider than a point (C6's interim until S5). A
            /// polynomial net keeps its unit weights exactly, so it is
            /// admitted.
            fn enclosure(self) -> Self::Out {
                let PlanChain { curve, plans, fresh } = self;
                if plans.is_empty() {
                    return Ok(curve.clone());
                }
                let origin = $Point::<T>::origin();
                let poison = (
                    net::poison_point::<T, $Point<T>>() - origin,
                    spline::poison_from(T::one()),
                );
                let mut knots = curve.knots.clone();
                let mut hom: Vec<($Vector<T>, T)> = curve
                    .control
                    .iter()
                    .zip(curve.weights.values())
                    .map(|(p, w)| ((*p - origin) * *w, *w))
                    .collect();
                for plan in plans {
                    hom = plan.apply_homogeneous(knots.values(), &hom, (poison, fresh), |x, y, t: T| {
                        $Vector::new($(T::homogeneous_step(x.$c, y.$c, t)),+)
                    });
                    knots = knots.restructure(plan.knots().clone(), fresh);
                }
                let (control, weights): (Vec<$Point<T>>, Vec<T>) =
                    hom.into_iter().map(|(v, w)| (origin + v / w, w)).unzip();
                // A removal's ratio read from knots within rounding of each
                // other can reach 0 or 1, and its unblend divides by it: an
                // unbounded enclosure, which the door's poison test does not
                // see. Refused here, typed; a poison value is the door's.
                let unbounded = |v: &T| !v.is_poison() && !v.is_bounded();
                if let Some(index) = control.iter().zip(&weights).position(|(p, w)| {
                    unbounded(w) $(|| unbounded(&p.$c))+
                }) {
                    return Err(KnotAlgebraError::UnboundedAtEnclosure { index });
                }
                $Curve::new(knots, control, weights).map_err(KnotAlgebraError::Structure)
            }
        }

        /// The knot algebra, at every scalar: projective at an
        /// evaluation scalar, homogeneous at an enclosure scalar
        /// ([`PlanChain`]'s two halves; C6 S4).
        impl<T: SpanLocate> $Curve<T> {
            /// A value a knot operation brings in at the caller's `f64`
            /// parameter, entered as the constant it is.
            fn inserted(v: f64) -> T {
                T::from_f64(v)
            }

            /// Applies a chain of structure plans to this curve's net
            /// held in `T` ([`PlanChain`]: the projective applier at an
            /// evaluation scalar, the homogeneous one at an enclosure
            /// scalar), the held knots carried plan to plan
            /// ([`Knots::restructure`]). A value a plan brings in is
            /// `fresh(v)`: the caller's `f64` parameter entered as the
            /// constant it is ([`Self::inserted`]), or, for a removal's
            /// reinsertion, the held value the removal took out. Empty
            /// chain ⇒ clone.
            ///
            /// # Errors
            ///
            /// At an enclosure scalar, the carrier's door's refusals of
            /// the result ([`KnotAlgebraError::Structure`]); none at an
            /// evaluation scalar.
            fn apply_plans(&self, plans: &[spline::CurvePlan], fresh: &dyn Fn(f64) -> T) -> Result<Self, KnotAlgebraError> {
                T::split_projective(PlanChain { curve: self, plans, fresh })
            }

            /// Knot insertion (§5.2), single value `times`-fold — the
            /// future `split_edge` substrate. Evaluation-invariant in ℝ.
            ///
            /// # Errors
            ///
            /// [`KnotAlgebraError`]: out-of-domain `u`, interior
            /// multiplicity budget (`degree`) exceeded, structure
            /// mismatch, or [`KnotAlgebraError::WeightCollapse`] when an
            /// inserted weight rounds out of the positive regime (both
            /// blend terms underflowing, on subnormal weights).
            pub fn insert_knot(&self, u: f64, times: usize) -> Result<Self, KnotAlgebraError> {
                let plans = spline::plan_inputs::insert_knot_plan(&self.knots, &self.weights, u, times)?;
                self.apply_plans(&plans, &Self::inserted)
            }

            /// Splits the curve at interior parameter `u` into two
            /// clamped curves covering `[t₀, u]` and `[u, t₁]` — knot
            /// insertion to full interior multiplicity, then the
            /// control/knot partition (§5.2; C12.3: the NURBS
            /// `split_edge` substrate, M5 PR 9). Evaluation-invariant
            /// in ℝ, and each child keeps the PARENT's parameter (the
            /// split value is not re-normalized), so a caller's
            /// `[t₀, u]` interval on the child means exactly what it
            /// meant on the parent.
            ///
            /// # Errors
            ///
            /// [`KnotAlgebraError`]: `u` outside the open knot domain
            /// (boundary splits are refused — a clamped end already
            /// has full multiplicity and an empty child is not a
            /// curve), or the insertion path's refusals, its
            /// [`KnotAlgebraError::WeightCollapse`] included
            /// ([`Self::insert_knot`]).
            pub fn split_at(&self, u: f64) -> Result<(Self, Self), KnotAlgebraError> {
                let p = self.knots.degree();
                // `u`'s place, the domain test included, decided for every
                // member of the held knots (`spline::plan_inputs`).
                let have = spline::plan_inputs::split_multiplicity(&self.knots, u)?;
                let full = if have < p {
                    self.insert_knot(u, p - have)?
                } else {
                    self.clone()
                };
                // Child 1: every knot below `u` plus `p + 1` copies of
                // `u`; control points 0..f (C(u) is control index
                // f − 1 of the saturated curve, shared by both).
                // Child 2: one extra copy of `u`, then every knot from
                // index f on; control points f − 1 onward. Each child's
                // knots are breakpoints of the saturated curve, so each
                // keeps its held value (`u` included: present at
                // multiplicity `p`); nothing is entered fresh.
                let (f, k1, k2) = spline::plan_inputs::split_knots(&full.knots, u, Self::inserted)?;
                let c1 = Self::new(
                    k1,
                    full.control[..f].to_vec(),
                    full.weights.values()[..f].to_vec(),
                )
                .map_err(KnotAlgebraError::Structure)?;
                let c2 = Self::new(
                    k2,
                    full.control[f - 1..].to_vec(),
                    full.weights.values()[f - 1..].to_vec(),
                )
                .map_err(KnotAlgebraError::Structure)?;
                Ok((c1, c2))
            }

            /// Knot refinement (§5.3): inserts every value of `add`
            /// (ascending, ties consecutive). Evaluation-invariant in ℝ.
            ///
            /// # Errors
            ///
            /// As [`Self::insert_knot`], against the cumulative
            /// structure, [`KnotAlgebraError::WeightCollapse`] included.
            pub fn refine_knots(&self, add: &[f64]) -> Result<Self, KnotAlgebraError> {
                let plans = spline::plan_inputs::refine_plan(&self.knots, &self.weights, add)?;
                self.apply_plans(&plans, &Self::inserted)
            }

            /// Knot merging (§5.3): every curve refined onto the UNION
            /// of their knot vectors — each distinct interior value at
            /// the greatest multiplicity any of them gives it — so that
            /// afterwards all share one bit-identical knot vector, and
            /// (at one degree) one control-point count. The structure
            /// half is [`spline::algebra::union_refinements`]; each
            /// curve is then [`Self::refine_knots`], so the result is
            /// evaluation-invariant in ℝ and a curve already on the
            /// union comes back as itself. Same order as the input.
            ///
            /// # Errors
            ///
            /// As [`Self::refine_knots`] — curves at different degrees
            /// or on different domains name insertions their vector
            /// refuses (`MultiplicityOverflow`,
            /// `ParameterOutsideDomain`); a caller that wants them
            /// compatible elevates and checks the domain first.
            pub fn refine_to_union<'a>(
                curves: impl IntoIterator<Item = &'a Self>,
            ) -> Result<Vec<Self>, KnotAlgebraError>
            where
                T: 'a,
            {
                let curves: Vec<&Self> = curves.into_iter().collect();
                let vectors: Vec<&KnotVector> = curves
                    .iter()
                    .map(|c| spline::plan_inputs::point_knots(&c.knots))
                    .collect::<Result<_, _>>()?;
                curves
                    .iter()
                    .zip(spline::algebra::union_refinements(&vectors))
                    .map(|(c, add)| c.refine_knots(&add))
                    .collect()
            }

            /// Bounded knot removal (§5.4): removes `times` copies of
            /// the interior knot `u` and returns the rewritten curve
            /// **with a sup-norm error bound** `B` such that
            /// `|C(t) − Ĉ(t)| ≤ B` over the whole domain — removal is
            /// bounded, never silent. The bound is the Eq. 9.81
            /// mechanism in projective form: each pass reinserts the
            /// removed copy exactly and bounds the polygon perturbation
            /// through partition of unity and the positive-weight
            /// convex hull —
            /// `B_pass = (Cmax·Bw + Bwp) / w̃min`, where `Cmax` bounds
            /// `|C|` by the control hull, `Bw`/`Bwp` are the max weight
            /// and weighted-point perturbations, and `w̃min` lower-bounds
            /// the reinserted weight function; passes add (triangle
            /// inequality). Conservative: the per-basis sup factor is
            /// relaxed to 1. At interval scalars the bound is itself an
            /// enclosure (containment composes).
            ///
            /// # Errors
            ///
            /// [`KnotAlgebraError`]: `u` not an interior knot (exact
            /// f64 identity), removing past its multiplicity, or
            /// [`KnotAlgebraError::WeightCollapse`]: a weight the
            /// removal chain writes, or one its reinsertion writes,
            /// out of the positive regime.
            pub fn remove_knot(&self, u: f64, times: usize) -> Result<(Self, T), KnotAlgebraError> {
                let steps = spline::plan_inputs::remove_knot_plan(&self.knots, &self.weights, u, times)?;
                let mut cur = self.clone();
                let mut bound = T::zero();
                for step in &steps {
                    let removed = cur.apply_plans(core::slice::from_ref(&step.plan), &Self::inserted)?;
                    // The reinsertion puts back the knot the removal took
                    // out: its held value, so the bound reads the curve's
                    // own knot rather than a constant copy of it.
                    // The reinsertion brings in `u` alone.
                    let held_u = spline::plan_inputs::breakpoint_at(&cur.knots, u)?
                        .unwrap_or_else(|| Self::inserted(u));
                    let held = |v: f64| if v.to_bits() == u.to_bits() { held_u } else { Self::inserted(v) };
                    let reinserted =
                        removed.apply_plans(core::slice::from_ref(&step.reinsert), &held)?;
                    bound = bound + net::removal_pass_bound(
                        (&cur.control, cur.weights.values()),
                        (&reinserted.control, reinserted.weights.values()),
                    );
                    cur = removed;
                }
                Ok((cur, bound))
            }

        }

        impl<T: SpanLocate> $Curve<T> {
            /// A **certified lower bound** on `‖C′(t)‖` over the whole
            /// domain, in meters per parameter unit — the "meter" a
            /// parameter-space margin must be multiplied by to become a
            /// length (D4 ¶1). It is an
            /// [`InfSpeed`](geom_core::InfSpeed) by signature: the
            /// bound direction is what every consumer relies on (a
            /// span this meter proves forward IS forward in metres),
            /// and under-stating is what makes that sound.
            ///
            /// This is the rung-3 analogue of the conic lane's
            /// conservative meters (`Circle` ⇒ radius, `Ellipse` ⇒ the
            /// MINOR semi-axis): without it, a fitted SSI carrier
            /// reaching `split_edge` has no honest way to state
            /// "definitely interior in meters", and the parameter gate
            /// would either poison or, far worse, use an *upper* bound
            /// and accept a split that is not clear of the endpoints.
            ///
            /// # How it is certified
            ///
            /// The derivative of a clamped B-spline is a B-spline of
            /// degree `p − 1` over the derivative control points
            /// `Qᵢ = p·(Pᵢ₊₁ − Pᵢ)/(uᵢ₊ₚ₊₁ − uᵢ₊₁)`, so on every span
            /// `C′(t)` is a **convex combination** of the local `Qᵢ`.
            /// Fix any unit direction `d`: then
            /// `‖C′‖ ≥ d·C′ ≥ minᵢ (d·Qᵢ)` over the `Qᵢ` active where
            /// `d` is applied. The arm runs **three independent
            /// assemblies** of that inequality and states their join:
            ///
            /// 1. the **global-chord** assembly — the retired original
            ///    arm, verbatim: one direction (first→last control
            ///    point, the chord a monotone carrier advances along),
            ///    min over ALL `Qᵢ`;
            /// 2. the **per-span** assembly — the M8-2 rational
            ///    template carried over: each nonempty span projects
            ///    its ACTIVE `Qᵢ` (`i ∈ span−p .. span`) on the span's
            ///    own control chord `P_span − P_{span−p}`, and the
            ///    whole domain is the ascending `Real::min` fold over
            ///    spans. A per-span direction is legitimate because
            ///    `‖C′(t)‖ ≥ d_s·C′(t)` holds for *every* unit `d_s`,
            ///    so the min over spans of per-span bounds still
            ///    bounds the whole domain (the M8-2 review's
            ///    soundness argument, unchanged);
            /// 3. the **piece** assembly ([`Self::piece_floor`]):
            ///    each nonempty span cut into
            ///    [`SPEED_PIECE_SPLITS`] pieces, each piece
            ///    projecting the derivative's own Bernstein
            ///    coefficients there on their sum's direction. A curve
            ///    that sways across its control chords — a wavy
            ///    loft's corner — has `Qᵢ` pointing back along every
            ///    chord the first two try, while on a short piece the
            ///    coefficients crowd around `C′` itself, so this
            ///    assembly answers wherever the speed stays clear of
            ///    zero at the piece scale.
            ///
            /// **The join**: an assembly whose direction collapsed
            /// (poison) abstains; if every one abstains the answer is
            /// poison; otherwise it is the `max` of the real ones —
            /// sound because each is independently a lower bound on
            /// the same `inf‖C′‖`, and never below either chord
            /// assembly. Each assembly still decides some carrier: the
            /// pieces a curve that sways across its chords, the
            /// global chord a jagged nearly straight net whose pieces
            /// each lean off it, the per-span chords a few nets
            /// between (`tests/curves/piece_coverage_meter.rs`, which
            /// also pins the floor and the cell where only the pieces
            /// abstain; the chord-assembly cells are pinned in
            /// `lt_r1_probes.rs::r1_join_abstention_logic` and
            /// `r2_lt_probes.rs::the_join_lattice_is_pinned_cell_by_cell`).
            /// A long-turn carrier (a helix past half a revolution, a
            /// closed loop) whose speed never drops does not collapse
            /// the meter merely because its tangent leaves the global
            /// chord's half-space: the per-span and piece assemblies
            /// answer there.
            ///
            /// The result may still be zero or negative — a genuine
            /// stationary point (cusp, turn-around) defeats every
            /// direction on its own span — and that is reported
            /// honestly, so the margin collapses and the caller's
            /// trilean escalates rather than guessing.
            ///
            /// **Rational carriers** — weights not all one plain constant — take the
            /// second arm, [`Self::rational_speed_lower_bound`]. Equal
            /// weights cancel out of every evaluation, so such a curve
            /// is the polynomial one on the same net and this arm's
            /// argument holds for it unchanged — it is the same curve,
            /// which says nothing about which arm bounds it best. The derivative of
            /// a rational spline is *not* a convex combination of any
            /// control net, so the argument above does not apply
            /// directly and a quotient-rule assembly stands in. The
            /// contract (whole domain, m/param, honestly non-positive
            /// when the curve gives the assembly nothing to stand on,
            /// poison when the structure refuses) is the same on both
            /// arms.
            ///
            /// # What the bound does and does not certify
            ///
            /// It certifies **speed**, and through speed, **arc
            /// length**: over any parameter interval `[a, b]` inside
            /// the domain, `(b − a)·bound ≤ ∫ₐᵇ ‖C′‖`. That is exactly
            /// what every consumer asks of it — `interval_span_forward`
            /// converts a parameter span to metres of arc,
            /// `split_edge_param_interior` converts a distance-to-
            /// endpoint the same way.
            ///
            /// It certifies **nothing about injectivity, turning, or
            /// monotone advance along any direction**. A carrier may
            /// reverse, loop, or return arbitrarily close to a point it
            /// has already visited and still meter positively, provided
            /// its speed never collapses — reversal is not
            /// disqualifying, only a genuine stationary point (a cusp,
            /// a turn-around, a degenerate span) is. Callers that need
            /// non-self-intersection or a bounded turn must obtain it
            /// elsewhere; this number will not supply it, and reading it
            /// as if it did would be reading an arc-length rate as a
            /// chord-distance rate.
            ///
            /// Both arms read local directions — per span, and on the
            /// integral arm per piece too — and both are sound lower
            /// bounds on `‖C′‖`; neither is a claim about the curve's
            /// shape beyond its speed.
            ///
            /// The arm is chosen on the weights' **plain constants**
            /// ([`SpanLocate::plain_constant`]: every `w_j` the same plain
            /// constant, exactly), never on an evaluation scalar and
            /// never on the value channel alone, which would read a
            /// `Dual` or `Sym` weight that moves with a parameter or a
            /// symbol as the constant its value is. That is the
            /// conservative member of the class
            /// `work/inlane/held-value-equalities-decided-on-the-value-channel.md`
            /// names; a `T`-valued equality that `Sym` discharges would
            /// add reach, never soundness.
            ///
            /// # Rounding posture
            ///
            /// The directions are `chord/‖chord‖` — unit only to
            /// rounding at `f64` — and the piece assembly's
            /// coefficients are `q + 1` rounded steps from the stored
            /// net (its blossoms and one exact-weight combination), so
            /// the plain-f64 reading is a bound up to a few relative
            /// ulps of the derivative's scale (the stall rows in
            /// `tests/curves/piece_coverage_meter.rs` hold it to
            /// `1e-12` of that scale). The `Interval` instantiation is
            /// the certified lane, and the bracket row in
            /// `tests/curves/m5_pr7_speed_meter.rs` pins containment.
            ///
            /// # Poison (total, D4 ¶2)
            ///
            /// A zero degree, fewer than two control points, a
            /// non-positive difference of the knots framing any
            /// derivative coefficient (`u_{i+p+1} ≤ u_{i+1}` — a
            /// structural violation the old arm turned into ±∞/NaN
            /// arithmetic instead of naming), a knot vector with no
            /// nonempty span, or EVERY assembly abstaining — each one
            /// yields NaN. A bound is never fabricated. The
            /// knot-difference clause is DEFENSIVE: it needs an
            /// interior multiplicity of `p + 1`, which Clamped-v1
            /// validation forbids (interior multiplicity ≤ `p`, end
            /// multiplicity exactly `p + 1` with nonempty end spans),
            /// so no validated constructor reaches it — it guards
            /// future unvalidated paths, and is an untestable-by-
            /// construction claim, stated as such rather than pinned.
            ///
            /// Structural misses INSIDE the per-span scan (an active
            /// window past the control net, an index underflow) poison
            /// the WHOLE meter rather than abstaining the one
            /// assembly — deliberate asymmetry: chord collapse is a
            /// fact about a well-formed curve, a range miss is a
            /// construction-invariant break, and fail-loud beats
            /// recovering around corrupted structure.
            ///
            /// The join's one-sided recovery is NOT poison
            /// laundering: an assembly abstains only when its own
            /// chord DIRECTION collapsed (`0/0`), a structural fact
            /// about which projections exist, while a poisoned INPUT
            /// (a non-finite control point) poisons the projections
            /// of every assembly whose active set touches it — the
            /// global assembly's min covers all `Qᵢ` and the span
            /// containing the point poisons the per-span fold, so
            /// corrupted data still reaches the caller as poison
            /// through both arms at once. `Real::is_poison` here
            /// discriminates assembly structure, never geometry: the
            /// geometric decision (is the bound positive?) stays with
            /// the caller's trilean.
            ///
            /// **Memoised outside a symbolic session only.** A session's
            /// table holds the nodes it saw built, and a memo served
            /// across that boundary would change what it decides; inside
            /// one the meter is computed afresh and nothing it mints is
            /// kept.
            pub fn speed_lower_bound(&self) -> geom_core::InfSpeed<T> {
                session_memo(&self.speed, || self.speed_lower_bound_uncached())
            }

            /// A certified **upper** bound on `‖C′(t)‖` over the whole
            /// domain, in m/param: the max over the meter's pieces
            /// ([`Self::speed_lower_bound`]'s, on the same arm) of
            /// [`geom_core::spline::QuotientPiece::sup_norm`], the
            /// largest coefficient ratio's norm (the
            /// `geom_core::spline::derivative` module docs hold the
            /// derivation). At equal weights it is the max norm of the
            /// derivative's Bernstein coefficients on the pieces. At
            /// `Interval` it is certified; at `f64` it is the same
            /// expression in round-to-nearest. Poison where the
            /// statement refuses (a rational degree above 28).
            ///
            /// Memoised on the same terms as [`Self::speed_lower_bound`].
            pub fn speed_upper_bound(&self) -> geom_core::SupSpeed<T> {
                session_memo(&self.speed_sup, || {
                    geom_core::SupSpeed::new(
                        self.speed_pieces(self.is_rational())
                            .and_then(|ps| {
                                ps.iter().map(spline::QuotientPiece::sup_norm).reduce(Real::max)
                            })
                            .unwrap_or_else(|| T::from_f64(f64::NAN)),
                    )
                })
            }

            /// A certified **upper** bound on `‖C″(t)‖` over the whole
            /// domain, in m/param², read as a Lipschitz constant of `C′`
            /// across it: the max over the second-order pieces
            /// ([`spline::accel_pieces`], `C″ = M/W³` on
            /// [`SPEED_PIECE_SPLITS`] pieces per nonempty span) of
            /// [`spline::QuotientPiece::sup_norm`] (the
            /// `geom_core::spline::derivative` module docs, "The second
            /// order"), each a bound on its own open span alone. Infinite
            /// where the knots hold a crease (`KnotStructure::crease_runs`),
            /// where `C′` may jump and no finite bound holds across the
            /// knot (poison at `Interval`, which holds no infinity). The
            /// cost: a creased spline reads no second-order bound even
            /// inside a smooth span, so its touches and tangent meetings
            /// walk to the band.
            /// At `Interval` it is certified; at `f64` it is the same
            /// expression in round-to-nearest. Poison where the
            /// statement refuses (a degree above 18, where `3p` passes
            /// the ceiling); at `Interval`, uncertified where a high
            /// degree meets a wide weight spread (the module docs'
            /// practical ceiling).
            pub fn accel_upper_bound(&self) -> T {
                if self.knots.structure().crease_runs().next().is_some() {
                    return T::from_f64(f64::INFINITY);
                }
                let points: Vec<[T; $dim]> = self.control.iter().map(|p| [$(p.$c),+]).collect();
                spline::accel_pieces(&self.knots, &points, self.weights.values(), SPEED_PIECE_SPLITS)
                    .and_then(|ps| ps.iter().map(spline::QuotientPiece::sup_norm).reduce(Real::max))
                    .unwrap_or_else(|| T::from_f64(f64::NAN))
            }

            /// **Whether this carrier is polynomial**: the one plain
            /// constant every weight is, or `None`
            /// ([`Weights::constant`]; [`spline::constant_weight`] says
            /// why `Some(c)` is the polynomial on [`Self::control`] for
            /// every `c`).
            pub fn constant_weight(&self) -> Option<f64> {
                self.weights.constant()
            }

            /// Whether the weights are not all one plain constant: the
            /// arm choice of [`Self::speed_lower_bound`] (its docs).
            fn is_rational(&self) -> bool {
                self.constant_weight().is_none()
            }

            /// [`Self::speed_lower_bound`]'s computation: once per curve
            /// outside a symbolic session, at every read inside one.
            fn speed_lower_bound_uncached(&self) -> geom_core::InfSpeed<T> {
                let poison = T::from_f64(f64::NAN);
                // Weights not all one plain constant ⇒ rational: the
                // convexity argument does not hold directly, and the
                // quotient-rule arm takes over. Equal plain constants
                // cancel out of every evaluation, leaving the polynomial
                // curve on the same net, which this arm's argument
                // covers; a weight that carries a derivative or a symbol
                // is no plain constant, whatever its value.
                if self.is_rational() {
                    return geom_core::InfSpeed::new(self.rational_speed_lower_bound());
                }
                let p = self.knots.degree();
                if p == 0 || self.control.len() < 2 {
                    return geom_core::InfSpeed::new(poison);
                }
                let knots = self.knots_f64_interim().knots();
                let held = self.knots.values();
                // Derivative coefficients, once for the curve, with
                // every knot difference checked POSITIVE on f64
                // structure (the totality clause above), so each is
                // finite arithmetic on finite structure.
                let Some(coeffs) = derivative_net(&self.control, knots, held, p, 0) else {
                    return geom_core::InfSpeed::new(poison);
                };
                // ---- Assembly 1: the global chord (the retired
                // original arm, verbatim — same direction, same fold
                // order, bit-identical where it was defined). ----
                let (Some(first), Some(last)) = (self.control.first(), self.control.last()) else {
                    return geom_core::InfSpeed::new(poison);
                };
                let global = {
                    let chord = *last - *first;
                    // A collapsed chord makes this 0/0 ⇒ poison ⇒ the
                    // assembly abstains at the join (the rational
                    // arm's chord treatment).
                    let d = chord / chord.norm();
                    let mut acc: Option<T> = None;
                    for q in &coeffs {
                        let v = d.dot(*q);
                        acc = Some(match acc {
                            None => v,
                            // `Real::min`/`max` are NaN-propagating
                            // (poison in, poison out) and total — no
                            // comparison here or below.
                            Some(m) => m.min(v),
                        });
                    }
                    acc.unwrap_or(poison)
                };
                // ---- Assembly 2: per-span chords (the M8-2 rational
                // template), fixed ascending span order (D9): the min
                // of `d_span·Qᵢ` over the ACTIVE coefficients
                // (`i ∈ span−p .. span`), folded over spans. ----
                let perspan = {
                    let mut acc: Option<T> = None;
                    for index in self.knots_f64_interim().first_span()..=self.knots_f64_interim().last_span() {
                        // Emptiness check and span validation are one step.
                        let Some(span) = self.knots_f64_interim().span(index) else {
                            continue;
                        };
                        // The window's base, subtracted once inside
                        // `Span` — the `span − p` that used to need a
                        // `checked_sub` here.
                        let (lo_i, span) = (span.first_control(), span.index());
                        // The remaining range miss is the one a `Span`
                        // cannot speak to: it bounds its window by the
                        // KNOT vector's control count, not by this
                        // curve's array. A mismatch poisons the WHOLE
                        // meter (early return), not just this assembly
                        // — a construction-invariant break fails loud
                        // (doc: "Poison", the stated asymmetry with
                        // chord-collapse abstention).
                        if span >= self.control.len() {
                            return geom_core::InfSpeed::new(poison);
                        }
                        let Some(active) = coeffs.get(lo_i..span) else {
                            return geom_core::InfSpeed::new(poison);
                        };
                        // The span's own control chord, as unit
                        // direction; collapse ⇒ 0/0 ⇒ this span
                        // poisons THIS assembly (which then abstains
                        // at the join — the other assembly still
                        // covers the same span soundly).
                        let (Some(a), Some(b)) =
                            (self.control.get(lo_i), self.control.get(span))
                        else {
                            return geom_core::InfSpeed::new(poison);
                        };
                        let chord = *b - *a;
                        let d = chord / chord.norm();
                        for q in active {
                            let v = d.dot(*q);
                            acc = Some(match acc {
                                None => v,
                                Some(m) => m.min(v),
                            });
                        }
                    }
                    acc.unwrap_or(poison)
                };
                let pieces = if p == 1 { poison } else { self.piece_floor(false) };
                // ---- The join (doc: "The join"). ----
                let joined = [global, perspan, pieces]
                    .into_iter()
                    .filter(|b| !b.is_poison())
                    .reduce(Real::max);
                geom_core::InfSpeed::new(joined.unwrap_or(poison))
            }

            /// The derivative's quotient pieces
            /// ([`geom_core::spline::speed_pieces`]) on
            /// [`SPEED_PIECE_SPLITS`] pieces per nonempty span: `C′`
            /// over one where `rational` is false (the weights all one
            /// plain constant), `N/W²` where it is true. `None` where
            /// the statement refuses: a degree above its ceiling
            /// ([`geom_core::spline::QUOTIENT_MAX_DEGREE`], `2p ≤ 56`
            /// on the rational reading), an index past the net.
            fn speed_pieces(&self, rational: bool) -> Option<Vec<spline::QuotientPiece<T, $dim>>> {
                let points: Vec<[T; $dim]> = self.control.iter().map(|p| [$(p.$c),+]).collect();
                spline::speed_pieces(
                    &self.knots,
                    &points,
                    rational.then_some(self.weights.values()),
                    SPEED_PIECE_SPLITS,
                )
            }

            /// The piece reading of [`Self::speed_lower_bound`], one
            /// call on both arms: the min over the pieces of
            /// [`spline::QuotientPiece::inf_norm`], each piece's
            /// coefficient ratios projected on their numerator's sum.
            /// On a piece the quantity is a convex combination of its
            /// ratios (`geom_core::spline::derivative`'s module docs),
            /// so for the unit direction `d` of their sum
            /// `‖C′‖ ≥ minₖ d·aₖ/bₖ`, and the min over pieces and spans
            /// bounds the whole domain. At equal weights `bₖ = 1` and
            /// the ratios are the derivative's own Bernstein
            /// coefficients on the piece: a curve that sways across its
            /// control chords — a wavy loft's corner — has `Qᵢ`
            /// pointing back along every chord, while on a short piece
            /// the coefficients crowd around `C′` itself, so this
            /// reading answers wherever the speed stays clear of zero at
            /// the piece scale. Poison where a piece's numerator sums
            /// to zero or the statement refuses.
            fn piece_floor(&self, rational: bool) -> T {
                self.speed_pieces(rational)
                    .and_then(|ps| ps.iter().map(spline::QuotientPiece::inf_norm).reduce(Real::min))
                    .unwrap_or_else(|| T::from_f64(f64::NAN))
            }

            /// The **rational arm** of [`Self::speed_lower_bound`]: a
            /// certified lower bound on `‖C′(t)‖` over the whole domain
            /// for a carrier whose weights are not all one plain constant,
            /// in m/param. It is [`Self::piece_floor`] on `N/W²` (the
            /// derivation lives there), behind the licence it needs.
            ///
            /// # Licence
            ///
            /// **Strictly positive weights**, checked here on `f64`
            /// structure (never on an evaluation scalar): without them
            /// neither the rational basis nor `w`'s own hull is a convex
            /// combination.
            ///
            /// # Poison (total, D4 ¶2)
            ///
            /// A zero degree, fewer than two control points, any
            /// non-positive or non-finite weight, and every abstention
            /// of the piece reading yield NaN: a knot vector with no
            /// nonempty span, a piece whose numerator coefficients sum
            /// to zero, and **a degree above 28**, where `2p` passes the
            /// statement's ceiling
            /// ([`geom_core::spline::QUOTIENT_MAX_DEGREE`]). A rational
            /// carrier above that degree is refused, never metered by a
            /// looser reading. A bound is never fabricated.
            ///
            /// # Rounding posture
            ///
            /// At `f64` the reading runs in nearest rounding, like
            /// every other `Real`-generic bound in the kernel, and the
            /// `f64` reading is **not** a bound: its error grows with
            /// the weight ratio. **The `Interval` instantiation is the certified
            /// lane**: every coefficient it reads encloses the exact
            /// one, and `tests/curves/m5_pr7_speed_meter.rs`'s bracket
            /// row pins that the interval answer contains the `f64`
            /// one.
            ///
            /// # Conservatism
            ///
            /// The answer is a bound, not an estimate, and the gap can
            /// be wide. A curve that turns hard can refuse outright
            /// while its true speed is comfortably positive. Refusal is
            /// always sound and only ever a usability cost; the
            /// frontier row in `tests/curves/m5_pr7_speed_meter.rs`
            /// pins where it currently falls, and
            /// `tests/curves/swaying_corner_meter.rs` the reach on a
            /// swaying corner, so the schedule cannot be changed without
            /// the trade-off becoming visible.
            fn rational_speed_lower_bound(&self) -> T {
                let poison = T::from_f64(f64::NAN);
                let p = self.knots.degree();
                if p == 0 || self.control.len() < 2 {
                    return poison;
                }
                if !self.weights.weights_f64_interim().iter().all(|w| spline::weight_in_regime(*w)) {
                    return poison;
                }
                self.piece_floor(true)
            }

        }

        impl<T: Real> $Curve<T> {
            /// A certified sup-norm bound on `|C_self − C_other|` for
            /// two curves **sharing one knot vector** (same degree,
            /// same control count; weights may differ): the
            /// `net::removal_pass_bound` formula, which only uses
            /// that sharing — `(Cmax·Bw + Bwp)/w̃min` through partition
            /// of unity and the positive-weight convex hull. Poison
            /// (NaN) when the structures do not match — total, never
            /// a fabricated bound. Crate-internal: the fitting stack's
            /// deviation measurements (M5 PR 4) ride it.
            pub(crate) fn same_structure_deviation_bound(&self, other: &Self) -> T {
                if self.knots_f64_interim() != other.knots_f64_interim() || self.control.len() != other.control.len() {
                    return T::from_f64(f64::NAN);
                }
                net::removal_pass_bound(
                    (&self.control, self.weights.values()),
                    (&other.control, other.weights.values()),
                )
            }

        }

        impl<T: SpanLocate> $Curve<T> {
            /// Degree elevation (§5.5) by `raise` (≥ 1), via the Bézier
            /// route (`geom_core::spline::algebra::elevate_plan`):
            /// decompose, elevate each segment binomially, recompose
            /// with exact removals. Evaluation-invariant in ℝ.
            ///
            /// # Errors
            ///
            /// [`KnotAlgebraError`]: structure refusals, or
            /// [`KnotAlgebraError::WeightCollapse`] when floating point
            /// writes a weight out of the positive regime in any stage
            /// (the Bézier decomposition's insertions, the elevation's
            /// blends, the recomposition's removals); surfaced honestly.
            pub fn elevate_degree(&self, raise: usize) -> Result<Self, KnotAlgebraError> {
                let mut cur = self.clone();
                for _ in 0..raise {
                    let plans = spline::plan_inputs::elevate_plan(&cur.knots, &cur.weights)?;
                    cur = cur.apply_plans(&plans, &Self::inserted)?;
                }
                Ok(cur)
            }
        }

        impl<T: SpanLocate> $Curve<T> {
            /// The located-span walk every full evaluator IS: span
            /// selection through the sealed [`SpanLocate`] seam
            /// (per-instantiation semantics in
            /// `geom_core::spline::locate`), `door` on the first
            /// overlapped span, then for every further overlapped span
            /// `hull` of the running answer with `door` on that span —
            /// channel-independent hulls across spans for
            /// interval-natured scalars, a single call for the point
            /// scalars, whose locator names one span. One body, so the
            /// four doors below differ only in the per-span door they
            /// hand in and the per-channel hull of its answer.
            ///
            /// A poison `t` locates no span, and the walk answers
            /// `poison`: the door's own answer for a poison parameter.
            ///
            /// Empty spans (interior multiplicity) are skipped:
            /// `span_at` assigns every parameter — a repeated knot
            /// value included — to the nonempty span starting at it,
            /// which this loop's range always covers, so nothing is
            /// discarded (containment preserved); an empty span itself
            /// would only contribute poison (zero basis denominators).
            /// The emptiness check and the span's validation are the
            /// same operation.
            fn located_walk<R>(
                &self,
                t: T,
                door: impl Fn($Window<'_, T>, T) -> R,
                hull: impl Fn(R, R) -> R,
                poison: impl FnOnce() -> R,
            ) -> R {
                let Some((first, last)) = self.knots.locate(t) else {
                    return poison();
                };
                // `first` arrives already validated — the locator is
                // where span validity originates, so there is nothing to
                // re-check and no `expect` here.
                let mut acc = door(self.window_of(first), t);
                for s in (first.span().index() + 1)..=last.span().index() {
                    let Some(span) = self.knots.span(s) else { continue };
                    acc = hull(acc, door(self.window_of(span), t));
                }
                acc
            }

            /// The point the poison parameter `t` evaluates to:
            /// [`spline::poison_from`] in every coordinate.
            fn poison_point(t: T) -> $Point<T> {
                $Point::<T>::origin().map(|_| spline::poison_from(t))
            }

            /// The vector the poison parameter `t` evaluates to.
            fn poison_vector(t: T) -> $Vector<T> {
                $Vector::<T>::zero().map(|_| spline::poison_from(t))
            }

            /// The per-channel enclosure hull of two point answers.
            fn hull_point(acc: $Point<T>, q: $Point<T>) -> $Point<T> {
                $Point::new($(acc.$c.enclosure_hull(q.$c)),+)
            }

            /// The per-channel enclosure hull of two vector answers.
            fn hull_vector(acc: $Vector<T>, q: $Vector<T>) -> $Vector<T> {
                $Vector::new($(acc.$c.enclosure_hull(q.$c)),+)
            }

            /// The point at `t` — the located-span walk over the
            /// window's `eval_in_span`.
            pub fn eval(&self, t: T) -> $Point<T> {
                self.located_walk(
                    t,
                    |w, t| w.eval_in_span(t),
                    Self::hull_point,
                    || Self::poison_point(t),
                )
            }

            /// The first derivative at `t` — the located-span walk over
            /// the window's `deriv_in_span`; the `Dual` kink convention
            /// at knots is the seam's — the derivative of the program
            /// as evaluated.
            pub fn deriv(&self, t: T) -> $Vector<T> {
                self.located_walk(
                    t,
                    |w, t| w.deriv_in_span(t),
                    Self::hull_vector,
                    || Self::poison_vector(t),
                )
            }

            /// Point and first derivative at `t` from ONE span
            /// selection and one order-1 basis pass per overlapped span
            /// — the order-1 sibling of [`Self::ders`], for a consumer
            /// that wants a point and a tangent and would otherwise run
            /// [`Self::eval`] and [`Self::deriv`] as two located walks.
            /// The macro mints this door on `NurbsCurve2` too.
            ///
            /// Both halves are what their own evaluators answer, bit
            /// for bit, and the two halves rest on different grounds.
            /// The derivative IS `deriv_in_span`'s per span by
            /// construction (`deriv_in_span` is this door's derivative
            /// half, projected). The point is `eval_in_span`'s because
            /// `ders_basis_funs`'s order-0 row is `basis_funs`'s
            /// recursion (`geom_core::spline::basis`) — a second
            /// spelling of one recursion, pinned by rows rather than by
            /// construction — and `rational_corrections` at order 0 is
            /// `eval_in_span`'s division. Across overlapped spans the
            /// walk is the one every door runs, so each half's hull
            /// folds the same range in the same order as `eval` and
            /// `deriv` fold theirs.
            ///
            /// At `Dual` each half carries its own derivative channel,
            /// the derivative of the program as evaluated (the seam's
            /// kink convention at knots, as [`Self::deriv`]). At
            /// `Interval` the point box and the tangent box are hulled
            /// independently across the overlapped spans: each is its
            /// own evaluator's enclosure, and the pair is not a coupled
            /// jet (no box is a function of the other).
            ///
            /// The return is the tuple `ders1_in_span` and `ders` return
            /// — every consumer destructures it on the spot, and a named
            /// jet type would be a third spelling beside two tuples.
            pub fn ders1(&self, t: T) -> ($Point<T>, $Vector<T>) {
                self.located_walk(
                    t,
                    |w, t| w.ders1_in_span(t),
                    |(p, d1), (q, q1)| (Self::hull_point(p, q), Self::hull_vector(d1, q1)),
                    || (Self::poison_point(t), Self::poison_vector(t)),
                )
            }

            /// Point, first and second derivative at `t` — the jet a
            /// consumer that wants more than one of them computes ONCE
            /// (the located-span walk over the window's `ders_in_span`;
            /// each component hulled channel-independently across the
            /// overlapped spans, so every component is exactly what
            /// its own evaluator answers).
            pub fn ders(&self, t: T) -> ($Point<T>, $Vector<T>, $Vector<T>) {
                self.located_walk(
                    t,
                    |w, t| w.ders_in_span(t),
                    |(p, d1, d2), (q, q1, q2)| {
                        (
                            Self::hull_point(p, q),
                            Self::hull_vector(d1, q1),
                            Self::hull_vector(d2, q2),
                        )
                    },
                    || {
                        (
                            Self::poison_point(t),
                            Self::poison_vector(t),
                            Self::poison_vector(t),
                        )
                    },
                )
            }

            /// The second derivative at `t` (contract as
            /// [`Self::deriv`]): the last component of [`Self::ders`].
            /// Per span, that pass computes nothing a second derivative
            /// does not consume; per CALL the point and first-derivative
            /// hulls across the overlapped spans are surplus to it — a
            /// parameter at a knot on an interval-natured scalar hulls
            /// two spans, so this projection pays two extra
            /// `enclosure_hull`s per channel there, and nowhere else.
            pub fn deriv2(&self, t: T) -> $Vector<T> {
                self.ders(t).2
            }
        }
    };
}

nurbs_curve!(NurbsCurve2, CurveWindow2, Point2, Vec2, 2, x, y);
nurbs_curve!(NurbsCurve3, CurveWindow3, Point3, Vec3, 3, x, y, z);

/// The rational corrections through order `N − 1`, from one channel's
/// homogeneous accumulators `a[k] = A⁽ᵏ⁾` and the weight channel
/// `w[k] = w⁽ᵏ⁾` (The NURBS Book Eq. 4.8, Leibniz form):
/// `C⁽ᵏ⁾ = (A⁽ᵏ⁾ − Σᵢ₌ₖ..₁ C(k, i)·w⁽ⁱ⁾·C⁽ᵏ⁻ⁱ⁾) / w⁽⁰⁾`.
///
/// **The one spelling** of the correction every curve evaluator runs,
/// at whatever order it asked the basis for. Fixed association (D9):
/// the terms are subtracted in DESCENDING `i`, each formed as
/// `C⁽ᵏ⁻ⁱ⁾ · w⁽ⁱ⁾ · C(k, i)` in that order, and the binomial factor is
/// applied only when it is not `1` — so through order 2 this is, bit
/// for bit, `C = A⁰/w⁰`, `C′ = (A¹ − C·w¹)/w⁰`,
/// `C″ = (A² − C·w² − C′·w¹·2)/w⁰`. The binomials are `f64`
/// structure, exact for every order this is asked for.
fn rational_corrections<T: Real, const N: usize>(a: [T; N], w: [T; N]) -> [T; N] {
    let mut c = [T::zero(); N];
    for k in 0..N {
        let mut acc = a[k];
        for i in (1..=k).rev() {
            let term = c[k - i] * w[i];
            let binom = spline::pieces::binomial(k, i);
            acc = acc
                - if binom == 1.0 {
                    term
                } else {
                    term * T::from_f64(binom)
                };
        }
        c[k] = acc / w[0];
    }
    c
}

impl<T: geom_core::CertifiedBounds> NurbsCurve3<T> {
    /// The control coordinates lifted to enclosure points — the data-in
    /// shape of `geom_core::spline::compose`: channel `d`, point `i`,
    /// as `[x, y, z]` channels of certification enclosures. Pair with
    /// [`Self::knots_f64_interim`] and [`Self::weights`] to build a `CurveCertData`
    /// for composite bounds. The bracket seam this reads the net
    /// through is the shared one (`net::certified_coords`).
    pub fn certified_coords(&self) -> Vec<Vec<Interval>> {
        net::certified_coords(&self.control)
    }
}

impl NurbsCurve3<Interval> {
    /// **The curve over the parameter range `lo..hi`**, enclosed: the
    /// hull of the dehomogenized control points of each knot span's
    /// piece restricted to the range, which narrows with the range.
    /// [`Self::eval`] at a wide interval runs the basis recurrence on it
    /// instead and can enclose a span by the whole line.
    ///
    /// `None` as [`Self::span_deriv`].
    pub fn span_enclosure(&self, lo: f64, hi: f64) -> Option<Point3<Interval>> {
        let mut acc: Option<Point3<Interval>> = None;
        for (net, _) in self.restricted(lo, hi)? {
            for [x, y, z, w] in net {
                let p = Point3::new(x / w, y / w, z / w);
                acc = Some(acc.map_or(p, |a| {
                    Point3::new(
                        a.x.enclosure_hull(p.x),
                        a.y.enclosure_hull(p.y),
                        a.z.enclosure_hull(p.z),
                    )
                }));
            }
        }
        acc
    }

    /// **The first derivative over the parameter range `lo..hi`**,
    /// enclosed: `C′(t)` at every `t` in it.
    ///
    /// On each knot span the range meets, the homogeneous coordinates
    /// `(w·P, w)` and their derivative are restricted to the meet — the
    /// Bernstein coefficients of exactly that piece, the derivative's read
    /// off the blossom across the span's own knots
    /// ([`geom_core::spline::SpanRow::restrict_derivative`]) — so the
    /// hulls enclose `A`, `A′`, `w` and `w′`; `C′ = (A′·w − A·w′)/w²` follows in interval arithmetic with
    /// every restricted weight certified positive. [`Self::deriv`] at a
    /// wide interval runs the basis recurrence on it instead and can
    /// enclose a span's derivative by the whole line.
    ///
    /// `None` for a range with no extent, one past the domain, a
    /// restriction the blossom door refuses, and a weight not certified
    /// positive.
    pub fn span_deriv(&self, lo: f64, hi: f64) -> Option<Vec3<Interval>> {
        let at = |x: f64| Interval::from_bounds(x, x);
        let mut acc: Option<Vec3<Interval>> = None;
        for (net, slope) in self.restricted(lo, hi)? {
            let fold = |it: &mut dyn Iterator<Item = [Interval; 4]>| {
                it.reduce(|x, y| [0, 1, 2, 3].map(|c| x[c].enclosure_hull(y[c])))
            };
            let h = fold(&mut net.iter().copied())?;
            let hd = fold(&mut slope.iter().copied())?;
            let w = h[3];
            let (aa, ad) = (Vec3::new(h[0], h[1], h[2]), Vec3::new(hd[0], hd[1], hd[2]));
            let piece = (ad * w - aa * hd[3]) * (at(1.0) / w.powi(2));
            acc = Some(acc.map_or(piece, |x| {
                Vec3::new(
                    x.x.enclosure_hull(piece.x),
                    x.y.enclosure_hull(piece.y),
                    x.z.enclosure_hull(piece.z),
                )
            }));
        }
        acc
    }

    /// Each knot span's piece over `lo..hi`: its homogeneous Bernstein
    /// net, every weight certified positive, and its derivative's
    /// ([`geom_core::spline::SpanRow::restrict_derivative`]).
    /// `None` for a range with no extent or past the domain, a
    /// restriction the blossom door refuses, and an uncertified weight.
    #[allow(clippy::type_complexity)]
    fn restricted(
        &self,
        lo: f64,
        hi: f64,
    ) -> Option<Vec<(Vec<[Interval; 4]>, Vec<[Interval; 4]>)>> {
        use geom_core::Bounds as _;
        let (start, end) = self.domain();
        if !(lo < hi && start <= lo && hi <= end) {
            return None;
        }
        let (s0, s1) = self.knot_values().locate(Interval::from_bounds(lo, hi))?;
        let (first, last) = (s0.span().index(), s1.span().index());
        let at = |x: f64| Interval::from_bounds(x, x);
        let mut out = Vec::new();
        for index in first..=last {
            let Some(win) = self.span(index) else {
                continue;
            };
            let values = self.knot_values().values();
            // The knot ends only clip: the restriction re-admits `a` and
            // `b` to the span's own knots (`blossom`'s `admit`) and
            // refuses one outside it.
            let a = lo.max(values.get(index)?.lo());
            let b = hi.min(values.get(index + 1)?.hi());
            if a.total_cmp(&b).is_ge() {
                continue;
            }
            let p = win.degree();
            let homogeneous = |k: usize| {
                let i = win.first_control() + k;
                let (w, q) = (self.weights()[i], self.control()[i]);
                [w * q.x, w * q.y, w * q.z, w]
            };
            let mut net = vec![[Interval::zero(); 4]; p + 1];
            let mut slope = vec![[Interval::zero(); 4]; p.max(1)];
            for c in 0..4 {
                let row: Vec<Interval> = (0..=p).map(|k| homogeneous(k)[c]).collect();
                let row = win.span().with_window(&row)?;
                let restricted = row.restrict((at(a), at(b))).ok()?;
                for (slot, x) in net.iter_mut().zip(restricted) {
                    slot[c] = x;
                }
                let derived = row.restrict_derivative((at(a), at(b))).ok()?;
                for (slot, x) in slope.iter_mut().zip(derived) {
                    slot[c] = x;
                }
            }
            if !net.iter().all(|h| h[3].is_certified() && h[3].lo() > 0.0) {
                return None;
            }
            out.push((net, slope));
        }
        (!out.is_empty()).then_some(out)
    }
}

impl<T: geom_core::CertifiedBounds> NurbsCurve2<T> {
    /// [`NurbsCurve3::certified_coords`] at two channels: `[x, y]` channels
    /// of certification enclosures, through the same bracket seam and the same
    /// body.
    pub fn certified_coords(&self) -> Vec<Vec<Interval>> {
        net::certified_coords(&self.control)
    }
}

impl<T: Real> NurbsCurve3<T> {
    /// The "no description yet" placeholder payload for
    /// [`crate::curves::Curve3::Nurbs`]: a structurally valid degree-1 segment
    /// whose control points are all-poison, so every evaluation yields
    /// the all-poison point — bit-for-bit the totality behavior the
    /// former unit placeholder variant had (representable ≠ described;
    /// fails every downstream certification loudly, D4 ¶2).
    pub fn placeholder() -> Self {
        let p = net::poison_point::<T, Point3<T>>();
        Self {
            // Structurally valid by construction: clamped degree-1
            // vector, two positive weights, two control points.
            knots: Knots::lift(
                &Knots::from(KnotVector::unit_segment(NonZeroUsize::MIN)),
                T::from_f64,
            ),
            control: vec![p, p],
            weights: Weights::from_point_parts(vec![T::one(), T::one()], vec![1.0, 1.0]),
            speed: OnceLock::new(),
            speed_sup: OnceLock::new(),
        }
    }

    /// Is this payload the [`NurbsCurve3::placeholder`] — the "no
    /// description yet" state — rather than a described curve?
    ///
    /// The discriminator and the reason it is `all` and not
    /// `any` are the crate docs' totality-and-poison section;
    /// the surface and curve halves answer it identically.
    pub fn is_placeholder(&self) -> bool {
        net::is_placeholder(&self.control)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::NurbsCurve3;
    use geom_core::spline::{KnotVector, Knots};
    use geom_core::{Dual, Point3, Real, Sym};
    use test_utils::fuzz;

    /// A cubic whose net sways `±2` across its axis, at `f64`.
    fn swaying() -> NurbsCurve3<f64> {
        let control = [-2.0, 2.0, -2.0, 2.0, -2.0]
            .into_iter()
            .zip(0..)
            .map(|(x, z)| Point3::new(x, 0.0, f64::from(z)))
            .collect();
        let knots =
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 2.0, 2.0, 2.0], 3).unwrap();
        NurbsCurve3::new(knots, control, vec![1.0; 5]).unwrap()
    }

    /// **A carrier's weights are points, and their sign is certified
    /// first (C6 S2).** Held weights take a wide one, but a curve or
    /// surface reads its weights through their `f64` vector, so its door
    /// refuses one wholly above zero as `CarrierWeightNotAPoint`. A wide
    /// weight that reaches zero for some members refuses as that,
    /// `WeightSignUndecided`, with its enclosure as the margin. Point
    /// weights build. Red where the door admits a wide weight, or
    /// reports a straddling one as merely wide.
    #[test]
    fn a_carrier_door_refuses_a_weight_wider_than_a_point_after_its_sign() {
        use crate::surfaces::NurbsSurface;
        use geom_core::Interval;
        use geom_core::spline::SplineError;
        let pt = Interval::from_f64;
        let knots = |n: usize| {
            let mut k = vec![0.0, 0.0];
            k.extend((1..n - 1).map(|i| f64::from(u32::try_from(i).unwrap())));
            k.extend([f64::from(u32::try_from(n - 1).unwrap()); 2]);
            Knots::lift(
                &Knots::from(KnotVector::clamped(k, 1).unwrap()),
                Interval::from_f64,
            )
        };
        let net = |n: usize| -> Vec<Point3<Interval>> {
            (0..n)
                .map(|i| Point3::new(pt(f64::from(u32::try_from(i).unwrap())), pt(0.0), pt(0.0)))
                .collect()
        };
        let curve =
            |w: Interval| NurbsCurve3::new(knots(3), net(3), vec![pt(1.0), w, pt(1.0)]).map(|_| ());
        let surface = |w: Interval| {
            let mut ws = vec![pt(1.0); 9];
            ws[4] = w;
            NurbsSurface::new(knots(3), knots(3), net(9), ws).map(|_| ())
        };
        for (door, at, got) in [
            ("curve", 1, curve(Interval::from_bounds(0.5, 2.0))),
            ("surface", 4, surface(Interval::from_bounds(0.5, 2.0))),
        ] {
            match got {
                Err(SplineError::CarrierWeightNotAPoint { index }) => {
                    assert_eq!(index, at, "{door}")
                }
                other => panic!("{door}: a wide weight: {other:?}"),
            }
        }
        for (door, at, got) in [
            ("curve", 1, curve(Interval::from_bounds(-0.5, 2.0))),
            ("surface", 4, surface(Interval::from_bounds(-0.5, 2.0))),
        ] {
            match got {
                Err(SplineError::WeightSignUndecided { index, .. }) => {
                    assert_eq!(index, at, "{door}")
                }
                other => panic!("{door}: a straddling weight: {other:?}"),
            }
        }
        assert!(curve(pt(2.0)).is_ok(), "a point weight builds");
        assert!(surface(pt(2.0)).is_ok(), "a point weight builds");
    }

    /// **A carrier's breakpoints are points (C6 S1).** The held knots
    /// take a breakpoint wider than a point, but a curve or surface
    /// reads its knots through their `f64` vector, so each door that
    /// makes a carrier's knots refuses one typed, at its run: the
    /// curve's `new`, its `on_domain` (a breakpoint a third of the way
    /// along maps through `1/3`, which rounds outward at `Interval`;
    /// one halfway maps exactly), and the surface's `new` in
    /// either direction. The point carriers beside each build.
    #[test]
    fn a_carrier_door_refuses_a_breakpoint_wider_than_a_point() {
        use crate::surfaces::NurbsSurface;
        use core::num::NonZeroUsize;
        use geom_core::Interval;
        use geom_core::spline::SplineError;
        let m = |k: usize| NonZeroUsize::new(k).unwrap();
        let pt = Interval::from_f64;
        let knots = |mid: Interval| {
            Knots::new(vec![(pt(0.0), m(2)), (mid, m(1)), (pt(3.0), m(2))], 1).unwrap()
        };
        let wide = knots(Interval::from_bounds(0.9, 1.1));
        let point = knots(pt(1.0));
        let net = |n: usize| -> Vec<Point3<Interval>> {
            (0..n)
                .map(|i| Point3::new(pt(f64::from(u32::try_from(i).unwrap())), pt(0.0), pt(0.0)))
                .collect()
        };
        let refuses_at_1 = |r: Result<(), SplineError>, door: &str| match r {
            Err(SplineError::CarrierKnotNotAPoint { index: 1 }) => {}
            other => panic!("{door}: expected run 1's carrier refusal, got {other:?}"),
        };
        refuses_at_1(
            NurbsCurve3::new(wide.clone(), net(3), vec![pt(1.0); 3]).map(|_| ()),
            "curve new",
        );
        let curve = |k: Knots<Interval>| NurbsCurve3::new(k, net(3), vec![pt(1.0); 3]).unwrap();
        assert!(
            curve(knots(pt(1.5))).on_domain(0.0, 1.0).is_ok(),
            "an exact re-expression (a half) builds"
        );
        refuses_at_1(
            curve(point.clone()).on_domain(0.0, 1.0).map(|_| ()),
            "curve on_domain",
        );
        let surface = |u: &Knots<Interval>, v: &Knots<Interval>| {
            NurbsSurface::new(u.clone(), v.clone(), net(9), vec![pt(1.0); 9]).map(|_| ())
        };
        refuses_at_1(surface(&wide, &point), "surface new, u");
        refuses_at_1(surface(&point, &wide), "surface new, v");
        assert!(surface(&point, &point).is_ok(), "a point surface builds");
    }

    /// `c` read at the scalar `U`, every value lifted by `lift` except
    /// the weights, which are `weights(j)`.
    fn at<U: geom_core::spline::SpanLocate>(
        c: &NurbsCurve3<f64>,
        lift: impl Fn(f64) -> U,
        weights: impl Fn(usize) -> U,
    ) -> NurbsCurve3<U> {
        NurbsCurve3::new(
            Knots::lift(c.knot_values(), &lift),
            c.control().iter().map(|p| p.map(&lift)).collect(),
            (0..c.control().len()).map(weights).collect(),
        )
        .unwrap()
    }

    /// **A weight that moves with the parameter makes the curve
    /// rational**, though every weight's value is `1`: at `Dual` the
    /// meter is the rational arm's (its value channel bit for bit the
    /// `f64` rational arm on the same values), and its derivative
    /// channel reads the weight's tangent rather than the constant net.
    #[test]
    fn a_dual_weight_with_a_tangent_takes_the_rational_arm() {
        let c = swaying();
        let cd = at(&c, Dual::constant, |j| {
            Dual::new(1.0, if j == 1 { 0.5 } else { 0.0 })
        });
        let m = cd.speed_lower_bound().get();
        let rational = c.rational_speed_lower_bound();
        let integral = c.speed_lower_bound().get();
        assert!(
            rational.to_bits() != integral.to_bits(),
            "the fixture must tell the arms apart: {rational} / {integral}"
        );
        assert!(
            m.value.to_bits() == rational.to_bits(),
            "Dual meter {} against the rational arm {rational} (integral {integral})",
            m.value
        );
        assert!(
            m.deriv != 0.0,
            "the weight's tangent never reached the meter"
        );
        let constant = at(&c, Dual::constant, |_| Dual::constant(1.0));
        assert!(
            constant.speed_lower_bound().get().value.to_bits() == integral.to_bits(),
            "constant Dual weights keep the integral arm"
        );
    }

    /// **A weight that is not a literal makes the curve rational** at
    /// `Sym`, though its value is `1`: a computed weight may carry a
    /// symbol, so the meter is the rational arm's; literal weights keep
    /// the integral arm. Both bit for bit the `f64` arm on the same
    /// values.
    #[test]
    fn a_computed_sym_weight_takes_the_rational_arm() {
        let c = swaying();
        let lit = <Sym<f64> as Real>::from_f64;
        let computed = at(&c, lit, |j| {
            if j == 1 {
                lit(0.5) + lit(0.5)
            } else {
                lit(1.0)
            }
        });
        let literal = at(&c, lit, |_| lit(1.0));
        let rational = c.rational_speed_lower_bound();
        let integral = c.speed_lower_bound().get();
        let (mc, ml) = (
            computed.speed_lower_bound().get().value,
            literal.speed_lower_bound().get().value,
        );
        assert!(
            mc.to_bits() == rational.to_bits() && ml.to_bits() == integral.to_bits(),
            "computed weight {mc} (rational {rational}), literal weights {ml} (integral {integral})"
        );
    }

    /// **On a polynomial net the rational piece reading is the integral
    /// one degree-elevated** (counterexample search). With `w ≡ 1` the
    /// rational quotient's piece coefficients are the derivative's
    /// raised from degree `p − 1` to `2p` over ones: the same mean, so the same
    /// direction, and every raised coefficient a convex combination of
    /// the originals, so the rational reading is never the lower. On
    /// random integral nets, degree 2–6, it is at least the integral
    /// arm's piece reading, to `1e-9` relative.
    #[test]
    fn the_rational_piece_reading_is_the_elevated_integral_one() {
        let mut rng = fuzz::start("nurbs::arms_piece_readings");
        let cases = fuzz::scaled(60);
        for case in 0..cases {
            let p = 2 + case % 5;
            let n = p + 1 + rng.below(8);
            let control: Vec<Point3<f64>> = (0..n)
                .map(|_| {
                    Point3::new(
                        rng.range(-2.0, 2.0),
                        rng.range(-2.0, 2.0),
                        rng.range(-2.0, 2.0),
                    )
                })
                .collect();
            let mut knots = vec![0.0; p + 1];
            let mut u = 0.0;
            for _ in 0..n - p - 1 {
                u += rng.range(0.2, 1.2);
                knots.push(u);
            }
            u += rng.range(0.2, 1.2);
            knots.extend(std::iter::repeat_n(u, p + 1));
            let kv = KnotVector::clamped(knots, p).unwrap();
            let c = NurbsCurve3::<f64>::new(kv, control, vec![1.0; n]).unwrap();
            let (pieces, rational) = (c.piece_floor(false), c.piece_floor(true));
            assert!(
                rational >= pieces - 1e-9 * pieces.abs(),
                "case {case} (deg {p}, {n} pts): integral pieces {pieces}, rational pieces {rational} — {}",
                fuzz::replay()
            );
        }
    }

    /// A read inside a symbolic session leaves the memo empty: what the
    /// session minted is not kept for a later read, in or out of a
    /// session. A read outside one fills it.
    #[test]
    fn a_session_never_fills_the_speed_memo() {
        use geom_core::SymBudget;
        use geom_core::sym::with_session;
        let c = swaying().map_scalar(Sym::<f64>::from_f64);
        let budget = SymBudget {
            max_terms: 64,
            max_degree: 8,
        };
        let _ = with_session(budget, || c.speed_lower_bound());
        assert!(c.speed.get().is_none(), "a session's read filled the memo");
        let _ = c.speed_lower_bound();
        assert!(
            c.speed.get().is_some(),
            "a read outside a session left the memo empty"
        );
    }
}
