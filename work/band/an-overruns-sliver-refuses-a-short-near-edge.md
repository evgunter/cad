---
id: an-overruns-sliver-refuses-a-short-near-edge
kind: issue
title: blend: a chamfer overrun's sliver refuses FOOT_INSIDE_A_FACE where the far band's trimline meets the near edge past its far end
status: open
opened: 2026-10-12
---

A chamfer turn's overrun is metered as the far band `k`'s whole cut-off
sliver on the near band's face `Fⱼ` (`crates/sweep/src/blend/open/planar.rs`
`turn_plan`, through `end_face.rs` `EndCut::plan`). That sliver's feet
are `p_k` on L and `E`, band `k`'s trimline on the shared face `S` met
with the near band's requested edge `eⱼ`; `EndCut::plan` runs
`foot_param` on both, so `E` must lie strictly inside `eⱼ`'s span.

`|VE| = d / sin θ_S`, `θ_S` the shared face's angle at the vertex, while
band `j`'s own strip on `S` needs `eⱼ` only past the trimlines' crossing,
whose projection on `eⱼ` is `d·(1 + cos θ_S) / sin θ_S`. Past `θ_S = 90°`
the first exceeds the second by `−d·cot θ_S`, so an `eⱼ` between the two
(at `θ_S = 150°`: between `0.27d` plus its far station's setback and
`2d`) builds band `j` yet refuses the overrun `FOOT_INSIDE_A_FACE` at
the vertex, a refusal no geometry of the overrun calls for (the overrun
lies between `p_j`, `q` and `p_k`, far from `E`). Derived, not executed:
no fixture in the suite has so short a near edge, and at such lengths the
predicate-2 screen may refuse first.

The fix is a sliver whose `eⱼ`-side rim piece is read on `eⱼ`'s line
rather than its span (the region `Ω` is a closed form over the line), or
the overrun's own triangle as the metered region.
