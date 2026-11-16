---
id: corner-tables-body-moved-at-5848-and-is-pinned-unjudged
kind: issue
title: corner_table's body moved at PR 5848 and nobody has judged it: seat7's digest is pinned at main's reading
status: open
opened: 2026-11-14
priority: P1
cost: E
---


Filed by GREEN's `green/goldens` lane, from nightly 43371638371 (main `e36675c1`). This is the fix row for `corner_table` in `work/green/seat7-sweep-digest-drifted-on-main.md`.

**What moved.** `crates/editor-core/tests/seat7_sweep_lowering.rs` `the_sweep_documents_evaluate_to_their_committed_digests`, the `corner_table` row, red on main since 2026-11-01 at every ε row.

**The merge.** PR 5848 (`cleave/kept-side-in-one-place`, merge `9fc2720b5e`, 2026-10-31): `0x8078f0efe3f11dd2` became `0xfa506ba6d180fc96` there, bisected first-parent at ε 1e-9 by `onecopy/main-reds-triage` on 2026-11-09. Its description does not disclose it. CLEAVE's ground is CLEAVETAIL's now, so the row is here.

Every move of the row since is accounted for, and none of them judged this one:

- PR 6328 (`0da3e56b97`): `Curve3::Line`'s `Debug` spelling, to `0xc19eae46ab271546`;
- PR 6505: arena keys alone (its key-masked dump is main's line for line), re-pinned over this move at `0x3676c700af58ca2d`;
- PR 6832 (`9a256348`): arena keys alone (a probe read every cell by its geometry on both sides), to `0x189560e0ffba1a23`;
- PR 6856 (`a9df07339`): mint ids alone (a placement mints its source; with mint ids read by rank of first appearance the document's digest feed is byte-identical across the merge), to `0x17ca9ca76aa493c1`.

**What GREEN did.** The row is pinned at `0x17ca9ca76aa493c1`, what main reads, with this row named in the file, so the suite is green and the row reds on the next move.

**Owed.** Say what PR 5848 changed in the corner table's body and whether the new body is right. If it is, strike `corner_table` from the PINNED paragraph in `seat7_sweep_lowering.rs`; if it is wrong, that is a live wrong answer on a subtract and this row is P0.

Reproduce: `cargo nextest run -p editor-core -E 'test(the_sweep_documents_evaluate_to_their_committed_digests)'` at `9fc2720b5e^1` and at `9fc2720b5e`.
