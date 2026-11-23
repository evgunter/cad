---
id: no-row-cancels-the-door-between-replay-and-discharge
kind: issue
title: No row cancels the coincidence door between its replay and a discharge, and the threaded door seam's token is unpinned
status: open
opened: 2026-10-11
priority: P3
cost: E
---

Found in review of PR 4670 (`the-symbolic-door-proves-at-one-strength-at-rest`). Two gaps in the cancellation pins:

- `crates/editor-core/src/coincide/replay.rs` `prove_row` checks the caller's token before each discharge. No row cancels between the replay and a discharge (every cancel row pre-cancels, and the replay's `evaluate` catches it), so deleting both checks keeps the suite green. A row needs a cancel that lands after the replay: for instance a document whose rows discharge in a known order, with the token canceled from inside a discharge hook, or a counting token.
- `crates/viewer/src/evalseam.rs` `ThreadProver`: running each job under a fresh token keeps `door_at_rest::the_threaded_door_seam_answers_the_newest_landing` green, since the coalescing drops the superseded answer either way. The token's reach is pinned only in editor-core (`asm2a_instantiate::a_canceled_door_stops_in_its_replay`).

Done when each goes red under its mutant.
