---
id: seat7-sweep-digest-drifted-on-main
kind: issue
title: seat7_sweep_lowering's committed corner_table digest no longer matches on main
status: open
opened: 2026-11-01
---


`crates/editor-core/tests/seat7_sweep_lowering.rs`
`the_sweep_documents_evaluate_to_their_committed_digests` is red on main at
8509df2510 (reproduced on that base by the WIRE `wire-carries` lane,
2026-11-01): `corner_table` gives `0xfa506ba6d180fc96` where the test pins
`0x8078f0efe3f11dd2`. The document has no split or shell, so PR 6268 is not
the cause. Not bisected; the cause is unknown. The fix is to find the commit
that moved it and either restore the reading or re-pin the digest with that
commit's justification, not to re-pin blind.

## Note from ONEHOME (2026-11-02, PR 6328)

PR 6328 makes `Curve3::Line` hold a `geom::Line`, so the digest feed's `{curve:?}` now prints `Line(Line { .. })`. That PR re-blessed every digest that moved by the spelling alone, verified by restoring main's spelling inside the feed at ε 1e-9, 1e-6 and 1e-12. `corner_table` is the exception: it reads `0xfa50_6ba6_d180_fc96` with main's spelling (this row's drift, which predates PR 6328 and was confirmed unchanged by it), against the committed `0x8078_f0ef_e3f1_1dd2`. With PR 6328's spelling the drifted value reads `0xc19e_ae46_ab27_1546`, which is the value to bless once this row decides main's move is correct.

Moved from `work/issues/` to `work/green/` on 2026-11-03, when the `work/issues/` bucket was retired (Ev's direction, in chat); id and body unchanged.

## Bisected (ONECOPY, `onecopy/main-reds-triage`, 2026-11-09)

First-parent at ε 1e-9, `corner_table` moved twice:
- `0x8078f0efe3f11dd2` → `0xfa506ba6d180fc96` at PR 5848 (`cleave/kept-side-in-one-place`, merge `9fc2720b5e`, 2026-10-31). Its description does not disclose it. This is this row's drift: CLEAVE owes whether the corner table's new body is right.
- `0xfa506ba6d180fc96` → `0xc19eae46ab271546` at PR 6328 (`onehome/geom-readings`, merge `0da3e56b97`, 2026-11-02): the `Line(Line { .. })` spelling of the digest feed, disclosed above in ONEHOME's note. It is a spelling only, so `0xc19e…` is the value to bless once the first move is judged.

`work/green/main-red-two-slow-digest-rows-at-8ae2b9b.md`'s seat7 half is this same reading.
