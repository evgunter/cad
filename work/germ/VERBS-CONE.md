---
id: VERBS-CONE
kind: issue
title: cone and torus operand lanes
status: open
opened: 2026-08-21
refs: [1604, VERBS-C5ARMS]
priority: P0
cost: H
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

**2026-09-28 (the C5 pose-gate unit):** `replace_face.rs`'s gate is no
longer a kind-pair boolean. It asks the pair's arm about the pose
(`geom_brep::route_pose`) and refuses `NeighborPoseUnroutable` by the
arm's own grounds; `curved_face_containment` has a cone arm. What this
item's operand lanes still meet in `crates/topo/src/boolean/reduce.rs`
is the list of kind dispatches with no cone arm, each unreachable for a
cone only because `boolean_arm_exists` keeps it off the roster:
`boolean_arm_exists` and `revert_arm_exists` themselves, the curved
clearance's second-derivative match (`f2`, a `_ => frontier()` door),
and `wall_crossing`'s root lane (`_ => Unsettled`).

**2026-09-28, the interior-loop class previewed on the cone** (the GERM
measurement lane). `Cone` was added to `boolean_arm_exists` AND
`revert_arm_exists` in a scratch patch, never landed. The cone is the
triangle `(0,0) (1,0) (0,1)` revolved fully about `y`, then
`merge_coplanar_faces`.

- **Without the merge** the two apex-closed bands refuse at
  `NonMaximalFaces`.
- **A partial revolve (3π/2)** refuses every op at
  `Containment(PartialConeFace)`. `point_in_solid` fails the same way,
  so partial-cone operands cannot even be measured today.

What came back:

- **Cone × a partial cylinder face, no crossings: every op WRONG.** The
  face is a 300° arc, centre `(z, y) = (0.8, 0.45)`, `r = 0.3`,
  extruded over `x ∈ [−2, 2]`. It meets the lateral face in a saddle
  loop, with every B edge outside the cone face's box.
  - ∪ is `Ok(Assembly)` and ∩ is `Ok(Empty)`. Both differences are
    `Ok(OperandA)`: each returns its first operand unchanged.
  - The true lens is `0.02413 ± 0.00005` (Monte Carlo), and
    `(0, 0.45, 0.53)` is `In` both operands.
  - The no-crossings fallback has no cone extent gate:
    `cylinder_extent_gate`, `torus_extent_gate` and `sphere_extent_scan`
    are its whole roster.
- **The same, with a pin through the base disc** (`x ∈ [−0.7, −0.5]`,
  `y ∈ [−0.3, 0.1]`, `|z| ≤ 0.1`): every op WRONG, valid `Seamed`. ∩ =
  `0.004` (the pin alone), and ∪ = `vA + vB − 0.004`. The pin alone
  answers all four ops correctly.
- **Refused before any body** (the crossing layer, not the class):
  - any line edge whose box meets a cone face without crossing it (a
    cylinder face's end lines, a tilted box's bottom edges; the plane
    ellipse fixture). This is `CurvedPierceUnsupported`.
  - a sphere × cone pair (an off-axis ball, and a large ball with every
    edge outside the cone's box). Also `CurvedPierceUnsupported`.
  - an axis-normal slab: `CurvedBooleanUnsupported { kind: Cone }`.

**What a cone admission must carry:**

- a cone half in `interior_loop_verdict`;
- a cone arm in the no-crossings fallback's extent gates.

A plane × cone ellipse always encircles the axis and so crosses the
cone's seams, but no plane fixture reached the join to confirm it.
