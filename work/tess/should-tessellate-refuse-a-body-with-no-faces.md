---
id: should-tessellate-refuse-a-body-with-no-faces
kind: issue
title: should tessellate refuse a body with no faces, on that structural fact?
status: open
priority: P3
cost: E
opened: 2026-09-22
---



**A question, not a decision.** Left `unsure` by TESS-4's reviewer and
recorded rather than answered, because both answers are defensible and
neither is this row's to pick alone.

TESS-4 established that a `Mesh` with no triangles is not the mesh of a
solid, and `mesh::validate::check_mesh` now refuses it
(`MeshError::NoTriangles`). The producer reachable by VALID input is a
body with no faces: `topo::Body::new()` is public, `topo::validate`'s
own doc says the empty body validates vacuously (tier 1 and tier 2),
and `tessellate_impl` has no face-count guard — it answers `Ok` with a
mesh of nothing. Pinned by
`survives_the_empty_body_meshes_to_nothing_and_the_validator_says_so`
(`crates/mesh/tests/review_m2_pr6_checkmesh_audit.rs`).

**For a refusal at `tessellate`'s door.** "This body has no faces" is
STRUCTURAL and available before the walk, so it satisfies TESS-1's rule
that a refusal reads no count (`work/tess/TESS-1.md`, `## Closed`) — it
is a fact about the arena, not about what was emitted. `step-import`
has the precedent and the name: a file that declares no usable shape
refuses `StepImportError::NothingToImport` rather than hand back an
empty body. Answering `Ok` with nothing means every consumer that does
not call the validator — and `tessellate`'s contract says it does not
run one — carries an empty mesh onward silently; the STL writers then
turn it into a file whose keyword is `solid`
(`work/exch/stl-writes-a-solid-for-a-mesh-with-no-triangles.md`).

**Against.** F8 ratified that ∅ is a value and not an error, which is
why `topo::BooleanResult::Empty` is a typed empty SUCCESS; a mesh lane
that refuses the empty body makes ∅ un-meshable and pushes the
special-case onto every caller that might hold one. The kernel is also
deliberately count-blind at refusal sites, and "no faces" is a count of
faces however structurally it is phrased — TESS-1's rule bars reading
the emitted triangle count, and whether it bars reading the arena's
size is exactly the ambiguity this row is about. And the empty body
validates vacuously by ratified tier semantics, so refusing to mesh
what `topo::validate` calls valid puts two doors in disagreement.

**What settling it needs.** A reading of F8's scope (does "∅ is a
value" reach the mesh lane, or stop at the boolean's result type?), and
a decision on whether a refusal would be `TessellateError` or a new
name. If the answer is "no refusal", the residue is that
`tessellate`'s own doc should say the empty body meshes to nothing, so
a caller reads it there rather than discovering it.
