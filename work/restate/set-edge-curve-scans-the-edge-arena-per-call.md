---
id: set-edge-curve-scans-the-edge-arena-per-call
kind: issue
title: set_edge_curve's orphan check scans the edge arena once per call, so a door re-describing k edges one call at a time pays k × body
status: open
opened: 2026-10-24
priority: P3
cost: E
---

Found by OFFSET's `a-staging-doors-end-derives-tier1-once-and-its-orphan-sweep-is-batched`
(2026-10-24), which batched the orphan pass of the multi-key setter
(`Body::orphan_reaper` / `release_curve` / `reap_surfaces`,
`crates/topo/src/body.rs`).

`Body::set_edge_curve` (and every door through `Body::replace_edge_curve`,
`crates/topo/src/attach.rs`) reaps the curve it replaced with
`Body::remove_curve_if_orphaned`, which scans every edge — and, when it
removes a curve, every face and curve once per surface its description
named. One call is O(body), which is right for one write; a door that
re-describes k edges one `set_edge_curve` at a time pays k × body.
`replace_face::offset_charts_staged`'s re-anchor loop (`anchored`) is
one; the boolean's and merge's `describe_edges` are worth checking.

The batched pass is there to reuse: a door that knows its writes up
front can take one `orphan_reaper` for all of them. The counter
`body::ORPHAN_PASS_VISITS` (`cfg(test)`) counts both kinds of pass, so
a row in the shape of `attach::tests::orphan_sweep_scales_with_the_writes`
can pin it.
