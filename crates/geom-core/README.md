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
| Refusal text | `src/told.rs` (`Told`, the told quantity a refusal's payload holds and a sentence quotes, and `Compared`, two of them told in their order: "A number told to a person" below), `src/readable.rs` (`Readable`, an `f64` spelled exactly, for text that must read back) |
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

## Reading a bracket where nothing certifies (DUAL-DESIGN DL5)

A *bracket read* is `Bounds::lo`/`hi`: it turns a scalar `T` into an
`f64`, and so drops whatever else the scalar carries, a dual's tangent
or `Sym`'s symbols. A *lane door* is a bound that keeps such a scalar
out (`CertifiedBounds`). *Re-entry* is a computed `f64` turned back
into a `T`. Three clauses say when each is right, and every other
statement of them (`real.rs`, `dual.rs`, the bounds census) cites
these and restates neither.

**DL5.1 — the read.** At a dual a bracket read returns the base
scalar's bracket, bit for bit. A read that is legitimate at the base
scalar therefore needs no lane door, provided the `f64` it yields
never re-enters `T`. That `f64` ends one of two ways:

- it **leaves**: an error field, a box, a report;
- it is **only compared**: a branch, an index, a flag, the pick of
  which candidate is built or reported.

Neither owes a tangent. The branch taken carries `T` values computed
at `T`, with their own tangents, exactly as `min` returns its winner's.
A read that mints a certificate object or feeds a `CertifiedEnclosure`
consumer is never exempt: it takes a lane door. Whether a bracket may
decide anything at all is not this clause's question; that is the
`Bounds` scope rule's ("brackets never decide", `real.rs`), at every
scalar.

**DL5.2 — re-entry.** A runtime `f64` does not become a free `T`
because a `T` was the only spelling to hand. Structure is still found
at `f64` (a search, a subdivision, a comparison), and what that hands
back is sorted by what the `f64` is:

| The `f64` is | What the code does with it |
|---|---|
| a bound to decide against: a floor, a rounding pad, a lever arm, a hull bound | decides at `f64`, or carries it as an `f64` allowance term of the margin. It never becomes a `T` |
| a place the code chose to evaluate at: a sample, a station, a cell corner, the point a cell is subdivided at | hands it to the evaluator as an `f64` argument, and gets the point back on the value channel (`T::Value`: the scalar with its tangent removed, `f64` at `Dual<f64>`, `Interval` at `Dual<Interval>`, the scalar itself elsewhere). A place is never a `T`, so it has no tangent; what is evaluated there can be decided on, and is not a `T` a body can hold |
| data the model determines, built whole at `f64`: a wall skinned through its sections, a profile's arcs | builds it at `T`. The construction is the data's definition, so there is nothing to check a lifted copy against |
| a constant of the model | enters by where it comes from: a literal through a door that accepts only a compile-time constant, a count as an integer, a band as the tolerance type, an authored value through the document's parameter door |

A proposed value the result is built on is admitted by its check. A
value is proposed when the model does not determine it: an `f64`
routine picks one of many that would serve, and there is a description
to check it against other than itself. A root, a foot or a crossing
with no closed form is one number proposed; a curve fitted through
measured feet (an edge's image in a spline wall's chart) is a net of
them. A proposal is `f64` by type until one door admits it: the
candidate and its check go in, the door enters the candidate in `T` and
runs the check there (for a number, that its point reads on within the
band; for a fitted image, the certificate that the wall composed with
it stays within the band of the carrier), and the admitted `T` with
what the check produced, or the refusal, comes out. A caller cannot
have the `T` without its check, so what the value claims is what was
checked: this number reads on, this curve serves, never that it is the
true root or the true image. Where the value has a closed form it is
not proposed: it stays in `T`. At `Sym` an admitted value is an opaque
unknown and claims nothing. The admitting doors are bounded `Certifies`
(DL5.3), so a dual cannot reach them, and with the evaluators' private
step for a place they are the only callers of `Real::from_computed`.

Nothing is decided from an admitted proposal beyond what its check
bounded. Another candidate the same check would admit could have been
drawn, so a reader that reaches a decision (a point at an end or clear
of it, inside a trim or on its edge, two roots apart or one) gives the
answer every admissible candidate gives: it asks its question of the
model's own description, the proposal only saying where to look, or it
carries the check's bound as an allowance, and where it can do neither
it refuses as unresolved at the band. The door therefore returns the
admitted value with that bound: how far another admissible candidate
could lie from it, in each direction a reader decides on (for a fitted
image, the certificate's envelope; for a crossing parameter, the reach
along the edge as well as the distance across the face). A check that
bounds less than its readers decide on is widened, or the reader
re-posed. The check is the one decision a proposal may change, and its
refusal says which it is: no candidate passes, which a person can act
on, or the one drawn did not, which is the kernel's own frontier.

The door is for a value the body stores: a parameter an edge is split
at, an end an edge is re-anchored to. A solved value that is only
evaluated at, to decide (a normal read at a foot, an iterate of a
search), is a place, and takes the table's row. At an enclosure scalar
the admitted value is one number, or one net, for the whole family: a
parameter or an image the construction states, the same for every
member, with every member's point there, or every member's wall
composed with it, checked within the band. It is not an enclosure of each
member's own root, which lies elsewhere by up to the band over the
sine of the crossing angle; and where the root moves farther than that
across the box, no one number passes and the site refuses. A parameter
solved in closed form is an enclosure of each member's own, so a
stored parameter reads one way on an analytic carrier and the other
on a spline.

**DL5.3 — where a dual is admitted.** A dual belongs where the build
and its measurements run, and passes through decisions on its value
alone (DL1, DL3). Code that certifies is closed to it by type, on its
interior as well as at its doors:

- `Certifies` is the bound that says a scalar certifies and nothing
  else. It grants no bracket read, so it is free to write: no list,
  no ratification. Only a bracket read is gated. A dual does not
  implement it.
- Every lane type is declared over `T: Certifies`, so every function
  that names a lane carries the bound.
- A scalar's lanes are an associated type with no values at a dual,
  never an `Option` a dual answers `None` to.
- In a file listed as certification code, a generic item without
  `Certifies` fails the gate.
- A bound says which scalars the code has a meaning for. It is not
  widened to every scalar the body happens to compile at, and no work
  is done to support a scalar the code has no meaning for.

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
keep it. A certification refusal whose miss lies within ε_in but
beyond ε names re-exporting the file more precisely and offers no
tolerance: adoption builds every cache at ε from what the file is read
to describe at ε_in ([import]), so ε_in is never a setting to certify
at instead. A change to
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
a chart image, a fit, the description the build chose). A
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

### A number told to a person

**A number a sentence quotes is a told quantity, and the told quantity
says it.** A refusal, a recourse or a readout never formats a scalar
itself. The payload field that carries the number is one opaque type
in `geom-core`, built where the payload is built, while the scalar is
in hand. It holds the *reading* (a value, an enclosure, or unreadable),
the *dimension* (length, angle, dimensionless) and the *role*, and it
renders the number and its unit word. It offers no arithmetic, no
ordering and no way to read a computed number back, so nothing decides
on a payload (DESIGN [verdicts]). A refusal type declares no bare `f64` or `T`
field.

The role is where the number came from, and it fixes how the number is
spelled:

| Role | What the person reads |
|---|---|
| **Given**: the caller's or the file's own number | exactly what was received: the shortest digits that read back |
| **Computed** length | the shortest spelling within the finer of ε/10 and four significant figures, and never finer than the spacing of adjacent floats at that value |
| **Computed** angle or ratio | four significant figures; ε is a length and caps neither (DESIGN [tolerance.epsilon]) |
| **Compared**: two numbers whose order the sentence states, or a reading beside its own threshold | each to the first precision at which they read in the stated order |
| **Offered**: a bound the person is asked to type back | the shortest spelling on the computed grid that lies on the side that works when typed |

An enclosure reads `[lo, hi]` with each end rounded outward, a scalar
that is not an `f64` reads as its value or its enclosure, and a zero of
either sign reads `0`. Text that must be exact (an editable field, a
serialized form a content key hashes, a dump) is not told text and is
spelled exactly. An editable field over a written value shows that
value's exact spelling in the unit it was written in, positionally and
never in exponent form, because it is edited one character at a time;
a driven value is a readout and is told text.

Two told quantities are equal when their role, dimension and reading
are equal, and a dump shows the reading exactly. A test that means "the
person reads this" compares the telling in a named notation.

**A refusal is said in the notation of whoever reads it.** A sentence
is written through a teller that carries the notation, and a told
quantity has no `Display` of its own, so a number cannot be said
without its notation; a refusal's `Display` is its telling in metres
and radians. The GUI passes the working notation, so a person working
in millimetres reads a refusal in millimetres beside fields that show
millimetres.

**A unit says every quantity.** The number written in a unit is a
decimal, and it reads back as the exact decimal times the unit's exact
factor (1/1000 for mm, 254/10000 for in, π/180 for deg, with true π),
rounded once. That is the reading of every door that takes a number in
a unit: the text door, and the Rust and Python doors, which read a
float by its shortest decimal. So the unit a number is told in depends
only on the reader's notation and the number's dimension, never on its
value. The tolerance is the one quantity told in metres whatever the
notation: it is set in metres, and a recourse that names it says it in
metres.

## Float rounding

DESIGN `[simple.conditioned]`: every computation is posed well-conditioned
where its result is decided, and at `f64` is then computed as if exact.
This section is where that rule's parts live.

- **The run's admission rule** (`Tolerance`). The *admission floor*
  (`ADMISSION_FLOOR`, `SHORT_CHAIN_ULPS · u · box`) is the rounding of a
  short, division-free chain of operations on in-box coordinates: a
  fixed count of unit roundoffs times the session box. The *headroom* is
  `(K − 1)·ε`, the room between "the same point" (within ε) and
  "definitely apart" (beyond Kε). A run is admitted only where ε is at
  least ε_min(K), the larger of the admission floor over K's headroom,
  `ADMISSION_FLOOR / (K − 1)`, and a coordinate's spacing at the box
  edge, `u · box`. The check is a comparison against a constant, so the
  least admissible ε follows from K, and K < 2 is allowed at the price
  of a larger least ε. This is the one
  statement about rounding, and no site charges it.
- **Re-posing, never charging.** A quantity a formula would divide by
  that can vanish (a sine, a radius, a determinant) is a pose decided
  first (coaxial, parallel, tangent), and the formula divides only on
  the pose's definite arm. Where what is decided is singular at that
  pose and no re-posing removes it (the least turn's pole), the pose's
  band also reaches as far as the quotient's lever passes the run's
  *rounding reach*, `R = box · (K − 1)·ε / ADMISSION_FLOOR`: the
  admission rule read as a length, the longest lever over which a short
  chain's rounding still fits the headroom (a run is admitted exactly
  where `R` is at least the box). `R` is read through one `Tolerance`
  door, never a site's own constant. A read made before the extent it is
consumed over exists (a trace read through a face chart's turn before
the profile's loops resolve) is a selection: its pose is decided over the
session box, and its conditioning by the carriage that consumes it. A formula that loses digits where its
  result is decided (a squared residual over a small `2r`) is re-posed
  (as a distance). A stored form that loses digits is re-minted in a
  well-conditioned form: a conic is one of a family of placements
  behind the one `Conic` reading, held about its centre or about a
  point of the arc it serves with its parameter zero there; its
  construction fixes which, the same at every size, and a construction
  whose centre runs off at a pose it can reach holds the arc. A conic
  is the curve its stored numbers state, and no reading replaces it
  with a rounded copy: one held at its centre is read from its centre,
  and so fixes a point of its arc only to the rounding of the centre's
  distance and directions. A conic's position is the point it stores
  plus two vectors times two functions of its parameter, and every
  reading of where the conic is (at a parameter, along it, or of a
  point against it) is read from that position. Its centre is a datum
  of the whole conic, stored or derived. No
  `Margin` door takes hand-written slack. The backstop is a
  tree gate on bare `f64::EPSILON` and on every use of
  `running`'s charge doors (its `pub` items, and every call of a
  `with_…` method that adds a charge to a `Rounded`'s bound) outside
  this crate. Every
  site is a listed entry with its reason: in production, a formula or a
  stored form owed a re-posing, an error-free transformation, or
  test-support code; in a test, its oracle. An entry whose site has gone
  must be dropped. A count
  raised beside a new site is held only by review, not against the
  merge base
  (`work/ciw/the-bare-rounding-list-rises-in-the-diff-that-adds-its-site.md`).
- **A verdict's lever arm** (`Lever<T>`, the arm of
  `Margin::levered`). It is the length at which a tilt (a sine, a
  cosine, an angle between directions) displaces the geometry it
  governs, and a reading is levered only where that displacement is
  what is held against the tolerance. It is built only from its
  sources: a read of the
  consumed extent, a curvature arm `1/κ_rel`, or the session box where
  no geometry is in hand. Rates are not levers: they go through
  `levered_inv` and the metric doors. A length that is the reading
  itself (a residual, an offset, a stand-off) is the margin, and a
  tolerance is the band's: neither is an arm. A dimensionless factor
  belongs to the reading, not the arm. The point-level reads (two
  points' distance, a point set's reach from a pivot) live beside
  `Lever` in `geom-core`; covers and `Reach` stay in `geom_brep`. A
  lever is never shorter than the region its verdict governs and no
  longer than the site's own measure of it: a verdict that claims a
  definite sign reads the extent from below, one that acts on its Zero
  reads it from above, and one that acts on every sign reads both and
  acts only where they agree.
- **The consumed extent** (`geom_brep::Reach`): the region a verdict
  is used over. It is one type, built only from the geometry consumed
  (an edge over its span, a face's region, two faces' meet, a bounded
  carrier, each read as its cover), never from a length a caller
  vouches for. `ExtentBall` is its ball payload. A reach holds its
  region as a cover and no carrier. Where a verdict reads its datum is
  derived from the extent, never chosen: a datum fixed by a carrier's
  form (an apex, a centre) is read there, and a datum free along an
  axis is read at the middle of the extent's window along it. Its
  lever is the extent's far distance from that point; the least over
  all points is not owed. That point is the origin of a tilt bound,
  not a claim about the entity: any point is sound provided the datum
  and its lever are read at the same one, so a record that verdicts
  read from different points holds its region, not a lever.
- **The session box** is one predicate over the region a piece of
  geometry occupies, never a carrier's stored terms. The region is read
  from outside as its cover, and from inside off points known to lie on
  it. The box bounds each coordinate (`[tolerance.units]`), so it is a
  cube. A region is in the box only where its cover is. One with a point
  past it is refused as past the box. One whose cover alone reaches past
  it is refused as a limit. The door at which a region comes to exist
  asks it, before anything is decided on that region: certification asks
  it of an edge with its ends, the finished-body gate of a body's faces,
  and the door that stands a body by a map of that body where it stands.
- **The Interval lane** makes an ill-posed formula loud (its
  enclosure lands in the band, which is terminal); it does not certify
  around it.

## Related pages

`docs/DESIGN.md` (D2's addendum on refusal design, D4 poison, D9
determinism, Q1 on the comparison-free `Real`); `crates/geom-brep/README.md`
(CURVED-DESIGN C2.2, the sup-norm certificate this layer's hulls feed);
`crates/topo/README.md` (C2's per-knot-span identity clause).

## Open

- Nothing on this page.
