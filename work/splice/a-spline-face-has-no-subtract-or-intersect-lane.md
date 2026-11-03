---
id: a-spline-face-has-no-subtract-or-intersect-lane
kind: issue
title: Subtract and intersect refuse any spline (loft or sweep) face whose box may meet the other operand, at the revert roster: NURBS is on the union's roster only
status: open
opened: 2026-11-03
priority: P0
cost: H
refs: [torus-onto-the-subtract-and-intersect-roster, a-saddle-cut-by-its-tangent-plane-builds-at-its-closed-form, the-spline-rider-arm-is-pinned-by-no-fixture]
---


Filed 2026-11-03 from the curved-boolean bookkeeping pass
(`work/issues/curved-demo-joins-build.md`). Klein wall 4 has pinned this
refusal for weeks, and no row owned it.

## What

`crates/topo/src/boolean/ops.rs:904`: for any op but ∪, the front door
runs `reduce::first_unsupported_pair` with `reduce::revert_arm_exists`
(`crates/topo/src/boolean/reduce.rs:419`), which lists plane, cylinder,
sphere, torus and cone and leaves out `Nurbs`. The union's roster,
`boolean_arm_exists` beside it, has `Nurbs`. So once a loft or sweep
body's spline face may meet the other operand (box overlap), every ∖
and ∩ refuses `CurvedPairUnsupported { op: Some(_), site: RevertRoster,
kind: Nurbs, .. }`, before SPLICE's union lanes are reached. A declared
pair cannot cover it (the comment at `ops.rs:897`).

Witnesses:
- klein wall 4, cutting the flare where the neck passes through
  (`demos/tour/src/klein.rs` `wall4_pinned`: `{ op: Subtract,
  operand: B, kind: Nurbs, other_kind: Plane }`);
- SPLICE's saddle, both orders of ∩ and ∖
  (`a-saddle-cut-by-its-tangent-plane-builds-at-its-closed-form`);
- PARTING's spline-face arm, which no fixture reaches because this
  roster refuses first (`the-spline-rider-arm-is-pinned-by-no-fixture`).

Cutting a swept groove or a lofted pocket into a part is everyday
authoring, so the row is P0.

## What the taker owes

The torus took this step in `torus-onto-the-subtract-and-intersect-roster`
(PR 3416, M) once its union lanes existed: measure ∖ and ∩ over the
union's fixtures with the kind admitted, find the classes that return
wrong bodies, gate or fix them, then admit the kind. Do that for `Nurbs`
over SPLICE's union rows. The docs on `revert_arm_exists` and on
`BooleanError::CurvedPairUnsupported` (`crates/topo/src/boolean/mod.rs`,
"NURBS: no edge×NURBS-face crossing layer at all") predate SPLICE's
crossing lanes, so re-read them first. Priced H rather than the torus's
M because the spline union lanes behind it are still being built. Expect
admission to be staged by partner kind, as the union's was.

## Phase 0 measured (2026-11-03, main c89d725d)

**Why the roster exists, and what a spline face lacks for ∖ and ∩.**
∖ and ∩ differ from ∪ in three things: material sides (which
fragments are kept), the reversal of one operand (`A ∖ B ≡ A ∩
revert(B)`), and the result's orientation. All three are kind-generic
in the pipeline: the region selection and the revert run the same
doors for every op, and `Body::revert` only flips `Face::sense`.
The docs on `revert_arm_exists` (`reduce.rs:408`) and on
`CurvedPairUnsupported`'s NURBS bullet (`mod.rs:3722`, "no
edge×NURBS-face crossing layer at all") give two reasons. Both are
stale or op-generic:

- The crossing layer exists now (SPLICE's lanes). Every one of its
  doors runs under each op alike.
- The extent test (`NurbsExtentUnsupported`, `ops.rs:6138`) runs in
  the no-crossings path for every op, ∪ included.

The one new exposure is that, under ∖, a sense = false NURBS face
reaches the spline lanes for the first time. No loft or sweep mints
one (`sweep/src/loft.rs:1241`, `wall_surface`, `sense: true`). An
audit of the spline lanes found that every reader of a signed normal
folds the bit:

- `face_normal::spline_outward_normal_at` (face_normal.rs:534)
- the pierce normal in `vtxfac` (vtxfac.rs:555)
- the sector walk's `sector_face::spline_normal` (sector_face.rs:360, 502)
- the slit pass's `spline_floor` (slit.rs:1799)

The readers that take no sense use the normal without its sign:

- `dihedral::seam_face_reading` (dihedral.rs:468) reads `|n1×n2|` only.
- `spline_region`'s trim placement is ray parity, which is
  loop-direction-free.

So nothing was found that a spline face lacks for ∖ and ∩ beyond what
it lacks for ∪.

**Measured** with `Nurbs` on `revert_arm_exists` in a scratch tree, at
the witness, 1e-6 and 1e-12 rows. ∪, ∩ and ∖ ran in both orders, each
result checked against its closed form or by inclusion–exclusion, and
against a point sample of 400 points per run against both operands.
Spline operands: the loft prism, the lofted cylinder and the wavy
loft. Partners: a brick, a cylinder rod, a ball, a cone frustum, a
torus and a lofted cylinder, each placed inside the operand and across
its wall, plus a peg through the cap and a slab through every wall.
Also run: the saddle against its tangent half-space, and
`spline_spiric_operands`' census.

- **No body is wrong, and no new body is returned.** 40 cases × 6 runs
  at each ε. Every ∖ and ∩ refuses with exactly ∪'s refusal for the
  same operand order, variant and payload alike (0 differences, all
  three rows).
- Contained partners refuse at containment:
  `Containment(KindUnsupported { Nurbs })`, or
  `NurbsExtentUnsupported`.
- Wall-crossing partners refuse at the union's own frontier, per
  partner kind:
  - plane and cone: `GermFrameUnsupported`;
  - cylinder, sphere and torus: `SplineCrossingUnsettled`, or
    `PierceLeverUnread` on the lofted cylinder;
  - spline × spline: `SplineCrossingUnsettled`,
    `PointSplitCarrierUnsupported` or `PierceLeverUnread`.
- The saddle's ∩ and ∖ move from the roster to ∪'s
  `HeldPierceUnbuilt { Nurbs, Plane }`.
- `spline_spiric_operands`' census still passes.
- Klein wall 4 moves to wall 3's stop:
  `SplineCrossingUnsettled { operand: A, edge: 4v1, face: 3v3,
  face_of: B, why: Crease }` (the crease class), at all three ε.
- `review_cleave_nurbs_lane`'s enclosed M7-8 cube under ∖ moves to
  `SplineCrossingUnsettled { why: Floor }`. It still stops before the
  void door.
- topo, sweep and editor-core's full default-ε suites (slow set
  included) show no other change. Six further reds there are
  inherited: they fail identically on c89d725d's own tree.
