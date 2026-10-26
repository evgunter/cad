---
id: the-aspect-cap-stands-in-for-the-sliver-criterion
kind: issue
title: The NURBS split's aspect cap A = 16 is a heuristic standing in for the sliver criterion; replace it with what it means, or show it earns its place
status: open
opened: 2026-10-25
priority: P3
cost: M
design: true
---

Ev on PR 5708 (2026-10-25), on `ASPECT_CAP` (A = 16, ratified on #568):
"it can also be changed if it ends up not being beneficial; it seems
like a heuristic that might be best replaced with what we actually
mean, but i forget the context here".

The context. #547 measured the NURBS split schedule's dominant slack
(about 4.1×) in `grid_steps`' AM-GM decoupling. The unconstrained
optimum on the certified ellipse is a degenerate strip (leaf_a f2:
1×4905, parameter aspect about 5·10³). A strip admits Delaunay slivers
beside every off-lattice point, with certificate about
`(aspect² + 1)/8 · δ_s` (`docs/TESS-BUDGET.md`, "The sliver hazard").
A = 16 caps the first-fundamental-form aspect so the optimizer cannot
pick such a strip.

What A "actually means" is probably the sliver bound itself: choose
the cheapest ellipse point whose worst admissible sliver, under the
lane's lattice metric and residual line, certifies within the
`[mesh.sizing]` bound. A designer question first: is that criterion
statable in closed form at the split, and does it subsume
`MAX_RESIDUAL_SQ` and the lattice metric's power-of-two `k` too?
