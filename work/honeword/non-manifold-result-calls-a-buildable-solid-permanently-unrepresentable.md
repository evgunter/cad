---
id: non-manifold-result-calls-a-buildable-solid-permanently-unrepresentable
kind: issue
title: NonManifoldResult calls a solid with a manifold form permanently unrepresentable; after Ev's D1 ruling on PR 4795 it is a construction gap
status: open
opened: 2026-10-19
priority: P3
cost: M
---


## What

Ev's ruling on `[ev]` PR 4795 (2026-10-19) narrowed D1's
representability boundary. A single edge with more than two faces, or a
wedge fan sharing one entity, "assembles faces into something that is
not a solid". The clause "'Non-manifold' means non-representable" is
gone, and so is the principles line "Non-manifold boolean results are
typed errors". Ev's reason: "you can't use them to get a non-solid from
solids". The material-decides-the-edges sentence ratified on the same
PR gives every line or point where more than two sheets meet one edge
per creased material wedge and one vertex per piece of material.

`BooleanError::NonManifoldResult` (`crates/topo/src/boolean/mod.rs`
~2908–2925, `insert::reconcile_shared` at `insert.rs` ~1092) still
claims the old boundary:
- **Its doc** reads "That shared-entity wedge fan is unrepresentable
  (…"Representability boundary"), so this refusal is permanent at any
  tolerance".
- **Its `Display`** (~4481) reads "no boundary representation can hold
  that, at any tolerance".

The union of two pinches whose lines overlap is a solid. By the
ratified sentence it has a manifold form: coincident edges, and one
vertex per piece of material at the point. The refusal is therefore a
construction gap: the door does not yet build that form. It is not a
shape that cannot exist.

## Owed

- Rewrite the variant's doc and its `Display` as a kernel limitation
  that has not been built yet, ending in D4 ¶1(i)'s last resort. Keep
  the lever if it is still truthful ("meeting end to end at a point
  builds"), and drop "at any tolerance" and "no boundary representation".
- Check `union_flush_onto_edge_contact.rs` and any pin that asserts the
  old text or the word "permanent".
- Building the manifold form, i.e. the vertex split at the point, is a
  separate unit. File it when this lands.

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P2 → P3: the refusal calls a construction gap permanent after Ev's PR 4795 ruling: library usability: the refusal names the wrong cause, recourse or payload, P3 by `work/README.md` Priority.
- Moved to HONEWORD (`work/honeword/`) by `git mv`, id and body unchanged: whose charter is split and boolean refusal text, payloads and endings. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
