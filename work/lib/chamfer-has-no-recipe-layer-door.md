---
id: chamfer-has-no-recipe-layer-door
kind: issue
title: chamfer_edges has no recipe-layer door - no Node::chamfer, so no rebuild, no names, no selectors
status: open
opened: 2026-08-22
github: 918
refs: [708]
---

## From GitHub issue 918

opened 2026-08-22, 0 comments.

`sweep::chamfer_edges` ships (VERBS-CHAMFER) and is reachable only from the plain-body API. There is no `Node::chamfer`, so:

- a chamfer cannot appear in a recipe, cannot rebuild under an upstream edit, and cannot be diffed;
- the chamfer's birth records (`FilletNaming`'s `blends`/`corners`/`trims`/`feet`/`arcs`, which the surgery already writes for every chamfer) reach no emitter, so a chamfered face has no `StableName` and no selector can name one;
- the whole-body `all_edges` materializer — the door that makes "break every edge" one call — lives at the document layer, so a plain-body consumer enumerates arena keys instead. The tour's `spacer` scene (`demos/tour/src/bodies.rs`) records exactly that friction in its scene note, which is what the demo is evidence of.

## What closing it takes

A `Node::chamfer` alongside `Node::fillet` in `editor-core` (recipe node, eval arm, schema version), and `names::emit_chamfer`.

**The emitter is where the care is.** `emit_fillet` does not propagate an upstream N2 tie (#708): a legitimate `Entry::Tied` upstream makes two minted entities take the same `StableName` and the second `insert` refuses `DuplicateName`, reported as an aliasing bug in a crate that has none. Do not replicate that. `emit_topo`'s `TieRows` deferral (`names/emit_topo.rs`, `TieRows::flush`) is the shape to copy — it defers rows to a stage boundary precisely because a tie cannot be inserted one member at a time. #708's own text says the fix should land WITH the unit that mints the first tie, so a chamfer emitter written against the deferral shape from birth is the cheap outcome and one written like `emit_fillet` adds a second site to fix.

Also decide there whether chamfer strips and corner patches need `RoleSeg` variants of their own. VERBS-CHAMFER reused the fillet's kernel-side rows deliberately (the roles say what a chamfer needs: a band face off a source edge, a corner patch off a source vertex, a trimline off an edge-and-support); whether the NAME should distinguish a chamfer strip from a fillet blend is a naming-design question this issue's unit owns, not one the kernel-side records prejudge.

## Home

Recipe doors are LIB's by charter (`docs/RECIPE-DOORS-DESIGN.md`, chamfer landed as G16), and S-BLEND's own charter ceded the recipe layer to LIB.
