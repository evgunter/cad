---
id: g-moves-four-main-reds-messages
kind: issue
title: INTENT G (PR 5954) moved six slow rows on main; all are now pinned or re-blessed, and two moves stay unexplained: R2's rounded pad loses two symbolic decisions outside the bracket-only DECIDE-3 trade, and the m10_sym_profile plate digest and sweep thread-count line moved without a stated cause
status: closed
opened: 2026-10-29
closed: 2026-10-30
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

- **Re-blessed** as G's deliberate change: `sym_9_the_kept_atom_ladder_recovers_what_phase_1_measured`. PR 5954's cost table has rule H proving the bracket +2 and the link +8. The row reads the bracket bare `[1535, 1157]` → `[1537, 1155]` and the link `[954, 813]` → `[962, 805]` both ways. With the ladder the bracket reads `[1543, 1149]`, +8, and retries 0 → 6. That is the DECIDE-3 trade PR 5954 rules: rule G under H costs `carrier_matches_mapped_source` four decisions per attempt, the ladder recovers them and two more, and the exemption from never-loses names that one predicate on the bracket alone.
- **Pinned at main's value, not explained** (exact-value pins citing this row):
  - `sym_9_the_drive_writes_the_ladders_receipt`. Its document is R2's rounded pad (`pad_first_leg_by_angle`), not the bracket, and PR 5954's cost table shows no proof change for the pad. Yet the pad loses two `symbolic` decisions on both dials: shipped `[1463, 1380, 34]` → `[1461, 1382, 34]`, bare `[1429, 1414, 0]` → `[1427, 1416, 0]`. The bare value was measured on main (this lane's run, with only the shipped pin moved); no nightly reached it past the shipped red.
  - `m10_sym_profile_interval`'s plate `Early/Decision` digest, `9ff98070…` → `38b61e2e…`, every count holding.
  - The `sweep` 1e-12 thread-count table's `sym_arc_loft validate_geometric chk=99/104` → `97/104`.

  None of the three stops a row early, so un-pinning owes only an explanation of the move.

**2026-10-30, for INTENT.** The pad's two lost `symbolic` decisions under G are unexplained: G's cost table claims nothing for the pad, and DECIDE-3's exemption names the bracket alone. This row stays open for them, beside the plate digest and the `sym_arc_loft` checks.

## Re-homed to INTENT (2026-10-30, GREEN)

GREEN pinned or re-blessed every red this row lists (#6093), so main is
green on them. What remains is the cause, which is INTENT's: the pad's
loss of two `symbolic` decisions in `sym_9_the_drive_writes_the_ladders_receipt`
(pinned at main's value as not explained), the plate digest, and the
thread-count `chk 97/104` line. Moved to INTENT per GREEN's charter (the
fix row goes on the program that owns the code that moved). Un-pinning
each owes the re-measure its pin site names.

## Closed (2026-10-30, INTENT)

Each move was measured on G's merge `cdef4469` against the same tree
with rule H shut (`SymRules::without_angle_sum()`). The shut reading
matched G's first parent `bc30e8a3` every time, which was also re-run
for the `sweep` row. So rule H (`crates/geom-core/src/sym/turn.rs`)
accounts for all three moves. The probes were temporary and none is
committed.

- **R2's rounded pad, `symbolic` -2: a defect, filed as
  `work/sym/rule-h-loses-two-pad-decisions-to-a-coefficient-freeze.md`**
  (SYM owns `sym/turn.rs`). The bare leaf reads `[1427, 1416, 0]`
  shipped and `[1429, 1414, 0]` with rule H shut. One
  `carrier_on_surface_1` and one `carrier_on_surface_2` decision go from
  `Symbolic` to `NumericZero`. The leg's rounded `π/2` is a symbolic
  angle, so its products linearise into multiple-angle atoms. One square
  in each residual then freezes on the coefficient bound, a freeze that
  does not happen with the rule shut. This is a loss outside DECIDE-3's
  bracket-only exemption, so the pin stays and cites the SYM row.
- **The `m10_sym_profile` plate's `Early/Decision` digest: correct.**
  Rule H keys three `sin`/`cos` pairs whose argument has no
  rational-turn part by the sign-normalised argument, each `sin` read
  as `−sin(−ω)`. The 168 forms downstream carry the new keys at
  unchanged sizes, and every count holds.
- **The `sweep` 1e-12 `sym_arc_loft chk=99/104` → `97/104`: correct, at
  a cost in check coverage.** Under rule H one `Mul` node's kids carry
  coefficients of about 100 and 160 bits at this row's scale, so their
  product passes the 256-bit ring and the early walk freezes the node.
  The two contradiction checks whose residual is a multiple of it then
  read frozen. No decision is lost (`sym` +2 at every row, as G pinned),
  and `disp` stays 0. Rule H also turns two of `sym_thin_strip`'s
  build-time checks from frozen to expanded, but the door's receipt
  subtracts those, so they never reach the table.

Each pin site now carries its explanation
(`sym_9_retry_interval.rs`, `m10_sym_profile_interval.rs`'s
`PLATE_LEDGER`, and `mass_props_are_thread_count_invariant.rs`'s
`expected`).
