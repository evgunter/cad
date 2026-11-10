---
id: main-red-the-four-thread-walk-misses-tilted-cut-uppers-serial-golden
kind: issue
title: Main red (slow set): mass_props_are_thread_count_invariant::the_walk_matches_the_serial_golden_at_four_threads misses tilted_cut_upper's golden
status: open
opened: 2026-11-04
priority: P1
cost: M
---


Filed by the ONECOPY orchestrator on 2026-11-04. Two ONECOPY lanes saw this row red, each on its own base, and neither caused it:

- `onecopy/meet-interval` (PR 6516), at base `354f9b3f`. Its run of sweep's full suite listed "`mass_props` four threads" among main's reds.
- `onecopy/p0-sheared-fillet` (PR 6533), at base `4356bfa4`, sweep slow set at default ε. The `tilted_cut_upper` golden misses, and the readings are byte-identical at head and base.

`crates/sweep/tests/mass_props_are_thread_count_invariant.rs:453`. The earlier rows on this digest are all closed: `tcost/tilted-cut-upper-pads-moved-in-the-thread-count-digest-at-1e-12`, `quad/tilted-cut-upper-digest-moved-by-5734` and `green/main-red-tilted-cut-upper-moves-in-the-thread-count-digest-at-default-eps`. So the golden has moved again and nothing covers it.

## Owed

Bisect which merge moved the golden. Then decide: either the move is an honest change of a reading, and the golden is re-pinned with that reason, or it is a regression, which is fixed at its root.

## Evidence: every ε row (ONEHOME, `onehome/session-box-fixtures`, 2026-11-10)

On `onehome/session-box-fixtures` (main at `75243665`), the row misses `tilted_cut_upper`'s line, and only that line, at all three ε rows:
- 1e-9: `v=3fd921fb54442d18 a=40096d2e7648c6c9 vpad=3e81270c3aaaaaac apad=3ea9ba9257500000` read, against the golden's `…d1b …6cb vpad=3e81270c3c2aaaac apad=3ea9ba925a200000`.
- 1e-12: the volume moves too (`v=3fd921fb54442d17`, golden `…d07`).
- 1e-6: `vpad=3ee16a7af0139556 apad=3f0a1fb8681ac000` read, against `vpad=3ee16a7af0134000 apad=3f0a1fb8681c6000`. This row was hidden until now, because its arc lofts refused `OutsideSessionBox` before the digest was read. That branch moves those lofts into the box and re-cuts their 1e-6 lines, but it leaves `tilted_cut_upper`'s line as it was, because nothing on that branch explains the move. (ONEHOME session-box-fixtures lane)

## Evidence: the `Sym` validate counts moved at 1e-9 on main (ONEHOME, 2026-11-10)

On main `57161586` the default-ε digest also misses its two `Sym` `validate_geometric` lines: `sym_arc_loft` reads `decisions=611 … num=179 … chk=100/105` against the golden's 610/178/99-of-104, and `sym_thin_strip` reads `615 … 215` against 614/214. A main build in its own target directory reads exactly what `onehome/session-box-fixtures` reads at its merge of that main, so the branch does not explain the move. (ONEHOME session-box-fixtures lane)
