---
id: the-in-solid-walk-reads-no-outside-of-a-body-with-a-ringed-cylinder-wall
kind: issue
title: point_in_solid leaves every outside probe of a body with a ringed cylinder wall unread (VolumeUncertified)
status: open
opened: 2026-10-15
priority: P3
cost: M
refs: [a-pierce-point-inside-a-curved-face-of-the-piercing-operand-refuses-unbuilt]
---

## What

Found in PR 5155: the lying cylinder over its whole span, `a_plane_face_holding_the_pierce_over_a_cylinder_builds_in_every_op_and_order` in `crates/sweep/tests/a_pierce_held_in_a_curved_face.rs`.

**The bodies.** Two prisms stand across the lying cylinder's top, and each leaves a ring on the cylinder's wall. On those bodies, `topo::point_in_solid` answers `VolumeUncertified` at probes outside the body. The review traced it to `at_infinity_side` (`crates/topo/src/boolean/solid_contain.rs:5538`): a no-hit ray, with the closed-form volume refused.

**Census over the row's 1134 probes:**

| body | probes read |
|---|---|
| the union | 1116 |
| the cylinder less the pair | 495 (its inside alone) |

The same bodies with the cylinder capped across both prisms (no ring) read every probe. So the row reads the capped span in full and the whole span at 43%.

**What is wrong.** The bodies are sound, by tier 3 and their closed-form volumes. The gap is the walk's capability: it reads no side for a ray that meets nothing on a body whose volume it cannot certify. A ringed cylinder face is not in that inventory; `RingOnCurvedFace` refuses the same face at tessellation.

**Next step:** give a ringed cylinder face a closed-form flux (the analogue of `flux/sphere-face-with-a-hole-has-no-closed-form`), or have the walk aim a ray at a planar face. Then raise the row's read share for the whole span to 99%.
