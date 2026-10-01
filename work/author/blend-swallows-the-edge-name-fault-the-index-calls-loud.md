---
id: blend-swallows-the-edge-name-fault-the-index-calls-loud
kind: issue
title: The blend tool swallows every EdgeNameFault, including the arm the index calls loud
status: closed
opened: 2026-09-22
priority: P2
cost: M
branch: author/edge-name-fault
closed: 2026-09-30
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

## Moved (AUTH-10, 2026-09-30)

The first site left `blend.rs`: the held set's per-frame mark walk is
now `marks::HeldEdges::segments` (`crates/viewer/src/marks.rs`), which
the blend tool feeds through `BlendTool::held_edges`. The collapse is
unchanged — `index.edge_name_of(id).is_ok_and(...)` — so the row's
first bullet now lands on VGEOM's `marks.rs` as well as this slate.

Dispatched 2026-09-30 as **AUTH-14** (`docs/AUTH-14-SPEC.md`, branch `author/edge-name-fault`).

## Built (AUTH-14, 2026-09-30)

Branch `author/edge-name-fault`. `PickIndex::edge_names_in` reads a
drawn body's window whole and keeps the refusal as a value
(`EdgeNamesRefused`: the body, its named and unnamed counts, and the
first unnamed edge's `UnnamedEntity`, rendered through
`EdgeNameFault::Unnamed`'s own `Display`). A window walk has no address
to overrun and a body with no window answers nothing, so the loud arm
is the only one the type can hold. Both readers use it:

- `BlendTool::load_all_edges` refuses the whole load with
  `BlendEvent::EdgesUnnamed { refused }` when any drawn edge of the target refuses,
  partly or wholly, and leaves the held set untouched. It no longer
  answers `NoEdgesOnTarget` for a body whose edges it could not name.
- `marks::HeldEdges::mark` draws the named held edges and carries the
  refusal on `EdgeOverlay::held_refused`. The viewport writes that
  every frame into the same zeroed-per-frame channel as the profile
  count, and the toolbar draws `frame::held_edges_badge`. The fault
  wears `Subject::Document` on both the badge and the line.

A body the index does not draw (`NotDrawn`'s case) stays quiet on both
paths. The face-side twin stays on VGEOM's row with a note, and the
selection-mark sibling is filed as
`work/vgeom/a-selected-edge-whose-drawn-edge-lost-its-name-marks-as-vanished`.

## Closed 2026-09-30 — PR 3585 merged (`cd6cc9a0`)

**A blend target whose edges cannot be named says so.**
- Both readers go through one door, `PickIndex::edge_names_in`. It refuses only with the naming layer's own `UnnamedEntity`, and that is enforced by the type.
- **Select all edges** refuses the whole load in the index's words, and never says "no edges".
- The held-edge mark draws what is named and carries the rest to a toolbar badge. A row proves the badge goes away.
- A window that runs past its entities or names is an `unreachable!` stating the invariant, not an empty body.

Filed from the unit:
- VGEOM's `a-selected-edge-whose-drawn-edge-lost-its-name-marks-as-vanished`;
- CHROME's `the-per-frame-badge-reads-are-three-hand-copied-fields`.
