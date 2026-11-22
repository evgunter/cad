---
id: a-refusing-ask-runs-on-to-its-whole-budget-looking-for-a-verified-crossing
kind: issue
title: An injectivity ask that has met a refusal keeps walking to the end of its budget for a verified crossing, so refusals are 45% of the gate's measured price and the dearest asks in the suites are refusals on 4 to 7 face bodies
status: open
opened: 2026-11-18
priority: P1
cost: M
design: true
refs: [the-at-rest-gate-certifies-embedding]
---


## Finding

From the gate lane's measurements (2026-11-18, `embed/gate` and
`embed/gate-measure`, the census run on main `362cd72fd2`; the numbers
are in the gate row's `## Census`).

Of the gate's 27,344 s over the workspace's suites, 12,173 s is asks
that refuse:
- departure at an edge 5,126 s (mean 2.8 s an ask);
- `Budget(Pairs)` 5,095 s, every one running to the full budget (mean
  65,493 splits of 65,536);
- departure at a vertex 1,352 s;
- `Budget(Depth)` 468 s;
- crossings 61 s.

**A refusing ask does not stop at its first refusal.** `Bound` keeps
the first (`lower_refusal.get_or_insert`) and the walk goes on looking
for a verified crossing until the frontier empties or the budget is
spent. The twelve dearest asks in the whole suite are departure
refusals on torus bodies of 4 to 7 faces (`germ_torus_components`,
`germ_sphere_torus_placements`, `quadric_pair_sections`): 126 to 498 s
each, 27,000 to 65,500 splits.

## What is to decide

What a refused ask owes after its first refusal. It goes on today so
that a body which truly crosses itself elsewhere reads `Violated` with
its witness, the stronger finding, where stopping would report only
`Uncertified` at the first pair that could not be read.
`[topology.tiers]` says a refusal is "the list of what it found, every
finding kept". Against that: the gate's verdict is settled at the first
refusal, and most of these seconds are spent on valid bodies an engine
row will make hold.

The candidates the measurement suggests, none weighed:
- stop at the first refusal, and report it;
- go on, but only over pairs not yet refused, with a smaller budget
  for the search than for the proof;
- keep it as it is, and let the engine rows shrink the refusals.

It is not the price fork's question (PR 7101: a finish reuses the
answers of unchanged face pairs), which is about asks that hold. Under
that ruling's per-body budget a refusal's cost is not carried at all.

## Evidence still owed

How many of the 12,173 s are on bodies that are truly not embedded
(about 12 tests in the census) against valid bodies the engine cannot
yet read (1,056), and how many refusing asks end `Violated` after an
earlier `Uncertified`, which is the case going on exists for.

## Evidence (the gate lane, per-solid census on main `c2a0c41f40`, 2026-11-18)

Over the workspace suite with the slow set: 341,880 asks, 29,187 s of
gate, of which 11,434 s on asks that refuse. An ask that ends in a
verified crossing after it has already kept a refusal (the case going on
exists for) happened 38 times, 5 s in all. Crossings of any kind cost
little: the first take's 8,727 crossing asks took 61 s. So nearly all of
the refusing seconds are spent on valid bodies the engine refuses (1,076
failing tests), not on the bodies that truly cross. Measured with
`REFUSED_FIRST` in the `GATE` rows of `embed/gate-measure`
(`measure/gate/README.md`).

## Evidence: where a refusing ask spends after its first refusal (the gate lane, 2026-11-21)

`embed/gate-measure` at `95facbf3cb` (`CAD_GATE_WALK=1`, verdicts
unchanged), the per-solid gate on the census-2 base (main `c2a0c41f40`
merged to `4ef175f079`). Each refusing ask's walk was logged per cell pair,
and its time after the first refusal was split into (a) the face pair that
refused, (b) pairs that end certified, (c) other pairs that themselves
refuse, before and after their own first refusal, and (d) cells left open
when the budget ran out. Population, named in
`measure/gate/walk-population.json`:
- the four tests behind the census's twelve dearest refusing asks
  (`germ_torus_components` ×2, `germ_sphere_torus_placements`,
  `quadric_pair_sections`), 13 refusing asks;
- 150 refusing tests drawn by shape in proportion, cheapest first
  (departure at an edge 86, at a vertex 24, `Budget(Pairs)` 19,
  `Budget(Depth)` 13, second order 8), 153 refusing asks;
- the 22 tests behind the 38 asks that crossed after a refusal (206 more
  refusing asks).

| population | asks | seconds | before the first refusal | (a) | (b) | (c) before own / after own | stopping the ask saves | settling per pair saves |
|---|---:|---:|---:|---:|---:|---|---:|---:|
| twelve dearest | 13 | 4,434 | 4.6 | 0.0 | 27.9 | 3,250 / 1,150 | 99.9% | 25.9% |
| departure, edge | 89 | 2.8 | 2.5 | 0.0 | 0.0 | 0.1 / 0.2 | 10.0% | 7.0% |
| departure, vertex | 24 | 0.7 | 0.5 | 0.0 | 0.1 | 0.0 / 0.0 | 18.9% | 0.5% |
| `Budget(Pairs)` | 19 | 41.0 | 33.3 | 2.0 | 0.1 | 0.0 / 5.0 | 17.3% | 17.0% |
| `Budget(Depth)` | 13 | 1.5 | 1.4 | 0.0 | 0.0 | 0.0 / 0.0 | 5.6% | 5.6% |
| second order | 8 | 0.2 | 0.1 | 0.1 | 0.0 | 0.0 / 0.0 | 37% | 37% |
| refusing asks of the M3 tests | 206 | 56.5 | 49.7 | 0.7 | 0.6 | 2.0 / 2.9 | 11.0% | 6.2% |

(d) was zero in every ask: no level overflowed whole; budget refusals came
at admission, cell by cell.

- **The dear asks refuse at once and then pay for other pairs.** In all
  13 the first refusal comes at depth 0 within 3 splits (departure at an
  edge). The refusing pair then spends nothing more; almost all the time
  is other pairs walking to their own refusals (4 to 21 refusing pairs an
  ask, departure and `Budget(Pairs)`). Stopping the ask at its first
  refusal saves 99.9%; settling each pair at its first finding saves 26%.
- **The cheap asks spend before the first refusal.** Their first refusal
  comes at the depth floor (median depth 40; second order at 10), so most
  of their seconds precede it.
- **M2.** Distinct refusing pairs per refusing ask, over the 372 sample
  asks: 1 in 126, 2 in 75, 3 in 32, 4 in 56, 5 to 12 in the rest. Their
  kinds: departure at an edge 406 pairs, `Budget(Depth)` 296,
  `Budget(Pairs)` 206, departure at a vertex 67, and pairs refusing two
  ways (depth and departure) 225. 38 of 136 crossing asks also hold a
  refusing pair.
- **M3, the 38 asks that crossed after keeping a refusal:** the crossing
  is in the pair that refused first in 19, in another refusing pair in 2,
  and in a pair that never refused in 17. So settling per pair keeps 17 of
  the 38 crossings and loses 21 to a refusal; stopping the ask at its first
  refusal loses all 38.
- **M4, reading on past a crossing.** The 136 crossing asks in the
  population, each run again with the walk going on past its first
  verified crossing (the verdict kept): 7.8 s stopped against 15.3 s run
  on (median ratio 3.0), every one still `Crosses`.

## Evidence: why a cell pair splits (M5, the gate lane, 2026-11-22)

`embed/gate-measure` at `c5c02311de`: every split records its reason
(class, site, shared cell), and each child inherits its parent's, so a
child failing as its parent did is read directly. Verdicts unchanged.
Population (`measure/gate/walk-population.json`): the four tests behind
the twelve dearest refusing asks (13 asks); 40 asks whose first refusal
comes at the depth floor, from 39 tests (departure at an edge 16, at a
vertex 12, `Budget(Depth)` 12); the 22 tests behind the 38 asks that
crossed after a refusal.

**M5a, the reasons.**

| reason a cell pair splits | 13 dearest | depth-floor sample |
|---|---:|---:|
| not touching: distance undecided, separation straddling the band's edge | 904,922 (97%) | 18,059 (53%) |
| touching, one part `Local` at `unread` (an edge no presentation reads, not shared) | 18,960 | — |
| touching, one part `Structural` | 4,007 | — |
| touching, `Local` in `present_plane`'s catch-all (three or more of a plane face's edges in the cell) | — | 9,746 |
| touching, `Local` in `present_plane` at a corner (lean or span sign undecided; rims sharing no end) | — | 4,739 |
| touching, `Local` in `zero_of` / `present_chart` | 501 | — |
| touching, both presented, the departure margin wholly in band | 0 | 1,282 |
| touching, both presented, the margin straddling zero | 354 | 0 |
| touching, both presented, no margin read undecided | 0 | 299 |

In the dearest asks the walk is not stuck in the local certificate: 97%
of their splits are non-touching cell pairs whose separation keeps
straddling the band's edge. These torus, ball and rod placements have
faces that come within the band of each other off their shared cells,
and the walk halves them to the floor or the budget.

**M5b, per refusing face pair.** Two criteria:

| | 13 dearest (4,236 s) | depth-floor sample (1.4 s) |
|---|---|---|
| *strict*: a reason no smaller cell changes (a `Structural` part, a margin or separation wholly in band) | 9 of 166 pairs, first at depth 11; spent below it 293 s (6.9%) | 7 of 108 pairs, depth 8; 0.3 s (21%) |
| *repeat*: every split below that depth repeats its parent exactly | 72 of 166 pairs, from depth 1 (at most 23); 3,718 s (87.8%) | 95 of 108 pairs, from depth 1 (at most 27); 0.4 s (28%) |

The repeat criterion is a ceiling, not a proof: nearly all of it is the
distance straddle repeating, and a straddling separation can still narrow.

**M5c, the 38 crossings after a refusal.** 5 were found at depths 2 to 5,
at a fresh subtree root; 33 at the depth floor (40), deep in the walk. In
6 of the 38 the crossing's lineage holds a touching split made after the
ask's first refusal. So a continuation that, after the first refusal,
probes and splits only non-touching cell pairs would still find 32 of the
38.

## Evidence: where the straddling distance pairs are (the gate lane, 2026-11-22)

`embed/gate-measure` at `1811b44d1a` (merged with main `b99f71c3da`).
Each task now carries the reasons of its first and nearest touching
ancestors that split; each distance split records whether its two faces
share a cell, and, per face pair, the least upper end of a straddling
separation and the deepest straddling split with its enclosure. The four
tests behind the dearest asks (11 refusing asks this run, 3,365 s); the
depth-floor sample from a partial run (40 asks).

| straddling distance splits | dearest (647,151) | depth-floor sample (18,040) |
|---|---:|---:|
| faces share a cell, descending from a touching pair whose certificate split | 20.5% | 53.5% |
| faces share nothing, no touching ancestor | 79.5% | 46.5% |

The two columns line up exactly: every straddle between faces that share a
cell descends from a failed touching pair, and no straddle between faces
that share nothing does.

**The faces that share nothing are centimetres apart; their carriers
cross in the cells the walk keeps.** On the dearest asks, 52 such face
pairs straddle. At their deepest straddling split (depth 16 to 40) the
separation's lower end is exactly 0 in all 52. A box trace of three of
them, followed by a certified minimisation
(`share-nothing-face-pairs-are-halved-over-their-carriers-crossing-off-the-faces`),
shows:
- each cell's box equals its sampled extent;
- the faces are 1.4 to 5.5 cm apart (torus × sphere certified at least
  1.38e-2 m);
- the 0.14 to 0.54 mm closest sampled points are carrier points in
  cells that hold the two carriers' crossing and no point of one face.

The region reader drops none of those cells: the sphere and torus region
readings read no face a spline edge bounds. So it is neither enclosure
looseness nor faces that close.

**Descendants of a failed touching pair, by its reason** (dearest; the
nearest touching ancestor): one part `Local` in `unread` 77,915, in
`present_chart` 39,555; one part `Structural` 15,260.

**Per refusing face pair, the spend under the first touching pair whose
certificate split, by its reason**

| first touching ancestor | dearest: seconds | depth-floor sample |
|---|---:|---:|
| none (the share-nothing pairs) | 2,498 (74.6%) | 28% |
| both parts `Local` in `unread` | 820 (24.5%) | — |
| both parts `Local` in `present_chart` | 32 (1.0%) | — |
| both parts `Local` in `present_plane`'s catch-all | — | 72% |

So refusing a touching pair by its reason, descendants included, would
save about a quarter on the dearest asks and most of the depth-floor
sample's; the dearest asks' other three quarters are faces that share
nothing, halved over their carriers' crossing off the faces.
