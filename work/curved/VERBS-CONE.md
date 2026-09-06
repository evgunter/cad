---
id: VERBS-CONE
kind: issue
title: cone and torus operand lanes
status: open
opened: 2026-08-21
refs: [1604, VERBS-C5ARMS]
---

Wave 2 row 10: cone (and torus) operand lanes for the boolean, sequenced on
what rows 6–9 learn. Gates Klein wall 3 (measured at #1001). Known traps named
in the plan: #226 residual 1 (conic-trimmed cylinder walls slip both sense
gates) and #685 (cone-wedge grid sizing drops `nv` at `nu == 1`, mesh side).
No spec; queued behind CYLSPH and the C5ARMS remainder.

**VERBS closed** (exit walk ratified, PR #1793); re-homed to
`work/issues/` awaiting an owner.

**Adopted by CURVED** at its opening for dispatch (2026-09-04, Ev's
in-chat direction): the plan's lane that carries this item is in
`work/curved/plan.md`.

**Remaining scope at re-home:** the cone and torus OPERAND lanes —
never cut as a spec; the C5-section half was executed by TORAX +
C5ARMS PR-1. A candidate seed for a successor program.

**This item carries `cone_cylinder_section`'s consumer schedule
(2026-09-05, VERBS-C5ARMS PR-2's fix pass).** The arm ships with the
`(Cylinder, Cone)` table row flipped and it has **zero production
callers** — the same standing as `plane_torus_section` and
`cylinder_sphere_section`. Nothing in the offset/shell path ever calls
a section function: `replace_face.rs`'s `route(..).implemented` is a
boolean gate, and `offset_charts_together` re-solves carriers in the
meridian half-plane without consulting the C5 table at all. Two
measured consequences bank here rather than in that PR's body:

- `shell(coned_tube)` **succeeds** at `t = 0.02 / 0.05 / 0.1` and is
  bit-identical with the flag reverted (measured by the PR-2 review).
  The spec's "`coned_tube`'s offset validates tier-3 with a closed-form
  volume pin" was a category error — that is a row about the
  simultaneous axial door, not about a section arm.
- What the flag actually buys today is one door of honesty at the
  PER-CHART door: `replace_face_offset` on a coned tube's cone stops at
  `ReanchorOffCarrier` (`d·cos α`) instead of `NeighborPairUnroutable`.

**The consumer that would make the arm load-bearing** is a caller that
asks for the cone×cylinder RIM CURVE rather than for the pair's
routability — the germ/chord lanes this item's operand work opens, and
`chord_join::section_case` / `boolean::join::pair_section_frame`, both
of which still refuse every cone-bearing pair typed. Until one of those
lands, the arm's evidence is its own acceptance rows.
