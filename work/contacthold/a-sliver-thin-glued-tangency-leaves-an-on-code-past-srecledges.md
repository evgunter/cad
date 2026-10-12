---
id: a-sliver-thin-glued-tangency-leaves-an-on-code-past-srecledges
kind: issue
title: A sliver-thin glued tangency leaves an On code past srecledges and refuses ClassificationInvariant
status: open
opened: 2026-10-12
priority: P3
cost: M
---



## What

A disc of radius 2 about the origin, z ∈ [0, 1e-5], against a disc of
radius 1 about (1, 0), z ∈ [−1, 2e-5]: internally tangent along the
ruling through (2, 0), normals aligned, the thin disc run past the
sliver both ways. Every op refuses `ClassificationInvariant { what:
"surviving record with an On code after srecledges" }`, the postcondition
of `recl::recl_edges` (pinned:
`crates/sweep/tests/review_4831_probes.rs::p10_a_sliver_thin_disc_nested_on_a_long_one`).
That is at ε 1e-9; at 1e-6 the union escalates (`VertexOnFace`) and at
1e-12 it builds (hosted CI). At ε 1e-9, z ∈ [0, 1e-4] escalates and
[0, 1e-3] answers every op at its closed form.

PR 4831's fix pass exposed it. Before, `recl::resolve_edge_sector` handed
the glued tangency to `recl::require_same`, which refused
`CurvedBooleanUnsupported` (measured with that arm restored behind an
env toggle). Now the edge–sector lane lumps the on-flanker
(`sectors::tangent_lump`) and the op runs on to the postcondition.

Not traced to a line. The likely route, unverified: along the 1e-5 seam
edge the second-order sagitta at the sector's arm reads in the zero band,
so `recl::recl_sectors`' tangent arm reads `On` for a bound that does not
ride the locus (`sectors::tangent_relative_side`'s exact zero), and no
edge event rewrites it. The fix is either to escalate that reading as the
sliver it is (F6) or to resolve it as `tangent_lump` resolves the exact
zero, by the pair's sense bit.
