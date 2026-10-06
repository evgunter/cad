---
id: sphere-wedge-arm-does-not-fold-split-meridians-by-lineage
kind: issue
title: sphere()'s wedge arm reads a two-edge boundary and refuses a meridian that arrives in lineage pieces, where the torus arm folds pieces by lineage
status: open
opened: 2026-09-16
refs: [2748, 542]
priority: P0
cost: H
---

Found by BOOL-5's dual review (PR 2748) and filed by the S-BOOL
orchestrator on PROPS's slate (`crates/geom-brep/src/props/` is PROPS's
path). The rimless spherical-wedge arm counts the boundary's meridian
edges and serves exactly two; a real wedge whose meridian is two
lineage pieces (a split edge) refuses under the count `what` — now
worded truthfully, "the wedge arm reads a two-edge boundary; a meridian
in pieces is not folded on the sphere" — and BOOL-5 disclosed the
pair-only scope as a narrowing rather than build the fold. The torus
arm already folds meridian pieces by lineage (`fold_torus_meridians`),
which is the precedent and the shape of the fix: fold the sphere
boundary's meridian pieces by lineage before the count, so a split
meridian measures as the wedge it is instead of refusing as three
edges. Measured, not acted on; difficulty S–M.
