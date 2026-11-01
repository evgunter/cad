---
id: injectivity-departure-refuses-at-depth-on-loft-cap-corners-and-twisted-seams
kind: issue
title: The injectivity check's departure arm refuses Uncertified at depth 40 on every cap-wall pair of the tour's lily blades, Klein loop and m8_14's half-turn helix, and along each seam of the tour's twisted loft
status: parked
opened: 2026-10-31
priority: P0
cost: H
blocked_on: [departure-reads-a-spline-sides-zero-to-second-order-so-a-cap-corner-outruns-the-depth-budget]
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

## Diagnosis (SEAMDIAG, 2026-11-01)

Each locus was instrumented at its depth-40 cells. Every cause is a
reader's (`crates/topo/src/injective.rs`), and no producer fix
applies, so `injective.rs` is unchanged.

- **Cap against wall** (`lily_leaf_a`, `lily_leaf_b`, the Klein loop):
  - The cell sits at a vertex of the cap where two circular arcs meet:
    the lens tip on the lily, and a wall seam on the Klein loop's
    four-arc circle.
  - The cap is unpresented there (`Structural`, then `Local` beside
    it). The carrier arm, where the wall departs the cap's plane,
    certifies only once its second-order zero-side residual passes the
    band, at depth 51–73.
  - It is the cleartail row's two-curved-edges case, not its plane
    line-edge case.
  - Fix shape and measurements:
    `departure-reads-a-spline-sides-zero-to-second-order-so-a-cap-corner-outruns-the-depth-budget`.
- **`m8_14`'s half-turn helix** no longer reproduces: every cap–wall
  pair holds at depth 6–8 on today's main. Pinned holding there too.
- **Twisted seam** (all four of `twisted_loft`'s):
  - The graph separator's normal weighs each face's heading by its
    cell's offset along the seam, and at the seams' 51° dihedral it
    leaves both faces on one side.
  - Bisecting the headings flips all four seams to `Holds` at depth
    21–24 (measured locally).
  - Fix shape:
    `departure-graph-separator-weighs-each-heading-by-its-cells-offset-along-the-edge`.
