---
id: the-gui-has-no-clearance-consumer
kind: issue
title: the GUI has no clearance consumer: no verdict, no witness, no refusal
status: open
opened: 2026-09-30
priority: P1
cost: H
design: true
---

## What the GUI is missing

A `min_clearance` measure has a value only at a certifying scalar. The
viewer evaluates at `f64` (`Evaluation<f64>` in
`crates/viewer/src/session.rs`), so the measure's row shows the
kernel's `MeasureUnavailableAt` sentence and nothing else. An assertion
over it reads `Unevaluated`, and nothing marks where the parts come
closest. Nothing in `crates/viewer` asks for the certified answer.

The measure's engine is `topo::clearance::min_separation`, reached
through `ClearanceLane` from inside a certifying evaluation
(`crates/editor-core/src/eval/wire.rs`). It answers a certified
bracket `[lo, hi]` of the separation, or `meets` (zero, read off
topology), or one of `MinSeparationRefusal`'s three arms. A measure's
verdict comes from an assertion over it. `editor_core::clearance`
(`clearance`, `clearance_over`) answers a different question: E7's
toleranced "separation ≥ c over a drive's certified leaves". It has no
caller outside tests, and the `pncad` façade leaves it out on purpose.
`Bracket::witness` sets `hi` from a pair of placed stations and then
discards which pair it was, so the engine has no witness to give.

## The design

- **A kernel door on `pncad`: certified readings at nominal.**
  - *Input:* the document, its resolver, the tolerance, a cancel
    token, and the nodes to read.
  - *Lane:* it evaluates at `Interval` with no parameter box, so every
    free variable is a point interval at its nominal.
  - *Output, per node, in plain numbers:*
    - a measure's bracket with its closest pair found;
    - or "no upper end";
    - or the measure's refusal;
    - or an upstream node's refusal at the certified lane;
    - for an assertion over the measure, `Holds`, `Violated`, or
      `Indeterminate` when the bracket straddles the bound.
  - The lane and the evaluated scope are the door's own: the whole
    document now, the asked nodes' upstream cone once that walk
    exists. The GUI never names a scalar.
- **Topo.** `MinSeparation` keeps the station pair that set `hi`: the
  face, chart uv and point on each side, or the shared cell when the
  scopes meet. The certified `Measure` payload carries that pair as
  E3's output, "the closest pair found". Its comment, "none of them is
  a pair of points anyone evaluated", is reworded with the change.
- **Viewer: a certify seam with the at-rest coincidence door's shape.**
  - `DocSession::land` submits it only when the landed `f64` evaluation
    holds a `MeasureUnavailable` payload.
  - It is keyed by generation, cancelled by the next submit, and
    settled into `LandedRun` as `Pending | Landed | NotOwed`.
  - A row reads it exactly when its `f64` payload is
    `MeasureUnavailable`; every other row keeps its `f64` value.
  - What the rows show:
    - pending reads "certifying…";
    - a bracket is spelled in the working notation, rounded outward
      (`[0.39996, 0.40004] m` reads `399.9 – 400.1 mm`), and becomes
      one number only when the rounded ends agree;
    - `meets` reads `0`;
    - a refusal, or an upstream node that refused at `Interval`, reads
      as that node's failure, never as a clearance value.
  - `props::Computed` gains a bracket form.
  - Selecting or hovering the measure's row draws the found pair in
    the viewport as two points and a segment, labelled with its
    distance.
- `crates/viewer/README.md`'s clause that a landing is "taken in that
  pair's single `land`" is reworded: a landing may owe answers that
  settle later, which the coincidence door already makes true.

Not part of this row:
- a tolerance panel, which belongs to every measure;
- a "tighten" control, which would re-ask at a larger cell budget;
- the faces that floor `lo`, which only that control would read.

## Seams

- The pair: topo (CLEAR, CLEARTAIL).
- The door: editor-core and `pncad`.
- The seam and the rows: DOORS, shared with VSEAM.
- The pair's mark: `marks.rs` (VGEOM).

`work/cleartail/measure-clearance-refused-states-no-recourse-and-prints-its-goldening-payload`
words the refusals these rows draw.
