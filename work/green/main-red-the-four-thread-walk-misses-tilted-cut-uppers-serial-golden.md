---
id: main-red-the-four-thread-walk-misses-tilted-cut-uppers-serial-golden
kind: issue
title: Main red (slow set): mass_props_are_thread_count_invariant::the_walk_matches_the_serial_golden_at_four_threads misses tilted_cut_upper's golden
status: open
opened: 2026-11-04
priority: P1
cost: M
parent: nightly-reds-editor-core-digests-and-goldens
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

## Greened by pinning, still unbisected (GREEN, `green/goldens`, 2026-11-14)

From nightly 43371638371 and main `b363f5fa1`. The row is green: the lines that moved are re-cut at what main reads in `crates/sweep/tests/thread-count-digest/`, and the file's `expected` doc names this row for them. Nothing here is judged, and no merge is named. What main reads against the old golden:

- `tilted_cut_upper`, every ε row. At 1e-9: `v=3fd921fb54442d18 a=40096d2e7648c6c9 vpad=3e81270c3ae55556 apad=3ea9ba9257780000` against `v=…d1b a=…6cb vpad=3e81270c3c2aaaac apad=3ea9ba925a200000`. At 1e-6 only the pads move (`vpad=3ee16a85da905556 apad=3f0a1fbd5abe4000` against `3ee16a7af0134000`, `3f0a1fb8681c6000`). At 1e-12 `v=…d17 a=…6c9` against `…d07`, `…6bc`, with smaller pads. `3fd921fb54442d18` is the double nearest π/8, so if that is the body's volume the 1e-9 and 1e-6 centres are now exact where the old golden's was 3 ulp off.
- The pads moved at least twice: on 2026-11-10 this row recorded `vpad=3e81270c3aaaaaac apad=3ea9ba9257500000` at 1e-9, and main now reads `3e81270c3ae55556`, `3ea9ba9257780000`.
- `arc_loft_1e9eps` at 1e-12: `v=3e43a1a4c860cf44` against `…cf43`.
- `sym_arc_loft` and `sym_thin_strip` `validate_geometric`, every ε row. At 1e-9: `decisions=639 sym=440 num=199 chk=136/141 frozen=643` against `610 / 432 / 178 / 99/104 / 462`, and `643 / 392 / 251 / 128/138 / 536` against `614 / 400 / 214 / 92/102 / 348`. More than the 2026-11-10 note above (611 and 615), so these moved again; PR 6820 and PR 6869 moved `sym11_exact_channel_rows`' counts in the same window and are candidates, unmeasured.

Owed, unchanged: bisect each moved line to its merge (the `tilted_cut_upper` golden line dates from before 2026-10-30), then move each finding to the program whose merge it is and judge it.

Reproduce: `cargo nextest run -p sweep -E 'test(the_walk_matches_the_serial_golden_at_four_threads)'`, and with `CAD_TOLERANCE_EPS=1e-6` and `1e-12`.
