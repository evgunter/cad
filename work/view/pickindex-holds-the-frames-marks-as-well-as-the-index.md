---
id: pickindex-holds-the-frames-marks-as-well-as-the-index
kind: issue
title: pickindex wants a second split: the marks a frame draws are 400 lines that only use PickIndex's public doors
status: closed
opened: 2026-09-06
refs: [2079]
closed: 2026-09-06
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

## Closed — the split is taken, as a pure move (2026-09-06)

`crates/viewer/src/marks.rs` is the module. All eleven named members
moved, and nothing outside the range came with them.

**The three properties, member by member.** Two of the eleven are value
types and three take no index at all, so the first property is checked
where it applies and the other two carry the members that have no
`&PickIndex` argument:

| member | takes `&PickIndex`, public doors only | consumers |
|---|---|---|
| `Highlight` | value type — `highlight`'s answer | `gpu`, `lib` |
| `highlight` | yes — `ids_of_target` | `pane::viewport`, four suites |
| `EdgeOverlay` (+ `is_empty`, `segments`) | value type — `edge_overlay`'s answer | `gpu`, `pane::viewport`, `datums`, `edge_pick` |
| `edge_overlay` | yes — via `edge_segments` | `pane::viewport`, `datums`, `edge_pick` |
| `edge_segments` | yes — `edges_of_target` | `blend` (by name, in prose) |
| `edge_id_segments` | yes — `edge_polyline_for` | `blend`, `blend_authoring` |
| `segments_of` | private helper of the two above | — |
| `focus` | yes — `ids()`, `name_of` | `app`, `scene`, `focus_highlight` |
| `marked_for` | yes — `ids_of_node` | private to `focus` |
| `drives` | none — reads the `Doc` | private to `focus` |
| `cursor_projection` | none — a matrix | `gpu`, `lib`, three suites |

Every door reached is `pub` on `PickIndex` (`ids`, `name_of`,
`ids_of_node`, `ids_of_target`, `edges_of_target`,
`edge_polyline_for`); `PickIndex`'s five fields and `IdMap`'s two are
private, and the compiler is the check that none was touched. The
consumer set is the different one the item named — `gpu`, `blend`,
`datums`, `app` reach only for these, and the ray and screen paths are
`pane::viewport`'s alone. `focus` is not a picking concept: it walks
`doc.order()` and inspects parameter drivers, and reaches for an index
only because that is where the ids live.

**Nothing failed the properties, and nothing outside the range passed
them.** The one judgement call was `cursor_projection`, which takes no
index at all and could have stayed: it went because its consumer set is
the marks' one and its subject is the id pass, not the index.

**The receipt, and it certifies ONE COMMIT rather than the branch.**
Merge base `499a17b`'s `pickindex.rs` against the move commit
`6702d15`'s `pickindex.rs + marks.rs`, sorted, whitespace-sensitive:
**34 lines removed, 75 added, and not one of them is code**. The same
diff measured at the branch head is 35/76 after the rename commit and
35/90 after the fix pass, because both edit prose in those files — the
numbers above are true of the commit whose warrant they are, and
quoting them against the tree is what the review of this PR caught.
(All three head figures are measured, not projected: an earlier draft
of this paragraph wrote "38/89" by estimating one, which is the same
error one paragraph after admitting it.) The no-code-line property
holds at every one of the three.

Every removed line is a `//!` header line, a `///` doc line the
doc-link fix reflowed, or a `use` line whose name list shrank. The 144
declarations (`fn`, `struct`, `enum`, `const`, `type`, `impl`, `mod`,
`trait`, `#[derive]`) are byte-identical between the two sides, and
that half holds at head as well as at `6702d15`. `#[test]` over `crates/viewer/{src,tests}` is 533 on the
merge base and 533 on head; `--test all` is 503 passed / 1 ignored on
both, and `--lib` 24 passed / 1 failed on both (the Vulkan-less
`gpu::every_pass_builds_on_a_real_device`).

Two lines could not be carried verbatim and both are doc links the move
broke: `[`Camera::project`]` became `[`crate::camera::Camera::project`]`
(the type is not imported in `marks`, and an import used only by a doc
link is an `unused_imports` error under `-D warnings`), reflowing its
four-line paragraph; and `pickindex.rs`'s `(see [`focus`])` at `:889`
became `(see [`crate::marks::focus`])`. Both headers were written from
scratch rather than carried.

**No `pub use` shims.** `viewer::{Highlight, EdgeOverlay, highlight,
edge_overlay, edge_segments, edge_id_segments, cursor_projection}` still
resolve at the crate root, re-exported from `marks` instead of
`pickindex`; `pickindex::highlight` and its siblings do not resolve at
all. Every call site in `src`, `tests` and `examples` was re-pointed.
