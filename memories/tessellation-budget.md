---
name: Tessellation budget instrument
description: How to measure where a mesh's triangles go and how much of the deviation budget they bought — the instrument, where it may live, and the sliver hazard behind the NURBS schedule
type: infrastructure
---

**Reach for this whenever a question sounds like "is this mesh bigger
than it needs to be".** The answer is measurable, per face, over the
whole tour — do not estimate it.

```sh
scripts/tess_budget_sweep.sh /tmp/b.csv
cd tools/tess-lint && cargo run -- /tmp/b.csv --top 20
```

Four pieces. `mesh::budget` is the KERNEL half and only that: the
per-face measurements nothing downstream can recover (trim box, the
cells the schedule built, the certified bounds the sizing read, the
worst certificate, the sampled deviation), behind the crate's `budget`
feature, gated at the MODULE boundary so `armed()` folds to
`const false` and the tessellation lane carries no `#[cfg]`. **The
kernel derives nothing and asserts nothing**: deviation samples reduce
to `worst_ratio` and the SUITE asserts on it. The precise claim: **no
`assert!` in `mesh` is reachable only under a feature**, so no build
flag can add a panic to the tessellation path — which is not "the path
cannot panic". `tools/tess-meter` is the consumer half (CSV schema,
counterfactual schedules, split optimizer, one row per face);
`mesh::nurbs_cert::nurbs_cell_bounds` reports certificate assembly per
knot-span cell as a SECOND path, so the shipped bound stays
bit-identical; `tools/tess-lint` is the report + regression gate,
k-lint's posture. Figures live in `docs/TESS-BUDGET.md` and the
committed baseline `docs/tess-budget-data/`, never here.

**When you gate telemetry behind a feature, budget for its CI row** —
the default rows then exercise only the inert half, and that row has
teeth only if it is unconditional. CI runs them in the `k-lint` job.

**Where instrument belongs, which a gating rule does not answer.** A
gating rule answers "does it run in shipped builds?" and is easy to
certify green; it says nothing about how much instrument the kernel
should CONTAIN. The kernel keeps only what nothing downstream can
recover from `(body, mesh, measurements)`; schema, arithmetic and prose
live with the consumer.

**The sliver hazard the NURBS schedule is built around.** An
anisotropic lattice strip admits a Delaunay-legal sliver of certificate
~`(aspect²+1)/8·δ_s` beside ANY off-lattice point — trim chords, band
interfaces' foreign columns, anchor tops, refinement centroids — and no
local insertion converges. What keeps slivers out on ruled walls is
lattice ALIGNMENT, not spacing: the shipped per-v-band schedule
(`NurbsCellGrid::band_schedule`) snaps `nuc` to the whole-patch count
where aspect demands it, and the split selection projects to an exact
patch count run to a fixpoint. **Read both aspect bounds at their
constants — `mesh::nurbs_cert::SAFE_ASPECT` and `ASPECT_CAP` — never
from a second copy**; they are different quantities and both bind.

**Two live caveats.** The cheapest split is a STRIP at a parameter
aspect nothing would choose, so `opt_cells` is an UPPER BOUND on a
practical schedule, and capping aspect properly needs the first
fundamental form (parameter aspect is not 3-D aspect). And
`nurbs_cert`'s grid steps are shared with the chord pass's
adjacent-face boundary tightening, so a schedule change is not local to
the grid.

**The gate's rule.** It compares DIFFERENCES against the committed
baseline (a scene grew, a face's sizing got wastefuller, a scene
vanished) — never absolute slack, which could only be met by coarsening
δ or simplifying geometry, destroying the measurement. Re-cut the
baseline when a growth is intended, and say why.
