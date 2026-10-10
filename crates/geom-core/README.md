# geom-core

`geom-core` is the scalar substrate: the [`Real`] ring the kernel evaluates
in and its instantiations (`f64`, `Dual`, `Interval`, `Sym`, the `Probe`
recorder), the certified decision door (`Decide`, `Band`, `Tol`), linear
algebra, and the **B-spline/NURBS structure layer** in `src/spline/`.
Nothing here knows about topology; nothing here decides anything a caller
did not ask it to.

## Where in the code

| Area | Modules |
|---|---|
| The ring and its instantiations | `src/real.rs`, `src/dual.rs`, `src/interval.rs`, `src/sym.rs`, `src/k_stats.rs` (the `Probe` recorder) |
| Decisions and tolerance | `src/predicate.rs`, `src/tolerance.rs`, `src/bit_identity.rs` |
| Refusal text | `src/readable.rs` (`Readable`, the one rendering of an `f64` a refusal message interpolates) |
| Exact arithmetic | `src/exact.rs` (`two_sum`, the error-free transform a structural door decides a real identity with) |
| Linear algebra | `src/linalg.rs`, `src/linalg/` |
| The planar arc carrier | `src/arc.rs` (`Arc2`: centre, radius and signed sweep, the one arc value `profile` and `geom-brep` share, and its one evaluation) |
| Knot structure | `src/spline/knots.rs` (`KnotVector`, `Span`, `InteriorKnot`), `src/spline/locate.rs` (`SpanLocate`, `SpanSet`) — the S1 clause below |
| Knot algebra | `src/spline/algebra.rs` (insertion, refinement, removal, degree elevation, the union-and-refine routine) |
| Evaluation and bounds | `src/spline/basis.rs` (basis values and derivatives), `src/spline/hull.rs` (the C2.2 sup-norm mechanism), `src/spline/compose.rs`, `src/spline/net.rs` |

## The spline layer's pairing rule (SPLINE-DESIGN S1)

**S1 — a proof about a structure travels with that structure.** A span
index is only meaningful for the knot vector it was located in; a control
window is only meaningful for the curve or surface whose net it selects
from. Each is therefore a **borrow, not a plain value**:

```rust
pub struct Span<'a> { kv: &'a KnotVector, index: usize, first_control: usize, degree: usize }
```

and, one level up in `geom`, `CurveWindow{2,3}<'a, T>` holds
`&'a NurbsCurve{2,3}<T>` beside its `Span<'a>`, and `SurfaceWindow<'a, T>`
holds `&'a NurbsSurface<T>` beside two of them. The consequences are the
clause:

- **A door restricted to a span takes exactly one structure, and reads
  everything from it.** In this crate that is the span:
  `basis_funs(span, t)`, `ders_basis_funs(span, t, n)` read their knots
  through `Span::knots`, and the hull doors read through a
  `CoeffWindow` (below). In `geom` it is the window: evaluation lives on
  `CurveWindow::{eval_in_span, ders_in_span, ders1_in_span,
  deriv_in_span, deriv2_in_span}` and `SurfaceWindow::{eval_in_span,
  ders_in_span, ders3_in_span}`, each reading its basis from its own
  span and its control net from the curve or surface that span was drawn
  from. A door taking `(structure, proof)` has two arguments nothing
  relates; a door taking only the proof has nothing to relate.
- **The mints are `&self`, and they are the only ones.**
  `KnotVector::{span, span_at, span_of, span_range}` for a `Span`;
  `KnotVector::with_coeffs` for a `SplineCoeffs` and
  `KnotVector::with_rational_coeffs` for a `RationalCoeffs`, each
  pair's `{span, span_at}` for its window;
  `NurbsCurve::{span, span_at}` and `NurbsSurface::{window, window_at}`
  for a window. So a window names the curve or surface that minted it,
  and that is the one it answers for.
- **No pairing guard, and no poison route for one.** The state a guard
  would test is not representable, so no door checks that a span,
  window or coefficient array belongs to the structure beside it, none
  has a refusal for a mismatch, and D9's "the kernel never panics on
  any input" holds for the pairing by construction rather than by
  check. There is no `admits` predicate on
  `KnotVector` or on `NurbsSurface`.
- **Equality on all three types is address equality on the borrow**,
  plus the indices. A proof is about *that* structure; two bit-equal
  knot vectors at different addresses are two structures. (None of
  `KnotVector`, `NurbsCurve` or `NurbsSurface` is `Eq` — their knots and
  weights are `f64` — so a by-value equality is not available in any
  case.) `Debug` prints the borrow as an **address** and never follows
  it: the alternative dumps a whole control net at every `{:?}`.
- **A borrow cannot be held across a rebinding of what it borrows.**
  That costs nothing here: every knot-algebra door in this crate and in
  `geom` is `&self -> Self`, so a refinement is a new value and a proof
  about the original goes on naming the original. Compile-fail doctests
  pin it — on `Span` (an escaped borrow, a rebinding, and a second knot
  vector that has no parameter to arrive through) and on
  `geom::curves::nurbs` (a span of one curve against another curve, and
  a window outliving its curve), each with a legal twin.

**Coefficients against knots take the same shape, one level down.** A
coefficient array — whatever a fitting or composition pass produced,
`f64` or `Interval` brackets — is a proof about the knot
vector it was fitted against, so `hull`'s doors read it through a pair
that borrows both, and the pair is one of **two types** according to
the claim it licenses:

```rust
pub struct SplineCoeffs<'a, E: CertifiedEnclosure>   { knots: &'a KnotVector, coeffs: &'a [E] }
pub struct RationalCoeffs<'a, E: CertifiedEnclosure> { knots: &'a KnotVector, coeffs: &'a [E], weights: &'a [f64] }
```

minted only by `KnotVector::with_coeffs` and
`KnotVector::with_rational_coeffs`, where the count relation
`coeffs.len() == control_count()` (and the weights' count) is checked
**once** and a wrong length is `None` — the one relation a length can
state, and the bound that keeps every window inside the array. A span of
that vector is taken FROM the pair: each type's `{span, span_at}` mint
its window (`CoeffWindow<'a, E>`, `RationalWindow<'a, E>`) holding the
pair beside a `Span<'a>` of its own vector, and every door reads
everything from the borrow. `SplineCoeffs` carries the **nonrational**
doors and no other — per span `CoeffWindow::{hull, derivative_hull,
sup_norm_bound}`, over the domain `SplineCoeffs::{domain_hull,
derivative_coeffs, derivative_domain_hull, sup_norm_bound}`;
`RationalCoeffs` carries the **rational** doors and no other — per span
`RationalWindow::hull_rational`, over the domain
`RationalCoeffs::{domain_hull_rational, sup_norm_bound_rational}`. So a
rational claim on a pair minted without weights, and a nonrational bound
on a pair minted with them (one that would ignore the weights it was
handed), are unrepresentable rather than refused — D2 addendum row 0
in both directions — as is a span of another vector beside the pair;
each is a `compile_fail` doctest with a legal twin on the type it
concerns. No free function in `hull` takes a coefficient array; the one
door beside the mints that takes one, `KnotVector::difference_coeffs`
(the knot-differencing step or a one-element poison vector), mints
first; it is also the line step a tensor pair differences with, so a
line of the wrong count refuses whole. An array that is a consumer's
own construction is minted by construction instead —
`KnotVector::with_coeffs_from_fn` builds it at the vector's count — and
a derivative that is differenced again travels as a `SplineCoeffsBuf`,
the owned pair `SplineCoeffs::derivative` answers with the derived
vector. `TensorNet` stays the unpaired tensor storage; a net held
beside the clamped vectors it is a proof about is a `TensorCoeffs`
(one channel, its partial nets as doors) or a `TensorChannels` (the
channels of one surface over one pair of vectors, refined only by
chains it builds from those vectors). The `compile_fail` rows on
`TensorCoeffs` pin two retired spellings — a grid beside two vectors
at `PatchSpans::decompose`, a struct literal of another extent — and
not the shape: a new door taking a net beside two vectors would
compile, and only a sweep finds it. Weight positivity stays a per-window check at the rational
door: it is a *value* precondition of the claim on exactly the weights a
window reads, where the count is a *pairing* fact and the mint's
business. The count relation at `NurbsCurve::new` and
`NurbsSurface::new` is the same relation one level up, checked once at
construction; a curve's `certified_coords()` channels mint against its own
`knots()` by that fact.

The family is closed, with one deliberate exception: `InteriorKnot` — a
value proved interior to one vector's domain, carried without that
vector — stays crate-private for it, the type being a guard only in
combination with the privacy of its two consumers, argued at its doc.

## Choosing among spellings of one quantity (W1)

**W1 — a spelling is chosen by what its width and its error scale
with.** When two spellings of one value disagree at some scalar (a
point turned about an axis, a point on an arc, a restricted range of a
description's parameter):

- **Scale.** Width or error proportional to the geometry's own scale (a
  radius, a chord, the split parameter's own rounding) is the floor.
  Width proportional to the distance from the world origin, or to the
  number of restrictions a value has been through, is a defect: it
  rejects the spelling at every scalar.
- **Among spellings that pass:**
  - a difference of a few ulps of the coordinates at f64 decides
    nothing;
  - prefer the spelling that is the quantity's definition;
  - break a remaining tie on Interval width at the input widths real
    producers hand it, since a width no producer makes is not evidence;
  - and on f64 error against an exact reference, never against a
    carrier, whose agreement is not accuracy.

## Related pages

`docs/DESIGN.md` (D2's addendum on refusal design, D4 poison, D9
determinism, Q1 on the comparison-free `Real`); `crates/geom-brep/README.md`
(CURVED-DESIGN C2.2, the sup-norm certificate this layer's hulls feed);
`crates/topo/README.md` (C2's per-knot-span identity clause).

## Open

- Nothing on this page.
