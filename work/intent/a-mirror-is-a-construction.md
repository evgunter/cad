---
id: a-mirror-is-a-construction
kind: unit
title: A mirror is a construction: Node::Mirror { body, plane } over a reflection door, and every pose is proper
status: dispatched
opened: 2026-10-22
parent: patterns-are-index-variables
cost: H
priority: P0
branch: intent/mirror-is-a-construction
---


**Spec:** `docs/INTENT-STAGE3-SPEC.md` §8 (c), "Mirror": `Node::Mirror { body, plane }` is a construction that defines a new `Body` in its source's space, its plane a `Plane` pose whose reads reach that space alone (`docs/MIRROR-DESIGN.md` P1–P4, P6; D10 `[repetition]`). `Frame`'s improper arm, `Frame::admission_fault`'s improper case, `EditError::ImproperPlacement` and pncad-py's `mirror_across_plane` retire, so every pose is proper.

**What the spec leaves out.** §8's parallelism note lists the editor-side sites only. The kernel has no reflection door: `topo::transform_rigid` takes a `RigidMotion`, proper by type, and P2's sibling door and P3's audit were never built. This unit builds them too (`topo`), on P1's ratified `u ↦ −u` convention.
