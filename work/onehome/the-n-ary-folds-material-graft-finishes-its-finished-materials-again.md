---
id: the-n-ary-folds-material-graft-finishes-its-finished-materials-again
kind: issue
title: The n-ary fold's material graft finishes again the finished materials it grafts beside the carved tool
status: open
opened: 2026-11-24
priority: P3
cost: M
---


Found by the gather-carries lane's sweep (2026-11-24) for doors that
graft finished bodies as construction-state data and finish the graft
again from values.

`topo::boolean::material` (`crates/topo/src/boolean/material.rs`, the
fold's material union): where a tool stands beside folded materials, the
body is the carved tool's standing solids (`carve::solids_alone`) with
each folded material grafted beside them
(`graft_disjoint_all_keyed(&mut body, &material.body().body)`), then
`T::finish(body, tol)`. Each material is a finished body (the fold's
result), so its solids' verdicts are derived a second time; the carved
part is construction state and has to be derived.

`topo::gather` (the gather-carries PR) carries a disjoint graft of
finished bodies. Here only some solids are finished, so carrying them
needs a finish that derives the carved solids and carries the grafted
ones: tier 3 is local to one solid, so it is the same argument, but no
door does a partial carry today. The cost is unmeasured; no verdict is
wrong, so P3. (gather-carries lane)
