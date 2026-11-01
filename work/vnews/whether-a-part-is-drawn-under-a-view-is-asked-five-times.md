---
id: whether-a-part-is-drawn-under-a-view-is-asked-five-times
kind: issue
title: Whether a part is drawn under a display view is asked in five places, held the same only by prose
status: open
opened: 2026-09-28
priority: P3
cost: M
---

Found in review of `vnews/every-id-is-said`, which added the fifth
copy.

## The finding

"Is this part in the picture under this `DisplayView`?" is one rule,
`display.hidden_placements.contains(&<the part's placement>)`, and it is written
out five times in `crates/viewer/src`:

- `pickindex.rs` `PickIndex::scene_focused` (~:1193): the picture
  skips a hidden part.
- `pickindex.rs` `PickIndex::pick_for` (~:1298): the `visible` filter
  over the unmoved batch.
- `pickindex.rs` `PickIndex::pick_for` (~:1323): a moved placement that is
  also hidden is skipped.
- `pickindex.rs` `PickIndex::edge_polyline_for` (~:1533): a hidden
  part's edge draws no polyline.
- `idpass.rs` `IdAnswer::of` (~:247): an id whose patch's node is
  hidden is `IdAnswer::Undrawn`.

Only prose keeps them the same. `scene_for`'s doc says an id "names
the same patch whatever is hidden", and `marks.rs`'s `edge_id_segments`
doc says it keeps "`PickIndex::edge_polyline_for`'s rule unaltered, so
the two doors cannot disagree". `marks.rs` itself (`edge_id_segments`) holds no
copy, because it delegates to `edge_polyline_for`. If the rule grew a
second clause (a focus-isolated view, or a hidden body rather than a
hidden placement), the picture, the pick, the edge overlay and the id pass
could each be changed on its own, and nothing would report that they
had drifted apart.

## What a fix would be

A single door, for example `DisplayView::draws(node) -> bool` or a
`PickIndex` method that answers for a part or a patch id. All five
sites read it. `IdAnswer::of` would then ask whether the index draws
the id under the view, rather than reading `IdMap::key_of`'s node
itself.

## Fence

`crates/viewer/src/pickindex.rs`, `crates/viewer/src/display.rs`,
`crates/viewer/src/idpass.rs`.
