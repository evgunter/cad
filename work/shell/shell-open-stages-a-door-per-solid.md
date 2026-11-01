---
id: shell-open-stages-a-door-per-solid
kind: issue
title: shell_open calls a staged offset door once per solid, so shelling N solids clones the cavity N times and walks tier 1 2N times over N solids
status: open
opened: 2026-10-24
priority: P2
cost: M
---


Found by OFFSET's designer pair on `doors-still-read-the-whole-body-for-tier1`
(2026-10-24). `crates/topo/src/shell.rs`'s cavity and lift loops call
`offset_charts_together_staged` / `offset_planes_together_staged` /
`offset_surfaces_together_staged` once per solid. Each staged call
clones the working body and gates it (two tier-1 walks today, one
after OFFSET's `a-staging-doors-end-derives-tier1-once-and-its-orphan-sweep-is-batched`),
so a shell of N solids costs N clones and O(N) walks of N solids:
quadratic. Measured on 64 boxes: the per-solid doors are about 45% of
the shell's run (the closing census is most of the rest, tracked
elsewhere). `replace_faces_offset` has the same shape and also re-mints
pcurves for the whole body and scans every half-edge for stale rows.

Shape of a fix (one designer's, not weighed by a pair): only public
doors stage; the crate-internal `_staged` entries run on the caller's
working body inside the caller's surgery scope, with no clone and no
gate, so `shell` pays one clone and one closing walk. Merge's
skip-recording groups (`RecordsASkip`) need a rollback per group and
would stay their own staged boundary. SHELF's
`offset-doors-take-solids-and-a-per-chart-rule` already moves `shell`
onto one inward call per solid; whoever builds either reads the other.
Filed by the OFFSET orchestrator.
