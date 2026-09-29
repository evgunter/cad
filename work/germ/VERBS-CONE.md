---
id: VERBS-CONE
kind: issue
title: cone and torus operand lanes
status: dispatched
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

**2026-09-29, U1 and U2: the root lanes** (the PR titled "germ: certified
root lanes for line and circle edges against a cone face (VERBS-CONE
U1+U2)"). The silent arm above is closed: `curved_face_arm`'s same-side
roots arm is guarded `Torus | Cone`, so a cone takes the exact roots for
every endpoint-sign pattern and the convexity arms never see it.

- **Line × cone** (`crates/topo/src/boolean/line_cone.rs`): the ray
  lane's quadratic, factored into `solid_contain::line_cone_roots` and
  shared bit-identically, wrapped in an `f64` noise meter. The
  discriminant is decided again with its evaluation error charged, and
  each root's slack is held to the band unless the root is farther from
  the span than its slack. A generator-parallel line, a tangency and a
  line through the apex keep the door (Q3).
- **Circle × cone** (`crates/topo/src/boolean/circle_cone.rs`): the
  half-angle quartic through `half_angle_roots`, with the cone's
  harmonics, the apex's distance as the noise meter's floor, and the
  carrier's own `2ρ` as the lever; the coaxial pose decided in closed
  form at the circle rung (a parallel ON the cone is the door); the
  parallel-axes pose in closed form on the elevation, its admitted tilt
  charged to the margins and to the root positions.
- **P3 measured:** the apex pin's four crossings, recorded as none with
  the guard reverted, are recorded now (`sweep_past_the_cone_roster`,
  a `topo::test_support` door that lets the cone past the roster and
  keeps every other gate). The roster is untouched; the flip is U7's.
- **Filed:** `ray-cone-quadratic-is-unmetered-for-f64-noise` (P3); the
  cone lanes' far-origin and large-radius answer rates are recorded on
  `circle-torus-roots-refuse-large-circles-for-want-of-recentering`.
