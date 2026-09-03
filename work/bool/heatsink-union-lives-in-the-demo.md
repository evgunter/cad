---
id: heatsink-union-lives-in-the-demo
kind: issue
title: heatsink.pncad's product is a base and five fins interpenetrating: the union lives in the demo, never in the recipe
status: open
opened: 2026-08-29
github: 1261
refs: [1162, 1230, 1344]
---

## From GitHub issue 1261

opened 2026-08-29, 0 comments.

## What the document says

```
heatsink.pncad   5 nodes, 2 root(s), product 36 faces, V = 1.321289, 5 separation findings
```

All five findings are `root 1 output 0 … not certifiably disjoint from root 4 output N` for N = 0…4 — the base against each of the five fin instances. They are correct. The fins really do interpenetrate the base, because `Node::Pattern`'s instances and the base are two product roots and nothing in the recipe joins them.

The join happens in the demo instead, in `demos/tour/src/heatsink.rs::solidify()`, which accumulates `try_union(&acc, fin, tol)` over the pattern's instances at render time. So the tour draws a solid heatsink and the *document* denotes a base with five fins parked inside it.

## Why this is not the die's bug

[#1230](https://github.com/evgunter/cad/pull/1230) fixed `diefillet.pncad`, which had the same symptom from a different cause: it carried a **spare body** (the blank, authored for a narration stop, consumed by nothing), and deleting that node made the document say what the scene meant.

There is nothing to delete here. The recipe cannot currently express what the demo means, so the fix is to **author the union** — and that changes what the scene demonstrates. The heatsink's stops are the memo counters over a structural count edit (`SetStructuralParam` on the pattern's `Count`, cold evaluation at 5 then warm re-evaluations at 7 and 9), so whatever joins base to fins has to survive that edit and re-evaluate incrementally, which is the whole point of the scene.

## One thing to keep while fixing it

The heatsink is currently the **only in-corpus document that exercises the separation resident** — the die's finding went away with #1230, and `checks.pncad`'s finding is a connectedness one. #1162's own design argument leaned on it ("its product genuinely is overlapping solids and now says so", five findings all correct and none acknowledgeable, which is what makes DS6's waiver vocabulary non-paper).

`crates/viewer/tests/doc_io.rs::overlapping_roots_still_draw_and_land_a_finding` and `editor-core`'s rows use synthetic fixtures, so the check itself stays covered. But whoever fixes this should decide deliberately whether the corpus keeps a document that reports, rather than discovering afterwards that nothing does.

## Where it shows

`demo-tour gallery` now prints it as it writes ([#1230](https://github.com/evgunter/cad/pull/1230)):

```
   heatsink.pncad — 5 node(s), 2 product root(s), 7219 byte(s) — 5 finding(s), 5 of them separation
```

and `gallery.rs`'s `each_gallery_document_denotes_its_scene_or_says_why_not` records the expected numbers with this reasoning, so the row has to be updated deliberately when this lands.

## Home

`work/bool/` — the fix is authoring the base/fin union the recipe never expresses, the same ground as issue 1344's unfinished `PlacedUnion` migration, which S-BOOL owns as boolean composition.
