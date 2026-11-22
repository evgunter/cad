---
id: nested-tori-a-thin-gap-apart-spend-the-pair-budget-on-box-distance
kind: issue
title: Two nested tori a thin gap apart, sharing nothing in M, are separated only by box distance, so three valid f64 torus cavities run out of pairs
status: open
opened: 2026-11-18
priority: P0
cost: M
---

Traced by lane `embed/cavity-trace` (2026-11-18) for
`the-f64-cavity-gate-refuses-thirty-two-valid-shells`. Main `da49c63f`,
f64, the cavity asked through a local, uncommitted `CavityGate` switch,
with `topo::clearance` instrumented (uncommitted).

## Evidence

Three valid f64 cavity rows refuse `Budget(Pairs { max_cell_pairs: 65536 })`
on one cavity, with one receipt: 52,548 discharged, 12,988 refused (the
frontier at overflow), 11,642 outside, 352 certified, 65,526 splits.
- `verbs_shell::the_full_period_torus_shells_solid_and_hollow_alike`
  (the cheapest);
- `shell7_r2_probes::e2e_a_consumer_shells_classifies_measures_and_tessellates_tori`;
- `cross_solid_apart::a_hollow_shelled_again_passes_the_census`.

The cavity is four torus faces about one axis, major radius 2: an inner
torus of minor radius 0.425 (`1v5`, `2v3`) and an outer one of 0.45
(`3v1`, `4v1`), each split in two. The gap between them is 0.025. The
first, two-face cavity in the same rows (`3v1`, `4v1`) is embedded in
682 pairs.

## Trace

**The touching pairs are not the cost.** `1v5 × 2v3` and `3v1 × 4v1`
(across `7v5`, `8v1`, `3v1`, `4v1`) certify 176 cells each and split
about 680.

**The budget goes on four pairs that share nothing in M**, inner
against outer across the gap. Each is decided by the distance test
alone, to depth 22:

| pair | distance splits |
|---|---:|
| `1v5 × 4v1` | 24,823 |
| `2v3 × 3v1` | 24,823 |
| `1v5 × 3v1` | 13,919 |
| `2v3 × 4v1` | 13,591 |

At depth 22 the cells' boxes are still about 0.035 × 0.16 × 0.12. They
reach past the outer torus, so `separation` reads `[0, …]`. These pairs
certify slowly, not never: what is decided is discharged `outside`, and
none refuses. Separating two surfaces 0.025 apart by boxes over tori of
about 34 m² each takes cells near the gap's size, of the order of 50k
per face.

**The missing reading.** A carrier separator for a pair sharing nothing.
The inner torus's implicit function (its carrier, as the departure arm
reads one) is at least 0.025 on the outer torus, so it separates the
pair at the root. `separated_by_support` is tried only where the pair
shares a cell (`sh` non-empty). A thin-walled shell of any curved body
(a pipe, a hollow ball) will meet the same cost; these three rows are
the measured instance.
