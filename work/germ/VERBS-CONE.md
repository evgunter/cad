---
id: VERBS-CONE
kind: issue
title: cone and torus operand lanes
status: closed
branch: germ/cone-roster-flip
pr: 4418
opened: 2026-08-21
closed: 2026-10-10
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
  a corner gap (`d·cos α`, the since-retired `ReanchorOffCarrier`)
  instead of `NeighborPairUnroutable`; the door now derives that rim
  as the cone × coaxial cylinder section, and the offset builds.

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

**2026-09-28, the preview is superseded.** The interior-loop preview
this item carried was measured on `378f66744`, before the section
certificate (PR 3372) merged. The spec's §0 re-ran every fixture on
`e6f3eaaf9` (`docs/GERM-VERBS-CONE-SPEC.md`, deleted when the last unit
merges): with `Cone` on both rosters, every fixture now refuses typed
and none answers wrong. The preview's two wrong answers — the
no-crossings path with no cone gate, the crossings path with no
interior-loop guard — are closed by the certificate's per-pair pass,
which scopes cone faces on both paths and answers R-reach for every
cone pair until its cone rows land. The extent gates the preview named
(`cylinder_extent_gate`, `torus_extent_gate`) no longer exist; the
no-crossings path runs `sphere_extent_scan`, then the section pass.

**What a cone admission must carry** is the spec's §3, in order:

- **the root lanes first** (U1, U2). The one silent arm is
  `curved_face_arm`'s `(Negative, Negative) => None`, which clears on a
  convexity the double cone lacks (§0's P3: four crossings near the
  apex cleared silently, masked today only by R-reach). Every W1 or W2
  clearance rests on premise S, which that arm breaks.
- **the apex closure** (U3) and **the certificate's cone rows** (U4),
  then the sphere pairs on the no-crossings pass (U6), then the roster
  flip (U7).

U3 and U6 land together in the PR titled "germ: the cone apex closure,
and sphere pairs certified on the no-crossings path".

**2026-09-28, from the section-certificate lane's premise-S audit.** Two more arms must learn the cone before `boolean_arm_exists` admits it:

- **`reduce.rs` `curved_face_arm`, the "both endpoints inside" arm (about `:1519` on `2ba90bced`).** It rests on the carrier's convexity, which holds for a cylinder or a sphere. A cone's is not the same: a nappe pair is not convex, and the apex breaks it. Unreachable today only through the roster.
- **The section certificate's cone arms (its spec's Q3):** the crossings-path half, AND the no-crossings arm that replaces the extent gates. These land with this item.

**2026-09-29, U4 (the certificate's cone rows).** `section_cert::classify`
answers cone × {plane, sphere, coaxial cylinder, coaxial cone, coaxial
torus, parallel-axis cylinder}; oblique cylinders, tilted and
parallel-axis cones, non-coaxial tori and splines stay R-reach. Cone ×
plane is decided on the aperture margin alone, never on the apex's
offset (a Zero offset does not bound the ellipse). The roster stays
closed until U7.

**2026-10-09, U4 lands (PR 4352).** The cone rows ship with the near-axis
offset taken through `square_to` at every arm (the dual pair's bilateral
MAJOR, DR-117). The witness is checked against the plane's carrier, and
the search samples down into the band. Still owed before U7: the cone
sector units U-S1..U-S4 (`docs/GERM-CONE-SECTOR-SPEC.md`).

**2026-10-09, U7 (the roster flip).** `Cone` is on `boolean_arm_exists`
and `revert_arm_exists`. Measured on the flip, every op of the spec's
fixtures builds its closed form (P9, P6, P4, P2a, P10, P5, P7, the
3π/2 cone's gap brick, P8, and the cone sector spec's C1) or refuses
typed: P3 naming the hyperbola, P1 and P2 at the cone × cylinder frame,
and the held configurations (a tangent plane, a cone sector on the cone
face) at the crossing layer. No body is wrong. P3 against the mutant
"U1's guard reverted" returns valid wrong bodies, and its row is red
there.

**2026-10-10, closed: U7 lands (PR 4418, DR-128, sequential arm).** `Cone` is on `boolean_arm_exists` and `revert_arm_exists`, so a cone operand reaches every op in production. The single review fuzzed 26,376 bodies against an independent analytic oracle and found none wrong. Still open, as their own items:
- cone × cylinder and cone × cone in general pose (the spec's optional U5): `cone-pairs-in-general-pose-have-no-section-arm`;
- rings on a cone face in `face_flux`: `docs/GERM-CONE-SECTOR-SPEC.md` U-S5;
- the held configurations under D10 (U-H1, U-H2).

`docs/GERM-VERBS-CONE-SPEC.md` is kept rather than deleted at this merge: its U5 is still unbuilt and is cited by the open item above. It goes when U5 lands or at GERM's close.
