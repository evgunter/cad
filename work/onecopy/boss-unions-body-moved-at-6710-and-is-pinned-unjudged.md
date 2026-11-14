---
id: boss-unions-body-moved-at-6710-and-is-pinned-unjudged
kind: issue
title: boss_union's body moved at PR 6710 and nobody has judged it: seat7's digest is pinned at main's reading
status: open
opened: 2026-11-14
priority: P1
cost: E
---


Filed by GREEN's `green/goldens` lane (PR 6956), from nightly 43371638371. It takes `boss_union` over from `work/green/seat7-sweep-digest-drifted-on-main.md`, which is closed.

**What moved.** `crates/editor-core/tests/seat7_sweep_lowering.rs` `the_sweep_documents_evaluate_to_their_committed_digests`, the `boss_union` row (a carrier-loop boss unioned onto a block): `0x65453fae8b1d55d3` became `0x74fcab092e34e83b`, the same at every ε row.

**The merge.** PR 6710 (`onecopy/edge-ends-turns`, merge `75a84e844`, 2026-11-09: "edge ends through edge_vertices, periodic reductions through Period, and the thin-ellipse crossing splits (split gate on Span, check 2 by the chord)"). Bisected first-parent at ε 1e-9 over `5714d610e..151631cc`, nine points: the row is green on `boss_union` at `d3e3311b5` and reads `0x74fc…` at `75a84e844`, the next first-parent commit. PR 6710's diff touches no `boss_union` pin and no seat suite, and its gate did not run this row (slow set).

Since then only PR 6856 moved it, by mint ids alone (with mint ids read by rank of first appearance the document's digest feed is byte-identical across that merge), to `0x5be66936f8d10205`.

**What GREEN did.** The row is pinned at `0x5be66936f8d10205`, what main reads, with this row named in the file, so the suite is green and the row reds on the next move.

**What is known about the move.** Only that the digest moved at that merge. Whether a vertex point moved was not measured: `m10_p_fence`'s ids-masked row was last re-cut by PR 6795, two days after PR 6710, so its holding on main says nothing about this merge. The digest feeds each body's points, every curve's and surface's `Debug`, the arena keys and the attachments; which of those moved was not dumped.

**Owed.** Dump `boss_union`'s digest feed on both sides of `75a84e844` and say what moved and whether it is right. If it is, strike `boss_union` from the pinned paragraph in `seat7_sweep_lowering.rs`; if the body is wrong, that is a live wrong answer on a union and this row is P0.

Reproduce: `cargo nextest run -p editor-core -E 'test(the_sweep_documents_evaluate_to_their_committed_digests)'` at `d3e3311b5` and at `75a84e844`.
