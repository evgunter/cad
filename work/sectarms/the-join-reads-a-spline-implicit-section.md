---
id: the-join-reads-a-spline-implicit-section
kind: unit
title: The join reads a NURBS x implicit section: open components carried, chart pads in metres, GermLane::PlaneWall retired for a NURBS wall
status: open
opened: 2026-11-02
priority: P0
cost: H
blocked_on: [point-containments-ray-test-has-no-spline-arm]
---


Filed by the SECTARMS orchestrator (2026-11-02) from the NURBS arm's phase-0 plan. The arm's four stages build the classification and the whole reading on the knot rectangle. None of them makes the boolean's join read the result.

## What

Once `a-nurbs-face-against-a-cone-or-sphere-has-no-section-arm` lands its whole reading (stage 3), the join still has to read a NURBS × implicit section through the value:
- carry open components with rim ends;
- convert the chart's pads to metres by the chart speeds (`pad_of`);
- match sites by foot along an open component;
- retire `GermLane::PlaneWall`/`WallPlane` for a NURBS wall (`topo/src/boolean/join.rs`).

This is what makes teapot wall 3 join once SPLICE clears its seam split. Tier: dual.

## Nodes

An arc that ends at a node is carried to it by
`a-spline-implicit-arc-is-carried-to-its-node`, the unit after this one
(split out 2026-11-12; the design is PR 6829's). This unit carries open
components between rim ends and leaves a node arc as stage 3b does.

## Done when

The join builds a sphere × lofted-wall union through the value, and the NURBS wall's plane-wall lane is gone.

**This now waits on INSIDE** (`blocked_on`): `work/inside/point-containments-ray-test-has-no-spline-arm.md`.
With stage B (PR 6879) the join completes for a sphere, cylinder or
torus across a lofted wall at the default ε and at 1e-6: its carried
chords certify at rest. The union still does not build, because the
next layer refuses: `BooleanError::Containment(PointInSolidError::KindUnsupported { kind: Nurbs })`,
`point_in_solid`'s ray test having no spline arm. That row is P3 on
INSIDE's slate while this unit is P0, so by the band rule
(`work/README.md`, "A blocker does inherit the band of what waits on
it") the part of it that lets a ray cross a lofted wall is P0 work; the
re-band or the split is the two orchestrators' to make. What is left
here once it lands is to pin the built union and its volume
(`sweep` `a_ball_across_a_lofted_wall_is_joined_and_its_chord_certifies_at_rest`),
and the 1e-12 row, which stops at the chord's image
(`a-curved-partners-arc-is-imaged-at-rest-no-finer-than-the-mints-finest-rung`).

## The ray test crosses the wall; the next stop is the result's edge-in-face check (2026-11-25, INSIDE `inside/ray-spline-face-arm`)

Point containment's ray test now reads a spline face (the line × spline
patch arm of `curve_face::cross`), so no run here stops at
`Containment(KindUnsupported { kind: Nurbs })`. Measured at the witness
ε (1e-9), `sweep` `a_partner_across_a_lofted_wall_is_joined_on_its_section_value`:

- a ball (r 0.25) across the flat wall, all five operations:
  `BooleanError::EdgeInFace { gap: CarrierUnread }`;
- the same ball across the flared wall: `EdgeInFace { gap: SplineFace }`;
- a brick across the flat wall: every operation builds, validates, and
  its volumes agree (∪ 9.09375, ∩ 0.03125, ∖ 8.96875 and 0.09375 against
  9 and 0.125);
- a brick inside the flat wall's face: `ResultInvalid { VolumeUncomputable
  { source: RingOnCurvedFace } }` (filed on QUAD,
  `a-ringed-spline-face-has-no-volume-lane`).

The row's rows now assert each run builds a valid body or stops past the
join and past containment; the edge-in-face stop is this unit's next
measured layer, not containment's.
