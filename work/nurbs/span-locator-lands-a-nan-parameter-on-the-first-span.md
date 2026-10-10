---
id: span-locator-lands-a-nan-parameter-on-the-first-span
kind: issue
title: KnotVector::span_at/span_range land a NaN parameter on the first span by documented tie-break, so every window reader that does not check first reads the first span's hull as certified
status: closed
opened: 2026-10-01
priority: P3
cost: H
closed: 2026-10-09
pr: 4442
---


Filed by the SSI chart-tube lane (PR #3683), as the root of a class
whose instances it closed or filed elsewhere.

## The finding

`crates/geom-core/src/spline/knots.rs`: `KnotVector::find_span`, and
`span_at`/`span_range` on top of it, are total on all of `f64`, and
their docs make that a contract: an out-of-domain parameter clamps to an
end span, and **NaN lands on the first span**. Every window that reads
spans through them therefore needs its own check that its ends are
numbers and are ordered. A reader without that check returns the first
span's hull, certified, for a window nobody asked about. That is a
strict subset of the true region whenever there is more than one span.

## Instances

- `crates/geom-brep/src/ssi/enclose.rs`, `NurbsBoxes::cells` (and
  `rect_box`, which clamps before it locates): a NaN tube pad became a
  NaN window, and limb 3 certified over the first span. This is closed
  in PR #3683 by a per-reader door (`ordered_window`).
- `crates/geom-brep/src/props/quad.rs`, the `Collapse::Over` readers
  (`range_hull`, `raw_range_hull` through the local `raw_span`, which
  has the same tie-break, and `Dir::const_index`). These are open on
  PROPS' slate as `work/props/props-collapse-over-lands-a-nan-window-on-the-first-span.md`.
- `crates/geom-core/src/interval.rs`, `Interval::locate_spans`: NaN brackets
  land on the first span here too. Its doc argues the poisoned `t`
  then propagates through the evaluation, so this one is safe, but only
  because of what follows the locate.

## Proposal

Close the class at its source: have the locator refuse a NaN parameter
rather than tie-break it, for example a `span_at` returning `Option<Span>`
(or a `span_range` returning `None` for a NaN or inverted pair). Then a
window reader cannot forget the check, because there would be no span
to read. Clamping out-of-domain finite parameters to an end span is a
separate rule, and every caller here relies on it, so the proposal
leaves it alone.

This is not small. `span_at` is the totality the `Span` validity argument
leans on ("locating is where span validity originates, `span_at` is
total"), and every caller would gain a refusal arm. The per-reader doors
already landed or filed are the local fixes. This row is the decision
whether to make them unnecessary.

## Priced (2026-10-09)

P3: latent unsoundness (the known instances are closed or filed, so no
live wrong answer is reachable through this row). Cost H with `design`
set: whether the locator refuses NaN, and in what shape, is a fork with
several viable answers that every span reader inherits, so it is
weighed by a designer pair before a lane builds it. (NURBS orchestrator)

## Decided (2026-10-09): the designer pair converged

One Opus and one Fable designer weighed the question
(`docs/prompts/designer.md`). They were reconciled over two rounds,
with each shown the other's report, and converged on a single final
state. Neither proposes changing a ratified DESIGN.md decision. The
texts the decision changes are code documentation an agent lane wrote
(`43c940f1c4`, "fold the span guards into Span"). No Ev ruling for the
NaN route exists. So this is not a fork that waits on Ev: the
orchestrator takes the converged answer and reports it to Ev.

**The premise, corrected.**
- The defect is one level above the locator's tie-break. A parameter
  REGION has no type, so a window travels as two bare `f64`s, and a
  refused bracket travels as `(NaN, NaN)` by convention
  (`props/quad.rs` `lo_or_refuse`/`hi_or_refuse`).
- The tie-break also lets poison choose structure. DESIGN.md D4's
  taxonomy says poison flows through values, never through decisions;
  row 0 says a state that can be made unrepresentable gets a type.
- `Interval::locate_spans` is NOT safe. On a `Net` carrier,
  `pcurve_cache/projected.rs` `chart_box`/`sweep_box`/`overlapped`
  and `net_incidence` read structure chosen by a poisoned locate,
  because `frame_box_on`'s `Net` arm builds each box from knot values
  alone.
- The public point-hull doors `SplineCoeffs::span_at(t).hull()` and
  `RationalCoeffs::span_at(t).hull_rational()` (`spline/hull.rs`) read
  the span as their whole output, so "a point locate re-uses `t`" is a
  convention, not a guarantee.

**The final state to build.**
1. `KnotVector::span_at(t) -> Option<Span>`.
   - It returns `None` exactly when `t` is NaN.
   - Finite and ±∞ out-of-domain values still clamp to an end span.
   - The coefficient-pair `span_at` doors follow, returning
     `Option<CoeffWindow>`; or they are deleted, since only doctests
     call them.
   - `find_span`, whose one non-test caller is in `spline/algebra.rs`,
     folds in.
2. A `ParamRange` in `geom_core::spline` is a closed `[lo, hi]` with
   `lo ≤ hi`, no NaN and private fields.
   - Constructors: `ParamRange::new(lo, hi) -> Option<_>` and
     `ParamRange::certified(Interval) -> Option<_>`, which goes through
     `CertifiedEnclosure`.
   - Total methods: `clamp_to(domain)`, `lo`, `hi`, `mid`.
   - The invariant is a BRACKET. Certification is a property of how a
     range was minted, not of the type.
3. `KnotVector::span_range(ParamRange) -> (Span, Span)`, total.
   - Every window reader takes a `ParamRange`: props
     `Collapse::Over`, which carries `range_hull`, `raw_range_hull` and
     `Dir::Const`'s window arm, and ssi `NurbsBoxes::cells`/`rect_box`.
   - ssi's `ordered_window` becomes the constructor.
   - Props' window producers mint through `ParamRange::certified`;
     they already refuse uncertified brackets.
   - Clamping happens on an already-ordered range.
4. `SpanLocate::locate_spans -> Option<SpanSet>`.
   - It returns `None` for a poison scalar: a NaN `f64`, an `Interval`
     that is NaI or empty, and the `Dual`/`Sym`/`Probe` wrappers by
     delegation.
   - `Interval`'s impl reads the BRACKET
     (`ParamRange::new(lo(self), hi(self))`), not the certificate.
   - A `Trv` enclosure, which is sound but uncertified, still locates.
     Its decoration rides through to the reader that certifies, which
     keeps DESIGN.md's row-1 distinction between a curable Invalid and
     a terminal NaI.
5. Every caller's `None` arm returns the answer its type already gives
   for poison:
   - an evaluator returns its NaN/NaI point;
   - a hull reader returns `Interval::refused()`;
   - a `Result`/`Option` door returns its refusal.
6. `Dir::Const`'s point arm (`Collapse::At(t)` → `const_index`), which
   today returns `coeffs[0]` for a NaN `t`, refuses a non-finite `t`
   the way the `Kv` and `Raw` arms beside it already do.

The geom-core README's pairing clause, "these doors are total on their
inputs", is re-worded to say what it is about, the absence of a pairing
guard. The decision it records does not change. The tests that pin
`find_span(NaN) == 2` are re-baselined.

This supersedes FLUX's
`props-collapse-over-lands-a-nan-window-on-the-first-span`: its sites
take a `ParamRange`, so the unit that builds this closes or re-points
that row. Off-question finding, filed:
`work/flux/a-loop-area-nurbs-segment-integrates-an-inverted-window-as-zero.md`.
(NURBS orchestrator)

## Closed (2026-10-09, PR 4442)

Built as decided. The parts:
- `KnotVector::span_at` returns `Option<Span>`, `None` exactly at NaN. `find_span` folded in.
- `ParamRange` (`geom_core::spline::range`) is a bracket with `new`, `spanning` and `certified` constructors and `start`/`end`/`mid`/`clamp_to`.
- `Param` is a value that cannot be NaN, and `last_at_or_below` is the one knot search that the knot vector, `quad.rs`'s raw and constant locators, and `compose/tensor.rs`'s `cells_touched` share.
- `span_range(ParamRange)` is total, and `locate_spans` returns `Option` and reads the bracket, so a Trv interval still locates.
- Every window reader takes a range: props `Collapse::Over`, ssi `NurbsBoxes` (all of it, through `UvWindow`), and pcurve `piece_controls`/`chart_box`/`sweep_box`/`overlapped`/`net_incidence`.
- Poison arms seed from the parameter (`poison_from`), and one row per arm goes red under a `from_f64(NaN)` mutant.
- There is one UV-rectangle refusal (`PatchRect::new`).

Review: a DUAL concurrent pair; both returned APPROVE-WITH-FIXES with no MAJOR (DR row in `docs/DUAL-REVIEW-LOG.md`). The fix pass took the union of both reviews. One item's mutant is equivalent: a producer box is a hull, which `Certification::hull` leaves either certified or NaI, never Trv. That is pinned behaviourally.

Rows filed:
- `work/flux/props-collapse-over-lands-a-nan-window-on-the-first-span` (closed: superseded by this row);
- `work/flux/piece-monotone-span-drops-a-refused-step-through-f64-min`;
- `work/chord/nurbs-cell-bounds-cert-drops-a-nan-vertex-and-lands-a-nan-end-on-the-first-cell`;
- `work/ssiedge/chart-sweep-cells-carry-f64-pairs-and-mint-a-window-per-reading` (the SSI sweeps' own cell type still holds `f64` pairs).
