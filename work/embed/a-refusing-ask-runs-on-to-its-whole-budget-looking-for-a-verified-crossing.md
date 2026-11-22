---
id: a-refusing-ask-runs-on-to-its-whole-budget-looking-for-a-verified-crossing
kind: issue
title: An injectivity ask that has met a refusal keeps walking to the end of its budget for a verified crossing, so refusals are 45% of the gate's measured price and the dearest asks in the suites are refusals on 4 to 7 face bodies
status: dispatched
opened: 2026-11-18
priority: P1
cost: M
design: true
refs: [the-at-rest-gate-certifies-embedding]
pr: 7319
branch: embed/first-finding
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

## Weighed (2026-11-21)

Two designers weighed this over four rounds (fork-log row 296), with
the gate lane's measurement of where a refusing ask's seconds go
(`origin/embed/gate-measure`, `measure/gate/walk-rows.tsv`; the tables
are on `embed/gate`'s copy of this row under Evidence). They end
agreed, and they correct the finding above:

- **The seconds are spent reaching refusals, not running on after
  them.** On the 13 dearest refusing asks (4,434 s) the first refusal
  comes within 3 splits and the pair that refused spends nothing more;
  4 to 21 other face pairs then refuse in their turn, 3,250 s of it
  before each reaches its own first refusal. On the cheap asks the
  first refusal comes at the depth floor and 71 to 93% of the time
  precedes it.
- **What runs after a refusal is the whole proof**, not a search for a
  crossing: local certificates on touching cell pairs, which cannot
  produce a crossing. Crossings come from a cheap probe.
- **A crossing ends the ask and a refusal does not** (`Sweep::run`), so
  a crossing found after a refusal drops the refusal, and keeping the
  first refusal while counting the rest is a fold.
- **A refusal does not settle its face pair**: in 19 of the 38 asks
  that crossed after a refusal the crossing lies in the pair that
  refused first.

**Now.** The embedding ask ends at its first finding, a verified
crossing or a refusal, and reports it. The injectivity question only:
the metric questions keep today's rule. Every refusal names the two
faces of the cell pair it was made on (`Budget`, `Sliver` and
`WitnessUnverified` carry none today).

**The engine defect this is evidence of**, to be filed and built as its
own unit: a local certificate that fails answers "split, a smaller pair
may" with no ground (`Outcome::Split`), so the engine learns that no
cell of any size will certify only by reaching the depth floor or the
budget. A split is to carry its ground, the thing a smaller cell would
reduce; a failure with none refuses that cell at once, typed
(`[one-way.readings]`: "an unlisted arm refuses typed, naming what it
lacks"). Measured first: why each touching cell splits, and whether a
child fails as its parent did.

**Later, if a refusing ask then costs about what a body of its size
costs when it holds:** the ask no longer ends at a refusal. It ends at
its first verified crossing; with none it refuses and reports every
refusal it met, each once by its faces and place. One crossing a run.

**Dropped by both:** one finding per face pair as the shape of the
answer; a face pair settled by its first finding; a change to
`ERROR-DESIGN` E7; a second, smaller budget; a probe-only search after
a refusal (near a shared edge it has no natural end).

## Ruled (Ev, PR 7309, 2026-11-21)

"sounds great! i think there is no need for DESIGN.md to say anything
about this, though, unless it explicitly says that it checks everything
now". Both steps above stand as ruled, and no design text changes:
`[topology.tiers]` says a refusal is the list of what was found, and
nowhere says the embedding question reads on after a finding.

- **The unit, now:** the embedding ask ends at its first finding
  (`Break` at each lower-bound refusal site of the injectivity
  question; the metric questions unchanged; every refusal names the two
  faces of its cell pair).
- **Later**, with `a-failed-local-certificate-answers-split-with-no-ground`
  built and measured: if a refusing ask then costs about what a body of
  its size costs when it holds, the ask ends only at a verified
  crossing and otherwise reports every refusal it met. Not asked again;
  logged when it lands.

