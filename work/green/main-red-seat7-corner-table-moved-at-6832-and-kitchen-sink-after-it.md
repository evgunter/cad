---
id: main-red-seat7-corner-table-moved-at-6832-and-kitchen-sink-after-it
kind: issue
title: main red: seat7's sweep digests moved again: corner_table at PR 6832 by arena keys alone, kitchen_sink in a later merge
status: closed
opened: 2026-11-13
parent: nightly-reds-editor-core-digests-and-goldens
closed: 2026-11-14
---


Found by ONEDOOR's `onedoor/crossing-slots-red` lane while bisecting
`work/onedoor/the-boolean-corpus-name-digests-are-red-on-main.md`.

`crates/editor-core/tests/seat7_sweep_lowering.rs`
`the_sweep_documents_evaluate_to_their_committed_digests` reads three
moved rows on main at `31cba239`, measured at the default ε:

- `boss_union: 0x74fcab092e34e83b (want 0x65453fae8b1d55d3)`: already
  recorded in `work/green/seat7-sweep-digest-drifted-on-main.md`.
- `corner_table: 0x189560e0ffba1a23 (want 0x3676c700af58ca2d)`: **new at
  PR 6832** (`gauge/in-arc-pairing`, merge `9a256348`). The row is green
  on `corner_table` at `9a256348^1` (`d02dacd2a`), the merge's first
  parent, and reads this value at the merge. It is the move that reddened
  the three name-digest rows of the ONEDOOR issue above: PR 6832 starts a
  vertex pair's null-edge pairing from the side the op keeps (before, where
  two crossings met, it ran in the records' order), so the boolean builds in another
  order and its cells take other arena keys. A lane probe dumped every
  node of `corner_table` on both sides of the merge with each cell read
  by its geometry (a face by sense, carrier and corner points, an edge by
  ends and curve, a vertex by point): the two dumps are identical, and no
  two cells of a body share a reading. Only `VertexKey`s differ in the
  raw tables. So the value to bless for this move is `0x1895…`. It was
  left unpinned for scope, not doctrine: PR 6505 re-pinned this same row
  for a key-only move over the same unjudged PR 5848 move, and the row
  stays red on `boss_union` and `kitchen_sink` either way.
- `kitchen_sink: 0x9e357534965a74a4 (want 0x33f517f1f2a9621a)`: **new
  after PR 6832 and not bisected.** The row is green on `kitchen_sink`
  at `9a256348` and red at `31cba239`. `lib_g16_corpus_name_digests`
  holds `kitchen_sink` on main, so its names and their keys did not
  move; the digest's body half did. No candidate is
  confirmed. (A change seen in a different document, `axis_projection`'s
  circle-edge certificates, is not evidence for this one.)

Owed: bisect first-parent `9a256348..31cba239` on `kitchen_sink`, and
judge that move before the row is re-pinned.

This row and `work/green/seat7-sweep-digest-drifted-on-main.md` are one
red test. Fold them when either is worked, so the test has one row.

## Closed (GREEN, `green/goldens`, 2026-11-14)

Greened with the rest of `the_sweep_documents_evaluate_to_their_committed_digests`:

- `kitchen_sink`: **bisected and re-blessed.** First-parent at ε 1e-9 over `9a256348..e36675c1`: green at `f162f739d`, `0x9e357534965a74a4` at `3d3fe2311`, the merge of PR 6820 (`onehome/rigid-motion`), the next first-parent commit. PR 6820 re-blessed this document to this value in `seat8_split_lowering`, off the same feed (`fixture::digest`), with its reason: a placed copy carries its source's edge certificates rather than re-deriving them, so each edge's `max_residual` in the `Debug` the digest feeds is the source's. It missed seat7's copy of the pin.
- `corner_table`: **pinned.** PR 6832's move is keys alone, as this row found, but PR 5848's move under it is still unjudged. Fix row: `work/cleavetail/corner-tables-body-moved-at-5848-and-is-pinned-unjudged.md`.
- `boss_union`: pinned, still unbisected, on `work/green/seat7-sweep-digest-drifted-on-main.md`.

Every row then moved again at PR 6856 (`a9df07339`, mint ids alone), which the same PR re-blesses.
