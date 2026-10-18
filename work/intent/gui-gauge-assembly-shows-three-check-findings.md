---
id: gui-gauge-assembly-shows-three-check-findings
kind: issue
title: the viewer opens the corpus gauge assembly (04c71837…) with 'checks: 3 finding(s)' in red, which the 2026-10-07 render did not show; nobody has read what the three are
status: closed
opened: 2026-10-15
priority: P2
cost: E
closed: 2026-10-16
---


## What

render.yml run 38816108307, lane `viewer gui montage`, re-rendered on PR 5205 (`demos/renders-gui/montage-gui.png`, the gauge cell: a desk with a box on it, the document `04c718374cb49ae558c3d9f6b5fce9f6`). The status bar now reads `at rest: certified (3 declaration(s))` followed by **`checks: 3 finding(s)`** in red. The montage committed from the 2026-10-07 render shows the same document with no checks readout. The geometry is identical between the two renders. The feature list gained `PlaceInWorld` nodes (12 nodes, was 8), so the findings arrived with the placement nodes from INTENT's stage work.

Not read: which checks fire, at what severity, and whether they are right (a placement the corpus document should declare and does not) or wrong (a check misreading `PlaceInWorld`). The tessellation-budget sweep's assembly scene logs `enforce_checks refuses: 1 check finding(s) at Error severity` on the same corpus family (nightly 61, `k-lint (release-budget)`). That may or may not be the same cause.

## Owed

Open the document headless (`run_checks` over it) and name the three findings. Then fix the document or the check, whichever is wrong. If a finding is intended, say so in the corpus so the GUI's red is expected.

— (CIW orchestrator, from the gui montage re-baseline)

## 2026-10-16 — closed: the three are expected until stage 4 PR H (INTENT, `intent/j-follow-up-reds`)

The document `04c71837…` is the tour's bench (`demos/tour/src/assembly.rs`
`bench`): two posts and a shelf seated by two mates, and a crate mated
onto the shelf. Read headless, its `run_checks` report has exactly three
findings. Each is `unproven-coincidence` against one of the three mates
(nodes 17, 20 and 31), with recourse `Assert` and replay `NotReplayed`.
Each finding's row is the at-rest census's `CensusAtRest` row for its
mate's own seat: two caps of the posts and the shelf, then the shelf's
top and the crate's bottom.

D10 makes a mate-placed face structural ("Contact between copies is an
`unproven-coincidence` finding unless it is structural (a mate-placed
face is)", ratified 2026-10-03). The door does not prove it yet: it
replays no census row, and no rung reads a placement's frame. That rung
is INTENT stage 4 PR H, `placed-carriers-compare-through-their-frames`
(parked on stage 3 C). `mate6_gather_mints::a_mated_record_cites_the_census_row_the_lint_reports`
already pins this state on purpose ("since no rung proves two placed
copies' faces one construction yet").

So the document and the check are both right for today's door, and the
GUI's red is expected. The bench's builder now says so beside `bench`,
naming H. The montage's red clears when H lands. The release-budget
sweep's `enforce_checks refuses: 1 check finding(s) at Error severity`
on an assembly scene is likely the same class, but it was not read here.

## 2026-10-18 — after stage 4 H

The door now replays each seat (H, `placed-carriers-compare-through-their-frames`): the three findings report replay `Numeric`, not `NotReplayed`, and recourse `OneConstruction`. They are still expected. A mated seat stays unproven within the door's budget while the parts' frames hold component symbols, so the montage's red clears with `a-mate-placed-seat-is-unproven-at-the-doors-budget`, not with H. The mate6 row is now `a_mated_record_cites_the_census_row_the_lint_replays`.
