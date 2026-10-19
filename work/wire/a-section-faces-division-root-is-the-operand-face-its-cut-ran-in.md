---
id: a-section-faces-division-root-is-the-operand-face-its-cut-ran-in
kind: issue
title: FaceDivisions::root of a section face descends to the operand face the section's mef ran in, so a consumer reading it as adjacency names a section chord by build order
status: closed
opened: 2026-10-18
priority: P3
cost: E
closed: 2026-10-19
---


Found by both FORK-INPLANE designers (2026-10-18). `name_split_edges_vertices`'s
chord pass (`crates/editor-core/src/names/emit_topo.rs`) applies
`frag_rows.root()` to the face across a chord before checking that the
face is not itself a section face. A section face's division row
descends to whichever operand face its `mef` was issued in, so an edge
between two section faces is named `SectionEdge { <that operand face> }`
(measured: NOTCHED lower tip edge `e16v1`, faces
`SectionFace 1 | SectionFace 0`, named `SectionEdge { Lateral(s38) }`,
the left slant, an upper-half face). `SplitNaming::face_fragments`'s doc
says consumers exclude the keys listed in `sections`, but `root()` does
not. The `section chord adjacent to a section face` guard runs after
the chase and so misses it.

Fix shape: check for a section face before the chase, and refuse
`Emission` there; or make `root()` refuse a section key. Under
FORK-INPLANE's merge, no edge between two section faces survives, so
this becomes a kernel-bug guard. It rides with whatever unit builds
FORK-INPLANE.

Signed (WIRE orchestrator).

## Resolution (2026-10-19)

Closed by PR 5334: a split finishes each half with maximal faces (recording a curved skip), names in-plane edges and merged groups per N2/N3 as Ev approved on PR 5306, and reads `Keeps` by descent in one builder. Pinned in `split_pinch_keeps.rs`, `m3_pr3_split.rs` and `a_lower_half_on_the_plane_takes_a_box_hole`.
