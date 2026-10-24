---
id: w2-describes-folds-a-corrupt-face-into-a-miss
kind: issue
title: ChartCache::describes reads every chart_boundary Err, Stale and MissingCache included, as 'does not describe', so a corrupt face reads as a W2 miss rather than a kernel defect
status: open
opened: 2026-10-24
priority: P3
cost: E
rides_with: w2-reads-a-one-sheet-predicate-off-the-walk
---


## What

`topo::boolean::ops::ChartCache::describes` (`crates/topo/src/boolean/ops.rs`) is
`chart_boundary(..).is_ok() || apex_closure_describes(..)`. Every `Err` from
`chart_boundary` folds into "does not describe": the topological refusals W2
(the section certificate) is asking about (a loop that wraps, a singular joint),
and also the ones that say the face is torn (a stale or missing pcurve row, a
row that fails to certify). A torn face therefore reads as "this face's
interior does not lift with zero winding", and the pair refuses on W2's
grounds. D9's taxonomy wants a kernel defect there instead.

Found by CHART's region-description designers (2026-10-24) while reading W2 as
`chart_boundary`'s second consumer. Not reproduced. The fix is to match the error
kinds and raise the torn ones as the defect they are. (CHART orchestrator)
