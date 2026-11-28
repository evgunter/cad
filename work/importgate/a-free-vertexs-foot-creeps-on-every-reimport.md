---
id: a-free-vertexs-foot-creeps-on-every-reimport
kind: issue
title: A free vertex's foot creeps on every re-import: a revolved cone under a hemisphere is no byte fixed point
status: open
opened: 2026-11-25
---


Found by `lathe/import-wrap-tip` while pinning native revolve → export →
import at a chart singularity
(`step-import/tests/a_revolved_pole_or_apex_imports_as_built.rs`,
`a_revolved_cone_under_a_hemisphere_imports_as_built`).

## What

The full revolve of the profile `(0, −1.25) → (0.5, 0)` (a cone's
generator), then a quarter arc of bulge `tan(π/8)` to `(0, 0.5)` (a
hemisphere), about the sketch y axis. The body is a cone wall and a sphere
wall meeting on one rim circle, each reaching its apex or pole by a wrap
edge from the rim's one vertex. Export, then import and re-export over and
over. Every hop reports one `NormalizationKind::VertexBuilt` on that rim
vertex, `moved_by: 1.064962222048366e-29`, every time. Its printed y
creeps by that much per hop and never settles:

    4.484050470211467E-16, …573E-16, …680E-16, …786E-16, …893E-16, …

So no number of round trips reaches a byte fixed point. Census and
certified volume hold. The geometry carries a rounding the revolve leaves:
the rim circle's centre prints at `y = 2.22e-16` and the sphere's at the
origin, so the vertex's y is a residue. Through `build.rs`'s
`build_vertices` (the free vertex is footed on one built carrier), the
foot of the stored point is not idempotent: the foot of a foot moves
again, by a constant amount.

A frustum under a spherical zone (no apex, no pole: `(0, −1.25) → (0.2,
−1.25) → (0.5, 0) ⌒ (0.3, 0.4) → (0, 0.4)`) is byte-stable over six hops,
so the creep needs this rim's geometry. Read, not bisected further.

## What would close it

The foot (or whichever builder places this vertex) answers its own
input, so a re-import of its output reports no move. Then the pin above
holds the cone-under-hemisphere's re-export text as it holds the ball's
and the cone's.
