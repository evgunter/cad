---
id: a-tilted-plate-across-a-held-pierce-tessellates-short
kind: issue
title: A plate tilted 89.9° across a held pierce intersects to a body the tessellator meshes ~9e-4 m³ short, not converging under refinement
status: open
opened: 2026-10-15
priority: P3
cost: M
refs: [a-pierce-point-inside-a-curved-face-of-the-piercing-operand-refuses-unbuilt]
---

## What

Found by the full review of PR 5155, on a local-only pose. Nothing in the tree builds it yet.

**The pose.** The rod and ruling-square pair of `crates/sweep/tests/a_pierce_held_in_a_curved_face.rs`, intersected with the plate tilted 89.9° about the rod's tangent at the pierce point, toward tangency with the wall. The plate is the second operand.

**The B-rep is right:**

- mass properties read 0.7539476 in both orders;
- an independent column integration of the closed-form membership reads 0.753952.

**The mesh is not.** It is watertight in admesh but short, and refinement does not converge on the B-rep's volume:

| chord | admesh volume |
|---|---|
| 1e-4 | 0.75306 |
| 3e-5 | 0.752958 |
| 1e-5 | 0.753082 |

The same op with the operands swapped meshes 0.753793 at chord 1e-4.

**Unmeasured:** which face the tessellation loses, and whether it is the near-tangent strip between the tilted plate and the wall.

**Next step:** pin a row with the pose in the mesh suite, against the props volume, and find the face whose patch carries the deficit.

## Moved to TESSLIE (the mesh answers Ok, or panics, where it should refuse or mesh right) (2026-10-28)

Cut out of TESS by its semantic-seam split (`work/README.md`, Track size) when TESS measured 146 budget points against 30. Moved by `git mv`; the id and body are unchanged.
