---
id: chart-v-du-drops-a-decided-zero-azimuth-harmonic-unbounded
kind: issue
title: chart_v_du drops a decided-zero azimuth harmonic from ∫ (v − anchor) du and nothing bounds what it drops
status: open
opened: 2026-10-13
priority: P3
cost: E
---


(PCERT implementer, review 4952's fix pass; filed on HONE's slate as
latent unsoundness on the split lane.)

## What

`chart_v_du` in `crates/topo/src/chord_join.rs` decides
`split_chart_azimuth_linear` on the azimuth channel's harmonic part,
`√(pa.x² + pb.x²)` levered by the radius, and on `Zero` integrates
`∫ (v − anchor) du` with `du = pl.x·dt` alone. A harmonic azimuth part
the gate admitted is dropped from the integral:

    ∫ (v − anchor)·(pb.x·cos t − pa.x·sin t) dt,
    |that| ≤ sup |v − anchor|·√(pa.x² + pb.x²)·|Δt| < sup |v − anchor|·(zero/r)·|Δt|.

Its caller `chart_island_winding` (the `WINDING_PREDICATE` decide on
`area·2r/length`) adds nothing for it, so
an area within that much of zero can be decided on a value the
integral did not compute. PR 4952 made the gate read the pair at its
exact sup (it read `|pa.x| + |pb.x|`), which admits pairs up to `√2`
larger in ℓ1; the drop itself predates it.

## Direction

Either add the bound above to the area's slack (or to `length`'s lever)
before the winding decide, or integrate the harmonic part of `du`
exactly (`v·du` is a product of two harmonics plus linear terms, so its
integral is closed-form). A row: an image with `pa.x = pb.x` just under
the gate and `v − anchor` large, whose decided winding the dropped
term would flip.
