---
id: g-moves-four-main-reds-messages
kind: issue
title: Six rows red on main read G's numbers (four slow rows, the m10_sym_profile plate digest, the sweep thread-count digest at 1e-12); whoever greens them pins G's
status: open
opened: 2026-10-29
priority: P1
cost: M
---

Found in INTENT stage 3 G (PR 5954). The comparison is the slow set on G's merge base `a195f777` against G's head; the table is on PR 5954.

These four rows are red on main, and under G they stay red with the same message. The numbers inside the message move, though. So a fix written against main's numbers alone goes red again once it meets G's tree. Whoever greens one of them pins G's numbers:

- `sym_9_retry_interval::sym_9_the_kept_atom_ladder_recovers_what_phase_1_measured` (`crates/editor-core/tests/sym_9_retry_interval.rs:657`). Rule H moves the bracket from `[1535, 1167]` to `[1537, 1165]` without the ladder, and to `[1543, 1159]` with it, retried 0 → 6. The link moves likewise. This is the DECIDE-3 trade named in PR 5954's body.
- `sym_9_retry_interval::sym_9_the_drive_writes_the_ladders_receipt` (`crates/editor-core/tests/sym_9_retry_interval.rs:796`). The shipped ladder's leaf receipt reads `[1463, 1380, 34]` on main and `[1461, 1382, 34]` under G, against the pin `[1421, 1386, 34]`. The difference is two `carrier_matches_mapped_source` decisions on the bracket that go numeric where rule G shuts.
- `deny_unknown_fields_census::the_walk_still_sees_every_attribute_site` (`crates/test-utils/tests/deny_unknown_fields_census.rs:409`). Under G, `crates/editor-core/src/node.rs` holds 9 sites against main's 10 (`:347`), because the retired pattern payload took one with it.
- `review_pick_r2::the_certified_determinant_refuses_no_genuine_crossing_over_the_corpus` (`crates/viewer/tests/review_pick_r2.rs:302`, filed as `work/pcert/review-pick-r2-tally-moved-at-5225-azimuth-placement.md`). G adds 32 grazes, all in `kitchen_sink` (1710 → 1742). Its ring is now eight turned corners that land exactly, so the rays aimed at them meet them at the vertex they are aimed at.

**What closing it takes.** Each row is green on a tree that carries G, or its own main-red item carries G's numbers above.

**2026-10-29, nightly triage** (run 40823227622, main `5648e577eb`, with G merged): G merged without re-pinning these, and the nightly reads G's numbers at every ε. The `sym_9` ladder reads `[1537, 1155]` / `[1543, 1149]` for the bracket, retried 0 → 6. The leaf receipt reads `[1461, 1382, 34]` against the pin `[1463, 1380, 34]`. `review_pick_r2` reads `(392112, 131405, 12996, 6342)`. The nightly triage PR re-syncs `deny_unknown_fields_census` (node.rs 10 → 9, var.rs 4 → 5) and retires the two `*-part2-pattern-place2-space` gui cells G renamed to `*-place3-space`. Three more G moves are red, so this row now carries five reds. `m10_sym_profile_interval::the_forms_the_walks_build_are_pinned_per_eps_row` (`:837`): only the plate's `Early/Decision` digest moves, `9ff98070…` → `38b61e2e…`, at every ε. That is attributed by reading; it was not run. G is the only editor-core `src` change since #5966 took those pins, apart from #5976's two-line budget rename in `eval/wire.rs`. The `m4_pr8_k_probe` pinned key splits are logged on `probe-lane-content-and-naming-keys-split-from-f64-on-every-corpus-document`.

**2026-10-29, nightly triage, a sixth.** `sweep` `mass_props_are_thread_count_invariant::the_walk_matches_the_serial_golden_at_four_threads` (`crates/sweep/tests/mass_props_are_thread_count_invariant.rs:418`) is red at ε 1e-12 only. One line moves: `sym_arc_loft … chk=97/104` against the table's `chk=99/104`, and every other column holds. Bisected over first-parent merges at 1e-12: `bc30e8a3`, G's first parent, passes, and `cdef4469`, G's merge, fails. Re-cut the table at G's reading, with the two checks that stopped expanding explained.

**2026-10-30, nightly triage** (nightly [41008276467](https://github.com/evgunter/cad/actions/runs/41008276467), main `39bbdfc777`):

- Red at every ε with the 2026-10-29 messages unchanged: both `sym_9` rows (the bracket ladder `[1537, 1155]`, the leaf receipt `[1461, 1382, 34]` against `[1463, 1380, 34]`), and `m10_sym_profile`'s plate `Early/Decision` digest (`38b61e2e…` against `9ff98070…`).
- Red at 1e-12 only and unchanged: the `sweep` thread-count digest (`sym_arc_loft chk=97/104` against `99/104`).
- `deny_unknown_fields_census` is red again for a new reason. PR 6043 added three sites to `crates/editor-core/src/program.rs` (`ProgramTarget`, `TraceCrossing`, `DrawnOn`), taking it from 4 to 7. The nightly triage PR re-syncs it; that move is not G's.
- `review_pick_r2` reads `(431064, 143483, 14556, 7266)` against #6023's pin `(431064, 143421, 27618, 9216)`. That pin was never measured on #6023's tree: its pin commit, head and merge all read tonight's tally (see `pcert/review-pick-r2-tally-moved-at-5225-azimuth-placement`). The nightly triage PR re-pins it, so this row's `review_pick_r2` red is closed.

**2026-10-30, GREEN (lane `green/nightly-77-pins`)**, on main `2e1c1bff`, run locally at the ε rows the nightly reported:

- **Re-blessed** (this row explains them as G's deliberate change, the DECIDE-3 trade PR 5954's body names): `sym_9_the_kept_atom_ladder_recovers_what_phase_1_measured`, with the bracket `[1535, 1157]` → `[1537, 1155]` bare and `[1543, 1149]` with the ladder, retried 0 → 6, and the link `[954, 813]` → `[962, 805]` either way. Every move is up (`symbolic` rises), which the row's own rule re-baselines. Also `sym_9_the_drive_writes_the_ladders_receipt`: shipped `[1463, 1380, 34]` → `[1461, 1382, 34]`, and bare `[1429, 1414, 0]` → `[1427, 1416, 0]`. The bare receipt had not run on main since G, because the shipped assertion stopped the row first. Both are green at default ε.
- **Pinned at main's value, not explained** (exact-value pins, cited to this row): `m10_sym_profile_interval`'s plate `Early/Decision` digest `9ff98070…` → `38b61e2e…`, every count holding (green at default ε), and the `sweep` 1e-12 thread-count table's `sym_arc_loft validate_geometric chk=99/104` → `97/104`. Which two checks stopped seeing a whole form is still owed here. The digest is the whole ledger, so nothing ran past a stop; un-pinning owes only reading the move.

Every red this row carried is green on that branch. The row stays open for the two unexplained moves.
