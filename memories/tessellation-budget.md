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
`#[cfg]`; the same shape would gate `probe_stats`, which today keeps a
live per-face `env::var_os` and a 91-sample-per-triangle block),
`mesh::nurbs_cert::nurbs_cell_bounds` (the certificate assembly
reported per knot-span cell — a SECOND path, so the shipped bound stays
bit-identical), and `tools/tess-lint` (report + regression gate,
`k-lint`'s posture). CI runs them in the `k-lint` job, plus a
`--features budget` row for the armed half; the committed baseline is
`docs/tess-budget-data/`.

## The findings, so they are not re-derived

Full writeup: `docs/TESS-BUDGET.md`. The headline, at the head where
the instrument was written:

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
