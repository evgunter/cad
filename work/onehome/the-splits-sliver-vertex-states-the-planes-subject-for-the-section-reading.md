---
id: the-splits-sliver-vertex-states-the-planes-subject-for-the-section-reading
kind: issue
title: The split's SliverVertex states the plane's subject for the section reading's margin
status: closed
priority: P1
cost: E
opened: 2026-11-02
closed: 2026-11-15
pr: 7009
branch: onehome/split-sliver-vertex
---


Found by the sweep of `onehome/vertex-section-room`, the split's twin of
that unit's item 6 (one refusal, two subjects), which fixed the
Boolean's.

`vertex_section::read_sections` refuses on two decisions: the offset
from the plane (under the door's name, `split_vertex_side`) and the
vertex's reading against the section the plane ends at it
(`vertex_section_side`, margin `δ / sin ψ`). The Boolean now refuses
them as `Coincide::VertexOnFace` and `Coincide::VertexOnSection`
(`boolean::reduce::vertex_refusal`). The split still maps both
(`splitting::classify`, the `read_sections` call's `map_err`) to
`SplitReduceError::SliverVertex`, whose `Display`
(`splitting/mod.rs`) states "whether a vertex lies on the split plane"
whatever its `diag` decided. Its doc comment names both readings, so
this was chosen, but the words then do not match the margin printed
beside them on the section arm.

Fix: a `SliverSection { vertex, diag }` arm (or the subject read off the
refusal's decision, never off the predicate's name), with the
`f6_variants!` list and the concision row in
`crates/editor-core/tests/refusal_concision_chains.rs` updated, and the
section-arm matches in `crates/sweep/tests/a_vertex_on_a_grazing_plane.rs`
and `crates/sweep/tests/split_tangent_edge_curved.rs` moved to it.

P1 by `work/README.md` Priority: the same decision stated two ways at
two doors.

## Closed (2026-11-15)

The split now refuses its two readings under two subjects, as the Boolean
does. The offset from the plane (`split_vertex_side`) stays
`SplitReduceError::SliverVertex`, "whether a vertex lies on the split plane".
The vertex's reading against the section the plane ends at it
(`vertex_section_side`, margin `δ / sin ψ`) is the new
`SplitReduceError::SliverSection`, "whether a vertex on the split plane lies
on the section that plane cuts in a face at the vertex", the twin of the
Boolean's `Coincide::VertexOnSection` wording. Both take the split's
coincidence lever.

Rows moved: `refusal_concision_chains` lists the arm (`f6_variants!`) and
words it. `a_vertex_on_a_grazing_plane`'s band arm takes each reading at its
own variant, and its shallower-face row takes `SliverSection`.
`split_tangent_edge_curved`'s in-band seam arm reads `split_vertex_side`, the
offset, so it stays on `SliverVertex`.
