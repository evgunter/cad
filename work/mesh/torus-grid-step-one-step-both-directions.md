---
id: torus-grid-step-one-step-both-directions
kind: issue
title: torus_grid_step sizes both chart directions off one step, costing ~65x the triangles the chord asks for
status: open
opened: 2026-08-29
github: 1260
refs: [568, 1045, 1217, 1247, 1259]
---

## From GitHub issue 1260

Opened 2026-08-29; 0 comments.

## The formula

```rust
pub(crate) fn torus_grid_step(delta_s: f64, major: f64, minor: f64) -> f64 {
    (delta_s / (3.0 * (major + 2.0 * minor))).sqrt()
}
```

One step, used for **both** chart directions. The tour's `hollowring` (R = 0.30 m, r = 0.07 m) at the viewer's δ = 0.1 mm, so δ_s = 5·10⁻⁵:

- `torus_grid_step` → 0.006155 rad → **1021 divisions each way**, and the body measures **3 984 276 triangles**.
- The per-direction sagitta wants far less: minor, r = 0.07 → `sqrt(8·δ_s/r)` = 0.0756 rad → 83 divisions; major, at the widest radius R + r = 0.37 → 0.0329 rad → 191. So **83 × 191 against 1021 × 1021 ≈ 65× in triangles**.

Put another way: the ring drawn at δ = 6.4 mm carries 62 536 triangles, which is about what a correct 0.1 mm chord tolerance needs.

## The honest caveat, up front

**The per-direction sagitta is not a rigorous bound for a doubly-curved chart.** A bilinear quad on a torus deviates by more than either direction's chord alone — the two couple, and a correct bound costs a small constant more, on the order of 2× in triangles. So the slack is large but it is not the whole 65×, and this issue is not a claim that 83 × 191 is admissible. It is a claim that something between the two is, and that the gap is worth a derivation.

## Why it matters now

It is the single biggest lever on GUI responsiveness. Everything else in this area is a workaround downstream of it:

- [#1217](https://github.com/evgunter/cad/pull/1217) made the BVH build O(n log n) — 2× on a cost that should not have been 4·10⁶ items.
- [#1247](https://github.com/evgunter/cad/pull/1247) added a display triangle budget, which coarsens δ to keep the picture affordable. Its constant's own doc says so: *"if this number ever has to be RAISED to make something look right, the fault is upstream in the sizing"*. It is a safety net under this, never its answer.
- [#1259](https://github.com/evgunter/cad/issues/1259) would move the cost off the UI thread, which hides it rather than removing it.

Fixing the sizing would move all three by roughly an order of magnitude, and would buy back picture quality the budget currently trades away.

## The care this needs

TESS-BUDGET's VERBS-TESSFOLD audit (#1045) verified `hollowring`'s baseline **"exactly against the torus grid step (`mesh::sizing::torus_grid_step`)"**. So the formula is not merely implemented, it is the reference an audited baseline was checked against — changing it moves that reference, and the re-cut has to be deliberate and recorded rather than a fold. `docs/TESS-BUDGET.md`'s own section on this ("a re-cut that FOLDS IN uncovered scenes restores coverage, it does not verify it") is the procedure.

The split schedule's aspect policy (#568) is the precedent for what a ruling here looks like: a measured slack, the options with their costs, and a named constant chosen rather than tuned.

## Home

`work/mesh/` — `mesh::sizing::torus_grid_step` sits in the `crates/mesh/*` territory glob and S-MESH's charter names sizing intent versus budget (note its keep_out reserves the tess-budget re-baseline to S-CERT until that slate closes).
