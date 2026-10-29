---
id: g-moves-four-main-reds-messages
kind: issue
title: Four slow rows red on main read different numbers under G; whoever greens them re-derives at G's
status: open
opened: 2026-10-29
priority: P2
---

Found in INTENT stage 3 G (PR 5954). The comparison is the slow set on G's merge base `a195f777` against G's head; the table is on PR 5954.

These four rows are red on main, and under G they stay red with the same message. The numbers inside the message move, though. So a fix written against main's numbers alone goes red again once it meets G's tree. Whoever greens one of them pins G's numbers:

- `sym_9_retry_interval::sym_9_the_kept_atom_ladder_recovers_what_phase_1_measured` (`crates/editor-core/tests/sym_9_retry_interval.rs:657`). Rule H moves the bracket from `[1535, 1167]` to `[1537, 1165]` without the ladder, and to `[1543, 1159]` with it, retried 0 → 6. The link moves likewise. This is the DECIDE-3 trade named in PR 5954's body.
- `sym_9_retry_interval::sym_9_the_drive_writes_the_ladders_receipt` (`crates/editor-core/tests/sym_9_retry_interval.rs:796`). The shipped ladder's leaf receipt reads `[1463, 1380, 34]` on main and `[1461, 1382, 34]` under G, against the pin `[1421, 1386, 34]`. The difference is two `carrier_matches_mapped_source` decisions on the bracket that go numeric where rule G shuts.
- `deny_unknown_fields_census::the_walk_still_sees_every_attribute_site` (`crates/test-utils/tests/deny_unknown_fields_census.rs:409`). Under G, `crates/editor-core/src/node.rs` holds 9 sites against main's 10 (`:347`), because the retired pattern payload took one with it.
- `review_pick_r2::the_certified_determinant_refuses_no_genuine_crossing_over_the_corpus` (`crates/viewer/tests/review_pick_r2.rs:302`, filed as `work/pcert/review-pick-r2-tally-moved-at-5225-azimuth-placement.md`). G adds 32 grazes, all in `kitchen_sink` (1710 → 1742). Its ring is now eight turned corners that land exactly, so the rays aimed at them meet them at the vertex they are aimed at.

**What closing it takes.** Each row is green on a tree that carries G, or its own main-red item carries G's numbers above.
