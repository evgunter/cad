---
id: pickindex-holds-the-frames-marks-as-well-as-the-index
kind: issue
title: pickindex wants a second split: the marks a frame draws are 400 lines that only use PickIndex's public doors
status: open
opened: 2026-09-06
refs: [2079]
---



Found by the style review of #2079, answering the dispatch's question
*"is `pickindex` at 2,383 lines a coherent module or a bag?"*.

## The second boundary, named

It is mostly coherent, with one tail that is a different subject and
comes off cleanly. `crates/viewer/src/pickindex.rs:1806-2210` — about
405 lines — is **the marks a frame draws**, as opposed to what is
under the cursor:

`Highlight`, `highlight`, `EdgeOverlay`, `edge_overlay`,
`edge_segments`, `edge_id_segments`, `segments_of`, `focus`,
`marked_for`, `drives`, `cursor_projection`.

Three properties make this a boundary rather than a line number:

- **It takes the index as an argument.** Every one of them receives
  `&PickIndex` and reads it through public doors only — `ids()`,
  `name_of()`, `ids_of_target()`, `edges_in()`, `edge_polyline_for()`
  (verified over `highlight` at `:1836`, `focus` at `:2076`,
  `edge_overlay` at `:1942`). Nothing touches a private field, so the
  move is mechanical.
- **Its consumers are a different set.** `gpu`, `pane::viewport`,
  `blend`, `datums` and `app` reach for these; the ray and screen
  paths above them are reached by `pane::viewport` alone.
- **`focus` is not a picking concept at all.** It answers *which drawn
  patches is the side panel's selection responsible for*, walks
  `doc.order()` and inspects parameter drivers (`drives`,
  `pickindex.rs:2165`). It is in this file because `PickIndex` is
  where the ids live, not because it is about the cursor.

The remaining ~1,800 lines are one subject: keys and errors, the
`PartWindows` machinery, `PickIndex` and its queries, and the pixel
geometry the screen path needs.

## Not urgent

The split #2079 made was the one the cycle demanded. This one is a
readability call with no mechanical forcing function behind it, and
naming it is the point — a later reader should not have to re-derive
where the seam is.

## Confidence

`likely` that this is the right second boundary; `sure` on the public-
doors-only property that makes it available.
