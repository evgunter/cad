---
id: a-refusing-ask-runs-on-to-its-whole-budget-looking-for-a-verified-crossing
kind: issue
title: An injectivity ask that has met a refusal keeps walking to the end of its budget for a verified crossing, so refusals are 45% of the gate's measured price and the dearest asks in the suites are refusals on 4 to 7 face bodies
status: closed
opened: 2026-11-18
priority: P1
cost: M
design: true
refs: [the-at-rest-gate-certifies-embedding]
pr: 7319
branch: embed/first-finding
closed: 2026-11-23
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

**A refusing ask did not stop at its first refusal** (it does since
`embed/first-finding`, "Landed" below). `Bound` kept the first
(`lower_refusal.get_or_insert`) and the walk went on looking for a
verified crossing until the frontier emptied or the budget was
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


## Landed (2026-11-22, `embed/first-finding`)

**The three outcomes, in plain words.** The embedding check asks
whether a solid's surface passes through or touches itself anywhere
but along its own edges. It ends one of three ways: it proves the
solid clean (`Embedded`, accepted); it finds and verifies a place
where the surface meets itself (`Crosses`, "a crossing", rejected);
or it cannot decide a pair of faces and stops (`Refused`, "a refusal",
rejected). "Refusing ask" below is a run of the check that ends the
third way.

**What the code reads.** One rule, `Bound::refuse_below`
(`crates/topo/src/clearance.rs`): a refusal of the lower bound is kept,
and the injectivity question (`Bound::injective`: the strict bound over
one body) ends the walk there, as a verified crossing does, with the
frontier counted `abandoned`. A metric bound keeps its first refusal
and reads on. The policy is a named one (`AtRefusal`), read in that
one function. Six sites read the rule: a face its kind does not make
injective, a witness the verifier declines, a certificate's own
refusal, a touching pair's refused split, a terminal sliver or poison
enclosure, and a distance pair's refused split. A level that does not
fit the pair budget ends the walk at every question: the verdict
refuses and the level is counted abandoned, none of it refused, since
none was examined. `Budget`, `Sliver` and `WitnessUnverified` carry
the two faces of the cell pair they were made on.

**The order that makes a finding the first**: faces discharged by kind
in the order of the side's faces, then the frontier a level at a time,
candidates ascending by their faces' places in the side, a split's
children in its cut's order.

**Whether a body passes is the shape's; which finding a body that
does not pass is told is the walk's** (the review's measurement, taken
in as a row). The plate whose void touches its top, placed 92 ways (44
turns, a mirror, offsets, the void's base listed from each vertex, the
faces in order and reversed): no placement reads `Embedded` under
either rule; under this one 16 read `Crosses` and 76 refuse for the
budget, where main's rule read 87 and 5. A cell's box is on the
frame's axes, so the level a pair is refused at moves with the pose,
and the candidates' order is the caller's order of faces. The
question this raises for `[topology.tiers]`'s isometry sentence is
Ev's, on PR 7319.

**Measured.**
- *The dearest asks*: the four tests behind 12 of the 13
  (`germ_torus_components` twice, `germ_sphere_torus_placements`,
  `quadric_pair_sections`), through the gate on
  `embed/first-finding-measure`, before at `ad8a774ba` (main `df73fd38d`
  and the harness at `c82f64759`) and after at `1c0e98740` (the same
  and this branch). 38 asks both times, 11 of them refusing, all
  departure at an edge. Before: ten take 107.9 to 386.6 s and one
  0.38 s, 2,351 s together, 27,317 to 65,526 splits. After: 0.22 to
  0.38 s each, 2.6 s together, within 3 splits. The tests end at their
  first refused solid, so these are 11 asks, not the 12 of the gate
  lane's own table (`measure/gate/walk-rows.tsv`, 4,433 s on main
  `c2a0c41f40`, taken with the gate passing every body). The
  thirteenth (`sliver_shell_role`, 4.9 s, `Budget(Pairs)`) was not
  rerun.
- *The 22 tests whose asks crossed after a refusal*, through the gate
  before and after on that branch: 2,007 asks both times. Before: 136
  cross, 38 of them after a refusal, in all 22 tests; 58.3 s in the
  asks. After: 98 cross and none after a refusal; the 38 report the
  refusal they met first (30 `Budget(Depth)`, 3 departure at an edge, 5
  at a vertex); 52.7 s. Two of the 22 pass with the gate asking, before
  and after, the same two.
- *On main without the gate*, no test's verdict class moves. Two
  receipts move in `self_intersection_proves_injectivity`: the
  collinear-cap drum refuses one cell pair where it refused 210, and
  the one-face torus cut's ask ends at its meridian, so its caps are
  asked alone. Both re-pinned.
- *The tour walk*, 85 stops: 78 hold, four refuse and three benches
  cross, on main and here, each refusal naming the same faces and
  edge. The four refusing stops' receipts read `refused 1` where they
  read 56, 42, 56 and 76, with as many splits: their first refusal
  comes on the walk's last level.
- *`self_intersection_proves_injectivity` and `shell_cavity_embedded`*
  at the default ε, 1e-6 and 1e-12: 57 of 57 on main, 60 of 60 here
  (three rows added). Wall time 7.6 to 7.9 s on main, 7.2 to 7.4 s
  here, two runs each on an idle four-core box: no change this suite
  can show.

**What it gives up, as ruled.** A plate whose void touches its top
(`a_plate_whose_void_touches_its_top_reads_the_refusal_it_meets_first`)
read `Crosses` on the top and a wall of the void; it now refuses out of
depth on those same two faces, on a cell a rounding off the apex that
comes before the cell holding it on the same level.

**Left open**: two of the sites are reached by no built body and are
pinned at the engine
(`three-refusal-sites-of-the-embedding-ask-are-reached-by-no-built-body`).

**A body whose only refusal is the pair budget gets no early end**
(the review's note): two unit bricks 1 to 10ε apart spend the whole
budget before the refusal they end at, about 6 s an ask. The stop
saves what comes after a first refusal, not what comes before it;
that is `a-failed-local-certificate-answers-split-with-no-ground`'s
ground and the distance arm's.
