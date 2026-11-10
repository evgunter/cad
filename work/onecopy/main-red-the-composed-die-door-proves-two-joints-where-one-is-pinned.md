---
id: main-red-the-composed-die-door-proves-two-joints-where-one-is-pinned
kind: issue
title: main red (slow set): coincidence_door pins the composed die's door at one theorem of four, and main reads two
status: open
opened: 2026-11-10
priority: P0
---

Found by `onecopy/crease-at`'s slow run (PR 6709), and reproduced identically on `origin/main` `361df7ff`, that branch's base, at default ε:

- `crates/editor-core/tests/coincidence_door.rs:2084` `the_door_proves_half_the_composed_dies_joints`: `die_composed: left: (2, 4), right: (1, 4)`. The door proves two of the composed die's four replayed rows as `MarginIdentity` theorems, where the row pins one. Its meetings count (`met`, 2) still holds; the tour document's pin was not reached, since the loop stops at the first assertion.

Not bisected. A candidate is the same move as `work/tcost/main-red-the-filleted-die-moved-in-seat4-and-the-step-export-fixture.md`, since the composed die carries its fillet; that row's window is 11-03..11-07. Nothing in `coincidence_door.rs` changed on main since `361df7ff`.

Reproduce: `cargo nextest run -p editor-core --profile ci --ignore-default-filter -E 'test(/coincidence_door::the_door_proves_half_the_composed_dies_joints/)'`.

Done when the move is bisected, judged, and the pin is re-blessed (with the reason) or the regression fixed.
