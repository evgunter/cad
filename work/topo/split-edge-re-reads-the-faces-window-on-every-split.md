---
id: split-edge-re-reads-the-faces-window-on-every-split
kind: issue
title: split_edge re-reads its faces' whole window on every split, so N splits on one minted face cost O(N²) reads
status: closed
opened: 2026-10-05
closed: 2026-10-06
priority: P3
cost: M
design: true
refs: [the-site-mints-plan-reads-the-rewired-loop-whole-on-every-op]
---

Found by the sweep for O(loop) re-reads in the Euler doors' plan
phases, while working
`the-site-mints-plan-reads-the-rewired-loop-whole-on-every-op`.

`split_cache` (`crates/topo/src/pcurves.rs`, called from
`Body::split_edge` in `crates/topo/src/split.rs`) certifies each
restricted child against the face's window. It takes that window from
`stored_rows(body, face).window`, which walks every loop of the face
and computes the chart box of every row. So one split costs a chart
box per row of each face the edge bounds, and `n` splits on one
minted face cost O(n²) boxes. The window enters the verdicts here as
real work: it holds the parent's box, so check 5 is a real check.

Measured with `crates/topo/tests/site_mint_scaling.rs` (`#[ignore]`d):
`n` splits of one rim of `cyl_wall_sheet` inside a surgery scope.
Release: 0.0007 / 0.0013 / 0.0032 / 0.0084 / 0.0258 s at
n = 25 / 50 / 100 / 200 / 400, about ×3 per doubling. Shared box, so
read the shape only.

A delta fix needs the face's window carried from the previous door. A
split can shrink that window, because a child's box lies inside its
parent's. So the window cannot be a plain hull updated by insertion.
That is the question
`the-site-mints-plan-reads-the-rewired-loop-whole-on-every-op` holds,
and this row is decided with it.

## Closed

Closed by the retirement of check 5 (PCERT's
`pcurve-loop-decisions-state-a-3d-identity-plus-a-branch-margin`,
ratified on [ev] PR 3919). The pcurve certificate tests no row against
a window, so `split_cache` reads none: it certifies each restricted
child against its carrier and chart and decides the joint between the
two children at the split point, and reads nothing else of the face.
One split's cost no longer depends on how many rows its faces hold.
