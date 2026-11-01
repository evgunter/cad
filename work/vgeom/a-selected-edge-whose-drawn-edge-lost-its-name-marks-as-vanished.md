---
id: a-selected-edge-whose-drawn-edge-lost-its-name-marks-as-vanished
kind: issue
title: A selection whose drawn edge or face lost its name lights nothing, and reads as vanished
status: open
opened: 2026-09-30
priority: P3
cost: M
---


Found by AUTH-14's sweep (`author/edge-name-fault`) for the class *a
typed name fault collapsed to an `Option` or a `bool`*, on its second
pass: the gap the callee sweep cannot see, where no fault is computed
at all because the lookup goes by NAME.

## The finding

`crates/viewer/src/marks.rs`'s single-selection marks find their entity
by name: `edge_segments` through `PickIndex::edges_of_target`, and
`face_id` (so `highlight` and `drawn_patch`) through
`PickIndex::ids_of_target`. Both read the index's name inverse, which
`PartWindows::push_names` (`crates/viewer/src/pickindex.rs`) fills from
the `Ok` names only. So when the drawn edge or patch that carried a
selected name now refuses its name (the loud `UnnamedEntity` arm), the
lookup answers nothing, and the mark is empty exactly as it is for a
selection that vanished. The refusal was never computed, so nothing
counts or says it.

AUTH-14 closed the same gap for the blend tool's HELD set by walking
the held body's drawn edges (`PickIndex::edge_names_in`) and carrying
the refusal to a badge (`frame::held_edges_badge`). A single selection
could ask the same walk of its own (node, body) when its name is not
found, and hand the refusal to the same surface or a sibling.

## Stakes

A drawing, not a claim, like the face-side
`marks-focus-drops-an-unnamed-patch-from-attribution`, and reached only
through a naming-emission bug. Take the two together.

## Fence

`crates/viewer/src/marks.rs` (CHROME, VGEOM).
