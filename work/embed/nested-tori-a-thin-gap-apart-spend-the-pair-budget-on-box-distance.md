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
pair at the root. `separated_by_support` is tried on these pairs too,
after their boxes (the 2026-11-18 section below measured it: an earlier
copy of this sentence said it ran only where the pair shares a cell),
and does not part them at the depths reached. This lane's reading of
why, not a measurement: a curved cell's support along a direction is
loose to second order in the cell's size, so it parts the pair only
once cells are small against `√(gap · radius)`, and the pairs to be
decided grow with the faces' area over that. A
thin-walled shell of any curved body (a pipe, a hollow ball) will meet
the same cost; these three rows are the measured instance.

## Landed and measured (lane `embed/nested-tori`, 2026-11-18)

Main `ff2319654`, f64, the cavity asked through the local
`CavityGate` switch of `origin/embed/cavity-trace-measure`
(uncommitted here).

**The reading.** The kernel already reads this fact: the census parts
a cross-solid pair no direction separates by its depth gap
(`census::depth_gap`: where one face's greatest distance from an axis,
a centre or a torus's core circle lies below the other's least, read
off the two faces' covers). `topo::clearance` now asks that reading of
a face pair's root (`apart_in_depth`), at the support arm's site
(`CLEARANCE_SUPPORT_GAP`), against the bound asked: a gap bounds the
two faces' separation from below, so the root discharges whole. It is
one reading (`[one-way.readings]`), not a second: a first draft of
this lane read a carrier's distance function over a cell in
`clearance.rs`, and the cover gate refused it.

**The three rows shell.** Each cavity of four torus faces holds in
1,358 splits (65,526 and a refusal before): 679 for each pair of
halves that touch, and none for the four inner-against-outer pairs
(24,823, 24,823, 13,919 and 13,591 before, to the overflow).
`cross_solid_apart`'s second shell asks one more cavity, of eight
torus faces, which holds in 3,148.

**The cavity rows** (the same 222 tests, before and after): 14 fail
before, 11 after. The three nested-tori rows pass. The other nine
valid rows refuse as before (six at the pole, the lune, two at a
window), and the two crossing pins still read `Crosses`. No verdict
moved the other way. Over the 302 cavities asked, the splits fall
from 1,121,010 to 644,137; besides the tori, the cylinder cavities
with coaxial walls now discharge those pairs at their roots (a
40-face one from 52,518 splits to 270).

**`self_intersection_proves_injectivity`**, the whole module at the
default ε, 1e-6 and 1e-12 (49 rows before, 50 after): every row
passes at every ε before and after, and the receipts are the same at
the three. Four moved, at f64 and Interval alike: the thin cylinder
shell 2,355 splits to 0, the toroidal wall 6,647 to 200, the stepped
ring 356 to 180, the drum's cavity 994 to 972. The conical wall is
unchanged at 3,099
(`coaxial-cone-walls-a-thin-gap-apart-have-no-depth-that-parts-them`).

**The tour walk** (85 bodies, the harness of
`origin/embed/conic-vertex-measure`): 82 hold before and after, the
three bench assemblies cross as before, and no verdict moved.
Candidates 6,929 both; discharged 479,866 to 311,721; splits 473,162
to 305,017. 22 bodies split less and none more. The ten largest:

| body | splits before | after |
|---|---:|---:|
| `hollowtorus` | 31,162 | 1,582 |
| `hollowring` | 27,546 | 1,582 |
| `torusvessel` | 27,809 | 10,289 |
| `torusvesselcup` | 27,231 | 10,289 |
| `diecomposed` | 45,271 | 29,320 |
| `klein_bulb` | 23,531 | 8,361 |
| `hollowelbow` | 9,974 | 368 |
| `fivewallcup` | 16,806 | 7,290 |
| `teapotvessel` | 8,452 | 1,305 |
| `fivewall` | 15,022 | 7,935 |

**The lofts and the helix do not move**: the Klein loop 51,302 splits,
`lily_leaf_a` 3,410, `lily_sepal_a` 1,102, and the module's loft and
helix rows keep their receipts. A spline wall proposes no site, and a
wall against itself has no gap
(`the-depth-gap-is-asked-of-whole-faces-not-of-cells`).
