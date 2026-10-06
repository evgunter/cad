---
id: a-vanished-name-carries-no-holding-node
kind: issue
title: A vanished name forwarded bare carries no holding node, so two copies' faces read alike in it
status: open
opened: 2026-10-06
priority: P3
cost: M
---


Found by PR 3886's third fix pass, building Ev's ruling on #4069 (`refusals-with-the-longest-scoped-names-overrun-the-budget`, "the missing holding node in a forwarded name's payload").

A name says the node that minted it, not the node whose output holds the entity, so two `Transform` copies of one body hold names alike. A sentence that names a face of each says which copy holds which where the payload carries the holder:

- `SelectRefusal::PairInBand` now carries `at` (the flush query's two nodes, `names/flush.rs` `pair_verdict`), and its sentence says each face's node where the two read alike (`names/geompred.rs`).
- `HitTestError::Ambiguous` carries each `PickHit::node` and says it the same way (`resolve/hit.rs`).

`ResolveError::Vanished` (`resolve/mod.rs`) carries only the `StableName`. Where a node or pane says it, the sentence leads with the reference (`this fillet's edge 2`, `this face`, `resolve::AboutReference`), which fixes the holder. Said bare — Python's `resolution` door (`pncad-py/src/py/resolve.rs`), a log — it says the name once, in full, with no holder, so a reference to a copy's face and one to the master's read alike there.

Carrying the holder means a field on `ResolveError::Vanished` (and `NodeGone`, `Ambiguous` for the same reason), set by every N5 ladder that mints one: `resolve::resolve`'s rungs and `eval::wire`'s `ladder`. That is the resolve payload's shape across the kernel, the Python crossing and the tests that build these values, wider than the words unit.

Priority low while the bare form has no viewer surface: every viewer door that says a resolve failure leads with the reference.
