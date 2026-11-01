---
id: injectivity-departure-refuses-at-depth-on-loft-cap-corners-and-twisted-seams
kind: issue
title: The injectivity check's departure arm refuses Uncertified at depth 40 on every cap-wall pair of the tour's lily blades, Klein loop and m8_14's half-turn helix, and along each seam of the tour's twisted loft
status: dispatched
opened: 2026-10-31
priority: P0
cost: H
---


## What was found (CARVE's embedding-certificate measurement, 2026-10-31)

Each face pair asked alone (`carve_embed_scratch_tour_pairs` in
`demos/tour/tests/carve_embed_scratch.rs`, and
`carve_embed_scratch_arc_pairs` with `CARVE_EMBED_HELIX` in
`crates/sweep/tests/carve_embed_scratch.rs`, both on
`carve/embedding-certificate`, budget 8192 or 16384). Every refusal
below is `Uncertified { arm: Departure }` across the pair's shared
edge, reached when a split hits `max_cell_depth` 40. All the bodies are
embedded tour scenes or rows the suite certifies by other means.

- **A cap against a wall, about 300 splits to depth 40 (a point
  locus).** These are every cap-wall pair of the tour's
  `lily_leaf_a`, `lily_leaf_b` and Klein loop (`klein::bottle().top`),
  and two of the four top-wall pairs of `m8_14_long_turn_sweep`'s
  half-turn helix at 33 stations. The shared edge is the cap's boundary
  and the wall's `v = 0` or `v = 1` iso. Each locus is probably a
  corner where the cap's edge meets a wall seam. If so, this is the
  plane-edge case of
  `work/cleartail/departure-reads-no-curved-corner-off-its-shared-edge.md`,
  but that is not confirmed: no cell was printed.
- **Two adjacent walls of `skinned`'s `twisted_loft`, about 5000 splits,
  800–1000 pairs at depth 40 (a line locus).** These are all four
  seams. The shared edge is a `u` iso of both NURBS charts, curved in
  space, and the dihedral varies along it. The `s_duct`'s seams, which
  are not twisted, hold in 0.15 s.

The whole-body verdicts these produce: `twisted_loft` (43 s),
`twisted_tube`, `lily_leaf_b`, `lily_leaf_c`, the projectbox spring (a
6-turn, 193-station square-wire sweep, 190 s) and the half-turn helix
at 33 stations all refuse Departure.

## Home

CLEAR's injectivity arms (`crates/topo/src/injective.rs`). A loft or
sweep door that certifies embedding refuses all of these until this
is fixed. That is CARVE's `self-overlapping-spines-build-and-validate`.
