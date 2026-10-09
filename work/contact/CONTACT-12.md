---
id: CONTACT-12
kind: unit
title: the edge-on-face overlap lane cuts at boundary crossings, and ef_bound_backed migrates to region confinement, measured
status: closed
opened: 2026-09-29
priority: P0
cost: H
branch: contact/12-ef-crossing-cuts
closed: 2026-10-09
---


Carries `overlap-lane-boundary-crossing-cuts`.

Spec: `docs/CONTACT-12-SPEC.md`.

Review tier: **dual.** The unit changes the census's cut schedule,
which every edge-on-face touch reading stands on. It also moves one
grandfathered rung to region confinement under Ev's measured-migration
ruling (2026-09-01). A wrong cell bound is a wrong clear on the door
every consumer reads as proof.

## Re-scoped under the D10 hold (2026-10-08)

This unit was started before the hold (its rows are at `a6724f53b`),
so it finishes. Its step 2 migrates `ef_bound_backed`'s declared
face-pair arms to region confinement, and those arms are declared
pairs, which retire at INTENT stage 4. That step is dropped and parked
in the carried row (`blocked_on: [intent-stage4-is-built]`).

The unit finishes steps 1 and 3:
- boundary-crossing cuts, decided metrically;
- the touch analysis reading every cell.

Those two are the census's own geometry, which an interference finding
still needs after stage 4. Nothing on a declared rung is re-baselined.

## Closed

The census's edge-on-face overlap lane now cuts an edge wherever a
boundary edge of the face crosses it, as well as at coincident vertices
(`boundary_crossings`, split per carrier):
- **Straight boundary edges:** read by metric side and span rows.
- **Circle and ellipse arcs:** their raw roots are placed on the arc
  through the arc's own metric boundary reading, the `ConicArc` that
  `carrier_loop` builds.
- **Spiric and spline arcs:** refused, typed, unless a certified ball
  clears the edge.

The touch analysis now reads every cell of an overlap, and a decided
`Crossing` in any cell wins. Coincident cuts have a fixed order. The
`ef_bound_backed` migration, step 2, was dropped under the D10 hold and
parked as `ef-bound-backed-migrates-to-region-confinement`.

Review: a dual review, concurrent because the class is H (DR-111).
Both reviewers returned APPROVE-WITH-FIXES and raised the same MAJOR:
the conic arm reused the split lane's root-interiority Zero, which is
metered at the minor semi-axis, so a steep ellipse's crossing near an
arc end was dropped. That defect was not a regression; main cut no
crossings at all. The fix pass placed roots with the arc's metric
reading, made both witnesses rows (red first), and pinned every decide
with a mutant.

Filed:
- `census-edge-pass-reads-no-line-conic-crossing`;
- `edge-face-crossing-cut-and-pass-five-decide-one-crossing-twice`;
- `work/cleave/conic-plane-root-at-an-arc-end-reads-the-minor-meter`,
  the splitting twin.
