---
id: along-edge-ring-on-a-curved-face-has-no-join-arm
kind: issue
title: A segment along an edge of both solids that closes a ring on a curved face has no join arm (CurvedBooleanUnsupported in choose_roles)
status: open
opened: 2026-10-04
priority: P3
cost: M
---


Found by the parallel cylinder arm's sweep (PR 4031) for join sites
that refuse "no arm" after the germ pair's frame was accepted.

## What

`boolean::join::choose_roles`, ring lane, `RingClosure::AlongEdge`: a
segment whose loci are `OnEdge` on both operands reads no section
(`SegmentLane::AlongEdge`), so a planar face's island closes on its own
plane, and a curved face's has nothing to close along and refuses
`CurvedBooleanUnsupported` naming that face. The `RingClosure::AlongEdge`
doc calls it "a join arm not yet built".

## Measured

Unreached. Instrumented on this branch, the topo, sweep and
editor-core suites (7 042 tests) and the 25 JOIN batteries
(`join1_r1_*`, `join1_delta_*`, `rc_wide_battery`, `j3r2_*`) never
reach it. A pose needs a section running along an edge both solids
hold, inside a ring of a curved face: coincident edges, which today is
coincidence ground (held under D10 while
`work/intent/d10-one-way-to-say-intent-is-unbuilt.md` is unbuilt).

## What a fix has to supply

The island's closing along the edge itself, on the face's chart: the
edge's own curve is the chord, so the winding reads the edge's chart
image rather than a section plane. Build it once a pose reaches it.

## Moved to SECTTAIL (2026-10-25)

From SECT, cut on its priority seam (SECT measured 46 budget points against 31) in a chat-approved refold. Read SECT's step 3 first: SECT's plan says several of these may close by retirement then.

Moved from SECTTAIL to SECT on 2026-11-02, by `git mv`, when SECTTAIL merged back into SECT: the track budget now counts only the P0, P1 and unbanded rows a sitting dispatches (Ev, in chat, 2026-11-02), so SECT and the three tracks cut from it (SECTTAIL, SECTRUNG, SECTSITE) came in under one budget.

## The D10 hold has lifted (2026-11-03)

INTENT closed on 2026-10-30 with D10's umbrella row (#6103,
`docs/doc-ledger/intent-leaves-the-tracker.md`), so the hold named above
no longer stands. What is left is the reach: no public pose reaches the
case yet, as the row says.
