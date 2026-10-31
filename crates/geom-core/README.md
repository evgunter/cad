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
| Two coplanar circles | `src/arc/pair.rs` (`Circles`: the identity, external, reach, internal, nesting and secant margins and the radical foot and half-chord, from one `d`, `r₁ + r₂` and `r₁ − r₂`; `CirclesAt`, and `Circles2` in the plane: the foot and the chord's ends placed at the centres) |
| Knot structure | `src/spline/knots.rs` (`KnotVector`, `Span`, `InteriorKnot`), `src/spline/locate.rs` (`SpanLocate`, `SpanSet`) — the S1 clause below |
| Knot algebra | `src/spline/algebra.rs` (insertion, refinement, removal, degree elevation, the union-and-refine routine) |
| Evaluation and bounds | `src/spline/basis.rs` (basis values and derivatives), `src/spline/hull.rs` (the C2.2 sup-norm mechanism), `src/spline/blossom.rs` (the certified blossom of a span window, every level through `Certification::between`), `src/spline/compose.rs`, `src/spline/net.rs` |

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
  `KnotVector`, `NurbsCurve` or `NurbsSurface` is `Eq` — a carrier's
  data is held in a scalar that has no equality — so a by-value
  equality is not available in any case.) `Debug` prints the borrow as an **address** and never follows
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
sup_norm_bound, blossom, restrict}`, over the domain `SplineCoeffs::{domain_hull,
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
  Width proportional to the distance from the world origin, or
  inherited width multiplied at each restriction, insertion or cut a
  value goes through, is a defect: it rejects the spelling at every
  scalar. Rounding added once per step is the floor.
- **Among spellings that pass:**
  - a difference of a few ulps of the coordinates at f64 decides
    nothing;
  - prefer the spelling that is the quantity's definition;
  - break a remaining tie on Interval width at the input widths real
    producers hand it, since a width no producer makes is not evidence;
  - and on f64 error against an exact reference, never against a
    carrier, whose agreement is not accuracy.

## The recourse rules at each door

How DESIGN `[tolerance.recourse]` is applied: its text before the
rules and their application were split (2026-10-18), verbatim except for
clause (iv), which DESIGN keeps. Bracketed ids are `docs/DESIGN.md`'s
clauses.

Two roles are kernel-wide
vocabulary: **ε_precision** (this ε — certification residuals, what
gets built) and **ε_input** — the least precision a user might care
about: what counts as too-close-to-a-coincidence when interpreting
input. ε_input IS K·ε, a synonym, not a third dial; K stays the one
knob (`Tolerance.k`, env `CAD_AMBIGUITY_K`). Binding consequences:
(i) user-facing messages and recourse never fork on exactly-on vs
in-band below ε_input — ONE message, ONE recourse per **decision**
(the one question a site asks), written once in the decision's own
levers and shared by all its refused arms, with the margin riding
the payload as data; kernel semantics keep the distinction (message
policy, not predicate policy). The recourse follows from the
decision and its verdict: *make the two one construction* ([coincidence])
only where the refused side is a coincidence a shared construction
would decide; *move the geometry*, phrased as the decision's own lever, always;
*tighten the tolerance* only where the refused margin is a size the
user may intend and a smaller ε decides it — the band-decided arms
(in band, or Zero where Zero does not pass) of a decision that passes
on a nonzero sign — phrased conditionally and with the value the
margin gives ("if this size is intended, tighten the tolerance below
m/K"); never on a sign-certain arm, and never on a decision that
passes only at Zero (a residual), whose refused margin is a miss,
not a size. No refusal advises loosening ε, with one exception: a
refusal that would otherwise name no recourse at all (a kernel
approximation limit — an offset fit that stalls, a quadrature budget
spent, a built curve's residual) names loosening the tolerance as a
last resort and says the refusal may indicate a kernel bug worth
reporting. At adoption, certification runs at ε and a refusal ends at
the import door, as the table below gives. ε_in is the file's own coincidence distance ([import]): it
governs interpretation — what the file's data is evidence of, where
healing may move geometry by up to O(ε_in) — and its refusals are
the import door's, named in ε_in with the value the margin gives; a
size below ε_in is not one the file intends, so no refusal offers to
keep it. Until adoption rebuilds its caches from the file's
descriptions ([import]), a certification refusal whose miss lies within
ε_in but beyond ε (the file's data agrees to its own declared
coincidence distance, not to this run's) names setting ε to ε_in as
a stopgap, beside re-exporting the file more precisely. A change to
a document's ε reports what it flips at the change itself. The
three-arm sentence (make them one construction / move the geometry
/ tighten the tolerance) is thus the recourse of a decision whose
refused side is a coincidence a construction could decide; an
`unproven-coincidence` finding refuses nothing, and its recourse is
one construction or an assertion at its site ([assertions]). The decision is a closed type at its site (as
`CertCheck` is), so its recourse is an exhaustive match, never a
lookup by predicate name. (ii) Error variants may stay distinct as
data; their user stories converge per decision. `Indeterminate`
carries data, not a recourse: its Display renders the payload, and
the site that knows the decision appends the one ending
(`COINCIDENCE_RECOURSE` is the sentence coincidence decisions
compose). (iii) [import]'s ε_in is an instance of ε_input, not a separate
concept.

Clause (iv)'s definite readings end at each door by the decision's
refused side (`geom_brep::recourse::RefusedSide`), read by every
reading that shows no size — an exact zero, a zero the symbolic tier
proved, a reading in band wholly on the refused side, and one past the
band there (`Shows::NoSize` and `Shows::Refused`), on whichever arm
carries it; a reading that may still show a size (an offer, a floor, an
unresolved enclosure, a straddle) ends in the lever whatever the side.

| Refused side | At a build | At the import door, past ε_in | Over a body at rest |
|---|---|---|---|
| `Lever` | the lever and the note | the lever and the note | the lever and the note |
| `StoredContradiction` | the lever and the note | `DAMAGED_FILE_ENDING` | `KERNEL_OR_FILE_DEFECT_ENDING` |
| `Unminted` | the lever and the note | `DAMAGED_FILE_ENDING` | `KERNEL_OR_FILE_DEFECT_ENDING` |

One chooser names the culprit of every defect ending read through
`geom_brep::recourse`, sized or not
(`geom_brep::recourse::ReadAt::culprit`), by the door alone. At a
build it names the kernel and at rest the kernel or the file. At the
import door past ε_in it names the file: a bad file reaches every
defect ending there, whether the reading compares a value the file
stored or one adoption derives from it (an edge's carrier parameters,
a carrier the import mints, the witness at the carrier's mid-parameter,
a chart image, a fit, the adoption ladder's description candidate). A
sound file such an ending reaches is a kernel bug to fix, not a reason
for the words to hedge. An unreadable margin's note follows the same
chooser (`geom_brep::recourse::unreadable_margin_note`):
`UNREADABLE_MARGIN_NOTE` at a build, `UNREADABLE_STORED_MARGIN_NOTE` at
rest, `UNREADABLE_FILE_MARGIN_NOTE` at the import door. Defect endings
spelled as a constant at their own site (`KERNEL_DEFECT_ENDING`,
`KERNEL_OR_FILE_DEFECT_ENDING`) do not read the chooser: elsewhere in
`geom_brep` (`offset`, `offset_fit`, `patch_bound`, `props`) and in
`topo`, `sweep`, `profile`, `editor-core`, `pncad` and `viewer`, each
names the culprit its own door gives; a refusal of theirs the import
door renders takes the door instead (`topo::JoinRefusal::render`,
`topo::TransformError::render`, `topo::PcurveMintError::render`).

Within ε_in the import door ends every band-decided reading in its one
sentence and the lever, whatever the side. No public door returns a
refusal on an unminted side read at a build: every
public certification door (`geom_brep::EdgeCurve::certify`,
`EdgeCurve::recertify` and their `_via` forms) reports a span check's
reading certainly on its refused side as its input's
(`geom_brep::CertifyError::StatedInterval`, at a build in the check's
own lever): a caller's interval, or a stored arc re-certified at a band
finer than the one it was minted at, since the winding decision passes
a headroom its band reads as zero. Where that input is an edge the
kernel holds, re-minted or re-certified at the run's one band (a rigid
motion, a graft, a re-description in place, a split child, a re-based
run), the call names that origin (`topo`'s `policy_lane::Origin::Kernels`,
which every certification in `topo` names one of) and panics: a kernel
bug. Only a refusal built by hand reaches `CertCheck::ParamSpan`'s
or `CertCheck::ParamWinding`'s own ending at a build; rendering it is
input, so it ends there as the door's own refusal does, in the lever,
and the kernel-bug panic is `Origin::Kernels`'s alone. So the `Unminted`
build cell keeps clause (iv)'s panic where the kernel constructs, the
only place such a reading arises; the cell names what a hand-built
value renders. The table departs from clause (iv) in one cell, typed
until the door that would keep input from it exists, and scheduled by
its row: the at-rest column, which the validator also reads at the
import gate (`[import]` step 4) and so a damaged file reaches until
adoption checks everything the validator does
(`work/encl/at-rest-stored-contradictions-and-unminted-sides-panic-once-import-validates-everything.md`).
The same row holds the one departure from (iv)'s names-the-file
sentence: the verdicts that gate returns (`step_import`'s
`TierInvalid`) are read at rest, so at the import door they still name
the kernel or the file.

Review checklist form: for every arm added to a decision, name which
ε_input story it belongs to, or say why it belongs to none.

## Related pages

`docs/DESIGN.md` (D2's addendum on refusal design, D4 poison, D9
determinism, Q1 on the comparison-free `Real`); `crates/geom-brep/README.md`
(CURVED-DESIGN C2.2, the sup-norm certificate this layer's hulls feed);
`crates/topo/README.md` (C2's per-knot-span identity clause).

## Open

- Nothing on this page.
