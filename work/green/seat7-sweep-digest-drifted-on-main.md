---
id: seat7-sweep-digest-drifted-on-main
kind: issue
title: seat7_sweep_lowering's committed corner_table digest no longer matches on main
status: closed
opened: 2026-11-01
parent: nightly-reds-editor-core-digests-and-goldens
closed: 2026-11-14
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
- 2026-11-10 — measured on main at `151631cc` by ONEHOME's rigid-motion lane (PR 6717), in its own build: `boss_union` is red here too, reading `0x74fcab092e34e83b` against the pinned `0x65453fae8b1d55d3`, beside `corner_table`'s `0xc19eae46ab271546`. PR 6717 moves neither.

## Note from ONEDOOR (2026-11-11, PR 6505)

PR 6505 re-pins `corner_table` at `0x3676c700af58ca2d`. Over main's
`0xc19eae46ab271546` it moves only arena keys: with every curve,
surface, face, edge and point key masked, its document dump is main's
line for line. The first move above (PR 5848) is still unjudged; when
it is, the value to compare is the one PR 6505's dump gives with keys
masked, not the pin.

## Greened, and what is left here (GREEN, `green/goldens`, 2026-11-14)

`the_sweep_documents_evaluate_to_their_committed_digests` is green: every row is re-blessed or pinned at main's reading (nightly 43371638371, then PR 6856's id move).

- `corner_table` has left this row: its unjudged move (PR 5848) is on `work/cleavetail/corner-tables-body-moved-at-5848-and-is-pinned-unjudged.md`.
- `kitchen_sink` was PR 6820's disclosed move (`work/green/main-red-seat7-corner-table-moved-at-6832-and-kitchen-sink-after-it.md`, closed).
- **`boss_union` is what this row still holds.** It read `0x74fcab092e34e83b` against `0x65453fae8b1d55d3` and is pinned at `0x5be66936f8d10205` (that reading under PR 6856's id move). It is **not bisected**: the pin `0x6545…` was cut by PR 6463 (`5714d610e`, 2026-11-03), the row was first seen red on `boss_union` at `151631cc` (2026-11-10), and it reads `0x74fc…` at every commit from `9a256348` to `d8e1c86d6`. `m10_p_fence`'s ids-masked row holds on main, so no vertex point moved since PR 6795.

Owed: bisect first-parent `5714d610e..151631cc` on `boss_union`, move the finding to the program whose merge it is, and judge the move. The in-file PINNED paragraph names this row until then.

## Closed (GREEN, `green/goldens`, PR 6956, 2026-11-14)

Nothing is left here. `boss_union` is bisected to PR 6710 (`onecopy/edge-ends-turns`, merge `75a84e844`; green at `d3e3311b5`, the commit before) and its pinned, unjudged move is on `work/onecopy/boss-unions-body-moved-at-6710-and-is-pinned-unjudged.md`, banded and un-parented. `corner_table`'s is on `work/cleavetail/corner-tables-body-moved-at-5848-and-is-pinned-unjudged.md`. The range the section above gives for `boss_union` is superseded by that bisect.
