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

## Weighed (2026-11-21)

Two designers weighed this (fork-log row 296). Their first reports agree,
and both correct the finding above:

- **The seconds are not spent looking for a crossing.** What runs after
  a refusal is the whole proof, local certificates on touching cell pairs
  included, and a touching pair is never offered to the verifier.
  Crossings come from a probe at the top of each subtree and are cheap.
- **A crossing ends the ask and a refusal does not** (`Sweep::run`: a
  verified violation breaks the walk; a refusal is kept with
  `get_or_insert` and the walk goes on). So whether a second finding is
  seen depends on which kind came first, and a crossing found after a
  refusal drops the refusal.
- **A face the engine does not read refuses the whole ask before any
  pair is read** (`Side::windows`), so "it goes on so that a crossing
  elsewhere is found" is not a rule the code keeps.
- **About 5,100 s of the first census's 12,173 s of refusals are asks
  whose only refusal is the budget.** No stopping rule touches those:
  they are the engine rows' or the constant's.
- **Keeping the first refusal and counting the rest is a fold**, which
  `[topology.tiers]` rules out ("every finding kept, none folded into
  another").

**The final state both recommend:** the face pair is the unit. Each pair
the gate asks ends apart, crossing or unread; a pair is settled by the
first reading that decides it and its remaining cells are dropped; the
other pairs are still asked; the gate lists one finding per pair that is
not apart; where the constant stops the ask, the pairs still open are
named. The clearance assertion keeps its single verdict.

**The alternative both call defensible:** the ask ends at its first
crossing or refusal. It is the least work and the simplest code. A body
that crosses itself can then be reported as "the kernel cannot read these
two faces" whenever an unread pair comes first, and of several unread
pairs one is reported, chosen by face order.

**For the unit, under the pair:** `Bound.lower_refusal` becomes a state
per candidate pair; the `get_or_insert` sites become one place a pair is
closed; a closed pair's queued cells count as abandoned; `overflow`
closes every pair still on the frontier; `Side::windows` answers per
face; `EngineRefusal::Budget` and `Sliver` gain faces. Pair closing is
the injectivity question's alone: for a metric lower bound one face pair
can be the whole assertion.

