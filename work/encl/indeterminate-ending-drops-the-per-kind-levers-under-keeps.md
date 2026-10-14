---
id: indeterminate-ending-drops-the-per-kind-levers-under-keeps
kind: issue
title: geom-core: Indeterminate::ending drops the per-kind levers under() adds (subdivide on a straddle, check inputs on an unreadable margin), so whole-escalation renders cannot take the one undecided shape
status: closed
closed: 2026-10-11
branch: encl/indeterminate-ending-levers
priority: P3
cost: M
opened: 2026-10-11
pr: 4629
---

(Found by the PR 4612 fix pass, the geom-brep slice of `too-close-to-call-remainder`.)

## What

`UndecidedRefusal` (`geom-core/src/predicate.rs`, around 2014-2027) is documented as the one spelling of `X is undecided: {payload}. {ending}` for every door's Display. A Display that renders a whole escalation cannot take that shape yet:

- **The gap:** `Indeterminate::ending(levers)` drops two levers that `IndeterminateUnder` (`under()`) adds per margin kind.
  - On a straddling enclosure: "subdivide the parameter box for a tighter enclosure, or …".
  - On an unreadable margin: "check the operation's inputs upstream, then …".
- **Where it bites:** the geom-brep sites `newell.rs` (`NewellError::Escalated`), `nurbs_iso.rs` (`Escalated`) and `offset.rs` (`Escalated`) keep `X is undecided: {payload} — {advice}; {ending}`. The test `nurbs_iso` `an_escalated_row_states_its_decision_and_no_declaration` pins those levers at all three margin kinds. Most of the ~40 remaining sites in topo, sweep, profile and editor-core are whole renders too. The precedent `sweep/src/swept.rs` (the Bend arm) uses `ending()`, so it already loses them.
- **Also waiting on this:** newell's callers all take no declaration (extrude side planes, and `swept::cap_plane` for extrude, loft and partial revolve), so it should end in `NO_DECLARATION_RECOURSE`. That closes a bullet of `work/issues/unowned-viewer-refusals-short-of-the-shape-guard.md`, but only once the shape exists.

## Repair shape

Give `Indeterminate` one ending that keeps the per-kind levers (for example, make `ending(levers)` carry them, so `under()` becomes `undecided` plus that ending). Then route the three geom-brep arms, and swept's Bend arm, through `undecided(subject, ending(..))`, with newell on `NO_DECLARATION_RECOURSE`. This unblocks the whole-render sites in `too-close-to-call-remainder`'s later slices.
