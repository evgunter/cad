---
id: display-budget-rows-restate-three-private-constants
kind: issue
title: display_budget.rs restates three of viewer's private constants as literals
status: dispatched
opened: 2026-09-15
priority: P1
cost: E
branch: chrome/one-number-one-home
---


Found by the sweep the `probe-rows-assert-in-one-direction-only`
finding 2 fix owed (`crates/viewer/tests/` for "a row restates a
constant that lives in `src/` as a literal").

`crates/viewer/src/camera.rs` settles what this costs, above
`Camera::pitch_limit()`: a test that restates a contract as a literal
"is a hand-synced copy of a private constant — the defect this
accessor exists to remove. One home; read it". Three rows in
`crates/viewer/tests/display_budget.rs` are that copy, each with a
comment admitting it:

- `INITIAL_DELTA = 1.0e-4` (`display_budget.rs:36`) copies
  `app::INITIAL_DELTA` (`crates/viewer/src/app.rs:179`). The comment
  names the reason — the original is `cfg`-gated behind the `app`
  feature — so this one needs a door that survives the gate, not just
  a `pub`.
- `PROBE_FACTOR = 8.0` (`display_budget.rs:42`) copies `scene`'s
  private `PROBE_FACTOR` (`crates/viewer/src/scene.rs:857`), and the
  rung bound below it is read against the copy.
- `1.0e9` inline (`display_budget.rs:1135`) copies `scene`'s private
  `SCALE_PROBE_DELTA` (`crates/viewer/src/scene.rs:869`), the δ the
  scale probe runs at.

Each is a number the row's assertion depends on, so the copy going
stale does not fail the build — it makes the row assert against a
number the code no longer uses. The shape of the fix is the accessor
`camera.rs` ships, one home per constant, read by the row.

## What the sweep could not see

**This list is hand-made and nothing reds when a seventh site
appears.** It was built by matching every numeric `const` in
`crates/viewer/src/**` against its literal spelling across
`crates/viewer/tests/**`, then reading the hits. Three blind spots,
stated because a census without them reads as a guarantee:

- **A literal spelled differently is invisible.** `8.0` finds `8.0`,
  not `8f64`, `8.` or `4.0 * 2.0`, and nothing finds a literal that is
  an arithmetic CONSEQUENCE of a constant — which is the class the
  `valid_range.rs` instance was in, found by reading a row that named
  it rather than by any pattern.
- **`#[cfg(test)]` modules under `crates/viewer/src/` were not
  looked at** — nine of them. The scope sentence above says
  `crates/viewer/tests/`, and that is exactly what it means.
- **Other crates' suites were not swept at all.** The shape is not
  viewer-specific and this row makes no claim about them.

A second sweep of a different shape, run by this unit's reviewer over
`crates/viewer/tests/`, turned up no fourth instance there, so the
population above looks right for the directory it covers.

**Not every restatement is this defect, and three in the same suite
are the argued opposite** — recorded here so a lane fixing the three
above does not "fix" them too: `edge_pick.rs:435` spells its own
`1e-6` band DELIBERATELY rather than importing `OCCLUSION_SLACK_REL`
("would make the row agree with the code by construction"),
`datum_draw.rs:546` states the arms structurally rather than against a
copy of `FRAME_ARM_PX`, and `gesture_table.rs:64` refuses to restate a
41-row table. A constant a row DEPENDS on wants one home; a threshold
a row CHOOSES is the row's own.

## Home

CHROME owns `crates/viewer`. `work.py territory` reports
`crates/viewer/src/app.rs` and `crates/viewer/src/scene.rs` as
"chrome, view" and `crates/viewer/tests/display_budget.rs` as
"chrome, tcost, tint, view" — double claims, not crossings, so
**CHROME co-owns every file this row touches** and the accessor half
is dispatchable here rather than waiting on anyone. A lane taking it
announces the overlap to VIEW, which is what a double claim is for.

## Measured

All three copies are gone; the suite reads each number from its one
home, and the three homes are not spelled the same way, because the
three numbers are not the same kind of thing.

- `SCALE_PROBE_DELTA` is a **value** the row has to run at, so it is
  `pub const` beside the `TRIANGLE_BUDGET` it sits with.
- `INITIAL_DELTA` needed a door that survives the `app` gate rather
  than a `pub`: the value itself moved out from behind the gate into
  `scene`, beside the δ vocabulary it belongs to, and `app` reads
  `scene::INITIAL_DELTA`. Nothing is `cfg`-gated any more, so there
  is no second spelling to keep in sync.
- `PROBE_FACTOR` **stayed private.** What the row wants is not the
  factor but the bound it derives — *a rung is placed at
  `TRIANGLE_BUDGET / PROBE_FACTOR` triangles* — so `scene` exposes
  `placed_rung_cost()` and keeps the ingredient to itself, which is
  what `Camera::pitch_limit` does with `POLE_MARGIN`.

### Reading the placement is not agree-by-construction

This needed deciding rather than assuming, because two hundred lines
away this same suite argues the opposite about `reads_back_as_a_delta`
(below). The distinction is **where the two sides of the comparison
come from.** `reads_back_as_a_delta` would derive BOTH sides from
`MM_PER_METRE` — the render forms `d * MM_PER_METRE` and the
expectation would too — so a wrong constant passes and the row is
void. The placement row compares `placed_rung_cost()` against a rung's
**measured** triangle count, tessellated from a real body, which is
not derived from `PROBE_FACTOR` at all.

Measured, not argued: dividing `fit_delta`'s probe placement by eight
(`PROBE_FACTOR * constant / (8.0 * budget)`) makes the row fail with
*"a single rung tessellated 234516 triangles, past the 125000 a rung
is placed at by more than the law's error"* — the assertion that reads
the door, firing on a ladder that misplaced its probe. A row that had
restated `8.0` would have held the bound fixed while the code's actual
guarantee moved, which is the stale-copy defect in its quiet form.

### What the mutations do and do not show

- `INITIAL_DELTA` 1.0e-4 → 2.0e-4 fails
  `the_budget_commits_the_delta_it_always_has` and
  `no_probe_out_tessellates_the_picture_it_sizes`. **This is the one
  that proves the fix.** Before the move no mutation of the
  application's value could reach the suite at all — the gate made the
  copy unreachable, not merely stale.
- `SCALE_PROBE_DELTA` 1.0e9 → 1.0e-6 fails
  `a_bodys_count_has_stopped_falling_by_its_own_extent`, the row that
  reads it.
- `PROBE_FACTOR` 8.0 → 16.0 fails
  `the_budget_commits_the_delta_it_always_has` — **and proves
  nothing about this fix.** It fails identically at the merge base,
  with the constant still private and the suite still holding its
  copy, because that row is pinned by the `ANSWERS` golden, which
  moves whenever the fit's answers move. The claim this bullet used
  to make was wrong. The placement mutation above is the evidence for
  `PROBE_FACTOR`; this one is a golden doing its job.

## The sweep re-run for the fix, stated as it actually ran

Value equality alone is **useless** here, and saying only that it was
value-normalised would have been a census that could not have produced
its own hit list. `crates/viewer/tests/display_budget.rs` holds **55**
literals numerically equal to `INITIAL_DELTA` and
`crates/viewer/tests/scene_build.rs` six more; the common values
(`0.5`, `8.0`, `0.001`) run to a hundred hits each. The pattern had
three stages and the last two are what made it a hit list:

1. Every numeric `const` in `crates/viewer/src/**` whose initialiser
   is a bare literal, matched against numerically equal literals in
   `crates/viewer/tests/**` and in the in-`src` `#[cfg(test)]`
   modules — so `8.0`, `8f64`, `8.` and `1_000.0` all compare equal.
   This is what closes the "spelled differently" blind spot above.
2. `0.0`, `1.0` and `2.0` dropped outright.
3. A hit KEPT only when the test-side binding's own name matched a
   src constant's name, or when a comment within seven lines named
   the constant or its module.

Stage 3 is the whole filter and it is itself a blind spot: **a copy
with neither a matching name nor an admitting comment is invisible to
it.** The three sites this row is about all had both, which is how
they were found and is not evidence that a silent one does not exist.
Stages 1–2 alone return several hundred hits, almost all coincidence.

## The largest population of the shape, and why it is not the defect

`display_budget.rs`'s `ANSWERS` table (`:504` on) restates
`INITIAL_DELTA` **56 times**, in the `requested:` and `delta:` columns
of its fifty-six rows. That is far more instances than everything else
in this row put together, and the census above was silent about it.

**It is a golden, and a golden is the argued-opposite case.** Its doc
says so: *"The budget's answer is a value, and this is the whole of
it … a row that moves is a change to what the viewer opens documents
at, and has to be argued as one."* The table's job is to fail when
the answers change, which is exactly what it did under the
`INITIAL_DELTA` mutation below — it is the mechanism that makes the
constant's change visible, not a copy that hides it. Deriving the
columns from `INITIAL_DELTA` would make the golden agree with the
code by construction and delete the row.

## The fourth site, and which kind it is

The sweep turned up one further site the earlier census did not,
because that census looked only at PRIVATE constants:
`display_budget.rs`'s `reads_back_as_a_delta` spells `1.0e3` and
`1.0e-3` where `scene::MM_PER_METRE` is public.

**That one is the argued opposite and is left alone.** The row exists
to check that `render_mm` applies the millimetre conversion; reading
the constant `render_mm` reads would make the row agree with the code
by construction, which is `edge_pick.rs`'s argument for its own
occlusion band. A comment now says so, so the next sweep does not have
to re-derive it.

`scene.rs`'s own `#[cfg(test)]` module spells `1.0e-4` as the request
it fits at. That is numerically the opening δ but does not depend on
being it — the row is about how many rungs a flat body pays — so it is
a value the row chooses, not a copy.
