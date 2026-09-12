---
id: pickindex-header-names-four-consumers-and-holds-six
kind: issue
title: pickindex's doc header is pick.rs's old one carried over and no longer describes the module it heads
status: closed
opened: 2026-09-06
refs: [2079]
closed: 2026-09-06
pr: 2079
---



Found by the style review of #2079.

## What

`pick.rs` got a new doc header in #2079. `pickindex.rs` got the old
one, unedited — every line of `crates/viewer/src/pickindex.rs:1-50`
is a line of the merge base's `pick.rs`, which a sorted-line diff of
`old pick.rs` against `new pick.rs + pickindex.rs` confirms (43 lines
added, one removed, all of the additions in the two new headers and
the import blocks).

It reads *"# One index, four consumers"* and enumerates the ray path,
the screen path, the id map and the drawn mesh
(`pickindex.rs:4-25`). The module also holds, and the header names
none of them:

- `highlight` / `Highlight` (`pickindex.rs:1812-1848`);
- `edge_overlay` / `EdgeOverlay` / `edge_segments` /
  `edge_id_segments` (`pickindex.rs:1862-2010`);
- `focus` / `marked_for` / `drives` (`pickindex.rs:2076-2192`) — which
  reads the `Doc`, walks `doc.order()` and answers *what does this
  parameter move*, a side-panel question with no cursor in it;
- `cursor_projection` (`pickindex.rs:2194`).

The header's first line does gesture at one of them (*"and the
highlight"*), which is why this is easy to miss.

## Why it counts

The header is the module's own claim about itself and it was the one
document in the diff nobody re-read; the README was re-read and two
rows corrected, `pick.rs`'s header was rewritten from scratch. A
2,383-line module whose header enumerates four of its six concerns is
the accumulation failure the reviewer brief's Q8 describes, arriving
in a file that was just created.

## Confidence

`sure` that the four-consumer list omits `focus`, `edge_overlay` and
`cursor_projection`, and that the header is carried verbatim.

## Closed

`crates/viewer/src/pickindex.rs:1-85` is rewritten and no longer
carries a line of the merge base's header. It opens on the module
having **two** subjects rather than one — what is under the cursor, and
what the frame marks because of it — names `highlight`, `edge_overlay`,
`edge_segments`, `edge_id_segments`, `focus` and `cursor_projection`
under the second, and says of `focus` that it is not a cursor question
at all and is here only because `PickIndex` is where the ids live. It
also names the picking POLICY (`op_for`, `op_under`, `hovered_for`'s
priority rule) as this module's, which the old header did not, and
points at `pickindex-holds-the-frames-marks-as-well-as-the-index` for
the second boundary the two-subject opening implies.
