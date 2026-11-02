---
id: clearance-recomputes-a-cells-net-and-jet-per-pair
kind: issue
title: The clearance walk recomputes a cell's restricted net, its jets and its centre normal for every pair the cell is in; nothing is cached per cell
status: closed
opened: 2026-11-01
priority: P0
cost: M
closed: 2026-11-02
---

Found by DIRSUPPORT's review (2026-11-01), the review's Q6.

`crates/topo/src/clearance.rs` `support_separation` builds each cell's
`Hull` (`NurbsSurface::rect_net`, or a plane's jet) and evaluates
each cell's centre normal (`Surface::normal`, one jet) afresh for
every cell pair it is asked about. A cell sits in many pairs on a
walk's frontier, so the net and the jet are recomputed once per pair,
not once per cell. `Cell` is `Copy` and holds only its box (`enc`),
so there is nowhere to keep them.

Evidence: after DIRSUPPORT,
`editor-core` `m10_6_r1_probes_interval::a_planted_violated_reads_violated_over_a_certified_leaf`
ran at 1.0 s on the PR gate, the fast-set bar, and moved to the slow
set. Its share of the support's per-pair cost is not measured.

A per-cell cache (the restricted points and the centre normal, taken
when the cell is made, as `enc` is) would make the support's cost per
cell, not per pair.

## Re-banded P0 (it blocks CARVE's P0 spines row) and moved to CLEAR (2026-11-01)

The PRICEMEASURE run on main after the four price fixes (see
`injectivity-price-on-tour-lofts-exceeds-the-pair-budget`) found that
the pair count no longer dominates. All 14 tour bodies hold, but at 0.5–10
ms per split, which is 100–6,000× the build.

A callgrind run on `m8_14`'s one-turn, 33-station helix puts 71% of the
time in `injective::departure`. 57% of the total is `nurbs::read_net`,
which restricts the face net per cell, and `Interval::between`, the
certified blossom step, is 47% alone. The cost grows with net size: at 65
stations the helix does the same 558 splits as at 33, in 1.5× the time.

So the restricted net is recomputed per pair, in departure as well as in
the support. Caching it per cell, along with the jets and the centre
normal, is the remaining lever on the price. The door question for
CARVE's loop waits on its numbers.

## Measured with the per-cell memo (NETCACHE, 2026-11-02)

The walk's window charts and their boundary edges now memoise, keyed
by the exact bits of the rectangle, range or point:
- each cell's restricted net (`rect_net`);
- its jet (`rect_jet`);
- the point jets of the split rule's nine stations;
- each edge range's enclosure and derivative (`span_enclosure`,
  `span_deriv`).

Every receipt below is bit-identical before and after, on every body,
in two runs of each.

The question is PRICEMEASURE's: `StrictlyPositive`, the body against
itself, `DEFAULT_MAX_CELL_DEPTH`, 65536 pairs. The harness is the row's,
rebuilt as an untracked tour example. Builds are release with debug
assertions and overflow checks on (the tour's profile, which here also
built the helices), pinned to one core of a 4-core box. Each time is
the better of two runs. "Build" is PRICEMEASURE's column, re-used: no
build door reads the memo.

| body | split | certified | before | after | speed-up | build | before / build | after / build |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `lily_leaf_a` | 3410 | 1058 | 21.30 s | 2.27 s | 9.4× | 5.6 ms | 3,804× | 405× |
| `lily_leaf_b` | 311 | 104 | 2.15 s | 0.19 s | 11.3× | 2 ms | 1,077× | 96× |
| `lily_leaf_c` | 359 | 106 | 2.52 s | 0.21 s | 12.2× | 1.8 ms | 1,398× | 114× |
| `lily_sepal_a` | 1102 | 347 | 7.91 s | 0.76 s | 10.4× | 2.8 ms | 2,826× | 271× |
| `lily_sepal_b` | 1206 | 369 | 7.91 s | 0.85 s | 9.3× | 2.8 ms | 2,825× | 305× |
| `lily_sepal_c` | 1241 | 385 | 7.88 s | 0.90 s | 8.7× | 2.8 ms | 2,812× | 322× |
| Klein loop | 48162 | 14888 | 32.52 s | 12.12 s | 2.7× | 7.6 ms | 4,280× | 1,594× |
| `twisted_loft` | 9403 | 2765 | 2.76 s | 0.95 s | 2.9× | 1.4 ms | 1,974× | 680× |
| `twisted_tube` | 765 | 257 | 2.29 s | 0.33 s | 7.0× | 4 ms | 572× | 81× |
| `s_duct` | 189 | 92 | 0.89 s | 0.14 s | 6.4× | 4.8 ms | 185× | 29× |
| `nonuniform_loft` | 388 | 163 | 84 ms | 40 ms | 2.1× | 1.9 ms | 44× | 21× |
| helix, 1 turn / 33 | 558 | 220 | 2.60 s | 0.40 s | 6.5× | 14 ms | 186× | 28× |
| helix, 1 turn / 65 | 558 | 220 | 3.83 s | 0.63 s | 6.1× | 25 ms | 153× | 25× |
| helix, 2 turns / 65 | 1249 | 444 | 6.52 s | 0.98 s | 6.6× | 24 ms | 272× | 41× |

**Where the time went (callgrind).** On the one-turn, 33-station helix
with only the surface memo, `read_net` fell from 57% of the run to 11%.
That exposed the larger term behind it: 77% of the run was
`injective::edge_piece` restricting the shared edge's net
(`NurbsCurve3::span_enclosure`), per pair, over a fixed piece grid and
its bisections. Those are the same ranges from pair to pair, so the
edge memo takes most of it.

On `twisted_loft`, with every memo in place, the question's time
splits as follows:
- `split_axis`'s station jets (before their memo): about half.
- The first read of each new cell's jet (`rect_jet` under `monotone`):
  about a third. A memo cannot remove a first read.

**What remains.** The cost now scales with the number of cells, at
0.1–0.25 ms per split. The Klein loop's 48162 splits still cost
1,600× its build.

Cutting the first-read cost per cell means restricting a child from
its parent's net, not from the face's. That changes the numbers in each
net, so it is not bit-identical, and its receipts would have to be
re-derived and argued sound.

## Closed (CLEAR, 2026-11-02): PR 6288

The walk's window charts and edges keep a memo keyed by the exact bits
of each rectangle, range or point (`geom::memo`), so a cell's restricted
net, jet and point reads are computed once per question rather than once
per pair. Every receipt on the 14 bodies is bit-identical. The review's
fix pass routed every point read in the walk through the memo, which
took the Klein loop to 10.1 s and `twisted_loft` to 0.70 s; the other
rows are unchanged within noise. One FULL review (APPROVE-WITH-FIXES,
no MAJOR).

The certificate now costs 21–1,330× the build. Against the at-rest gate,
which every finished body already pays (0.1–1.8 s here, 4.07 s on the
Klein loop), it is about 1–3×. Where that price is paid is
`where-the-embedding-certificate-is-paid`, which goes to Ev; CARVE's
spines row is parked on it.
