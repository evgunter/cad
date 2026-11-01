---
id: a-pick-read-takes-the-whole-copy
kind: issue
title: the picture takes a whole copy for a pick of one of its pieces
status: open
opened: 2026-11-01
---


Found in the dual review of PR 6265 (both reviewers; API-only today).

`world::taken_by` (`crates/viewer/src/world.rs`) resolves a body read whose variable is a `VarDef::Pick` to the copy the pick reads (`pick.placement`) with `Covers::all()`. A pick names ONE piece of a copy of a `⊔` (`Pick { placement, of }`, `crates/editor-core/src/var.rs`). So a placement or an operation reading the bolt of a copy of `bolt ⊔ plate` hides the whole copy from the picture, and the plate piece is drawn nowhere, although nothing reads it.

**Probe.** Build `bolt ⊔ plate` in a space and place its copy `C` into the world. Add `Place { source: Pick { placement: C, of: [bolt] } }` into the world. `world::drawn_bodies` then holds the pick's placement but not `C`, so the plate is not in the picture. No viewer gesture authors a `Pick` read yet, so only the API reaches this.

**What it should do.** A copy of a `⊔` is drawn as its pieces, as the `⊔` itself is. A pick takes the piece it names, and the copy's other pieces stay drawn. That needs the copy's pieces addressed as bodies of the drawing (a piece key under the copy), which the pick index does not have today.

Done when: a row in `crates/viewer/tests/` places a pick of one piece of a copy of a two-piece `⊔`, and asserts the other piece is drawn.
