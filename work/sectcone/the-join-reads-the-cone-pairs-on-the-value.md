---
id: the-join-reads-the-cone-pairs-on-the-value
kind: unit
title: The join reads cone × oblique cylinder and cone × cone on the section value: the tilted rod holds its truth, coverage has its row, GermFrameUnsupported retired for the swept poses
status: open
opened: 2026-11-22
priority: P1
cost: H
parent: cone-germ-pairs-against-a-curved-face-have-no-section-frame
---


U3 of `cone-germ-pairs-against-a-curved-face-have-no-section-frame`.

## What landed before this unit's build (2026-11-24)

- **The join** (PR 7457): `serves` takes `Arm::ConeRuled` and
  `Arm::ConeSphere`, so the section table holds cone × cylinder, cone ×
  cone and cone × sphere; `join.rs` `rides_carried` carries
  `PairPose::{ConeCylinder, ConeCone, ConeSphere}`, and
  `conic_section_frame_at` desyncs on a held cone pair and answers
  `NoArm` for the poses the value reads `Elsewhere`.
- **Carry** (PR 7457): `march_over_the_ball` marches the cone pairs
  (`ssi::march_cone_ruled`); `NoCarrier::ConeUnenclosed` and
  `ConeUnmarched` are gone. A section that lists a node refuses
  `Unminted(NoCarrier::ConeNode)` before any march.
- **Coverage** (PR 7457): `ruling.rs` `coverage`'s cone side has its
  caller, the march's debug check.
- **The bite** (PR 7457): `cone_operand_rows.rs`'s row flipped to
  `the_bite_rides_the_carried_cone_cylinder_section`.
- **The certificate** (PR 7637, U2): `section_cert` reads cone ×
  cylinder and cone × cone on the value.

## What this unit still owes

- **The tilted rod:** `crates/sweep/tests/reach_cone_root_lane.rs`
  `every_op_on_a_cone_wall_answers_its_truth_or_refuses_typed` still
  pins every op of the tilted rod across the narrowing wall to
  `GermFrameUnsupported { Cone, Cylinder }`. The pair is now held and
  carried, so the row is re-read: each op holds its truth (volume by the
  closed-form overlap, tier 3, membership over a grid), or refuses typed
  at a site that is not the cone × cylinder frame, pinned. Measured on
  PR 7735: the join builds, and the finish refuses where a cone face
  trimmed by the marched section has no volume lane, filed P1 as
  `a-cone-face-trimmed-by-a-marched-section-has-no-volume-lane`; the row
  pins that refusal op by op.
- **Coverage's row:** `coverage`'s cone side has a caller but no row in
  the default set; a row holds it to its quadratic on closed-form rings,
  a cone ruled against a coaxial cylinder and against a coaxial cone.
- **`GermFrameUnsupported` on a cone germ pair:** no general-pose cone ×
  cylinder or cone × cone pair reaches it. The poses the value reads
  `Elsewhere` still answer `NoArm` (a sphere on a cone's axis, two cones
  of one aperture on parallel axes, a coaxial torus); they are not this
  unit's (the parent's `## Decided` is the swept pose) and are filed as
  their own row.

## Not this unit's

- **The apex node in `node.rs`** waits on the march's node end
  (`the-r3-march-ends-no-branch-at-a-cone-node`): a section that lists a
  node is refused at the carry (`ConeNode`) before the join sees an arc
  ending there, so the node pass has nothing to mint. The item moves to
  that row.
- **`route_pose` on oblique cone × cylinder** is the offset door's
  question, filed as `route-pose-reads-oblique-cone-cylinder-on-its-closed-form`.
- **Downstream:** SPLITDOOR B3's follow-on (the pierce germ and the gate
  at the apex) waits on the node end too.
