---
id: kitchen-sink-census-pairs-planes-out-of-order
kind: issue
title: kitchen_sink's census reads bool_plane_parallel as laundered: one name, one sign, operands in another order
status: open
opened: 2026-10-10
priority: P3
cost: M
refs: [the-from-f64-gate-reads-every-generic-crate]
---


`crates/editor-core/tests/revalue_corpus.rs`'s `revalue_census`, with
`REVALUE_TIER=off REVALUE_DOC=kitchen_sink` in release, reads `laundered
{bool_plane_parallel}` at decisions #2989–#2993. The mismatches are
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
