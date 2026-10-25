---
id: a-mirror-is-a-construction
kind: unit
title: A mirror is a construction: Node::Mirror { body, plane } over a reflection door, and every pose is proper
status: closed
opened: 2026-10-22
parent: patterns-are-index-variables
cost: H
priority: P0
branch: intent/mirror-is-a-construction
closed: 2026-10-22
pr: 5607
---


**Spec:** `docs/INTENT-STAGE3-SPEC.md` §8 (c), "Mirror": `Node::Mirror { body, plane }` is a construction that defines a new `Body` in its source's space, its plane a `Plane` pose whose reads reach that space alone (`docs/MIRROR-DESIGN.md` P1–P4, P6; D10 `[repetition]`). `Frame`'s improper arm, `Frame::admission_fault`'s improper case, `EditError::ImproperPlacement` and pncad-py's `mirror_across_plane` retire, so every pose is proper.

**What the spec leaves out.** §8's parallelism note lists the editor-side sites only. The kernel has no reflection door: `topo::transform_rigid` takes a `RigidMotion`, proper by type, and P2's sibling door and P3's audit were never built. This unit builds them too (`topo`), on P1's ratified `u ↦ −u` convention.

## Closed

2026-10-22, PR 5607. `topo::mirror_body` reflects a body into its inside-out image (P1's `u ↦ −u` on the analytic charts) and applies `revert`'s reversal map. P2's open claim is refuted: the half-edges must reverse. `Node::Mirror { body, plane }` evaluates over that door, carrying its source's names verbatim (`VerbatimEdge::Reflected`). It has the façade's `Node::mirror` and pncad-py's `Node.mirror`. The improper frame arms and `Frame.mirror_across_plane` retire: a mirrored stored frame refuses as non-rigid until G (b) retires `PatternKind::Explicit`.

Filed from this unit: `the-viewer-cannot-author-a-mirror` (AUTHTAIL), `a-symmetric-parts-seam-is-not-walked-as-one-construction` (INTENT), `lily-leaves-are-placed-by-hand-where-a-mirror-would-do` (`work/issues/`).
