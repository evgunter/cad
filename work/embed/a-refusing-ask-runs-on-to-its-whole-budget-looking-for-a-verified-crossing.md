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
