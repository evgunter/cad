---
name: Tessellation budget instrument
description: How to measure where a mesh's triangles go and how much of the deviation budget they bought; the #320 findings and the gate that keeps them from regressing
type: infrastructure
---

# The tessellation budget instrument (#320)

**Reach for this whenever a question sounds like "is this mesh bigger
than it needs to be".** The answer is measurable, per face, in about
four seconds over the whole tour — do not estimate it.

```sh
scripts/tess_budget_sweep.sh /tmp/b.csv
cd tools/tess-lint && cargo run -- /tmp/b.csv --top 20
```

Three pieces: `mesh::budget` (per-face meter behind the crate's
`budget` feature — gated at the MODULE boundary with a no-op stub, so
`armed()` folds to `const false` and the tessellation lane carries no
`#[cfg]`; `probe_stats` was gated to the same shape by #558, behind
its OWN `probe-stats` feature — deliberately not this one, because
`scripts/tess_budget_sweep.sh` runs the tour at `--features budget`
and a shared feature would put the falsifier's 12-sample-per-edge
resampling and its `assert!` into that release artifact),
`mesh::nurbs_cert::nurbs_cell_bounds` (the certificate assembly
reported per knot-span cell — a SECOND path, so the shipped bound stays
bit-identical), and `tools/tess-lint` (report + regression gate,
`k-lint`'s posture). CI runs them in the `k-lint` job, plus a
`--features budget` row for the armed half (and, beside it, a
`--features probe-stats` row for the falsifier's); the committed
baseline is `docs/tess-budget-data/`.

## TESS-SPAN landed the span half (2026-08-17, PR #594)

The shipped NURBS schedule is now a **per-v-band tensor** from
`NurbsCellGrid::band_schedule` (one derivation, consumed by the lane
AND the meter's prediction — the `agree` column verifies the lane's
realisation): per-band `(nuc, nvc)` through the UNCHANGED `grid_steps`
point selection; bands subdividing `u` at realized aspect >
`SAFE_ASPECT = 5` plus their ±1 neighbours snap `nuc` to the
whole-patch count. THE LESSON PAID FOR SIX TIMES (each variant's
failing face + certificate in asm/tess-span's commit messages): an
anisotropic lattice strip admits a Delaunay-legal sliver of cert
~`(aspect²+1)/8·δ_s` beside ANY off-lattice point (trim chords, band
interfaces' foreign columns, anchor tops, refinement centroids — no
local insertion converges), and the OLD lane was safe only because
chords and grid shared the whole-patch steps (accidental phase
alignment). The snap restores that alignment exactly where it is
load-bearing; the chord pass keeps whole-patch steps (D-2 safe arm,
now deliberate). Results: leaf_a 3.35x fewer triangles, tour NURBS
cells 390,100 → 158,444 (held 2.46x of the measured 2.5x span share),
NURBS share of the mesh 68% → 33%, worst cert 0.60·δ. Meter columns
re-derived (`grid_cells`/`patch_cells` counterfactual/`span_cells`
prediction; report prints held/agree/split/total); baseline re-cut.
The SPLIT half (aspect policy) stays open — docs/TESS-SPLIT-SPEC.md.

## The findings, so they are not re-derived

Full writeup: `docs/TESS-BUDGET.md`. The headline, at the head where
the instrument was written (PRE-TESS-SPAN record):

- 64 NURBS faces (6.2%) carry 68% of the tour's 1.15M triangles.
- 390,100 grid cells used against 44,457 the SAME certificates admit —
  **8.8x over the whole tour**, 17.0x on #320's lofted leaf.
- The biggest factor is NOT the leaf. It is the **u/v split**: the
  schedule reaches the certificate's constraint ellipse through
  `2·a_u·a_v ≤ a_u² + a_v²`, which charges the cross term to both
  directions and grids a RULED wall ~70 ways across the direction it is
  straight in. ~4x on every NURBS wall in the tour, the well-behaved
  swept blades included.
- #320's own hypothesis (whole-patch sup vs per-span) is real and
  second: 3.8x on the leaf, ~1.0x on uniform walls.

## The caveat that keeps the split number honest

The cheapest split is a STRIP (`70 × 328` → `1 × 4905`, parameter
aspect ~5e3). It certifies; nothing downstream has been asked whether
it minds. So `opt_cells` is an UPPER BOUND on a practical schedule, and
capping aspect properly needs the first fundamental form (parameter
aspect is not 3-D aspect). Also: `nurbs_cert`'s grid steps are shared
with the chord pass's adjacent-face boundary tightening, so a schedule
change is not local to the grid.

## The gate's rule

It compares DIFFERENCES against the committed baseline (a scene grew, a
face's sizing got wastefuller, a scene vanished) — never absolute
slack. Absolute thresholds here could only be met by coarsening δ or
simplifying geometry, which destroys the measurement. Re-cut the
baseline when a growth is intended, and say why.
