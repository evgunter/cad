---
id: a-split-whose-notch-floor-reads-on-at-0-9-eps-ships-bodies-whose-census-escalates
kind: issue
title: A split whose notch floor is read On 0.9 ε off the plane ships pieces whose tier-3′ census escalates pm_census_ef_cut_gap at 1.4–2.4 ε
status: open
opened: 2026-10-11
priority: P2
cost: M
refs: [the-census-edge-face-cut-escalates-a-line-gap-at-a-boundary-vertex-beyond-the-edge, a-split-recharts-an-edge-at-a-vertex-read-on-off-the-plane-into-the-band]
---

Found by CLEAVE while checking that the split's witness P123 builds
sound now that the section plane is charted in its plane
(`crates/topo/src/splitting/section_loops.rs` `chord_u_ref`).

## What

`topo::split` of `meeting::apex_pyramid` over the base `(y, z)` =
(-0.3709, 0.1491) (-0.0536, -0.2641) (0.3537, -0.2767)
(-0.0022, 0.0927) (-0.0422, δ) builds two pieces that pass tier 3
(`validate_geometric`), `meeting::corners_disjoint` and both closed-form
volumes at every pose and either way up
(`crates/topo/tests/split_through_an_apex.rs`
`a_notch_floor_in_the_band_splits_sound`, δ = 0.9 ε). The last corner is
the floor of a notch, read On, so the two lobes above meet along the
edge from the apex to it.

`validate_pseudomanifold` (tier 3′, no contact records) on those pieces
depends on δ, measured at ε = 1e-9 over all five poses either way up:

- δ = 0 and δ = ±0.3 ε: `Ok` on both pieces, every cell.
- δ = ±0.9 ε: `CensusEscalated { Indeterminate }` on both pieces, every
  cell — `pm_census_ef_cut_gap` and `pm_census_ef_cross_side` at
  1.443 ε and 2.391 ε, `pm_census_ee_gap` at 2.391 ε and
  `pm_census_ee_parallel` at 1.443 ε (3 to 8 errors per piece). The
  same at ε = 1e-6 and 1e-12.

The margins scale with δ (1.60 δ and 2.66 δ), so 0.3 ε reads inside
the band's zero and 0.9 ε lands in its escalation. The base the same
test file pins as `random_bases_with_corners_in_the_band_split_sound`'s
third (corners at 0.9 ε) escalates the same way on the piece holding
them. Neither row asserts tier 3′.

## Where to look (not traced)

The census lane is `crates/topo/src/census.rs` `ef_overlap_cells`. A
margin that is a fixed multiple of the vertex's offset reads like a gap
measured off an extended line — the shape
`the-census-edge-face-cut-escalates-a-line-gap-at-a-boundary-vertex-beyond-the-edge`
names. Re-run this witness once that reorder lands. If it still
escalates, find which vertex and edge the margin is read between. Then
decide whether the census is reading a real near-touch between the
lobes, which the split would owe a contact record for, or a proxy.


