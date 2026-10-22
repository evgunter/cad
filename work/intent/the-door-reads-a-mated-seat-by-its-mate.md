---
id: the-door-reads-a-mated-seat-by-its-mate
kind: unit
title: the coincidence door reads an at-rest row on two poses a mate equates as structural, by the first rung, with no Sym replay
status: open
opened: 2026-10-22
priority: P0
cost: M
---


## What

The ruling (PR 5555, `[coincidence.structural]`) is that a mate is the
first rung's statement. The two poses a placement's mate equates, read
with the sense the mate states, are one construction read twice,
wherever the placement evaluates. Today the door ignores this. It
re-derives a mated seat through the `Sym` replay
(`crates/editor-core/src/coincide/replay.rs` `prove_row`, behind
`coincide::REPLAYS_PLACED_COPIES`). The replay proves mate6's seat only
from 512 terms, and the tour bench's seats not at all.

## Owed

- **The match.** An at-rest row whose two cells are the two poses a
  mate in the copy's bundle equates is `Structural` at rung 1, with no
  symbolic work. The row's relation must agree with the mate's `Flip`
  parity. A target that a value moves off the mate's plane (a standoff)
  does not match. The product's at-rest rows already carry
  `AtRestBy::Mate` (`assembly.rs` `AtRestRows::decide`).
- **The replay** runs only for rows that rung 1 leaves. It resolves
  parts at `Sym` only when such a row exists, and the switch retires.
  A row frozen by the budget names the budget in its recourse, not "one
  construction".
- **Rows:**
  - mate6's seat, the tour bench's three seats, and a top seated on a
    pattern copy each give 0 `unproven-coincidence` findings;
  - the viewer rows `tree_badges` and `story_assembly::the_windmill_story`
    run at main's times;
  - a standoff seat stays `Numeric`.
- **Ride-alongs:** `the-h-seat-probe-measures-the-frames-d-retired`, and
  the owed list in `a-mate-placed-seat-is-unproven-at-the-doors-budget`
  (§ "Owed when … turns on").
- **Closes:** `a-mate-placed-seat-is-unproven-at-the-doors-budget` and
  `placed-carriers-compare-through-their-frames`, for mated rows. Rows no
  mate states go to the census unit.
