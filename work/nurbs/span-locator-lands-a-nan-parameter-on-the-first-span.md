---
id: span-locator-lands-a-nan-parameter-on-the-first-span
kind: issue
title: KnotVector::span_at/span_range land a NaN parameter on the first span by documented tie-break, so every window reader that does not check first reads the first span's hull as certified
status: closed
opened: 2026-10-01
priority: P3
cost: H
design: true
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
