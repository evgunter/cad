---
id: marks-focus-drops-an-unnamed-patch-from-attribution
kind: issue
title: marks::focus drops a patch whose name refuses from highlight attribution without a count
status: open
opened: 2026-09-25
priority: P3
cost: E
---

Found by the review of `vnews/ray-refusal-is-not-a-disagreement`
(PR 3221), which swept by callee (`name_of`, `edge_name_of`) across
the viewer crate for the class *a typed refusal computed and then
dropped*.

## The finding

`crates/viewer/src/marks.rs`, `focus`, collects the patches it can
attribute to a node as

```
.filter_map(|id| Some((id, attribute(index.name_of(id)?.as_ref().ok()?))))
```

`PickIndex::name_of` answers `Some(Err(HitTestError))` for the
unnamed-face bug arm, which the index calls loud. Here that arm is
dropped with the ids the index did not assign. A patch whose name
refuses is never lit when its feature is selected, and nothing counts
or says the loss.

## Stakes

It is drawing, not a claim: nothing tells the reader that a patch
belongs to nothing. That puts it below the `blend.rs` sibling
(`work/author/blend-swallows-the-edge-name-fault-the-index-calls-loud`),
where the same collapse reaches a false sentence. It is the same
collapse all the same. A lane that makes `name_of`'s refusal a count
or a badge (as `marks::LegLane::undrawn` does for legs the display
seam refuses) should take this site with it.

## Fence

`crates/viewer/src/marks.rs` (CHROME, VGEOM).

## Left here by AUTH-14 (2026-09-30)

AUTH-14 made the EDGE refusal a value (`PickIndex::edge_names_in`,
`EdgeNamesRefused`) and badged it for the blend tool's held mark
(`frame::held_edges_badge`). It did not take this site: `focus` is not
on the per-frame mark path that badge reads. It runs in
`ViewerApp::sync_scene` (`crates/viewer/src/app.rs`) and feeds
`PickIndex::scene_focused`, a scene rebuild cached on `scene_focus`, so
its refusal needs a surface on that seam, not the viewport's per-frame
read. The door is also the flat patch lookup (`name_of`), not the
edge window. Its selection-mark sibling is filed beside it:
`a-selected-edge-whose-drawn-edge-lost-its-name-marks-as-vanished`.
