---
id: an-overruns-sliver-refuses-a-short-near-edge
kind: issue
title: blend: a turn overrun's sliver refuses FOOT_INSIDE_A_FACE where the far band's trimline meets the near edge past its far end
status: open
opened: 2026-10-12
priority: P3
cost: M
---

A turn's overrun, chamfer or fillet, is metered as the far band `k`'s whole cut-off
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
lies between `p_j`, `q` and `p_k`, far from `E`).

**Executed** (both reviews of PR 4926), pinned by
`crates/sweep/tests/band_turn_overrun_oracles.rs`
`a_short_near_edge_refuses_foot_inside_a_face_where_the_overrun_fits`:

- **A hexahedron**: the leaning prism at `s = 0.4` with its top tilted
  (`z ≤ 1 + 0.2x`), met with a front wall `y ≤ 0.3z + c·x`, `d = 0.1`,
  the near band's edge (top ∩ wall) shortened by moving the back face
  in. At `c = 1` (`θ_S ≈ 136.1°`): near edge `0.15` builds exactly;
  `0.13` and `0.11` refuse `FOOT_INSIDE_A_FACE`; `0.09` refuses
  `FaceClearanceUncertified`. At `c = 1.8` (`θ_S ≈ 151.3°`) it refuses
  from `0.20` down to `0.13`.
- **A cone** `φ = (60°, 90°)`, `γ = 150°` (`θ_S ≈ 138.6°`,
  `|VE| ≈ 0.151`), `band_turn_overrun_verdict::corner`, its near edge
  `e₂` cut short by the plane normal to it at `ℓ`: `ℓ = 0.12` and
  `0.14` refuse `FOOT_INSIDE_A_FACE` at the vertex while the near band
  alone builds; `0.16` builds; `ℓ ≤ 0.1` refuses `FaceClearance`
  first.

**The fillet too** (both reviews of PR 4962; pinned by
`crates/sweep/tests/band_turn_overrun_fillet.rs`
`a_short_near_edge_refuses_foot_inside_a_face_for_the_fillet_too`):
the same hexahedron at `c = 1`, near edge `0.40`, `0.30`, `0.25`,
`0.22`, `0.20`, `0.18` and `0.16` builds at the fillet oracle; `0.14`
and `0.12` refuse `FOOT_INSIDE_A_FACE` while the near band alone builds,
and the far band takes beyond the near one what it takes on the long
fixture (the fillet oracle, to `1e-10`). The far band filleted alone
refuses the same way there: the refusal is the far band's foot `E`, not
anything the overrun needs.

**The refusals are spurious.** At every one of them the far band's
half-space takes beyond the near band's exactly what it takes on the
long fixture (`vol(B ∩ Hⱼ) − vol(B ∩ Hⱼ ∩ H_k)` equal to `1e-12`), so
the true body is the ordinary overrun. The refusal's text also speaks
of "the end face's plane", which a user who requested a turn never
named.

The fix is a sliver whose `eⱼ`-side rim piece is read on `eⱼ`'s line
rather than its span (the region `Ω` is a closed form over the line), or
the overrun's own triangle as the metered region, which
`a-turns-overrun-meters-the-whole-cut-off-sliver` weighs for its own
reasons; one change can settle both.
