---
id: torus-operand-gate-admission
kind: issue
title: Torus onto boolean_arm_exists - the stem glue's door sequence after the box and the residual arm (a gate-policy unit, CURVED's)
status: closed
opened: 2026-09-04
refs: [torus-operand-boxes-span-whole-ring, circle-residual-harmonics-needs-torus-arm, torus-declared-rest-lane-banked, torus-coincident-pair-cannot-reach-the-covered-rung]
priority: P0
cost: D
branch: germ/torus-doors
closed: 2026-09-26
pr: 3265
---


## What

`docs/CURVED-TORUS-SPEC.md` §Lily (deleted at PR-2's merge; recoverable at the SHA `docs/DOC-LEDGER.md` names) derives the lily stem glue's door
sequence beyond the box (PR-1) and the residual arm (PR-2):

- (a) `Torus` onto `boolean_arm_exists` (`reduce.rs`) — the gate never
  boxes a supported kind, so the box's tightness then serves the sweep
  tree, `separation`, `ops` and the census, not the gate. A gate-policy
  change; honest only once the crossing layer has torus arms.
- (b) the arch's outer-equator seam against the STEM's carrier crosses
  it at 29° along the arch, outside the stem face's window — a
  circle×torus pierce/trim question with no root lane; the honest
  negative certificate maps the arc into the stem's chart.
- (c) `curved_face_containment`'s torus interior — S-BOOL's
  `curved-face-containment-lacks-cone-torus` (handover to CURVED
  pending on the away channel).
- (d) the weld's coplanar concentric caps at the F7 merge door.
- (e) `torus-coincident-pair-cannot-reach-the-covered-rung` — with
  the circle rung's torus arm landed (PR-2, #1489), a COINCIDENT
  torus pair decides definitely-Negative rather than Zero, so the
  declared-cover rung behind it is still never consulted. The
  reduction-order question that raises is this unit's, not PR-2's.

**Correction by citation.** MATE-7a's "one function away" (PR #1477,
issue 1489) was measured on the coincident full-torus pair, not on the
lily; and its R2's "a boundary-tight box retires wall 1" (issue 1488)
is refuted by the spec's R3 (a disc concentric and coplanar with a
larger circle lies in every AABB of that circle). Neither PR-1 nor
PR-2 retires wall 1; this item is the unit that can.

## Home

CURVED — the operand gate is the curved-operand-reach lane
(`work/curved/plan.md` §Lanes "Torus lane completion"); filed by the
orchestrator at the spec's ratification.

## HIGH PRIORITY — Ev asked for it (2026-09-17)

**Ev asked for this in chat on 2026-09-17 and wants it treated as
high priority.** Ev hit it as a user: unioning the two halves of a
dumbbell (torus-faced bells against planar faces) in the viewer
refused with "face … of operand A is a torus and its box MAY INTERSECT
face … (plane) of operand B … it has no seam lane for the (torus,
plane) germ pair", and the viewer offers no way around it. The refusal
names the germ-pair JOIN dispatch (only (Plane, Plane),
(Plane, Cylinder) and (Plane, Sphere) are wired) as what blocks, so
admitting the torus operand here needs that join arm as well as the
gate change. Whoever takes this should confirm that before scoping.
The viewer-side rows this surfaced are
`work/view/a-refusal-offers-no-action-in-the-viewer.md` and
`work/view/a-derived-pick-index-failure-outshouts-its-cause.md`.

## Closed (PR 3265, 2026-09-26)

**(a) The torus is on `boolean_arm_exists`.** The crossing layer has its
torus arms:
- line×torus through `line_torus_roots`, keeping the door on a certified
  root its landing point contradicts;
- the sector walk;
- the pierce normal;
- a torus extent gate in `ops.rs` for a face with no crossings.

(c) and (e) closed with it. **Where the other sub-items went:**
- (b) is `circle-crosses-a-torus-face-with-no-root-lane`;
- (d) the joint weld is ZIP's `dumbbell-joint-union-leaves-four-loose-ends`.
  Past every torus door, the revolved dumbbell stops there, exactly where a
  cylinder-handle control stops.

The premise that the (Plane, Torus) join arm is what blocks the dumbbell
was measured false: Ev's refusal came from the operand gate. The F7 wall
ahead of it is CARVE's `full-revolve-emits-split-planar-walls`, ruled by
Ev on 2026-09-25.
