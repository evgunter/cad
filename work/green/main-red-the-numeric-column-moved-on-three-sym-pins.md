---
id: main-red-the-numeric-column-moved-on-three-sym-pins
kind: issue
title: Main red: the numeric column moved on the plate, the annulus and the pad past the ceiling, and the plate's Plain/Decision digest, on main at 025822bc
status: open
opened: 2026-11-23
---


Run on a worktree of `origin/main` at `025822bc` (default ε, `editor-core`'s
`all` binary), three slow-set rows red, each by a moved NUMERIC count
(symbolic and frozen unmoved):

- `sym11_exact_channel_rows::sym11_the_exact_channel_never_contradicts_past_the_ceiling`
  (`crates/editor-core/tests/sym11_exact_channel_rows.rs:230`), at ε 1e-9:
  plate `[979, 1051, 36]` against stored `[979, 1047, 36]`, annulus
  `[511, 550, 36]` / `[511, 548, 36]`, link `[786, 945, 2600]` /
  `[786, 943, 2600]`, bracket `[1367, 1550, 57]` / `[1367, 1546, 57]`, pad
  `[1483, 1711, 1967]` / `[1483, 1709, 1967]`: +4, +2, +2, +4, +2 numeric.
- `m10_9_pins_interval::m10_9_the_pad_at_both_rule_f_dials`
  (`m10_9_pins_interval.rs:973` on main): `(1483, 1711, 1967)` against
  `(1483, 1709, 1967)`.
- `m10_sym_profile_interval::the_forms_the_plates_walks_build_are_pinned`:
  `Plain/Decision` keeps calls 979 and forms 33088 and moves its digest
  `2e82fe8f…` → `31fb2516…`; `Plain/Assertion` calls 1047 → 1051 (forms
  12224 → 12370) and `Door/Decision` calls 550 → 554 (forms 12366 →
  12512), the four new numeric decisions each walked by the check.

The numeric deltas are two per placement on the plate and the bracket
(two placements) and two on the one-placement documents, the shape PR
7088's session-box move had ("each placement asks one"); not bisected.
`boxfinish/decide-path-check` merged main at `14c412a2` and reads the
same moves at its own pins (pad `(1483, 1711, 1307)`, plate
`Plain/Decision` forms 34792 → 34919), so the move is main's and the
branch's pins wait on the re-pin this row owes.
