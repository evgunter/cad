---
id: main-red-the-composed-die-door-proves-two-joints-where-one-is-pinned
kind: issue
title: main red (slow set): coincidence_door pins the composed die's door at one theorem of four, and main reads two
status: closed
opened: 2026-11-10
priority: P0
closed: 2026-11-10
branch: onecopy/composed-die-joints
---

Found by `onecopy/crease-at`'s slow run (PR 6709), and reproduced identically on `origin/main` `361df7ff`, that branch's base, at default ε:

- `crates/editor-core/tests/coincidence_door.rs:2084` `the_door_proves_half_the_composed_dies_joints`: `die_composed: left: (2, 4), right: (1, 4)`. The door proves two of the composed die's four replayed rows as `MarginIdentity` theorems, where the row pins one. Its meetings count (`met`, 2) still holds; the tour document's pin was not reached, since the loop stops at the first assertion.

Not bisected. A candidate is the same move as `work/tcost/main-red-the-filleted-die-moved-in-seat4-and-the-step-export-fixture.md`, since the composed die carries its fillet; that row's window is 11-03..11-07. Nothing in `coincidence_door.rs` changed on main since `361df7ff`.

Reproduce: `cargo nextest run -p editor-core --profile ci --ignore-default-filter -E 'test(/coincidence_door::the_door_proves_half_the_composed_dies_joints/)'`.

Done when the move is bisected, judged, and the pin is re-blessed (with the reason) or the regression fixed.

## Bisected and re-blessed (2026-11-10)

**The move is ONECOPY's own: PR 6710** (`onecopy/edge-ends-turns`, merge `75a84e84`).
- The first-parent bisect over main, from `a6f0b5e99c` (good) to main, lands on `75a84e84`. Its first parent `d3e3311b` passes.
- Inside the PR, the first-parent bisect over its branch lands on `4445b9cb`, "forward reductions into [0, τ) read Period (at_or_above, a new forward)". That commit replaced `lo + (t − lo).reduce_periodic(τ)` with `Period::new(τ).at_or_above(t, lo)` in four places: `chart_singularity::foot`, `pair_section`'s `CarriedComponent`, `chord_join` and `shell`.
- **Not the filleted die's move.** That one is PR 6533 (`work/tcost/main-red-the-filleted-die-moved-in-seat4-and-the-step-export-fixture.md`). PR 6482 is ruled out too: it and its parent both pass this row.

**Judged a sound improvement.** Per row, `die_composed`'s coincidence rows are identical before and after. Only its joint row's proof changes, from `Unproven { recourse: Budget }` to `Structural(MarginIdentity)`, and the row's run drops from 164 s to 8 s. Both reductions name the same copy of the angle; they differ only at the rounding edge, where a point a hair behind `lo` reads `lo` rather than `lo + τ`. So each margin is the same function. The new spelling is one the symbolic replay cancels within its budget, and a `MarginIdentity` is an exact identity, so each new theorem is true.

The tour, which the loop never reached, moves the same way at `75a84e84`: 21 of its 42 joints prove, up from 14. Its `met` (42) and the check that every unproven row is `Numeric` both hold.

**Re-blessed:** `die_composed` (2, 4) and `die_composed_tour` (21, 84). The row's doc says why. All 48 `coincidence_door` rows pass at default ε, and the re-blessed row passes at ε 1e-12.
