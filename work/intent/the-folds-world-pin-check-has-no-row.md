---
id: the-folds-world-pin-check-has-no-row
kind: issue
title: The fold's own world-pin stand-alone check has no row: every door asks the same function first
status: open
opened: 2026-10-18
priority: P3
cost: E
---


Found by the second review pair on PR #5267 (R3, MINOR, a surviving mutant), left unfixed in that PR.

**What.** "A world pin stands alone in its bundle" is one function, `world_pin_stands_alone` (`crates/editor-core/src/mate/solve.rs:891`), asked of the whole bundle by `AddConstraint`, the insert door, the load door and the fold (`crates/editor-core/src/eval/wire.rs:602`). The two edit doors and the load door each have a row that goes red with their check removed (`intent_s3_c_mates::an_inserted_placement_holding_a_world_pin_and_a_mate_refuses`, `a_loaded_bundle_holding_a_world_pin_and_a_mate_refuses`, `a_world_pin_stands_alone_in_its_bundle`). The fold's call has none: its `Err` arm is unreachable through the public API, because every door refuses such a bundle first, so a mutant that makes the fold's check dead survives (R3 ran it against 243 rows).

**Why it matters, and why it was left.** The fold's check is defence in depth: it decides nothing a door has not already decided, and its `Ok(true)` arm (the pin places its copy by no map) is covered. A row would need a document built past the doors, which the crate offers no test door for.

**Options.** (a) A `#[cfg(test)]` constructor that writes a bundle without admission, and a row that evaluates it; (b) make the fold's `Err` arm an `unreachable!` naming the doors, so a dead check is a panic, not a silent pass; (c) leave it, stating the invariant at the call. Retires with the interim (`a-copy-standing-where-its-body-stands-has-no-d10-spelling`) if D removes the world pin first.
