---
id: resolve-bisector-graze-reads-the-other-solids-face-about-the-line-by-side-codes
kind: issue
title: recl::resolve_bisector_graze reads a wide sector's two halves in the other solid's face's half-turn by side codes, beside the line cone
status: open
opened: 2026-11-02
priority: P3
cost: M
---


Filed by `resolve-edge-sector-reads-the-other-solids-face-about-the-line-by-table-ii`'s sweep (branch `oneread/edge-sector-line-cone`), which made `recl::resolve_edge_sector` read the other solid's face's half-turn as a line cone (`boolean::sectors::half_turn`, `line_fans`, `link::Cone::reads_tied`).

## What

`crates/topo/src/boolean/recl.rs` `resolve_bisector_graze` decides a ray that is only a subdivision bisector of one solid's wide sector (no real edge of either solid on it), lying in a face of the other's (the reference sector found by `find_ref_sector`, or the other's own bisector twin). It reads the wide sector's two halves' outer bounds against the reference face's plane by first-order side codes (`flank_key`, through `side_code`), and calls the sector crossed iff the two keys transition. That is the same fact `resolve_edge_sector` now reads through the line cone: which of the two flanks about the ray lie in the other solid's material about it, the reference face's half-turn. Here the two flanks are one face's halves, not an edge's two faces. It is a second reader beside the cone, and `flank_key` / `side_code` survive in recl only for it.

Not yet weighed: its `k1 == k2` refusal (`bisector_zero_refusal`, `bool_sector_bisector_side`) argues the bisector's own On at K > 2. Under the cone the bisector reads `Boundary` or `On`, and that refusal's argument would have to be restated or retired.

## Owed

Weigh reading the two halves' outer bounds through `half_turn` + `line_fans` + `Cone::reads_tied`, as `resolve_edge_sector` does, with a decision diff at every call. Otherwise say why the graze reads something the cone does not.
