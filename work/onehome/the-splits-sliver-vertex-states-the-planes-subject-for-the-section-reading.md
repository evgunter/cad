---
id: the-splits-sliver-vertex-states-the-planes-subject-for-the-section-reading
kind: issue
title: The split's SliverVertex states the plane's subject for the section reading's margin
status: open
priority: P1
cost: E
opened: 2026-11-02
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
