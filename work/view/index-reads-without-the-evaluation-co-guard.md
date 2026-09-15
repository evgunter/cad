---
id: index-reads-without-the-evaluation-co-guard
kind: issue
title: Two viewport reads of the pick index lack the evaluation co-guard the pick path has, and one of them writes a false diagnosis
status: closed
opened: 2026-09-05
branch: view/index-co-guard
pr: 2615
closed: 2026-09-15
---


## The class

`ViewerBehavior` holds `index: Option<&PickIndex>` and reads it at six
sites in `crates/viewer/src/pane/viewport.rs`. The pick path takes it
**with the evaluation**, in one destructuring —

```rust
if let (Some(index), Some(eval)) = (self.index, self.session.evaluation()) {   // :161
```

— because an index answers questions about a run, and the run is the
session's. Two other sites take the index **alone**:

- **`:366`, `frame::disagreement`.** The one that does harm. The GPU id
  pass renders `ViewportCallback.scene`, which is `ViewerApp::scene` —
  the mesh of whatever picture last landed — and the id it answers with
  is resolved through the id map of the index in hand. Where those are
  two different documents the comparison finds a mismatch and writes
  *"the two picking paths disagree"* to the status line. Issue #1097 §4
  tells an operator to read that sentence as an `R32Uint` clear fault,
  so it is not merely noise: it is a **false diagnosis naming the wrong
  subsystem**, which is worse than saying nothing.
- **`:215`, `BlendTool::mark_segments`.** Marks the open blend tool's
  held edges from the index alone. Harm is cosmetic by comparison —
  edges marked from one document's tessellation over another's picture
  — but it is the same missing guard.

`:146` (`index.map(PickIndex::generation)`, feeding `IdQueryLog::step`)
reads only the generation as a cache key and is fine as it stands;
`:179` is the not-indexed refusal, which is about the index's ABSENCE.
Both are named here so a taker does not have to re-derive which of the
six matter.

## Pre-existing, and why it is filed now

Not created by VIEW-6b (#1888): both sites have read the index ungated
since the pane was split. What 6b changes is how OFTEN the two can
disagree — before it, the index was built synchronously inside
`sync_scene`, so an index and a scene from different documents existed
only across the narrow paths that clear one and not the other; after
it, the window is seconds long by design. A latent gate becomes a
reachable one.

## What the fix owes

Not two edits. The obligation is the **sweep**: every read of
`self.index` in the viewport (and any that grows later) either takes
the evaluation with it or states why it does not need to. The two
sites above are the instances known today; the fix is the rule, and
`crates/viewer/README.md`'s picking section is where it would be
stated if it is worth stating there.

## What the sweep found, and where the framing moved

**The population is eight uses, not six, and it is not one file.**
`ViewerBehavior::index` is taken at four bindings — three in
`pane::viewport`, one in `pane::create` (`all_edges_row`, which already
takes it with the evaluation) — and those bindings fan out to eight
uses. The two this item does not enumerate are `marks::highlight` and
`marks::edge_overlay`, both ungated and both picture-side: a highlight
is an id the shader compares against the drawn corners' ids, and an edge
overlay is world-space geometry drawn over them. A fix guided by this
item's list alone would have repaired two of five.

**And the guard the harm site wants is not the evaluation.**
`frame::disagreement` resolves an id the id pass produced through an id
map. An id is a word of the alphabet of the index that MINTED the
picture's corners, so what the comparison needs is that the scene and
the index are one build — `(generation, δ)`, because a δ typed while the
document stands rebuilds the index at the same generation over a
different tessellation. Adding the evaluation to that tuple would have
read as co-identity while checking something else, which is the shape
this program keeps finding rather than one to re-mint. The rule as
landed is in `crates/viewer/README.md`, *A pick id is one index's word*.

**Residue, each with its own file**:
`id-query-is-keyed-on-the-generation-not-on-the-picture` (a second
producer of the same false sentence, which no co-guard closes) and
`a-pick-over-a-stale-picture-answers-about-a-picture-nobody-can-see`
(the product question the rule's document-side half leaves open).
