---
id: blend-swallows-the-edge-name-fault-the-index-calls-loud
kind: issue
title: The blend tool swallows every EdgeNameFault, including the arm the index calls loud
status: open
opened: 2026-09-22
priority: P2
cost: D
---



Filed by `vgeom/seam-refusals`' sweep for the class *a typed refusal
that is computed and then dropped on the floor*.

## The finding

`crates/viewer/src/pickindex.rs`'s `PickIndex::edge_name_of` says, in
its header, that **the ways it fails are not the same news**:
`EdgeNameFault::NotDrawn` is ordinary (a stale or foreign selection),
`EdgeNameFault::OutOfRange` is an address minted by hand, and
`EdgeNameFault::Unnamed` is *"the naming-emission bug arm"*. The
index's own edge-pick path is written to that rule — at the
`edge_name_of` call in `PickIndex::edge_near`, the comment reads
*"The loud unnamed-entity arm reaches the caller as a refusal, not as
a silent miss"*, and all three arms are returned.

`crates/viewer/src/blend.rs` has two callers and neither of them
carries any arm:

- `BlendTool::mark_segments` — `index.edge_name_of(id).is_ok_and(...)`,
  so a refused name is an edge that is simply not marked.
- `BlendTool::load_all_edges` —
  `.filter_map(|&id| index.edge_name_of(id).ok())`, so a refused name
  is an edge that is not in the set. When every name refuses, the set
  is empty and the tool answers `BlendEvent::NoEdgesOnTarget`, which
  tells the user the target has no edges when what actually happened
  is that the index could not name the ones it drew.

So a naming-emission bug on a blend target reads to a person as an
empty body, and the mark lane quietly loses edges with no count and no
word.

## Why this is not the viewer's f32 seam row that found it

Same class, different seam: the finding is a typed fault collapsed to
an `Option`, not a value that will not narrow. `blend.rs` is this
program's under `work/author/program.md`'s `paths`.

## Still present (2026-09-25)

Found a second time by the review of `vnews/ray-refusal-is-not-a-disagreement`
(PR 3221), sweeping by callee (`edge_name_of`) across the viewer
crate. Both sites (`load_all_edges`'s `filter_map(.. .ok())` and
`mark_segments`'s `is_ok_and`) are unchanged on main at that date. The
`marks.rs` sibling for faces is
`work/vgeom/marks-focus-drops-an-unnamed-patch-from-attribution`.
