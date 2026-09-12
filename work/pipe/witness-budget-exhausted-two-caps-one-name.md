---
id: witness-budget-exhausted-two-caps-one-name
kind: issue
title: WitnessOutcome::BudgetExhausted fires from two caps (segments, cells) under one name — the payload, not the type, says which
status: open
opened: 2026-09-06
---


## Finding

Filed by the PROPS `budgetexhausted-conflates-three-terminations` unit's
sweep (discipline §5): the shape swept is *one refusal variant fired
from more than one termination, naming one lever*, over every
`Budget*` / `*Exhausted` variant in `crates/*/src`. This is the one hit
that is the shape.

`crates/topo/src/chart_region.rs` raises `WitnessOutcome::BudgetExhausted`
from two terminations with two levers:

- `chart_region.rs:1981` — the segment cap (`WITNESS_BUDGET.segments`,
  128) declined before the arrangement was walked; `cells: 0`.
- `chart_region.rs:2003` — the cell cap (`WITNESS_BUDGET.cells`, 4096)
  cut the walk off; `cells: spent`.

Both reach the caller as one `ChartRegionError::WitnessBudgetExhausted
{ segments, cells }` (`chart_region.rs:779`), whose doc
(`chart_region.rs:304-318`) says which cap by a payload convention —
"`cells` is zero when the segment cap declined first, which is the
whole of that arm". So the type does not say which cap fired: a caller
reads `cells == 0` and has to know the convention to name the knob,
and the two levers are one struct constant (`WITNESS_BUDGET`) rather
than two named ones.

The consumers today match on `{ .. }` only (`census.rs:1695`,
`census.rs:2814`, `tests/census_g2_carrier.rs:399`), so nothing reads
the convention yet; the cost is the caller that will. The fix shape is
the offset-fit split's: a face per cap, each naming its lever
(`SegmentCapDeclined { segments }` / `CellCapReached { segments,
cells }`, or similar), a D2 row-1 refinement — the admission set does
not move.

## Home

`work/issues/` because the owner is disputed by the tracker itself:
`crates/topo/src/chart_region.rs` sits in no program's `paths`, and
`work/topo/program.md`'s `keep_out` names it as "S-BOOL's and
CURVED's (code-quality Track Q's fence)". Whichever claims it moves
this file.

## Re-homed to PIPE (2026-09-11, the cut in `docs/WORK-TRACKS-2026-09.md` addendum 3)

PIPE collects the topology pipeline's own rows: the two engines' shared
cores, the census arms, and what a refusal there is allowed to say. This
row is one of them.

Its class at the cut was **M** — split one variant into two named faces
plus levers; three consumers, unclaimed owner. The class is a dispatch
estimate made by reading the row against the tree on 2026-09-11, not a
verdict on the finding, and a lane that finds it wrong says so in its
PR. The id, the `track:` letter where the row carries one, and the body
above are unchanged by the move.
