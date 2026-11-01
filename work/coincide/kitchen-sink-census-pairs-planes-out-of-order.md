---
id: kitchen-sink-census-pairs-planes-out-of-order
kind: issue
title: kitchen_sink's census reads bool_plane_parallel as laundered: one name, one sign, operands in another order
status: closed
opened: 2026-10-10
priority: P3
cost: M
closed: 2026-10-18
refs: [4596, under-the-guided-lift-boss-unions-bool-pair-gap-decisions-swap-order-under-the-apart-move]
---


`crates/editor-core/tests/revalue_corpus.rs`'s `revalue_census`, with
`REVALUE_TIER=off REVALUE_DOC=kitchen_sink` in release, reads `laundered
{bool_plane_parallel}` at decisions #2989–#2993.

(EDIT, 2026-10-17, PR 5247.) `REVALUE_TIER` is gone. Every census build
now runs in the coincidence door's replay session
(`editor_core::coincide::with_replay_session`: deferring, 256 terms,
guided lift), so `REVALUE_DOC=kitchen_sink` replays a different session,
and the decision indices above may not reproduce as given. Re-read them
there before working this. The same class, an order swap read as a
laundering, turned up on boss_union under the guided lift:
`work/inlane/under-the-guided-lift-boss-unions-bool-pair-gap-decisions-swap-order-under-the-apart-move.md`. The mismatches are
about 1e-7 relative.

- The re-valued #2992 equals the `p1` build's #2989 bit for bit
  (1.272255251383648). So the `p1` build decided the same predicates
  over the same planes in a different order, and pairing decisions by
  position and name compares two margins.
- No sign differs before them, so the `after_flip` reading
  (`tally`) does not catch it.
- PR #4543 named the same candidate in kitchen_sink and kiss_carry.
  kiss_carry's goes to `after_flip` now: a sign flips at #2981.

It is very likely a pairing artefact, not a laundering, but nothing
tells the two apart today.

Owed: read why the order differs. If it is a tie the move breaks
another way, it is the census's to handle: pair the plane decisions as
a set, or part there. If it is a laundering, fix it at its site.

## Closed: folded

Folded into `under-the-guided-lift-boss-unions-bool-pair-gap-decisions-swap-order-under-the-apart-move`, re-scoped by the INLANE orchestrator's ruling (b) (2026-10-18) to the revalue instrument: p0 and p1 decisions are to be paired by what they decide, not by index. This item's plane swap is one of that item's two done-when rows.
