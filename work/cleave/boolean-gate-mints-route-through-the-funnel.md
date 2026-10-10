---
id: boolean-gate-mints-route-through-the-funnel
kind: unit
title: boolean, census and merge gates build their decided-sign escalations by hand, off the log (mints C10-C12, C19, C23)
status: open
opened: 2026-10-10
priority: P1
cost: M
parent: topo-mints-indeterminates-outside-the-funnel
refs: [a-gate-rejection-of-a-decided-enclosure-bisects-to-budget]
---

Unit 4 of `topo-mints-indeterminates-outside-the-funnel`'s re-scope
(2026-10-10, main `98a3817a1d`).

## Sites

- **C10, `boolean/contain.rs:626` (`bool_contact_arc_end_vertex`, I11).**
  Every end decided against every vertex, and the point is still in the
  band of the end. The two passes disagree by construction. Decide once,
  or type the finding (design item 7).
- **C11, `boolean/sectors.rs:418` (`bisector_zero_refusal`).** An honest
  `±zero` enclosure, built by hand and off the log, reachable at K ≤ 2.
  Route it through the decision's gate.
- **C12, `boolean/plane_eq.rs:102` (`orientation_zero`, I17).** A decided
  Zero with its decided margin. Route it through `decide_nonzero`.
  - The undeclared caller is `plane_eq.rs:254`.
  - The declared caller, `carrier_eq.rs:726`, is D10-held. Leave it, or
    let it take the same door.
- **C19, `census.rs:2583` (`material_wedge_side`, I8, on RESTREAD's
  ground).** `classify_dihedral` reads `Transverse` after the edge screen
  decided agreement. Decide once, so that the second question takes the
  screen's verdict as typed input.
  - Today `own_close` gives this arm the defect ending because its
    margin is `INVALID`.
  - Keep that ending by type, not by `is_invalid()`.
- **C23, `merge_faces.rs:2901` (`LoopWinding` decided zero, on FUSE and
  TOPO's ground).** The decided margin is honest, but it is built by hand
  and off the log. Route it through `decide_nonzero`. Its `SizedDecision`
  ending (`LOOP_WINDING`) stays as it is.

Announce the RESTREAD, FUSE and TOPO seams in the PR.

## A cost to note

Routing a decided verdict through a gate door adds rejections that the
box driver bisects until its budget runs out. That is SCALAR's open
`a-gate-rejection-of-a-decided-enclosure-bisects-to-budget` (P3, E). It
is not a blocker, because every existing gate rejection has the same
cost.
