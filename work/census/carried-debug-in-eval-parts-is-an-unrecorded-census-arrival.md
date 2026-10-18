---
id: carried-debug-in-eval-parts-is-an-unrecorded-census-arrival
kind: issue
title: editor-core's eval/parts.rs gained a hand-written Debug (Carried<T>) that IMPL_FILES_TODAY does not record; the slow-set census row is red on main
status: closed
opened: 2026-10-18
closed: 2026-10-18
refs: [5289]
priority: P2
cost: E
---


`crates/test-utils/tests/hand_written_impl_census.rs:748` (`the_walk_still_sees_every_file_holding_one`) is red on main.
- **The arrival.** The walk now finds `crates/editor-core/src/eval/parts.rs`, which `IMPL_FILES_TODAY` does not list.
  - The file holds `impl<T: Decide> core::fmt::Debug for Carried<T>` (`crates/editor-core/src/eval/parts.rs:907`).
  - The wire program's part census added it (`4788844956`, `439a4944af`, `6e7e80608c`).
- **Why main's CI missed it.** The row is in the slow set (`.config/nextest.toml:68`), which CI's PR filter skips.
- **Found by** INLANE's local full-suite run on PR 5287. That PR does not touch the file.
- **Reproduce:** `cargo nextest run -p test-utils --profile ci --ignore-default-filter -E 'test(=hand_written_impl_census::the_walk_still_sees_every_file_holding_one)'`.

**Fix.** This is the census's arrival check doing its job. The owner of `Carried` reads what its hand-written `Debug` prints and omits (the row above the assertion says what an arrival owes), then adds the file to `IMPL_FILES_TODAY`.

**Closed by PR 5289** (the INLANE orchestrator's). It read the arrival: `Carried<T>`'s `Debug` reads `self.0`, a newtype index the census answers itself. It recorded `crates/editor-core/src/eval/parts.rs` in `IMPL_FILES_TODAY`.
