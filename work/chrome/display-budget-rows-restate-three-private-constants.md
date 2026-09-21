---
id: display-budget-rows-restate-three-private-constants
kind: issue
title: display_budget.rs restates three of viewer's private constants as literals
status: open
opened: 2026-09-15
priority: P1
cost: E
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
